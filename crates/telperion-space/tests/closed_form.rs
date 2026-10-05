//! R2: GreenLab's structural factorisation gives the organ counts per PA
//! and per cycle in closed form; deterministic trees grow exactly those.
mod common;
use common::{age, sets, species, tree};
use std::f64::consts::PI;
use telperion_space::{expected_counts, CountTable, Form, NodeLaw, PaState, Species, Zone};

fn state(lifespan: u32, next: Option<usize>, zones: Vec<Zone>) -> PaState {
    PaState {
        lifespan,
        next,
        viability: 1.0,
        zones,
        shedding: None,
        internode: 1.0,
        // Shallow enough that nine nested reiterations still rise.
        insertion: 0.1,
        divergence: PI / 2.0,
        abortion: 0.0,
        abortion_rise: 0.0,
        relay: 0.0,
        relay_at: 1.0,
        epitony: 0.0,
        erection: 0.0,
        readiness: 1.0,
        rhythm: 1.0,
        leaf_area: 0.0,
        shade_hazard: 0.0,
        shade_size: 0.0,
        straightening: 0.0,
        form: Form::default(),
    }
}

fn zone(nodes: u32, buds: u8, lateral: &[f64]) -> Zone {
    Zone {
        nodes: NodeLaw::Uniform {
            min: nodes,
            max: nodes,
        },
        buds,
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
                None,
                vec![zone(2, 1, &[0.0, 0.0, 1.0]), zone(1, 1, &[0.0, 1.0, 0.0])],
            ),
            state(2, Some(2), vec![zone(2, 1, &[0.0, 0.0, 1.0])]),
            state(2, None, vec![zone(1, 1, &[0.0, 0.0, 0.0])]),
        ],
    };
    let whorled = Species {
        states: vec![
            state(1, Some(0), vec![zone(1, 3, &[0.0, 1.0, 0.0])]),
            state(3, None, vec![zone(2, 2, &[0.0, 0.0, 1.0])]),
            state(1, Some(2), vec![zone(1, 1, &[0.0, 0.0, 0.0])]),
        ],
    };
    let reiterated = Species {
        states: vec![
            state(2, Some(1), vec![zone(1, 1, &[1.0, 0.0])]),
            state(3, None, vec![zone(2, 1, &[0.0, 0.0])]),
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
        let formula = expected_counts(&species(set), age(set)).unwrap();
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
            state(3, None, vec![zone(1, 1, &[0.0, 1.0])]),
            state(2, None, vec![zone(2, 1, &[0.0, 0.0])]),
        ],
    };
    let formula = expected_counts(&species, 3).unwrap();
    let rows: Vec<Vec<f64>> = (0..2)
        .map(|pa| (1..=3).map(|c| formula.get(pa, c)).collect())
        .collect();
    assert_eq!(rows, vec![vec![1.0, 1.0, 1.0], vec![0.0, 2.0, 4.0]]);
}
