//! R1's minimal geometry: internodes of their PA's length, each lateral at
//! its PA's insertion angle to its parent, successive nodes turned by the
//! divergence.
use std::f64::consts::PI;
use telperion_space::{grow, Form, NodeLaw, Origin, PaState, Request, Species, Vec3, Zone};

fn length(v: Vec3) -> f64 {
    (v.x * v.x + v.y * v.y + v.z * v.z).sqrt()
}

#[test]
fn axes_take_their_lengths_and_angles_from_their_pa() {
    let state = |internode, insertion, lateral: &[f64]| PaState {
        lifespan: 5,
        next: None,
        viability: 1.0,
        zones: vec![Zone {
            nodes: NodeLaw::Uniform { min: 2, max: 2 },
            buds: 2,
            dormant: vec![0.0; lateral.len()],
            delay: 0.0,
            rate: 0.0,
            lateral: lateral.to_vec(),
        }],
        shedding: None,
        internode,
        insertion,
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
        apical_control: 0.5,
        upkeep: 0.0,
        balance_hazard: 0.0,
        tolerance: 0.0,
        straightening: 0.0,
        form: Form::default(),
    };
    let species = Species {
        states: vec![state(0.5, 0.0, &[0.0, 1.0]), state(0.3, 0.6, &[0.0, 0.0])],
    };
    let tree = grow(
        &species,
        Request {
            age: 5,
            seed: 3,
            budget: 1_000,
            light: telperion_space::Light::NEUTRAL,
        },
    )
    .unwrap();
    let trunk = &tree.axes[0];
    assert_eq!(
        trunk.phytomers.last().unwrap().tip,
        Vec3::new(0.0, 0.0, 5.0)
    );
    let mut sides = Vec::new();
    for axis in tree.axes.iter().filter(|a| !a.phytomers.is_empty()) {
        let Origin::Lateral { node, .. } = axis.origin else {
            continue;
        };
        let tip = axis.phytomers.last().unwrap().tip;
        let run = tip - axis.base;
        assert!((length(run) - 0.3 * axis.phytomers.len() as f64).abs() < 1e-12);
        assert!(
            (run.z / length(run) - 0.6f64.cos()).abs() < 1e-12,
            "the insertion angle"
        );
        assert_eq!(axis.base, trunk.phytomers[node].tip);
        sides.push(((run.y.atan2(run.x).to_degrees().round() as i64) + 360) % 360);
    }
    sides.sort_unstable();
    sides.dedup();
    assert_eq!(
        sides,
        vec![0, 90, 180, 270],
        "opposite pairs turned a quarter each node"
    );
}

/// A PA that renews itself every growth unit keeps turning its nodes: the
/// phyllotaxis runs on across each continuation.
#[test]
fn phyllotaxis_runs_on_across_a_continuation() {
    let state = |next, lateral: &[f64]| PaState {
        lifespan: 1,
        next,
        viability: 1.0,
        zones: vec![Zone {
            nodes: NodeLaw::Uniform { min: 1, max: 1 },
            buds: 1,
            dormant: vec![0.0; lateral.len()],
            delay: 0.0,
            rate: 0.0,
            lateral: lateral.to_vec(),
        }],
        shedding: None,
        internode: 1.0,
        insertion: 0.6,
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
        apical_control: 0.5,
        upkeep: 0.0,
        balance_hazard: 0.0,
        tolerance: 0.0,
        straightening: 0.0,
        form: Form::default(),
    };
    let species = Species {
        states: vec![state(Some(0), &[0.0, 1.0]), state(None, &[0.0, 0.0])],
    };
    let request = Request {
        age: 5,
        seed: 1,
        budget: 100,
        light: telperion_space::Light::NEUTRAL,
    };
    let tree = grow(&species, request).unwrap();
    let mut sides: Vec<i64> = Vec::new();
    for axis in tree
        .axes
        .iter()
        .filter(|a| a.pa == 1 && !a.phytomers.is_empty())
    {
        let run = axis.phytomers[0].tip - axis.base;
        sides.push(((run.y.atan2(run.x).to_degrees().round() as i64) + 360) % 360);
    }
    assert_eq!(sides, vec![0, 90, 180, 270]);
}

/// A species whose expected wood overflows a double still draws the tree
/// it grew in finite numbers: one phytomer whose six buds stayed bare.
#[test]
fn an_overflowing_expectation_draws_finite_wood() {
    let species = Species {
        states: vec![PaState {
            lifespan: 1,
            next: None,
            viability: 0.9,
            zones: vec![Zone {
                nodes: NodeLaw::Uniform { min: 1, max: 1 },
                buds: 6,
                dormant: vec![0.0; 1],
                delay: 0.0,
                rate: 0.0,
                lateral: vec![0.3],
            }],
            shedding: None,
            internode: 1.0,
            insertion: 0.5,
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
            apical_control: 0.5,
            upkeep: 0.0,
            balance_hazard: 0.0,
            tolerance: 0.0,
            straightening: 0.0,
            form: Form::default(),
        }],
    };
    let request = Request {
        age: 1500,
        seed: 18,
        budget: 1,
        light: telperion_space::Light::NEUTRAL,
    };
    let tree = grow(&species, request).unwrap();
    for p in tree.axes.iter().flat_map(|a| &a.phytomers) {
        assert!(p.scale.is_finite() && p.tip.z.is_finite(), "{p:?}");
    }
}

/// An unreachable PA whose expected wood dwarfs the tree's draws no NaN:
/// a probability of zero takes no part in a window.
#[test]
fn an_unreachable_pa_takes_no_part_in_a_window() {
    let state = |viability, internode, nodes, buds, lateral: [f64; 2]| PaState {
        lifespan: 1,
        next: None,
        viability,
        zones: vec![Zone {
            nodes,
            buds,
            dormant: vec![0.0; lateral.len()],
            delay: 0.0,
            rate: 0.0,
            lateral: lateral.to_vec(),
        }],
        shedding: None,
        internode,
        insertion: 0.5,
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
        apical_control: 0.5,
        upkeep: 0.0,
        balance_hazard: 0.0,
        tolerance: 0.0,
        straightening: 0.0,
        form: Form::default(),
    };
    let species = Species {
        states: vec![
            state(1.0, 0.1, NodeLaw::Poisson { mean: 1.0 }, 1, [0.0, 0.0]),
            state(0.9, 1.0, NodeLaw::Uniform { min: 1, max: 1 }, 6, [0.0, 0.3]),
        ],
    };
    let request = Request {
        age: 1470,
        seed: 1,
        budget: 1,
        light: telperion_space::Light::NEUTRAL,
    };
    let tree = grow(&species, request).unwrap();
    for p in tree.axes.iter().flat_map(|a| &a.phytomers) {
        assert!(p.scale.is_finite() && p.tip.z.is_finite(), "{p:?}");
    }
}
