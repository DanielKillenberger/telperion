//! fn-223 R1: an axis of an age that states a cluster is a short shoot,
//! and its cluster stands at its own last drawn node, one an axis; every
//! node it draws is marked, and an axis that draws none bears none.
use super::*;
use crate::presets::Preset;
use std::collections::{HashMap, HashSet};

/// The beech's short shoots' age.
const SHORT: usize = 14;

#[test]
fn a_short_shoots_cluster_stands_at_its_own_tip() {
    let family = Preset::EuropeanBeech.parameters();
    let mut species = family.space.species.unwrap();
    (
        species.states[SHORT].cluster_leaves,
        species.states[SHORT].cluster_spread,
    ) = (3.0, 0.6);
    let request = Request {
        age: 15,
        seed: 7,
        budget: BUDGET,
        light: family.space.light,
    };
    let structure = telperion_space::grow(&species, request).unwrap();
    let tree = convert(
        &structure,
        (&[], &[]),
        (&family.space.trunk, &pa_rows(&species)),
    );
    // Each node by the phytomer that made it.
    let node: HashMap<u64, usize> = (1..tree.nodes.len())
        .map(|i| (tree.nodes[i].identity.birth_order(), i))
        .collect();
    let (mut tips, mut bare, mut marked) = (HashSet::new(), 0, HashSet::new());
    for axis in &structure.axes {
        let drawn: Vec<usize> = (axis.phytomers.iter())
            .filter_map(|p| node.get(&p.key()).copied())
            .collect();
        if axis.pa != SHORT {
            continue;
        }
        marked.extend(drawn.iter().copied());
        match drawn.last() {
            Some(&tip) => assert!(tips.insert(tip), "two axes share a tip"),
            None => bare += 1,
        }
    }
    assert!(tips.len() > 100, "{} short shoots", tips.len());
    assert!(bare > 0, "every short shoot drew a node");
    let found: HashSet<usize> = tree.clusters.iter().map(|c| c.node as usize).collect();
    assert_eq!(found.len(), tree.clusters.len(), "a tip bears two clusters");
    assert_eq!(found, tips);
    assert!(tree
        .clusters
        .iter()
        .all(|c| (c.leaves, c.spread) == (3, 0.6)));
    for i in 0..tree.nodes.len() {
        assert_eq!(tree.is_clustered(i), marked.contains(&i), "node {i}");
    }
}

/// At no cluster the tree carries nothing new.
#[test]
fn a_tree_of_no_cluster_carries_none() {
    let family = Preset::EuropeanBeech.parameters();
    let species = family.space.species.unwrap();
    let request = Request {
        age: 15,
        seed: 7,
        budget: BUDGET,
        light: family.space.light,
    };
    let structure = telperion_space::grow(&species, request).unwrap();
    let tree = convert(
        &structure,
        (&[], &[]),
        (&family.space.trunk, &pa_rows(&species)),
    );
    assert!(tree.clusters.is_empty() && tree.clustered.is_empty());
}
