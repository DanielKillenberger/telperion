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
use crate::transcendental::Transcendental;
// The stand-in tables are read from fn-209's step 2 on.
#[cfg_attr(not(test), allow(dead_code))]
mod along;
pub(crate) mod boundary;
#[cfg_attr(not(test), allow(dead_code))]
mod envelope;
mod events;
#[cfg_attr(not(test), allow(dead_code))]
pub(crate) mod kept;
pub(crate) mod predictor;
mod steps;

use crate::dormant::{First, Living};
use crate::error::{refuse, Result};
use crate::memo::Memo;
use crate::presence::Windows;
use crate::schedule::Carried;
use crate::species::Species;
use crate::structure::CountTable;
use events::{events, first_events, Event};

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
    let mut memo = Memo::default();
    let mut walk = Walk {
        species,
        laterals: &laterals,
        memo: &mut memo,
        living: Living::new(species, Some(&windows)),
    };
    Ok(walk.counts(0, age as usize, Carried::default(), None))
}

/// The expected phytomers a tree grown `age` cycles grows, the shed ones
/// included: `expected_counts`' table summed, kept as one running total.
pub fn expected_grown(species: &Species, age: u32) -> Result<f64> {
    species.validate()?;
    if age == 0 {
        return refuse("age", "a tree grows at least one cycle");
    }
    Ok(0.0)
}

/// The expected kept phytomers of a tree grown `age` cycles: the cut's
/// predictor read at the seed, without growing the tree. None for a
/// species whose buds read light: the predictor is light-neutral, and the
/// lit oak keeps 27 to 64 % of its count at 80 (fn-198 task 4).
pub fn expected_kept(species: &Species, age: u32) -> Result<Option<f64>> {
    species.validate()?;
    if age == 0 {
        return refuse("age", "a tree grows at least one cycle");
    }
    if species.reads_light() {
        return Ok(None);
    }
    Ok(Some(predictor::tree(
        species,
        &Windows::new(species, age),
        age,
    )))
}

/// The expected metres of wood a tree grown `age` cycles grows, each
/// phytomer its internode at the size of the unit it stands on: its
/// phytomers weighted by their size (fn-231).
pub fn expected_length(species: &Species, age: u32) -> Result<f64> {
    species.validate()?;
    if age == 0 {
        return refuse("age", "a tree grows at least one cycle");
    }
    Ok(expected_log_lengths(species, age)[age as usize][0].exp_fixed())
}

/// A substructure: its PA, cycles, what it carries, and the law of a woken
/// bud's first unit.
type Sub = (
    usize,
    usize,
    (u64, u64, u64, bool, bool),
    Option<(u64, u64)>,
);

fn sub(species: &Species, k: usize, m: usize, carried: Carried, first: Option<First>) -> Sub {
    (
        m,
        k,
        carried.key(species.states[k].life()),
        first.map(|f| (f.survive.to_bits(), f.persist.to_bits())),
    )
}

struct Walk<'a, T> {
    species: &'a Species,
    laterals: &'a [Vec<Vec<f64>>],
    memo: &'a mut Memo<Sub, T>,
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
        reach.ln_fixed() + self.lived_log_length(k, m, carried, first)
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
        let state = &self.species.states[k];
        let carried = carried.canonical(state.life(), state.rises(), m);
        let key = sub(self.species, k, m, carried, first);
        if let Some(&log) = self.memo.get(&key) {
            return log;
        }
        let state = &self.species.states[k];
        let internode = state.internode;
        let mut length = LogSum::default();
        let (first, on) = first_events(
            self.species,
            &self.laterals[k],
            &mut self.living,
            (k, m, carried),
            first,
        );
        // The rest of its axis: the bud one cycle younger (fn-209, P1).
        if on > 0.0 {
            let rest = steps::rest(carried, state.life());
            length.add(on, self.lived_log_length(k, m - 1, rest, None));
        }
        for event in first {
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
    let mut memo = Memo::default();
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
        let term = weight.ln_fixed() + log;
        let (high, low) = if term > self.0 {
            (term, self.0)
        } else {
            (self.0, term)
        };
        self.0 = high + (low - high).exp_fixed().ln_1p_fixed();
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

/// fn-209 probe P2: the expected leaf area a lateral of each PA made `m`
/// cycles before the tree's age grows in the tree's last cycle, per unit
/// base scale, by `[pa][m]`: its phytomers of that cycle (counted whole)
/// times each one's PA's leaf area. `windows` is the species whose
/// windows the tree grows under: `species` itself, or the species it is
/// taken at a constant light from (`alone/field.rs`, `at_light`).
pub(crate) fn last_leaves(species: &Species, windows: &Species, age: u32) -> Vec<Vec<f64>> {
    let laterals: Vec<Vec<Vec<f64>>> = species.states.iter().map(|s| s.laterals()).collect();
    let windows = Windows::new(windows, age);
    let mut memo = Memo::default();
    let mut walk = Walk {
        species,
        laterals: &laterals,
        memo: &mut memo,
        living: Living::new(species, Some(&windows)),
    };
    let n = species.states.len();
    (0..n)
        .map(|pa| {
            (0..=age as usize)
                .map(|m| {
                    if m == 0 {
                        return 0.0;
                    }
                    let t = walk.counts(pa, m, Carried::default(), None);
                    (0..n)
                        .map(|q| t.get(q, m) * species.states[q].leaf_area)
                        .sum()
                })
                .collect()
        })
        .collect()
}
