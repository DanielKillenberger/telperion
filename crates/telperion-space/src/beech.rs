//! The European beech (*Fagus sylvatica*) as a point in the tree space:
//! values only, on Troll's model as Millet, Bouchard and Édelin (1998)
//! describe it in *Fagus* and as Letort et al.'s GreenLab beech encodes
//! its growth units. Each value's source is in
//! `.flow/evidence/fn-193-tree-space-c-the-beech-as-the-first/SOURCES.md`;
//! a value no source gives for a mature open-grown beech is marked there
//! as estimated.
use crate::species::{Form, NodeLaw, PaState, Species, Zone};
use std::f64::consts::{FRAC_PI_2, PI};

/// The reference axis, youngest first.
const FORK: usize = 1;
const LEADER: usize = 2;
const LIMB: usize = 3;
const BOUGH: usize = 4;
const SPUR: usize = 5;
const BRANCH: usize = 6;
const SHOOT: usize = 7;
const SHORT: usize = 8;
const PAS: usize = 9;

/// One zone: nodes from `min` to `max`, one bud each, bearing `pa` with
/// probability `p` (bare where `p` is 0).
fn zone(min: u32, max: u32, laterals: &[(usize, f64)]) -> Zone {
    let mut lateral = vec![0.0; PAS];
    for &(pa, p) in laterals {
        lateral[pa] = p;
    }
    Zone {
        nodes: NodeLaw::Uniform { min, max },
        buds: 1,
        lateral,
    }
}

/// The neutral state every row below starts from.
fn state(lifespan: u32, next: Option<usize>, zones: Vec<Zone>) -> PaState {
    PaState {
        lifespan,
        next,
        viability: 1.0,
        zones,
        shedding: None,
        internode: 0.04,
        insertion: 0.0,
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

/// The beech, one growth cycle a year.
pub fn beech() -> Species {
    // A1, the young stem: orthotropic, monopodial in this engine, bearing
    // plagiotropic A2 systems acrotonically, until the fork.
    // Troll's modules: each grows erect and bends plagiotropic at its tip,
    // and ends after two or three growth units, its hazard rising with
    // its length; a relay from a bud on the upper side of its curvature
    // zone straightens and carries the stem on, counting the stem's
    // growth units, while the module's head stays a branch. Each
    // module's base straightens further over the years (Millet's
    // secondary straightening), the seedling's included.
    let trunk = PaState {
        divergence: 2.4,
        internode: 0.07,
        insertion: 0.3,
        abortion: 0.1,
        abortion_rise: 2.0,
        relay: 1.0,
        relay_at: 0.5,
        epitony: 0.6,
        erection: 0.1,
        straightening: 1.0,
        form: Form {
            tropism: 0.6,
            elevation: 0.45,
            wander: 0.2,
            plane: FRAC_PI_2,
            pipe: 0.00057,
        },
        ..state(
            12,
            Some(FORK),
            vec![
                zone(3, 4, &[]),
                zone(2, 3, &[(BRANCH, 0.35)]),
                zone(1, 1, &[(BRANCH, 0.8)]),
            ],
        )
    };
    // The top of the stem: "the branches become large and oblique, and a
    // fork is created from subterminal buds".
    let fork = PaState {
        divergence: 2.4,
        form: Form {
            tropism: 0.3,
            elevation: FRAC_PI_2,
            ..trunk.form
        },
        ..state(
            2,
            Some(LEADER),
            vec![zone(2, 3, &[]), zone(3, 3, &[(LIMB, 0.9)])],
        )
    };
    // Total reiterates: oblique, thick, forking in turn into reiterates
    // "increasingly smaller and less branched".
    let reiterate = |lifespan, child: Option<(usize, f64)>, elevation: f64| PaState {
        insertion: 0.7,
        straightening: 0.25,
        viability: 0.995,
        form: Form {
            tropism: 0.25,
            elevation,
            wander: 1.2,
            plane: 0.0,
            pipe: 0.00057,
        },
        ..state(
            lifespan,
            None,
            vec![
                zone(2, 3, &[]),
                zone(2, 3, &[(SHORT, 0.3), (BRANCH, 0.6)]),
                zone(1, 1, &child.map_or(vec![(BRANCH, 0.6)], |c| vec![c])),
            ],
        )
    };
    // The stem's own relay at the fork, one of its equals but the most erect.
    let leader = reiterate(70, Some((LIMB, 0.1)), 1.25);
    let limb = reiterate(70, Some((BOUGH, 0.12)), 1.1);
    let bough = reiterate(30, Some((SPUR, 0.1)), 0.95);
    let spur = reiterate(15, None, 0.75);
    // GreenLab's PA 2, the long ramified shoot: Z20 bare, Z24 short
    // shoots, Z23 long shoots bearing short shoots, Z22 partial
    // reiteration, base to tip (acrotony).
    let branch = PaState {
        insertion: 1.0,
        viability: 0.97,
        shedding: Some(1),
        internode: 0.04,
        form: Form {
            tropism: 0.8,
            elevation: 0.35,
            wander: 0.8,
            plane: 0.0,
            pipe: 0.00057,
        },
        ..state(
            10,
            Some(SHOOT),
            vec![
                zone(1, 2, &[]),
                zone(2, 3, &[(SHORT, 0.6)]),
                zone(1, 2, &[(SHOOT, 0.8)]),
                zone(1, 1, &[(BRANCH, 0.08)]),
            ],
        )
    };
    // GreenLab's PA 3: a long shoot bearing only short shoots.
    let shoot = PaState {
        insertion: 1.0,
        viability: 0.95,
        shedding: Some(1),
        internode: 0.035,
        form: Form {
            tropism: 1.0,
            elevation: 0.15,
            wander: 2.0,
            plane: 0.0,
            pipe: 0.00057,
        },
        ..state(5, None, vec![zone(1, 1, &[]), zone(2, 3, &[(SHORT, 0.6)])])
    };
    // GreenLab's PA 4: a short shoot of three to five metamers that never
    // branches.
    let short = PaState {
        insertion: 0.9,
        viability: 0.9,
        shedding: Some(1),
        internode: 0.006,
        form: Form {
            tropism: 0.0,
            elevation: 0.0,
            wander: 0.0,
            plane: 0.0,
            pipe: 0.00107,
        },
        ..state(3, None, vec![zone(3, 5, &[])])
    };
    Species {
        states: vec![trunk, fork, leader, limb, bough, spur, branch, shoot, short],
    }
}
