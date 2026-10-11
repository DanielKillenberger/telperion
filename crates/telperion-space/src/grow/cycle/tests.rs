//! fn-255's checks on the one cycle (DESIGN.md section 1, CYCLE-COST.md).
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

/// The one cycle with every axis open and none retired: the reference
/// its working set and its retirement are held to.
fn reference(species: &Species, r: Request) -> Structure {
    run(species, r, false, Working::reference()).unwrap().structure
}

/// The working set's frozen subtrees and the subtrees retired as the tree
/// grows leave every sum, and so the tree, the reference's to the bit,
/// where many are retired: the oak (which reads light and keeps leaf
/// girth, its retired phytomers kept as records), the spruce and the
/// beech.
#[test]
fn the_working_set_and_retirement_leave_the_tree_as_it_was() {
    for (species, k) in [(crate::oak(), 0.5), (crate::spruce(), 0.0), (crate::beech(), 0.0)] {
        let r = request(40, 7, k);
        let cycled = grow_census(&species, r).unwrap();
        let retired = cycled.census.expect("a census").retired;
        assert!(retired > 100, "{retired} retired");
        assert!(cycled.structure == reference(&species, r), "{retired} retired");
    }
}

/// At zero and arbitrarily near zero the same code runs: the tree at no
/// extinction is the tree at 1e-30, and with no response to shade the
/// tree with 1e-30 of it.
#[test]
fn zero_and_near_zero_grow_the_same_tree() {
    let species = crate::oak();
    let tree = |s: &Species, k: f64| grow_cycle(s, request(16, 3, k)).unwrap().structure;
    assert!(tree(&species, 0.0) == tree(&species, 1e-30), "extinction");
    let response = |v: f64| {
        let mut s = species.clone();
        for state in &mut s.states {
            (state.shade_hazard, state.shade_size, state.balance_hazard) = (v, v, v);
        }
        s
    };
    assert!(tree(&response(0.0), 0.5) == tree(&response(1e-30), 0.5), "response");
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

/// The f32 survival the device reads: the table's viability, shaded in
/// f32; whole without shade or in full light.
#[test]
fn survival_reads_the_tables_f32() {
    use crate::contract::decide::shaded;
    assert_eq!(shaded(0.995, 1.0, 1.0), 0.995);
    assert_eq!(shaded(0.995, 0.0, 0.3), 0.995);
    let p = shaded(0.995, 1.0, 0.5);
    assert!(p < 0.995 && (f64::from(p) - 0.995f64.powf(2.0)).abs() < 1e-6, "{p}");
}

/// How far the one cycle's tree lies from today's whole-tree engine, by
/// lineage, under the spike's contracts (f32 viability, the one reduction
/// order, the integer deposit): printed for the evidence, not bounded.
#[test]
#[ignore]
fn agreement_with_todays_tree() {
    for (name, species, k) in [("oak", crate::oak(), 0.5), ("beech", crate::beech(), 0.0), ("spruce", crate::spruce(), 0.0)] {
        for (age, seed) in [(24, 7), (40, 1)] {
            let r = request(age, seed, k);
            let today = crate::grow::whole(&species, r).unwrap();
            let cycled = grow_cycle(&species, r).unwrap();
            let a = crate::agree::agreement(&today, &cycled.structure);
            eprintln!("AGREE {name} {age} {seed}: {a:?}");
        }
    }
}
