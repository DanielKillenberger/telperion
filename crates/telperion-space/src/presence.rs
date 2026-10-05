//! Presence (fn-192 R2): how grown each element a draw made is, from 0 where
//! a setting crosses the draw to 1 once the setting is a window past it.
//! The window is in log-odds and scales with the wood the draw decides, as
//! the closed form expects it of the element's physiological age over the
//! cycles left, relative to the whole tree's expected length. A crown grows
//! in over a long stretch of a walk and a twig over a short one, and since
//! the expectation is a smooth function of the settings, never of the wood
//! that grew, a branch made anywhere moves no other draw's window.
use crate::closed_form::expected_log_lengths;
use crate::dormant::wakes_in;
use crate::species::{Species, Zone};
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

    /// The wood a sleeping bud of `pa` on a node grown in `cycle` is
    /// expected to grow, over the years it may wake in.
    fn dormant(&self, zone: &Zone, pa: usize, cycle: u32) -> f64 {
        (0..self.age.saturating_sub(cycle))
            .map(|s| wakes_in(zone, s) * self.wood(pa, cycle + 1 + s))
            .sum()
    }

    /// The wood each PA's sleeping bud on a node of `zone` grown in `cycle`
    /// is expected to grow, into `woods` (none where the zone bears none),
    /// and the wood the sleeping buds of one bud place are expected to.
    pub fn sleeping(&self, zone: &Zone, cycle: u32, woods: &mut Vec<f64>) -> f64 {
        woods.clear();
        let mut expected = 0.0;
        for (pa, &p) in zone.dormant.iter().enumerate() {
            let wood = if p > 0.0 {
                self.dormant(zone, pa, cycle)
            } else {
                0.0
            };
            expected += p * wood;
            woods.push(wood);
        }
        expected
    }

    /// `metres` as a share of the whole tree's expected wood.
    pub fn share(&self, metres: f64) -> f64 {
        metres * (-self.whole).exp()
    }

    /// The presence of a draw `lead` log-odds past its bound that decides
    /// `share` of the tree's expected wood.
    pub fn presence(&self, lead: f64, share: f64) -> f64 {
        self.decided(lead, share, 1.0)
    }

    /// The presence of a draw `lead` log-odds past its bound that decides
    /// `decided` of the `wood` share it would carry: its window is that
    /// share of the whole window, floor included, so a draw that decides
    /// no wood (a stop that is always relayed) has none.
    pub fn decided(&self, lead: f64, wood: f64, decided: f64) -> f64 {
        let window = (RATE * wood * decided).clamp(FLOOR * decided, SPAN);
        if lead == f64::INFINITY || window <= 0.0 {
            return 1.0;
        }
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
    /// Per growth unit, where light shades: its phytomers' size from its
    /// bud's vigour (`allocation.rs`); empty where nothing shades.
    pub sizes: Vec<f64>,
    /// Per growth unit, where light shades: the light its bud grew in.
    pub lights: Vec<f64>,
    /// Per phytomer: its node's draw.
    pub nodes: Vec<f64>,
    /// The apex still lives at the tree's age.
    pub alive: bool,
    /// The apex stopped by failing to survive a growth unit.
    pub failed: bool,
    /// The units a woken bud slept in its stage, which its abortion hazard
    /// counts.
    pub aged: u32,
    /// The share of its first cycle a bud that woke still slept, which
    /// its first growth unit lacks.
    pub sleep: f64,
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
            let grown = if k == 0 { 1.0 - draws[i].sleep } else { 1.0 };
            while j < axis.phytomers.len() && (axis.phytomers[j].cycle - birth - 1) as usize == k {
                axis.phytomers[j].scale = running * draws[i].nodes[j] * grown;
                axis.phytomers[j].size = draws[i].sizes.get(k).copied().unwrap_or(1.0);
                axis.phytomers[j].light = draws[i].lights.get(k).copied().unwrap_or(1.0);
                axis.phytomers[j].rank = rank;
                rank += draws[i].nodes[j] * grown;
                j += 1;
            }
            running *= persist;
        }
        end[i] = running;
        axis.units = presences;
        axis.sleep = draws[i].sleep;
        axis.rank = rank;
        axis.alive = if draws[i].alive { running } else { 0.0 };
        let made = draws[i].birth[0] * draws[i].birth[1];
        axis.vigour = match axis.origin {
            Origin::Seed | Origin::Lateral { .. } => made,
            Origin::Continuation { parent } | Origin::Relay { parent, .. } => end[parent] * made,
        };
    }
}
