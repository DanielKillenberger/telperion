//! A bud's own axis, growth unit by growth unit, as the closed form meets
//! it: the chance it reaches each unit and grows it, what stops it after,
//! and the relays and continuation that carry it on. The flat events the
//! counts and lengths read (`events.rs`) and the kept quantities, which
//! need the path a bud's axis took (`kept.rs`), both read these steps, so
//! the schedule and its chances are worked out in one place.
use super::along::{Along, Count};
use super::events::Event;
use crate::dormant::{First, Living};
use crate::schedule::Carried;
use crate::species::{Species, ALONG};

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

/// One growth unit of the bud's own axis.
pub(super) struct Step {
    pub i: usize,
    /// The chances the axis reaches the unit, grows it, and carries on
    /// past it (none where its age ends with it).
    pub before: f64,
    pub grown: f64,
    pub after: f64,
    /// The share of a whole unit's length its phytomers grow at.
    pub size: f64,
    /// Where it starts along the axis and how long it is.
    pub start: Along,
    pub length: Along,
    pub failed: Option<Relay>,
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
            },
            weight,
            on,
        })
    };
    let mut out = Vec::new();
    let mut path = Along::default();
    let mut survival = 1.0;
    for i in 1..=m.min(units) {
        let unit = carried.unit(state.lifespan, i);
        let spent = unit.before + unit.share;
        // A woken bud's partial first unit runs its share of the year's
        // risks (`First`); a unit that shares its cycle, its share.
        let partial = first.filter(|_| i == 1);
        let before = survival;
        let survive = partial.map_or(shared(state.viability, unit.share), |f| f.survive);
        // A unit that failed is spent; its relay stands on the unit before.
        let failed = relay(
            (i, spent, 0.0, true),
            survival * (1.0 - survive) * state.relay_failed,
            i - 1,
        );
        survival *= survive;
        // The chance it then aborts, its hazard counting the time grown.
        let grown = carried.aged + spent - carried.spent;
        let abortion = match partial {
            Some(f) if !unit.ends && f.survive > 0.0 => 1.0 - f.persist / f.survive,
            Some(_) => 0.0,
            None if !unit.ends => {
                // Over the share of the age's next unit.
                let exposure = carried.exposure(state.lifespan, i);
                1.0 - shared(1.0 - state.abortion_at(grown), exposure)
            }
            None => 0.0,
        };
        // A partial unit counts its phytomers whole, at its share of
        // their length; a woken bud's first unit as before, whole.
        let size = if partial.is_some() { 1.0 } else { unit.share };
        let internode = state.internode * size;
        let mut sleeping = Vec::with_capacity(state.zones.len());
        for (z, zone) in state.zones.iter().enumerate() {
            let nodes = survival * zone.nodes.mean();
            let start = path.then(scaled(starts[z], internode));
            let bearer = ((k, carried, m), i, abortion);
            let placed = (counts[z], start, internode);
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
        let length = scaled(whole, internode);
        let mut step = Step {
            i,
            before,
            grown: survival,
            after: survival,
            size,
            start: path,
            length,
            failed,
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
        p.powf(share)
    } else {
        p
    }
}
