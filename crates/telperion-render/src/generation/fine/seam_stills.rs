//! fn-212 task 7, D4's seams at a close camera (`cargo test --profile ci
//! -p telperion-render seam_stills -- --ignored --nocapture`,
//! `PROBE=norway-spruce:80:1[:lobed]`, `STILLS=<dir>`, under the GPU lock): the
//! tree built on the real path and submitted, then the CPU path's, each
//! drawn bare and whole from the same cameras at the seams of the two
//! fine roots the device draws on the thickest bearers in the middle of
//! the crown.
use super::super::{Delivery, Generator};
use super::resident::{build, Tools};
use super::roots::kernels;
use crate::{Camera, Gpu, Renderer, View};
use std::path::PathBuf;
use telperion_core::math::Vec3;
use telperion_core::presets::Family;
use telperion_core::tree::Tree;

const SIZE: (u32, u32) = (1024, 768);

/// Cameras at the seams of two device-drawn fine roots borne on limbs of
/// 3 to 15 mm between 0.4 and 0.7 of the height, the outermost first and
/// the second a metre or more from it, each looking in from outside the
/// crown, 0.3 m off.
pub(super) fn cameras(tree: &Tree) -> Vec<(u32, Camera)> {
    let height = tree.nodes.iter().map(|n| n.position.y).fold(0.0, f64::max);
    let reach = |n: u32| {
        let p = tree.nodes[n as usize].position;
        p.x.hypot(p.z)
    };
    let mut at: Vec<u32> = (tree.attachments.iter())
        .filter(|a| a.seams.is_empty())
        .map(|a| a.node)
        .filter(|&n| {
            let node = &tree.nodes[n as usize];
            (0.4..0.7).contains(&(node.position.y / height)) && (3e-3..15e-3).contains(&node.radius)
        })
        .collect();
    at.sort_by(|&a, &b| reach(b).total_cmp(&reach(a)).then(a.cmp(&b)));
    let mut picked: Vec<u32> = Vec::new();
    for n in at {
        let p = tree.nodes[n as usize].position;
        if picked
            .iter()
            .all(|&m| tree.nodes[m as usize].position.distance(p) > 1.0)
        {
            picked.push(n);
        }
        if picked.len() == 2 {
            break;
        }
    }
    (picked.into_iter())
        .map(|n| {
            let node = &tree.nodes[n as usize];
            let out = Vec3::new(node.position.x, 0.0, node.position.z).normalized();
            let away = 0.3;
            let camera = Camera {
                position: node.position + (out + Vec3::new(0.0, 0.4, 0.0)).normalized() * away,
                target: node.position,
                field_of_view: 35.0,
                near: 0.01,
                far: 60.0,
            };
            (n, camera)
        })
        .collect()
}

fn shoot(renderer: &mut Renderer, cameras: &[(u32, Camera)], dir: &PathBuf, side: &str) {
    for (k, (_, camera)) in cameras.iter().enumerate() {
        for (view, name) in [(View::Bare, "bare"), (View::Whole, "whole")] {
            renderer.set_view(view);
            let still = crate::render(renderer, camera, SIZE.0, SIZE.1).unwrap();
            let path = dir.join(format!("seam{k}-{name}-{side}.png"));
            crate::write_png(&path, &still).unwrap();
            println!("{}: {} triangles", path.display(), still.stats.triangles);
        }
    }
}

fn family(only: &str) -> Family {
    let parts: Vec<&str> = only.split(':').collect();
    let mut family = telperion_core::presets::by_identity(parts[0]).unwrap();
    family.age = parts[1].parse().unwrap();
    family.skeleton.seed = parts[2].parse().unwrap();
    // `:lobed`: the wood two lobes a tenth deep (fn-212 task 11).
    if parts.get(3) == Some(&"lobed") {
        (family.surface.lobes, family.surface.lobe_depth) = (2, 0.1);
    }
    family
}

#[test]
#[ignore]
fn seam_stills() {
    let Some(gpu) = crate::generation::test_gpu() else {
        return;
    };
    let only = std::env::var("PROBE").unwrap_or_else(|_| "norway-spruce:80:1".into());
    let dir = PathBuf::from(std::env::var("STILLS").expect("STILLS: where the stills go"));
    std::fs::create_dir_all(&dir).unwrap();
    let family = family(&only);
    let cameras = device_side(&gpu, &family, &dir);
    let mut renderer = Renderer::new(gpu.clone(), crate::STILL_FORMAT);
    let gen = Generator::new(&renderer).unwrap();
    let prepared = pollster::block_on(gen.prepare_whole(&family, Delivery::Resident)).unwrap();
    renderer.submit_prepared(prepared).unwrap();
    shoot(&mut renderer, &cameras, &dir, "cpu");
}

/// The real path's stills, and the cameras they were taken from.
fn device_side(gpu: &Gpu, family: &Family, dir: &PathBuf) -> Vec<(u32, Camera)> {
    let mut renderer = Renderer::new(gpu.clone(), crate::STILL_FORMAT);
    let gen = Generator::new(&renderer).unwrap();
    let k = kernels(gpu);
    let built = pollster::block_on(build(gpu, &gen, &k, &Tools::new(gpu), family)).unwrap();
    let cameras = cameras(built.expansion.tree());
    for (n, c) in &cameras {
        let p = built.expansion.tree().nodes[*n as usize].position;
        println!("seam at node {n} {p:?}, camera {:?}", c.position);
    }
    renderer.submit_prepared(built.prepared).unwrap();
    shoot(&mut renderer, &cameras, dir, "device");
    cameras
}
