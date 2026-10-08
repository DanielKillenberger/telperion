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
//!    handed over; every grower's leaves are deposited into one lattice
//!    where its rough layout put them, and each reads its own buds.
//!
//! The tree is then assembled and settled as today's engine settles it.
//! An experiment behind its own entry point: `grow` never builds one.
mod ahead;
mod buds;
mod feedback;
mod handover;
mod lay;
mod outline;
mod phases;
mod schedule;
mod settle;

pub use ahead::{grow_outline, Outlined};
use feedback::again;
pub use feedback::{Cpu, Feedback};
pub use lay::{Laid, LayJob, LayPa};

use super::partition::Order;
use super::partition::{ordered, Digest, Grouping};
use super::{start, Grower, Options, Request};
use crate::bounded::Expected;
use crate::closed_form::predictor::Predictor;
use crate::error::{Error, Result};
use crate::fine::{Fine, Plan};
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
    let at = (Some(counts(expected)), cut, grouping);
    run(species, request, at, None, Keep::Digests)
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
    let keep = if complete {
        Keep::States
    } else {
        Keep::Digests
    };
    run(
        species,
        request,
        (Some(counts(expected)), cut, grouping),
        device,
        keep,
    )
}

/// The build's executor (fn-211 task 12, host decision (b)): every root
/// at the cut its own grower, the cut read from the predictor's counts
/// (fn-209 host decision 9), the items visited forward.
pub(crate) fn build(species: &Species, request: Request, plan: Plan) -> Result<Fine> {
    let order = match plan.order {
        crate::fine::Order::Forward => Order::Forward,
        crate::fine::Order::Reverse => Order::Reverse,
        crate::fine::Order::Shuffled(seed) => Order::Shuffled(seed),
    };
    let grouping = Grouping {
        order,
        chunk: plan.chunk,
        sizes: true,
        leaves: true,
    };
    let at = (None, plan.cut, grouping);
    Ok(run_with(species, request, at, (None, plan), Keep::Tree)?.1)
}

/// `build` at the default plan, with its counts.
pub(super) fn built(species: &Species, request: Request) -> Result<(Stepped, Fine)> {
    let plan = Plan::default();
    let grouping = Grouping {
        order: Order::Forward,
        chunk: plan.chunk,
        sizes: true,
        leaves: true,
    };
    run_with(species, request, (None, plan.cut, grouping), (None, plan), Keep::Tree)
}

/// What a run keeps beside its tree: nothing (the build), each phase's
/// digest, or with them every grower's whole state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Keep {
    Tree,
    Digests,
    States,
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
    (counts, cut, grouping): (Option<Counts>, f64, Grouping),
    device: Option<&mut dyn Feedback>,
    keep: Keep,
) -> Result<Stepped> {
    let at = (counts, cut, grouping);
    let (mut stepped, fine) = run_with(species, request, at, (device, Plan::default()), keep)?;
    stepped.structure = fine.structure;
    Ok(stepped)
}

/// A run, its tree also reported root by root (`Fine`), the roots that
/// go over as buds spread by `plan`.
fn run_with(
    species: &Species,
    request: Request,
    (counts, cut, grouping): (Option<Counts>, f64, Grouping),
    (mut device, plan): (Option<&mut dyn Feedback>, Plan),
    keep: Keep,
) -> Result<(Stepped, Fine)> {
    let (mut coarse, _) = start(species, request, true, Options::default())?;
    if plan.threads > 0 {
        coarse.threads = plan.threads;
    }
    // The cut reads the predictor's counts where none are given.
    let counts = counts.unwrap_or_else(|| {
        let predictor = Predictor::new(species, &coarse.rules.windows, request.age);
        // A PA no zone draws makes no bud: none is ever fine.
        Arc::new(move |pa, birth| predictor.count(pa, birth).unwrap_or(f64::INFINITY))
    });
    let mut x = Executor::new(coarse, (counts, cut, grouping), request, keep);
    x.plan = plan;
    x.hand_buds();
    let mut years = Vec::with_capacity(request.age as usize);
    for cycle in 1..=request.age {
        let started = crate::clock::seconds();
        x.cycle(cycle, again(&mut device))?;
        years.push(crate::clock::seconds() - started);
    }
    let started = crate::clock::seconds();
    x.grow_buds()?;
    if keep != Keep::Tree {
        x.grown();
    }
    let roots = x.items.len() + x.finished.len();
    let fine_grown = (x.items.iter().map(|i| i.g.rules.grown))
        .chain(x.finished.iter().map(|f| f.root.work.grown))
        .sum();
    let coarse_grown = x.coarse.rules.grown;
    let phases = std::mem::take(&mut x.phases);
    let (states, lattices) = (
        std::mem::take(&mut x.states),
        std::mem::take(&mut x.lattices),
    );
    let fine = x.settle()?;
    let stepped = Stepped {
        // The tree is the report's (`run` moves it here).
        structure: Structure {
            age: request.age,
            pas: species.states.len(),
            axes: Vec::new(),
        },
        phases,
        roots,
        fine_grown,
        coarse_grown,
        years,
        settle: crate::clock::seconds() - started,
        states,
        lattices,
    };
    Ok((stepped, fine))
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
    pub keep: Keep,
    pub states: Vec<[u64; 4]>,
    pub lattices: Vec<u64>,
    /// Each cycle's growth digest from the coarse grower, the dead items'
    /// part added at each cycle, and each cycle's from the items' runs.
    pub coarse_grown: Vec<Option<Digest>>,
    pub burials: Vec<Digest>,
    pub grown_parts: Vec<Digest>,
    /// The items due at each cycle, those that died in this one, and the
    /// phytomers the items have grown.
    pub due: Vec<Vec<usize>>,
    pub died: Vec<usize>,
    pub items_grown: u64,
    /// The roots handed over as buds, grown and finished at the end, and
    /// whether the structure above was marked for them.
    pub finished: Vec<buds::Finished>,
    pub marked: bool,
    /// How the roots are spread over the cores: their order, chunking
    /// and cores.
    pub plan: Plan,
}

impl<'a> Executor<'a> {
    /// An executor about to grow `coarse`'s first cycle, its roots at
    /// `cut` by `rows`, visited in `grouping`'s order.
    pub(super) fn new(
        coarse: Grower<'a>,
        (counts, cut, grouping): (Counts, f64, Grouping),
        request: Request,
        keep: Keep,
    ) -> Self {
        Self {
            coarse,
            items: Vec::new(),
            counts,
            cut,
            grouping,
            request,
            phases: Vec::new(),
            keep,
            states: Vec::new(),
            lattices: Vec::new(),
            coarse_grown: Vec::new(),
            burials: Vec::new(),
            grown_parts: Vec::new(),
            due: vec![Vec::new(); request.age as usize + 2],
            died: Vec::new(),
            items_grown: 0,
            finished: Vec::new(),
            marked: false,
            plan: Plan::default(),
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
        // Each item due runs to its next synchronisation point; until it
        // is reached, nothing reads it, so it runs there, with every other
        // item due by then, all at once.
        let (reach, keep) = (self.reach(cycle), self.keep);
        let mut ks = std::mem::take(&mut self.due[cycle as usize]);
        if reach > cycle {
            self.due[cycle as usize + 1].append(&mut ks);
        }
        // Only the items due are handed out, each piece to the same pool
        // thread while the items due stay the same (`cores::pinned`).
        ks.sort_unstable();
        let mut want = ks.iter().copied().peekable();
        let due: Vec<(usize, &mut Item)> = (self.items.iter_mut().enumerate())
            .filter(|(k, _)| want.next_if_eq(k).is_some())
            .collect();
        let threads = self.coarse.threads;
        let ran = crate::cores::pinned(due, ITEMS, threads, &|_, (k, item)| -> Result<Ran> {
            let before = item.g.rules.grown;
            let mut ran = Ran {
                item: k,
                from: item.next,
                ..Ran::default()
            };
            for c in item.next..=reach {
                let advanced = living(&item.g);
                step(&mut item.g, c, &advanced, !laying)?;
                if keep != Keep::Tree {
                    let mut part = item.g.digest(2);
                    part.counts[0] -= 2;
                    ran.parts.push(part);
                }
                item.next = c + 1;
                if item.g.idle(c + 1) {
                    (item.next, item.died, ran.died) = (u32::MAX, Some(c), Some(c));
                    break;
                }
            }
            (ran.next, ran.grown) = (item.next, item.g.rules.grown - before);
            Ok(ran)
        });
        for r in ran {
            let r = r?;
            self.items_grown += r.grown;
            match r.died {
                Some(_) => self.died.push(r.item),
                None => self.due[r.next as usize].push(r.item),
            }
            self.record(r);
        }
        if let (true, Some(device)) = (sketching, again(&mut device)) {
            self.sketch_on(cycle, device, advanced_items)?;
        }
        let grown = self.coarse.rules.grown + self.items_grown;
        if grown > u64::from(self.request.budget) {
            return Err(Error::Budget {
                limit: self.request.budget,
            });
        }
        self.mark(cycle, 2);
        let buds = self.coarse.rules.hands.is_some();
        if (sketching || self.coarse.sketch.is_none()) && !buds {
            self.hand_over(first_new, cycle + 1);
        }
        if sketching {
            self.shade(device);
            self.mark(cycle, 3);
        }
        // An item that died this cycle shades no more.
        for k in std::mem::take(&mut self.died) {
            self.items[k].g.leaves.clear();
        }
        Ok(())
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
}

/// The items in a piece of the work spread over the cores.
pub(super) const ITEMS: usize = 64;

#[cfg(test)]
mod tests;
