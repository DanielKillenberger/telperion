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
use crate::fixed;
use crate::fixed::presence::{q, real, times, ONE};
use crate::species::{Species, Zone};
use crate::structure::{Axis, Origin};
use crate::transcendental::Transcendental;
use std::sync::Arc;

/// The window, in log-odds, per share of the tree's expected length that a
/// draw decides.
pub const RATE: f64 = 6.0;
/// The widest window, in log-odds: a crown's.
pub const SPAN: f64 = 2.0;
/// The narrowest window, in log-odds: a twig's.
pub const FLOOR: f64 = 0.05;

/// The expected wood each draw decides, as a share of the whole tree's.
/// Its tables are shared: every fine root's grower holds the tree's
/// (`Rules`), and a copy of them a root cost most of a root's growth in
/// Wasm.
#[derive(Clone)]
pub(crate) struct Windows {
    /// expected[m][k]: the log of the metres a bud of PA k grows in m cycles.
    expected: Arc<Vec<Vec<f64>>>,
    /// The log of the whole tree's expected metres.
    whole: f64,
    /// woods[m][k]: `expected[m][k]`'s share of the whole tree, Q32, by
    /// the contract's `exp` (host decision 8): taken once a tree, not
    /// once a draw.
    woods: Arc<Vec<Vec<i64>>>,
    age: u32,
}

impl Windows {
    pub fn new(species: &Species, age: u32) -> Self {
        use crate::fixed::{exp, q32};
        let expected = expected_log_lengths(species, age);
        let whole = expected[age as usize][0];
        // A wood of none stays none: its log less a finite one is -inf.
        let woods = expected
            .iter()
            .map(|row| row.iter().map(|&log| exp(q32(log - whole))).collect())
            .collect();
        Self {
            expected: Arc::new(expected),
            whole,
            woods: Arc::new(woods),
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
        (log - self.whole).exp_fixed()
    }

    /// The log of `wood`: none where it is none (FINE-CONTRACT.md D10's
    /// table).
    pub fn log_wood(&self, pa: usize, cycle: u32) -> f64 {
        let log = self.expected[(self.age + 1).saturating_sub(cycle) as usize][pa];
        if log == f64::NEG_INFINITY {
            return log;
        }
        log - self.whole
    }

    /// The wood a bud of `pa` is expected to grow over the `left` cycles
    /// left of the tree's age, as a share of the whole tree's: `wood` at
    /// the cycle `left` cycles before the tree's age.
    pub fn wood_left(&self, pa: usize, left: usize) -> f64 {
        let log = self.expected[left.min(self.age as usize)][pa];
        if log == f64::NEG_INFINITY {
            return 0.0;
        }
        (log - self.whole).exp_fixed()
    }

    /// The chance that a draw made with probability `p` is made within its
    /// window, at a presence below 1, deciding `decided` of `wood`: where
    /// a stop all but made grows both outcomes (host decision 11).
    pub fn borderline(&self, p: f64, wood: f64, decided: f64) -> f64 {
        let window = (RATE * wood * decided).clamp(FLOOR * decided, SPAN);
        if p <= 0.0 || p >= 1.0 || window <= 0.0 {
            return 0.0;
        }
        let logit = (p / (1.0 - p)).ln_fixed();
        p - 1.0 / (1.0 + (window - logit).exp_fixed())
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
        metres * (-self.whole).exp_fixed()
    }

    /// The window, Q32, of a draw deciding `stake` of the wood a bud of
    /// `pa` is expected to grow from `cycle` on, or from `used` of the way
    /// through it (host decision 8, `fixed::presence`).
    pub fn window_q(&self, (pa, cycle, used): (usize, u32, Option<f64>), stake: f64) -> i64 {
        use crate::fixed::{presence, q32, ticks};
        let log = |c: u32| q32(self.log_wood(pa, c));
        let wood = match used {
            None => self.woods[(self.age + 1).saturating_sub(cycle) as usize][pa],
            Some(used) => presence::wood_at((log(cycle), log(cycle + 1)), ticks(used)),
        };
        let window = (q32(RATE * stake), q32(FLOOR * stake), q32(SPAN));
        presence::window(wood, window)
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
    /// Per growth unit, in one record (fn-211 task 3: one vector where
    /// there were two, made, copied and freed per axis).
    pub units: Vec<Unit>,
    /// Per phytomer: its node's draw.
    pub nodes: Vec<f64>,
    /// The apex still lives at the tree's age.
    pub alive: bool,
    /// How much a living apex lives on (`Grower::living`).
    pub living: f64,
    /// Per growth unit, where light shades: its phytomers' size from its
    /// bud's vigour (`allocation.rs`) and the light its bud grew in, whole
    /// where nothing reads it; empty where nothing shades.
    pub shade: Vec<[f64; 2]>,
    /// The apex stopped by failing to survive a growth unit.
    pub failed: bool,
    /// A relay of an age that ended or of a unit that failed
    /// (`schedule.rs`).
    pub ended: bool,
    /// A failed unit's relay, whose first unit grows the rest of the cycle
    /// the unit failed in, drawn apart from it (fn-199 R1).
    pub rest: bool,
    /// What its first unit holds below its ticks, in 2^-32 of a cycle:
    /// the part of a tick its failed unit's lived time was rounded up by
    /// (fn-199 B2).
    pub below: u64,
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

/// One growth unit's draws.
#[derive(Debug, Clone, Copy)]
pub(crate) struct Unit {
    /// Its survival, then the apex's persistence past it (not aborting).
    pub survive: f64,
    pub persist: f64,
    /// The share of its cycle it grew (`schedule.rs`).
    pub share: f64,
}

impl Draws {
    /// The share of their cycles its units grew, summed in turn.
    pub fn grown(&self) -> f64 {
        self.units.iter().map(|u| u.share).sum::<f64>()
    }

    /// Growth unit `k`'s size and light, whole where nothing shades.
    pub fn shade(&self, k: usize) -> [f64; 2] {
        self.shade.get(k).copied().unwrap_or([1.0; 2])
    }
}

/// Sets every axis's vigour, growth-unit presences, phytomer scales (each
/// relative to its axis's base), phyllotactic ranks and living presence,
/// from each axis's draws, read where they stand.
pub(crate) fn assign<D: std::borrow::Borrow<Draws> + Sync>(
    axes: &mut [Axis],
    draws: &[D],
    species: &Species,
    threads: usize,
) {
    let living = |_: usize, d: &Draws| d.alive.then_some(d.living);
    assign_living(axes, draws, species, &living, threads);
}

/// `assign`, where `living(k, draws)` is how much axis `k`'s apex lives
/// on, none where it has stopped.
pub(crate) fn assign_living<D: std::borrow::Borrow<Draws> + Sync>(
    axes: &mut [Axis],
    draws: &[D],
    species: &Species,
    living: &(dyn Fn(usize, &Draws) -> Option<f64> + Sync),
    threads: usize,
) {
    let at = |k: usize| draws[k].borrow();
    // A relay's node: where along its parent's last growth unit it
    // stands, which girth and placement both read, on its nodes'
    // presences (FINE-CONTRACT.md D18). It reads only the parent's
    // phytomers' cycles and node draws, which assigning leaves alone.
    for i in 0..axes.len() {
        if let Origin::Relay { parent, .. } = axes[i].origin {
            let share = species.states[axes[parent].pa].relay_at;
            let node = relay_node(&axes[parent], at(parent), share);
            axes[i].origin = Origin::Relay { parent, node };
        }
    }
    // Each axis's presence at its end, which what carries it on is made
    // at: its own alone, so every axis is assigned on its own, on up to
    // `threads` cores, and its vigour then from its parent's end.
    let pieces: Vec<(usize, &mut [Axis])> = axes
        .chunks_mut(crate::cores::PIECE)
        .enumerate()
        .map(|(k, c)| (k * crate::cores::PIECE, c))
        .collect();
    let ends = crate::cores::map(pieces, 1, threads, &|_, (from, piece)| {
        piece
            .iter_mut()
            .enumerate()
            .map(|(j, axis)| {
                let k = from + j;
                assign_one(axis, at(k), living(k, at(k)))
            })
            .collect::<Vec<f64>>()
    });
    let end: Vec<f64> = ends.into_iter().flatten().collect();
    for (axis, draws) in axes.iter_mut().zip(draws) {
        let draws: &Draws = draws.borrow();
        let made = times(q(draws.birth[0]), q(draws.birth[1]));
        axis.vigour = match axis.origin {
            Origin::Seed | Origin::Lateral { .. } => real(made),
            Origin::Continuation { parent } | Origin::Relay { parent, .. } => {
                real(times(q(end[parent]), made))
            }
        };
    }
}

/// The node of axis `p`'s last growth unit a relay of it stands at,
/// `share` of the way along the unit by its nodes' presences.
pub(crate) fn relay_node(p: &Axis, draws: &Draws, share: f64) -> usize {
    let n = p.phytomers.len();
    let Some(last) = p.phytomers.last().map(|q| q.cycle) else {
        return 0;
    };
    let first = p
        .phytomers
        .iter()
        .position(|q| q.cycle == last)
        .unwrap_or(0);
    // Each presence is a Q32 fixed point held exactly in its real.
    let presences: Vec<u64> = draws.nodes[first..n]
        .iter()
        .map(|&x| (x * fixed::ONE32 as f64) as u64)
        .collect();
    fixed::decide::relay_node(&presences, first, n, share)
}

/// Assigns one axis's presences from its draws: its end's presence.
/// Every presence a fade reads is Q32, each product floored in turn (host
/// decision 8); the ranks stay real, which geometry alone reads.
fn assign_one(axis: &mut Axis, draws: &Draws, living: Option<f64>) -> f64 {
    let birth = axis.birth;
    let mut running = ONE;
    let mut presences = Vec::with_capacity(draws.units.len());
    let mut j = 0;
    let mut rank = 0.0;
    axis.first = 0.0;
    for (k, unit) in draws.units.iter().enumerate() {
        running = times(running, q(unit.survive));
        let grown = q(unit.share);
        presences.push(real(times(running, grown)));
        let [size, light] = draws.shade(k);
        while j < axis.phytomers.len() && (axis.phytomers[j].cycle - birth - 1) as usize == k {
            axis.phytomers[j].scale = real(times(times(running, q(draws.nodes[j])), grown));
            axis.first = (axis.first + real(times(running, q(draws.nodes[j])))).min(1.0);
            axis.phytomers[j].size = size;
            axis.phytomers[j].light = light;
            axis.phytomers[j].rank = rank;
            rank += draws.nodes[j] * unit.share;
            j += 1;
        }
        running = times(running, q(unit.persist));
    }
    axis.units = presences;
    axis.sleep = draws.sleep;
    axis.rank = rank;
    axis.alive = living.map_or(0.0, |l| real(times(running, q(l))));
    real(running)
}
