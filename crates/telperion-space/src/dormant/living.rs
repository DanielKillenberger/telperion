//! The closed form of a sleeping bud's bearer: the chance its axis is
//! still carried on when the bud wakes (`Living`), split from `dormant.rs`.
use super::{share, stages, First, Piece};
use crate::memo::Memo;
use crate::presence::Windows;
use crate::schedule::{lived, Carried, LIVED, LIVED_FLOOR};
use crate::species::Species;
use crate::transcendental::Transcendental;
use std::rc::Rc;

/// The closed form of `carried`: the chance that the axis a bud grows is
/// still carried on, by its apex or a continuation or relay of it, at the
/// start of a cycle. Cycles count from the bud's birth, its first growth
/// unit growing in cycle 1, as the closed form's do.
pub(crate) struct Living<'a> {
    species: &'a Species,
    /// The tree's windows, where its counts are asked: an apex that all
    /// but stopped moving on is carried on by its relay as well (host
    /// decision 13). None for the expected lengths that make them.
    windows: Option<&'a Windows>,
    memo: Memo<Asked, f64>,
    /// The stages a sleeping bud of a zone may wake in over a year, each
    /// with its first unit's law (`stages`, `First::of`), by the zone's PA
    /// and place, the bud's PA and the year: the same for every bearer.
    woken: Memo<(usize, usize, usize, u32), Wakes>,
}

/// The stages a sleeping bud may wake in over a year, each with the law
/// of its first unit.
pub(crate) type Wakes = Rc<[(Piece, First)]>;

/// A chance `Living` was asked: the bud (PA and what it carries), the
/// cycles of the tree's age left at its birth where windows are read, the
/// unit, the cycle, and the abortion after the unit when it is given.
type Asked = (
    usize,
    (u64, u64, u64, bool, bool),
    usize,
    usize,
    usize,
    Option<u64>,
);

impl<'a> Living<'a> {
    pub fn new(species: &'a Species, windows: Option<&'a Windows>) -> Self {
        Self {
            species,
            windows,
            memo: Memo::default(),
            woken: Memo::default(),
        }
    }

    pub fn species(&self) -> &'a Species {
        self.species
    }

    /// The stages a sleeping bud of PA `j` in zone `z` of PA `k` may wake
    /// in over its `s`th year, each with the law of its first unit.
    pub fn woken(&mut self, (k, z): (usize, usize), j: usize, s: u32) -> Wakes {
        let species = self.species;
        self.woken
            .entry((k, z, j, s))
            .or_insert_with(|| {
                let zone = &species.states[k].zones[z];
                stages(species, zone, j, s)
                    .into_iter()
                    .map(|p| (p, First::of(species, zone, (s, p.within), (p.pa, p.slept))))
                    .collect()
            })
            .clone()
    }

    /// The chance an apex of PA `k` whose age ended in a unit with `left`
    /// cycles of the tree's age after it moves on within its window and
    /// so is relayed as well (`grow/stop.rs`); none without windows.
    pub fn borderline(&self, k: usize, left: usize, used: f64) -> f64 {
        let (Some(windows), Some((next, go))) = (self.windows, self.species.successor(k)) else {
            return 0.0;
        };
        let relay = self.species.states[k].relay_ended;
        // From `used` of the way through the cycle it ended in, as the
        // grower reads it (`Windows::wood_at`).
        let (a, b) = (
            windows.wood_left(next, left + 1),
            windows.wood_left(next, left),
        );
        let wood = if a > 0.0 && b > 0.0 {
            (a.ln_fixed() + (b.ln_fixed() - a.ln_fixed()) * used).exp_fixed()
        } else {
            a + (b - a) * used
        };
        relay * windows.borderline(go, wood, 1.0 - relay)
    }

    /// The expected number of axes that carry on the axis a bud of PA `k`
    /// carrying `carried` grows, `m` cycles of the tree's age left at its
    /// birth, given its apex grew its unit `i` and then aborts with chance
    /// `abortion`, at the start of cycle `y` (after `i`): a chance, but
    /// for an apex that all but stopped moving on, carried on by both its
    /// continuation and its relay.
    pub fn after(
        &mut self,
        (k, carried, m): (usize, Carried, usize),
        i: usize,
        y: usize,
        abortion: f64,
    ) -> f64 {
        let state = &self.species.states[k];
        let left = if self.windows.is_some() { m } else { 0 };
        let key = (
            k,
            carried.key(state.life()),
            left,
            i,
            y,
            Some(abortion.to_bits()),
        );
        if let Some(&p) = self.memo.get(&key) {
            return p;
        }
        let unit = carried.unit(state.life(), i);
        let spent = unit.before + unit.share;
        let (relay, ended) = (state.relay, state.relay_ended);
        let p = if !unit.ends {
            let mut p = (1.0 - abortion) * self.from((k, carried, m), i + 1, y);
            if abortion * relay > 0.0 {
                p += abortion
                    * relay
                    * self.from(
                        (
                            k,
                            Carried {
                                spent,
                                aged: carried.aged + spent - carried.spent,
                                relay: false,
                                ..Carried::default()
                            },
                            m.saturating_sub(i),
                        ),
                        1,
                        y - i,
                    );
            }
            p
        } else {
            // It moves on by its continuation, or stops and may relay, in
            // this cycle where its age ended within it.
            let (onward, at) = unit.onward(i);
            let (go, on) = match self.species.successor(k) {
                Some((next, go)) => (
                    go,
                    self.from((next, onward, m.saturating_sub(at)), 1, y - at),
                ),
                None => (0.0, 0.0),
            };
            let mut p = go * on;
            // Where it ends in the cycle before `y`, the cycle a bud wakes
            // in, it bears the bud even if it stops (`carried`).
            if i + 1 == y {
                p += (1.0 - go) * (1.0 - ended);
            }
            let both = self.borderline(k, m.saturating_sub(i), unit.used);
            if (1.0 - go) * ended + both > 0.0 {
                let relayed = Carried {
                    spent,
                    aged: carried.aged + spent - carried.spent,
                    relay: true,
                    ..onward
                };
                let relays = (1.0 - go) * ended + both;
                p += relays * self.from((k, relayed, m.saturating_sub(at)), 1, y - at);
            }
            p
        };
        self.memo.insert(key, p);
        p
    }

    /// The same chance given its apex is about to grow its unit `u`. A
    /// woken bud's abortion hazard counts the time it slept (`aged`); a
    /// relay's the time its axis grew (host decision 11).
    fn from(&mut self, (k, carried, m): (usize, Carried, usize), u: usize, y: usize) -> f64 {
        if y == u {
            return 1.0;
        }
        let state = &self.species.states[k];
        if u > 1 {
            // From its unit u on, an axis is the same PA's bud u − 1
            // cycles younger, carrying the time spent (fn-209, P1).
            let before = carried.unit(state.life(), u).before;
            let rest = Carried {
                spent: before,
                aged: carried.aged + before - carried.spent,
                gone: 0.0,
                relay: false,
                rest: false,
            };
            return self.from((k, rest, m.saturating_sub(u - 1)), 1, y - (u - 1));
        }
        let carried = carried.canonical(state.life(), state.rises(), y);
        let left = if self.windows.is_some() { m } else { 0 };
        let key = (k, carried.key(state.life()), left, u, y, None);
        if let Some(&p) = self.memo.get(&key) {
            return p;
        }
        let unit = carried.unit(state.life(), u);
        let spent = unit.before + unit.share;
        let viability = share(state.viability, unit.share);
        let relay = state.relay_failed;
        let abortion = if unit.ends {
            0.0
        } else {
            let grown = carried.aged + spent - carried.spent;
            let exposure = carried.exposure(state.life(), u);
            1.0 - share(1.0 - state.abortion_at(grown), exposure)
        };
        // A failure its relay takes over in a sliver of a cycle, or in a
        // failed unit's relay's first unit, carries on (`LIVED_FLOOR`,
        // host decision A3).
        let (viability, relay) = if unit.share < LIVED_FLOOR || carried.rest {
            (viability + (1.0 - viability) * relay, 0.0)
        } else {
            (viability, relay)
        };
        let mut p = viability * self.after((k, carried, m), u, y, abortion);
        // A unit that failed grows the share of its cycle its apex lived,
        // and its relay the rest (fn-199 R1), at the schedule's points.
        if (1.0 - viability) * relay > 0.0 {
            let weight = (1.0 - viability) * relay / LIVED as f64;
            for t in lived(state.viability, unit.share) {
                let (relayed, same) = carried.failed(unit, t);
                let at = if same { u - 1 } else { u };
                p += weight * self.from((k, relayed, m.saturating_sub(at)), 1, y - at);
            }
        }
        self.memo.insert(key, p);
        p
    }
}
