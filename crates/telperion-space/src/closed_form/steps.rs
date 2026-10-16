//! A bud's own axis, growth unit by growth unit, as the closed form meets
//! it: the chance it reaches each unit and grows it, what stops it after,
//! and the relays and continuation that carry it on. The flat events the
//! counts and lengths read (`events.rs`) and the kept quantities, which
//! need the path a bud's axis took (`kept.rs`), both read these steps, so
//! the schedule and its chances are worked out in one place.
use super::along::{Along, Count};
use super::events::Event;
use crate::dormant::{First, Living};
use crate::schedule::{lived, Carried, LIVED, LIVED_FLOOR};
use crate::species::{Species, ALONG};
use crate::transcendental::Transcendental;

/// A relay a stop leaves: the cycle offset it is made in (its first unit
/// grows in the next), what it carries, its unconditional chance, and the
/// unit it stands on (0: the axis's base).
#[derive(Debug, Clone, Copy)]
pub(super) struct Relay {
    pub at: usize,
    pub carried: Carried,
    pub weight: f64,
    pub on: usize,
}

/// An age's end after a unit: its continuation (offset, PA, what it
/// carries, unconditional chance), the relay a stop leaves, and the chance
/// of its relay where it all but stopped.
#[derive(Debug, Clone, Copy)]
pub(super) struct End {
    pub continuation: Option<(usize, usize, Carried, f64)>,
    pub relay: Option<Relay>,
    pub both: f64,
}

/// A failed unit's lived part at one quadrature point, where its relay
/// takes it over (fn-199 R1): its unconditional chance, the share of a
/// whole unit's length it grew, its zones' sleeping buds, and its relay,
/// standing on it.
pub(super) struct Lived {
    pub weight: f64,
    pub size: f64,
    pub sleeping: Vec<Vec<Event>>,
    pub relay: Relay,
}

/// A unit as a part grows it: its index, the share of a whole unit's
/// length its phytomers grow at, its chance, and its zones' sleeping
/// buds; the unit itself or a failed unit's lived part.
#[derive(Clone, Copy)]
pub(super) struct Grown<'s> {
    pub i: usize,
    pub size: f64,
    pub grown: f64,
    pub sleeping: &'s [Vec<Event>],
    /// The share of each zone's nodes it lays (`Step::laid`).
    pub laid: f64,
}

impl Step {
    /// The unit as it grows whole.
    pub fn whole(&self) -> Grown<'_> {
        Grown {
            i: self.i,
            size: self.size,
            grown: self.grown,
            sleeping: &self.sleeping,
            laid: self.laid,
        }
    }

    /// Its lived part `l` (fn-199 R1).
    pub fn part<'s>(&self, l: &'s Lived) -> Grown<'s> {
        Grown {
            i: self.i,
            size: l.size,
            grown: l.weight,
            sleeping: &l.sleeping,
            laid: 1.0,
        }
    }
}

/// One growth unit of the bud's own axis.
pub(super) struct Step {
    pub i: usize,
    /// The chances the axis grows the unit and carries on past it (none
    /// where its age ends with it).
    pub grown: f64,
    pub after: f64,
    /// The share of a whole unit's length its phytomers grow at.
    pub size: f64,
    /// The share of each zone's nodes it lays, whole: below 1 where a
    /// lifespan's fraction cuts it (`grow::advance::cut`, fn-231).
    pub laid: f64,
    /// The share of its cycle gone where a lifespan's fraction ends its age
    /// inside it, 1 otherwise: the axis, ending with it, ends that share
    /// into the cycle, which the kept tables take in expectation over the
    /// cycle and the one before (fn-231).
    pub ends_at: f64,
    /// Where it starts along the axis and how long it is.
    pub start: Along,
    pub length: Along,
    /// Its failure: the chance it fails with no relay, which ends the
    /// axis at the unit before, and its lived parts where a relay takes
    /// it over (fn-199 R1).
    pub failed: f64,
    pub lived: Vec<Lived>,
    /// Each zone's sleeping buds that wake, as events, by zone index.
    pub sleeping: Vec<Vec<Event>>,
    pub aborted: Option<Relay>,
    pub end: Option<End>,
}

/// Each zone's node count, and where it starts along a unit one internode
/// a node long, after the zones before it (`ALONG`); and the whole unit.
pub(super) fn zones(counts: &[Count]) -> (Vec<Along>, Along) {
    let mut starts = vec![Along::default(); counts.len()];
    let mut whole = Along::default();
    for &z in ALONG.iter().filter(|&&z| z < counts.len()) {
        starts[z] = whole;
        whole = whole.then(counts[z].length(1.0));
    }
    (starts, whole)
}

/// A length along a unit one internode a node long, at `l` a node.
pub(super) fn scaled(a: Along, l: f64) -> Along {
    Along {
        mean: a.mean * l,
        square: a.square * l * l,
    }
}

/// The steps of a bud of PA `k` carrying `carried`, `m` cycles after it
/// was made. A relay carries on the time spent; a unit that ends its age
/// mid-cycle hands the rest of its cycle on (`schedule`).
pub(super) fn steps(
    species: &Species,
    living: &mut Living,
    bud: (usize, usize, Carried),
    first: Option<First>,
) -> Vec<Step> {
    steps_to(species, living, bud, first, usize::MAX)
}

/// A bud's first growth unit, if it grows any; the rest of its axis is
/// the same PA's bud one cycle younger carrying `rest` (fn-209, P1).
pub(super) fn first_step(
    species: &Species,
    living: &mut Living,
    bud: (usize, usize, Carried),
    first: Option<First>,
) -> Option<Step> {
    steps_to(species, living, bud, first, 1).pop()
}

/// What the rest of an axis carries from its second unit on: the time
/// its first unit spent, no part of a cycle gone, no relay's whole unit.
pub(super) fn rest(carried: Carried, lifespan: f64) -> Carried {
    let unit = carried.unit(lifespan, 1);
    let spent = unit.before + unit.share;
    Carried {
        spent,
        aged: carried.aged + spent - carried.spent,
        gone: 0.0,
        relay: false,
        rest: false,
    }
}

fn steps_to(
    species: &Species,
    living: &mut Living,
    (k, m, carried): (usize, usize, Carried),
    first: Option<First>,
    units: usize,
) -> Vec<Step> {
    let state = &species.states[k];
    let counts: Vec<Count> = state.zones.iter().map(|z| Count::of(z.nodes)).collect();
    let (starts, whole) = zones(&counts);
    // A relay carries on the time its axis has grown, which its abortion
    // hazard counts (host decision 11). The relay of an age that `ended`
    // grows a whole first unit.
    let relay = |(at, spent, gone, ended): (usize, f64, f64, bool), weight: f64, on: usize| {
        (weight > 0.0).then_some(Relay {
            at,
            carried: Carried {
                spent,
                aged: carried.aged + spent - carried.spent,
                gone,
                relay: ended,
                rest: false,
            },
            weight,
            on,
        })
    };
    let mut out = Vec::new();
    let mut path = Along::default();
    let mut survival = 1.0;
    for i in 1..=m.min(units) {
        let unit = carried.unit(state.life(), i);
        let spent = unit.before + unit.share;
        // A woken bud's partial first unit runs its share of the year's
        // risks (`First`); a unit that shares its cycle, its share.
        let partial = first.filter(|_| i == 1);
        let before = survival;
        let survive = partial.map_or(shared(state.viability, unit.share), |f| f.survive);
        // A unit that failed grows the share of its cycle its apex lived
        // where a relay takes it over, and the relay the rest (fn-199 R1);
        // otherwise the axis ends at the unit before.
        let fails = survival * (1.0 - survive);
        let failed = fails * (1.0 - state.relay_failed);
        let mut relayed = fails * state.relay_failed / LIVED as f64;
        let lived_at = lived(state.viability, unit.share);
        let mut survive = survive;
        // A sliver's failure, or the failure of a failed unit's relay's
        // first unit (relays do not nest within a cycle, host decision
        // A3), carries on where it relays.
        if unit.share < LIVED_FLOOR || (i == 1 && carried.rest) {
            survive += (1.0 - survive) * state.relay_failed;
            relayed = 0.0;
        }
        survival *= survive;
        // The chance it then aborts, its hazard counting the time grown.
        let grown = carried.aged + spent - carried.spent;
        let abortion = match partial {
            Some(f) if !unit.ends && f.survive > 0.0 => 1.0 - f.persist / f.survive,
            Some(_) => 0.0,
            None if !unit.ends => {
                // Over the share of the age's next unit.
                let exposure = carried.exposure(state.life(), i);
                1.0 - shared(1.0 - state.abortion_at(grown), exposure)
            }
            None => 0.0,
        };
        // A unit a lifespan's fraction cuts lays that share of each zone's
        // nodes, whole (fn-231); another partial unit counts its phytomers
        // whole, at its share of their length; a woken bud's first unit as
        // before, whole.
        let gone = if i == 1 { carried.gone } else { 0.0 };
        // What carries the axis on where its age ended inside the cycle:
        // its continuation, or the relay of an age that ended.
        let successor = i == 1 && !carried.rest;
        let laid = match partial {
            None => unit.laid(gone, (successor, false)),
            Some(_) => None,
        };
        let size = match (partial, laid) {
            (Some(_), _) => 1.0,
            (None, Some(c)) => c.of,
            (None, None) => unit.share,
        };
        let cut: Option<Vec<Count>> = laid.map(|c| {
            let laws = state.zones.iter().map(|z| Count::pmf(z.nodes));
            laws.map(|p| Count::of_pmf(&Count::laid_pmf(&p, c.laid())))
                .collect()
        });
        let (lays, lay_starts, lay_whole) = match &cut {
            Some(c) => {
                let (starts, whole) = zones(c);
                (c.as_slice(), starts, whole)
            }
            None => (counts.as_slice(), starts.clone(), whole),
        };
        let internode = state.internode * size;
        let mut sleeping = Vec::with_capacity(state.zones.len());
        for (z, zone) in state.zones.iter().enumerate() {
            let mean = if cut.is_some() {
                lays[z].mean()
            } else {
                zone.nodes.mean()
            };
            let nodes = survival * mean;
            let start = path.then(scaled(lay_starts[z], internode));
            let bearer = ((k, carried, m), i, abortion);
            let placed = (lays[z], start, internode);
            let mut woken = Vec::new();
            super::events::sleeping(
                living,
                (zone, z),
                (nodes, size),
                bearer,
                (m, placed),
                &mut woken,
            );
            sleeping.push(woken);
        }
        let mut lived_parts = Vec::new();
        for (j, &t) in lived_at.iter().enumerate() {
            if relayed <= 0.0 || (j > 0 && t == lived_at[j - 1]) {
                continue;
            }
            // Equal points share their parts' chances.
            let same = lived_at[j..].iter().filter(|&&u| u == t).count() as f64;
            let weight = relayed * same;
            let size = if partial.is_some() { t / unit.share } else { t };
            let internode = state.internode * size;
            let mut sleeping = Vec::with_capacity(state.zones.len());
            for (z, zone) in state.zones.iter().enumerate() {
                let start = path.then(scaled(starts[z], internode));
                let bearer = ((k, carried, m), i, 0.0);
                let placed = (counts[z], start, internode);
                let mut woken = Vec::new();
                let nodes = (weight * zone.nodes.mean(), size);
                super::events::sleeping(living, (zone, z), nodes, bearer, (m, placed), &mut woken);
                sleeping.push(woken);
            }
            let (onward, same_cycle) = carried.failed(unit, t);
            let at = if same_cycle { i - 1 } else { i };
            let relay = Relay {
                at,
                carried: onward,
                weight: weight * carried.lived_on(state, (i, unit), t),
                on: i,
            };
            lived_parts.push(Lived {
                weight,
                size,
                sleeping,
                relay,
            });
        }
        let length = scaled(lay_whole, internode);
        let mut step = Step {
            i,
            grown: survival,
            after: survival,
            size,
            laid: laid.map_or(1.0, |c| c.laid()),
            ends_at: match laid.is_some() && unit.ends {
                true => unit.used.min(1.0),
                false => 1.0,
            },
            start: path,
            length,
            failed,
            lived: lived_parts,
            sleeping,
            aborted: None,
            end: None,
        };
        path = path.then(length);
        if !unit.ends {
            match partial {
                None => {
                    let weight = survival * abortion * state.relay;
                    step.aborted = relay((i, spent, 0.0, false), weight, i);
                    survival *= 1.0 - abortion;
                }
                Some(f) => {
                    let stopped = before * (f.survive - f.persist);
                    step.aborted = relay((i, spent, 0.0, false), stopped * state.relay, i);
                    survival = before * f.persist;
                }
            }
            step.after = survival;
            out.push(step);
            continue;
        }
        // It moves on by its continuation, or stops and may relay, in this
        // cycle where the age ended within it.
        let (next, go) = species.successor(k).unwrap_or((k, 0.0));
        let (onward, at) = unit.onward(i);
        let continuation = (m > at && go > 0.0).then_some((at, next, onward, survival * go));
        let mut end = End {
            continuation,
            relay: None,
            both: 0.0,
        };
        if m > at {
            // And where it all but stopped, it grows its relay as well.
            let both = living.borderline(k, m.saturating_sub(i), unit.used);
            let ended = survival * ((1.0 - go) * state.relay_ended + both);
            end.relay = relay((at, spent, onward.gone, true), ended, i);
            end.both = both;
        }
        step.end = Some(end);
        out.push(step);
        break;
    }
    out
}

/// A survival over a share of a cycle: to the share, itself whole.
pub(super) fn shared(p: f64, share: f64) -> f64 {
    if share < 1.0 {
        p.powf_fixed(share)
    } else {
        p
    }
}
