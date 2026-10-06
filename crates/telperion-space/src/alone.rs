//! fn-209 probe P2: a tree whose chains are grown and shed alone, each
//! from its committed boundary, never reading a realized descendant or
//! the realized whole tree; then resolved by option 4's chain geometry
//! (`bounded.rs`).
//!
//! A chain's shedding reads its own axes and, for every bud it bears, that
//! bud's committed extinction curve (the closed form's chance that the
//! bud's subtree has no living apex past a cycle, by its PA and birth). A
//! lateral or relay of the chain is shed when its subtree has held no
//! living apex for more than its delay (`shed.rs`): where its own axes
//! lived past that, it is kept; otherwise it is kept with the chance that
//! some bud it bears still lived, a draw under its own lineage. A bud on
//! a shed axis is never grown. Fades by partial presence are not taken:
//! a kept axis is whole.
//!
//! The chains are grown breadth first by `chains`, which F1's fine roots
//! share (`fine.rs`): a bud it is told to hand on is recorded and handed
//! back, its chain grown elsewhere from that record alone.
mod field;
mod shed;

use crate::bounded::{Bounded, Expected};
use crate::closed_form::boundary::Rows;
use crate::error::{Error, Result};
use crate::geometry::scale;
use crate::girth::{thicken, Girth};
use crate::grow::alone::{self, Bud};
use crate::grow::{Record, Request, Rules};
use crate::species::Species;
use crate::structure::{Axis, Structure};
pub use field::{field_probe, FieldProbe};
use shed::{in_chain, set_parent, shed};
use std::collections::VecDeque;
use std::time::Instant;

/// One chain's growth and shedding: microseconds, phytomers grown,
/// whether its first axis was kept, its order, the cycles it was grown
/// through and the axes it made, shed ones included.
#[derive(Debug, Clone, Copy)]
pub struct ChainCost {
    pub micros: f64,
    pub grown: u64,
    pub kept: bool,
    pub order: usize,
    pub cycles: u32,
    pub axes: usize,
}

/// A tree grown chain by chain alone, every chain resolved.
pub struct Alone<'s> {
    pub bounded: Bounded<'s>,
    /// Every chain grown, in the order grown (breadth first).
    pub costs: Vec<ChainCost>,
}

/// A chain to grow: the bud it grows from (none: the seed's), its
/// bearer's index among the axes kept, its bud's record, its order, and
/// its bearer's shedding context (`shed.rs`).
pub(crate) struct Item {
    pub bud: Option<Bud>,
    pub bearer: usize,
    pub record: Option<usize>,
    pub order: usize,
    pub context: Option<f64>,
}

/// What growing chains alone leaves: the axes kept, every bud made on a
/// kept axis by its record, each chain's cost, and the buds handed on,
/// each with its record and context.
pub(crate) struct Chains {
    pub axes: Vec<Axis>,
    pub records: Vec<Record>,
    pub costs: Vec<ChainCost>,
    pub handed: Vec<Item>,
}

/// Grows `species` chain by chain alone, then resolves every chain's
/// geometry against its committed boundary. Refuses a species any bud of
/// which reads light.
pub fn grow_alone<'s>(
    species: &'s Species,
    request: Request,
    expected: &Expected,
) -> Result<Alone<'s>> {
    grow_alone_with(species, request, expected, true)
}

/// `grow_alone`, its shedding conditioned on its bearer's being kept
/// (`correct`) or not.
pub fn grow_alone_with<'s>(
    species: &'s Species,
    request: Request,
    expected: &Expected,
    correct: bool,
) -> Result<Alone<'s>> {
    let rules = alone::rules(species, request.age, request.budget)?;
    let rows = expected.rows();
    let start = (Vec::new(), Vec::new(), seed_item());
    let grown = chains(&rules, &rows, request.seed, start, &|_| false, correct)?;
    let mut structure = Structure {
        age: request.age,
        pas: species.states.len(),
        axes: grown.axes,
    };
    if structure.phytomer_count() == 0 {
        return Err(Error::Collapsed);
    }
    scale(&mut structure, species);
    thicken(&mut structure, species, &Girth::default());
    let bounded = Bounded::settled(species, structure, grown.records, (rows, None));
    Ok(Alone {
        bounded,
        costs: grown.costs,
    })
}

/// The seed's chain, the first grown.
pub(crate) fn seed_item() -> Item {
    Item {
        bud: None,
        bearer: usize::MAX,
        record: None,
        order: 0,
        context: None,
    }
}

/// Grows chains breadth first from `start` (the axes and records held
/// already, and the first chain), each from its bud alone, and sheds
/// each; a bud `hand` takes is recorded and handed back, not grown.
pub(crate) fn chains(
    rules: &Rules,
    rows: &Rows,
    seed: u64,
    (axes, records, first): (Vec<Axis>, Vec<Record>, Item),
    hand: &dyn Fn(&Bud) -> bool,
    correct: bool,
) -> Result<Chains> {
    let (mut axes, mut records) = (axes, records);
    let (mut costs, mut handed) = (Vec::new(), Vec::new());
    let mut queue = VecDeque::from([first]);
    while let Some(item) = queue.pop_front() {
        let started = Instant::now();
        let chain = alone::grow(rules, seed, item.bud.as_ref())?;
        let (kept, contexts) = shed(&chain, rows, rules, item.context.filter(|_| correct));
        costs.push(ChainCost {
            micros: started.elapsed().as_secs_f64() * 1e6,
            grown: chain.grown,
            kept: kept[chain.root],
            order: item.order,
            cycles: chain.cycles,
            axes: chain.axes.len() - chain.root,
        });
        if !kept[chain.root] {
            continue;
        }
        let mut at = vec![usize::MAX; chain.axes.len()];
        if chain.root > 0 {
            at[0] = item.bearer;
        }
        for i in chain.root..chain.axes.len() {
            if !in_chain(&chain, i) || !kept[i] {
                continue;
            }
            let mut axis = chain.axes[i].clone();
            set_parent(&mut axis.origin, &at);
            at[i] = axes.len();
            axes.push(axis);
        }
        if let Some(r) = item.record {
            records[r].axis = Some(at[chain.root]);
        }
        for (b, context) in chain.buds.into_iter().zip(contexts) {
            let bearer = at[b.bearer()];
            if bearer == usize::MAX {
                continue;
            }
            let mut origin = b.axis.origin;
            set_parent(&mut origin, &at);
            records.push(Record {
                origin,
                pa: b.axis.pa,
                birth: b.axis.birth,
                lineage: b.axis.lineage,
                vigour: b.axis.vigour,
                axis: None,
            });
            let next = Item {
                record: Some(records.len() - 1),
                order: item.order + 1,
                bearer,
                context,
                bud: None,
            };
            if hand(&b) {
                handed.push(Item {
                    bud: Some(b),
                    ..next
                });
            } else {
                queue.push_back(Item {
                    bud: Some(b),
                    ..next
                });
            }
        }
    }
    Ok(Chains {
        axes,
        records,
        costs,
        handed,
    })
}
