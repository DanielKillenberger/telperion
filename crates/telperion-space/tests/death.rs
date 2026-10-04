//! Death and shedding: an apex that fails its viability stops, and a lateral
//! with no living apex is shed after its PA's delay.
mod common;
use common::{tree, Moments};
use std::f64::consts::PI;
use telperion_space::{expected_counts, Form, NodeLaw, PaState, Species, Zone};

fn state(
    lifespan: u32,
    viability: f64,
    nodes: NodeLaw,
    lateral: &[f64],
    shedding: Option<u32>,
) -> PaState {
    PaState {
        lifespan,
        next: None,
        viability,
        zones: vec![Zone {
            nodes,
            buds: 1,
            lateral: lateral.to_vec(),
        }],
        shedding,
        internode: 1.0,
        insertion: PI / 4.0,
        divergence: PI,
        abortion: 0.0,
        abortion_rise: 0.0,
        relay: 0.0,
        relay_at: 1.0,
        epitony: 0.0,
        erection: 0.0,
        readiness: 1.0,
        rhythm: 1.0,
        straightening: 0.0,
        form: Form::default(),
    }
}

const FIXED_ONE: NodeLaw = NodeLaw::Uniform { min: 1, max: 1 };

/// Mortal apices: the engine's mean over many seeds is the closed form's
/// expectation, whose survival factor is the viability per cycle.
#[test]
fn mortal_apices_grow_the_expected_counts() {
    let species = Species {
        states: vec![
            state(
                10,
                1.0,
                NodeLaw::Uniform { min: 1, max: 2 },
                &[0.0, 0.8, 0.0],
                None,
            ),
            state(
                4,
                0.7,
                NodeLaw::Uniform { min: 2, max: 2 },
                &[0.0, 0.0, 0.5],
                None,
            ),
            state(
                3,
                0.5,
                NodeLaw::Poisson { mean: 1.5 },
                &[0.0, 0.0, 0.0],
                None,
            ),
        ],
    };
    let age = 10;
    let formula = expected_counts(&species, age).unwrap();
    let tables: Vec<_> = (0..4000)
        .map(|seed| tree(&species, age, seed).counts())
        .collect();
    for pa in 0..3 {
        for cycle in 1..=age as usize {
            let values: Vec<f64> = tables.iter().map(|t| t.get(pa, cycle)).collect();
            let ours = Moments::of(&values);
            let (error, _) = ours.errors();
            let delta = (ours.mean - formula.get(pa, cycle)).abs();
            assert!(
                delta <= 4.5 * error + 1e-12,
                "PA {} cycle {cycle}: {delta} off",
                pa + 1
            );
        }
    }
    let immortal = Species {
        states: species
            .states
            .iter()
            .map(|s| PaState {
                viability: 1.0,
                ..s.clone()
            })
            .collect(),
    };
    let fewer = formula.total(2) < expected_counts(&immortal, age).unwrap().total(2);
    assert!(fewer, "death removes phytomers");
}

/// One PA 2 lateral a cycle, each growing one node for two cycles and shed
/// once it has had no living apex for more than one cycle: at age 6 the
/// laterals born in cycles 1 and 2 are gone.
#[test]
fn laterals_without_a_living_apex_are_shed_after_their_delay() {
    let grown = |shedding| {
        let species = Species {
            states: vec![
                state(6, 1.0, FIXED_ONE, &[0.0, 1.0], None),
                state(2, 1.0, FIXED_ONE, &[0.0, 0.0], shedding),
            ],
        };
        let counts = tree(&species, 6, 1).counts();
        (1..=6)
            .map(|cycle| counts.get(1, cycle))
            .collect::<Vec<_>>()
    };
    assert_eq!(grown(None), vec![0.0, 1.0, 2.0, 2.0, 2.0, 2.0]);
    assert_eq!(grown(Some(1)), vec![0.0, 0.0, 0.0, 1.0, 2.0, 2.0]);
}

#[test]
fn a_seed_grows_the_same_tree_every_time() {
    let species = Species {
        states: vec![
            state(8, 1.0, NodeLaw::Poisson { mean: 2.0 }, &[0.0, 0.6], None),
            state(
                3,
                0.8,
                NodeLaw::Uniform { min: 1, max: 3 },
                &[0.0, 0.0],
                Some(2),
            ),
        ],
    };
    assert_eq!(tree(&species, 8, 42), tree(&species, 8, 42));
    assert_ne!(tree(&species, 8, 42), tree(&species, 8, 43));
}

/// A relay that lives keeps the lateral it relays: a parent whose apex
/// died is not shed while its relay grows.
#[test]
fn a_living_relay_keeps_its_parent() {
    let mut relaying = state(4, 0.5, FIXED_ONE, &[0.0, 0.0], Some(0));
    relaying.relay = 1.0;
    relaying.insertion = 0.0;
    let species = Species {
        states: vec![state(8, 1.0, FIXED_ONE, &[0.0, 1.0], None), relaying],
    };
    for seed in 0..64 {
        let grown = tree(&species, 4, seed);
        for axis in grown.axes.iter().filter(|a| a.apex_end.is_none()) {
            for phytomer in &axis.phytomers {
                assert!(
                    phytomer.scale > 0.0,
                    "seed {seed}: a living {:?} drawn at no size",
                    axis.origin
                );
            }
        }
    }
}

/// The fade floor follows a living apex's presence, so an abortion decided
/// in the last cycle unmakes its branch by degree (the review's crossing).
#[test]
fn an_abortion_in_the_last_cycle_fades_by_degree() {
    let one = |lifespan, lateral: &[f64], shedding| {
        let mut s = state(lifespan, 1.0, FIXED_ONE, lateral, shedding);
        s.insertion = 0.0;
        s
    };
    let at = |abortion| {
        let mut twig = one(4, &[0.0, 0.0, 0.0], None);
        twig.abortion = abortion;
        let species = Species {
            states: vec![
                one(1, &[0.0, 1.0, 0.0], None),
                one(1, &[0.0, 0.0, 0.07884088684532432], Some(0)),
                twig,
            ],
        };
        let grown = tree(&species, 4, 24);
        grown
            .axes
            .iter()
            .flat_map(|a| &a.phytomers)
            .map(|p| p.scale)
            .sum::<f64>()
    };
    let (before, after) = (at(0.5866100885647014), at(0.5866100887647014));
    assert!((before - after).abs() < 1e-6, "{before} -> {after}");
}
