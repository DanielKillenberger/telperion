//! fn-241 R1: the fine path's tree stands where `mesh::build`'s does. Its
//! box was every wood cluster's sphere, which hung the trunk's radius
//! below the ground (2.6 m on the oak at 80); it is now each ring's own
//! extent, on the CPU curve (`Curve::bounds`) and the device's handoff
//! (`handoff.wgsl`), the drawn wood to the vertex (R1.md).
use super::super::{Delivery, Generator};
use super::resident::{build, Tools};
use super::roots::kernels;
use crate::{hero_pose, render, write_png, Renderer, GROUND_REACH};
use telperion_core::{presets::Family, surface::Bounds};

/// fn-125's screen: metres on a position, and radians on a leaf's
/// direction, which moves its far corner by that times its reach.
const MOST: f64 = 50e-6;
const TURN: f64 = 0.005;
const SHIPPED: [&str; 4] = [
    "european-beech",
    "oregon-white-oak",
    "norway-spruce",
    "date-palm",
];

fn family(id: &str, age: f64) -> Family {
    let mut family = telperion_core::presets::by_identity(id).unwrap();
    family.age = age;
    family
}

fn corners(b: Bounds) -> [f64; 6] {
    [b.min.x, b.min.y, b.min.z, b.max.x, b.max.y, b.max.z]
}

/// The farthest a leaf's vertex stands from its origin, in metres.
fn leaf_reach(mesh: &telperion_core::mesh::TreeMesh) -> f64 {
    let element = (mesh.foliage.element.positions.iter())
        .map(|p| p.length())
        .fold(0.0, f64::max);
    let scale = (mesh.foliage.instances.matrices())
        .map(|m| {
            (0..3)
                .map(|c| f64::from(m[4 * c..4 * c + 3].iter().map(|v| v * v).sum::<f32>()).sqrt())
                .fold(0.0, f64::max)
        })
        .fold(0.0, f64::max);
    element * scale
}

#[test]
fn the_fine_path_stands_where_mesh_build_does() {
    let Some(gpu) = crate::generation::test_gpu() else {
        return;
    };
    let renderer = Renderer::new(gpu, crate::STILL_FORMAT);
    let gen = Generator::new(&renderer).unwrap();
    for id in SHIPPED {
        for age in [40.0, 80.0] {
            let family = family(id, age);
            let mesh = telperion_core::mesh::build(&family).unwrap();
            let built = corners(mesh.bounds);
            let screen = MOST + TURN * leaf_reach(&mesh);
            drop(mesh);
            let prepared = gen.prepare(&family, Delivery::Resident).unwrap();
            let fine = corners(prepared.bounds());
            let off = (0..6)
                .map(|k| (built[k] - fine[k]).abs())
                .fold(0.0, f64::max);
            println!(
                "{id} {age} {:?}: off {off:.2e} m, ground {:.2e} m, screen {screen:.2e} m",
                prepared.backend,
                (built[1] - fine[1]).abs()
            );
            // The ground is the wood's: the trunk's foot, held to the
            // position screen.
            assert!(
                (built[1] - fine[1]).abs() <= MOST,
                "{id} at {age}: stands at {} against mesh::build's {}",
                fine[1],
                built[1]
            );
            assert!(
                off <= screen,
                "{id} at {age}: the prepared box {fine:?} against mesh::build's {built:?}"
            );
            // Each tree's buffers freed before the next is made: the
            // spruce at 80 does not fit beside another's.
            drop(prepared);
            renderer
                .gpu()
                .device
                .poll(wgpu::PollType::wait_indefinitely())
                .unwrap();
        }
    }
}

/// The hero still of each fine-path species at 40 and 80, framed on the
/// prepared box, into `STILLS`; and the CPU curve's and the handoff's
/// milliseconds over `RUNS` builds (`cargo test --profile ci -p
/// telperion-render --lib ground_stills -- --ignored --nocapture`).
#[test]
#[ignore = "stills and cost: run by hand on both sides of the box"]
fn ground_stills() {
    let Some(gpu) = crate::generation::test_gpu() else {
        return;
    };
    let dir = std::path::PathBuf::from(std::env::var("STILLS").expect("STILLS=<dir>"));
    std::fs::create_dir_all(&dir).unwrap();
    let runs: usize = std::env::var("RUNS").map_or(3, |r| r.parse().unwrap());
    let k = kernels(&gpu);
    let tools = Tools::new(&gpu);
    for id in &SHIPPED[..3] {
        for age in [40.0, 80.0] {
            let family = family(id, age);
            let mut times = Vec::new();
            for run in 0..runs {
                let mut renderer = Renderer::new(gpu.clone(), crate::STILL_FORMAT);
                let gen = Generator::new(&renderer).unwrap();
                let built = pollster::block_on(build(&gpu, &gen, &k, &tools, &family)).unwrap();
                times.push((built.timed.cpu, built.timed.handoff));
                if run > 0 {
                    continue;
                }
                let b = built.prepared.bounds();
                renderer.submit_prepared(built.prepared).unwrap();
                renderer.set_family(&family);
                let camera = hero_pose(b, 16.0 / 9.0, GROUND_REACH);
                let still = render(&mut renderer, &camera, 960, 540).unwrap();
                write_png(&dir.join(format!("{id}-{age}.png")), &still).unwrap();
                println!("{id} {age}: box {:?}", corners(b));
            }
            println!("{id} {age}: (cpu ms, handoff ms) {times:?}");
        }
    }
}
