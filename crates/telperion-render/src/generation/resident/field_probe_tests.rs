//! fn-257's early probe: what a field grown on the device would cost per
//! tree, warm, by stage: the device's growth, settle and dress
//! (`tree::drawn`), the drawn records read back, and the CPU's leaf plan
//! and its field read out to 36 cells a side (the CPU tree standing in for
//! the read-back one, the same size). `PROBE_PRESETS` and
//! `PROBE_SEEDS` narrow it.
//! `cargo test --profile ci -p telperion-render field_probe -- --ignored --nocapture`.
use super::super::cycle::tree::{self, Kernels};
use super::agree_tests::drawn_nodes;
use super::super::Clock;
use telperion_core::pipeline::{self, executor, FieldRequest, Request};

#[test]
#[ignore = "a measurement"]
fn field_probe() {
    let Some(gpu) = crate::generation::test_gpu() else {
        return;
    };
    let k = Kernels::new(&gpu);
    let only = std::env::var("PROBE_PRESETS").ok();
    let seeds: Vec<u32> = std::env::var("PROBE_SEEDS")
        .unwrap_or_else(|_| "1,1234,5678".into())
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();
    let ids = telperion_core::presets::CATALOGUE.iter().map(|c| c.1);
    let ids = ids.filter(|id| only.as_ref().is_none_or(|o| o.split(',').any(|x| x == *id)));
    println!("| Preset | Seed | Age | Drawn ms | Read ms | Read MB | Nodes | CPU grow ms | Clothe ms | Plan ms | Field 36³ ms |");
    println!("|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|");
    for id in ids {
        for &seed in &seeds {
            let mut family = telperion_core::presets::by_identity(id).unwrap();
            family.skeleton.seed = seed;
            let growth = executor::growth(&family).unwrap();
            // Warm: the second of two draws.
            let mut drawn_ms = 0.0;
            let mut dressed = None;
            for _ in 0..2 {
                drop(dressed.take());
                gpu.device.poll(wgpu::PollType::wait_indefinitely()).unwrap();
                let t = Clock::now();
                match pollster::block_on(tree::drawn(&gpu, &k, &growth)) {
                    Ok((d, _)) => dressed = Some(d),
                    Err(e) => {
                        println!("| {id} | {seed} | {} | refused: {e} |", family.age);
                        break;
                    }
                }
                drawn_ms = t.elapsed_ms();
            }
            let Some(dressed) = dressed else { continue };
            let bytes: u64 = dressed.waves.iter().map(|w| w.buffer.size()).sum();
            let t = Clock::now();
            let nodes = drawn_nodes(&gpu, &dressed);
            let read_ms = t.elapsed_ms();
            drop(dressed);
            let request = Request {
                field: Some(FieldRequest {
                    n: 36,
                    limb_order: None,
                }),
                ..Request::default()
            };
            let built = pipeline::build(&family, request).unwrap();
            let s = built.outputs.stages;
            let clothe = s.total_ms - s.skeleton_ms - s.plan_ms - s.field_ms;
            println!(
                "| {id} | {seed} | {} | {drawn_ms:.0} | {read_ms:.0} | {:.1} | {} | {:.0} | {clothe:.0} | {:.0} | {:.0} |",
                family.age,
                bytes as f64 / 1e6,
                nodes.len(),
                s.skeleton_ms,
                s.plan_ms,
                s.field_ms,
            );
        }
    }
}
