//! fn-270 task 6: what a grow-in holds, against the page's budget. The
//! measure: a preset at an age (`GROW_AGE`, else its own) built at once
//! and put up, as the harness opens it, then grown in from year 1 at
//! whole years as the harness's "grow in" grows it (`grow_to` the year,
//! `settle` it, put it up in place of the last), under the page's budget
//! (`GROW_BUDGET_MIB`, else `BROWSER`). Each step's peak is a labelled
//! census (`CENSUS` lines, MB). The grow-in grows whole, or is refused by
//! the budget by name with the tree drawn before still up (R2c). Run by
//! name:
//!
//!   GROW_PRESET=norway-spruce GROW_AGE=80 cargo test --profile ci \
//!     -p telperion-render --lib grows_in_under_the_page_budget -- --ignored --nocapture
use super::super::{Delivery, Generator};
use crate::Renderer;
use pollster::block_on;

/// One labelled census line, MB.
fn census(id: &str, what: &str, bytes: u64, by: &[(&'static str, u64)]) {
    let mb = |b: u64| b as f64 / 1e6;
    let rows: Vec<String> = by.iter().map(|(l, b)| format!("{l} {:.0}", mb(*b))).collect();
    println!("CENSUS {id} {what}: {:.0} MB: {}", mb(bytes), rows.join(", "));
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
    let budget = std::env::var("GROW_BUDGET_MIB").map_or(super::budget::BROWSER, |m| m.parse::<u64>().unwrap() << 20);
    let mut renderer = Renderer::new(gpu.clone(), crate::STILL_FORMAT);
    let gen = Generator::new(&renderer).unwrap();
    block_on(gen.prepare_programs()).unwrap();
    gen.budget.set(Some(budget));
    let label = format!("{id}@{}", family.age);
    gpu.ledger.take_peak_census();
    // The harness opens the tree built at once, and keeps it up.
    let built = gen.prepare(&family, Delivery::Resident).expect("built at once");
    let (peak, by) = gpu.ledger.take_peak_census();
    census(&label, "built at once", peak, &by);
    renderer.submit_prepared(built).unwrap();
    let mut tree = block_on(gen.begin_growth(&family)).expect("begun");
    let (mut most, mut at_most) = (0, (0, "", vec![]));
    let mut failed = None;
    for year in 1..=tree.horizon() {
        let up = Generator::tree_buffer_bytes(&renderer);
        let grown = step("grow", year, block_on(gen.grow_to(&mut tree, f64::from(year))));
        let (peak, by) = gpu.ledger.take_peak_census();
        if peak > most {
            (most, at_most) = (peak, (year, "grow", by));
        }
        let drawn = match grown {
            Some(e) => Err(e),
            None => block_on(gen.settle(&tree, f64::from(year))).map_err(|e| format!("draw at {year}: {e}")),
        };
        let (peak, by) = gpu.ledger.take_peak_census();
        if peak > most {
            (most, at_most) = (peak, (year, "draw", by));
        }
        match drawn {
            Ok(drawn) => drop(renderer.submit_prepared(drawn).unwrap()),
            Err(e) => {
                // The tree drawn before stays up, whole (R2c).
                assert_eq!(Generator::tree_buffer_bytes(&renderer), up, "{e}");
                failed = Some(e);
                break;
            }
        }
    }
    let (year, phase, by) = at_most;
    census(&label, &format!("grow-in peak ({phase} at {year})"), most, &by);
    println!("GROWN {label} under {} MiB: peak {} MiB, {}", budget >> 20, most >> 20, failed.as_deref().unwrap_or("grown whole"));
    if let Some(e) = failed {
        assert!(e.contains("the device budget"), "{label}: refused by the budget by name, not the device: {e}");
    }
}
