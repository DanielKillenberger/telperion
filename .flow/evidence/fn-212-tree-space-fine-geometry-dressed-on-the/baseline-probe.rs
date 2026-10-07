//! fn-212 task 1's baseline probe (copied to
//! crates/telperion-render/examples/f212_baseline.rs to run, never committed
//! there; needs baseline-marks.patch applied). One preset and seed a
//! process: `builds` builds through `Generator::prepare` (resident), each
//! then submitted and the device drained. Prints one JSON line a build: the
//! stage marks from settled geometry on (ms, peak RSS MB since settled), the
//! submit, the drain, and the generator's own metrics.
//!
//!   f212_baseline <preset> <age> <seed> <builds>
use std::time::Instant;
use telperion_core::{presets::Preset, probe_marks};
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
    let l = renderer.gpu().device.limits();
    eprintln!(
        "LIMITS max_buffer_size {} max_storage_buffer_binding_size {} max_compute_workgroups_per_dimension {}",
        l.max_buffer_size, l.max_storage_buffer_binding_size, l.max_compute_workgroups_per_dimension
    );
    for build in 0..builds {
        let started = Instant::now();
        let prepared = generator.prepare(&family, Delivery::Resident)?;
        let prepare_ms = ms(started);
        let marks: Vec<_> = probe_marks::MARKS.lock().unwrap().clone();
        let m = &prepared.metrics;
        let metrics = format!(
            "{{\"skeleton\":{:.1},\"descriptors\":{:.1},\"position_prepare\":{:.1},\"position_upload\":{:.1},\"position_wait\":{:.1},\"upload_dispatch\":{:.1},\"placement_wait\":{:.1},\"compact\":{:.1},\"mass\":{:.1},\"wood\":{:.1},\"total\":{:.1},\"gpu_positions\":{},\"position_fallback\":{:?},\"wood_fallback\":{:?},\"backend\":\"{:?}\",\"input_instances\":{},\"instances\":{},\"base_cpu_bytes\":{},\"shared_contact_cpu_bytes\":{},\"shared_prepare_cpu_bytes\":{},\"position_cpu_bytes\":{},\"position_gpu_peak_bytes\":{},\"gpu_compute_peak_bytes\":{},\"retained_gpu_bytes\":{},\"descriptor_cpu_bytes\":{}}}",
            m.skeleton_ms, m.descriptors_ms, m.position_prepare_ms, m.position_upload_ms,
            m.position_wait_ms, m.upload_dispatch_ms, m.placement_wait_ms, m.compact_ms,
            m.mass_ms, m.wood_ms, m.total_ms, m.gpu_positions, m.position_fallback,
            m.wood_fallback, prepared.backend, m.input_instances, m.instances,
            m.base_cpu_bytes, m.shared_contact_cpu_bytes, m.shared_prepare_cpu_bytes,
            m.position_cpu_bytes, m.position_gpu_peak_bytes, m.gpu_compute_peak_bytes,
            m.retained_gpu_bytes, m.descriptor_cpu_bytes,
        );
        let curve_points = prepared.curve().points.len();
        let leaves = prepared.count();
        let t = Instant::now();
        renderer.submit_prepared(prepared)?;
        let submit_ms = ms(t);
        let t = Instant::now();
        // The submit's uploads are queued writes: an empty submission
        // flushes them, and the wait holds until the device has them.
        renderer.gpu().queue.submit([]);
        renderer.gpu().device.poll(wgpu::PollType::wait_indefinitely())?;
        let drain_ms = ms(t);
        let after_hwm = hwm();
        let tree_bytes = Generator::tree_buffer_bytes(&renderer);
        let since: f64 = marks.iter().map(|m| m.1).sum::<f64>() + submit_ms + drain_ms;
        let marks: Vec<String> = marks
            .iter()
            .map(|(n, ms, mb)| format!("[\"{n}\",{ms:.1},{mb}]"))
            .collect();
        println!(
            "{{\"preset\":\"{}\",\"age\":{},\"seed\":{},\"build\":{build},\"load\":{},\"prepare_ms\":{prepare_ms:.1},\"settled_to_resident_ms\":{since:.1},\"submit_ms\":{submit_ms:.1},\"drain_ms\":{drain_ms:.1},\"peak_rss_mb_since_settled\":{after_hwm},\"tree_buffer_bytes\":{tree_bytes},\"leaves\":{leaves},\"curve_points\":{curve_points},\"marks\":[{}],\"metrics\":{metrics}}}",
            a[0], a[1], a[2], load(), marks.join(",")
        );
    }
    Ok(())
}

fn ms(t: Instant) -> f64 {
    t.elapsed().as_secs_f64() * 1e3
}
fn load() -> String {
    std::fs::read_to_string("/proc/loadavg")
        .map(|s| s.split_whitespace().next().unwrap_or("?").to_owned())
        .unwrap_or_default()
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
