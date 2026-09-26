//! A clump that parts above the ground keeps its stems in every read. The
//! shoot-less wood, the sparse interval's selected wood and the change records
//! all tell a stem from a limb by the flag the node carries, so each sweeps
//! the fork the way the full read does.
use super::tests::buffers;
use super::*;
use crate::presets::{Family, Preset};
use std::collections::BTreeMap;

/// Two stems on the oak's table, one upright and one leaning out, parting at
/// 0.4 of the bole, grown as a specimen to `age` years.
fn clump(age: f64) -> Family {
    let mut f = Preset::OregonWhiteOak.parameters();
    f.skeleton.seed = 7;
    f.skeleton.habit.stems = 2;
    f.skeleton.habit.stem_lean = 24.0;
    f.skeleton.habit.stem_lean_spread = 1.0;
    f.skeleton.habit.stem_fork_height = 0.4;
    f.growth.leaf_lifetime = 6.0;
    f.age = age;
    f
}

fn stems(tree: &Tree) -> BTreeMap<NodeIdentity, bool> {
    tree.nodes.iter().map(|n| (n.identity, n.stem)).collect()
}

/// Whether two stems leave any node above the root.
fn parted(tree: &Tree) -> bool {
    let mut count = vec![0; tree.nodes.len()];
    for n in tree.nodes.iter().filter(|n| n.stem) {
        count[n.parent.unwrap() as usize] += 1;
    }
    count.iter().skip(1).any(|&c| c > 1)
}

#[test]
fn every_read_of_a_clump_keeps_its_stems() {
    let f = clump(12.25);
    let s = Specimen::build(&f).unwrap();
    // Before the first stem reaches the fork, one stem; after it, two.
    let ages = [4.25, 8.0, 12.25];
    let mut forked = Vec::new();
    for &years in &ages {
        let age = Age::from_years(years).unwrap();
        let (full, _) = s.wood_at(age, true).unwrap();
        let (bare, _) = s.wood_at(age, false).unwrap();
        forked.push(parted(&full));
        assert_eq!(stems(&bare), stems(&full), "at {years}");
        let height = s.surface_height();
        assert_eq!(
            crate::pipeline::surface::build(&bare, height, &f.surface).unwrap(),
            crate::pipeline::surface::build(&full, height, &f.surface).unwrap(),
            "at {years} the shoot-less read sweeps another fork"
        );
        let every = full.nodes.iter().map(|n| n.identity).collect();
        assert_eq!(stems(&s.selected_wood(age, &every).unwrap()), stems(&full));
    }
    assert!(
        forked[2],
        "the clump never parted above the ground: {forked:?}"
    );
    // Every record between them reconciles with a fresh read, runs and
    // placements both, and so does a year's advance on the sparse interval.
    let reads: Vec<_> = ages.iter().map(|&a| buffers(&s, a)).collect();
    for (i, &from) in ages.iter().enumerate() {
        for (j, &to) in ages.iter().enumerate() {
            let record = s.changes_between(from, to).unwrap();
            record.validate(&reads[i], &reads[j]).unwrap();
        }
    }
    let mut s = s;
    let record = s.advance(1.0).unwrap();
    record
        .validate(&buffers(&s, 12.25), &buffers(&s, 13.25))
        .unwrap();
}
