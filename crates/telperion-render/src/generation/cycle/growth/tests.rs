//! fn-270 R2 on the device: a tree paused, drawn and grown on is, word for
//! word, the tree grown in one call, its tables started from the least so
//! they move, and its arena compacted after every cycle; and a request the
//! state cannot meet, or a failure while it grows, is refused by name.
use super::super::build_grow::{grow_now, Compact, Sizes};
use super::super::grown_tests::{grown_from, YOUNG};
use super::super::hier_scan::read_words;
use super::super::settle_run::settle_now;
use super::super::settle_tests::constants;
use super::*;
use std::hash::{Hash, Hasher};
use telperion_core::pipeline::executor;

/// The least tables, compacted after every cycle: every move and
/// compaction a small tree can meet.
fn forced() -> Sizes {
    Sizes { compact: Compact::Every, ..Sizes::least() }
}

/// A digest of a settled tree's node table, to the word, and its fault.
fn digest(gpu: &Gpu, s: &Settled) -> u64 {
    let mut words = s.error.to_vec();
    words.extend(read_words(gpu, &s.nt, 0, s.table.words).expect("a readback"));
    let mut h = std::collections::hash_map::DefaultHasher::new();
    words.hash(&mut h);
    h.finish()
}

fn block<T>(f: impl std::future::Future<Output = T>) -> T {
    pollster::block_on(f)
}

/// R2 on the device: grown to its horizon in one call (the build `tree`
/// draws), and through the state paused at 2, half way and the year
/// before, drawn twice there, advanced within its year and grown on: one
/// node table, word for word; and drawn within its last year, the state
/// left as it was.
#[test]
fn a_tree_grown_on_on_the_device_is_the_tree_grown_at_once() {
    let _held = super::super::super::whole_tree();
    let Some(gpu) = super::super::super::test_gpu() else {
        return;
    };
    let k = Kernels::new(&gpu);
    let mut differ = Vec::new();
    for (preset, name, h) in YOUNG {
        let family = preset.parameters();
        let growth = executor::growth(&family).expect("a growth");
        let g = grown_from(growth.species.clone(), growth.request.light, (h, growth.request.seed));
        let once = {
            let t = Tables::new(&gpu, &g.tables, &g.chain).expect("tables");
            let (b, ..) = grow_now(&gpu, &k.build, &t, &g.tables, (forced(), h)).expect("a build");
            let s = settle_now(&gpu, (&k.build, &k.settle), (&b, &t), (&constants(&g), SettleTo::Laid)).expect("a settle");
            digest(&gpu, &s)
        };
        let seed = g.request.seed;
        let mut whole = GrowthState::sized_now(&gpu, &k, &growth, (seed, h));
        block(whole.grow_to(&gpu, &k, f64::from(h))).expect("grown");
        let at_once = digest(&gpu, &block(whole.settled(&gpu, &k)).expect("settled"));
        drop(whole);
        let mut paused = Vec::new();
        for a in [2, h / 2, h - 1] {
            let mut s = GrowthState::sized_now(&gpu, &k, &growth, (seed, h));
            block(s.grow_to(&gpu, &k, f64::from(a))).expect("grown");
            let first = digest(&gpu, &block(s.settled(&gpu, &k)).expect("settled"));
            block(s.grow_to(&gpu, &k, f64::from(a) - 0.5)).expect("met");
            let again = digest(&gpu, &block(s.settled(&gpu, &k)).expect("settled"));
            block(s.settle(&gpu, &k, f64::from(a) - 0.25)).expect("drawn within the year");
            block(s.grow_to(&gpu, &k, f64::from(h) - 0.75)).expect("grown on");
            assert_eq!(s.committed(), Some(h));
            block(s.settle(&gpu, &k, f64::from(h) - 0.5)).expect("drawn within the year");
            let on = digest(&gpu, &block(s.settled(&gpu, &k)).expect("settled"));
            paused.push((a, first == again, on));
        }
        println!("{name} {h}: once {once:x}, through the state {at_once:x}, paused {paused:x?}");
        let same = paused.iter().all(|(_, repeats, on)| *repeats && *on == at_once);
        if once != at_once || !same {
            differ.push(name);
        }
    }
    assert!(differ.is_empty(), "grown on, the tree differs: {differ:?}");
}

/// R2b on the device: an age not finite, past the horizon or a year back
/// and a draw outside the last year grown are refused by name, the state
/// left to grow on; a growth that fails (here past its budget) spends it.
#[test]
fn a_request_the_device_cannot_meet_is_refused_by_name() {
    let _held = super::super::super::whole_tree();
    let Some(gpu) = super::super::super::test_gpu() else {
        return;
    };
    let k = Kernels::new(&gpu);
    let family = telperion_core::presets::Preset::EuropeanBeech.parameters();
    let mut growth = executor::growth(&family).expect("a growth");
    let mut s = GrowthState::sized_now(&gpu, &k, &growth, (7, 8));
    block(s.grow_to(&gpu, &k, 4.0)).expect("grown");
    let named = |r: Result<()>, name: &str| match r {
        Err(e) => assert!(e.to_string().contains(name), "{name}: {e}"),
        Ok(()) => panic!("{name}: met"),
    };
    named(block(s.grow_to(&gpu, &k, f64::NAN)), "age:");
    named(block(s.grow_to(&gpu, &k, 8.5)), "age.horizon");
    named(block(s.grow_to(&gpu, &k, 3.0)), "age.backward");
    named(block(s.settle(&gpu, &k, 4.5)).map(drop), "at:");
    assert_eq!(s.committed(), Some(4));
    block(s.grow_to(&gpu, &k, 8.0)).expect("grown on");
    growth.request.budget = 200;
    let mut s = GrowthState::sized_now(&gpu, &k, &growth, (7, 8));
    let failed = (1..=8).find_map(|y| block(s.grow_to(&gpu, &k, f64::from(y))).err());
    assert!(failed.is_some(), "a budget of 200 phytomers holds the beech to 8");
    assert_eq!(s.committed(), None);
    named(block(s.grow_to(&gpu, &k, 8.0)), "state.failed");
    named(block(s.settle(&gpu, &k, 8.0)).map(drop), "state.failed");
}

impl GrowthState {
    /// `sized` from the least tables, compacted every cycle, blocked on.
    fn sized_now(gpu: &Gpu, k: &Kernels, growth: &Growth, at: (u64, u32)) -> Self {
        block(Self::sized(gpu, k, growth, at, forced())).expect("begun")
    }
}
