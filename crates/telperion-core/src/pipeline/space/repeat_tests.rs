//! fn-254 R4 checks 1 and 3 on every shipped preset: a tree is the same
//! tree, to the bit, built again and on one core or four, and it keeps
//! its invariants (`telperion_space::invariants`).
use super::*;
use crate::presets::{by_identity, CATALOGUE};

fn grown(family: &crate::Family) -> telperion_space::Structure {
    let species = family.space.species.clone().unwrap();
    let request = Request {
        age: family.age.round() as u32,
        seed: u64::from(family.skeleton.seed),
        budget: BUDGET,
        light: family.space.light,
    };
    telperion_space::grow_marked(&species, request).unwrap().0
}

fn on(threads: usize, family: &crate::Family) -> telperion_space::Structure {
    let pool = rayon::ThreadPoolBuilder::new().num_threads(threads).build().unwrap();
    pool.install(|| grown(family))
}

#[test]
fn every_preset_grows_the_same_tree_again_and_on_any_cores() {
    for &(_, id, _, _) in CATALOGUE {
        let family = by_identity(id).unwrap();
        let first = on(4, &family);
        assert!(first == on(4, &family), "{id}: built again");
        assert!(first == on(1, &family), "{id}: on one core");
        let broken = telperion_space::invariants::invariants(&first);
        assert!(broken.is_empty(), "{id}: {broken:#?}");
        println!("{id}: {} axes, the same on four cores, one and again", first.axes.len());
    }
}
