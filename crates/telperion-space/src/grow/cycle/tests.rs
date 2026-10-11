//! fn-255 R1's checks on the one cycle (DESIGN.md section 1), and the
//! quantised deposit's (COUNCIL.md's early test 2).
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
        let cycled = grow_cycle(&species, r, Deposit::F32).unwrap();
        assert!(cycled.structure == today, "the one cycle's tree is today's");
    }
}

/// The oak through the one cycle is today's build, to the tolerance the
/// stepped executor holds against the whole tree.
#[test]
fn the_lit_oak_through_the_cycle_is_todays_build() {
    let species = crate::oak();
    let r = request(24, 7, 0.5);
    let today = crate::grow::grow(&species, r).unwrap();
    let cycled = grow_cycle(&species, r, Deposit::F32).unwrap();
    let agree = crate::agree::agreement(&today, &cycled.structure);
    assert!(agree.within(&crate::agree::PATHS), "{agree:?}");
}

/// At zero and arbitrarily near zero the same code runs: the tree at no
/// extinction is the tree at 1e-30, and with no response to shade the
/// tree with 1e-30 of it, on either deposit.
#[test]
fn zero_and_near_zero_grow_the_same_tree() {
    let species = crate::oak();
    for deposit in [Deposit::F32, Deposit::Quantised] {
        let tree = |s: &Species, k: f64| grow_cycle(s, request(16, 3, k), deposit).unwrap().structure;
        assert!(tree(&species, 0.0) == tree(&species, 1e-30), "{deposit:?}: extinction");
        let response = |v: f64| {
            let mut s = species.clone();
            for state in &mut s.states {
                (state.shade_hazard, state.shade_size, state.balance_hazard) = (v, v, v);
            }
            s
        };
        assert!(tree(&response(0.0), 0.5) == tree(&response(1e-30), 0.5), "{deposit:?}: response");
    }
}

/// The quantised deposit is the same lattice in any order of the leaves,
/// and a node past its u32 of quanta is refused by name.
#[test]
fn the_quantised_deposit_is_order_free_and_refuses_overflow() {
    let light = Light { extinction: 0.5, sky: 0.5 };
    let leaves: Vec<(Vec3, f64)> = (0..500)
        .map(|k| {
            let t = f64::from(k);
            (Vec3::new((t * 0.37).sin() * 3.0, (t * 0.71).cos() * 3.0, 2.0 + (t * 0.13).sin()), 0.0018 * (1.0 + (t * 0.3).cos()))
        })
        .collect();
    let reads: Vec<Vec3> = leaves.iter().map(|l| l.0 - Vec3::new(0.0, 0.0, 0.3)).collect();
    let forward = Field::quantised(&light, &leaves, &reads, 1).unwrap();
    let mut reversed = leaves.clone();
    reversed.reverse();
    let back = Field::quantised(&light, &reversed, &reads, 1).unwrap();
    for &r in &reads {
        assert_eq!(forward.at(r).to_bits(), back.at(r).to_bits());
    }
    let heavy = vec![(Vec3::new(0.0, 0.0, 1.0), 3000.0); 2];
    let refused = Field::quantised(&light, &heavy, &[], 1).err().expect("refused");
    assert!(format!("{refused}").starts_with("light.deposit"), "{refused}");
}
