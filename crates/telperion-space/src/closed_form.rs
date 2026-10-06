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
// The stand-in tables are read from fn-209's step 2 on.
#[cfg_attr(not(test), allow(dead_code))]
mod along;
pub(crate) mod boundary;
#[cfg_attr(not(test), allow(dead_code))]
mod envelope;
mod events;
#[cfg_attr(not(test), allow(dead_code))]
pub(crate) mod kept;
mod steps;

use crate::dormant::{First, Living};
use crate::error::{refuse, Result};
use crate::presence::Windows;
use crate::schedule::Carried;
use crate::species::Species;
use crate::structure::CountTable;
use events::{events, Event};
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
                    ..
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
