//! GreenLab's structural factorisation (Yan et al. 2004; Cournède et al.
//! 2006; de Reffye et al. 2021): the phytomers a bud of PA k grows in m
//! cycles are its own growth units plus, shifted by the cycle each was born
//! in, the substructures of its lateral buds and of its continuation. Each
//! substructure depends only on its PA and age, so it is computed once. Under
//! the stochastic laws the same recursion gives the expected counts (the
//! potential structure's existence rates), the counts themselves when every
//! law is deterministic. Shedding is outside the formula.
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
    let lifespan = state.lifespan as usize;
    let mut table = CountTable::new(species.states.len(), m);
    let mut survival = 1.0;
    for i in 1..=m.min(lifespan) {
        survival *= state.viability;
        for zone in &state.zones {
            let nodes = survival * zone.nodes.mean();
            table.add(k, i, nodes);
            for (j, &p) in zone.lateral.iter().enumerate() {
                let buds = nodes * f64::from(zone.buds) * p;
                if buds > 0.0 {
                    add_shifted(&mut table, &done[m - i][j], i, buds);
                }
            }
        }
    }
    if let (true, Some(next)) = (m > lifespan, state.next) {
        add_shifted(&mut table, &done[m - lifespan][next], lifespan, survival);
    }
    table
}

/// Adds `weight` copies of `sub`, born in cycle `born`, to `table`.
fn add_shifted(table: &mut CountTable, sub: &CountTable, born: usize, weight: f64) {
    for pa in 0..sub.pas {
        for cycle in 1..=sub.cycles {
            table.add(pa, cycle + born, weight * sub.get(pa, cycle));
        }
    }
}
