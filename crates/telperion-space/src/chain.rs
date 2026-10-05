//! The shared reference axis (fn-206): one ordered chain of physiological
//! ages that every species is written on, as AmapSim's reference axis is
//! one for all (host, 2026-10-05). An age a species does not use is
//! passed through at lifespan 0, so a species is a vector of settings on
//! one chain and two species have a midpoint. The order is a common one
//! of the beech, spruce, oak and palm: each moves only forward along it,
//! and each lateral is no younger than its bearer
//! (`.flow/evidence/fn-206-tree-space-one-reference-axis-every/PROPOSAL.md`).
use crate::species::{Form, NodeLaw, PaState, Species, Zone};
use std::f64::consts::PI;

pub const SEEDLING: usize = 0;
pub const SAPLING: usize = 1;
pub const TRUNK: usize = 2;
/// The oak's: its trunk forks below its leader.
pub const LOW_FORK: usize = 3;
pub const LEADER: usize = 4;
/// The beech's: its leader forks at its top.
pub const TOP_FORK: usize = 5;
pub const LIMB: usize = 6;
pub const BOUGH: usize = 7;
pub const SPRIG: usize = 8;
pub const BRANCH: usize = 9;
pub const BRANCHLET: usize = 10;
pub const SPUR: usize = 11;
pub const SHOOT: usize = 12;
pub const TWIG: usize = 13;
pub const SHORT: usize = 14;
/// The ages on the chain.
pub const AGES: usize = 15;

/// An age a species passes through: no lifespan, moving on to the next.
fn passed() -> PaState {
    PaState {
        lifespan: 0.0,
        continuation: 1.0,
        viability: 1.0,
        zones: vec![Zone {
            nodes: NodeLaw::Uniform { min: 0.0, max: 0.0 },
            buds: 1.0,
            lateral: vec![0.0; AGES],
            dormant: vec![0.0; AGES],
            delay: 0.0,
            rate: 0.0,
        }],
        shedding: f64::INFINITY,
        internode: 0.04,
        insertion: 0.0,
        divergence: PI,
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
        straightening: 0.0,
        leaf_area: 0.0,
        shade_hazard: 0.0,
        shade_size: 0.0,
        apical_control: 0.5,
        upkeep: 0.0,
        balance_hazard: 0.0,
        tolerance: 0.0,
        retained: 0.0,
        leaf_girth: 0.0,
        form: Form::default(),
    }
}

/// A species from the ages it lives in, each at its place on the chain;
/// every other age is passed through, in canonical form.
pub(crate) fn on_chain(ages: Vec<(usize, PaState)>) -> Species {
    let mut states: Vec<PaState> = (0..AGES).map(|_| passed()).collect();
    for (age, state) in ages {
        states[age] = state;
    }
    Species { states }.canonical()
}
