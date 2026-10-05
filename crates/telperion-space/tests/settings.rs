//! fn-192 R3: discrete botany as continuous settings, each with a neutral
//! value and dormant where its structure is absent: sympodial growth an
//! abortion rate, rhythm a strength, Corner's unbranched stem branching
//! readiness at zero, relay readiness and base straightening for Troll.
mod common;
mod walk;
use common::Moments;
use telperion_space::{expected_counts, grow, Origin, Request, Species};

fn grown(species: &Species, seed: u64) -> telperion_space::Structure {
    walk::tree(species, seed).unwrap()
}

/// Each setting set where its structure is absent leaves the tree as it
/// was, to the bit: rhythm on a growth unit of one zone, relay on an apex
/// that never stops, straightening on the seed, and the neutral values.
#[test]
fn a_setting_is_dormant_where_its_structure_is_absent() {
    type Edit = fn(&mut Species);
    let cases: [(&str, Edit); 6] = [
        ("rhythm, one zone", |s| s.states[1].rhythm = 0.0),
        ("relay, an apex that never stops", |s| {
            s.states[0].relay = 1.0
        }),
        ("straightening, the seed", |s| {
            s.states[0].straightening = 1.0
        }),
        ("abortion, neutral", |s| s.states[0].abortion = 0.0),
        ("readiness, neutral", |s| s.states[2].readiness = 1.0),
        ("straightening, neutral", |s| {
            s.states[2].straightening = 0.0
        }),
    ];
    for seed in [1, 2] {
        let before = grown(&walk::species(), seed);
        for (name, edit) in cases {
            let mut species = walk::species();
            edit(&mut species);
            assert!(grown(&species, seed) == before, "{name}");
        }
    }
}

/// Readiness at zero is Corner's model: one unbranched stem.
#[test]
fn no_readiness_is_an_unbranched_stem() {
    let mut species = walk::species();
    species.states[0].readiness = 0.0;
    let tree = grown(&species, 1);
    assert_eq!(tree.axes.len(), 1, "one axis");
    assert_eq!(
        tree.axes[0].phytomers.len(),
        grown(&walk::species(), 1).axes[0].phytomers.len()
    );
}

/// Abortion with a rising hazard, relays, readiness and rhythm away from
/// neutral: the engine's mean over many seeds is the closed form's
/// expectation.
#[test]
fn the_settings_grow_the_expected_counts() {
    let mut species = walk::species();
    species.states[0].rhythm = 0.4;
    species.states[0].readiness = 0.7;
    species.states[1].abortion = 0.3;
    species.states[1].relay = 0.6;
    species.states[1].abortion_rise = 1.5;
    species.states[2].abortion = 0.2;
    species.states[2].relay = 0.5;
    species.states[2].shedding = None;
    // Shallow insertions keep chains of relays above the ground.
    for state in &mut species.states {
        state.insertion = state.insertion.min(0.15);
    }
    let age = 8;
    let formula = expected_counts(&species, age).unwrap();
    let tables: Vec<_> = (0..4000)
        .map(|seed| common::tree(&species, age, seed).counts())
        .collect();
    for pa in 0..3 {
        for cycle in 1..=age as usize {
            let values: Vec<f64> = tables.iter().map(|t| t.get(pa, cycle)).collect();
            let ours = Moments::of(&values);
            let delta = (ours.mean - formula.get(pa, cycle)).abs();
            assert!(
                delta <= 4.5 * ours.errors().0 + 1e-12,
                "PA {} cycle {cycle}: {delta} off",
                pa + 1
            );
        }
    }
    let relays = grown(&species, 1)
        .axes
        .iter()
        .filter(|a| matches!(a.origin, Origin::Relay { .. }))
        .count();
    assert!(relays > 0, "stopped apices relay");
}

/// Straightening lifts a lateral's base to the vertical and leaves its tip
/// on its heading.
#[test]
fn straightening_lifts_a_laterals_base() {
    let mut species = walk::species();
    species.states[1].straightening = 1.0;
    let tree = grow(
        &species,
        Request {
            age: 12,
            seed: 1,
            budget: 2_000_000,
            light: telperion_space::Light::NEUTRAL,
        },
    )
    .unwrap();
    let limb = tree
        .axes
        .iter()
        .filter(|a| a.pa == 1 && matches!(a.origin, Origin::Lateral { .. }) && a.heading.z > 0.0)
        .max_by_key(|a| a.phytomers.len())
        .unwrap();
    let rise = |from: telperion_space::Vec3, to: telperion_space::Vec3| {
        let run = to - from;
        run.z / run.length()
    };
    let first = rise(limb.base, limb.phytomers[0].tip);
    let n = limb.phytomers.len();
    let last = rise(limb.phytomers[n - 2].tip, limb.phytomers[n - 1].tip);
    assert!(first > 0.95, "the base stands nearly upright: {first}");
    assert!(
        (last - limb.heading.z).abs() < 0.2,
        "the tip keeps its heading: {last} vs {}",
        limb.heading.z
    );
}
