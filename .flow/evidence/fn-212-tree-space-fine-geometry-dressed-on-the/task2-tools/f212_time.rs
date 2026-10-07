//! fn-212 task 2 scratch (never committed): per build, the resident path's
//! time from the grown tree to draw-ready resident artifacts (the
//! generator's total less its skeleton stage, plus the submit and the
//! drain), its metrics and peak RSS. With DUMP=<file>, the last build's
//! leaves are written there (12 B a leaf) for a comparison across builds.
//!
//!   f212_time <preset> <age> <seed> <builds>
use std::time::Instant;
use telperion_core::presets::Preset;
use telperion_render::{
    generation::{Delivery, Generator},
    Gpu, Renderer, STILL_FORMAT,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let a: Vec<String> = std::env::args().skip(1).collect();
    let mut family = Preset::from_id(&a[0]).ok_or("no preset")?.parameters();
    family.age = a[1].parse()?;
    family.skeleton.seed = a[2].parse()?;
    let builds: usize = a[3].parse()?;
    let mut renderer = Renderer::new(pollster::block_on(Gpu::request(None))?, STILL_FORMAT);
    let generator = Generator::new(&renderer)?;
    for build in 0..builds {
        let load = load();
        let prepared = generator.prepare(&family, Delivery::Resident)?;
        let m = &prepared.metrics;
        let (total_ms, skeleton_ms) = (m.total_ms, m.skeleton_ms);
        let fields = format!(
            "\"skeleton_ms\":{:.1},\"total_ms\":{:.1},\"position_prepare_ms\":{:.1},\"descriptors_ms\":{:.1},\"upload_dispatch_ms\":{:.1},\"placement_wait_ms\":{:.1},\"wood_ms\":{:.1},\"wood_prepare_ms\":{:.1},\"gpu_positions\":{},\"position_fallback\":\"{:?}\",\"wood_fallback\":\"{:?}\",\"gpu_compute_peak_bytes\":{}",
            m.skeleton_ms, m.total_ms, m.position_prepare_ms, m.descriptors_ms,
            m.upload_dispatch_ms, m.placement_wait_ms, m.wood_ms, m.wood_prepare_ms,
            m.gpu_positions, m.position_fallback, m.wood_fallback, m.gpu_compute_peak_bytes,
        );
        let leaves = prepared.count();
        let points = prepared.curve().points.len();
        if build + 1 == builds {
            if let Ok(path) = std::env::var("DUMP") {
                let instances = generator.read_instances(&prepared)?;
                let bytes: Vec<u8> = instances
                    .leaves
                    .iter()
                    .flat_map(|l| l.iter().flat_map(|w| w.to_le_bytes()))
                    .collect();
                std::fs::write(path, bytes)?;
            }
        }
        let t = Instant::now();
        renderer.submit_prepared(prepared)?;
        let submit = ms(t);
        let t = Instant::now();
        renderer.gpu().queue.submit([]);
        renderer.gpu().device.poll(wgpu::PollType::wait_indefinitely())?;
        let drain = ms(t);
        let resident = total_ms - skeleton_ms + submit + drain;
        println!(
            "{{\"preset\":\"{}\",\"age\":{},\"seed\":{},\"build\":{build},\"load\":{load},\"grown_to_resident_ms\":{resident:.1},\"submit_ms\":{submit:.1},\"drain_ms\":{drain:.1},\"leaves\":{leaves},\"curve_points\":{points},\"peak_rss_mb\":{},{fields}}}",
            a[0], a[1], a[2], hwm(),
        );
    }
    Ok(())
}

fn ms(t: Instant) -> f64 {
    t.elapsed().as_secs_f64() * 1e3
}
fn load() -> String {
    std::fs::read_to_string("/proc/loadavg")
        .map(|s| s.split_whitespace().next().unwrap_or("0").to_owned())
        .unwrap_or_else(|_| "0".into())
}
fn hwm() -> u64 {
    std::fs::read_to_string("/proc/self/status")
        .ok()
        .and_then(|s| {
            let l = s.lines().find(|l| l.starts_with("VmHWM:"))?.to_owned();
            l.split_whitespace().nth(1)?.parse::<u64>().ok()
        })
        .map_or(0, |kb| kb / 1024)
}
