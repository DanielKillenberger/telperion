//! R2: GreenLab's structural factorisation gives the organ counts per PA
//! and per cycle in closed form; deterministic trees grow exactly those.
mod common;
use common::{age, sets, species, tree};
use std::f64::consts::PI;
use telperion_space::{expected_counts, CountTable, Form, NodeLaw, PaState, Species, Zone};

fn state(lifespan: u32, continuation: f64, zones: Vec<Zone>) -> PaState {
    PaState {
        lifespan: f64::from(lifespan),
        continuation,
        viability: 1.0,
        zones,
        shedding: f64::INFINITY,
        internode: 1.0,
        // Shallow enough that nine nested reiterations still rise.
        insertion: 0.1,
        divergence: PI / 2.0,
        abortion: 0.0,
        abortion_rise: 0.0,
        relay: 0.0,
        relay_ended: 0.0,
        relay_failed: 0.0,
        relay_at: 1.0,
        epitony: 0.0,
        erection: 0.0,
        readiness: 1.0,
        rhythm: 1.0,
        leaf_area: 0.0,
        shade_hazard: 0.0,
        shade_size: 0.0,
        apical_control: 0.5,
        upkeep: 0.0,
        balance_hazard: 0.0,
        tolerance: 0.0,
        retained: 0.0,
        leaf_girth: 0.0,
        straightening: 0.0,
        form: Form::default(),
    }
}

fn zone(nodes: u32, buds: u8, lateral: &[f64]) -> Zone {
    Zone {
        nodes: NodeLaw::Uniform {
            min: f64::from(nodes),
            max: f64::from(nodes),
        },
        buds: f64::from(buds),
        dormant: vec![0.0; lateral.len()],
        delay: 0.0,
        rate: 0.0,
        lateral: lateral.to_vec(),
    }
}

/// Three reference axes beyond the simulators' reach: zones that set the
/// lateral PA by position, whorls with a self-renewing apex, reiteration.
fn parameter_sets() -> Vec<(&'static str, Species, u32)> {
    let zoned = Species {
        states: vec![
            state(
                6,
                0.0,
                vec![zone(2, 1, &[0.0, 0.0, 1.0]), zone(1, 1, &[0.0, 1.0, 0.0])],
            ),
            state(2, 1.0, vec![zone(2, 1, &[0.0, 0.0, 1.0])]),
            state(2, 0.0, vec![zone(1, 1, &[0.0, 0.0, 0.0])]),
        ],
    };
    let whorled = Species {
        states: vec![
            // A self-renewing apex is one that grows on past the tree's
            // age: the reference axis has no loops (fn-206).
            state(100, 0.0, vec![zone(1, 3, &[0.0, 1.0, 0.0])]),
            state(3, 0.0, vec![zone(2, 2, &[0.0, 0.0, 1.0])]),
            state(100, 0.0, vec![zone(1, 1, &[0.0, 0.0, 0.0])]),
        ],
    };
    let reiterated = Species {
        states: vec![
            state(2, 1.0, vec![zone(1, 1, &[1.0, 0.0])]),
            state(3, 0.0, vec![zone(2, 1, &[0.0, 0.0])]),
        ],
    };
    vec![
        ("zoned", zoned, 8),
        ("whorled", whorled, 7),
        ("reiterated", reiterated, 9),
    ]
}

fn assert_equal(label: &str, ours: &CountTable, formula: &CountTable) {
    assert_eq!(
        (ours.pas, ours.cycles),
        (formula.pas, formula.cycles),
        "{label}"
    );
    for pa in 0..ours.pas {
        for cycle in 1..=ours.cycles {
            let (got, want) = (ours.get(pa, cycle), formula.get(pa, cycle));
            assert_eq!(got, want, "{label}: PA {} cycle {cycle}", pa + 1);
        }
    }
}

#[test]
fn deterministic_trees_grow_the_closed_form_counts() {
    for (name, species, age) in parameter_sets() {
        let formula = expected_counts(&species, age).unwrap();
        assert!(
            formula.total(0) + formula.total(1) > 20.0,
            "{name} grows a real tree"
        );
        assert_equal(name, &tree(&species, age, 7).counts(), &formula);
    }
}

#[test]
fn the_closed_form_counts_the_simulators_deterministic_trees() {
    for set in sets().iter().filter(|s| s["deterministic"] == true) {
        let formula = common::fold(set, &expected_counts(&species(set), age(set)).unwrap());
        for (pa, row) in set["counts"].as_array().unwrap().iter().enumerate() {
            for (c, count) in row.as_array().unwrap().iter().enumerate() {
                assert_eq!(
                    formula.get(pa, c + 1),
                    count.as_f64().unwrap(),
                    "{}",
                    set["name"]
                );
            }
        }
    }
}

/// A bud of PA 1 grows one node a cycle, each bearing a PA 2 bud that grows
/// two nodes a cycle for two cycles: by hand, PA 1 is 1, 1, 1 and PA 2 is
/// 0, 2 (the first bud), 4 (both).
#[test]
fn the_closed_form_matches_a_count_by_hand() {
    let species = Species {
        states: vec![
            state(3, 0.0, vec![zone(1, 1, &[0.0, 1.0])]),
            state(2, 0.0, vec![zone(2, 1, &[0.0, 0.0])]),
        ],
    };
    let formula = expected_counts(&species, 3).unwrap();
    let rows: Vec<Vec<f64>> = (0..2)
        .map(|pa| (1..=3).map(|c| formula.get(pa, c)).collect())
        .collect();
    assert_eq!(rows, vec![vec![1.0, 1.0, 1.0], vec![0.0, 2.0, 4.0]]);
}

/// A fractional bud place grows whole, at its share of the size (fn-206):
/// the closed form counts every place, as the grower grows it.
#[test]
fn a_fractional_bud_place_is_counted_whole() {
    let mut first = state(1, 1.0, vec![zone(1, 1, &[0.0, 1.0])]);
    first.zones[0].buds = 1.5;
    let species = Species {
        states: vec![first, state(1, 0.0, vec![zone(1, 1, &[0.0, 0.0])])],
    };
    let ours = tree(&species, 3, 1).counts();
    let formula = expected_counts(&species, 3).unwrap();
    assert_equal("fractional buds", &ours, &formula);
}

/// Host decision 13: an apex that all but stopped moving on grows its
/// continuation and its relay, and the closed form counts the relay
/// through the window it grows in: the engine's mean over 20,000 seeds is
/// its expectation in every cell.
#[test]
fn the_closed_form_counts_the_relay_of_a_move_all_but_made() {
    let mut first = state(1, 0.5, vec![zone(1, 1, &[0.0, 0.0])]);
    (first.relay, first.relay_ended, first.relay_failed) = (0.5, 0.5, 0.5);
    let species = Species {
        states: vec![first, state(1, 0.0, vec![zone(1, 1, &[0.0, 0.0])])],
    };
    for age in [2, 4] {
        let formula = expected_counts(&species, age).unwrap();
        let tables: Vec<_> = (0..20_000)
            .map(|seed| tree(&species, age, seed).counts())
            .collect();
        for pa in 0..2 {
            for cycle in 1..=age as usize {
                let values: Vec<f64> = tables.iter().map(|t| t.get(pa, cycle)).collect();
                let ours = common::Moments::of(&values);
                let delta = (ours.mean - formula.get(pa, cycle)).abs();
                assert!(
                    delta <= 4.5 * ours.errors().0 + 1e-12,
                    "age {age}, PA {} cycle {cycle}: {} vs {}",
                    pa + 1,
                    ours.mean,
                    formula.get(pa, cycle)
                );
            }
        }
    }
}

/// The engine's mean over `n` seeds is the closed form's in every cell.
fn means_agree(label: &str, species: &Species, age: u32, n: u64) {
    let formula = expected_counts(species, age).unwrap();
    let tables: Vec<_> = (0..n)
        .map(|seed| tree(species, age, seed).counts())
        .collect();
    for pa in 0..species.states.len() {
        for cycle in 1..=age as usize {
            let values: Vec<f64> = tables.iter().map(|t| t.get(pa, cycle)).collect();
            let ours = common::Moments::of(&values);
            let delta = (ours.mean - formula.get(pa, cycle)).abs();
            assert!(
                delta <= 4.5 * ours.errors().0 + 1e-9,
                "{label}: PA {} cycle {cycle}: {} vs {}",
                pa + 1,
                ours.mean,
                formula.get(pa, cycle)
            );
        }
    }
}

/// A relay grown in the cycle its age ended in draws apart from the move
/// that failed there (Codex round 3 on fn-206): an age of half a cycle,
/// moving on at even odds and relayed when it does not.
#[test]
fn a_relay_in_its_axis_cycle_draws_its_own_move() {
    let mut first = state(1, 0.5, vec![zone(1, 1, &[0.0, 0.0])]);
    first.lifespan = 0.5;
    first.relay_ended = 1.0;
    let species = Species {
        states: vec![first, state(3, 0.0, vec![zone(1, 1, &[0.0, 0.0])])],
    };
    means_agree("same-cycle relay", &species, 3, 20_000);
}

/// A bud that wakes into a stage with less of its lifespan left than of
/// its waking cycle runs its first unit's risks over the share its
/// lifespan leaves (Codex round 3 on fn-206).
#[test]
fn a_bud_woken_late_in_a_short_stage_runs_its_risks_over_its_share() {
    let mut bearer = state(5, 0.0, vec![zone(1, 1, &[0.0, 0.0])]);
    bearer.zones[0].dormant = vec![0.0, 1.0];
    (bearer.zones[0].delay, bearer.zones[0].rate) = (0.0, 1.0);
    let mut short = state(1, 0.0, vec![zone(1, 1, &[0.0, 0.0])]);
    (short.lifespan, short.viability) = (0.25, 0.5);
    let species = Species {
        states: vec![bearer, short],
    };
    means_agree("short stage", &species, 3, 20_000);
}

/// An age that ends just past a whole cycle relays as one that ends on
/// it (Codex round 4 on fn-206): its relay's stop decisions are drawn
/// apart whether it grows in its axis's cycle or from the next.
#[test]
fn an_ended_relay_grows_alike_either_side_of_a_whole_lifespan() {
    let length = |lifespan: f64, seed: u64| {
        let mut only = state(1, 0.0, vec![zone(1, 1, &[0.0])]);
        only.lifespan = lifespan;
        only.relay_ended = 1.0;
        only.zones[0].nodes = NodeLaw::Uniform { min: 1.0, max: 2.0 };
        let species = Species { states: vec![only] };
        tree(&species, 2, seed)
            .axes
            .iter()
            .flat_map(|a| &a.phytomers)
            .map(|p| p.scale)
            .sum::<f64>()
    };
    for seed in 1..=8 {
        let (on, past) = (length(1.0, seed), length(1.0 + 1e-9, seed));
        assert!((on - past).abs() < 1e-6, "seed {seed}: {on} vs {past}");
    }
}

/// A sleeping bud that wakes into a stage it cannot survive grows
/// nothing, and the closed form counts nothing (Codex round 4 on fn-206).
#[test]
fn a_bud_woken_into_a_stage_it_cannot_survive_counts_nothing() {
    let mut bearer = state(5, 0.0, vec![zone(1, 1, &[0.0, 0.0])]);
    bearer.zones[0].dormant = vec![0.0, 1.0];
    (bearer.zones[0].delay, bearer.zones[0].rate) = (0.0, 1.0);
    let mut short = state(1, 0.0, vec![zone(1, 1, &[0.0, 0.0])]);
    (short.lifespan, short.viability) = (0.25, 0.0);
    let species = Species {
        states: vec![bearer, short],
    };
    let formula = expected_counts(&species, 3).unwrap();
    for cycle in 1..=3 {
        assert_eq!(formula.get(1, cycle), 0.0, "cycle {cycle}");
    }
}

/// A sleeping bud that wakes into a stage past one it reaches at a share
/// grows whole at that share of its size, and is counted whole (Codex
/// round 5 on fn-206).
#[test]
fn a_bud_reaching_its_stage_in_part_is_counted_whole() {
    let mut bearer = state(5, 0.0, vec![zone(1, 1, &[0.0, 0.0, 0.0])]);
    bearer.zones[0].dormant = vec![0.0, 1.0, 0.0];
    (bearer.zones[0].delay, bearer.zones[0].rate) = (1.2, 100.0);
    let passing = state(1, 0.5, vec![zone(1, 1, &[0.0, 0.0, 0.0])]);
    let after = state(100, 0.0, vec![zone(1, 1, &[0.0, 0.0, 0.0])]);
    let species = Species {
        states: vec![bearer, passing, after],
    };
    means_agree("reach", &species, 4, 4_000);
}

/// A bearer whose age ends within the cycle a bud wakes in bears it as
/// far as it lived past the waking, so its lifespan crossing a whole cycle
/// moves the tree by degree (Codex round 5 on fn-206).
#[test]
fn a_bearer_ending_as_a_bud_wakes_moves_it_by_degree() {
    let length = |lifespan: f64| {
        let mut bearer = state(3, 0.0, vec![zone(1, 1, &[0.0, 0.0])]);
        bearer.lifespan = lifespan;
        bearer.zones[0].dormant = vec![0.0, 1.0];
        (bearer.zones[0].delay, bearer.zones[0].rate) = (1.5, 100.0);
        let woken = state(10, 0.0, vec![zone(1, 1, &[0.0, 0.0])]);
        let species = Species {
            states: vec![bearer, woken],
        };
        let tree = tree(&species, 3, 1);
        tree.axes
            .iter()
            .filter(|a| a.pa == 1)
            .flat_map(|a| &a.phytomers)
            .map(|p| p.scale)
            .sum::<f64>()
    };
    let (below, above) = (length(3.0 - 1e-9), length(3.0 + 1e-9));
    assert!((below - above).abs() < 1e-6, "{below} vs {above}");
    let mut bearer = state(3, 0.0, vec![zone(1, 1, &[0.0, 0.0])]);
    bearer.lifespan = 2.7;
    bearer.zones[0].dormant = vec![0.0, 1.0];
    (bearer.zones[0].delay, bearer.zones[0].rate) = (1.5, 2.0);
    let species = Species {
        states: vec![bearer, state(10, 0.0, vec![zone(1, 1, &[0.0, 0.0])])],
    };
    means_agree("ending bearer", &species, 4, 4_000);
}

/// The same in the tree's last cycle: a bearer whose age ends just after
/// a whole cycle, or just after the tree's age, keeps its bud as one
/// ending just before it does (Codex round 6 on fn-206).
#[test]
fn a_bearer_ending_in_the_last_cycle_moves_its_bud_by_degree() {
    let length = |lifespan: f64| {
        let mut bearer = state(3, 0.0, vec![zone(1, 1, &[0.0, 0.0])]);
        bearer.lifespan = lifespan;
        bearer.zones[0].dormant = vec![0.0, 1.0];
        (bearer.zones[0].delay, bearer.zones[0].rate) = (1.5, 100.0);
        let woken = state(10, 0.0, vec![zone(1, 1, &[0.0, 0.0])]);
        let species = Species {
            states: vec![bearer, woken],
        };
        tree(&species, 3, 1)
            .axes
            .iter()
            .filter(|a| a.pa == 1)
            .flat_map(|a| &a.phytomers)
            .map(|p| p.scale)
            .sum::<f64>()
    };
    for whole in [2.0, 3.0] {
        let (below, above) = (length(whole - 1e-9), length(whole + 1e-9));
        assert!((below - above).abs() < 1e-6, "{whole}: {below} vs {above}");
    }
}

/// An age's end is drawn once for the age, whatever cycle it falls in, its
/// window read at the moment it ends: a lifespan crossing a whole cycle
/// moves the tree by degree where it moves on at even odds (Codex round 7
/// on fn-206).
#[test]
fn an_age_ending_either_side_of_a_whole_cycle_moves_on_alike() {
    let length = |lifespan: f64, seed: u64| {
        let mut first = state(1, 0.5, vec![zone(1, 1, &[0.0, 0.0])]);
        first.lifespan = lifespan;
        let species = Species {
            states: vec![first, state(5, 0.0, vec![zone(1, 1, &[0.0, 0.0])])],
        };
        tree(&species, 3, seed)
            .axes
            .iter()
            .flat_map(|a| &a.phytomers)
            .map(|p| p.scale)
            .sum::<f64>()
    };
    for seed in 1..=8 {
        let (below, above) = (length(1.0 - 1e-9, seed), length(1.0 + 1e-9, seed));
        assert!(
            (below - above).abs() < 1e-6,
            "seed {seed}: {below} vs {above}"
        );
    }
}

/// An age of no lifespan is passed through and moves on: one that would
/// stop is refused, never drawn another way than an age all but as short
/// (Codex round 7 on fn-206).
#[test]
fn an_age_of_no_lifespan_that_stops_is_refused() {
    let mut passed = state(1, 0.5, vec![zone(1, 1, &[0.0, 0.0])]);
    passed.lifespan = 0.0;
    let species = Species {
        states: vec![passed, state(5, 0.0, vec![zone(1, 1, &[0.0, 0.0])])],
    };
    assert!(expected_counts(&species, 3).is_err());
}

/// A straightened lateral whose age ends just past a whole cycle hands its
/// next age the frame one ending just before it does: the continuation
/// runs on in the direction its parent ends on, where its pull has faded
/// (Codex round 8 on fn-206).
#[test]
fn a_straightened_lateral_changes_age_alike_either_side_of_a_whole_cycle() {
    let tips = |lifespan: f64| {
        let seed = state(1, 0.0, vec![zone(1, 1, &[0.0, 1.0, 0.0])]);
        let mut lateral = state(1, 1.0, vec![zone(1, 1, &[0.0, 0.0, 0.0])]);
        lateral.lifespan = lifespan;
        (lateral.insertion, lateral.straightening) = (PI / 2.0, 1.0);
        let after = state(10, 0.0, vec![zone(1, 1, &[0.0, 0.0, 0.0])]);
        let species = Species {
            states: vec![seed, lateral, after],
        };
        tree(&species, 4, 1)
            .axes
            .iter()
            .filter_map(|a| a.phytomers.last().map(|p| p.tip))
            .collect::<Vec<_>>()
    };
    let (below, above) = (tips(1.0 - 1e-9), tips(1.0 + 1e-9));
    let far =
        |tips: &[telperion_space::Vec3]| tips.iter().map(|t| t.x.hypot(t.y)).fold(0.0, f64::max);
    let top = |tips: &[telperion_space::Vec3]| tips.iter().map(|t| t.z).fold(0.0, f64::max);
    assert!(
        (far(&below) - far(&above)).abs() < 1e-6,
        "{} vs {}",
        far(&below),
        far(&above)
    );
    assert!(
        (top(&below) - top(&above)).abs() < 1e-6,
        "{} vs {}",
        top(&below),
        top(&above)
    );
}
