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
mod coarse;
mod exchange;
mod expansion;
mod geometry;
mod landing;
mod lay;
mod likely;
mod realized;
mod spare;
mod step;
mod surface;
mod survey;
mod tables;
#[cfg(test)]
mod tests;

use crate::error::{Error, Result};
use crate::grow::Request;
use crate::species::Species;
use crate::structure::Structure;
pub use canonical::{canonical, digest, identities, lineage_order, lineage_order_marked};
pub(crate) use canonical::lineage_permutation;
pub use cases::{landed, realized_cases, root_axes, Presences, RealizedCase, RootAxes};
pub use coarse::{grow_coarse, Coarse, Stand};
pub use expansion::{settle, Lived, Loaded, RootSeed, Roots, Settled};
pub(crate) use expansion::{settle_grown, Above, Stages};
pub use geometry::{
    bearer_frame, BearerFrame, GeometryTables, PaGeometry, RootGeometry, ZoneGeometry,
};
pub(crate) use realized::{Cpu, Made, Root};
pub(crate) use step::now;
pub use step::{ready, Step};
pub use surface::{chain_cases, ChainAxis, ChainBud, ChainTables, PaRow, Woke};
pub use survey::{survey_laid, survey_lit, Surveyed};
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
    /// Each live root's first axis's lineage and its base radius as phase
    /// 2 handed it up (`Settled::radii`), by lineage.
    pub bases: Vec<(u64, f64)>,
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

/// The build (`grow`, the stepped executor) at `plan`'s cut, order,
/// chunking and cores, reported root by root: the CPU reference the GPU
/// executor's tests hold its roots to (fn-211 task 12). The tree is in
/// the order the build assembles it, not canonical.
pub fn grow_roots(species: &Species, request: Request, plan: Plan) -> Result<Fine> {
    crate::grow::stepped::build(species, request, plan)
}

/// The tree whole: each kept root's first axis where its stand-in stood,
/// its others after the structure above the cut, in the roots' order.
pub(crate) fn assemble(
    settled: Settled,
    roots: Vec<Root>,
    predicted: &dyn Fn(usize, u32) -> f64,
) -> Result<Fine> {
    let bases = bases(&settled);
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
            predicted: predicted(root.pa, root.birth),
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
        bases,
        above,
        roots: out,
        starts,
        trips: phases.len(),
        phases,
        predict: 0.0,
    })
}

/// A settled tree as the CPU converts it where another executor holds
/// its fine roots (fn-212 task 7, host decision 1): the structure above
/// the cut with each root laid on the CPU (D3(c)) assembled where it
/// stands, as `assemble` does, and each other live root's first axis left
/// where its stand-in stood with no phytomers, so the conversion records
/// its attachment and draws none of its wood; marked as `grow_marked`
/// marks, in the canonical order. Also each root's first axis's lineage
/// (none: shed), and the roots laid on the CPU.
pub struct Partial {
    pub structure: Structure,
    pub fine: Vec<bool>,
    pub bases: Vec<(u64, f64)>,
    pub lineages: Vec<Option<u64>>,
    pub grounded: Vec<usize>,
}

/// Each live root's first axis's lineage and its base radius as phase 2
/// handed it up, by lineage: what the conversion reads a seam's
/// codominance from on every path (fn-212 task 7, host option 3).
fn bases(settled: &Settled) -> Vec<(u64, f64)> {
    let axes = &settled.structure.axes;
    let mut out: Vec<(u64, f64)> = (settled.live.iter().zip(&settled.radii))
        .map(|(&k, &r)| (axes[settled.at[k].expect("live")].lineage, r))
        .collect();
    out.sort_by_key(|b| b.0);
    out
}

/// `settled` as the CPU converts it beside an executor holding its other
/// fine roots (`Partial`).
pub fn partial(settled: Settled) -> Partial {
    let bases = bases(&settled);
    let Settled {
        structure: mut tree,
        at,
        live,
        bearers,
        grounded,
        landed,
        ..
    } = settled;
    let lineages = (at.iter())
        .map(|a| a.map(|i| tree.axes[i].lineage))
        .collect();
    let mut fine = vec![false; tree.axes.len()];
    for i in live.iter().map(|&k| at[k].expect("live")) {
        fine[i] = true;
        tree.axes[i].phytomers.clear();
    }
    for (j, root) in landed {
        let (i, offset) = (at[live[j]].expect("live"), tree.axes.len());
        let mut axes = root.into_axes(bearers[j], (i, offset)).into_iter();
        tree.axes[i] = axes.next().expect("a kept root's first axis");
        tree.axes.extend(axes);
    }
    fine.resize(tree.axes.len(), true);
    let (structure, fine) = lineage_order_marked(tree, fine);
    Partial {
        structure,
        fine,
        bases,
        lineages,
        grounded,
    }
}

/// The seconds each phase of the expansion took.
#[derive(Default)]
pub(crate) struct Clock {
    pub phases: Vec<f64>,
}

impl Clock {
    #[cfg(not(target_family = "wasm"))]
    pub fn time<T>(&mut self, work: impl FnOnce() -> T) -> T {
        let started = crate::clock::seconds();
        let out = work();
        self.phases.push(crate::clock::seconds() - started);
        out
    }

    /// An executor's step, awaited and timed as a phase.
    pub async fn timed<T>(&mut self, step: impl std::future::Future<Output = T>) -> T {
        let started = crate::clock::seconds();
        let out = step.await;
        self.phases.push(crate::clock::seconds() - started);
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
