//! fn-256 task 3: each shipped preset's resident build, from its family
//! to draw-ready, timed warm (its second build in the process) at its
//! default age and seed: the same call before and after the one cycle
//! took the harness. `BUILD_PRESETS` names a comma list to time alone.
//! `cargo test --profile ci -p telperion-render build_time -- --ignored
//! --nocapture`.
use super::{Delivery, Generator};
use crate::Renderer;

#[test]
#[ignore = "a measurement"]
fn build_time() {
    let Some(gpu) = crate::generation::test_gpu() else {
        return;
    };
    let renderer = Renderer::new(gpu.clone(), crate::STILL_FORMAT);
    let gen = Generator::new(&renderer).unwrap();
    let only = std::env::var("BUILD_PRESETS").ok();
    let ids = telperion_core::presets::CATALOGUE.iter().map(|c| c.1);
    let ids = ids.filter(|id| only.as_ref().is_none_or(|o| o.split(',').any(|x| x == *id)));
    println!("| Preset | Age | Cold ms | Warm ms | Instances |");
    println!("|---|--:|--:|--:|--:|");
    for id in ids {
        let family = telperion_core::presets::by_identity(id).unwrap();
        let mut ms = Vec::new();
        let mut count = 0;
        for _ in 0..2 {
            let t = super::Clock::now();
            let prepared = match gen.prepare(&family, Delivery::Resident) {
                Ok(p) => p,
                Err(e) => {
                    println!("| {id} | {} | refused: {e} | | |", family.age);
                    break;
                }
            };
            ms.push(t.elapsed_ms());
            count = prepared.count();
            drop(prepared);
            gpu.device.poll(wgpu::PollType::wait_indefinitely()).unwrap();
        }
        if ms.len() < 2 {
            continue;
        }
        println!("| {id} | {} | {:.0} | {:.0} | {count} |", family.age, ms[0], ms[1]);
    }
}
