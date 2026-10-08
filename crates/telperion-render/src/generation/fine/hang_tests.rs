//! fn-235 R1: the browser's build route run natively, under the page's
//! device limits (TASK10.md: `maxBufferSize` 4,294,967,292,
//! `maxStorageBufferBindingSize` 2,147,483,644; every other limit
//! WebGPU's default), on one generator and renderer as the page holds
//! them. One tree a process, under a wall-clock `timeout` that kills it,
//! with `TELPERION_TRACE` set so the last step logged is the one that
//! never completed: `TREE=silver-birch:20 TELPERION_TRACE=1 timeout 120
//! cargo test --profile ci -p telperion-render browser_route -- --ignored`.
use super::super::{Delivery, Generator};
use crate::{Gpu, Renderer};

/// The page's device: its two raised limits, WebGPU's defaults beside.
pub(super) fn browser_gpu() -> Option<Gpu> {
    let limits = |available: &wgpu::Limits| wgpu::Limits {
        max_buffer_size: available.max_buffer_size.min(4_294_967_292),
        max_storage_buffer_binding_size: available
            .max_storage_buffer_binding_size
            .min(2_147_483_644),
        ..wgpu::Limits::default()
    };
    match pollster::block_on(Gpu::request_within(None, limits)) {
        Ok(gpu) => Some(gpu),
        Err(error) => {
            println!("skipped: {error}");
            None
        }
    }
}

#[test]
#[ignore = "a native repro, one tree a process"]
fn browser_route() {
    let tree = std::env::var("TREE").expect("TREE=<species>:<age>");
    let (id, age) = tree.split_once(':').expect("TREE=<species>:<age>");
    let Some(gpu) = browser_gpu() else {
        return;
    };
    let mut family = telperion_core::presets::by_identity(id).unwrap();
    family.age = age.parse().unwrap();
    let mut renderer = Renderer::new(gpu, crate::STILL_FORMAT);
    let gen = pollster::block_on(Generator::new_async(&renderer)).unwrap();
    let t = std::time::Instant::now();
    let prepared = gen.prepare(&family, Delivery::Resident).unwrap();
    let built = t.elapsed().as_secs_f64();
    renderer.submit_prepared(prepared).unwrap();
    let camera = crate::hero_pose(renderer.bounds().unwrap(), 4.0 / 3.0, crate::GROUND_REACH);
    crate::render(&mut renderer, &camera, 320, 240).unwrap();
    println!("{tree}: built in {built:.2} s and drawn");
}
