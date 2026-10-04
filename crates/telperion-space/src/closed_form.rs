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
use crate::dormant::{wakes_in, Living};
use crate::error::{refuse, Result};
use crate::species::Species;
use crate::structure::CountTable;
use std::collections::HashMap;

/// The expected phytomers per PA and cycle of a tree grown `age` cycles.
pub fn expected_counts(species: &Species, age: u32) -> Result<CountTable> {
    species.validate()?;
    if age == 0 {
        return refuse("age", "a tree grows at least one cycle");
    }
    let laterals: Vec<Vec<Vec<f64>>> = species.states.iter().map(|s| s.laterals()).collect();
    let mut memo = HashMap::new();
    let mut walk = Walk {
        species,
        laterals: &laterals,
        memo: &mut memo,
        living: Living::new(species),
    };
    Ok(walk.counts(0, age as usize, 0))
}

/// A bud's growth units in its PA before it transitions, having spent
/// `spent` already: every bud grows at least one.
pub(crate) fn units_left(lifespan: u32, spent: u32) -> usize {
    lifespan.saturating_sub(spent).max(1) as usize
}

/// A carried count of spent units past the last that changes anything.
pub(crate) fn spent_key(lifespan: u32, spent: u32) -> u32 {
    spent.min(lifespan.saturating_sub(1))
}

/// One step of a bud's development, as the recursion meets it.
enum Event {
    /// Phytomers of the bud's own PA grown in cycle `at`.
    Nodes { at: usize, count: f64 },
    /// A bud of `pa` born in cycle `at` with `spent` units, `weight` of them.
    Bud {
        at: usize,
        pa: usize,
        spent: u32,
        weight: f64,
    },
}

/// Every event of a bud of PA `k` with `spent` units, `m` cycles after it
/// was made: its growth units, laterals, relays and continuation, each
/// weighted by its expectation. A relay carries on the units spent.
fn events(
    species: &Species,
    laterals: &[Vec<f64>],
    living: &mut Living,
    (k, m, spent): (usize, usize, u32),
) -> Vec<Event> {
    let state = &species.states[k];
    let n = units_left(state.lifespan, spent);
    let mut out = Vec::new();
    let relay = |out: &mut Vec<Event>, at: usize, weight: f64, carried: usize| {
        if weight * state.relay > 0.0 {
            out.push(Event::Bud {
                at,
                pa: k,
                spent: spent + carried as u32,
                weight: weight * state.relay,
            });
        }
    };
    let mut survival = 1.0;
    for i in 1..=m.min(n) {
        // A unit that failed is spent.
        relay(&mut out, i, survival * (1.0 - state.viability), i);
        survival *= state.viability;
        for (zone, lateral) in state.zones.iter().zip(laterals) {
            let nodes = survival * zone.nodes.mean();
            out.push(Event::Nodes {
                at: i,
                count: nodes,
            });
            for (j, &p) in lateral.iter().enumerate() {
                let buds = nodes * f64::from(zone.buds) * p;
                if buds > 0.0 {
                    out.push(Event::Bud {
                        at: i,
                        pa: j,
                        spent: 0,
                        weight: buds,
                    });
                }
            }
            for (j, &p) in zone.dormant.iter().enumerate() {
                let buds = nodes * f64::from(zone.buds) * p;
                if buds <= 0.0 {
                    continue;
                }
                // Woken `s` years on, it grows from cycle i + 1 + s.
                for s in 0..(m - i) as u32 {
                    let wakes = wakes_in(zone, s);
                    if wakes > 0.0 {
                        let carried = living.after(k, spent, i, i + 1 + s as usize);
                        out.push(Event::Bud {
                            at: i + s as usize,
                            pa: j,
                            spent: 0,
                            weight: buds * wakes * carried,
                        });
                    }
                }
            }
        }
        if i < n {
            let abortion = state.abortion_at(i);
            relay(&mut out, i, survival * abortion, i);
            survival *= 1.0 - abortion;
        }
    }
    match state.next {
        Some(next) if m > n => out.push(Event::Bud {
            at: n,
            pa: next,
            spent: 0,
            weight: survival,
        }),
        None if m >= n => relay(&mut out, n, survival, n),
        _ => {}
    }
    out
}

struct Walk<'a, T> {
    species: &'a Species,
    laterals: &'a [Vec<Vec<f64>>],
    memo: &'a mut HashMap<(usize, usize, u32), T>,
    living: Living<'a>,
}

impl Walk<'_, CountTable> {
    fn counts(&mut self, k: usize, m: usize, spent: u32) -> CountTable {
        let spent = spent_key(self.species.states[k].lifespan, spent);
        if let Some(table) = self.memo.get(&(m, k, spent)) {
            return table.clone();
        }
        let mut table = CountTable::new(self.species.states.len(), m);
        for event in events(
            self.species,
            &self.laterals[k],
            &mut self.living,
            (k, m, spent),
        ) {
            match event {
                Event::Nodes { at, count } => table.add(k, at, count),
                Event::Bud {
                    at,
                    pa,
                    spent,
                    weight,
                } => {
                    let sub = self.counts(pa, m - at, spent);
                    add_shifted(&mut table, &sub, at, weight);
                }
            }
        }
        self.memo.insert((m, k, spent), table.clone());
        table
    }
}

impl Walk<'_, f64> {
    /// The log of the expected metres a bud grows: each phytomer its
    /// internode.
    fn log_length(&mut self, k: usize, m: usize, spent: u32) -> f64 {
        let spent = spent_key(self.species.states[k].lifespan, spent);
        if m == 0 {
            return f64::NEG_INFINITY;
        }
        if let Some(&log) = self.memo.get(&(m, k, spent)) {
            return log;
        }
        let internode = self.species.states[k].internode;
        let mut length = LogSum::default();
        for event in events(
            self.species,
            &self.laterals[k],
            &mut self.living,
            (k, m, spent),
        ) {
            match event {
                Event::Nodes { count, .. } => length.add(count * internode, 0.0),
                Event::Bud {
                    at,
                    pa,
                    spent,
                    weight,
                } => {
                    let sub = self.log_length(pa, m - at, spent);
                    length.add(weight, sub);
                }
            }
        }
        self.memo.insert((m, k, spent), length.0);
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
        living: Living::new(species),
    };
    (0..=age as usize)
        .map(|m| {
            (0..species.states.len())
                .map(|k| walk.log_length(k, m, 0))
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
mod tests {
    use super::{expected_counts, expected_log_lengths};
    use crate::species::{Form, NodeLaw, PaState, Species, Zone};

    /// The expected length is the expected counts weighted by internode,
    /// with every setting away from neutral.
    #[test]
    fn the_expected_length_weighs_the_expected_counts() {
        let state = |next, internode, lateral: [f64; 2]| PaState {
            lifespan: 3,
            next,
            viability: 0.9,
            zones: vec![
                Zone {
                    nodes: NodeLaw::Poisson { mean: 1.5 },
                    buds: 2,
                    dormant: vec![0.0; lateral.len()],
                    delay: 0.0,
                    rate: 0.0,
                    lateral: lateral.to_vec(),
                },
                Zone {
                    nodes: NodeLaw::Uniform { min: 1, max: 2 },
                    buds: 1,
                    dormant: vec![0.0; 2],
                    delay: 0.0,
                    rate: 0.0,
                    lateral: vec![0.0, 0.2],
                },
            ],
            shedding: None,
            internode,
            insertion: 0.5,
            divergence: 2.4,
            abortion: 0.2,
            abortion_rise: 0.0,
            relay: 0.4,
            relay_at: 1.0,
            epitony: 0.0,
            erection: 0.0,
            readiness: 0.8,
            rhythm: 0.6,
            straightening: 0.0,
            form: Form::default(),
        };
        let species = Species {
            states: vec![
                state(Some(1), 0.7, [0.1, 0.5]),
                state(None, 0.3, [0.0, 0.3]),
            ],
        };
        let counts = expected_counts(&species, 7).unwrap();
        let weighed = counts.total(0) * 0.7 + counts.total(1) * 0.3;
        let length = expected_log_lengths(&species, 7)[7][0].exp();
        assert!(
            (length - weighed).abs() < 1e-9 * weighed,
            "{length} vs {weighed}"
        );
    }
}
