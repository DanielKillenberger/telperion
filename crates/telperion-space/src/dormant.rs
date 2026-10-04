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
use crate::closed_form::{spent_key, units_left};
use crate::lineage::{self, Key, DORMANT};
use crate::presence::Draws;
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
    pub whorl: u8,
    pub pa: usize,
    /// Its lineage, and its draw's lead past its bound.
    pub key: u64,
    pub lead: f64,
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
        wood: woods[pa],
        sleep: years - whole,
        slept: whole as u32,
    };
    Some(((cycle + 1) as usize + whole as usize, sleeper))
}

/// The presence of the draws that kept the bud's bearer alive from its
/// node's growth unit to now: the apex persisting past that unit, each
/// later unit's survival and persistence, and each continuation or relay
/// that carried the axis on. None when nothing carries it on any more.
pub(crate) fn carried(
    axes: &[Axis],
    draws: &[Draws],
    successor: &[Option<usize>],
    sleeper: &Sleeper,
) -> Option<f64> {
    let mut axis = sleeper.parent;
    let units = &draws[axis].units;
    let mut presence = units[sleeper.unit][1];
    let mut from = sleeper.unit + 1;
    loop {
        presence *= draws[axis].units[from..]
            .iter()
            .map(|[survive, persist]| survive * persist)
            .product::<f64>();
        match successor[axis] {
            Some(next) => {
                presence *= draws[next].birth[0] * draws[next].birth[1];
                axis = next;
                from = 0;
            }
            None => return axes[axis].apex_end.is_none().then_some(presence),
        }
    }
}

/// The stage a bud of PA `pa` that slept `slept` years wakes in, and the
/// units it carries into it: a dormant bud ages in the bark as its axis
/// does (host, 2026-10-04), passing each stage whose lifespan it slept
/// through. None when it slept past its last.
pub(crate) fn aged(species: &Species, mut pa: usize, mut slept: u32) -> Option<(usize, u32)> {
    loop {
        let state = &species.states[pa];
        if slept < state.lifespan {
            return Some((pa, slept));
        }
        slept -= state.lifespan;
        pa = state.next?;
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
    /// For a bud woken in the `s`th year after its node grew, into `pa`
    /// carrying `spent` units.
    pub fn of(species: &Species, zone: &Zone, s: u32, (pa, spent): (usize, u32)) -> Self {
        let state = &species.states[pa];
        let survive = shared(zone, s, state.viability);
        let persist = if units_left(state.lifespan, spent) > 1 {
            shared(zone, s, state.viability * (1.0 - state.abortion_at(1)))
        } else {
            survive
        };
        Self { survive, persist }
    }
}

/// The mean of `c` to the share of the year a bud woken in its `s`th year
/// still grows, its waking time past `s` distributed as the release law
/// gives it there.
fn shared(zone: &Zone, s: u32, c: f64) -> f64 {
    if c <= 0.0 {
        return 0.0;
    }
    let r = zone.rate;
    // Its waking time past s lies in [x0, 1), with density in
    // proportion to exp(-r x); its share is 1 - x.
    let x0 = (zone.delay - f64::from(s)).max(0.0);
    let d = 1.0 - x0;
    let lambda = c.ln() + r;
    let spread = |rate: f64| {
        if rate.abs() < 1e-12 {
            d
        } else {
            -(-rate * d).exp_m1() / rate
        }
    };
    r * c.powf(d) * spread(lambda) / spread(r) / r
}

/// The closed form of `carried`: the chance that the axis a bud grows is
/// still carried on, by its apex or a continuation or relay of it, at the
/// start of a cycle. Cycles count from the bud's birth, its first growth
/// unit growing in cycle 1, as the closed form's do.
pub(crate) struct Living<'a> {
    species: &'a Species,
    memo: HashMap<(usize, u32, usize, usize, Option<u64>), f64>,
}

impl<'a> Living<'a> {
    pub fn new(species: &'a Species) -> Self {
        Self {
            species,
            memo: HashMap::new(),
        }
    }

    /// The chance for a bud of PA `k` carrying `spent` units, given its
    /// apex grew its unit `i` and then aborts with chance `abortion`, that
    /// its axis is carried on at the start of cycle `y` (after `i`).
    pub fn after(&mut self, k: usize, spent: u32, i: usize, y: usize, abortion: f64) -> f64 {
        let state = &self.species.states[k];
        let spent = spent_key(state.lifespan, spent);
        let key = (k, spent, i, y, Some(abortion.to_bits()));
        if let Some(&p) = self.memo.get(&key) {
            return p;
        }
        let n = units_left(state.lifespan, spent);
        let relay = state.relay;
        let p = if i < n {
            let mut p = (1.0 - abortion) * self.from(k, spent, i + 1, y);
            if abortion * relay > 0.0 {
                p += abortion * relay * self.from(k, spent + i as u32, 1, y - i);
            }
            p
        } else {
            match state.next {
                Some(next) => self.from(next, 0, 1, y - n),
                None if relay > 0.0 => relay * self.from(k, spent + n as u32, 1, y - n),
                None => 0.0,
            }
        };
        self.memo.insert(key, p);
        p
    }

    /// The same chance given its apex is about to grow its unit `u`.
    fn from(&mut self, k: usize, spent: u32, u: usize, y: usize) -> f64 {
        if y == u {
            return 1.0;
        }
        let state = &self.species.states[k];
        let spent = spent_key(state.lifespan, spent);
        if let Some(&p) = self.memo.get(&(k, spent, u, y, None)) {
            return p;
        }
        let (viability, relay) = (state.viability, state.relay);
        let abortion = if u < units_left(state.lifespan, spent) {
            state.abortion_at(u)
        } else {
            0.0
        };
        let mut p = viability * self.after(k, spent, u, y, abortion);
        // A unit that failed is spent; its relay grows from the next cycle.
        if (1.0 - viability) * relay > 0.0 {
            p += (1.0 - viability) * relay * self.from(k, spent + u as u32, 1, y - u);
        }
        self.memo.insert((k, spent, u, y, None), p);
        p
    }
}
