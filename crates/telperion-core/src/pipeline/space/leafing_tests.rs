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

/// fn-270 R5a: drawn within its year, a node's wood keeps the leaves it
/// kept a year before moved a share of the way to what it keeps now, so
/// last year's leaves fall through the year, not at its turn.
#[test]
fn within_its_year_the_wood_keeps_a_share_of_the_way_to_this_years_leaves() {
    let (structure, whole, species) = grown(1.5);
    let rows = pa_rows(&species);
    let at = |fraction| unfolded(&structure, (&[][..], &rows), Unfolding { leaves: &[], fraction }).0;
    let (start, quarter) = (at(0.0), at(0.25));
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
            let mixed = (f64::from(was) + (f64::from(now) - f64::from(was)) * 0.25).round() as u16;
            assert_eq!(start.leafing[i].keep as u16, was, "node {i} at the year's turn");
            assert_eq!(quarter.leafing[i].keep as u16, mixed, "node {i} a quarter in");
            assert_eq!(whole.leafing[i].keep as u16, now, "node {i} whole");
            seen[(age - p.cycle).min(2) as usize] += 1;
        }
    }
    assert!(seen.iter().all(|&n| n > 100), "{seen:?}");
}
