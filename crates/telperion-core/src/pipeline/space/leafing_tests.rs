//! fn-248 R1: each drawn node records its wood's leaf retention by age,
//! the phytomers merged into it included; a tree whose ages bear for ever
//! records none.
use super::*;
use crate::presets::Preset;
use crate::tree::{kept_share, retention, Unfolded, KEEP};
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

/// fn-270 R5a and R5c: drawn within its year, a node keeps the
/// retentions its wood has now and had a year before, and its leaves'
/// season: on last year's wood, under a lifespan of half a year, every
/// leaf passes, each falling whole on its own day over the fall and growing again from
/// budburst; this year's wood passes none.
#[test]
fn within_its_year_old_wood_drops_its_leaves_and_grows_them_again() {
    let (structure, whole, mut species) = grown(1.5);
    for s in &mut species.states {
        (
            s.leaf_life,
            s.budburst,
            s.senescence,
            s.colouring,
            s.leaf_fall,
        ) = (0.5, 115.0, 264.0, 16.0, 45.0);
    }
    let rows = pa_rows(&species);
    let season = rows[2].season;
    let age = structure.age;
    let r = retention(1.5);
    let node: HashMap<u64, usize> = (1..whole.nodes.len())
        .map(|i| (whole.nodes[i].identity.birth_order(), i))
        .collect();
    for f in [0.25, 0.6, 0.95] {
        let at = unfolded(
            &structure,
            (&[][..], &rows),
            Unfolding {
                leaves: &[],
                fraction: f,
                gone: usize::MAX,
            },
        )
        .0;
        let tau = f * telperion_space::unfold::YEAR;
        let mut seen = [0usize; 2];
        for axis in &structure.axes {
            for p in &axis.phytomers {
                let Some(&i) = node.get(&p.key()) else {
                    continue;
                };
                let now = kept_share(r, age - p.cycle);
                let was = if p.cycle < age {
                    kept_share(r, age - 1 - p.cycle)
                } else {
                    now
                };
                let (l, u) = (at.leafing[i], at.unfolding[i]);
                assert_eq!((l.keep as u16, l.was as u16), (now, was), "node {i}");
                let old = p.cycle < age;
                assert_eq!(u.life, if old { u16::MAX } else { 0 }, "node {i}");
                assert_eq!(
                    (u.fall, u.regrow),
                    (season.fallen(tau) as f32, season.regrow(tau) as f32)
                );
                seen[usize::from(old)] += 1;
            }
        }
        assert!(seen.iter().all(|&n| n > 100), "{seen:?}");
    }
    assert!(whole.unfolding.is_empty(), "the whole year records none");
    // A station that passes stays whole until its abscission day and is
    // gone from then, kept growing again; one that does not stands at
    // this year's share; one kept no longer only falls.
    let u = Unfolded {
        grow: 0.5,
        fall: 0.0,
        colour: 0.75,
        regrow: 0.125,
        life: u16::MAX,
    };
    assert_eq!(u.leaf(true, 7, 3), (1.125, 0.75));
    assert_eq!(u.leaf(false, 7, 3), (1.0, 0.75));
    assert_eq!(Unfolded { fall: 1.0, ..u }.leaf(true, 7, 3), (0.125, 0.75));
    assert_eq!(Unfolded { fall: 1.0, ..u }.leaf(false, 7, 3), (0.0, 0.75));
    assert_eq!(Unfolded { life: 0, ..u }.leaf(true, 7, 3), (0.5, 0.0));
    assert_eq!(Unfolded { life: 0, ..u }.leaf(false, 7, 3), (1.0, 0.75));
    // Part way through the fall that share of the leaves is gone, each
    // whole until it goes.
    let falling = Unfolded { fall: 0.25, ..u };
    let shares: Vec<f64> = (0..400).map(|k| falling.leaf(false, 7, k).0).collect();
    assert!(shares.iter().all(|&s| s == 0.0 || s == 1.0));
    let gone = shares.iter().filter(|&&s| s == 0.0).count();
    assert!(
        (70..=130).contains(&gone),
        "{gone} of 400 gone at a quarter of the fall"
    );
    let l = Leafing {
        older: 0.0,
        keep: 0,
        was: u32::from(KEEP),
    };
    assert_eq!(l.kept((0.0, 0.0), 7, 3), Some(false));
    assert_eq!(Leafing { was: 0, ..l }.kept((0.0, 0.0), 7, 3), None);
}
