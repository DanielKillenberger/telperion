//! Fine orders grown in the expansion stage (fn-209 F1; DESIGN-FINE.md,
//! accepted with Astra's changes, ASTRA-REVIEW-FINE.md), under realized
//! mechanics (task 6; fn-211's DESIGN-INTERLEAVE.md, "After Astra's
//! review", host decision 3).
//!
//! The CPU grows the structure above a cut by the engine's own grower; a
//! bud below the cut is made where it stands and handed on, its stand-in
//! keeping its place among its bearer's children. The cut is chosen
//! before growth, by the closed form's expected kept phytomers of the bud
//! (`closed_form::predictor`, light-neutral), so which buds are fine never
//! depends on what they grow into. The expansion grows every fine root
//! whole from its bud alone (`realized.rs`), and the two settle together
//! in four phases: each root reports its realized totals up (its
//! subtree's living, its base radius, its load), the structure above the
//! cut settles on them exactly as the whole tree does, and hands each
//! root its bearer's scale and laid frames down.
//!
//! The cut decides where the work runs, never what is computed (Astra's
//! change 2): every pass is the whole tree's own, and every sum a root's
//! subtree enters is taken in the whole tree's order, so the tree is
//! today's whole-tree engine's to the bit, at every cut, in every order
//! of its items and in any chunking of them (`tests/realized.rs`).
//! Where no bud reads light only; a species that reads light follows
//! fn-211's stepped executor.
mod canonical;
mod cases;
mod expansion;
mod geometry;
mod landing;
mod realized;
mod surface;
mod survey;
mod tables;
#[cfg(test)]
mod tests;

use crate::closed_form::predictor::Predictor;
use crate::error::{refuse, Error, Result};
use crate::grow::alone::rules;
use crate::grow::{keeps, Request};
use crate::species::Species;
use crate::structure::Structure;
pub use canonical::{canonical, digest, identities, lineage_order};
pub use cases::{landed, realized_cases, root_axes, Presences, RealizedCase, RootAxes};
pub use expansion::{settle, Lived, Loaded, RootSeed, Roots, Settled};
pub use geometry::{
    bearer_frame, BearerFrame, GeometryTables, PaGeometry, RootGeometry, ZoneGeometry,
};
use realized::{Cpu, Root};
use std::time::Instant;
pub use surface::{chain_cases, ChainAxis, ChainBud, ChainTables, PaRow, Woke};
pub use survey::{survey_lit, Surveyed};
pub use tables::{UnitZone, Units};

/// Where the work runs: the cut in expected kept phytomers (a bud at or
/// below it is a fine root), the order the items are taken in, the items
/// in each piece handed to a core, and the cores (0: every core).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Plan {
    pub cut: f64,
    pub order: Order,
    pub chunk: usize,
    pub threads: usize,
}

/// The order a plan takes its items in.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Order {
    Forward,
    Reverse,
    /// A permutation keyed by the seed given.
    Shuffled(u64),
}

impl Default for Plan {
    /// SPLIT.md's starting cut, N = 1,024.
    fn default() -> Self {
        Self {
            cut: 1024.0,
            order: Order::Forward,
            chunk: 16,
            threads: 0,
        }
    }
}

/// The work inside one item: chains grown, phytomers grown (the shed
/// ones included), the cycles its chains were grown through, the axes
/// made, and microseconds.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Work {
    pub chains: usize,
    pub grown: u64,
    pub cycles: u64,
    pub axes: usize,
    pub micros: f64,
}

/// A fine root as the expansion reports it: where its first axis stands
/// in the tree (none: shed), its expected and kept phytomers, whether it
/// woke, and its work.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Expanded {
    pub at: Option<usize>,
    pub predicted: f64,
    pub kept: usize,
    pub woken: bool,
    pub work: Work,
}

/// A tree grown above the cut and expanded below it: the tree (the
/// structure above the cut, each fine root's first axis where its
/// stand-in stood, then each root's other axes in the roots' order), its
/// axes above the cut, each root, where each root's other axes start,
/// the seconds each phase of the expansion took on the plan's cores, the
/// round trips between the structure above the cut and the items, and
/// the seconds the cut's predictor took.
pub struct Fine {
    pub structure: Structure,
    pub above: usize,
    pub roots: Vec<Expanded>,
    pub starts: Vec<usize>,
    pub phases: Vec<f64>,
    pub trips: usize,
    pub predict: f64,
}

impl Fine {
    /// The seconds the expansion took, its phases summed.
    pub fn expand_seconds(&self) -> f64 {
        self.phases.iter().sum()
    }
}

/// Grows `species` above `plan`'s cut on the CPU, expands every fine root
/// from its bud alone on the CPU reference executor, and settles the two
/// together: today's whole-tree engine's tree. Refuses a species any bud
/// of which reads light (fn-211), or whose girth reads the tree as grown.
pub fn grow_fine(species: &Species, request: Request, plan: Plan) -> Result<Fine> {
    if keeps(species) {
        return refuse(
            "retained",
            "a fine root's shed pipes and leaf girth are fn-211's stepped executor's",
        );
    }
    let rules = rules(species, request.age, request.budget)?;
    request.light.validate()?;
    let started = Instant::now();
    let predictor = Predictor::new(species, &rules.windows, request.age);
    let predict = started.elapsed().as_secs_f64();
    let threads = match plan.threads {
        0 => crate::cores::available(),
        n => n,
    };
    let mut cpu = Cpu {
        species,
        rules: &rules,
        plan,
        threads,
        roots: Vec::new(),
    };
    let (settled, _) =
        expansion::settle_with(species, (request, &predictor, &rules), plan, &mut cpu, None)?;
    let fine = assemble(settled, cpu.roots, &predictor)?;
    Ok(Fine { predict, ..fine })
}

/// The tree whole: each kept root's first axis where its stand-in stood,
/// its others after the structure above the cut, in the roots' order.
fn assemble(settled: Settled, roots: Vec<Root>, predictor: &Predictor) -> Result<Fine> {
    let Settled {
        structure: mut tree,
        at,
        live,
        bearers,
        phases,
        ..
    } = settled;
    let kept: usize = live.iter().map(|&k| roots[k].phytomers()).sum();
    if tree.phytomer_count() + kept == 0 {
        return Err(Error::Collapsed);
    }
    let above = tree.axes.len();
    let mut bearer = vec![usize::MAX; roots.len()];
    for (j, &k) in live.iter().enumerate() {
        bearer[k] = bearers[j];
    }
    let (mut out, mut starts) = (Vec::with_capacity(roots.len()), Vec::new());
    for (k, root) in roots.into_iter().enumerate() {
        let report = |kept| Expanded {
            at: at[k],
            predicted: predictor.count(root.pa, root.birth),
            kept,
            woken: root.woken,
            work: root.work,
        };
        let Some(i) = at[k] else {
            out.push(report(0));
            continue;
        };
        let (kept, offset) = (root.phytomers(), tree.axes.len());
        out.push(report(kept));
        starts.push(offset);
        let mut axes = root.into_axes(bearer[k], (i, offset)).into_iter();
        tree.axes[i] = axes.next().expect("a kept root's first axis");
        tree.axes.extend(axes);
    }
    starts.push(tree.axes.len());
    // Wood that stands exactly at its draw has no size.
    if tree
        .axes
        .iter()
        .flat_map(|a| &a.phytomers)
        .all(|p| p.scale == 0.0)
    {
        return Err(Error::Collapsed);
    }
    Ok(Fine {
        structure: tree,
        above,
        roots: out,
        starts,
        trips: phases.len(),
        phases,
        predict: 0.0,
    })
}

/// The seconds each phase of the expansion took.
#[derive(Default)]
pub(crate) struct Clock {
    pub phases: Vec<f64>,
}

impl Clock {
    pub fn time<T>(&mut self, work: impl FnOnce() -> T) -> T {
        let started = Instant::now();
        let out = work();
        self.phases.push(started.elapsed().as_secs_f64());
        out
    }
}

/// The roots at `live`, in that order.
pub(crate) fn pick<'r>(roots: &'r mut [Root], live: &[usize]) -> Vec<&'r mut Root> {
    let mut slots: Vec<Option<&mut Root>> = roots.iter_mut().map(Some).collect();
    live.iter()
        .map(|&k| slots[k].take().expect("each root once"))
        .collect()
}

/// `work(k, &mut items[k])` for every item, taken in the plan's order,
/// `chunk` at a time on `threads` cores: what each gave, by its place.
pub(crate) fn each<T: Send, R: Send>(
    items: &mut [T],
    plan: Plan,
    threads: usize,
    work: &(dyn Fn(usize, &mut T) -> R + Sync),
) -> Vec<R> {
    use std::sync::Mutex;
    let order = ordered(items.len(), plan.order);
    let chunk = plan.chunk.max(1);
    let pieces = order.len().div_ceil(chunk);
    let slots: Vec<Mutex<&mut T>> = items.iter_mut().map(Mutex::new).collect();
    let done: Vec<Mutex<Option<R>>> = (0..slots.len()).map(|_| Mutex::new(None)).collect();
    crate::cores::spread(pieces, threads, &|p| {
        for &k in &order[p * chunk..((p + 1) * chunk).min(order.len())] {
            let mut item = slots[k].lock().unwrap();
            *done[k].lock().unwrap() = Some(work(k, &mut item));
        }
    });
    done.into_iter()
        .map(|d| d.into_inner().unwrap().expect("every item ran"))
        .collect()
}

/// The items' indices in `order`.
fn ordered(n: usize, order: Order) -> Vec<usize> {
    let mut items: Vec<usize> = (0..n).collect();
    match order {
        Order::Forward => {}
        Order::Reverse => items.reverse(),
        Order::Shuffled(seed) => {
            let key = crate::lineage::Key::root(seed);
            items.sort_by_key(|&i| key.child(i as u64).0);
        }
    }
    items
}
