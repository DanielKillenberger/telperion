//! fn-270 R2 on the device at a shipped preset's own age, one preset a
//! process (`PRESET=<identity>`), run by name:
//!
//!   PRESET=telperion cargo test --profile ci -p telperion-render --lib presets_grow_on_on_the_device -- --ignored --nocapture
//!
//! The tree grown in one call as the resident build grows it (tables from
//! the species' hint, scratch let go before the settle), then through the
//! state paused half way and the year before its age, the tree drawn at
//! the pause held while it grows on: the node tables word for word, the
//! seconds a paused build takes to make its scratch and cycle again, and
//! each way's device peak, by label (`CENSUS` lines, MB: fn-270 R4), the
//! tree drawn within its last year beside the drawn one among them.
use super::super::build_arena::hint;
use super::super::build_grow::{grow_now, Sizes};
use super::super::settle_run::settle_now;
use super::tests::digest;
use super::*;
use std::time::Instant;
use telperion_core::pipeline::executor;

#[test]
#[ignore = "a measurement: one shipped preset at its age, by PRESET"]
fn presets_grow_on_on_the_device() {
    let _held = super::super::super::whole_tree();
    let Some(gpu) = super::super::super::test_gpu() else {
        return;
    };
    let id = std::env::var("PRESET").expect("PRESET names a shipped preset");
    let family = telperion_core::presets::by_identity(&id).expect("a shipped preset");
    let growth = executor::growth(&family).expect("a growth");
    let (h, seed) = (growth.request.age, growth.request.seed);
    let k = Kernels::new(&gpu);
    let sizes = Sizes::hinted(hint(&growth.species, h).expect("a hint")).expect("sizes");
    let (once, once_peak, once_s) = {
        let g = super::super::grown_tests::grown_from(
            growth.species.clone(),
            growth.request.light,
            (h, seed),
        );
        let t = Tables::new(&gpu, &g.tables, &g.chain).expect("tables");
        gpu.ledger.take_peak_census();
        let started = Instant::now();
        let (b, ..) = grow_now(&gpu, &k.build, &t, &g.tables, (sizes, h)).expect("a build");
        let b = pollster::block_on(b.settling(&gpu, (&t, &g.tables))).expect("settling");
        let s = settle_now(
            &gpu,
            (&k.build, &k.settle),
            (&b, &t),
            (&t.settle_pa, SettleTo::Laid),
        )
        .expect("a settle");
        // The peak before the digest's readback, which a build makes none of.
        let (peak, by) = gpu.ledger.take_peak_census();
        census(&id, "standalone, laid", peak, &by);
        let d = digest(&gpu, &s);
        (d, peak, started.elapsed().as_secs_f64())
    };
    gpu.device
        .poll(wgpu::PollType::wait_indefinitely())
        .expect("let go");
    println!(
        "PRESET {id} {h}: once {once:x}, {once_s:.2} s, peak {} MiB",
        once_peak >> 20
    );
    let mut differ = Vec::new();
    for a in [h / 2, h - 1] {
        gpu.ledger.take_peak_census();
        let started = Instant::now();
        let mut s = pollster::block_on(GrowthState::begin(&gpu, &k, &growth, (seed, h), 9 << 30))
            .expect("begun");
        pollster::block_on(s.grow_to(&gpu, &k, f64::from(a))).expect("grown");
        let paused = gpu.ledger.live();
        census(
            &id,
            &format!("paused at {a}"),
            paused,
            &gpu.ledger.census_now(),
        );
        let drawn = pollster::block_on(s.settle(&gpu, &k, f64::from(a))).expect("drawn");
        let (first, by) = gpu.ledger.take_peak_census();
        census(&id, &format!("to {a} and drawn"), first, &by);
        pollster::block_on(s.grow_to(&gpu, &k, f64::from(h))).expect("grown on");
        let (grown, by) = gpu.ledger.take_peak_census();
        census(
            &id,
            &format!("grown on from {a} beside the drawn"),
            grown,
            &by,
        );
        // The next tree drawn while the last is drawn still.
        let next = pollster::block_on(s.settle(&gpu, &k, f64::from(h))).expect("drawn on");
        let (redrawn, by) = gpu.ledger.take_peak_census();
        census(&id, &format!("drawn at {h} beside the drawn"), redrawn, &by);
        drop(next);
        // Drawn within its last year (R5a) beside the tree drawn.
        let within = pollster::block_on(s.settle(&gpu, &k, f64::from(h) - 0.5))
            .expect("drawn within the year");
        let (unfolding, by) = gpu.ledger.take_peak_census();
        census(
            &id,
            &format!("drawn at {} beside the drawn", f64::from(h) - 0.5),
            unfolding,
            &by,
        );
        drop(within);
        let then = grown.max(redrawn).max(unfolding);
        let on = digest(
            &gpu,
            &pollster::block_on(s.settled(&gpu, &k)).expect("settled"),
        );
        let g = s.growing.as_ref().expect("a build");
        let hier = &g.b.hier;
        let kept = [
            &hier.shape,
            &hier.counts,
            &hier.lists[0],
            &hier.lists[1],
            &hier.tally,
        ]
        .iter()
        .map(|b| b.size())
        .sum::<u64>();
        let (state, retire) = (g.b.state.bytes(), g.b.retire.bytes());
        println!(
            "PRESET {id} {h} paused at {a}: {on:x} {}, {:.2} s; held paused {} MiB (state {} MiB, retirement {} MiB, hierarchy {} MiB); peak to {a} and drawn {} MiB, grown on beside it {} MiB; scratch and cycle made again in {:.3} s, moves {}, compactions {}",
            if on == once { "the same" } else { "DIFFERS" },
            started.elapsed().as_secs_f64(), paused >> 20, state >> 20, retire >> 20, kept >> 20, first >> 20, then >> 20, g.grew.rebuilt, g.grew.moves, g.grew.compactions
        );
        if on != once {
            differ.push(a);
        }
        drop((drawn, s));
        gpu.device
            .poll(wgpu::PollType::wait_indefinitely())
            .expect("let go");
    }
    assert!(
        differ.is_empty(),
        "{id}: grown on from {differ:?}, the tree differs"
    );
}

/// One labelled census line: the bytes held and each label's, MB.
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
