//! The silver birch (*Betula pendula*) as a point in the tree space:
//! values only. Its architecture is the one Millet, Bouchard and Édelin
//! (1999) describe for its sister the grey birch: related to Rauh's model,
//! though sympodial, a hierarchy of five orthotropic axis categories on a
//! 2/5 spiral, rhythmic, branching acrotonically and deferred, the last
//! category an unbranched short shoot; few reiterates, and the old trunk
//! ending in small forks. Long shoots build the tree, short shoots bearing
//! one bud at best replace themselves (Jones and Harper 1987). The outer
//! long shoots hang: a branch holds its angle by its gravitropic set-point
//! (Muranen et al. 2026), which the birch's fine shoots set below the
//! horizontal, and bend under their own weight. Each value's source is in
//! `.flow/evidence/fn-213-tree-space-the-silver-birch-as-a-point/MODEL-BIRCH.md`;
//! a value no source gives is marked there as estimated.
use crate::chain::{self, on_chain};
use crate::species::{Form, NodeLaw, PaState, Species, Zone};
use std::f64::consts::{FRAC_PI_2, PI};

/// The birch's ages on the shared reference axis (`chain.rs`), youngest
/// first: the young stem, the stem in the crown, its old top of small
/// forks, the limbs and their boughs, the young stem's passing branches,
/// the hanging long-shoot systems, their long shoots and the short shoots.
const TRUNK: usize = chain::TRUNK;
const YOUNG: usize = chain::LOW_FORK;
const LEADER: usize = chain::LEADER;
const TOP: usize = chain::TOP_FORK;
const LIMB: usize = chain::LIMB;
const BOUGH: usize = chain::BOUGH;
const SPRIG: usize = chain::SPRIG;
const BRANCH: usize = chain::BRANCH;
const SHORT: usize = chain::SHORT;
const PAS: usize = chain::AGES;

/// The 2/5 spiral of the birch's leaves and buds (MBE99).
const SPIRAL: f64 = 2.0 * PI * 2.0 / 5.0;

/// One zone: nodes from `min` to `max`, one bud each (the leaves are
/// alternate), bearing `pa` with probability `p` (bare where `p` is 0).
fn zone(min: u32, max: u32, laterals: &[(usize, f64)]) -> Zone {
    let mut lateral = vec![0.0; PAS];
    for &(pa, p) in laterals {
        lateral[pa] = p;
    }
    Zone {
        nodes: NodeLaw::Uniform {
            min: f64::from(min),
            max: f64::from(max),
        },
        buds: 1.0,
        dormant: vec![0.0; lateral.len()],
        delay: 0.0,
        rate: 0.0,
        lateral,
    }
}

/// The neutral state every row below starts from: one growth unit a year
/// on the 2/5 spiral. The birch's axes are sympodia "with a hierarchic
/// development similar to that of a monopodium" (MBE99), so they are drawn
/// monopodial, as the beech's stem is.
fn state(lifespan: u32, continuation: f64, zones: Vec<Zone>) -> PaState {
    PaState {
        lifespan: f64::from(lifespan),
        continuation,
        viability: 1.0,
        zones,
        shedding: f64::INFINITY,
        internode: 0.04,
        insertion: 0.0,
        divergence: SPIRAL,
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
        flowering: 0.0,
        cluster_leaves: 0.0,
        cluster_spread: 0.0,
        form: Form::default(),
    }
}

/// Old limb wood's sleeping buds: each node of `zone` also holds, with
/// probability `p`, a bud of a hanging strand that wakes years later
/// (Cameron and Sani 1994: *B. pendula* sprouts epicormic shoots), so the
/// limbs stay hung with strands along their older wood.
fn sleeping(mut zone: Zone, p: f64) -> Zone {
    zone.dormant[BRANCH] = p;
    zone.delay = 4.0;
    zone.rate = 0.15;
    zone
}

/// A growth unit: a bare base, medial nodes and the acrotonic top.
fn unit(
    bare: (u32, u32),
    medial: ((u32, u32), &[(usize, f64)]),
    top: ((u32, u32), &[(usize, f64)]),
) -> Vec<Zone> {
    vec![
        zone(bare.0, bare.1, &[]),
        zone(medial.0 .0, medial.0 .1, medial.1),
        zone(top.0 .0, top.0 .1, top.1),
    ]
}

/// The birch, one growth cycle a year.
pub fn birch() -> Species {
    let stem = Form {
        tropism: 0.5,
        elevation: FRAC_PI_2,
        wander: 0.35,
        plane: 0.0,
        pipe: 0.004,
        exponent: 2.5,
        ripening: 0.0,
        dominance: 0.0,
        roll: 0.0,
        sag: 0.0,
        secondary: 1.0,
        bend_length: 6.0,
        internode_spread: 0.2,
        insertion_spread: 0.0,
        tropism_spread: 0.0,
    };
    // A1, the young stem: fast yearly units, bearing the young tree's
    // passing branches acrotonically.
    let trunk = PaState {
        internode: 0.055,
        form: stem,
        ..state(
            5,
            1.0,
            unit(
                (4, 6),
                ((6, 8), &[(SPRIG, 0.3), (SHORT, 0.3)]),
                ((2, 3), &[(SPRIG, 0.7)]),
            ),
        )
    };
    // The young stem in the crown, years 6 to 14: shorter units bearing
    // passing branches, which the bole sheds.
    let young = PaState {
        internode: 0.04,
        form: stem,
        ..state(
            9,
            1.0,
            unit(
                (3, 4),
                ((3, 5), &[(SPRIG, 0.25), (BRANCH, 0.06), (SHORT, 0.3)]),
                ((2, 2), &[(LIMB, 0.1), (SPRIG, 0.6), (BRANCH, 0.12)]),
            ),
        )
    };
    // The stem's middle years, 15 to 40: a few scaffold limbs that live
    // and grow for decades into the old crown's top (S1, S3; host, round
    // 7), among passing branches.
    let leader = PaState {
        internode: 0.04,
        form: stem,
        ..state(
            26,
            1.0,
            unit(
                (3, 4),
                (
                    (3, 5),
                    &[(LIMB, 0.04), (SPRIG, 0.25), (BRANCH, 0.06), (SHORT, 0.3)],
                ),
                ((2, 2), &[(LIMB, 0.22), (SPRIG, 0.45), (BRANCH, 0.12)]),
            ),
        )
    };
    // The old top: short units forking into small boughs.
    // The old stem gives up its lead (MBE99: "the extremity of the main
    // stem assume[s] a divaricated structure with very short growth
    // units"): from its 40s its tip often dies and a bud below it, barely
    // turned upward, carries on at a slant: small forks (round 6).
    let top = PaState {
        internode: 0.015,
        abortion: 0.8,
        relay: 1.0,
        relay_ended: 1.0,
        relay_failed: 1.0,
        relay_at: 0.6,
        epitony: 0.2,
        form: Form {
            tropism: 0.4,
            elevation: 1.0,
            wander: 0.5,
            ..stem
        },
        ..state(
            100,
            0.0,
            unit(
                (1, 2),
                ((2, 3), &[(BRANCH, 0.3), (SHORT, 0.3)]),
                ((1, 2), &[(BOUGH, 0.5), (BRANCH, 0.4)]),
            ),
        )
    };
    // Crooked, irregularly subdivided limbs (host and Astra, sheet 3):
    // the apex now and then dies and a bud on the upper side of its last
    // unit carries the limb on, a kink; boughs differ in vigour; the
    // limb's bend is remembered over metres (fn-207); a few limbs die and
    // are shed.
    let limb = PaState {
        insertion: 0.6,
        internode: 0.055,
        viability: 0.998,
        shedding: 3.0,
        abortion: 0.08,
        relay: 1.0,
        relay_ended: 1.0,
        relay_failed: 1.0,
        epitony: 0.4,
        form: Form {
            tropism: 0.9,
            elevation: 1.3,
            wander: 0.6,
            plane: 0.0,
            pipe: 0.002,
            exponent: 2.4,
            ripening: 10.0,
            dominance: 0.2,
            roll: 1.2,
            sag: 6e-5,
            secondary: 1.0,
            bend_length: 3.0,
            internode_spread: 0.7,
            insertion_spread: 0.0,
            tropism_spread: 0.0,
        },
        ..state(
            25,
            1.0,
            unit(
                (2, 3),
                ((3, 5), &[(BOUGH, 0.12), (BRANCH, 0.15), (SHORT, 0.35)]),
                ((1, 2), &[(BOUGH, 0.45), (BRANCH, 0.4)]),
            ),
        )
    };
    let bough = PaState {
        insertion: 0.6,
        internode: 0.03,
        viability: 0.995,
        shedding: 2.0,
        abortion: 0.08,
        relay: 1.0,
        relay_ended: 1.0,
        relay_failed: 1.0,
        epitony: 0.4,
        form: Form {
            tropism: 0.3,
            elevation: 0.5,
            sag: 8e-5,
            pipe: 0.002,
            ..limb.form
        },
        ..state(100, 0.0, {
            let mut u = unit(
                (1, 2),
                ((3, 4), &[(BRANCH, 0.4), (SHORT, 0.35)]),
                ((1, 1), &[(BRANCH, 0.5)]),
            );
            u[1] = sleeping(u[1].clone(), 0.2);
            u
        })
    };
    // The young stem's branches, which pass: the bole clears.
    let sprig = PaState {
        insertion: 0.7,
        internode: 0.04,
        viability: 0.97,
        shedding: 2.0,
        form: Form {
            tropism: 0.5,
            elevation: 0.8,
            wander: 0.4,
            plane: 0.0,
            pipe: 0.0012,
            exponent: 2.2,
            ripening: 0.0,
            dominance: 0.0,
            roll: 0.8,
            sag: 2e-4,
            secondary: 1.0,
            bend_length: 0.0,
            internode_spread: 0.4,
            insertion_spread: 0.0,
            tropism_spread: 0.0,
        },
        ..state(
            14,
            0.0,
            unit(
                (1, 2),
                ((3, 4), &[(BRANCH, 0.4), (SHORT, 0.4)]),
                ((1, 1), &[(BRANCH, 0.5)]),
            ),
        )
    };
    // The hanging long-shoot system: thin, set below the horizontal and
    // bowed by its own weight, clothed in short shoots.
    let hanging = Form {
        tropism: 3.0,
        elevation: -FRAC_PI_2,
        wander: 0.4,
        plane: 0.0,
        pipe: 0.0008,
        exponent: 2.0,
        ripening: 0.0,
        dominance: 0.0,
        roll: 1.6,
        sag: 2e-4,
        secondary: 1.0,
        bend_length: 1.0,
        internode_spread: 0.7,
        insertion_spread: 0.5,
        tropism_spread: 0.6,
    };
    let branch = PaState {
        insertion: 0.7,
        internode: 0.06,
        viability: 0.92,
        shedding: 1.0,
        form: hanging,
        ..state(
            12,
            0.0,
            vec![
                zone(1, 1, &[]),
                zone(3, 5, &[(SHORT, 0.5), (BRANCH, 0.005)]),
            ],
        )
    };
    // The short shoot: a few crowded nodes a year, its one bud carrying it
    // on, never branching. It bears its leaves at its tip, "two or three
    // leaves" (KU03 on B. pendula; SU03): three, fanned 0.6 rad either side.
    let short = PaState {
        cluster_leaves: 3.0,
        cluster_spread: 0.6,
        insertion: 0.8,
        internode: 0.003,
        viability: 0.92,
        shedding: 1.0,
        form: Form {
            pipe: 0.0007,
            ..Form::default()
        },
        ..state(6, 0.0, vec![zone(2, 3, &[])])
    };
    on_chain(vec![
        (TRUNK, trunk),
        (YOUNG, young),
        (LEADER, leader),
        (TOP, top),
        (LIMB, limb),
        (BOUGH, bough),
        (SPRIG, sprig),
        (BRANCH, branch),
        (SHORT, short),
    ])
}
