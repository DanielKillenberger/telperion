//! fn-198 task 2's timing: from a preset's parameters to a prepared tree on
//! the device (`Generator::prepare`, resident delivery), one tree a process
//! so the peak memory is its own. Cores are rayon's (`RAYON_NUM_THREADS=1`
//! for one). Prints one JSON line.
//!
//!   cargo run --profile ci -p telperion-render --example prepare_timing -- <preset> <age> [seed]
use std::time::Instant;
use telperion_core::presets::Preset;
use telperion_render::{generation::Generator, Gpu, Renderer, STILL_FORMAT};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let [id, age, rest @ ..] = &args[..] else {
        return Err("usage: prepare_timing <preset> <age> [seed]".into());
    };
    let mut family = Preset::from_id(id).ok_or("no preset")?.parameters();
    family.age = age.parse()?;
    family.skeleton.seed = rest.first().map_or(Ok(1), |s| s.parse())?;
    let renderer = Renderer::new(pollster::block_on(Gpu::request(None))?, STILL_FORMAT);
    let generator = Generator::new(&renderer)?;
    let started = Instant::now();
    let prepared = generator.prepare(&family)?;
    let total = started.elapsed().as_secs_f64() * 1e3;
    let m = &prepared.metrics;
    println!(
        "{{\"preset\":\"{id}\",\"age\":{age},\"seed\":{},\"threads\":{},\"total_ms\":{total:.0},\"skeleton_ms\":{:.0},\"descriptors_ms\":{:.0},\"wood_ms\":{:.0},\"leaves\":{},\"peak_rss_mb\":{}}}",
        family.skeleton.seed,
        rayon_threads(),
        m.skeleton_ms,
        m.descriptors_ms,
        m.wood_ms,
        prepared.count(),
        peak_mb()
    );
    Ok(())
}

fn rayon_threads() -> String {
    std::env::var("RAYON_NUM_THREADS").unwrap_or_else(|_| "all".into())
}

/// The process's peak resident memory in MB, from `/proc/self/status`.
fn peak_mb() -> u64 {
    std::fs::read_to_string("/proc/self/status")
        .ok()
        .and_then(|s| {
            let line = s.lines().find(|l| l.starts_with("VmHWM:"))?.to_owned();
            line.split_whitespace().nth(1)?.parse::<u64>().ok()
        })
        .map_or(0, |kb| kb / 1024)
}
