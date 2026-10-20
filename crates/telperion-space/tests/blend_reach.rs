//! fn-299 R2 (host, 2026-10-20, 2026-10-21): between the ends of a walk,
//! an age one end never reaches takes the other end's timing rows, its
//! other rows mix, and an age both ends reach mixes as before. The palm lives age 2 alone and stops there; the
//! oak goes on through the ages after it.
use telperion_space::{beech, blend, most_units, oak, palm, settings, spruce, PaState, Species};

const T: [f64; 4] = [0.0005, 0.5, 0.9, 0.97];

/// The ages after the palm's stem that the oak lives.
fn past_the_palm(oak: &Species) -> Vec<usize> {
    (3..oak.states.len())
        .filter(|&k| oak.states[k].lifespan > 0.0)
        .collect()
}

/// The rows that decide whether and for how long a bud lives an age.
const TIMING: [&str; 8] = [
    "lifespan",
    "continuation",
    "abortion",
    "abortion_rise",
    "relay",
    "relay_ended",
    "viability",
    "relay_failed",
];

/// An age's timing rows, by name.
fn timing(s: &PaState) -> Vec<(&'static str, f64)> {
    let mut s = s.clone();
    settings(&mut s)
        .into_iter()
        .filter(|(n, _)| TIMING.contains(n))
        .map(|(n, v)| (n, *v))
        .collect()
}

/// Whether two ages share their timing rows.
fn same(a: &PaState, b: &PaState) -> bool {
    timing(a) == timing(b)
}

#[test]
fn an_age_one_end_never_reaches_keeps_the_others_timing() {
    let (o, p) = (oak(), palm());
    let ages = past_the_palm(&o);
    assert!(!ages.is_empty(), "the oak lives ages past the palm's stem");
    for t in T {
        // Both directions of the walk, at the same point.
        for (name, frame) in [
            ("oak to palm", blend(&o, &p, t).unwrap()),
            ("palm to oak", blend(&p, &o, 1.0 - t).unwrap()),
        ] {
            for &k in &ages {
                assert!(
                    same(&frame.states[k], &o.states[k]),
                    "{name} at t {t}: age {k}"
                );
            }
        }
    }
}

/// A lateral row the axis never draws reaches nothing: the palm's stem has
/// no readiness, so a lateral probability into age 3 leaves it unbranched
/// and the ages past it unreached.
#[test]
fn a_lateral_the_axis_never_draws_reaches_no_age() {
    let o = oak();
    let mut p = palm();
    assert_eq!(p.states[2].readiness, 0.0, "the palm's stem is unready");
    p.states[2].zones[0].lateral[3] = 1.0;
    for t in T {
        for (name, frame) in [
            ("oak to palm", blend(&o, &p, t).unwrap()),
            ("palm to oak", blend(&p, &o, 1.0 - t).unwrap()),
        ] {
            for &k in &past_the_palm(&o) {
                assert!(
                    same(&frame.states[k], &o.states[k]),
                    "{name} at t {t}: age {k}"
                );
            }
        }
    }
}

/// The rest of an age one end never reaches mixes: oak to palm, the oak's
/// internode at its ages past the palm's stem lies between the ends' rows.
#[test]
fn the_rest_of_an_unreached_age_still_mixes() {
    let (o, p) = (oak(), palm());
    let frame = blend(&o, &p, 0.5).unwrap();
    let moved: Vec<usize> = past_the_palm(&o)
        .into_iter()
        .filter(|&k| o.states[k].internode != p.states[k].internode)
        .collect();
    assert!(!moved.is_empty(), "an age whose internode differs");
    for k in moved {
        let (x, y, m) = (
            o.states[k].internode,
            p.states[k].internode,
            frame.states[k].internode,
        );
        assert!(x.min(y) < m && m < x.max(y), "age {k}: {x} {m} {y}");
    }
}

#[test]
fn an_age_both_ends_reach_still_mixes() {
    // The palm's stem age, lived at both ends, lies strictly between them.
    let (o, p) = (oak(), palm());
    let (x, y) = (o.states[2].lifespan, p.states[2].lifespan);
    let mid = blend(&o, &p, 0.5).unwrap().states[2].lifespan;
    assert!(x.min(y) < mid && mid < x.max(y), "{x} {mid} {y}");
    // Ages the beech passes through and the spruce lives: still a sliver
    // near the beech, the pass-through case rule B leaves (R1.md).
    let (b, s) = (beech(), spruce());
    let near = blend(&b, &s, 0.0005).unwrap();
    let passed: Vec<usize> = (0..b.states.len())
        .filter(|&k| b.states[k].lifespan == 0.0 && s.states[k].lifespan > 0.0)
        .collect();
    let sliver = |k: &usize| (0.0..0.01).contains(&near.states[*k].lifespan);
    assert!(
        passed
            .iter()
            .any(|k| sliver(k) && near.states[*k].lifespan > 0.0),
        "a pass-through age mixes from 0: {passed:?}"
    );
}

#[test]
fn oak_to_palm_grows_within_its_ends() {
    let (o, p) = (oak(), palm());
    let ends = [most_units(&o), most_units(&p)];
    let high = ends[0].max(ends[1]);
    for t in T {
        let units = most_units(&blend(&o, &p, t).unwrap());
        assert!(units <= high, "t {t}: {units} past the ends {ends:?}");
    }
}
