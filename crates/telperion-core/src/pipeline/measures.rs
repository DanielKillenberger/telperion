//! fn-323's two metrics, as the manifest of goal fn-321 defines them. A tree
//! a metric cannot be read from is refused, never a zero.
use super::foliage::clumping::systems;
use crate::{tree::Tree, Error, Result};
use std::collections::BTreeSet;

/// The distinct limb systems of `tree` at `order`, over its nodes.
pub fn limb_count(tree: &Tree, order: u32) -> Result<usize> {
    if tree.nodes.is_empty() {
        return Err(Error::InvalidInput("a limb count of a tree with no nodes"));
    }
    Ok(systems(tree, order).into_iter().collect::<BTreeSet<_>>().len())
}

/// One less the phytomers a finished tree keeps over those grown for it, the
/// retired and shed ones included.
pub fn shed_share(kept: u64, grown: u64) -> Result<f64> {
    let held = kept as f64 / grown as f64;
    if !(0.0..=1.0).contains(&held) {
        return Err(Error::InvalidInput("a shed share of kept phytomers past those grown"));
    }
    Ok(1.0 - held)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::Vec3;
    use crate::tree::{Node, NodeKind};

    /// A trunk of three nodes with `laterals` limbs off its middle node, each
    /// carried on by one node and bearing one thinner lateral of its own.
    fn tree(laterals: usize) -> Tree {
        let node = |parent: u32, radius: f64, x: f64| Node {
            parent: Some(parent),
            position: Vec3::new(x, f64::from(parent), 0.0),
            radius,
            start_radius: radius,
            kind: NodeKind::Structural,
            ..Node::root()
        };
        let mut nodes = vec![Node::root(), node(0, 1.0, 0.0), node(1, 0.9, 0.0), node(2, 0.8, 0.0)];
        for k in 0..laterals {
            let limb = nodes.len() as u32;
            nodes.push(node(2, 0.3, 1.0 + k as f64));
            nodes.push(node(limb, 0.25, 2.0 + k as f64));
            nodes.push(node(limb, 0.1, 3.0 + k as f64));
        }
        Tree {
            nodes,
            ..Tree::default()
        }
    }

    #[test]
    fn each_limb_opens_a_system_at_its_order_and_only_there() {
        // The trunk's stem, and each limb at order 1; each limb's lateral too
        // at order 2.
        assert_eq!(limb_count(&tree(3), 0).unwrap(), 2);
        assert_eq!(limb_count(&tree(3), 1).unwrap(), 5);
        assert_eq!(limb_count(&tree(3), 2).unwrap(), 8);
    }

    #[test]
    fn a_tree_with_no_nodes_has_no_limb_count_to_read() {
        assert!(limb_count(&Tree::default(), 1).is_err());
    }

    #[test]
    fn the_shed_share_is_what_the_tree_did_not_keep() {
        assert_eq!(shed_share(75, 100).unwrap(), 0.25);
        assert_eq!(shed_share(100, 100).unwrap(), 0.0);
    }

    #[test]
    fn a_shed_share_that_cannot_be_read_is_refused() {
        assert!(shed_share(0, 0).is_err());
        assert!(shed_share(101, 100).is_err());
    }
}
