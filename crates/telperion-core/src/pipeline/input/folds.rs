//! The grown tree's whole-tree folds (fn-212 task 4): its height, the
//! lowest node of its crown and its widest reach. Each is a fold over the
//! tree's nodes that any split of them reaches exactly, so an executor
//! that holds part of a tree's nodes (the GPU's fine roots) folds its part
//! and the pipeline combines it with its own. The shoulder needs these
//! folds first; it then reads the crown's points (`shoulder.rs`).
use crate::math::{Transcendental, Vec3};
use crate::tree::Node;

/// The folds over some of a tree's nodes.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Folds {
    /// The highest node's height, and never under the ground (0).
    pub height: f64,
    /// The lowest crown node's height (off the stem and not the root);
    /// infinite with none.
    pub low: f64,
    /// The widest node's reach off the root and its height; with several
    /// as wide, the last folded; none with no node.
    pub widest: Option<(f64, f64)>,
}

impl Default for Folds {
    /// The folds over no node.
    fn default() -> Self {
        Self {
            height: 0.0,
            low: f64::INFINITY,
            widest: None,
        }
    }
}

impl Folds {
    /// The folds of one node at `position`, in the crown or not.
    pub fn node(position: Vec3, crown: bool) -> Self {
        Self {
            height: position.y.max(0.0),
            low: if crown { position.y } else { f64::INFINITY },
            widest: Some((reach(position), position.y)),
        }
    }

    /// The folds over the nodes `keep` keeps, in their order.
    pub fn of(nodes: &[Node], keep: impl Fn(usize) -> bool) -> Self {
        let crown = |n: &Node| !n.stem && n.parent.is_some();
        (nodes.iter().enumerate())
            .filter(|&(i, _)| keep(i))
            .fold(Self::default(), |f, (_, n)| {
                f.with(Self::node(n.position, crown(n)))
            })
    }

    /// These folds and `later`'s, `later` folded after: the folds over
    /// both sets of nodes, exactly.
    pub fn with(self, later: Self) -> Self {
        let widest = match (self.widest, later.widest) {
            (Some(a), Some(b)) => Some(if b.0.total_cmp(&a.0).is_ge() { b } else { a }),
            (a, b) => b.or(a),
        };
        Self {
            height: self.height.max(later.height),
            low: self.low.min(later.low),
            widest,
        }
    }

    /// The crown's base, height, widest reach and fullness, which the
    /// envelope and the shoulder read.
    pub fn shape(&self) -> [f64; 4] {
        let height = self.height;
        let base = self.low.min(height);
        let (radius, at) = self.widest.unwrap_or((0.0, height));
        let fullness = share(at - base, height - base);
        [base, height, radius, fullness]
    }
}

/// A node's reach off the root's vertical.
pub fn reach(position: Vec3) -> f64 {
    position.x.hypot_fixed(position.z)
}

/// `v` as a share of `of`, from 0 to 1; 0 where `of` is none.
pub(super) fn share(v: f64, of: f64) -> f64 {
    if of > 0.0 {
        (v / of).clamp(0.0, 1.0)
    } else {
        0.0
    }
}

#[cfg(test)]
mod tests {
    use super::super::{bins, envelope, grown_envelope};
    use super::*;
    use crate::pipeline::executor;

    /// The folds, the shoulder's bins and the leaf box of a grown tree's
    /// nodes split in two (its fine roots' seams' subtrees and the rest,
    /// and every other node) combine to the whole tree's, to the bit.
    #[test]
    fn parts_of_a_tree_combine_to_the_whole() {
        let mut family = crate::presets::by_identity("european-beech").unwrap();
        family.age = 30.0;
        let expansion = executor::grow(&family).unwrap().expansion().unwrap();
        let tree = expansion.tree();
        assert!(!tree.seams.is_empty() && !tree.attachments.is_empty());
        let mut fine = vec![false; tree.nodes.len()];
        for &s in &tree.seams {
            fine[s as usize] = true;
        }
        for (i, n) in tree.nodes.iter().enumerate().skip(1) {
            fine[i] |= fine[n.parent.unwrap() as usize];
        }
        let odd: Vec<bool> = (0..tree.nodes.len()).map(|i| i % 2 == 1).collect();
        let whole_box = expansion.wood_box(|_| true).unwrap();
        let spanned = crate::foliage::Reference::spanning(whole_box.0, whole_box.1);
        assert_eq!(spanned, expansion.reference());
        for side in [&fine, &odd] {
            let (a, b) = (
                Folds::of(&tree.nodes, |i| !side[i]),
                Folds::of(&tree.nodes, |i| side[i]),
            );
            assert_eq!(a.with(b), Folds::of(&tree.nodes, |_| true));
            let shape = a.with(b).shape();
            let at = |keep: bool| {
                let nodes = tree.nodes.iter().enumerate();
                nodes
                    .filter(move |(i, _)| side[*i] == keep)
                    .map(|(_, n)| n.position)
            };
            let most: Vec<f64> = (bins(at(false), shape).iter())
                .zip(bins(at(true), shape))
                .map(|(x, y)| x.max(y))
                .collect();
            assert_eq!(most, bins(tree.nodes.iter().map(|n| n.position), shape));
            let (lo, hi) = expansion.wood_box(|i| !side[i]).unwrap();
            let (l2, h2) = expansion.wood_box(|i| side[i]).unwrap();
            let combined = (
                Vec3::new(lo.x.min(l2.x), lo.y.min(l2.y), lo.z.min(l2.z)),
                Vec3::new(hi.x.max(h2.x), hi.y.max(h2.y), hi.z.max(h2.z)),
            );
            assert_eq!(combined, whole_box);
        }
        // The whole envelope from the folds and every node's position.
        let points = tree.nodes.iter().map(|n| n.position);
        assert_eq!(
            envelope(Folds::of(&tree.nodes, |_| true), points),
            grown_envelope(tree)
        );
    }
}
