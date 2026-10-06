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
use crate::bounded::{Bounded, Expected};
use crate::closed_form::boundary::Rows;
use crate::error::{Error, Result};
use crate::geometry::scale;
use crate::girth::{thicken, Girth};
use crate::grow::alone::{self, Bud, Chain};
use crate::grow::{Record, Request, Rules};
use crate::lineage::{Key, SHED};
use crate::species::Species;
use crate::structure::{Axis, Origin, Structure};
use std::collections::VecDeque;
use std::time::Instant;

/// One chain's growth and shedding: microseconds, phytomers grown, and
/// whether its first axis was kept.
#[derive(Debug, Clone, Copy)]
pub struct ChainCost {
    pub micros: f64,
    pub grown: u64,
    pub kept: bool,
    pub order: usize,
}

/// A tree grown chain by chain alone, every chain resolved.
pub struct Alone<'s> {
    pub bounded: Bounded<'s>,
    /// Every chain grown, in the order grown (breadth first).
    pub costs: Vec<ChainCost>,
}

struct Item {
    bud: Option<Bud>,
    bearer: usize,
    record: Option<usize>,
    order: usize,
}

/// Grows `species` chain by chain alone, then resolves every chain's
/// geometry against its committed boundary. Refuses a species any bud of
/// which reads light.
pub fn grow_alone<'s>(
    species: &'s Species,
    request: Request,
    expected: &Expected,
) -> Result<Alone<'s>> {
    let rules = alone::rules(species, request.age, request.budget)?;
    let rows = expected.rows();
    let mut axes: Vec<Axis> = Vec::new();
    let mut records: Vec<Record> = Vec::new();
    let mut costs = Vec::new();
    let mut queue = VecDeque::from([Item {
        bud: None,
        bearer: usize::MAX,
        record: None,
        order: 0,
    }]);
    while let Some(item) = queue.pop_front() {
        let started = Instant::now();
        let chain = alone::grow(&rules, request.seed, item.bud.as_ref())?;
        let kept = shed(&chain, &rows, &rules);
        costs.push(ChainCost {
            micros: started.elapsed().as_secs_f64() * 1e6,
            grown: chain.grown,
            kept: kept[chain.root],
            order: item.order,
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
        for b in chain.buds {
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
            queue.push_back(Item {
                bud: Some(b),
                bearer,
                record: Some(records.len() - 1),
                order: item.order + 1,
            });
        }
    }
    let mut structure = Structure {
        age: request.age,
        pas: species.states.len(),
        axes,
    };
    if structure.phytomer_count() == 0 {
        return Err(Error::Collapsed);
    }
    scale(&mut structure, species);
    thicken(&mut structure, species, &Girth::default());
    let bounded = Bounded::settled(species, structure, records, (rows, None));
    Ok(Alone { bounded, costs })
}

/// Whether axis `i` of `chain` is one of its own: its first axis, or a
/// continuation or relay of one.
fn in_chain(chain: &Chain, i: usize) -> bool {
    i == chain.root
        || (i > chain.root && !matches!(chain.axes[i].origin, Origin::Lateral { .. }))
}

fn set_parent(origin: &mut Origin, at: &[usize]) {
    match origin {
        Origin::Seed => {}
        Origin::Lateral { parent, .. }
        | Origin::Continuation { parent }
        | Origin::Relay { parent, .. } => *parent = at[*parent],
    }
}

/// Which of the chain's axes are kept at the tree's age, from its own
/// axes and its borne buds' extinction curves alone.
fn shed(chain: &Chain, rows: &Rows, rules: &Rules) -> Vec<bool> {
    let (species, age) = (rules.species, i64::from(rules.age));
    let n = chain.axes.len();
    let parent = |i: usize| {
        chain.axes[i]
            .origin
            .parent()
            .filter(|&p| p >= chain.root && in_chain(chain, i))
    };
    // Each own axis's delay's last dead cycle, where it is shed at all.
    let threshold: Vec<Option<i64>> = (0..n)
        .map(|i| {
            if !in_chain(chain, i) {
                return None;
            }
            let a = &chain.axes[i];
            let d = species.states[a.pa].shedding;
            let sheds = matches!(a.origin, Origin::Lateral { .. } | Origin::Relay { .. });
            (sheds && d.is_finite()).then(|| (age as f64 - d - 1.0).floor() as i64)
        })
        .collect();
    // The last cycle its own axes lived, through each own axis's subtree.
    let mut until = vec![i64::MIN; n];
    for i in (chain.root..n).rev().filter(|&i| in_chain(chain, i)) {
        let own = chain.axes[i].apex_end.map_or(i64::MAX, i64::from);
        until[i] = until[i].max(own);
        if let Some(p) = parent(i) {
            until[p] = until[p].max(until[i]);
        }
    }
    // The chance every bud borne in each own axis's subtree had died by
    // that axis's threshold.
    let mut dead = vec![1.0f64; n];
    for b in &chain.buds {
        let mut j = Some(b.bearer()).filter(|&j| j >= chain.root);
        while let Some(at) = j {
            if let Some(t) = threshold[at] {
                dead[at] *= rows.dead_by(b.axis.pa, b.axis.birth, t);
            }
            j = parent(at);
        }
    }
    let mut kept = vec![false; n];
    for i in (chain.root..n).filter(|&i| in_chain(chain, i)) {
        let above = parent(i).map_or(true, |p| kept[p]);
        let pass = match threshold[i] {
            None => true,
            Some(t) if until[i] > t => true,
            Some(_) => {
                let u = Key(chain.axes[i].lineage).child(SHED).child(u64::MAX).unit();
                u >= dead[i]
            }
        };
        kept[i] = above && pass;
    }
    kept
}
