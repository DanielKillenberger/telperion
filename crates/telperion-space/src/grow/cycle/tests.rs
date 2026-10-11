//! fn-255 R1's checks on the one cycle (DESIGN.md section 1).
use super::*;
use crate::light::Light;

fn request(age: u32, seed: u64, extinction: f64) -> Request {
    Request {
        age,
        seed,
        budget: 20_000_000,
        light: Light {
            extinction,
            sky: 0.5,
        },
    }
}

/// A species that reads no light grows through the one cycle, every
/// phase run, to today's tree: each zero is a neutral term.
#[test]
fn a_species_that_reads_no_light_grows_todays_tree() {
    for species in [crate::beech(), crate::spruce()] {
        let r = request(20, 1, 0.0);
        let today = crate::grow::whole(&species, r).unwrap();
        let cycled = grow_cycle(&species, r, Model::Lattice, &[]).unwrap();
        assert!(cycled.structure == today, "the one cycle's tree is today's");
    }
}

/// The oak through the one cycle under the lattice is today's build, to
/// the tolerance the stepped executor holds against the whole tree.
#[test]
fn the_lit_oak_through_the_cycle_is_todays_build() {
    let species = crate::oak();
    let r = request(24, 7, 0.5);
    let today = crate::grow::grow(&species, r).unwrap();
    let cycled = grow_cycle(&species, r, Model::Lattice, &[]).unwrap();
    let agree = crate::agree::agreement(&today, &cycled.structure);
    assert!(agree.within(&crate::agree::PATHS), "{agree:?}");
}

/// At zero and arbitrarily near zero the same code runs: the tree at no
/// extinction is the tree at 1e-30, and with no response to shade the
/// tree with 1e-30 of it, under either model.
#[test]
fn zero_and_near_zero_grow_the_same_tree() {
    let species = crate::oak();
    for model in [Model::Lattice, Model::Volumes(Candidate::default())] {
        let tree = |s: &Species, k: f64| grow_cycle(s, request(16, 3, k), model, &[]).unwrap().structure;
        assert!(tree(&species, 0.0) == tree(&species, 1e-30), "{model:?}: extinction");
        let response = |v: f64| {
            let mut s = species.clone();
            for state in &mut s.states {
                (state.shade_hazard, state.shade_size, state.balance_hazard) = (v, v, v);
            }
            s
        };
        assert!(tree(&response(0.0), 0.5) == tree(&response(1e-30), 0.5), "{model:?}: response");
    }
}
