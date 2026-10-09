//! fn-241 R1: how far below ground each path puts a shipped species, at
//! 40 and 80: `mesh::build`'s bounds, its wood surface's, the curve's own
//! rings (the drawn wood, to the vertex), the curve's box
//! (`Curve::bounds`), and `prepare`'s bounds on both deliveries. Measure
//! only: `cargo test --profile ci -p telperion-render --lib ground_measure
//! -- --ignored --nocapture`.
use super::super::{Delivery, Generator};
use crate::Renderer;

#[test]
#[ignore = "measure: every shipped species at 40 and 80 on both paths"]
fn ground_measure() {
    let Some(gpu) = crate::generation::test_gpu() else {
        return;
    };
    let renderer = Renderer::new(gpu, crate::STILL_FORMAT);
    let gen = Generator::new(&renderer).unwrap();
    for id in ["european-beech", "oregon-white-oak", "norway-spruce", "date-palm"] {
        for age in [40.0, 80.0] {
            let mut family = telperion_core::presets::by_identity(id).unwrap();
            family.age = age;
            let mesh = telperion_core::mesh::build(&family).unwrap();
            let wood = mesh.wood.bounds.map_or(f64::NAN, |b| b.min.y);
            let curve = mesh.curve.bounds().unwrap().min.y;
            let (rings, _) = mesh.curve.rings(16).unwrap();
            let rings = rings.chunks(3).map(|v| v[1]).fold(f32::MAX, f32::min);
            let built = mesh.bounds.min.y;
            drop(mesh);
            let resident = gen.prepare(&family, Delivery::Resident).unwrap();
            let (on_device, backend) = (resident.bounds().min.y, resident.backend);
            drop(resident);
            let on_cpu = gen.prepare(&family, Delivery::Cpu).unwrap().bounds().min.y;
            println!(
                "{id} {age}: build {built:.4} wood {wood:.4} rings {rings:.4} curve {curve:.4} \
                 | prepare resident {on_device:.4} ({backend:?}) cpu {on_cpu:.4}"
            );
        }
    }
}
