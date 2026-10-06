//! Host decision 3: a fine root whose extent bound from its attachment can
//! reach the ground's landing zone is grown on the CPU reference executor.
//! Every other root's wood stays out of that zone, so the ground never
//! acts on what the GPU grows.
use super::{at, request};
use crate::bounded::Expected;
use crate::geometry::ground_zone;
use crate::species::Species;
use crate::{beech, spruce};

/// The roots routed to the CPU, and the roots whose wood came into the
/// landing zone though they were not.
fn routed(species: &Species, age: u32, seed: u64) -> (usize, usize, usize) {
    let expected = Expected::new(species, age, 0);
    let fine = at(species, request(age, seed), &expected, 1024.0);
    let mut grounded = 0;
    let mut near = 0;
    for (k, (root, ..)) in fine.roots.iter().enumerate() {
        if root.grounded {
            grounded += 1;
            continue;
        }
        let axes = &fine.structure.axes[fine.starts[k]..fine.starts[k + 1]];
        let reached = axes.iter().any(|axis| {
            let mut base = axis.base;
            axis.phytomers.iter().any(|p| {
                let length = (p.tip - base).length();
                let low = base.z.min(p.tip.z);
                base = p.tip;
                low < ground_zone(p.radius, length)
            })
        });
        near += usize::from(reached);
    }
    (fine.roots.len(), grounded, near)
}

#[test]
fn a_root_that_can_reach_the_ground_is_grown_on_the_cpu() {
    for (name, species) in [("beech", beech()), ("spruce", spruce())] {
        for seed in [1, 7] {
            let (roots, grounded, near) = routed(&species, 40, seed);
            println!("{name} 40 {seed}: {grounded} of {roots} roots on the CPU");
            assert_eq!(
                near, 0,
                "{name} 40 {seed}: a root the GPU grows reached the ground"
            );
            assert!(
                grounded < roots,
                "{name} 40 {seed}: some roots stay on the GPU"
            );
        }
    }
}
