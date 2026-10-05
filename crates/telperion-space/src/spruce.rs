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
const SAPLING: usize = 1;
const TRUNK: usize = 2;
const CROWN: usize = 3;
const SPRIG: usize = 4;
const BRANCH: usize = 5;
const BRANCHLET: usize = 6;
const SPUR: usize = 7;
const SHOOT: usize = 8;
const PAS: usize = 9;

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
        dormant: vec![0.0; lateral.len()],
        delay: 0.0,
        rate: 0.0,
        lateral,
    }
}

/// A main branch's zone with a sleeping branchlet bud at every bud place,
/// waking from a year on at a yearly hazard of 0.3 (fn-202's release law):
/// the draperies, new branchlets released along old branches as the old
/// ones die (M10). The share asleep is tuned to S1's dark lower curtains.
fn sleeping(mut zone: Zone) -> Zone {
    zone.dormant[BRANCHLET] = 0.45;
    zone.delay = 1.0;
    zone.rate = 0.3;
    zone
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
        leaf_area: 0.0,
        shade_hazard: 0.0,
        shade_size: 0.0,
        apical_control: 0.5,
        upkeep: 0.0,
        balance_hazard: 0.0,
        tolerance: 0.0,
        straightening: 0.0,
        form: Form::default(),
    }
}

/// The trunk's growth unit: bare internodes, a medial zone of weak
/// laterals (M9), and the whorl of branches at its top (M3).
fn trunk_unit(bare: (u32, u32), medial: f64, whorl: f64) -> Vec<Zone> {
    vec![
        zone(bare.0, bare.1, 1, &[]),
        zone(1, 2, 1, &[(SPRIG, medial)]),
        zone(1, 1, 5, &[(SPRIG, whorl)]),
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
        wander: 0.1,
        plane: FRAC_PI_2,
        pipe: 0.0025,
        exponent: 2.4,
        ripening: 0.0,
        dominance: 0.0,
        roll: 0.0,
        sag: 0.0,
        secondary: 1.0,
    };
    // The seedling (M4): two short, unbranched years.
    let seedling = PaState {
        divergence: 2.4,
        internode: 0.02,
        rhythm: 0.3,
        form: stem,
        ..state(2, Some(SAPLING), trunk_unit((2, 3), 0.0, 0.0))
    };
    // The sapling: whorls from its third year, on yearly shoots of about
    // 0.2 m, its rhythm still establishing (M4).
    let sapling = PaState {
        divergence: 2.4,
        internode: 0.028,
        rhythm: 0.7,
        form: stem,
        ..state(8, Some(TRUNK), trunk_unit((4, 5), 0.25, 0.85))
    };
    // The trunk in its vigorous years: a yearly shoot of about 0.35 m.
    let trunk = PaState {
        divergence: 2.4,
        internode: 0.042,
        form: stem,
        ..state(35, Some(CROWN), trunk_unit((5, 6), 0.3, 0.85))
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
    // They bend under their load (sag), the lower and longer the more,
    // their tips still rising; a few die and are shed, and siblings are
    // unequal (dominance), so no two crowns are alike.
    let branch = PaState {
        insertion: 1.5,
        internode: 0.034,
        viability: 0.998,
        shedding: Some(3),
        form: Form {
            tropism: 1.4,
            elevation: 1.0,
            wander: 1.0,
            plane: 0.0,
            pipe: 0.0008,
            exponent: 2.2,
            ripening: 10.0,
            dominance: 0.35,
            roll: 0.8,
            sag: 9e-4,
            secondary: 1.0,
        },
        ..state(
            1_000,
            None,
            vec![
                sleeping(zone(1, 1, 1, &[])),
                sleeping(zone(2, 2, 2, &[(BRANCHLET, 0.7)])),
                sleeping(zone(1, 1, 2, &[(BRANCHLET, 0.95)])),
            ],
        )
    };
    // A branch's first years: longer yearly shoots, so a young tree's
    // tiers spread wide and the crown's top is a cone, not a spire.
    // They rise a little more than the older wood beyond them, so the
    // upper crown, all young branches, stays level to ascending.
    let sprig = PaState {
        lifespan: 6,
        next: Some(BRANCH),
        internode: 0.05,
        shedding: None,
        form: Form {
            tropism: 0.4,
            elevation: 0.35,
            sag: 1e-4,
            secondary: 1.0,
            ..branch.form
        },
        ..branch.clone()
    };
    // Second-order branchlets: in the branch's plane, hanging as the
    // comb's curtains (M12, reference-visible only), shed when they die
    // (M11). After ten years a branchlet barely lengthens and keeps
    // making shoots for decades, so a branch stays green far inside.
    let branchlet = PaState {
        insertion: 0.9,
        internode: 0.016,
        viability: 0.99,
        next: Some(SPUR),
        shedding: Some(1),
        form: Form {
            tropism: 2.0,
            elevation: -1.0,
            wander: 0.6,
            plane: 0.0,
            pipe: 0.0005,
            exponent: 2.0,
            ripening: 0.0,
            dominance: 0.0,
            roll: 0.2,
            sag: 5e-4,
            secondary: 1.0,
        },
        ..state(
            10,
            None,
            vec![
                zone(2, 2, 2, &[(SHOOT, 0.55)]),
                zone(1, 1, 3, &[(SHOOT, 0.8)]),
            ],
        )
    };
    let spur = PaState {
        lifespan: 40,
        viability: 0.99,
        next: None,
        internode: 0.006,
        // Each year a pair of shoots, the pairs turning by the golden angle,
        // so an old branchlet hangs as a needled mass all round, not a
        // flat shelf; the young branchlet keeps its flat spray.
        zones: vec![zone(1, 1, 2, &[(SHOOT, 0.6)])],
        divergence: 2.4,
        form: Form {
            tropism: 3.0,
            elevation: -1.35,
            ..branchlet.form
        },
        ..branchlet.clone()
    };
    // Third-order shoots: short, unbranched, two or three a node, hanging
    // so each curtain is a needled spray with width. A shoot grows two
    // years and keeps its needles three more, about five in all: half a
    // cohort's needles are shed at 5.5 years (Muukkonen and Lehtonen 2004).
    let shoot = PaState {
        insertion: 1.1,
        internode: 0.02,
        viability: 0.9,
        shedding: Some(3),
        form: Form {
            tropism: 1.0,
            elevation: -0.5,
            wander: 1.0,
            pipe: 0.0004,
            roll: 1.2,
            ..Form::default()
        },
        ..state(2, None, vec![zone(2, 2, 1, &[])])
    };
    Species {
        states: vec![
            seedling, sapling, trunk, crown, sprig, branch, branchlet, spur, shoot,
        ],
    }
}
