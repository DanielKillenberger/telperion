//! Presence (fn-192 R2): how grown each element a draw made is, from 0 where
//! a setting crosses the draw to 1 once the setting is a window past it.
//! The window is in log-odds and scales with the wood the draw decides, as
//! the closed form expects it of the element's physiological age over the
//! cycles left, relative to the whole tree's expected length. A crown grows
//! in over a long stretch of a walk and a twig over a short one, and since
//! the expectation is a smooth function of the settings, never of the wood
//! that grew, a branch made anywhere moves no other draw's window.
use crate::contract::presence::{q, real, times, ONE};
use crate::species::Species;
use crate::structure::{Axis, Origin};

mod windows;
pub(crate) use windows::{At, Windows};

/// The window, in log-odds, per share of the tree's expected length that a
/// draw decides.
pub const RATE: f64 = 6.0;
/// The widest window, in log-odds: a crown's.
pub const SPAN: f64 = 2.0;
/// The narrowest window, in log-odds: a twig's.
pub const FLOOR: f64 = 0.05;
/// The most cycles a draw looks over (fn-355): the cycles left of its PA's
/// lifespan, at most this many. Estimate, the host's: 40 is the age most
/// presets are asked for and their walk bounds were measured at, so a
/// young draw's window stays near what it has been, and a lifespan of any
/// size costs the expected-length table no more than this.
pub const LOOK: f64 = 40.0;

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
    /// bud's vigour (`allocation.rs`); empty where nothing shades.
    pub shade: Vec<f64>,
    /// The apex stopped by failing to survive a growth unit.
    pub failed: bool,
    /// A relay of an age that ended or of a unit that failed
    /// (`schedule.rs`).
    pub ended: bool,
    /// A failed unit's relay, whose first unit grows the rest of the cycle
    /// the unit failed in, drawn apart from it (fn-199 R1).
    pub rest: bool,
    /// Carried on in the cycle the axis before it stopped in, growing the
    /// rest of it (`Shoot::carry_on`): where its age ended there, a rest a
    /// lifespan's fraction cuts (fn-231).
    pub carried: bool,
    /// The axis a sleeping bud grows in as it wakes (`Shoot::woke`), whose
    /// first unit its waking draws and no lifespan's fraction cuts.
    pub woke: bool,
    /// What its first unit holds below its ticks, in 2^-32 of a cycle:
    /// the part of a tick its failed unit's lived time was rounded up by
    /// (fn-199 B2).
    pub below: u64,
    /// The share of the cycle its age ended at, where it ended there and
    /// stopped (`outlives`): a sleeping bud waking in that cycle wakes on
    /// it. Read through `outlived`; the flag sits beside the others, not
    /// in an `Option`'s tag (`an_axis_and_its_draws_stay_compact`).
    ended_at: f64,
    outlives: bool,
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
    /// The share of a whole node's length its nodes grow at: its share,
    /// or whole where a lifespan's fraction cut it and it laid that share
    /// of its nodes instead (fn-231).
    pub size: f64,
}

impl Draws {
    /// The draws of an axis just made, by the draws that made it.
    pub fn made(birth: [f64; 2]) -> Self {
        Self {
            birth,
            ..Self::default()
        }
    }

    /// The share of their cycles its units grew, summed in turn.
    pub fn grown(&self) -> f64 {
        self.units.iter().map(|u| u.share).sum::<f64>()
    }

    /// Growth unit `k`'s size, whole where nothing shades.
    pub fn shade(&self, k: usize) -> f64 {
        self.shade.get(k).copied().unwrap_or(1.0)
    }

    /// The share of the cycle its age ended at, where it ended there and
    /// stopped.
    pub fn outlived(&self) -> Option<f64> {
        self.outlives.then_some(self.ended_at)
    }

    /// Its age ended `used` of the way through the cycle it stopped in.
    pub fn outlive(&mut self, used: f64) {
        (self.ended_at, self.outlives) = (used, true);
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
    // Each presence is an f32 held exactly in its real.
    let presences: Vec<f32> = draws.nodes[first..n].iter().map(|&x| x as f32).collect();
    crate::contract::decide::relay_node(&presences, first, n, share)
}

/// Assigns one axis's presences from its draws: its end's presence.
/// Every presence a fade reads is an f32, each product taken in turn (host
/// decision 8); the ranks stay real, which geometry alone reads.
fn assign_one(axis: &mut Axis, draws: &Draws, living: Option<f64>) -> f64 {
    let birth = axis.birth;
    let mut running = ONE;
    let mut presences = Vec::with_capacity(draws.units.len());
    let mut j = 0;
    let mut rank = 0.0;
    for (k, unit) in draws.units.iter().enumerate() {
        running = times(running, q(unit.survive));
        let grown = q(unit.share);
        presences.push(real(times(running, grown)));
        let size = draws.shade(k);
        while j < axis.phytomers.len() && (axis.phytomers[j].cycle - birth - 1) as usize == k {
            let laid = q(unit.size);
            axis.phytomers[j].scale = real(times(times(running, q(draws.nodes[j])), laid));
            axis.phytomers[j].grown = axis.phytomers[j].scale;
            axis.phytomers[j].size = size;
            axis.phytomers[j].rank = rank;
            rank += draws.nodes[j] * unit.size;
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
