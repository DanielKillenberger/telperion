//! fn-270 task 6: what a grow-in holds, against the page's budget. The
//! measure: a preset at an age (`GROW_AGE`, else its own) built at once
//! and put up, as the harness opens it, then grown in from year 1 at
//! whole years as the harness's "grow in" grows it (`grow_to` the year,
//! `settle` it, put it up in place of the last), under the page's budget
//! (`GROW_BUDGET_MIB`, else `BROWSER`). Each step's peak is a labelled
//! census (`CENSUS` lines, MB). Each draw is admitted as the page admits
//! it (`draw_admission`): beside the drawn tree, or that tree let go
//! first. The grow-in grows whole, or is refused by the budget by name
//! with the tree drawn before still up. Run by name:
//!
//!   GROW_PRESET=norway-spruce GROW_AGE=80 cargo test --profile ci \
//!     -p telperion-render --lib grows_in_under_the_page_budget -- --ignored --nocapture
use super::super::{Admitted, Generator};
use crate::Renderer;
use pollster::block_on;

/// One labelled census line, MB.
fn census(id: &str, what: &str, bytes: u64, by: &[(&'static str, u64)]) {
    let mb = |b: u64| b as f64 / 1e6;
    let rows: Vec<String> = by
        .iter()
        .map(|(l, b)| format!("{l} {:.0}", mb(*b)))
        .collect();
    println!(
        "CENSUS {id} {what}: {:.0} MB: {}",
        mb(bytes),
        rows.join(", ")
    );
}

/// A buffer let go ahead of its owner leaves the ledger once the device
/// is done with it, and its owner's drop charges nothing back: the ledger
/// charged a destroyed buffer until its owner dropped, and a grow-in's
/// draw was refused for bytes the device no longer held.
#[test]
fn a_buffer_let_go_early_leaves_the_ledger_once() {
    let Some(gpu) = crate::generation::test_gpu() else {
        return;
    };
    let usage = wgpu::BufferUsages::STORAGE;
    block_on(crate::generation::io::complete(&gpu)).unwrap();
    let before = gpu.ledger.live();
    let held = crate::generation::io::buffer(&gpu, "let go early", 1 << 20, usage, &[]).unwrap();
    assert_eq!(gpu.ledger.live(), before + (1 << 20));
    held.destroy();
    block_on(crate::generation::io::complete(&gpu)).unwrap();
    assert_eq!(gpu.ledger.live(), before, "let go once the device is done");
    drop(held);
    block_on(crate::generation::io::complete(&gpu)).unwrap();
    assert_eq!(gpu.ledger.live(), before, "the drop lets nothing go twice");
}

/// A step of the grow-in, its failure stated.
fn step(phase: &str, year: u32, result: crate::Result<()>) -> Option<String> {
    result.err().map(|e| format!("{phase} at {year}: {e}"))
}

#[test]
#[ignore = "fn-270 task 6 measure: run by name"]
fn grows_in_under_the_page_budget() {
    let _held = crate::whole_tree::whole_tree();
    let Some(gpu) = crate::generation::test_gpu() else {
        return;
    };
    let id = std::env::var("GROW_PRESET").unwrap_or_else(|_| "norway-spruce".into());
    let mut family = telperion_core::presets::by_identity(&id).expect("a shipped preset");
    if let Ok(age) = std::env::var("GROW_AGE") {
        family.age = age.parse().expect("GROW_AGE is a number");
    }
    let budget = std::env::var("GROW_BUDGET_MIB")
        .map_or(super::budget::BROWSER, |m| m.parse::<u64>().unwrap() << 20);
    let mut renderer = Renderer::new(gpu.clone(), crate::STILL_FORMAT);
    let gen = Generator::new(&renderer).unwrap();
    block_on(gen.prepare_programs()).unwrap();
    gen.budget.set(Some(budget));
    let label = format!("{id}@{}", family.age);
    gpu.ledger.take_peak_census();
    // The harness opens the tree built at once, and keeps it up.
    let built = gen.prepare(&family).expect("built at once");
    let (peak, by) = gpu.ledger.take_peak_census();
    census(&label, "built at once", peak, &by);
    renderer.submit_prepared(built).unwrap();
    let mut tree = block_on(gen.begin_growth(&family)).expect("begun");
    let (mut most, mut at_most) = (0, (0, "", vec![]));
    let (mut failed, mut room, mut worst) = (None, vec![], 0.0f64);
    for year in 1..=tree.horizon() {
        let up = Generator::tree_buffer_bytes(&renderer);
        let grown = step(
            "grow",
            year,
            block_on(gen.grow_to(&mut tree, f64::from(year))),
        );
        let (peak, by) = gpu.ledger.take_peak_census();
        if peak > most {
            (most, at_most) = (peak, (year, "grow", by));
        }
        let admitted = match grown {
            Some(e) => Err(e),
            None => Generator::draw_admission(&tree, &renderer, budget)
                .map_err(|e| format!("draw at {year}: {e}")),
        };
        let expected = tree.expected_draw();
        if admitted.as_ref().is_ok_and(|a| *a == Admitted::MakeRoom) {
            renderer.release_tree();
            block_on(crate::generation::io::complete(&gpu)).unwrap();
            room.push(year);
        }
        let drawn = admitted.and_then(|_| {
            block_on(gen.settle(&tree, f64::from(year))).map_err(|e| format!("draw at {year}: {e}"))
        });
        if let (Ok(_), added, true) = (&drawn, tree.last_draw(), expected >= 64 << 20) {
            worst = worst.max(added as f64 / expected as f64);
            println!(
                "DRAW {label} {year}: {} MB added, {} MB expected",
                added / 1_000_000,
                expected / 1_000_000
            );
        }
        let (peak, by) = gpu.ledger.take_peak_census();
        if peak > most {
            (most, at_most) = (peak, (year, "draw", by));
        }
        match drawn {
            Ok(drawn) => drop(renderer.submit_prepared(drawn).unwrap()),
            Err(e) => {
                // A refusal keeps the tree drawn before up, whole; where it
                // was let go to make room, the screen stays empty.
                let kept = if room.last() == Some(&year) { 0 } else { up };
                assert_eq!(Generator::tree_buffer_bytes(&renderer), kept, "{e}");
                failed = Some(e);
                break;
            }
        }
        if year % 10 == 0 {
            println!(
                "YEAR {label} {year}: {} MB held between draws, the peak so far {} MB",
                gpu.ledger.live() / 1_000_000,
                most / 1_000_000
            );
        }
    }
    let (year, phase, by) = at_most;
    census(
        &label,
        &format!("grow-in peak ({phase} at {year})"),
        most,
        &by,
    );
    println!(
        "GROWN {label} under {} MiB: peak {} MiB, {}",
        budget >> 20,
        most >> 20,
        failed.as_deref().unwrap_or("grown whole")
    );
    println!("ROOM {label}: the drawn tree let go before the draws of years {room:?}; a draw's bytes at most {worst:.3} of those expected");
    if let Some(e) = failed {
        assert!(
            e.contains("the device budget"),
            "{label}: refused by the budget by name, not the device: {e}"
        );
    }
}
