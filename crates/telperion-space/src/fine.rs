//! Fine orders grown in the expansion stage (fn-209 F1; DESIGN-FINE.md,
//! accepted with Astra's changes, ASTRA-REVIEW-FINE.md).
//!
//! The CPU grows the structure above a cut and records each bud below it
//! as a fine root (`record.rs`); the expansion grows every fine root from
//! its record alone (`executor.rs`). The cut is chosen before growth, by
//! the closed form's expected kept phytomers of the bud (its committed
//! row's `count`, light-neutral), so which buds are fine never depends on
//! what they grow into.
//!
//! The cut decides where the work runs, never what is computed (Astra's
//! change 2): a chain is grown from its bud alone and shed by its own axes
//! and its borne buds' committed extinction, and resolved by option 4
//! against its bearer's laid frame and its own committed boundary, on
//! either side of the cut. So a tree is the same, to the bit, at every
//! cut, in every order of its items and in any chunking of them, and it
//! is the tree P2's chain grower grows whole (`tests.rs`).
mod canonical;
mod executor;
mod record;
mod survey;
mod surface;
mod tables;
#[cfg(test)]
mod tests;

use crate::alone::{chains, seed_item, Chains, Item};
use crate::bounded::{Bounded, Expected};
use crate::error::{Error, Result};
use crate::geometry::scale;
use crate::grow::alone::rules;
use crate::grow::Request;
use crate::species::Species;
use crate::structure::{Axis, Origin, Structure};
pub use canonical::{canonical, digest, identities};
pub use executor::Work;
use executor::{expand, Context, Expanded, BEARER};
pub use record::FineRoot;
use record::{Attachment, Committed, Frame};
pub use survey::{survey_lit, Surveyed};
pub use surface::{chain_cases, ChainAxis, ChainBud, ChainTables, PaRow, Woke};
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

/// A tree grown above the cut and expanded below it: the tree (the
/// structure above the cut first, then each fine root's axes in the
/// order the roots were recorded), its axes above the cut, and each
/// root's record, kept phytomers and work.
pub struct Fine {
    pub structure: Structure,
    pub above: usize,
    pub roots: Vec<(FineRoot, usize, Work)>,
}

/// Grows `species` above `plan`'s cut on the CPU, then expands every fine
/// root from its record alone on the CPU reference executor. Refuses a
/// species any bud of which reads light (F2b).
pub fn grow_fine(
    species: &Species,
    request: Request,
    expected: &Expected,
    plan: Plan,
) -> Result<Fine> {
    let rules = rules(species, request.age, request.budget)?;
    let rows = expected.rows();
    let threads = match plan.threads {
        0 => crate::cores::available(),
        n => n,
    };
    let fine =
        |bud: &crate::grow::alone::Bud| rows.lateral(bud.axis.pa, bud.axis.birth).count <= plan.cut;
    let start = (Vec::new(), Vec::new(), seed_item());
    let Chains {
        axes,
        records,
        handed,
        ..
    } = chains(&rules, &rows, request.seed, start, (&fine, &mut |_, _| {}), true)?;
    let mut above = Structure {
        age: request.age,
        pas: species.states.len(),
        axes,
    };
    if above.phytomer_count() == 0 && handed.is_empty() {
        return Err(Error::Collapsed);
    }
    scale(&mut above, species);
    let mut bounded = Bounded::settled(species, above, records, (rows.clone(), None));
    bounded.resolve_all(threads)?;
    let roots: Vec<FineRoot> = handed
        .into_iter()
        .map(|item| record(&bounded, species, item))
        .collect();
    let cx = Context {
        species,
        rules: &rules,
        rows: &rows,
        seed: request.seed,
        age: request.age,
    };
    let expanded = execute(&cx, &roots, plan, threads)?;
    let mut structure = bounded.into_structure();
    let above = structure.axes.len();
    let mut out = Vec::with_capacity(roots.len());
    for (root, done) in roots.into_iter().zip(expanded) {
        let offset = structure.axes.len();
        let kept = done.axes.iter().map(|a| a.phytomers.len()).sum();
        let bearer = root.attachment.bearer;
        structure
            .axes
            .extend(done.axes.into_iter().map(|a| placed(a, offset, bearer)));
        out.push((root, kept, done.work));
    }
    if structure.phytomer_count() == 0 {
        return Err(Error::Collapsed);
    }
    Ok(Fine {
        structure,
        above,
        roots: out,
    })
}

/// The record of the bud `item` hands on, from the laid tree above the cut.
fn record(bounded: &Bounded, species: &Species, item: Item) -> FineRoot {
    let r = item.record.expect("a handed bud is recorded");
    let record = bounded.records()[r];
    let Origin::Lateral { parent, node, .. } = record.origin else {
        unreachable!("a fine root is a lateral bud")
    };
    let (unbent, bent) = (
        &bounded.unbent().axes[parent],
        &bounded.structure().axes[parent],
    );
    let attachment = Attachment {
        bearer: parent,
        pa: bent.pa,
        lineage: bent.lineage,
        birth: bent.birth,
        node,
        unbent: Frame::of(unbent, node),
        bent: Frame::of(bent, node),
    };
    debug_assert_eq!(
        attachment.bent.at.size, 1.0,
        "no unit is sized where no bud reads light"
    );
    let rows = bounded.rows_of();
    let row = rows.lateral(record.pa, record.birth);
    let s = bounded.base_scale(&record);
    let committed = Committed {
        section: bounded.handed(&record, species.states[bent.pa].form.exponent),
        mass: row.mass * s.powi(3),
        moment: row.moment * s.powi(4),
        radius: bounded.base_radius(row, s, record.pa),
    };
    FineRoot {
        bud: item.bud.expect("a handed bud"),
        context: item.context,
        record,
        attachment,
        committed,
        predicted: row.count,
    }
}

/// Every root expanded, by its place among `roots`, taken in the plan's
/// order, `chunk` at a time on `threads` cores.
fn execute(cx: &Context, roots: &[FineRoot], plan: Plan, threads: usize) -> Result<Vec<Expanded>> {
    use std::sync::Mutex;
    let order = ordered(roots.len(), plan.order);
    let chunk = plan.chunk.max(1);
    let pieces = order.len().div_ceil(chunk);
    let done: Vec<Mutex<Option<Result<Expanded>>>> =
        (0..roots.len()).map(|_| Mutex::new(None)).collect();
    crate::cores::spread(pieces, threads, &|k| {
        for &i in &order[k * chunk..((k + 1) * chunk).min(order.len())] {
            *done[i].lock().unwrap() = Some(expand(cx, &roots[i]));
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

/// An expanded axis placed in the whole tree: its root on its bearer, the
/// rest after `offset`.
fn placed(mut axis: Axis, offset: usize, bearer: usize) -> Axis {
    match &mut axis.origin {
        Origin::Seed => unreachable!("a fine root is a lateral bud"),
        Origin::Lateral { parent, .. }
        | Origin::Continuation { parent }
        | Origin::Relay { parent, .. } => {
            *parent = if *parent == BEARER {
                bearer
            } else {
                *parent + offset
            };
        }
    }
    axis
}
