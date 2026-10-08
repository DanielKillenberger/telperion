//! fn-229, the device's florets at a close camera (`cargo test --profile
//! ci -p telperion-render floret_stills -- --ignored --nocapture`,
//! `PROBE=european-beech:40:7`, `STILLS=<dir>`, under the GPU lock): the
//! flowering tree delivered resident on the real path, its fine roots'
//! florets placed on the device, then grown whole on the CPU and delivered
//! resident, each drawn from the same cameras at two fine roots' seams (`seam_stills::cameras`).
use super::super::{Delivery, Generator};
use super::florets_tests::flowering;
use super::resident::{build, Tools};
use super::roots::kernels;
use super::seam_stills::cameras;
use crate::{Camera, Renderer, View};
use std::path::PathBuf;
use telperion_core::pipeline::executor;

const SIZE: (u32, u32) = (1024, 768);

fn shoot(renderer: &mut Renderer, cameras: &[Camera], dir: &PathBuf, side: &str) {
    renderer.set_view(View::Whole);
    for (k, camera) in cameras.iter().enumerate() {
        let still = crate::render(renderer, camera, SIZE.0, SIZE.1).unwrap();
        let path = dir.join(format!("florets{k}-{side}.png"));
        crate::write_png(&path, &still).unwrap();
        println!("{}: {} triangles", path.display(), still.stats.triangles);
    }
}

#[test]
#[ignore]
fn floret_stills() {
    let Some(gpu) = crate::generation::test_gpu() else {
        return;
    };
    let only = std::env::var("PROBE").unwrap_or_else(|_| "european-beech:40:7".into());
    let dir = PathBuf::from(std::env::var("STILLS").expect("STILLS: where the stills go"));
    std::fs::create_dir_all(&dir).unwrap();
    let parts: Vec<&str> = only.split(':').collect();
    let family = flowering(
        parts[0],
        (parts[1].parse().unwrap(), parts[2].parse().unwrap()),
    );
    assert!(
        executor::apart(&family).is_ok(),
        "a flowering tree comes apart"
    );
    let mut renderer = Renderer::new(gpu.clone(), crate::STILL_FORMAT);
    let gen = Generator::new(&renderer).unwrap();
    let k = kernels(&gpu);
    let built = pollster::block_on(build(&gpu, &gen, &k, &Tools::new(&gpu), &family)).unwrap();
    // Cameras at two device-drawn roots' seams, pulled back to take in
    // their catkins.
    let cameras: Vec<Camera> = (cameras(built.expansion.tree()).into_iter())
        .map(|(_, c)| Camera {
            position: c.target + (c.position - c.target) * 2.5,
            ..c
        })
        .collect();
    let florets = |p: &super::super::Prepared| {
        let held = p.mesh.organ.as_ref().map_or(0, |o| o.instances.len());
        held + p.organ.as_ref().map_or(0, |o| o.count as usize)
    };
    println!("device: {} florets", florets(&built.prepared));
    renderer.submit_prepared(built.prepared).unwrap();
    shoot(&mut renderer, &cameras, &dir, "device");
    drop(renderer);
    let mut renderer = Renderer::new(gpu.clone(), crate::STILL_FORMAT);
    let gen = Generator::new(&renderer).unwrap();
    let prepared = pollster::block_on(gen.prepare_whole(&family, Delivery::Resident)).unwrap();
    println!("cpu: {} florets", florets(&prepared));
    renderer.submit_prepared(prepared).unwrap();
    shoot(&mut renderer, &cameras, &dir, "cpu");
}
