//! Sleeping buds (fn-202). A bud's place on a node may also hold a bud
//! that sleeps and wakes years later, as the buds along an old spruce
//! bough release the branchlets that fill its curtain and an oak's along
//! its limbs release epicormic shoots. Whether a place holds one and when
//! it wakes are drawn under their own key on the node's lineage. Its
//! waking time is continuous: the zone's delay, then an exponential wait
//! at its rate. It wakes in the cycle that time falls in, and the share of
//! that cycle it still slept takes from its first growth unit, so a
//! release law that moves its waking moves its growth by degree.
//! It wakes only while the axis that bears it lives, through that axis's
//! continuations and relays, and its size carries every draw on the way.
use crate::fixed::decide::node_window;
use crate::fixed::presence::{q, real, times, ONE};
use crate::lineage::{self, Key, DORMANT};
use crate::presence::Draws;
use crate::schedule::Carried;
use crate::species::{Species, Zone};
use crate::structure::Axis;
use crate::transcendental::Transcendental;

/// The probability that a sleeping bud wakes in the `s`th whole year after
/// its node grew: its waking time lies in [s, s + 1).
pub(crate) fn wakes_in(zone: &Zone, s: u32) -> f64 {
    if zone.rate <= 0.0 {
        return 0.0;
    }
    // The probability that it still sleeps at `years`.
    let asleep = |years: f64| (-zone.rate * (years - zone.delay).max(0.0)).exp_fixed();
    asleep(f64::from(s)) - asleep(f64::from(s) + 1.0)
}

impl Sleeper {
    /// The presences, Q32 (host decision 8), of the draws that made the
    /// bud as it wakes, `carried` there and reaching its stage by `reach`:
    /// its own draw's, by its place's share, and its bearer's carrying.
    pub fn made(&self, carried: f64, reach: f64) -> [f64; 2] {
        [times(self.own, q(self.placed)), times(q(carried), q(reach))].map(real)
    }
}

/// A sleeping bud that will wake: where it stands and what it was drawn.
#[derive(Debug, Clone, Copy)]
pub(crate) struct Sleeper {
    pub parent: usize,
    /// Its node, and the index of the growth unit that grew it.
    pub node: usize,
    pub unit: usize,
    pub slot: u8,
    pub whorl: f64,
    pub pa: usize,
    /// Its lineage, its own draw's presence, Q32, over the window of the
    /// expected wood it decides, and the share its bud place stands at (1
    /// for a whole place).
    pub key: u64,
    pub own: u64,
    pub placed: f64,
    /// The share of its first cycle it still slept, and the whole years
    /// before it, which it aged through.
    pub sleep: f64,
    pub slept: u32,
}

/// The sleeping bud of the bud place keyed `place`, slot `slot` of the
/// node at `at` (its axis, node and growth unit) grown in `cycle`, if it
/// holds one that wakes within the tree's `age`: the cycle it first grows
/// in, and the bud.
pub(crate) fn draw(
    zone: &Zone,
    place: Key,
    ((parent, node, unit), slot): ((usize, usize, usize), u8),
    woods: &[f64],
    (cycle, age): (u32, u32),
) -> Option<(usize, Sleeper)> {
    let key = place.child(DORMANT);
    let pa = lineage::bud(&zone.dormant, key.draw())?;
    // By the inverse of the release law's distribution, on the tick grid
    // (FINE-CONTRACT.md D11).
    let years = crate::fixed::decide::wake(zone, key.child(DORMANT).draw());
    if years >= f64::from(age - cycle) {
        return None;
    }
    let whole = years.floor();
    let sleeper = Sleeper {
        parent,
        node,
        unit,
        slot,
        whorl: zone.buds,
        pa,
        key: key.0,
        own: lineage::bud_presence(&zone.dormant, pa, key.draw(), node_window(woods[pa])),
        placed: 1.0,
        sleep: years - whole,
        slept: whole as u32,
    };
    Some(((cycle + 1) as usize + whole as usize, sleeper))
}

/// The presence of the draws that kept the bud's bearer alive from its
/// node's growth unit to now: the apex persisting past that unit, each
/// later unit's survival and persistence, and each continuation or relay
/// that carried the axis on; with each axis that carries it on now. None
/// where nothing carries it on any more; more than one where a stop all
/// but made grew both its continuation and its relay (host decision 11).
pub(crate) fn carried(
    axes: &[Axis],
    draws: &[Draws],
    successor: &[Vec<usize>],
    sleeper: &Sleeper,
    cycle: u32,
) -> Vec<(f64, usize)> {
    let axis = sleeper.parent;
    let presence = q(draws[axis].units[sleeper.unit].persist);
    let mut open = vec![(presence, axis, sleeper.unit + 1)];
    let mut out = Vec::new();
    // Each presence Q32, its products floored in turn (host decision 8).
    while let Some((mut presence, axis, from)) = open.pop() {
        for u in &draws[axis].units[from..] {
            presence = times(times(presence, q(u.survive)), q(u.persist));
        }
        let presence_real = real(presence);
        if successor[axis].is_empty() {
            // A bearer whose age ended within the waking cycle bears it,
            // as far as it lived past the waking (`Woken::kept`), so a
            // lifespan crossing a whole cycle moves it by degree (Codex
            // round 5 on fn-206).
            let ended_here = axes[axis].apex_end == Some(cycle) && draws[axis].outlived.is_some();
            if axes[axis].apex_end.is_none() || ended_here {
                out.push((presence_real, axis));
            }
            continue;
        }
        for &next in successor[axis].iter().rev() {
            open.push((
                times(
                    times(presence, q(draws[next].birth[0])),
                    q(draws[next].birth[1]),
                ),
                next,
                0,
            ));
        }
    }
    out
}

/// The stage a bud of PA `pa` that slept `slept` years, a real time,
/// wakes in, the time it has spent in it, and the share of it that reaches
/// that stage: a dormant bud ages in the bark as its axis does (host,
/// 2026-10-04), passing each stage whose lifespan it slept through, by its
/// continuation, its lifespans real (host decision 7). None when it slept
/// past its last.
pub(crate) fn aged(species: &Species, pa: usize, slept: f64) -> Option<(usize, f64, f64)> {
    let (mut pa, mut reach) = species.lived(pa)?;
    let mut slept = slept;
    loop {
        let state = &species.states[pa];
        if slept < state.life() {
            return Some((pa, slept, reach));
        }
        slept -= state.life();
        let (next, go) = species.successor(pa)?;
        (pa, reach) = (next, reach * go);
    }
}

/// The law of a woken bud's partial first unit, which runs only its share
/// of the year's risks: the chance it survives (viability to the share),
/// and that it survives and does not then abort, each averaged over where
/// in its year it woke.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct First {
    pub survive: f64,
    pub persist: f64,
}

impl First {
    /// For a bud woken in the `s`th year after its node grew, between
    /// `within` of the way through it, into `pa` having spent `spent` of
    /// it.
    pub fn of(
        species: &Species,
        zone: &Zone,
        (s, within): (u32, (f64, f64)),
        (pa, spent): (usize, f64),
    ) -> Self {
        let state = &species.states[pa];
        // Woken `x` past the year's start, as the grower wakes it: its time
        // in the stage holds the share of the cycle it slept, its first
        // unit the share its lifespan and its cycle leave, its hazard over
        // that unit's exposure (codex round 3 on fn-206).
        let at = |x: f64| -> (f64, f64) {
            let woken = Carried {
                spent: spent + x,
                aged: spent + x,
                gone: x,
                relay: false,
            };
            let unit = woken.unit(state.life(), 1);
            let survive = share(state.viability, unit.share);
            if unit.ends {
                return (survive, survive);
            }
            let grown = woken.aged + unit.share;
            let exposure = woken.exposure(state.life(), 1);
            let abortion = 1.0 - share(1.0 - state.abortion_at(grown), exposure);
            (survive, survive * (1.0 - abortion))
        };
        let (survive, persist) = mean(zone, (s, within), at);
        Self { survive, persist }
    }
}

/// The mean of `f` over the waking time past the start of a bud's `s`th
/// year, between `within` of the way through it, distributed as the
/// release law gives it there: by the midpoint rule, which never samples
/// the part's ends, where a stage ends and its share is none (Codex round
/// 4 on fn-206), fine enough that the closed form's counts hold to the
/// engine's to a millionth.
fn mean(
    zone: &Zone,
    (s, (from, to)): (u32, (f64, f64)),
    f: impl Fn(f64) -> (f64, f64),
) -> (f64, f64) {
    const STEPS: usize = 512;
    let r = zone.rate;
    let x0 = (zone.delay - f64::from(s)).max(from);
    if to - x0 <= 0.0 {
        return f(x0);
    }
    let h = (to - x0) / STEPS as f64;
    let (mut a, mut b, mut weight) = (0.0, 0.0, 0.0);
    for k in 0..STEPS {
        let x = x0 + h * (k as f64 + 0.5);
        let w = (-r * (x - x0)).exp_fixed();
        let (p, q) = f(x);
        a += w * p;
        b += w * q;
        weight += w;
    }
    (a / weight, b / weight)
}

/// A stage a sleeping bud may wake in over one year (`stages`).
#[derive(Debug, Clone, Copy)]
pub(crate) struct Piece {
    /// The part of the year that wakes it there.
    pub within: (f64, f64),
    /// The chance it wakes in that part.
    pub wakes: f64,
    pub pa: usize,
    /// Its time in the stage at the year's start: before the stage began
    /// where it begins within the year, so its first unit's schedule and
    /// abortion hazard run as the grower's do from the waking.
    pub slept: f64,
    /// The share of it that reaches the stage.
    pub reach: f64,
}

/// The stages a bud of PA `pa` may wake in over its `s`th year after its
/// node grew: each with the part of the year that wakes it there, the
/// chance it wakes in that part, the time it has spent in the stage at
/// the year's start (less than none for a stage it enters within it) and the share
/// of it that reaches the stage. A stage boundary within the year splits
/// it, so no bud wakes in a stage it has outlived.
pub(crate) fn stages(species: &Species, zone: &Zone, pa: usize, s: u32) -> Vec<Piece> {
    let mut out = Vec::new();
    let start = f64::from(s);
    let Some((mut at, mut reach)) = species.lived(pa) else {
        return out;
    };
    // The time the stage `at` began.
    let mut began = 0.0;
    loop {
        let ends = began + species.states[at].life();
        let (lo, hi) = (start.max(began), (start + 1.0).min(ends));
        if hi > lo {
            let wakes = asleep(zone, lo) - asleep(zone, hi);
            if wakes > 0.0 {
                let within = (lo - start, hi - start);
                out.push(Piece {
                    within,
                    wakes,
                    pa: at,
                    slept: start - began,
                    reach,
                });
            }
        }
        if ends >= start + 1.0 {
            return out;
        }
        let Some((next, go)) = species.successor(at) else {
            return out;
        };
        (at, reach, began) = (next, reach * go, ends);
    }
}

/// The chance a sleeping bud still sleeps `years` after its node grew.
fn asleep(zone: &Zone, years: f64) -> f64 {
    if zone.rate <= 0.0 {
        return 1.0;
    }
    (-zone.rate * (years - zone.delay).max(0.0)).exp_fixed()
}

/// A bud that woke in `cycle`, on a bearer carried on by axis `bearer`,
/// whose next growth unit is `next`; `share` is the part of the cycle it
/// grew in.
#[derive(Debug, Clone, Copy)]
pub(crate) struct Woken {
    pub axis: usize,
    pub bearer: usize,
    pub next: usize,
    pub cycle: u32,
    pub share: f64,
}

impl Woken {
    /// How much of its size a bud that woke keeps: all of it as far as
    /// its bearer then carries on through the next cycle (`p`), and the
    /// part of its waking cycle it grew otherwise, share + (1 - share) p,
    /// so a bud that woke just before its bearer's end is small (host,
    /// 2026-10-04). A Q32 presence, its sums and products floored in
    /// turn (host decision 8).
    pub fn kept(&self, axes: &[Axis], draws: &[Draws], successor: &[Vec<usize>], age: u32) -> f64 {
        let bearer = &draws[self.bearer];
        let share = q(self.share);
        // A bearer whose age ended within the waking cycle: the part of
        // the cycle it lived past the waking (times on the tick grid).
        if let (Some(used), Some(end)) = (bearer.outlived, axes[self.bearer].apex_end) {
            if end == self.cycle && successor[self.bearer].is_empty() {
                return real((q(used) + share).saturating_sub(ONE));
            }
        }
        let rest = |p: u64| real(share + times(ONE - share, p));
        // In the tree's last cycle, as far as its bearer lives on past the
        // tree's age (`Grower::living`), so a bearer whose age ends just
        // after it keeps the bud as one ending just before it does (Codex
        // round 6 on fn-206).
        if self.cycle >= age {
            return rest(if bearer.alive { q(bearer.living) } else { ONE });
        }
        let on = |next: &[usize]| {
            next.iter()
                .map(|&n| times(q(draws[n].birth[0]), q(draws[n].birth[1])))
                .sum::<u64>()
        };
        let p = match bearer.units.get(self.next) {
            // It died before its next unit: as far as a relay took over.
            None => on(&successor[self.bearer]),
            // It grew it, and stopped after it, or carried on.
            Some(unit) => {
                let stopped = axes[self.bearer].apex_end == Some(self.cycle + 1) && !bearer.failed;
                let carried = times(q(unit.survive), q(unit.persist));
                match bearer.outlived {
                    // Its age ended within that unit's cycle and nothing
                    // carries it on: it bears the bud as far as it lived
                    // into the cycle, as it does a bud that wakes in that
                    // cycle, so the two meet (fn-199).
                    Some(used) if stopped && successor[self.bearer].is_empty() => {
                        times(carried, q(used))
                    }
                    _ if stopped => times(carried, on(&successor[self.bearer])),
                    _ => carried,
                }
            }
        };
        rest(p)
    }
}

/// A survival over a share of a cycle: to the share, itself whole.
fn share(p: f64, share: f64) -> f64 {
    if share < 1.0 {
        p.powf_fixed(share)
    } else {
        p
    }
}

mod living;
pub(crate) use living::Living;
