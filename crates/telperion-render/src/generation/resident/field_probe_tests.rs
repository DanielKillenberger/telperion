//! fn-257's early probe: what a field grown on the device would cost per
//! tree, warm, by stage: the device's growth, settle and dress
//! (`tree::drawn`), the drawn records read back, and the CPU's leaf plan,
//! field and a whole 36-cell query over the tree's bounds (the CPU tree
//! standing in for the read-back one, the same size). `PROBE_PRESETS` and
//! `PROBE_SEEDS` narrow it.
//! `cargo test --profile ci -p telperion-render field_probe -- --ignored --nocapture`.
use super::super::cycle::tree::{self, Kernels};
use super::agree_tests::drawn_nodes;
use super::super::Clock;
use telperion_core::math::Vec3;
use telperion_core::pipeline::{self, executor, Request};

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
    println!("| Preset | Seed | Age | Drawn ms | Read ms | Read MB | Nodes | CPU grow ms | Clothe ms | Plan ms | Field ms | Query 36³ ms |");
    println!("|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|");
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
                field: Some(None),
                ..Request::default()
            };
            let built = pipeline::build(&family, request).unwrap();
            let s = built.outputs.stages;
            let clothe = s.total_ms - s.skeleton_ms - s.plan_ms - s.field_ms;
            let field = built.outputs.field.unwrap();
            let b = field.bounds().unwrap();
            let n = 36;
            let size = (b.max - b.min) * (1.0 / n as f64);
            let half = size.x.max(size.y).max(size.z) * 0.5;
            let t = Clock::now();
            for i in 0..n {
                for j in 0..n {
                    for l in 0..n {
                        let c = Vec3::new(
                            b.min.x + size.x * (i as f64 + 0.5),
                            b.min.y + size.y * (j as f64 + 0.5),
                            b.min.z + size.z * (l as f64 + 0.5),
                        );
                        field.query(c, half).unwrap();
                    }
                }
            }
            let query_ms = t.elapsed_ms();
            println!(
                "| {id} | {seed} | {} | {drawn_ms:.0} | {read_ms:.0} | {:.1} | {} | {:.0} | {clothe:.0} | {:.0} | {:.0} | {query_ms:.0} |",
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
