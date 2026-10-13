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

/// The node table's column of each axis's grown index (`settle_compact.wgsl`'s
/// `NA_GROWN`): its slot in the arena, which a compaction renumbers.
const NA_GROWN: usize = 23;

/// A digest of a settled tree's node table, to the word but for each
/// axis's grown index, which the arena's compactions renumber (an axis is
/// known by its lineage, DESIGN.md section 13), and its fault.
pub(in crate::generation::cycle) fn digest(gpu: &Gpu, s: &Settled) -> u64 {
    let mut words = s.error.to_vec();
    let mut table = read_words(gpu, &s.nt, 0, s.table.words).expect("a readback");
    let (sk, kept) = (s.table.sk as usize, s.table.kept as usize);
    table[NA_GROWN * sk..NA_GROWN * sk + kept].fill(0);
    words.extend(table);
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
        // Nothing grown yet: refused by name, the state left to grow on.
        let unborn = block(whole.settle(&gpu, &k, 0.0)).err().expect("no tree before a year");
        assert!(unborn.to_string().contains("at: "), "{unborn}");
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
            let grew = &s.growing.as_ref().expect("a build").grew;
            println!("{name} paused at {a}: moves {} after cycles {:?}, compactions {}", grew.moves, grew.moved_at, grew.compactions);
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

/// R2c: the lit oak grows on year by year with its last tree drawn
/// throughout, held to its budget beside it; under a budget its overlap
/// cannot fit, the growth is refused by name, the state spent and the
/// tree drawn kept whole.
#[test]
fn a_lit_tree_grows_on_beside_the_tree_drawn() {
    let _held = super::super::super::whole_tree();
    let Some(gpu) = super::super::super::test_gpu() else {
        return;
    };
    let k = Kernels::new(&gpu);
    let family = telperion_core::presets::Preset::OregonWhiteOak.parameters();
    let growth = executor::growth(&family).expect("a growth");
    assert!(growth.request.light.extinction > 0.0, "the oak reads light");
    let budget = 1 << 30;
    let mut s = block(GrowthState::begin(&gpu, &k, &growth, (7, 12), budget)).expect("begun");
    let mut drawn = None;
    gpu.ledger.take_peak();
    for y in 1..=12 {
        block(s.grow_to(&gpu, &k, f64::from(y))).expect("grown beside the tree drawn");
        drawn = Some(block(s.settle(&gpu, &k, f64::from(y))).expect("drawn"));
    }
    let peak = gpu.ledger.take_peak();
    assert!(drawn.is_some());
    println!("the oak to 12 year by year, its last tree drawn: peak {} MiB of {}", peak >> 20, budget >> 20);
    assert!(peak <= budget);
    let mut s = block(GrowthState::begin(&gpu, &k, &growth, (7, 12), budget)).expect("begun");
    block(s.grow_to(&gpu, &k, 6.0)).expect("grown");
    let drawn = block(s.settle(&gpu, &k, 6.0)).expect("drawn").0;
    let before = read_words(&gpu, &drawn.waves[1].buffer, 0, 64).expect("the tree drawn");
    s.budget = gpu.ledger.live() + (1 << 20);
    let refused = block(s.grow_to(&gpu, &k, 7.0)).expect_err("no room beside the tree drawn");
    assert!(refused.to_string().contains("the device budget"), "{refused}");
    assert_eq!(s.committed(), None);
    assert_eq!(read_words(&gpu, &drawn.waves[1].buffer, 0, 64).expect("the tree drawn"), before);
    // A draw with no room beside the tree drawn spends the state too.
    let mut s = block(GrowthState::begin(&gpu, &k, &growth, (7, 12), budget)).expect("begun");
    block(s.grow_to(&gpu, &k, 6.0)).expect("grown");
    s.budget = gpu.ledger.live() + 1;
    let refused = block(s.settle(&gpu, &k, 6.0)).err().expect("no room to draw");
    assert!(refused.to_string().contains("the device budget"), "{refused}");
    let spent = |e: crate::RenderError| assert!(e.to_string().contains("state.failed"), "{e}");
    spent(block(s.grow_to(&gpu, &k, 6.0)).expect_err("spent"));
    spent(block(s.settle(&gpu, &k, 6.0)).err().expect("spent"));
    assert_eq!(read_words(&gpu, &drawn.waves[1].buffer, 0, 64).expect("the tree drawn"), before);
}

impl GrowthState {
    /// `sized` from the least tables, compacted every cycle, blocked on,
    /// under 4 GiB.
    fn sized_now(gpu: &Gpu, k: &Kernels, growth: &Growth, at: (u64, u32)) -> Self {
        block(Self::sized(gpu, k, growth, at, (forced(), 4 << 30))).expect("begun")
    }
}
