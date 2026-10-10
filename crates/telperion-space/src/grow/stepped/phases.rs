//! Phases (i) to (iii) across the cut (PHASES.md): each item's totals
//! and lit presence reported to its bearer, the coarse grower's decisions
//! and vigour handed down, every grower's per-PA sums merged.
use super::Executor;
use crate::allocation::{size_sums, sized, split, subtotals};
use crate::grow::balance::Totals;
use crate::grow::partition::merge;
use crate::grow::sketch::{Inputs, Rows as Links};
use crate::grow::Grower;
use std::collections::HashMap;

impl Executor<'_> {
    /// Phase (i): each item's totals at its root reported to its bearer,
    /// the coarse grower's decisions at each placeholder handed down.
    pub(super) fn balance(&mut self, cycle: u32) {
        if !self.coarse.balances() {
            return;
        }
        let none = vec![(); self.items.len()];
        let done: Vec<(Links, Totals)> = self.each(none, &|item, ()| {
            let rows = links(&item.g);
            let totals = item.g.totals(&rows, &[]);
            (rows, totals)
        });
        let mut given: Vec<_> = (self.visit().into_iter())
            .map(|k| {
                let t = &done[k].1;
                (self.items[k].at, t.light[1], t.cost[1], t.living[1])
            })
            .collect();
        given.extend(self.stand_totals());
        let rows = self.coarse.rows().expect("laid");
        let totals = self.coarse.totals(&rows, &given);
        let (shed, kept) = self.coarse.shed_by(cycle, &rows, &totals, None);
        self.stand_kept(cycle, &shed, &kept);
        let pencils = &self.coarse.sketch.as_ref().expect("sketching").pencils;
        let decided: Vec<_> = (self.items.iter().zip(done))
            .map(|(i, d)| (d, (shed[i.at], kept[i.at], pencils[i.at].memory)))
            .collect();
        self.each(decided, &|item, ((rows, t), (shed, kept, memory))| {
            let root = &mut item.g.sketch.as_mut().expect("sketching").pencils[1];
            root.memory = memory;
            item.g.shed_by(cycle, &rows, &t, Some((shed, kept)));
        });
    }

    /// Phases (ii) and (iii): each item's lit presence reported to its
    /// bearer and its incoming vigour handed down; every grower's per-PA
    /// sums merged, and every bud sized by them; each PA's merged sums.
    pub(super) fn allot(&mut self) -> Vec<[f32; 2]> {
        if !self.coarse.allots() {
            return Vec::new();
        }
        let none = vec![(); self.items.len()];
        let done: Vec<(Links, Inputs)> = self.each(none, &|item, ()| {
            let rows = links(&item.g);
            let inputs = item.g.inputs(&rows);
            (rows, inputs)
        });
        let given: Vec<_> = (self.visit().into_iter())
            .map(|k| {
                let (r, i) = &done[k];
                (self.items[k].at, subtotals(&r.links, &i.lit)[1])
            })
            .collect();
        let rows = self.coarse.rows().expect("laid");
        let mut inputs = self.coarse.inputs(&rows);
        for &(at, lit) in given.iter().chain(&self.stand_lit()) {
            inputs.lit[at] = lit;
        }
        let (incoming, got) = split(&rows.links, &inputs.lit, &inputs.lambda, None);
        let pas = self.coarse.species.states.len();
        let sums = move |rows: &Links, i: &Inputs, got: &[f64]| {
            flat(&size_sums(got, &i.presence, &i.psi, &rows.pa, pas))
        };
        let mut total: Vec<f32> = sums(&rows, &inputs, &got);
        self.stand_sums(&got, &mut total);
        let based: Vec<_> = (self.items.iter().zip(done))
            .map(|(i, d)| (d, incoming[i.at]))
            .collect();
        let split_up = self.each(based, &|_, ((r, i), base)| {
            let got = split(&r.links, &i.lit, &i.lambda, Some(base)).1;
            let partial = sums(&r, &i, &got);
            ((r, i, got), partial)
        });
        let (done, partials): (Vec<_>, Vec<_>) = split_up.into_iter().unzip();
        let partial: HashMap<usize, Vec<f32>> = partials.into_iter().enumerate().collect();
        merge(&mut total, &partial, &self.grouping);
        let factors: Vec<[f32; 2]> = total.chunks(2).map(|s| [s[0], s[1]]).collect();
        let size = |g: &mut Grower, (rows, i, got): (Links, Inputs, Vec<f64>)| {
            g.set_sizes(&sized(&got, &i.presence, &i.psi, &rows.pa, &factors));
        };
        size(&mut self.coarse, (rows, inputs, got));
        self.each(done, &|item, d| size(&mut item.g, d));
        factors
    }
}

/// An item's links and PAs, its root the base of its own split.
pub(super) fn links(g: &Grower) -> Links {
    let mut rows = g.rows().expect("laid");
    rows.links[1].parent = None;
    rows
}

fn flat(sums: &[[f32; 2]]) -> Vec<f32> {
    sums.iter().flatten().copied().collect()
}
