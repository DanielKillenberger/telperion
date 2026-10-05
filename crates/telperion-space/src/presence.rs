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

    /// The wood a bud of `pa` is expected to grow from `used` of the way
    /// through `cycle`: between the cycle's and the next's by its log, so
    /// an age ending just before a whole cycle decides as one ending just
    /// after it (Codex round 7 on fn-206).
    pub fn wood_at(&self, pa: usize, cycle: u32, used: f64) -> f64 {
        let (a, b) = (self.wood(pa, cycle), self.wood(pa, cycle + 1));
        if a <= 0.0 || b <= 0.0 {
            return a + (b - a) * used;
        }
        (a.ln() + (b.ln() - a.ln()) * used).exp()
    }

    /// The wood a bud of `pa` is expected to grow over the `left` cycles
    /// left of the tree's age, as a share of the whole tree's: `wood` at
    /// the cycle `left` cycles before the tree's age.
    pub fn wood_left(&self, pa: usize, left: usize) -> f64 {
        let log = self.expected[left.min(self.age as usize)][pa];
        if log == f64::NEG_INFINITY {
            return 0.0;
        }
        (log - self.whole).exp()
    }

    /// The chance that a draw made with probability `p` is made within its
    /// window, at a presence below 1, deciding `decided` of `wood`: where
    /// a stop all but made grows both outcomes (host decision 11).
    pub fn borderline(&self, p: f64, wood: f64, decided: f64) -> f64 {
        let window = (RATE * wood * decided).clamp(FLOOR * decided, SPAN);
        if p <= 0.0 || p >= 1.0 || window <= 0.0 {
            return 0.0;
        }
        let logit = (p / (1.0 - p)).ln();
        p - 1.0 / (1.0 + (window - logit).exp())
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
    /// Per growth unit: the share of its cycle it grew (`schedule.rs`).
    pub shares: Vec<f64>,
    /// Per phytomer: its node's draw.
    pub nodes: Vec<f64>,
    /// The apex still lives at the tree's age.
    pub alive: bool,
    /// How much a living apex lives on (`Grower::living`).
    pub living: f64,
    /// Per growth unit, where light shades: its phytomers' size from its
    /// bud's vigour (`allocation.rs`); empty where nothing shades.
    pub sizes: Vec<f64>,
    /// Per growth unit, where light shades: the light its bud grew in.
    pub lights: Vec<f64>,
    /// The apex stopped by failing to survive a growth unit.
    pub failed: bool,
    /// A relay of an age that ended or of a unit that failed
    /// (`schedule.rs`).
    pub ended: bool,
    /// The share of the cycle its age ended at, where it ended there and
    /// stopped: a sleeping bud waking in that cycle wakes on it.
    pub outlived: Option<f64>,
    /// How many relays of an age that ended carry this axis on, each
    /// drawing its stop decisions apart.
    pub ended_relays: u32,
    /// The time a woken bud slept in its stage, which its abortion hazard
    /// counts.
    pub aged: f64,
    /// The share of its first cycle gone before it grew, asleep or to the
    /// age before it, which its first growth unit lacks.
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
            let grown = draws[i].shares[k];
            presences.push(running * grown);
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
        axis.alive = if draws[i].alive {
            running * draws[i].living
        } else {
            0.0
        };
        let made = draws[i].birth[0] * draws[i].birth[1];
        axis.vigour = match axis.origin {
            Origin::Seed | Origin::Lateral { .. } => made,
            Origin::Continuation { parent } | Origin::Relay { parent, .. } => end[parent] * made,
        };
    }
}
