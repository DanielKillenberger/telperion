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
mod feedback;
mod handover;
mod lay;
mod settle;

use feedback::{again, relaid};
pub use feedback::{Cpu, Feedback};
pub use lay::{Laid, LayJob, LayPa};

use super::balance::Totals;
use super::partition::{merge, ordered, Digest, Grouping};
use super::relay::RELAY_EVERY;
use super::sketch::{Inputs, Rows as Links};
use super::{start, Grower, Options, Request};
use crate::allocation::{size_sums, sized, split, subtotals};
use crate::bounded::Expected;
use crate::closed_form::boundary::Rows;
use crate::error::{Error, Result};
use crate::fixed::feedback::total::Total;
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
    /// Where asked (`grow_stepped_on`), every grower's whole state after
    /// each phase of each cycle, and each cycle's lattice and its light.
    pub states: Vec<[u64; 4]>,
    pub lattices: Vec<u64>,
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
    run(species, request, (expected, cut, grouping), None, false)
}

/// `grow_stepped` with the fine roots' layout, D16, deposits and sampling
/// on `device` (the stepped executor's own where none), and where
/// `complete`, every grower's whole state kept after each phase (fn-211
/// task 10).
pub fn grow_stepped_on(
    species: &Species,
    request: Request,
    (expected, cut, grouping): (&Expected, f64, Grouping),
    device: Option<&mut dyn Feedback>,
    complete: bool,
) -> Result<Stepped> {
    run(
        species,
        request,
        (expected, cut, grouping),
        device,
        complete,
    )
}

fn run(
    species: &Species,
    request: Request,
    (expected, cut, grouping): (&Expected, f64, Grouping),
    mut device: Option<&mut dyn Feedback>,
    complete: bool,
) -> Result<Stepped> {
    let (coarse, _) = start(species, request, true, Options::default())?;
    let mut x = Executor::new(coarse, (expected.rows(), cut, grouping), request, complete);
    let mut years = Vec::with_capacity(request.age as usize);
    for cycle in 1..=request.age {
        let started = std::time::Instant::now();
        x.cycle(cycle, again(&mut device))?;
        years.push(started.elapsed().as_secs_f64());
    }
    let started = std::time::Instant::now();
    let roots = x.items.len();
    let fine_grown = x.items.iter().map(|i| i.g.rules.grown).sum();
    let coarse_grown = x.coarse.rules.grown;
    let phases = std::mem::take(&mut x.phases);
    let (states, lattices) = (
        std::mem::take(&mut x.states),
        std::mem::take(&mut x.lattices),
    );
    let structure = x.settle()?;
    Ok(Stepped {
        structure,
        phases,
        roots,
        fine_grown,
        coarse_grown,
        years,
        settle: started.elapsed().as_secs_f64(),
        states,
        lattices,
    })
}

/// A fine root's grower and its placeholder's index in the coarse one.
pub(super) struct Item<'a> {
    pub at: usize,
    pub g: Grower<'a>,
    /// The next cycle it steps (`u32::MAX` once nothing in it lives or
    /// sleeps on), and the cycle it died, if it has (task 12).
    pub next: u32,
    pub died: Option<u32>,
}

/// What an item's run gave: its growth digest's part each cycle, and the
/// cycle it died in, if it did.
#[derive(Default)]
struct Ran {
    parts: Vec<Digest>,
    died: Option<u32>,
}

pub(super) struct Executor<'a> {
    pub coarse: Grower<'a>,
    pub items: Vec<Item<'a>>,
    pub rows: Arc<Rows>,
    pub cut: f64,
    pub grouping: Grouping,
    pub request: Request,
    pub phases: Vec<[u64; 4]>,
    pub complete: bool,
    pub states: Vec<[u64; 4]>,
    pub lattices: Vec<u64>,
    /// The dead items' part of the growth digest, which no cycle changes;
    /// the parts added at each cycle; and each cycle's from the items' runs.
    pub buried: Digest,
    pub burials: Vec<Digest>,
    pub grown_parts: Vec<Digest>,
}

impl<'a> Executor<'a> {
    /// An executor about to grow `coarse`'s first cycle, its roots at
    /// `cut` by `rows`, visited in `grouping`'s order.
    pub(super) fn new(
        coarse: Grower<'a>,
        (rows, cut, grouping): (Arc<Rows>, f64, Grouping),
        request: Request,
        complete: bool,
    ) -> Self {
        Self {
            coarse,
            items: Vec::new(),
            rows,
            cut,
            grouping,
            request,
            phases: Vec::new(),
            complete,
            states: Vec::new(),
            lattices: Vec::new(),
            buried: Digest::default(),
            burials: Vec::new(),
            grown_parts: Vec::new(),
        }
    }

    /// The items in the order they are visited and merged.
    pub(super) fn visit(&self) -> Vec<usize> {
        ordered((0..self.items.len()).collect(), self.grouping.order)
    }

    fn cycle(&mut self, cycle: u32, mut device: Option<&mut dyn Feedback>) -> Result<()> {
        let first_new = self.coarse.axes.len();
        let living = |g: &Grower| -> Vec<usize> { g.live.iter().map(|a| a.axis).collect() };
        let advanced = living(&self.coarse);
        // What each item grew from, where the device lays it; on the CPU
        // each item takes its own on the core it grows on.
        let advanced_items: Vec<Vec<usize>> = match device.is_some() {
            true => self.items.iter().map(|i| living(&i.g)).collect(),
            false => Vec::new(),
        };
        self.balance(cycle, again(&mut device));
        self.mark(cycle, 0);
        self.allot();
        self.mark(cycle, 1);
        // Phase (iv): each grower grows, wakes and lays its own (the
        // count the growth digest takes is the same once laid).
        let sketching = self.coarse.sketch.is_some() && cycle < self.request.age;
        // The items' layout on the device where one is given.
        let laying = device.is_some();
        let step = |g: &mut Grower, c: u32, advanced: &[usize], lays: bool| -> Result<()> {
            let live = std::mem::take(&mut g.live);
            g.grow_live(live, c, &mut |_| {})?;
            g.wake(c)?;
            if sketching && lays {
                g.sketch(c, advanced)?;
            }
            Ok(())
        };
        step(&mut self.coarse, cycle, &advanced, true)?;
        // Each item due runs ahead to its next synchronisation point.
        let reach = self.reach(cycle);
        let none = vec![(); self.items.len()];
        let ran = self.each(none, &|item, ()| -> Result<Ran> {
            let mut ran = Ran::default();
            if item.next != cycle {
                return Ok(ran);
            }
            for c in cycle..=reach {
                let advanced = living(&item.g);
                step(&mut item.g, c, &advanced, !laying)?;
                let mut part = item.g.digest(2);
                part.counts[0] -= 2;
                ran.parts.push(part);
                item.next = c + 1;
                if item.g.idle(c + 1) {
                    (item.next, item.died, ran.died) = (u32::MAX, Some(c), Some(c));
                    break;
                }
            }
            Ok(ran)
        });
        for r in ran {
            self.record(cycle, r?);
        }
        if let (true, Some(device)) = (sketching, again(&mut device)) {
            self.sketch_on(cycle, device, advanced_items)?;
        }
        let grown: u64 = self.growers().map(|g| g.rules.grown).sum();
        if grown > u64::from(self.request.budget) {
            return Err(Error::Budget {
                limit: self.request.budget,
            });
        }
        self.mark(cycle, 2);
        if sketching || self.coarse.sketch.is_none() {
            self.hand_over(first_new, cycle + 1);
        }
        if sketching {
            if cycle % RELAY_EVERY == 0 {
                self.relay(cycle)?;
            }
            self.shade(device);
            self.mark(cycle, 3);
        }
        // An item that died this cycle shades no more.
        for item in self.items.iter_mut().filter(|i| i.died == Some(cycle)) {
            item.g.leaves.clear();
        }
        Ok(())
    }

    /// The last cycle an item stepping in `cycle` runs to before anything
    /// crosses between it and the structure above (host decision on task
    /// 12, PHASES.md): its light, layout and re-lay at a cycle's end, and
    /// the balance or allotment reading its wood at a cycle's start.
    fn reach(&self, cycle: u32) -> u32 {
        let c = &self.coarse;
        let before = c.balances() || c.allots();
        let after = |k: u32| c.sketch.is_some() && k < self.request.age;
        let mut last = cycle;
        while last < self.request.age && !after(last) && !before {
            last += 1;
        }
        last
    }

    /// An item's run from `cycle` kept: its growth digest's part each
    /// cycle it ran, and, where it died, that part for every cycle after.
    fn record(&mut self, cycle: u32, ran: Ran) {
        let age = self.request.age as usize;
        if self.grown_parts.len() < age + 2 {
            self.grown_parts.resize(age + 2, Digest::default());
            self.burials.resize(age + 2, Digest::default());
        }
        for (k, part) in ran.parts.iter().enumerate() {
            let c = cycle as usize + k;
            self.grown_parts[c] = self.grown_parts[c].and(*part);
        }
        if let (Some(d), Some(last)) = (ran.died, ran.parts.last()) {
            self.burials[d as usize + 1] = self.burials[d as usize + 1].and(*last);
        }
    }

    /// The coarse grower, then each item.
    fn growers(&mut self) -> impl Iterator<Item = &mut Grower<'a>> {
        std::iter::once(&mut self.coarse).chain(self.items.iter_mut().map(|i| &mut i.g))
    }

    /// `work` on every item on every core, in the items' order: each
    /// item touches only itself.
    fn each<T: Send, R: Send>(
        &mut self,
        with: Vec<T>,
        work: &(dyn Fn(&mut Item<'a>, T) -> R + Sync),
    ) -> Vec<R> {
        let threads = self.coarse.threads;
        let items: Vec<_> = self.items.iter_mut().zip(with).collect();
        crate::cores::pinned(items, ITEMS, threads, &|_, (item, t)| work(item, t))
    }

    /// Phase (i): each item's totals at its root reported to its bearer,
    /// the coarse grower's decisions at each placeholder handed down.
    fn balance(&mut self, cycle: u32, device: Option<&mut dyn Feedback>) {
        if !self.coarse.balances() {
            return;
        }
        let none = vec![(); self.items.len()];
        let done: Vec<(Links, Totals)> = self.each(none, &|item, ()| {
            let rows = links(&item.g);
            let totals = item.g.totals(&rows, &[]);
            (rows, totals)
        });
        let given: Vec<_> = (self.visit().into_iter())
            .map(|k| {
                let t = &done[k].1;
                (self.items[k].at, t.light[1], t.cost[1], t.living[1])
            })
            .collect();
        let rows = self.coarse.rows().expect("laid");
        let totals = self.coarse.totals(&rows, &given);
        let (shed, kept) = self.coarse.shed_by(cycle, &rows, &totals, None);
        let pencils = &self.coarse.sketch.as_ref().expect("sketching").pencils;
        if let Some(device) = device {
            let roots = (self.items.iter().zip(done))
                .map(|(i, d)| (d.1, (shed[i.at], kept[i.at], pencils[i.at].memory)))
                .collect();
            return self.shed_on(cycle, device, roots);
        }
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
    /// sums merged, and every bud sized by them.
    fn allot(&mut self) {
        if !self.coarse.allots() {
            return;
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
        for &(at, lit) in &given {
            inputs.lit[at] = lit;
        }
        let (incoming, got) = split(&rows.links, &inputs.lit, &inputs.lambda, None);
        let pas = self.coarse.species.states.len();
        let sums = move |rows: &Links, i: &Inputs, got: &[f64]| {
            flat(&size_sums(got, &i.presence, &i.psi, &rows.pa, pas))
        };
        let mut total: Vec<Total> = sums(&rows, &inputs, &got);
        let based: Vec<_> = (self.items.iter().zip(done))
            .map(|(i, d)| (d, incoming[i.at]))
            .collect();
        let split_up = self.each(based, &|_, ((r, i), base)| {
            let got = split(&r.links, &i.lit, &i.lambda, Some(base)).1;
            let partial = sums(&r, &i, &got);
            ((r, i, got), partial)
        });
        let (done, partials): (Vec<_>, Vec<_>) = split_up.into_iter().unzip();
        let partial: HashMap<usize, Vec<Total>> = partials.into_iter().enumerate().collect();
        merge(&mut total, &partial, &self.grouping);
        let factors: Vec<[i128; 2]> = total
            .chunks(2)
            .map(|s| crate::allocation::factors(&[s[0], s[1]]))
            .collect();
        let size = |g: &mut Grower, (rows, i, got): (Links, Inputs, Vec<f64>)| {
            g.set_sizes(&sized(&got, &i.presence, &i.psi, &rows.pa, &factors));
        };
        size(&mut self.coarse, (rows, inputs, got));
        self.each(done, &|item, d| size(&mut item.g, d));
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
        let (mut tree, layout) = self.assemble();
        tree.relay(cycle, &mut |_| {})?;
        self.disperse(tree, layout);
        // The cycle's leaves where the re-lay put them, each grower's its
        // own (a placeholder bears none): the lattice is the same in any
        // grouping.
        relaid(&mut self.coarse, cycle);
        self.each(vec![(); self.items.len()], &|item, ()| {
            relaid(&mut item.g, cycle)
        });
        Ok(())
    }

    /// Phase `k`'s hash over every grower: the item's stub and root are
    /// counted once, as the coarse grower's placeholder.
    fn mark(&mut self, cycle: u32, k: usize) {
        let mut d = self.coarse.digest(k);
        let c = cycle as usize;
        if k == 2 {
            // Each item's growth is counted as it ran, ahead or not.
            if let Some(burial) = self.burials.get(c) {
                self.buried = self.buried.and(*burial);
            }
            let ran = self.grown_parts.get(c).copied().unwrap_or_default();
            d = d.and(self.buried).and(ran);
        } else {
            for item in self.items.iter().filter(|i| i.next != u32::MAX) {
                let mut part = item.g.digest(k);
                part.counts[0] -= 2;
                d = d.and(part);
            }
        }
        if self.phases.len() <= c {
            self.phases.resize(c + 1, [0; 4]);
        }
        self.phases[c][k] = d.finish(k);
        if self.complete {
            if self.states.len() <= c {
                self.states.resize(c + 1, [0; 4]);
            }
            self.states[c][k] = self.state();
        }
    }
}

/// The items in a piece of the work spread over the cores.
pub(super) const ITEMS: usize = 64;

/// An item's links and PAs, its root the base of its own split.
fn links(g: &Grower) -> Links {
    let mut rows = g.rows().expect("laid");
    rows.links[1].parent = None;
    rows
}

fn flat(sums: &[[Total; 2]]) -> Vec<Total> {
    sums.iter().flatten().copied().collect()
}

#[cfg(test)]
mod tests;
