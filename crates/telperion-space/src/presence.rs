//! Presence (fn-192 R2): how grown each element a draw made is, from 0 where
//! a setting crosses the draw to 1 once the setting is a window past it.
//! The window is in log-odds and scales with the wood the draw decides, as
//! the closed form expects it of the element's physiological age over the
//! cycles left, relative to the whole tree's expected length. A crown grows
//! in over a long stretch of a walk and a twig over a short one, and since
//! the expectation is a smooth function of the settings, never of the wood
//! that grew, a branch made anywhere moves no other draw's window.
use crate::closed_form::expected_log_lengths;
use crate::species::Species;
use crate::structure::{Axis, Origin};

/// The window, in log-odds, per share of the tree's expected length that a
/// draw decides.
pub const RATE: f64 = 6.0;
/// The widest window, in log-odds: a crown's.
pub const SPAN: f64 = 2.0;
/// The narrowest window, in log-odds: a twig's.
pub const FLOOR: f64 = 0.05;

/// The expected wood each draw decides, as a share of the whole tree's.
pub(crate) struct Windows {
    /// expected[m][k]: the log of the metres a bud of PA k grows in m cycles.
    expected: Vec<Vec<f64>>,
    /// The log of the whole tree's expected metres.
    whole: f64,
    age: u32,
}

impl Windows {
    pub fn new(species: &Species, age: u32) -> Self {
        let expected = expected_log_lengths(species, age);
        let whole = expected[age as usize][0];
        Self {
            expected,
            whole,
            age,
        }
    }

    /// The wood a bud of `pa` is expected to grow from `cycle` on, the cycle
    /// itself included, as a share of the whole tree's.
    pub fn wood(&self, pa: usize, cycle: u32) -> f64 {
        let log = self.expected[(self.age + 1).saturating_sub(cycle) as usize][pa];
        if log == f64::NEG_INFINITY {
            return 0.0;
        }
        (log - self.whole).exp()
    }

    /// `metres` as a share of the whole tree's expected wood.
    pub fn share(&self, metres: f64) -> f64 {
        metres * (-self.whole).exp()
    }

    /// The presence of a draw `lead` log-odds past its bound that decides
    /// `share` of the tree's expected wood.
    pub fn presence(&self, lead: f64, share: f64) -> f64 {
        if lead == f64::INFINITY {
            return 1.0;
        }
        let window = (RATE * share).clamp(FLOOR, SPAN);
        (lead / window).clamp(0.0, 1.0)
    }
}

/// The presences of the draws that shaped one axis.
#[derive(Debug, Clone, Default)]
pub(crate) struct Draws {
    /// The draws that made the axis: a lateral's PA, or a relay's stop and
    /// relay draws.
    pub birth: [f64; 2],
    /// Per growth unit: its survival, then the apex's persistence past it
    /// (not aborting).
    pub units: Vec<[f64; 2]>,
    /// Per phytomer: its node's draw.
    pub nodes: Vec<f64>,
    /// The apex still lives at the tree's age.
    pub alive: bool,
}

/// Sets every axis's vigour, growth-unit presences, phytomer scales (each
/// relative to its axis's base), phyllotactic ranks and living presence.
pub(crate) fn assign(axes: &mut [Axis], draws: &[Draws]) {
    let mut end = vec![1.0; axes.len()];
    for i in 0..axes.len() {
        let axis = &mut axes[i];
        let birth = axis.birth;
        let mut running = 1.0;
        let mut presences = Vec::with_capacity(draws[i].units.len());
        let mut j = 0;
        let mut rank = 0.0;
        for (k, &[survive, persist]) in draws[i].units.iter().enumerate() {
            running *= survive;
            presences.push(running);
            while j < axis.phytomers.len() && (axis.phytomers[j].cycle - birth - 1) as usize == k {
                axis.phytomers[j].scale = running * draws[i].nodes[j];
                axis.phytomers[j].rank = rank;
                rank += draws[i].nodes[j];
                j += 1;
            }
            running *= persist;
        }
        end[i] = running;
        axis.units = presences;
        axis.rank = rank;
        axis.alive = if draws[i].alive { running } else { 0.0 };
        let made = draws[i].birth[0] * draws[i].birth[1];
        axis.vigour = match axis.origin {
            Origin::Seed | Origin::Lateral { .. } => made,
            Origin::Continuation { parent } | Origin::Relay { parent } => end[parent] * made,
        };
    }
}
