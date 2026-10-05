//! The pedunculate oak (*Quercus robur*) as a point in the tree space:
//! values only, on Rauh's model as Prusinkiewicz and Remphrey (2000) and
//! Barthélémy and Caraglio (2007) state it: rhythmic, monopodial,
//! orthotropic axes, each a repetition of the trunk, bearing their
//! laterals acrotonically at the top of each yearly shoot, flowering
//! laterally. The mature open-grown oak departs from the model as Kędra
//! (2019) found every old oak does: its trunk forks low into a few
//! near-equal limbs, which fork again. The rules and their sources are in
//! `.flow/evidence/fn-195-tree-space-d-the-oak-as-a-point/SOURCES.md`; a
//! value no source gives is marked there as estimated.
use crate::chain::{self, on_chain};
use crate::species::{Form, NodeLaw, PaState, Species, Zone};
use std::f64::consts::FRAC_PI_2;

/// The oak's ages on the shared reference axis (`chain.rs`), youngest
/// first: its fork stands at the low fork, and its shoot, which its
/// branch moves on to, before its twig, which it bears.
const TRUNK: usize = chain::TRUNK;
const FORK: usize = chain::LOW_FORK;
const LEADER: usize = chain::LEADER;
const LIMB: usize = chain::LIMB;
const BOUGH: usize = chain::BOUGH;
const SPRIG: usize = chain::SPRIG;
const BRANCH: usize = chain::BRANCH;
const TWIG: usize = chain::TWIG;
const SHOOT: usize = chain::SHOOT;
const SHORT: usize = chain::SHORT;
const PAS: usize = chain::AGES;

/// The 2/5 spiral of the oak's leaves and buds.
const SPIRAL: f64 = 2.513;

/// One zone: nodes from `min` to `max`, `buds` each, bearing `pa` with
/// probability `p` (bare where `p` is 0).
fn zone(min: u32, max: u32, buds: u8, laterals: &[(usize, f64)]) -> Zone {
    let mut lateral = vec![0.0; PAS];
    for &(pa, p) in laterals {
        lateral[pa] = p;
    }
    Zone {
        nodes: NodeLaw::Uniform {
            min: f64::from(min),
            max: f64::from(max),
        },
        buds: f64::from(buds),
        dormant: vec![0.0; lateral.len()],
        delay: 0.0,
        rate: 0.0,
        lateral,
    }
}

/// A zone of old wood holding sleeping branch buds, which wake years
/// later as epicormic shoots: the oak's reserve buds.
fn sleeping(mut zone: Zone, p: f64) -> Zone {
    zone.dormant[BRANCH] = p;
    zone.delay = 6.0;
    zone.rate = 0.04;
    zone
}

/// The neutral state every row below starts from.
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
        // One leaf a node (Go Botany) of 18.1 cm2, the mean original area
        // of 616 Q. robur leaves on ten Oxfordshire oaks, five of them 150
        // to 200 years old (Visakorpi et al. 2020, PLoS ONE 15: e0228157,
        // Table 1).
        leaf_area: 0.00181,
        // Light as the oak's own values (fn-195 round 5, on fn-197's
        // engine; host decision 29): shade raises its shoots' death hazard
        // and shares its growth by light within each order (Borchert-Honda,
        // λ 0.45), its carbon balance sheds its starved laterals, its shed
        // branches leave a share of their pipe, and lit limbs thicken.
        shade_hazard: 1.0,
        shade_size: 1.5,
        apical_control: 0.45,
        upkeep: 0.35,
        balance_hazard: 2.0,
        tolerance: 0.0,
        retained: 0.5,
        leaf_girth: 1.0,
        straightening: 0.0,
        form: Form::default(),
    }
}

/// A rhythmic growth unit: bare nodes at its base, a few laterals below
/// its top, and the cluster of buds at its top (acrotony).
fn unit(bare: (u32, u32), below: &[(usize, f64)], top: (u8, &[(usize, f64)])) -> Vec<Zone> {
    vec![
        zone(bare.0, bare.1, 1, &[]),
        zone(2, 3, 1, below),
        zone(1, 1, top.0, top.1),
    ]
}

/// The oak, one growth cycle a year.
pub fn oak() -> Species {
    let stem = Form {
        tropism: 0.6,
        elevation: FRAC_PI_2,
        wander: 0.25,
        plane: 0.0,
        pipe: 0.009,
        exponent: 2.6,
        ripening: 0.0,
        dominance: 0.0,
        roll: 0.0,
        sag: 0.0,
        secondary: 1.0,
        bend_length: 0.0,
    };
    // The young stem: erect and monopodial, a yearly shoot of about
    // 0.36 m, its laterals temporary long shoots that die within three
    // decades and are shed, so the bole clears.
    let trunk = PaState {
        internode: 0.045,
        form: stem,
        ..state(12, 1.0, unit((4, 5), &[(SPRIG, 0.4)], (3, &[(SPRIG, 0.6)])))
    };
    // The fork: for two years the top cluster of each yearly shoot grows
    // out as limbs, so three or four limbs leave at two heights (K19's
    // main branches of unequal insertion height).
    let fork = PaState {
        internode: 0.045,
        form: stem,
        ..state(2, 1.0, unit((3, 4), &[(SPRIG, 0.3)], (3, &[(LIMB, 0.55)])))
    };
    // The stem carries on above the fork as a weaker leader for two
    // decades, bearing limbs now and then and boughs from its top
    // clusters, so the crown's top fills and rounds; then it carries on
    // as one more limb. K19: the trunk "contributes to the vertical tree
    // extent", the lowest main branches to the horizontal. The limbs it
    // bears are younger and shorter than the fork's, so the crown rounds.
    let leader = PaState {
        internode: 0.05,
        form: Form {
            tropism: 0.6,
            wander: 0.5,
            bend_length: 3.0,
            ..stem
        },
        ..state(
            19,
            1.0,
            unit(
                (3, 4),
                &[(SHORT, 0.25), (BRANCH, 0.4)],
                (3, &[(LIMB, 0.08), (BOUGH, 0.1)]),
            ),
        )
    };
    // The limbs: Rauh's axes repeating the trunk, ascending and settling
    // outward under their weight (sag); each yearly shoot ends in a
    // cluster that forks the limb now and then. Their apex dies often and
    // a bud of the cluster carries the limb on (CIR), so the limb kinks at
    // each relay: the oak's tortuous course. A large pipe exponent keeps
    // the few limbs heavy.
    let limb = PaState {
        // A limb that dies is dropped five years on: oaks hold dead
        // branches for years before they fall (fn-197 host decision 18;
        // estimated, an arborists' account, no measured persistence
        // found). An E-side value, reconciled with the oak's branch.
        shedding: 5.0,
        insertion: 0.5,
        internode: 0.045,
        straightening: 0.15,
        abortion: 0.15,
        relay: 1.0,
        relay_ended: 1.0,
        relay_failed: 1.0,
        epitony: 0.2,
        form: Form {
            tropism: 0.5,
            elevation: 1.05,
            wander: 0.5,
            plane: 0.0,
            pipe: 0.009,
            exponent: 2.8,
            ripening: 15.0,
            dominance: 0.0,
            roll: 0.8,
            sag: 0.0003,
            secondary: 1.0,
            bend_length: 3.0,
        },
        // A limb grows for as long as an oak lives (500 years), then stops.
        ..state(
            500,
            0.0,
            unit(
                (2, 3),
                &[(SHORT, 0.25), (BRANCH, 0.45)],
                (3, &[(LIMB, 0.007), (BOUGH, 0.08)]),
            ),
        )
    };
    let limb = PaState {
        zones: limb
            .zones
            .iter()
            .cloned()
            .map(|z| sleeping(z, 0.1))
            .collect(),
        ..limb
    };
    // Boughs: the limbs' smaller repetitions, living some decades,
    // kinked by relays as the limbs are and bending a little under load.
    let bough = PaState {
        insertion: 0.6,
        internode: 0.03,
        viability: 0.999,
        shedding: 4.0,
        abortion: 0.1,
        relay: 1.0,
        relay_ended: 1.0,
        relay_failed: 1.0,
        epitony: 0.2,
        form: Form {
            tropism: 0.48,
            elevation: 0.5,
            wander: 0.35,
            pipe: 0.004,
            exponent: 2.6,
            sag: 0.00005,
            secondary: 1.0,
            bend_length: 1.5,
            ..limb.form
        },
        ..state(
            60,
            0.0,
            unit(
                (2, 3),
                &[(SHORT, 0.3), (BRANCH, 0.3)],
                (3, &[(BRANCH, 0.3)]),
            ),
        )
    };
    // The young stem's laterals: long shoots of about 0.35 m a year for
    // ten years, ascending as Rauh's laterals repeat the trunk, and
    // branching, then branch systems, so a sapling is a bushy young tree.
    let sprig = PaState {
        insertion: 0.7,
        internode: 0.07,
        viability: 0.95,
        shedding: 3.0,
        form: Form {
            tropism: 0.5,
            elevation: 0.7,
            wander: 0.4,
            plane: 0.0,
            pipe: 0.0012,
            exponent: 2.0,
            ripening: 0.0,
            dominance: 0.0,
            roll: 1.2,
            sag: 0.0,
            secondary: 1.0,
            bend_length: 0.0,
        },
        ..state(
            10,
            1.0,
            unit(
                (1, 2),
                &[(SHORT, 0.3), (TWIG, 0.3)],
                (3, &[(TWIG, 0.4), (BRANCH, 0.1)]),
            ),
        )
    };
    // Branch systems: a long shoot each year, its laterals short shoots
    // and twigs and, clustered at its top, twigs.
    let branch = PaState {
        insertion: 0.8,
        internode: 0.017,
        viability: 0.98,
        shedding: 3.0,
        abortion: 0.0,
        relay: 1.0,
        relay_ended: 1.0,
        relay_failed: 1.0,
        epitony: 0.2,
        form: Form {
            tropism: 0.33,
            elevation: 0.2,
            wander: 0.4,
            plane: 0.0,
            pipe: 0.0009,
            exponent: 2.0,
            ripening: 0.0,
            dominance: 0.0,
            roll: 1.2,
            sag: 0.0,
            secondary: 1.0,
            bend_length: 0.0,
        },
        ..state(
            12,
            1.0,
            unit(
                (1, 2),
                &[(SHORT, 0.3), (TWIG, 0.18)],
                (3, &[(TWIG, 0.25), (BRANCH, 0.02)]),
            ),
        )
    };
    // Twigs: one more order, three years of short growth bearing short
    // shoots, so the fine wood between the boughs is a web of short
    // twigs rather than a few long shoots.
    let twig = PaState {
        insertion: 0.8,
        internode: 0.012,
        viability: 0.95,
        shedding: 2.0,
        form: Form {
            tropism: 0.4,
            elevation: 0.2,
            wander: 0.4,
            pipe: 0.0008,
            roll: 1.2,
            ..Form::default()
        },
        ..state(
            3,
            0.0,
            unit((1, 1), &[(SHORT, 0.4)], (3, &[(TWIG, 0.0), (SHORT, 0.3)])),
        )
    };
    // Long shoots bearing short shoots only.
    let shoot = PaState {
        insertion: 0.8,
        internode: 0.015,
        viability: 0.93,
        shedding: 2.0,
        form: Form {
            tropism: 0.8,
            elevation: 0.2,
            wander: 0.4,
            pipe: 0.0009,
            roll: 1.2,
            ..Form::default()
        },
        ..state(4, 0.0, unit((1, 1), &[(SHORT, 0.3)], (2, &[(SHORT, 0.35)])))
    };
    // Short shoots: a few crowded leaves and a bud cluster, unbranched.
    let short = PaState {
        insertion: 0.9,
        internode: 0.006,
        viability: 0.9,
        shedding: 2.0,
        form: Form {
            pipe: 0.00095,
            ..Form::default()
        },
        ..state(3, 0.0, vec![zone(3, 5, 1, &[])])
    };
    on_chain(vec![
        (TRUNK, trunk),
        (FORK, fork),
        (LEADER, leader),
        (LIMB, limb),
        (BOUGH, bough),
        (SPRIG, sprig),
        (BRANCH, branch),
        (TWIG, twig),
        (SHOOT, shoot),
        (SHORT, short),
    ])
}
