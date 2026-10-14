//! fn-251: a walk never grows a tree larger than its ends. The teaser's
//! frame that once passed the engine's budget grows within it, and the nine frames
//! of the sweep that miss its oracle (`examples/walk_size.rs`; WALKS.md,
//! "The seed-spread oracle") are named exceptions held at their measured
//! numbers until fn-273 takes them: none may move further outside its
//! band, nor reach the budget (host, 2026-10-14).
use telperion_core::{blend, presets::Preset, Family};
use telperion_space::{grow_cycle, Request};
use Preset::{
    DatePalm as Palm, EuropeanBeech as Beech, Laurelin, NorwaySpruce as Spruce,
    OregonWhiteOak as Oak, Ordinary, SilverBirch as Birch,
};

/// The engine's phytomer budget, the build's own (`pipeline/space.rs`).
const BUDGET: u32 = 20_000_000;
/// The oracle's widening of its band (fn-251 R1b) and its slack in whole
/// phytomers (host, 2026-10-14).
const WIDEN: f64 = 0.1;
const SLACK: f64 = 2.0;

fn end(preset: Preset, age: f64, seed: u32) -> Family {
    let mut family = preset.parameters();
    (family.age, family.skeleton.seed) = (age, seed);
    family
}

/// A frame that misses the oracle: its walk, age, seed and share; the
/// band, both ends' realized kept and grown phytomers over seeds 1, 7 and
/// 42 at its age and light (lowest, highest); and the kept and grown
/// phytomers it was measured at on 37398bd2b.
struct Exception {
    walk: (Preset, Preset),
    age: f64,
    seed: u32,
    t: f64,
    band: [(f64, f64); 2],
    measured: [f64; 2],
}

/// Both ends of the oak and spruce at 40 (seeds 1, 7 and 42).
const OAK_SPRUCE_40: [(f64, f64); 2] = [(354_122.0, 1_104_620.0), (1_047_055.0, 2_749_947.0)];

const EXCEPTIONS: [Exception; 9] = [
    Exception {
        walk: (Oak, Birch),
        age: 80.0,
        seed: 7,
        t: 0.01,
        band: [(1_422_181.0, 3_385_291.0), (8_000_869.0, 16_625_765.0)],
        measured: [3_953_260.0, 18_175_583.0],
    },
    Exception {
        walk: (Oak, Ordinary),
        age: 40.0,
        seed: 1,
        t: 0.75,
        band: [(98_544.0, 488_475.0), (236_384.0, 1_340_416.0)],
        measured: [84_798.0, 210_186.0],
    },
    Exception {
        walk: (Oak, Spruce),
        age: 40.0,
        seed: 1,
        t: 0.25,
        band: OAK_SPRUCE_40,
        measured: [370_943.0, 813_819.0],
    },
    Exception {
        walk: (Spruce, Oak),
        age: 40.0,
        seed: 7,
        t: 0.5,
        band: OAK_SPRUCE_40,
        measured: [346_727.0, 914_686.0],
    },
    Exception {
        walk: (Spruce, Oak),
        age: 40.0,
        seed: 7,
        t: 0.75,
        band: OAK_SPRUCE_40,
        measured: [263_622.0, 603_467.0],
    },
    Exception {
        walk: (Palm, Laurelin),
        age: 3.0,
        seed: 7,
        t: 0.25,
        band: [(36.0, 88.0), (36.0, 88.0)],
        measured: [31.0, 31.0],
    },
    Exception {
        walk: (Beech, Oak),
        age: 80.0,
        seed: 1,
        t: 0.5,
        band: [(1_471_492.0, 3_385_291.0), (8_243_029.0, 16_625_765.0)],
        measured: [1_304_437.0, 5_774_230.0],
    },
    Exception {
        walk: (Oak, Spruce),
        age: 80.0,
        seed: 1,
        t: 0.5,
        band: [(1_471_492.0, 3_800_870.0), (8_499_924.0, 17_915_812.0)],
        measured: [1_514_700.0, 7_375_617.0],
    },
    Exception {
        walk: (Birch, Oak),
        age: 40.0,
        seed: 7,
        t: 0.5,
        band: [(263_041.0, 488_475.0), (746_474.0, 1_340_416.0)],
        // Kept 579,740 to 579,745 with fn-252's birch ground hazard
        // (9a9a734a4): an owner-approved rule, 8.6e-6 of the count.
        measured: [579_745.0, 1_353_116.0],
    },
];

/// How far `n` lies outside the band `lo` to `hi` widened by `WIDEN`, or
/// by `SLACK` phytomers where that is wider, as a share of the edge it
/// passes; 0 within.
fn miss(n: f64, (lo, hi): (f64, f64)) -> f64 {
    let low = (lo * (1.0 - WIDEN)).min(lo - SLACK);
    let high = (hi * (1.0 + WIDEN)).max(hi + SLACK);
    ((low - n) / low).max((n - high) / high).max(0.0)
}

/// The frame's kept and grown phytomers, grown as the build grows it, at
/// its seed and light within the engine's budget.
fn grown(frame: &Family) -> telperion_space::Result<[f64; 2]> {
    let request = Request {
        age: frame.age as u32,
        seed: u64::from(frame.skeleton.seed),
        budget: BUDGET,
        light: frame.space.light,
    };
    let species = frame.space.species.as_ref().expect("an engine preset");
    let cycled = grow_cycle(species, request)?;
    let kept: usize = cycled.structure.axes.iter().map(|a| a.phytomers.len()).sum();
    Ok([kept as f64, cycled.grown as f64])
}

/// fn-251 R1: the teaser's walk from the oak to the birch at 80, seed 7,
/// the frame at t = 0.07504803866484144 that `headless --walk 6 --hold 0`
/// refused on the engine's phytomer budget (frame 24), grows within it;
/// then the named exceptions. One test, so no two trees of 80 years grow
/// at once (each takes several GB).
#[test]
fn the_teaser_frame_fits_and_the_named_exceptions_grow_no_further_out() {
    let (a, b) = (end(Oak, 80.0, 7), end(Birch, 80.0, 7));
    let teaser = blend::families(&a, &b, 0.07504803866484144).unwrap();
    if let Err(err) = grown(&teaser) {
        panic!("the teaser's frame: {err}");
    }
    let mut worse = Vec::new();
    for e in &EXCEPTIONS {
        let (a, b) = (end(e.walk.0, e.age, e.seed), end(e.walk.1, e.age, e.seed));
        let walked = blend::families(&a, &b, e.t).unwrap();
        let grown = grown(&walked).unwrap_or_else(|err| {
            panic!("{:?} at {} {}: {err}", e.walk, e.age, e.t);
        });
        let then = [0, 1].map(|k| miss(e.measured[k], e.band[k]));
        assert!(then.iter().any(|&m| m > 0.0), "{:?} meets its oracle", e.walk);
        for k in 0..2 {
            if miss(grown[k], e.band[k]) > then[k] {
                worse.push(format!("{:?} at {} t {}: count {k} {}", e.walk, e.age, e.t, grown[k]));
            }
        }
    }
    assert!(worse.is_empty(), "{worse:#?}");
}
