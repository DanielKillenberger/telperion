//! fn-212 task 10 (R2, R4, R5a), natively: a tree drawn through
//! `Generator::prepare`, its build latency and its steady frame reported
//! apart. One side a process (`SIDE=device` takes `prepare`, which settles
//! the fine roots on the device; `SIDE=whole` grows the tree whole on the
//! CPU executor), `PROBE=norway-spruce:80:1`, `STILLS=<dir>`, under the GPU
//! lock:
//!
//!   cargo test --profile ci -p telperion-render drawn_tree -- --ignored --nocapture
//!
//! A small tree is drawn first, as the page opens on one, so the build
//! timed is a dial's: the generator and its kernels already made.
use super::super::{Delivery, Generator};
use crate::{hero_pose, measure, render, write_png, Frame, Renderer, GROUND_REACH};
use std::path::PathBuf;
use std::time::Instant;

const SIZE: (u32, u32) = (1280, 900);

fn family(probe: &str) -> telperion_core::presets::Family {
    let parts: Vec<&str> = probe.split(':').collect();
    let mut family = telperion_core::presets::by_identity(parts[0]).unwrap();
    family.age = parts[1].parse().unwrap();
    family.skeleton.seed = parts[2].parse().unwrap();
    family
}

fn peak_mb() -> u64 {
    let status = std::fs::read_to_string("/proc/self/status").unwrap_or_default();
    let line = status.lines().find(|l| l.starts_with("VmHWM:"));
    let kb = line.and_then(|l| l.split_whitespace().nth(1)?.parse().ok());
    kb.unwrap_or(0) / 1024
}

#[test]
#[ignore]
fn drawn_tree() {
    let Some(gpu) = crate::generation::test_gpu() else {
        return;
    };
    let probe = std::env::var("PROBE").unwrap_or_else(|_| "norway-spruce:80:1".into());
    let side = std::env::var("SIDE").unwrap_or_else(|_| "device".into());
    let dir = PathBuf::from(std::env::var("STILLS").expect("STILLS: where the stills go"));
    std::fs::create_dir_all(&dir).unwrap();
    let mut renderer = Renderer::new(gpu.clone(), crate::STILL_FORMAT);
    let gen = Generator::new(&renderer).unwrap();
    let family = family(&probe);
    let build = |f: &telperion_core::presets::Family| match side.as_str() {
        "device" => gen.prepare(f, Delivery::Resident),
        "whole" => pollster::block_on(gen.prepare_whole(f, Delivery::Resident)),
        other => panic!("SIDE {other}: device or whole"),
    };
    let mut small = family.clone();
    small.age = 3.0;
    renderer.submit_prepared(build(&small).unwrap()).unwrap();
    let started = Instant::now();
    let prepared = build(&family).unwrap();
    let prepare_ms = started.elapsed().as_secs_f64() * 1e3;
    let leaves = prepared.count();
    if std::env::var("COHERENCE").is_ok() {
        let read = gen.read_instances(&prepared).unwrap();
        let at: Vec<_> = read
            .leaves
            .iter()
            .map(|&l| read.reference.position(l))
            .collect();
        let steps: Vec<f64> = at.windows(2).map(|w| w[0].distance(w[1])).collect();
        let mut sorted = steps.clone();
        sorted.sort_by(f64::total_cmp);
        let mean = steps.iter().sum::<f64>() / steps.len() as f64;
        let p = |q: f64| sorted[((sorted.len() - 1) as f64 * q) as usize];
        println!(
            "consecutive leaves apart: mean {mean:.4} m, p50 {:.4}, p90 {:.4}, p99 {:.4}",
            p(0.5),
            p(0.9),
            p(0.99)
        );
    }
    renderer.submit_prepared(prepared).unwrap();
    renderer.set_material(family.material);
    let aspect = f64::from(SIZE.0) / f64::from(SIZE.1);
    let camera = hero_pose(renderer.bounds().unwrap(), aspect, GROUND_REACH);
    let still = render(&mut renderer, &camera, SIZE.0, SIZE.1).unwrap();
    let drawn_ms = started.elapsed().as_secs_f64() * 1e3;
    let name = probe.replace(':', "-");
    write_png(&dir.join(format!("{name}-{side}.png")), &still).unwrap();
    let tree_bytes = Generator::tree_buffer_bytes(&renderer);
    let frame = Frame::new(&renderer, "timing", SIZE);
    let report = measure(&mut renderer, &camera, SIZE, frame.target()).unwrap();
    if let Ok(path) = std::env::var("REPORT") {
        std::fs::write(format!("{path}-{side}.json"), report.to_json()).unwrap();
    }
    let ms = |v: Option<f64>| v.map_or("null".into(), |v| format!("{v:.2}"));
    println!(
        "{{\"probe\":\"{probe}\",\"side\":\"{side}\",\"prepare_ms\":{prepare_ms:.0},\"drawn_ms\":{drawn_ms:.0},\"leaves\":{leaves},\"tree_gpu_mb\":{},\"peak_rss_mb\":{},\"verdict\":\"{}\",\"vegetation_p50\":{},\"surfacing_p50\":{},\"frame_p50\":{}}}",
        tree_bytes >> 20,
        peak_mb(),
        report.verdict().name(),
        ms(report.p50_ms()),
        ms(report.surfacing_p50_ms()),
        ms(report.total_p50_ms()),
    );
}
