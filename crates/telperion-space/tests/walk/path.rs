//! The longest path along the wood from the root collar to a living tip
//! of a grown tree (fn-349, DEVICE-ANSWERS.md 1): the unit a walk over the
//! inverse height is measured in, as `sag_bend` measures sag's.
use telperion_space::{Origin, Structure};

/// The longest path, in metres, from the root collar to a living tip:
/// each axis's path is its bearer's to the node it stands on (a lateral
/// or relay) or to its tip (a continuation), then its own internodes.
pub fn longest_path(tree: &Structure) -> f64 {
    let n = tree.axes.len();
    let mut at: Vec<Option<Vec<f64>>> = vec![None; n];
    let mut most = 0.0f64;
    for i in 0..n {
        let path = along(tree, i, &mut at);
        let a = &tree.axes[i];
        if a.apex_end.is_none() {
            most = most.max(path.last().copied().unwrap_or(path[0]));
        }
    }
    most
}

/// Axis `i`'s path at its base and at each node, memoised in `at`.
fn along(tree: &Structure, i: usize, at: &mut Vec<Option<Vec<f64>>>) -> Vec<f64> {
    if let Some(p) = &at[i] {
        return p.clone();
    }
    let a = &tree.axes[i];
    let root = match a.origin {
        Origin::Seed => 0.0,
        Origin::Lateral { parent, node, .. } | Origin::Relay { parent, node } => {
            let p = along(tree, parent, at);
            p[(node + 1).min(p.len() - 1)]
        }
        Origin::Continuation { parent } => *along(tree, parent, at).last().expect("a base"),
    };
    let mut path = vec![root];
    let mut from = a.base;
    for p in &a.phytomers {
        path.push(path.last().expect("a base") + (p.tip - from).length());
        from = p.tip;
    }
    at[i] = Some(path.clone());
    path
}
