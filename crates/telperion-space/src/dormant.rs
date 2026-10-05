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
use crate::lineage::{self, Key, DORMANT};
use crate::presence::{Draws, Windows};
use crate::schedule::Carried;
use crate::species::{Species, Zone};
use crate::structure::Axis;
use std::collections::HashMap;

/// The years after its node grew that a sleeping bud wakes, for the draw
/// `u`, by the inverse of the release law's distribution; infinite when it
/// never wakes.
pub(crate) fn wake(zone: &Zone, u: f64) -> f64 {
    if zone.rate <= 0.0 {
        return f64::INFINITY;
    }
    zone.delay - (-u).ln_1p() / zone.rate
}

/// The probability that a sleeping bud wakes in the `s`th whole year after
/// its node grew: its waking time lies in [s, s + 1).
pub(crate) fn wakes_in(zone: &Zone, s: u32) -> f64 {
    if zone.rate <= 0.0 {
        return 0.0;
    }
    // The probability that it still sleeps at `years`.
    let asleep = |years: f64| (-zone.rate * (years - zone.delay).max(0.0)).exp();
    asleep(f64::from(s)) - asleep(f64::from(s) + 1.0)
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
    /// Its lineage, its draw's lead past its bound, and the share its bud
    /// place stands at (1 for a whole place).
    pub key: u64,
    pub lead: f64,
    pub placed: f64,
    /// The expected wood it decides, as a share of the tree's.
    pub wood: f64,
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
    let (pa, lead) = lineage::bud(&zone.dormant, key.unit())?;
    let years = wake(zone, key.child(DORMANT).unit());
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
        lead,
        placed: 1.0,
        wood: woods[pa],
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
    let presence = draws[axis].units[sleeper.unit][1];
    let mut open = vec![(presence, axis, sleeper.unit + 1)];
    let mut out = Vec::new();
    while let Some((mut presence, axis, from)) = open.pop() {
        presence *= draws[axis].units[from..]
            .iter()
            .map(|[survive, persist]| survive * persist)
            .product::<f64>();
        if successor[axis].is_empty() {
            // A bearer whose age ended within the waking cycle bears it,
            // as far as it lived past the waking (`Woken::kept`), so a
            // lifespan crossing a whole cycle moves it by degree (Codex
            // round 5 on fn-206).
            let ended_here = axes[axis].apex_end == Some(cycle) && draws[axis].outlived.is_some();
            if axes[axis].apex_end.is_none() || ended_here {
                out.push((presence, axis));
            }
            continue;
        }
        for &next in successor[axis].iter().rev() {
            open.push((
                presence * draws[next].birth[0] * draws[next].birth[1],
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
        if slept < state.lifespan {
            return Some((pa, slept, reach));
        }
        slept -= state.lifespan;
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
            let unit = woken.unit(state.lifespan, 1);
            let survive = share(state.viability, unit.share);
            if unit.ends {
                return (survive, survive);
            }
            let grown = woken.aged + unit.share;
            let exposure = woken.exposure(state.lifespan, 1);
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
        let w = (-r * (x - x0)).exp();
        let (p, q) = f(x);
        a += w * p;
        b += w * q;
        weight += w;
    }
    (a / weight, b / weight)
}

/// A stage a sleeping bud may wake in over one year (`stages`).
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
        let ends = began + species.states[at].lifespan;
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
    (-zone.rate * (years - zone.delay).max(0.0)).exp()
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
    /// 2026-10-04).
    pub fn kept(&self, axes: &[Axis], draws: &[Draws], successor: &[Vec<usize>], age: u32) -> f64 {
        let bearer = &draws[self.bearer];
        // A bearer whose age ended within the waking cycle: the part of
        // the cycle it lived past the waking.
        if let (Some(used), Some(end)) = (bearer.outlived, axes[self.bearer].apex_end) {
            if end == self.cycle && successor[self.bearer].is_empty() {
                return (used + self.share - 1.0).max(0.0);
            }
        }
        // In the tree's last cycle, as far as its bearer lives on past the
        // tree's age (`Grower::living`), so a bearer whose age ends just
        // after it keeps the bud as one ending just before it does (Codex
        // round 6 on fn-206).
        if self.cycle >= age {
            let on = if bearer.alive { bearer.living } else { 1.0 };
            return self.share + (1.0 - self.share) * on;
        }
        let on = |next: &[usize]| {
            next.iter()
                .map(|&n| draws[n].birth[0] * draws[n].birth[1])
                .sum::<f64>()
        };
        let p = match bearer.units.get(self.next) {
            // It died before its next unit: as far as a relay took over.
            None => on(&successor[self.bearer]),
            // It grew it, and stopped after it, or carried on.
            Some([survive, persist]) => {
                let stopped = axes[self.bearer].apex_end == Some(self.cycle + 1) && !bearer.failed;
                survive
                    * persist
                    * if stopped {
                        on(&successor[self.bearer])
                    } else {
                        1.0
                    }
            }
        };
        self.share + (1.0 - self.share) * p
    }
}

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
    memo: HashMap<Asked, f64>,
}

/// A chance `Living` was asked: the bud (PA and what it carries), the
/// cycles of the tree's age left at its birth where windows are read, the
/// unit, the cycle, and the abortion after the unit when it is given.
type Asked = (
    usize,
    (u64, u64, u64, bool),
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
            memo: HashMap::new(),
        }
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
            (a.ln() + (b.ln() - a.ln()) * used).exp()
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
            carried.key(state.lifespan),
            left,
            i,
            y,
            Some(abortion.to_bits()),
        );
        if let Some(&p) = self.memo.get(&key) {
            return p;
        }
        let unit = carried.unit(state.lifespan, i);
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
        let left = if self.windows.is_some() { m } else { 0 };
        let key = (k, carried.key(state.lifespan), left, u, y, None);
        if let Some(&p) = self.memo.get(&key) {
            return p;
        }
        let unit = carried.unit(state.lifespan, u);
        let spent = unit.before + unit.share;
        let viability = share(state.viability, unit.share);
        let relay = state.relay_failed;
        let abortion = if unit.ends {
            0.0
        } else {
            let grown = carried.aged + spent - carried.spent;
            let exposure = carried.exposure(state.lifespan, u);
            1.0 - share(1.0 - state.abortion_at(grown), exposure)
        };
        let mut p = viability * self.after((k, carried, m), u, y, abortion);
        // A unit that failed is spent; its relay grows from the next cycle.
        if (1.0 - viability) * relay > 0.0 {
            let relayed = Carried {
                spent,
                aged: carried.aged + spent - carried.spent,
                relay: true,
                ..Carried::default()
            };
            let born = m.saturating_sub(u);
            p += (1.0 - viability) * relay * self.from((k, relayed, born), 1, y - u);
        }
        self.memo.insert(key, p);
        p
    }
}

/// A survival over a share of a cycle: to the share, itself whole.
fn share(p: f64, share: f64) -> f64 {
    if share < 1.0 {
        p.powf(share)
    } else {
        p
    }
}
