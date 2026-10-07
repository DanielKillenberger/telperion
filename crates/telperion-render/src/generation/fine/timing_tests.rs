//! fn-212 task 7, the 3x measurement on the real path (`cargo test
//! --profile ci -p telperion-render three_x -- --ignored --nocapture`,
//! `PROBE=norway-spruce:80:1`, `SIDE=today|device`, under the GPU lock,
//! one build a process).
//!
//! Five builds a side, alternately, one a process:
//! - **today:** the resident path from the converted tree to draw-ready
//!   (the generator's total less its skeleton, the submit and the drain),
//!   as task 2's timing probe measures it; the conversion is outside it,
//!   and TASK7.md adds baseline B's;
//! - **the device's:** the real path (`resident::build`) from the settled
//!   tree to draw-ready, its own partial conversion included, then the
//!   submit and the drain, each stage waited on. The settle is growth,
//!   outside the boundary on both sides.
//!
//! `PROFILE=1` also times every compute pass of the device's stations.
use super::super::{profile, Delivery, Generator};
use super::resident::{build, Tools};
use super::roots::{kernels, Kernels};
use crate::Gpu;
use std::time::Instant;
use telperion_core::presets::Family;

fn ms(t: Instant) -> f64 {
    t.elapsed().as_secs_f64() * 1e3
}

fn wait(gpu: &Gpu) {
    gpu.device
        .poll(wgpu::PollType::wait_indefinitely())
        .unwrap();
}

/// One build of today's resident path: milliseconds from the converted
/// tree to draw-ready.
fn today(gpu: &Gpu, family: &Family) -> f64 {
    // A renderer of its own, dropped with the tree it holds.
    let mut renderer = crate::Renderer::new(gpu.clone(), crate::STILL_FORMAT);
    let gen = Generator::new(&renderer).unwrap();
    let prepared = gen.prepare(family, Delivery::Resident).unwrap();
    let m = &prepared.metrics;
    let inside = m.total_ms - m.skeleton_ms;
    let t = Instant::now();
    renderer.submit_prepared(prepared).unwrap();
    renderer.gpu().queue.submit([]);
    wait(renderer.gpu());
    inside + ms(t)
}

/// One build on the real path: its stages, and the submit with its drain.
fn device(gpu: &Gpu, k: &Kernels, family: &Family) {
    let mut renderer = crate::Renderer::new(gpu.clone(), crate::STILL_FORMAT);
    let gen = Generator::new(&renderer).unwrap();
    let tools = Tools::new(gpu);
    let profiling = std::env::var("PROFILE").is_ok() && profile::start(gpu);
    let built = build(gpu, &gen, k, &tools, family).unwrap();
    let passes = profile::finish(gpu);
    let t = Instant::now();
    renderer.submit_prepared(built.prepared).unwrap();
    renderer.gpu().queue.submit([]);
    wait(renderer.gpu());
    let submit = ms(t);
    let inside = built.timed.inside() + submit;
    println!(
        "device {inside:.1} ms, submit {submit:.1}, {:?}",
        built.timed
    );
    if profiling {
        let mut by: Vec<(String, f64, usize)> = Vec::new();
        for (label, a, b) in passes {
            match by.iter_mut().find(|(l, ..)| *l == label) {
                Some(e) => (e.1, e.2) = (e.1 + (b - a) * 1e-6, e.2 + 1),
                None => by.push((label, (b - a) * 1e-6, 1)),
            }
        }
        by.sort_by(|a, b| b.1.total_cmp(&a.1));
        for (label, total, n) in by.iter().take(24) {
            println!("pass {label}: {total:.2} ms over {n}");
        }
    }
}

#[test]
#[ignore]
fn three_x() {
    let Some(gpu) = crate::generation::test_gpu() else {
        return;
    };
    let only = std::env::var("PROBE").unwrap_or_else(|_| "norway-spruce:80:1".into());
    let parts: Vec<&str> = only.split(':').collect();
    let mut family = telperion_core::presets::by_identity(parts[0]).unwrap();
    family.age = parts[1].parse().unwrap();
    family.skeleton.seed = parts[2].parse().unwrap();
    let side = std::env::var("SIDE").unwrap_or_else(|_| "device".into());
    print!("{only}: ");
    if side == "today" {
        println!("today {:.1} ms", today(&gpu, &family));
    } else {
        device(&gpu, &kernels(&gpu), &family);
    }
}
