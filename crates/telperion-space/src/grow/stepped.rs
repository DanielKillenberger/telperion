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
mod phases;
mod schedule;
mod settle;

use feedback::{again, relaid};
pub use feedback::{Cpu, Feedback};
pub use lay::{Laid, LayJob, LayPa};

use super::partition::{ordered, Digest, Grouping};
use super::relay::RELAY_EVERY;
use super::{start, Grower, Options, Request};
use crate::bounded::Expected;
use crate::closed_form::predictor::Predictor;
use crate::error::{Error, Result};
use crate::species::Species;
use crate::structure::Structure;
use schedule::Ran;
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
    run(
        species,
        request,
        (counts(expected), cut, grouping),
        None,
        false,
    )
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
    let at = (counts(expected), cut, grouping);
    run(species, request, at, device, complete)
}

/// The build's executor (fn-211 task 12, host decision (b)): every root
/// at the cut its own grower, the cut read from the predictor's counts
/// (fn-209 host decision 9), the items visited forward.
pub(crate) fn build(species: &Species, request: Request) -> Result<Structure> {
    species.validate()?;
    let windows = crate::presence::Windows::new(species, request.age);
    let predictor = Predictor::new(species, &windows, request.age);
    let counts: Counts = Arc::new(move |pa, birth| predictor.count(pa, birth));
    let grouping = Grouping {
        order: crate::grow::partition::Order::Forward,
        chunk: ITEMS,
        sizes: true,
        leaves: true,
        girth: true,
    };
    let at = (counts, crate::fine::Plan::default().cut, grouping);
    Ok(run(species, request, at, None, false)?.structure)
}

/// The closed form's expected kept phytomers of a bud of a PA born in a
/// cycle: what the cut reads.
pub(super) type Counts = Arc<dyn Fn(usize, u32) -> f64 + Send + Sync>;

fn counts(expected: &Expected) -> Counts {
    let rows = expected.rows();
    Arc::new(move |pa, birth| rows.lateral(pa, birth).count)
}

fn run(
    species: &Species,
    request: Request,
    (counts, cut, grouping): (Counts, f64, Grouping),
    mut device: Option<&mut dyn Feedback>,
    complete: bool,
) -> Result<Stepped> {
    let (coarse, _) = start(species, request, true, Options::default())?;
    let mut x = Executor::new(coarse, (counts, cut, grouping), request, complete);
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

pub(super) struct Executor<'a> {
    pub coarse: Grower<'a>,
    pub items: Vec<Item<'a>>,
    pub counts: Counts,
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
        (counts, cut, grouping): (Counts, f64, Grouping),
        request: Request,
        complete: bool,
    ) -> Self {
        Self {
            coarse,
            items: Vec::new(),
            counts,
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
}

/// The items in a piece of the work spread over the cores.
pub(super) const ITEMS: usize = 64;

#[cfg(test)]
mod tests;
