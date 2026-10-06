//! GreenLab's structural factorisation (Yan et al. 2004; Cournède et al.
//! 2006; de Reffye et al. 2021): the phytomers a bud of PA k grows in m
//! cycles are its own growth units plus, shifted by the cycle each was born
//! in, the substructures of its lateral buds and of its continuation. Each
//! substructure depends only on its PA and age, so it is computed once. Under
//! the stochastic laws the same recursion gives the expected counts (the
//! potential structure's existence rates), the counts themselves when every
//! law is deterministic. Abortion after a growth unit is a second survival
//! factor; a stopped apex's relay bud is one more substructure of its PA,
//! one that carries on the growth units its axis spent. A sleeping bud is
//! a lateral that grows from the cycle it wakes in, weighted by the chance
//! it wakes then and that its bearer is still carried on (`Living`).
//! Shedding is outside the formula.
use crate::dormant::{First, Living, Piece};
use crate::error::{refuse, Result};
use crate::presence::Windows;
use crate::schedule::Carried;
use crate::species::{Species, Zone};
use crate::structure::CountTable;
use std::collections::HashMap;

/// The expected phytomers per PA and cycle of a tree grown `age` cycles.
pub fn expected_counts(species: &Species, age: u32) -> Result<CountTable> {
    species.validate()?;
    if age == 0 {
        return refuse("age", "a tree grows at least one cycle");
    }
    let laterals: Vec<Vec<Vec<f64>>> = species.states.iter().map(|s| s.laterals()).collect();
    // The tree's windows: an apex that all but stopped moving on grows its
    // continuation and its relay, which count whole (host decision 13).
    let windows = Windows::new(species, age);
    let mut memo = HashMap::new();
    let mut walk = Walk {
        species,
        laterals: &laterals,
        memo: &mut memo,
        living: Living::new(species, Some(&windows)),
    };
    Ok(walk.counts(0, age as usize, Carried::default(), None))
}

/// One step of a bud's development, as the recursion meets it.
enum Event {
    /// Phytomers of the bud's own PA grown in cycle `at`, each grown at
    /// `size` of a whole unit's length (`schedule.rs`).
    Nodes { at: usize, count: f64, size: f64 },
    /// A bud of `pa` whose first unit grows in cycle `at + 1`, carrying
    /// `carried`, `weight` of them, born at `size` (a lateral of a partial
    /// unit's node); a woken one with the law of its partial first unit.
    Bud {
        at: usize,
        pa: usize,
        carried: Carried,
        weight: f64,
        size: f64,
        first: Option<First>,
    },
}

/// Every event of a bud of PA `k` carrying `carried`, `m` cycles after it
/// was made: its growth units, laterals, relays and continuation, each
/// weighted by its expectation. A relay carries on the time spent; a unit
/// that ends its age mid-cycle hands the rest of its cycle on (`schedule`).
fn events(
    species: &Species,
    laterals: &[Vec<f64>],
    living: &mut Living,
    (k, m, carried): (usize, usize, Carried),
    first: Option<First>,
) -> Vec<Event> {
    let state = &species.states[k];
    let mut out = Vec::new();
    // A relay carries on the time its axis has grown, which its abortion
    // hazard counts (host decision 11).
    // The relay of an age that `ended` grows a whole first unit.
    let relay =
        |out: &mut Vec<Event>, (at, spent, gone, ended): (usize, f64, f64, bool), weight: f64| {
            if weight > 0.0 {
                out.push(Event::Bud {
                    at,
                    pa: k,
                    carried: Carried {
                        spent,
                        aged: carried.aged + spent - carried.spent,
                        gone,
                        relay: ended,
                    },
                    weight,
                    size: 1.0,
                    first: None,
                });
            }
        };
    let mut survival = 1.0;
    for i in 1..=m {
        let unit = carried.unit(state.lifespan, i);
        let spent = unit.before + unit.share;
        // A woken bud's partial first unit runs its share of the year's
        // risks (`First`); a unit that shares its cycle, its share.
        let partial = first.filter(|_| i == 1);
        let before = survival;
        let survive = partial.map_or(shared(state.viability, unit.share), |f| f.survive);
        // A unit that failed is spent.
        relay(
            &mut out,
            (i, spent, 0.0, true),
            survival * (1.0 - survive) * state.relay_failed,
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
        for (z, (zone, lateral)) in state.zones.iter().zip(laterals).enumerate() {
            let nodes = survival * zone.nodes.mean();
            // A partial unit counts its phytomers whole, at its share of
            // their length; a woken bud's first unit as before, whole.
            let size = if partial.is_some() { 1.0 } else { unit.share };
            out.push(Event::Nodes {
                at: i,
                count: nodes,
                size,
            });
            // Every bud place grows, the last a fraction's at its share of
            // the size: counted whole, sized by the mean share.
            let (places, shared) = zone_places(zone);
            for (j, &p) in lateral.iter().enumerate() {
                let buds = nodes * places * p;
                if buds > 0.0 {
                    out.push(Event::Bud {
                        at: i,
                        pa: j,
                        carried: Carried::default(),
                        weight: buds,
                        size: size * shared,
                        first: None,
                    });
                }
            }
            let bearer = ((k, carried, m), i, abortion);
            sleeping(living, (zone, z), (nodes, size), bearer, m, &mut out);
        }
        if !unit.ends {
            match partial {
                None => {
                    relay(
                        &mut out,
                        (i, spent, 0.0, false),
                        survival * abortion * state.relay,
                    );
                    survival *= 1.0 - abortion;
                }
                Some(f) => {
                    let stopped = before * (f.survive - f.persist);
                    relay(&mut out, (i, spent, 0.0, false), stopped * state.relay);
                    survival = before * f.persist;
                }
            }
            continue;
        }
        // It moves on by its continuation, or stops and may relay, in this
        // cycle where the age ended within it.
        let (next, go) = species.successor(k).unwrap_or((k, 0.0));
        let (onward, at) = unit.onward(i);
        if m > at && go > 0.0 {
            out.push(Event::Bud {
                at,
                pa: next,
                carried: onward,
                weight: survival * go,
                size: 1.0,
                first: None,
            });
        }
        if m > at {
            // And where it all but stopped, it grows its relay as well.
            let both = living.borderline(k, m.saturating_sub(i), unit.used);
            let ended = survival * ((1.0 - go) * state.relay_ended + both);
            relay(&mut out, (at, spent, onward.gone, true), ended);
        }
        break;
    }
    out
}

/// A survival over a share of a cycle: to the share, itself whole.
fn shared(p: f64, share: f64) -> f64 {
    if share < 1.0 {
        p.powf(share)
    } else {
        p
    }
}

/// The sleeping buds of one zone's `nodes` on a growth unit of the bearer
/// (its PA, units carried, unit, and the chance it aborts after the
/// unit): each wakes `s` years on and grows from cycle i + 1 + s in the
/// stage it has aged into, while its bearer is still carried on.
fn sleeping(
    living: &mut Living,
    (zone, z): (&Zone, usize),
    (nodes, size): (f64, f64),
    (bearer, i, abortion): ((usize, Carried, usize), usize, f64),
    m: usize,
    out: &mut Vec<Event>,
) {
    let (places, shared) = zone_places(zone);
    for (j, &p) in zone.dormant.iter().enumerate() {
        let buds = nodes * places * p;
        if buds <= 0.0 {
            continue;
        }
        for s in 0..(m - i) as u32 {
            let pieces = living.woken((bearer.0, z), j, s);
            if pieces.is_empty() {
                continue;
            }
            // Its bearer carried on through the cycle it wakes in.
            let carried = living.after(bearer, i, i + 2 + s as usize, abortion);
            // Each stage it may wake in over the year, by the part of the
            // year that wakes it there. It ages its slept years in the
            // stage, and its abortion hazard counts them; its first unit's
            // time is a whole cycle's.
            for &(
                Piece {
                    wakes,
                    pa,
                    slept,
                    reach,
                    ..
                },
                first,
            ) in pieces.iter()
            {
                let woken = Carried {
                    spent: slept,
                    aged: slept,
                    ..Carried::default()
                };
                out.push(Event::Bud {
                    at: i + s as usize,
                    pa,
                    carried: woken,
                    // It reaches its stage at a share of its size, as the
                    // grower grows it; it is counted whole.
                    weight: buds * wakes * carried,
                    size: size * shared * reach,
                    first: Some(first),
                });
            }
        }
    }
}

/// A zone's bud places per node, every one grown, and their mean share
/// of a whole place's size.
fn zone_places(zone: &Zone) -> (f64, f64) {
    let places = zone.places() as f64;
    if places > 0.0 {
        (places, zone.buds / places)
    } else {
        (0.0, 0.0)
    }
}

/// A substructure: its PA, cycles, what it carries, and the law of a woken
/// bud's first unit.
type Sub = (usize, usize, (u64, u64, u64, bool), Option<(u64, u64)>);

fn sub(species: &Species, k: usize, m: usize, carried: Carried, first: Option<First>) -> Sub {
    (
        m,
        k,
        carried.key(species.states[k].lifespan),
        first.map(|f| (f.survive.to_bits(), f.persist.to_bits())),
    )
}

struct Walk<'a, T> {
    species: &'a Species,
    laterals: &'a [Vec<Vec<f64>>],
    memo: &'a mut HashMap<Sub, T>,
    living: Living<'a>,
}

impl Walk<'_, CountTable> {
    fn counts(&mut self, k: usize, m: usize, carried: Carried, first: Option<First>) -> CountTable {
        // A bud of an age passed through grows in the next it lives in, at
        // the share of its size that reaches it, as the grower grows it:
        // counted whole (Codex round 5 on fn-206).
        let Some((k, _)) = self.species.lived(k) else {
            return CountTable::new(self.species.states.len(), m);
        };
        self.lived_counts(k, m, carried, first)
    }

    fn lived_counts(
        &mut self,
        k: usize,
        m: usize,
        carried: Carried,
        first: Option<First>,
    ) -> CountTable {
        let key = sub(self.species, k, m, carried, first);
        if let Some(table) = self.memo.get(&key) {
            return table.clone();
        }
        let mut table = CountTable::new(self.species.states.len(), m);
        for event in events(
            self.species,
            &self.laterals[k],
            &mut self.living,
            (k, m, carried),
            first,
        ) {
            match event {
                Event::Nodes { at, count, .. } => table.add(k, at, count),
                Event::Bud {
                    at,
                    pa,
                    carried,
                    weight,
                    first,
                    ..
                } => {
                    let sub = self.counts(pa, m - at, carried, first);
                    add_shifted(&mut table, &sub, at, weight);
                }
            }
        }
        self.memo.insert(key, table.clone());
        table
    }
}

impl Walk<'_, f64> {
    /// The log of the expected metres a bud grows: each phytomer its
    /// internode.
    fn log_length(&mut self, k: usize, m: usize, carried: Carried, first: Option<First>) -> f64 {
        let Some((k, reach)) = self.species.lived(k) else {
            return f64::NEG_INFINITY;
        };
        reach.ln() + self.lived_log_length(k, m, carried, first)
    }

    fn lived_log_length(
        &mut self,
        k: usize,
        m: usize,
        carried: Carried,
        first: Option<First>,
    ) -> f64 {
        if m == 0 {
            return f64::NEG_INFINITY;
        }
        let key = sub(self.species, k, m, carried, first);
        if let Some(&log) = self.memo.get(&key) {
            return log;
        }
        let internode = self.species.states[k].internode;
        let mut length = LogSum::default();
        for event in events(
            self.species,
            &self.laterals[k],
            &mut self.living,
            (k, m, carried),
            first,
        ) {
            match event {
                Event::Nodes { count, size, .. } => length.add(count * size * internode, 0.0),
                Event::Bud {
                    at,
                    pa,
                    carried,
                    weight,
                    size,
                    first,
                } => {
                    let sub = self.log_length(pa, m - at, carried, first);
                    length.add(weight * size, sub);
                }
            }
        }
        self.memo.insert(key, length.0);
        length.0
    }
}

/// The natural log of the expected length in metres of the wood a bud of
/// each PA grows in 0 to `age` cycles, `[m][pa]`: the same recursion over
/// each phytomer's internode. In logs, so a branching that multiplies
/// without bound over many cycles stays finite; no wood is negative
/// infinity. The species is valid.
pub(crate) fn expected_log_lengths(species: &Species, age: u32) -> Vec<Vec<f64>> {
    let laterals: Vec<Vec<Vec<f64>>> = species.states.iter().map(|s| s.laterals()).collect();
    let mut memo = HashMap::new();
    let mut walk = Walk {
        species,
        laterals: &laterals,
        memo: &mut memo,
        living: Living::new(species, None),
    };
    (0..=age as usize)
        .map(|m| {
            (0..species.states.len())
                .map(|k| walk.log_length(k, m, Carried::default(), None))
                .collect()
        })
        .collect()
}

/// A sum of positive terms kept as its natural log.
struct LogSum(f64);

impl Default for LogSum {
    fn default() -> Self {
        Self(f64::NEG_INFINITY)
    }
}

impl LogSum {
    /// Adds `weight` times the value whose log is `log`.
    fn add(&mut self, weight: f64, log: f64) {
        if weight <= 0.0 || log == f64::NEG_INFINITY {
            return;
        }
        let term = weight.ln() + log;
        let (high, low) = if term > self.0 {
            (term, self.0)
        } else {
            (self.0, term)
        };
        self.0 = high + (low - high).exp().ln_1p();
    }
}

/// Adds `weight` copies of `sub`, born in cycle `born`, to `table`.
fn add_shifted(table: &mut CountTable, sub: &CountTable, born: usize, weight: f64) {
    for pa in 0..sub.pas {
        for cycle in 1..=sub.cycles {
            table.add(pa, cycle + born, weight * sub.get(pa, cycle));
        }
    }
}

#[cfg(test)]
mod tests;
