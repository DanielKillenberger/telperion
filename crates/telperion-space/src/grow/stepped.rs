//! The stepped executor (fn-211.4, host decision 2 on probe (a),
//! DESIGN-INTERLEAVE.md; its plan in PHASES.md): the engine's own grower
//! scoped to each fine root, stepped through every cycle's phases beside
//! a coarse grower that holds the structure above the cut, with only
//! per-root totals and the light lattice crossing between them.
//!
//! A fine root is a lateral whose bearer stands above the cut and whose
//! expected kept phytomers (the closed form's `count`) are at most the
//! cut. The coarse grower grows it in the cycle it is made, then hands it
//! and what it bore that cycle to an item of its own (`handover.rs`),
//! keeping a placeholder at its index so its bearer's children keep their
//! order. Item axis 0 is a stub for the bearer, axis 1 the root. Each
//! cycle:
//!
//! 1. **(i) balance**: each item sums its subtree's light, upkeep and
//!    living status; the coarse grower takes them at the placeholder,
//!    decides the root's shedding there and hands down its shed flag,
//!    kept share and memory, and the item decides inside the root.
//! 2. **(ii) demand**: each item sums its subtree's lit presence; the
//!    coarse grower splits its vigour with it at the placeholder and hands
//!    down the root's incoming vigour, which the item splits inside.
//! 3. **(iii) normalisers**: every grower's per-PA `Sum` partials merge
//!    (order-free, `fixed::sum`) and every grower sizes its buds by them.
//! 4. **(iv)** each grower grows, wakes and lays its own; new roots are
//!    handed over; every tenth cycle the tree is assembled for today's
//!    re-lay (`assemble.rs`); every grower's leaves are deposited into one
//!    lattice and each reads its own buds.
//!
//! The tree is then assembled and settled as today's engine settles it.
//! An experiment behind its own entry point: `grow` never builds one.
mod assemble;
mod handover;

use super::balance::Totals;
use super::partition::{merge, ordered, Grouping};
use super::relay::RELAY_EVERY;
use super::sketch::{Inputs, Rows as Links};
use super::{start, Grower, Options, Request, Stage};
use crate::allocation::{size_sums, sized, split, subtotals};
use crate::bounded::Expected;
use crate::closed_form::boundary::Rows;
use crate::error::{Error, Result};
use crate::fixed::sum::Sum;
use crate::light::Field;
use crate::species::Species;
use crate::structure::Structure;
use std::collections::HashMap;
use std::sync::Arc;

/// A tree grown by the stepped executor: its state hashed after each
/// phase of each cycle (`partition::Digest`), and its fine roots.
pub struct Stepped {
    pub structure: Structure,
    pub phases: Vec<[u64; 4]>,
    /// Fine roots handed over, and the phytomers grown by their growers
    /// and by the coarse one.
    pub roots: usize,
    pub fine_grown: u64,
    pub coarse_grown: u64,
    /// Seconds each cycle took, and the assembly and settle.
    pub years: Vec<f64>,
    pub settle: f64,
}

/// Grows `species` with every fine root at `cut` its own grower, the
/// items visited and their partials merged in `grouping`'s order and
/// chunks.
pub fn grow_stepped(
    species: &Species,
    request: Request,
    expected: &Expected,
    cut: f64,
    grouping: Grouping,
) -> Result<Stepped> {
    let (coarse, keeps) = start(species, request, true, Options::default())?;
    let mut x = Executor {
        coarse,
        items: Vec::new(),
        rows: expected.rows(),
        cut,
        grouping,
        request,
        phases: Vec::new(),
    };
    let mut years = Vec::with_capacity(request.age as usize);
    for cycle in 1..=request.age {
        let started = std::time::Instant::now();
        x.cycle(cycle)?;
        years.push(started.elapsed().as_secs_f64());
    }
    let started = std::time::Instant::now();
    let roots = x.items.len();
    let fine_grown = x.items.iter().map(|i| i.g.rules.grown).sum();
    let coarse_grown = x.coarse.rules.grown;
    let phases = std::mem::take(&mut x.phases);
    let tree = x.assemble(true).0;
    let quiet = &mut |_: Stage| {};
    let grown = tree.settle((request, Options::default()), (true, keeps), quiet)?;
    Ok(Stepped {
        structure: grown.structure,
        phases,
        roots,
        fine_grown,
        coarse_grown,
        years,
        settle: started.elapsed().as_secs_f64(),
    })
}

/// A fine root's grower and its placeholder's index in the coarse one.
pub(super) struct Item<'a> {
    pub at: usize,
    pub g: Grower<'a>,
}

pub(super) struct Executor<'a> {
    pub coarse: Grower<'a>,
    pub items: Vec<Item<'a>>,
    pub rows: Arc<Rows>,
    pub cut: f64,
    pub grouping: Grouping,
    pub request: Request,
    pub phases: Vec<[u64; 4]>,
}

impl<'a> Executor<'a> {
    /// The items in the order they are visited and merged.
    pub(super) fn visit(&self) -> Vec<usize> {
        ordered((0..self.items.len()).collect(), self.grouping.order)
    }

    fn cycle(&mut self, cycle: u32) -> Result<()> {
        let first_new = self.coarse.axes.len();
        let living = |g: &Grower| g.live.iter().map(|a| a.axis).collect::<Vec<_>>();
        let advanced = living(&self.coarse);
        let advanced_items: Vec<Vec<usize>> = self.items.iter().map(|i| living(&i.g)).collect();
        self.balance(cycle);
        self.mark(cycle, 0);
        self.allot();
        self.mark(cycle, 1);
        let quiet = &mut |_: Stage| {};
        let mut grown = 0;
        for g in self.growers() {
            let live = std::mem::take(&mut g.live);
            g.grow_live(live, cycle, quiet)?;
            g.wake(cycle)?;
            grown += g.rules.grown;
        }
        if grown > u64::from(self.request.budget) {
            return Err(Error::Budget {
                limit: self.request.budget,
            });
        }
        self.mark(cycle, 2);
        let sketching = self.coarse.sketch.is_some() && cycle < self.request.age;
        if sketching {
            self.coarse.sketch(cycle, &advanced)?;
            for (item, advanced) in self.items.iter_mut().zip(&advanced_items) {
                item.g.sketch(cycle, advanced)?;
            }
        }
        if sketching || self.coarse.sketch.is_none() {
            self.hand_over(first_new);
        }
        if sketching {
            if cycle % RELAY_EVERY == 0 {
                self.relay(cycle)?;
            }
            self.shade();
            self.mark(cycle, 3);
        }
        Ok(())
    }

    /// The coarse grower, then each item.
    fn growers(&mut self) -> impl Iterator<Item = &mut Grower<'a>> {
        std::iter::once(&mut self.coarse).chain(self.items.iter_mut().map(|i| &mut i.g))
    }

    /// Phase (i): each item's totals at its root reported to its bearer,
    /// the coarse grower's decisions at each placeholder handed down.
    fn balance(&mut self, cycle: u32) {
        if !self.coarse.balances() {
            return;
        }
        let mut done: HashMap<usize, (Links, Totals)> = HashMap::new();
        let mut given = Vec::with_capacity(self.items.len());
        for k in self.visit() {
            let item = &self.items[k];
            let rows = links(&item.g);
            let t = item.g.totals(&rows, &[]);
            given.push((item.at, t.light[1], t.cost[1], t.living[1]));
            done.insert(k, (rows, t));
        }
        let rows = self.coarse.rows().expect("laid");
        let totals = self.coarse.totals(&rows, &given);
        let (shed, kept) = self.coarse.shed_by(cycle, &rows, &totals, None);
        let pencils = &self.coarse.sketch.as_ref().expect("sketching").pencils;
        for k in self.visit() {
            let item = &mut self.items[k];
            let memory = pencils[item.at].memory;
            let root = &mut item.g.sketch.as_mut().expect("sketching").pencils[1];
            root.memory = memory;
            let (rows, t) = &done[&k];
            item.g
                .shed_by(cycle, rows, t, Some((shed[item.at], kept[item.at])));
        }
    }

    /// Phases (ii) and (iii): each item's lit presence reported to its
    /// bearer and its incoming vigour handed down; every grower's per-PA
    /// sums merged, and every bud sized by them.
    fn allot(&mut self) {
        if !self.coarse.allots() {
            return;
        }
        let mut done: HashMap<usize, (Links, Inputs)> = HashMap::new();
        let mut given = Vec::with_capacity(self.items.len());
        for k in self.visit() {
            let item = &self.items[k];
            let rows = links(&item.g);
            let inputs = item.g.inputs(&rows);
            given.push((item.at, subtotals(&rows.links, &inputs.lit)[1]));
            done.insert(k, (rows, inputs));
        }
        let rows = self.coarse.rows().expect("laid");
        let mut inputs = self.coarse.inputs(&rows);
        for &(at, lit) in &given {
            inputs.lit[at] = lit;
        }
        let (incoming, got) = split(&rows.links, &inputs.lit, &inputs.lambda, None);
        let pas = self.coarse.species.states.len();
        let sums = |rows: &Links, i: &Inputs, got: &[f64]| {
            size_sums(got, &i.presence, &i.psi, &rows.pa, pas)
        };
        let mut total: Vec<Sum> = flat(&sums(&rows, &inputs, &got));
        let mut partial = HashMap::new();
        let mut gots = HashMap::new();
        for k in self.visit() {
            let (r, i) = &done[&k];
            let base = incoming[self.items[k].at];
            let got = split(&r.links, &i.lit, &i.lambda, Some(base)).1;
            partial.insert(k, flat(&sums(r, i, &got)));
            gots.insert(k, got);
        }
        merge(&mut total, &partial, &self.grouping);
        let factors: Vec<[f64; 2]> = total
            .chunks(2)
            .map(|s| [s[0].value(), s[1].value()])
            .collect();
        let size = |g: &mut Grower, rows: &Links, i: &Inputs, got: &[f64]| {
            let s = sized(got, &i.presence, &i.psi, &rows.pa, &factors);
            g.set_sizes(&s);
        };
        size(&mut self.coarse, &rows, &inputs, &got);
        for (k, item) in self.items.iter_mut().enumerate() {
            let (r, i) = &done[&k];
            size(&mut item.g, r, i, &gots[&k]);
        }
    }

    /// Phase (iv)'s light: every grower's leaves of the cycle into one
    /// lattice, swept once, each grower reading its own buds.
    fn shade(&mut self) {
        let order = self.visit();
        let threads = self.coarse.threads;
        let light = self.coarse.sketch.as_ref().expect("sketching").light;
        let growers: Vec<usize> = std::iter::once(usize::MAX).chain(order).collect();
        let (mut leaves, mut tips) = (Vec::new(), Vec::new());
        for &k in &growers {
            let g = self.grower(k);
            leaves.extend_from_slice(&g.leaves);
            tips.extend(g.live.iter().map(|a| g.tip(a.axis)));
        }
        let field = Field::on(&light, &leaves, &tips, threads);
        let mut read = vec![0.0; tips.len()];
        crate::cores::fill(&mut read, 1 << 10, threads, &|k| field.at(tips[k]));
        let mut from = 0;
        for &k in &growers {
            let g = self.grower_mut(k);
            let n = g.live.len();
            let sketch = g.sketch.as_mut().expect("sketching");
            for (apex, &light) in g.live.iter().zip(&read[from..from + n]) {
                sketch.pencils[apex.axis].light = light;
            }
            from += n;
        }
    }

    fn grower(&self, k: usize) -> &Grower<'a> {
        match k {
            usize::MAX => &self.coarse,
            k => &self.items[k].g,
        }
    }

    fn grower_mut(&mut self, k: usize) -> &mut Grower<'a> {
        match k {
            usize::MAX => &mut self.coarse,
            k => &mut self.items[k].g,
        }
    }

    /// Every tenth cycle, today's re-lay on the tree assembled, its
    /// places and walks handed back, its leaves the cycle's.
    fn relay(&mut self, cycle: u32) -> Result<()> {
        let (mut tree, layout) = self.assemble(false);
        tree.relay(cycle, &mut |_| {})?;
        let leaves = std::mem::take(&mut tree.leaves);
        self.disperse(tree, layout);
        self.coarse.leaves = leaves;
        for item in &mut self.items {
            item.g.leaves.clear();
        }
        Ok(())
    }

    /// Phase `k`'s hash over every grower: the item's stub and root are
    /// counted once, as the coarse grower's placeholder.
    fn mark(&mut self, cycle: u32, k: usize) {
        let mut d = self.coarse.digest(k);
        for item in &self.items {
            let mut part = item.g.digest(k);
            part.counts[0] -= 2;
            d = d.and(part);
        }
        let c = cycle as usize;
        if self.phases.len() <= c {
            self.phases.resize(c + 1, [0; 4]);
        }
        self.phases[c][k] = d.finish(k);
    }
}

/// An item's links and PAs, its root the base of its own split.
fn links(g: &Grower) -> Links {
    let mut rows = g.rows().expect("laid");
    rows.links[1].parent = None;
    rows
}

fn flat(sums: &[[Sum; 2]]) -> Vec<Sum> {
    sums.iter().flatten().copied().collect()
}

#[cfg(test)]
mod tests;
