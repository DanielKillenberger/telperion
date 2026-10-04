//! fn-202: sleeping buds wake along old branches. Neutral where no bud
//! sleeps or none wakes within the tree's age (R1); every draw keyed to its
//! lineage, so a sleeping bud added anywhere moves no other (R2); and the
//! closed form's expectation the engine's mean (R4).
mod common;
mod walk;
use common::Moments;
use std::collections::HashMap;
use telperion_space::{expected_counts, Origin, Species, Structure};

fn grown(species: &Species, seed: u64) -> Structure {
    walk::tree(species, seed).unwrap()
}

/// R1: a release law with no sleeping bud, sleeping buds that never wake,
/// or that wake only past the tree's age, leave it as it was, to the bit.
#[test]
fn sleeping_buds_are_dormant_where_none_wakes() {
    type Edit = fn(&mut Species);
    let cases: [(&str, Edit); 3] = [
        ("a release law, no sleeping bud", |s| {
            walk::sleeping(s);
            for state in &mut s.states {
                for zone in &mut state.zones {
                    zone.dormant.fill(0.0);
                }
            }
        }),
        ("sleeping buds at rate 0", |s| {
            walk::sleeping(s);
            s.states[1].zones[0].rate = 0.0;
            s.states[0].zones[1].rate = 0.0;
        }),
        ("sleeping buds past the tree's age", |s| {
            walk::sleeping(s);
            s.states[1].zones[0].delay = f64::from(walk::AGE);
            s.states[0].zones[1].delay = f64::from(walk::AGE);
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

/// Each axis by its structural path: a sleeping bud that woke is marked
/// apart from the bud that grew at once in the same place.
fn by_path(tree: &Structure) -> HashMap<String, usize> {
    let mut paths: Vec<String> = Vec::with_capacity(tree.axes.len());
    for axis in &tree.axes {
        let path = match axis.origin {
            Origin::Seed => "s".to_string(),
            Origin::Lateral {
                parent,
                node,
                slot,
                woken,
                ..
            } => {
                let mark = if woken { "w" } else { "" };
                format!("{}/{node}.{slot}{mark}", paths[parent])
            }
            Origin::Continuation { parent } => format!("{}/c", paths[parent]),
            Origin::Relay { parent, .. } => format!("{}/r", paths[parent]),
        };
        paths.push(path);
    }
    paths.into_iter().enumerate().map(|(i, p)| (p, i)).collect()
}

/// R2: raising the limbs' sleeping probability adds woken buds and moves
/// no other draw: every axis keeps its PA and the cycles it grew in.
#[test]
fn adding_a_sleeping_bud_moves_no_other_draw() {
    let at = |p: f64| {
        let mut s = walk::species();
        walk::sleeping(&mut s);
        s.states[1].zones[0].dormant[2] = p;
        s
    };
    let mut added = 0;
    for seed in 1..=4 {
        for step in 0..6 {
            let p = 0.1 + 0.08 * f64::from(step);
            let (fewer, more) = (grown(&at(p), seed), grown(&at(p + 0.08), seed));
            let ours = by_path(&more);
            added += more.axes.len() - fewer.axes.len();
            for (path, &i) in &by_path(&fewer) {
                let Some(&j) = ours.get(path) else {
                    panic!("seed {seed}, p {p:.2}: {path} vanished");
                };
                let (a, b) = (&fewer.axes[i], &more.axes[j]);
                assert_eq!(a.pa, b.pa, "seed {seed}, p {p:.2}: {path} changed its PA");
                let cycles = |x: &telperion_space::Axis| {
                    x.phytomers.iter().map(|ph| ph.cycle).collect::<Vec<_>>()
                };
                assert_eq!(cycles(a), cycles(b), "seed {seed}, p {p:.2}: {path} regrew");
            }
        }
    }
    assert!(added > 100, "the walk wakes buds ({added})");
}

/// R4: with sleeping buds on limbs that abort, relay and die, the engine's
/// mean over 4,000 seeds is the closed form's expectation in every cell.
#[test]
fn sleeping_buds_grow_the_expected_counts() {
    let mut species = walk::species();
    walk::sleeping(&mut species);
    species.states[1].abortion = 0.3;
    species.states[1].relay = 0.6;
    species.states[2].abortion = 0.2;
    species.states[2].relay = 0.5;
    species.states[2].shedding = None;
    // Woken twigs along the twigs too, so a bearer that relays carries them.
    let twig = &mut species.states[2].zones[0];
    twig.dormant[2] = 0.4;
    (twig.delay, twig.rate) = (0.5, 0.8);
    // Shallow insertions keep chains of relays above the ground.
    for state in &mut species.states {
        state.insertion = state.insertion.min(0.15);
    }
    let age = 9;
    let formula = expected_counts(&species, age).unwrap();
    let tables: Vec<_> = (0..4000)
        .map(|seed| common::tree(&species, age, seed).counts())
        .collect();
    let woken = common::tree(&species, age, 1)
        .axes
        .iter()
        .filter(|a| matches!(a.origin, Origin::Lateral { woken: true, .. }))
        .count();
    assert!(woken > 0, "buds wake");
    for pa in 0..3 {
        for cycle in 1..=age as usize {
            let values: Vec<f64> = tables.iter().map(|t| t.get(pa, cycle)).collect();
            let ours = Moments::of(&values);
            let delta = (ours.mean - formula.get(pa, cycle)).abs();
            assert!(
                delta <= 4.5 * ours.errors().0 + 1e-12,
                "PA {} cycle {cycle}: {delta} off ({} vs {})",
                pa + 1,
                ours.mean,
                formula.get(pa, cycle)
            );
        }
    }
}
