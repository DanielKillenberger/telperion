//! The Norway spruce (*Picea abies*) as a point in the tree space: values
//! only, on Massart's model as Prusinkiewicz and Remphrey (2000) state it
//! and the CIRAD lecture describes the spruce: an orthotropic,
//! monopodial, indeterminate trunk whose rhythmic growth units each end
//! in a whorl of plagiotropic branches, which bear their own branchlets
//! in one plane. The rules (M1 to M14) and their sources are in
//! `.flow/evidence/fn-194-tree-space-d-the-norway-spruce-as-a/SOURCES.md`;
//! a value no source gives is marked there as estimated.
use crate::species::{Form, NodeLaw, PaState, Species, Zone};
use std::f64::consts::{FRAC_PI_2, PI};

/// The reference axis, youngest first.
const TRUNK: usize = 1;
const CROWN: usize = 2;
const BRANCH: usize = 3;
const BRANCHLET: usize = 4;
const SHOOT: usize = 5;
const PAS: usize = 6;

/// One zone: nodes from `min` to `max`, `buds` each, bearing `pa` with
/// probability `p` (bare where `p` is 0).
fn zone(min: u32, max: u32, buds: u8, laterals: &[(usize, f64)]) -> Zone {
    let mut lateral = vec![0.0; PAS];
    for &(pa, p) in laterals {
        lateral[pa] = p;
    }
    Zone {
        nodes: NodeLaw::Uniform { min, max },
        buds,
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

/// The trunk's growth unit: bare internodes, a medial zone of weak
/// laterals (M9), and the whorl of branches at its top (M3).
fn trunk_unit(bare: (u32, u32), medial: f64, whorl: f64) -> Vec<Zone> {
    vec![
        zone(bare.0, bare.1, 1, &[]),
        zone(1, 2, 1, &[(BRANCH, medial)]),
        zone(1, 1, 5, &[(BRANCH, whorl)]),
    ]
}

/// The spruce, one growth cycle a year.
pub fn spruce() -> Species {
    // The trunk (M1, M2): erect, never stopping, spiral (the golden
    // angle), each whorl's branches spread round it and turned into the
    // horizontal plane (`plane`), so their own laterals stand in it (M5).
    let stem = Form {
        tropism: 2.0,
        elevation: FRAC_PI_2,
        wander: 0.05,
        plane: FRAC_PI_2,
        pipe: 0.006,
        exponent: 2.4,
        ripening: 0.0,
        dominance: 0.0,
        roll: 0.0,
    };
    // The seedling (M4): short units, its laterals spread along them
    // (rhythm low) before the whorls establish.
    let seedling = PaState {
        divergence: 2.4,
        internode: 0.02,
        rhythm: 0.3,
        form: stem,
        ..state(5, Some(TRUNK), trunk_unit((2, 3), 0.0, 0.0))
    };
    // The trunk in its vigorous years: a yearly shoot of about 0.35 m.
    let trunk = PaState {
        divergence: 2.4,
        internode: 0.042,
        form: stem,
        ..state(40, Some(CROWN), trunk_unit((5, 6), 0.2, 0.85))
    };
    // The leader in the mature crown: shorter yearly shoots.
    let crown = PaState {
        lifespan: 1_000,
        next: None,
        internode: 0.03,
        ..trunk.clone()
    };
    // Main branches (M5, M6, M7, M13): plagiotropic, monopodial, near
    // horizontal from the whorl and turning up towards the tip; each
    // growth unit bears branchlets left and right in its plane, two at
    // its top and one between.
    let branch = PaState {
        insertion: 1.5,
        internode: 0.03,
        form: Form {
            tropism: 0.6,
            elevation: 0.55,
            wander: 0.5,
            plane: 0.0,
            pipe: 0.0008,
            exponent: 2.2,
            ripening: 10.0,
            dominance: 0.0,
            roll: 0.3,
        },
        ..state(
            1_000,
            None,
            vec![
                zone(1, 1, 1, &[]),
                zone(1, 1, 1, &[(BRANCHLET, 0.8)]),
                zone(1, 1, 2, &[(BRANCHLET, 0.95)]),
            ],
        )
    };
    // Second-order branchlets: in the branch's plane, hanging as the
    // comb's curtains (M12, reference-visible only), shed when they die
    // (M11).
    let branchlet = PaState {
        insertion: 0.9,
        internode: 0.016,
        viability: 0.98,
        shedding: Some(1),
        form: Form {
            tropism: 3.0,
            elevation: -0.85,
            wander: 0.6,
            plane: 0.0,
            pipe: 0.0005,
            exponent: 2.0,
            ripening: 0.0,
            dominance: 0.0,
            roll: 0.2,
        },
        ..state(
            25,
            None,
            vec![
                zone(2, 2, 1, &[(SHOOT, 0.6)]),
                zone(1, 1, 2, &[(SHOOT, 0.9)]),
            ],
        )
    };
    // Third-order shoots: short, unbranched, living a few years.
    let shoot = PaState {
        insertion: 0.8,
        internode: 0.02,
        viability: 0.9,
        shedding: Some(1),
        form: Form {
            tropism: 0.5,
            elevation: -0.6,
            wander: 1.0,
            pipe: 0.0004,
            ..Form::default()
        },
        ..state(4, None, vec![zone(2, 3, 1, &[])])
    };
    Species {
        states: vec![seedling, trunk, crown, branch, branchlet, shoot],
    }
}
