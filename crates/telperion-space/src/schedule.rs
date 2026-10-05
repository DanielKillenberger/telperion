//! An age's growth units in time (fn-206, host decision 7). An apex grows
//! one unit a cycle; an age's lifespan is a real number of cycles, so its
//! last unit grows at the share of its cycle the lifespan leaves, and what
//! carries the axis on grows in the same cycle at the rest, as a woken
//! bud's first unit does. The grower, the closed form and `Living` read
//! the one schedule here.

/// What a bud carries into its age: the time it has spent in it (a relay
/// carries its axis's on), the cycles it slept that its abortion hazard
/// counts, the share of its first cycle already gone, to the age before it
/// or asleep, and whether it is the relay of an age that ended or of a
/// unit that failed, whose first unit is whole.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub(crate) struct Carried {
    pub spent: f64,
    pub aged: f64,
    pub gone: f64,
    pub relay: bool,
}

/// A share of a cycle within this of a whole one is whole: a sum of shares
/// meeting it does not start the next age in the same cycle.
const WHOLE: f64 = 1e-12;

/// Unit `i` (from 1) of a bud in an age of `lifespan` cycles.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct Unit {
    /// The age's time spent before it.
    pub before: f64,
    /// Its share of its cycle.
    pub share: f64,
    /// The age ends with it.
    pub ends: bool,
    /// The share of its cycle gone when it ends: before it and with it.
    pub used: f64,
}

impl Carried {
    /// Unit `i` of a bud carrying this into an age of `lifespan` cycles,
    /// the age not having ended before it. The relay of an age that ended,
    /// or of a unit that failed, grows one whole unit, the rest of its
    /// first cycle, however little of its lifespan is left; the relay of
    /// an apex that aborted grows what the apex would have (host decision
    /// 11); a new bud's first unit is the share its lifespan leaves,
    /// vanishing with it.
    pub fn unit(self, lifespan: f64, i: usize) -> Unit {
        let gone = if i == 1 { self.gone } else { 0.0 };
        let before = if i == 1 {
            self.spent
        } else {
            self.spent + (1.0 - self.gone) + (i - 2) as f64
        };
        let room = 1.0 - gone;
        let left = lifespan - before;
        let share = if i == 1 && self.relay {
            room
        } else {
            room.min(left)
        };
        let ends = left <= room + WHOLE;
        Unit {
            before,
            share,
            ends,
            used: gone + share,
        }
    }

    /// The share of a cycle an apex is exposed to abortion after unit `i`,
    /// the age going on past it: the lesser of that unit's share and the
    /// next one's, so the chance vanishes with a unit that is all but
    /// gone, whether it is the first, grown in the last of a cycle, or the
    /// next, the last of a lifespan.
    pub fn exposure(self, lifespan: f64, i: usize) -> f64 {
        let this = self.unit(lifespan, i).share;
        this.min(self.unit(lifespan, i + 1).share)
    }

    /// Past its lifespan an age's time changes nothing (every bud grows
    /// one unit and ends), so a key may hold it there.
    pub fn key(self, lifespan: f64) -> (u64, u64, u64, bool) {
        (
            self.spent.min(lifespan).to_bits(),
            self.aged.to_bits(),
            self.gone.to_bits(),
            self.relay,
        )
    }
}

impl Unit {
    /// Whether what carries the axis on after it grows in its cycle: the
    /// cycle is not whole yet.
    pub fn same_cycle(self) -> bool {
        self.used < 1.0 - WHOLE
    }

    /// What carries the axis on after it into a new age: in this cycle at
    /// the rest of it, or from the next; and the cycle offset its first
    /// unit stands at, from this unit's `i`.
    pub fn onward(self, i: usize) -> (Carried, usize) {
        if self.same_cycle() {
            let carried = Carried {
                gone: self.used,
                ..Carried::default()
            };
            (carried, i - 1)
        } else {
            (Carried::default(), i)
        }
    }
}
