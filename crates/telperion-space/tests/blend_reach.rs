//! fn-299 R2 and fn-308 R2 (host, 2026-10-22): between the ends of a walk,
//! for a tree of a given age, an age one end never reaches takes the other
//! end's every row, each lifespan past the tree's age is clamped to it, an
//! age one end leaves within the tree's age and the other does not takes
//! the leaving end's exit rows, and an age both ends reach mixes as
//! before. The palm lives age 2 alone and stops there; the oak goes on
//! through the ages after it.
use telperion_space::{beech, blend, most_units, oak, palm, spruce, PaState, Species};

const T: [f64; 4] = [0.0005, 0.5, 0.9, 0.97];
/// The film's walk from the oak to the palm is drawn at 40.
const AGE: u32 = 40;

/// The ages after the palm's stem that the oak lives.
fn past_the_palm(oak: &Species) -> Vec<usize> {
    (3..oak.states.len())
        .filter(|&k| oak.states[k].lifespan > 0.0)
        .collect()
}

/// Whether `frame` is `end` exactly, each role `end` leaves out standing in
/// as a zone of no nodes, as the blend matches zones by role.
fn is(frame: &PaState, end: &PaState) -> bool {
    let mut cut = frame.clone();
    let left_out = cut.zones.split_off(end.zones.len());
    cut == *end && left_out.iter().all(|z| z.nodes.mean() == 0.0)
}

/// An end's age as a tree of `AGE` lives it: its lifespan clamped there.
fn clamped(end: &PaState) -> PaState {
    let mut s = end.clone();
    s.lifespan = s.lifespan.min(f64::from(AGE));
    s
}

/// fn-308 rule B: every row of an age the palm never reaches, the zones
/// included, is the oak's exactly, its lifespan clamped at the tree's age, both ways along the walk; a lateral row
/// the palm's stem never draws reaches nothing. The palm's stem has no
/// readiness, so a lateral probability into age 3 leaves it unbranched
/// and the ages past it unreached.
#[test]
fn every_row_of_an_unreached_age_is_the_reaching_ends() {
    let o = oak();
    let ages = past_the_palm(&o);
    assert!(!ages.is_empty(), "the oak lives ages past the palm's stem");
    let mut drawn = palm();
    assert_eq!(drawn.states[2].readiness, 0.0, "the palm's stem is unready");
    drawn.states[2].zones[0].lateral[3] = 1.0;
    for p in [palm(), drawn] {
        for t in T {
            for (name, frame) in [
                ("oak to palm", blend(&o, &p, t, AGE).unwrap()),
                ("palm to oak", blend(&p, &o, 1.0 - t, AGE).unwrap()),
            ] {
                for &k in &ages {
                    let (got, oak) = (&frame.states[k], &clamped(&o.states[k]));
                    assert!(is(got, oak), "{name} at t {t}: age {k}\n{got:?}\n{oak:?}");
                }
            }
        }
    }
}

/// fn-308's exit rule: the oak leaves its age 2 within a tree of 40 (a
/// lifespan of 12) and the palm does not (150, clamped to 40), so between
/// the ends that age moves on as the oak's does. At 10, which neither end
/// leaves the age within, its continuation mixes.
#[test]
fn an_age_one_end_leaves_within_the_tree_takes_its_exits() {
    let (o, p) = (oak(), palm());
    let (x, y) = (&o.states[2], &p.states[2]);
    assert_eq!((x.lifespan, y.lifespan), (12.0, 150.0));
    assert_ne!(x.continuation, y.continuation);
    for t in T {
        let left = &blend(&o, &p, t, AGE).unwrap().states[2];
        let exits = (left.continuation, left.relay_ended);
        assert_eq!(exits, (x.continuation, x.relay_ended), "at t {t}");
        let mixed = blend(&o, &p, t, 10).unwrap().states[2].continuation;
        let (lo, hi) = (
            x.continuation.min(y.continuation),
            x.continuation.max(y.continuation),
        );
        assert!(lo < mixed && mixed < hi, "at t {t}, 10: {mixed}");
    }
}

#[test]
fn an_age_both_ends_reach_still_mixes() {
    // The palm's stem age, lived at both ends, lies strictly between them,
    // the palm's lifespan clamped at the tree's age.
    let (o, p) = (oak(), palm());
    let (x, y) = (
        o.states[2].lifespan,
        p.states[2].lifespan.min(f64::from(AGE)),
    );
    let mid = blend(&o, &p, 0.5, AGE).unwrap().states[2].lifespan;
    assert!(x.min(y) < mid && mid < x.max(y), "{x} {mid} {y}");
    // Ages the beech passes through and the spruce lives: still a sliver
    // near the beech, the pass-through case rule B leaves (R1.md).
    let (b, s) = (beech(), spruce());
    let near = blend(&b, &s, 0.0005, AGE).unwrap();
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
        let units = most_units(&blend(&o, &p, t, AGE).unwrap());
        assert!(units <= high, "t {t}: {units} past the ends {ends:?}");
    }
}
