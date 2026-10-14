//! An age's growth units in time (fn-206, host decision 7). An apex grows
//! one unit a cycle; an age's lifespan is a real number of cycles, so its
//! last unit grows at the share of its cycle the lifespan leaves, and what
//! carries the axis on grows in the same cycle at the rest, as a woken
//! bud's first unit does. The grower, the closed form and `Living` read
//! the one schedule here.
use crate::species::{PaState, Species};
use crate::transcendental::Transcendental;

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
    /// It is a failed unit's relay, whose first unit, the rest of that
    /// cycle, carries on where its own failure relays (host decision A3).
    pub rest: bool,
}

/// A unit a lifespan's fraction cuts (`Unit::laid`): its share of a cycle
/// and the length `of` it is cut from, both on the tick grid.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct Cut {
    pub share: f64,
    pub of: f64,
}

impl Cut {
    /// The share of each zone's nodes it lays.
    pub fn laid(self) -> f64 {
        self.share / self.of
    }
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
    /// It is the whole first unit of a relay of an age that ended.
    pub whole: bool,
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
        let whole = i == 1 && self.relay;
        let share = if whole { room } else { room.min(left) };
        let ends = left <= room + WHOLE;
        Unit {
            before,
            share,
            ends,
            used: gone + share,
            whole,
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

    /// The same, as far as it changes what grows in the `m` cycles left
    /// (fn-209, P1): an age that cannot end within them grows alike at any
    /// time spent; a hazard that does not rise counts no aged time; a
    /// relay's whole first unit is the unit any bud grows when the age
    /// leaves it the room. So buds that grow alike share their tables.
    pub fn canonical(self, lifespan: f64, rises: bool, m: usize) -> Self {
        let mut out = self;
        let far = m as f64 + 3.0;
        if lifespan - self.spent > far {
            out.spent = (lifespan - far).max(0.0);
            out.aged = self.aged + out.spent - self.spent;
        }
        if !rises {
            out.aged = out.spent;
        }
        if lifespan - out.spent >= 1.0 - out.gone {
            out.relay = false;
        }
        out
    }

    /// Past its lifespan an age's time changes nothing (every bud grows
    /// one unit and ends), so a key may hold it there.
    pub fn key(self, lifespan: f64) -> (u64, u64, u64, bool, bool) {
        (
            self.spent.min(lifespan).to_bits(),
            self.aged.to_bits(),
            self.gone.to_bits(),
            self.relay,
            self.rest,
        )
    }
}

/// The points a failed unit's lived share is taken at in the closed form
/// (fn-199 R1, host 2026-10-07): the midpoints of `LIVED` equal parts of
/// its distribution, each with that part's chance. One, the fewest that
/// meet the comparison tests: two cost the predictor 7 to 115 times its
/// time, and it runs on every change of a setting (host, decision A).
pub(crate) const LIVED: usize = 1;

/// The least share of a cycle a failed unit's lived parts are taken for in
/// expectation: below it a failure its relay takes over is counted as the
/// apex carrying on, which it all but is, so the parts' relays, each the
/// rest of a shorter cycle, end.
pub(crate) const LIVED_FLOOR: f64 = 1.0 / 256.0;

/// The lived shares of a unit of `share` that failed under survival
/// `viability` a cycle, at `LIVED` points: its time `t` given failure
/// runs as `(1 - v^t) / (1 - v^share)`, inverted at each part's middle,
/// on the tick grid. The grower's `contract::decide::lived_at`, in expectation.
pub(crate) fn lived(viability: f64, share: f64) -> [f64; LIVED] {
    std::array::from_fn(|j| {
        if viability >= 1.0 || share <= 0.0 {
            return share;
        }
        if viability <= 0.0 {
            return 0.0;
        }
        let u = (j as f64 + 0.5) / LIVED as f64;
        let failing = 1.0 - viability.powf_fixed(share);
        let t = (1.0 - u * failing).ln_fixed() / viability.ln_fixed();
        crate::contract::grid(t.clamp(0.0, share))
    })
}

impl Carried {
    /// What the relay of unit `unit`, which failed `lived` into its share,
    /// carries: the apex's time on from that moment, and the rest of its
    /// cycle where any is left; whether it grows in that cycle. The relay
    /// of a whole unit grows that unit's rest whole, past the lifespan as
    /// the apex would have, so its share is never below none.
    pub fn failed(self, unit: Unit, lived: f64) -> (Self, bool) {
        let spent = unit.before + lived;
        let used = unit.used - unit.share + lived;
        let same = used < 1.0 - WHOLE;
        let relay = Carried {
            spent,
            aged: self.aged + spent - self.spent,
            gone: if same { used } else { 0.0 },
            relay: unit.whole,
            rest: same,
        };
        (relay, same)
    }
}

impl Carried {
    /// The chance a failed unit's relay carries its axis on, its unit
    /// `unit`, the `i`th, having lived `t`: the lived part takes the
    /// unit's abortion draw over `t` and the next unit's share, and where
    /// it aborts the relay stops at once and the abortion's relay, which
    /// carries the same, takes over (fn-199, host decision C1). Whole
    /// where the age ends with the unit.
    pub fn lived_on(self, state: &PaState, (i, unit): (usize, Unit), t: f64) -> f64 {
        if unit.ends {
            return 1.0;
        }
        let grown = self.aged + unit.before + t - self.spent;
        let exposure = t.min(self.unit(state.life(), i + 1).share);
        let kept = 1.0 - state.abortion_at(grown);
        let persists = if exposure < 1.0 {
            kept.powf_fixed(exposure)
        } else {
            kept
        };
        1.0 - (1.0 - persists) * (1.0 - state.relay)
    }
}

impl Unit {
    /// Where a lifespan's fraction cuts it (fn-231, host 2026-10-08), its
    /// share and the length it is cut from: it lays `share / of` of each
    /// zone's nodes, each `of` of a whole node's length. A successor's
    /// first unit, the rest of the cycle its age before left (its
    /// continuation's or its ended age's relay's), is cut from a whole
    /// unit. The age's last, ending inside the room its cycle
    /// leaves after `gone`, is cut from that room, whose own shortness a
    /// draw made (a failed unit's relay, a woken bud), so the cut grows
    /// from none as the age's end passes the room's. None for every other
    /// unit, which lays all its nodes at its share, and for a woken bud's
    /// first unit, whose share its waking draws (`First`).
    pub fn laid(self, gone: f64, (successor, woken): (bool, bool)) -> Option<Cut> {
        let room = 1.0 - gone;
        match () {
            _ if woken => None,
            _ if successor && gone > 0.0 => Some(Cut {
                share: self.share,
                of: 1.0,
            }),
            _ if self.ends && self.share < room => Some(Cut {
                share: self.share,
                of: room,
            }),
            _ => None,
        }
    }

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

/// The most growth units one job grows in a cycle (fn-276, host
/// 2026-10-14): a living apex, or a waking bud on one axis that carries
/// it, from the age it stands in through each later age it enters
/// within the cycle. An age is entered while those entered before it end
/// within the cycle, their lifespans summing to one cycle at most on the
/// tick grid. Each age grows one unit, and one more where the unit can
/// fail and a relay grows the rest of it (`relay_failed`). Where an age
/// ends within the cycle, the relay its end can leave (`relay_ended`)
/// grows the rest of the cycle whole, and its own failure's relay: beside
/// the continuation where that is not certain to move on, or in its
/// place where the apex stops. A bound, never a count.
pub fn most_units(species: &Species) -> u32 {
    let grows = |a: usize| {
        let s = &species.states[a];
        1 + u32::from(s.relay_failed > 0.0 && s.viability < 1.0)
    };
    let relays = |a: usize| if species.states[a].relay_ended > 0.0 { grows(a) } else { 0 };
    // Whether an apex whose age ended may stop rather than move on.
    let stops = |a: usize| species.successor(a).is_none_or(|(_, go)| go < 1.0);
    let ticks = |a: usize| crate::contract::ticks(species.states[a].life());
    let cycle = crate::contract::ticks(1.0);
    let mut most = 1;
    for first in (0..species.states.len()).filter(|&a| species.states[a].lifespan > 0.0) {
        let (mut at, mut sum, mut passed) = (first, grows(first), 0u64);
        loop {
            // The age entered last ends within the cycle: the first
            // always may, a later one where its lifespan fits.
            let ends = at == first || passed.saturating_add(ticks(at)) <= cycle;
            if ends && stops(at) {
                most = most.max(sum + relays(at));
            }
            most = most.max(sum);
            let Some((next, go)) = species.successor(at).filter(|_| ends) else {
                break;
            };
            if at != first {
                passed = passed.saturating_add(ticks(at));
            }
            if go < 1.0 {
                sum += relays(at);
            }
            sum += grows(next);
            at = next;
        }
    }
    most
}
