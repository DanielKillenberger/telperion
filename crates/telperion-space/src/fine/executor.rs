//! The expansion's fine growth, on the CPU reference executor (fn-209 F1;
//! `docs/pipeline.md`, "Fine growth"): one fine root grown, shed, scaled,
//! girthed and laid from its record alone.
//!
//! The root's tree holds its bearer as a stub (axis 0: the node's
//! phytomer on the point below it, before bending and as laid), then the
//! root's own chains, grown breadth first by P2's chain grower from the
//! root's bud, each shed by its own axes and its borne buds' committed
//! extinction, each resolved by option 4 against its committed boundary
//! and its bearer's laid frame. Every step is the one the structure above
//! the cut runs on its own chains, so a chain is the same chain whichever
//! side of the cut grows it.
use super::record::FineRoot;
use crate::alone::{chains, Item};
use crate::bounded::Bounded;
use crate::closed_form::boundary::Rows;
use crate::error::Result;
use crate::geometry::scale;
use crate::grow::Rules;
use crate::species::Species;
use crate::structure::{Axis, Origin, Structure};
use std::sync::Arc;
use std::time::Instant;

/// The root's parent in an expanded root's axes: its bearer above the cut.
pub(crate) const BEARER: usize = usize::MAX;

/// A fine root expanded: its kept axes (the first its root's, whose
/// parent is `BEARER`; the rest numbered among them), and its work.
#[derive(Debug, Clone, Default)]
pub(crate) struct Expanded {
    pub axes: Vec<Axis>,
    pub work: Work,
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

/// What every item of a tree reads: the species, the chain rules, the
/// committed rows, the seed and the tree's age.
pub(crate) struct Context<'a> {
    pub species: &'a Species,
    pub rules: &'a Rules<'a>,
    pub rows: &'a Arc<Rows>,
    pub seed: u64,
    pub age: u32,
}

/// Expands `root` from its record alone.
pub(crate) fn expand(cx: &Context, root: &FineRoot) -> Result<Expanded> {
    let started = Instant::now();
    let (unbent, bent) = root.attachment.stubs();
    let mut record = root.record;
    record.origin = local(record.origin);
    record.axis = None;
    let first = Item {
        bud: Some(root.bud.clone()),
        bearer: 0,
        record: Some(0),
        order: 1,
        context: root.context,
    };
    let start = (vec![unbent.clone()], vec![record], first);
    let grown = chains(cx.rules, cx.rows, cx.seed, start, &|_| false, true)?;
    let mut work = Work {
        chains: grown.costs.len(),
        ..Work::default()
    };
    for c in &grown.costs {
        work.grown += c.grown;
        work.cycles += u64::from(c.cycles);
        work.axes += c.axes;
    }
    if grown.axes.len() == 1 {
        work.micros = started.elapsed().as_secs_f64() * 1e6;
        return Ok(Expanded {
            axes: Vec::new(),
            work,
        });
    }
    let mut axes = grown.axes;
    axes[1].origin = local(axes[1].origin);
    let mut tree = Structure {
        age: cx.age,
        pas: cx.species.states.len(),
        axes,
    };
    scale(&mut tree, cx.species);
    tree.axes[0] = unbent;
    let mut bounded =
        Bounded::settled(cx.species, tree, grown.records, (cx.rows.clone(), None)).on_stub(bent);
    bounded.resolve_all(1)?;
    let node = root.attachment.node;
    let axes = bounded
        .into_structure()
        .axes
        .into_iter()
        .skip(1)
        .map(|mut axis| {
            axis.origin = match axis.origin {
                Origin::Lateral {
                    parent: 0,
                    slot,
                    whorl,
                    woken,
                    ..
                } => Origin::Lateral {
                    parent: BEARER,
                    node,
                    slot,
                    whorl,
                    woken,
                },
                other => shifted(other),
            };
            axis
        })
        .collect();
    work.micros = started.elapsed().as_secs_f64() * 1e6;
    Ok(Expanded { axes, work })
}

/// The root's origin in its own tree: on the stub's one phytomer.
fn local(origin: Origin) -> Origin {
    match origin {
        Origin::Lateral {
            slot, whorl, woken, ..
        } => Origin::Lateral {
            parent: 0,
            node: 0,
            slot,
            whorl,
            woken,
        },
        _ => unreachable!("a fine root is a lateral bud"),
    }
}

/// An origin among the expanded axes: one below its index in the root's
/// tree, which held the stub first.
fn shifted(origin: Origin) -> Origin {
    match origin {
        Origin::Seed => unreachable!("only the stub is the seed"),
        Origin::Lateral {
            parent,
            node,
            slot,
            whorl,
            woken,
        } => Origin::Lateral {
            parent: parent - 1,
            node,
            slot,
            whorl,
            woken,
        },
        Origin::Continuation { parent } => Origin::Continuation { parent: parent - 1 },
        Origin::Relay { parent, node } => Origin::Relay {
            parent: parent - 1,
            node,
        },
    }
}
