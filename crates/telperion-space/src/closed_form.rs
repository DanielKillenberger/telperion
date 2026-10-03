//! GreenLab's structural factorisation (Yan et al. 2004; Cournède et al.
//! 2006; de Reffye et al. 2021): the phytomers a bud of PA k grows in m
//! cycles are its own growth units plus, shifted by the cycle each was born
//! in, the substructures of its lateral buds and of its continuation. Each
//! substructure depends only on its PA and age, so it is computed once. Under
//! the stochastic laws the same recursion gives the expected counts (the
//! potential structure's existence rates), the counts themselves when every
//! law is deterministic. Abortion after a growth unit is a second survival
//! factor; a stopped apex's relay bud is one more substructure of its PA.
//! Shedding is outside the formula.
use crate::error::{refuse, Result};
use crate::species::Species;
use crate::structure::CountTable;

/// The expected phytomers per PA and cycle of a tree grown `age` cycles.
pub fn expected_counts(species: &Species, age: u32) -> Result<CountTable> {
    species.validate()?;
    if age == 0 {
        return refuse("age", "a tree grows at least one cycle");
    }
    let pas = species.states.len();
    // substructures[m][k]: a bud of PA k, m cycles after it was made.
    let mut substructures: Vec<Vec<CountTable>> = Vec::with_capacity(age as usize + 1);
    substructures.push(vec![CountTable::new(pas, 0); pas]);
    for m in 1..=age as usize {
        let row = (0..pas)
            .map(|k| substructure(species, k, m, &substructures))
            .collect();
        substructures.push(row);
    }
    Ok(substructures.swap_remove(age as usize).swap_remove(0))
}

fn substructure(species: &Species, k: usize, m: usize, done: &[Vec<CountTable>]) -> CountTable {
    let state = &species.states[k];
    let laterals = state.laterals();
    let lifespan = state.lifespan as usize;
    let mut table = CountTable::new(species.states.len(), m);
    // A relay bud of PA k, made in cycle `born` with probability `weight`.
    let relay = |table: &mut CountTable, born: usize, weight: f64| {
        if weight * state.relay > 0.0 {
            add_shifted(table, &done[m - born][k], born, weight * state.relay);
        }
    };
    let mut survival = 1.0;
    for i in 1..=m.min(lifespan) {
        relay(&mut table, i, survival * (1.0 - state.viability));
        survival *= state.viability;
        for (zone, lateral) in state.zones.iter().zip(&laterals) {
            let nodes = survival * zone.nodes.mean();
            table.add(k, i, nodes);
            for (j, &p) in lateral.iter().enumerate() {
                let buds = nodes * f64::from(zone.buds) * p;
                if buds > 0.0 {
                    add_shifted(&mut table, &done[m - i][j], i, buds);
                }
            }
        }
        relay(&mut table, i, survival * state.abortion);
        survival *= 1.0 - state.abortion;
    }
    match state.next {
        Some(next) if m > lifespan => {
            add_shifted(&mut table, &done[m - lifespan][next], lifespan, survival)
        }
        None if m >= lifespan => relay(&mut table, lifespan, survival),
        _ => {}
    }
    table
}

/// The natural log of the expected length in metres of the wood a bud of
/// each PA grows in 0 to `age` cycles, `[m][pa]`: the same recursion over
/// each phytomer's internode, as one sum per PA and age. In logs, so a
/// branching that multiplies without bound over many cycles stays finite;
/// no wood is negative infinity. The species is valid.
pub(crate) fn expected_log_lengths(species: &Species, age: u32) -> Vec<Vec<f64>> {
    let pas = species.states.len();
    let laterals: Vec<Vec<Vec<f64>>> = species.states.iter().map(|s| s.laterals()).collect();
    // done[m][k]: a bud of PA k, m cycles after it was made.
    let mut done: Vec<Vec<f64>> = vec![vec![f64::NEG_INFINITY; pas]];
    for m in 1..=age as usize {
        let row = (0..pas)
            .map(|k| {
                let state = &species.states[k];
                let lifespan = state.lifespan as usize;
                let mut length = LogSum::default();
                let mut survival = 1.0;
                for i in 1..=m.min(lifespan) {
                    let relayed = survival * (1.0 - state.viability) * state.relay;
                    length.add(relayed, done[m - i][k]);
                    survival *= state.viability;
                    for (zone, lateral) in state.zones.iter().zip(&laterals[k]) {
                        let nodes = survival * zone.nodes.mean();
                        length.add(nodes * state.internode, 0.0);
                        for (j, &p) in lateral.iter().enumerate() {
                            length.add(nodes * f64::from(zone.buds) * p, done[m - i][j]);
                        }
                    }
                    length.add(survival * state.abortion * state.relay, done[m - i][k]);
                    survival *= 1.0 - state.abortion;
                }
                match state.next {
                    Some(next) if m > lifespan => length.add(survival, done[m - lifespan][next]),
                    None if m >= lifespan => {
                        length.add(survival * state.relay, done[m - lifespan][k])
                    }
                    _ => {}
                }
                length.0
            })
            .collect();
        done.push(row);
    }
    done
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
                    lateral: lateral.to_vec(),
                },
                Zone {
                    nodes: NodeLaw::Uniform { min: 1, max: 2 },
                    buds: 1,
                    lateral: vec![0.0, 0.2],
                },
            ],
            shedding: None,
            internode,
            insertion: 0.5,
            divergence: 2.4,
            abortion: 0.2,
            relay: 0.4,
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
