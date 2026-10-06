//! A chain's shedding from its own axes and its borne buds' committed
//! extinction curves alone (fn-209 probe P2, `alone.rs`).
use crate::closed_form::boundary::Rows;
use crate::fixed::decide;
use crate::grow::alone::Chain;
use crate::grow::Rules;
use crate::lineage::{Key, SHED};
use crate::structure::Origin;

/// Whether axis `i` of `chain` is one of its own: its first axis, or a
/// continuation or relay of one.
pub(super) fn in_chain(chain: &Chain, i: usize) -> bool {
    i == chain.root || (i > chain.root && !matches!(chain.axes[i].origin, Origin::Lateral { .. }))
}

pub(super) fn set_parent(origin: &mut Origin, at: &[usize]) {
    match origin {
        Origin::Seed => {}
        Origin::Lateral { parent, .. }
        | Origin::Continuation { parent }
        | Origin::Relay { parent, .. } => *parent = at[*parent],
    }
}

/// Which of the chain's axes are kept at the tree's age, from its own
/// axes and its borne buds' extinction curves alone.
///
/// Where the bud that began the chain was borne on an axis whose own wood
/// had died by its threshold, that axis was kept on its borne buds'
/// chances alone, so this chain's being kept is conditioned on it:
/// `context` is the chance the bearer's other buds had all died, recorded
/// by the bearer, and the chain's own chance π becomes π / (1 - (1 - π)
/// context), the chance it lived given that its bearer was kept. The
/// second value is each borne bud's context, by its place in `chain.buds`.
/// Chances of death are summed as `-log2`, Q32, and the decision taken
/// on the contract's fixed point (FINE-CONTRACT.md D15).
pub(super) fn shed(
    chain: &Chain,
    rows: &Rows,
    rules: &Rules,
    context: Option<i64>,
) -> (Vec<bool>, Vec<Option<i64>>) {
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
    let mut dead = vec![0i64; n];
    for b in &chain.buds {
        let mut j = Some(b.bearer()).filter(|&j| j >= chain.root);
        while let Some(at) = j {
            if let Some(t) = threshold[at] {
                let p = rows.dead_by(b.axis.pa, b.axis.birth, t);
                dead[at] = decide::and(dead[at], decide::neg_log(p));
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
                let d = Key(chain.axes[i].lineage)
                    .child(SHED)
                    .child(u64::MAX)
                    .draw();
                decide::kept(dead[i], context.filter(|_| i == chain.root), d.0)
            }
        };
        kept[i] = above && pass;
    }
    let contexts = chain
        .buds
        .iter()
        .enumerate()
        .map(|(k, b)| {
            let mut j = Some(b.bearer()).filter(|&j| j >= chain.root);
            while let Some(at) = j {
                if let Some(t) = threshold[at] {
                    if until[at] > t {
                        return None;
                    }
                    let others = chain
                        .buds
                        .iter()
                        .enumerate()
                        .filter(|&(o, c)| o != k && bears(chain, c.bearer(), at))
                        .map(|(_, c)| decide::neg_log(rows.dead_by(c.axis.pa, c.axis.birth, t)))
                        .fold(0, decide::and);
                    return Some(others);
                }
                j = parent(at);
            }
            None
        })
        .collect();
    (kept, contexts)
}

/// Whether own axis `j` of `chain` is `at` or stands on it.
fn bears(chain: &Chain, mut j: usize, at: usize) -> bool {
    loop {
        if j == at {
            return true;
        }
        if j < chain.root || !in_chain(chain, j) {
            return false;
        }
        match chain.axes[j].origin.parent() {
            Some(p) if p >= chain.root => j = p,
            _ => return false,
        }
    }
}
