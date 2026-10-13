//! fn-248 R1: each drawn node records its wood's leaf retention by age,
//! the phytomers merged into it included; a tree whose ages bear for ever
//! records none.
use super::*;
use crate::presets::Preset;
use crate::tree::{kept_share, retention, KEEP};
use std::collections::HashMap;

fn grown(leaf_years: f64) -> (Structure, Tree, telperion_space::Species) {
    let family = Preset::EuropeanBeech.parameters();
    let mut species = family.space.species.unwrap();
    for s in &mut species.states {
        s.leaf_years = leaf_years;
    }
    let request = Request {
        age: 15,
        seed: 7,
        budget: BUDGET,
        light: family.space.light,
    };
    let structure = telperion_space::grow(&species, request).unwrap();
    let rows = pa_rows(&species);
    let tree = convert(&structure, (&family.space.trunk, &rows));
    (structure, tree, species)
}

#[test]
fn a_tree_whose_ages_bear_for_ever_records_no_retention() {
    let (_, tree, _) = grown(f64::INFINITY);
    assert!(tree.leafing.is_empty());
}

/// Each node's retention is its own phytomer's after the boundary, and
/// before it the retention of the wood merged in ahead of the last change.
#[test]
fn each_node_keeps_the_retention_of_the_wood_it_draws() {
    let (structure, tree, species) = grown(1.5);
    assert_eq!(tree.leafing.len(), tree.nodes.len());
    let node: HashMap<u64, usize> = (1..tree.nodes.len())
        .map(|i| (tree.nodes[i].identity.birth_order(), i))
        .collect();
    let (mut shares, mut mixed) = ([0usize; 3], 0);
    for axis in &structure.axes {
        let r = retention(species.states[axis.pa].leaf_years);
        let row = pa_rows(&species)[axis.pa];
        // The phytomers merged into the next node: their wood and share.
        let mut merged: Vec<(f64, u16)> = Vec::new();
        for p in &axis.phytomers {
            let q = kept_share(r, structure.age - p.cycle);
            let length =
                row.internode * p.scale * telperion_space::internode_factor(p.key(), row.spread);
            merged.push((length, q));
            let Some(&i) = node.get(&p.key()) else {
                continue;
            };
            let l = tree.leafing[i];
            assert_eq!(l.keep as u16, q, "node {i}: its own phytomer's");
            let last = merged.iter().rposition(|m| m.1 != q);
            match last {
                None => assert_eq!((l.older, l.keep >> 16), (0.0, u32::from(q)), "node {i}"),
                Some(k) => {
                    mixed += 1;
                    let older: f64 = merged[..=k].iter().map(|m| m.0).sum();
                    assert!((l.older - older).abs() <= 1e-12 * older, "node {i}");
                    assert_eq!(l.keep >> 16, u32::from(merged[k].1), "node {i}");
                }
            }
            shares[match q {
                KEEP => 0,
                0 => 2,
                _ => 1,
            }] += 1;
            merged.clear();
        }
    }
    // Every share is drawn: this year's wood keeps all, last year's half,
    // older none.
    assert!(shares.iter().all(|&n| n > 100), "{shares:?}");
    assert_eq!(retention(1.5), (1, 32768));
    println!("shares {shares:?}, {mixed} nodes of mixed wood");
}

/// fn-270 R5a: drawn within its year, a node keeps the leaves its wood
/// kept a year before, those it keeps no longer drawn at the share left
/// of the year (`Leafing::share`), so last year's leaves fall through the
/// year, not at its turn; at the whole year it is the tree grown.
#[test]
fn within_its_year_the_leaves_the_wood_no_longer_keeps_fall_by_degree() {
    let (structure, whole, species) = grown(1.5);
    let rows = pa_rows(&species);
    let quarter = unfolded(&structure, (&[][..], &rows), Unfolding { leaves: &[], fraction: 0.25 }).0;
    let r = retention(1.5);
    let age = structure.age;
    let node: HashMap<u64, usize> =
        (1..whole.nodes.len()).map(|i| (whole.nodes[i].identity.birth_order(), i)).collect();
    let mut seen = [0usize; 3];
    for axis in &structure.axes {
        for p in &axis.phytomers {
            let Some(&i) = node.get(&p.key()) else { continue };
            let now = kept_share(r, age - p.cycle);
            let was = if p.cycle < age { kept_share(r, age - 1 - p.cycle) } else { now };
            let l = quarter.leafing[i];
            assert_eq!((l.keep as u16, l.was as u16, l.fade), (now, was, 0.75), "node {i}");
            assert_eq!((whole.leafing[i].was, whole.leafing[i].fade), (whole.leafing[i].keep, 1.0), "node {i} whole");
            seen[(age - p.cycle).min(2) as usize] += 1;
        }
    }
    assert!(seen.iter().all(|&n| n > 100), "{seen:?}");
    // A station kept a year before and not now is drawn at the share left.
    let l = Leafing { older: 0.0, keep: 0, was: u32::from(KEEP), fade: 0.75 };
    assert_eq!(l.share((0.0, 0.0), 7, 3), Some(0.75));
    assert_eq!(Leafing { was: 0, ..l }.share((0.0, 0.0), 7, 3), None);
}
