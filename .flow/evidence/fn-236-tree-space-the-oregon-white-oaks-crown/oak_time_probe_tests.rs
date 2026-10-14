//! fn-236 probe, not product code: the oak's warm resident build, as the
//! harness takes it. Copied into `crates/telperion-render/src/generation/`
//! with a `mod` line by `time.sh` for a run and removed after.
//!
//! `OAK_VALUES` names a value file, `OAK_AGE` the age; seed 1. One
//! warm-up build, then five unstamped builds, wall-timed (`Clock`, family
//! to draw-ready, as `build_time_tests.rs`), then five with every compute
//! pass GPU-stamped (`profile`): the stamped passes' sum and their span
//! (first start to last end), with each stamped build's wall beside them.
use super::{Clock, Delivery, Generator};
use crate::Renderer;

fn median(mut v: Vec<f64>) -> f64 {
    v.sort_by(f64::total_cmp);
    v[v.len() / 2]
}

#[test]
#[ignore = "a measurement"]
fn oak_time_probe() {
    let Some(gpu) = super::test_gpu() else { return };
    let path = std::env::var("OAK_VALUES").expect("OAK_VALUES");
    let text = std::fs::read_to_string(&path).expect("the value file");
    let mut family = telperion_core::presets::values::read(&text).expect("it reads");
    family.age = std::env::var("OAK_AGE").expect("OAK_AGE").parse().unwrap();
    family.skeleton.seed = 1;
    let renderer = Renderer::new(gpu.clone(), crate::STILL_FORMAT);
    let gen = Generator::new(&renderer).unwrap();
    let let_go = || gpu.device.poll(wgpu::PollType::wait_indefinitely()).unwrap();
    gpu.ledger.take_peak_census();
    let t = Clock::now();
    let warm_up = gen.prepare(&family, Delivery::Resident).expect("a build");
    let first = t.elapsed_ms();
    let leaves = warm_up.count();
    drop(warm_up);
    let_go();
    let (mut wall, mut stamped_wall, mut sum, mut span) = (vec![], vec![], vec![], vec![]);
    for _ in 0..5 {
        let t = Clock::now();
        let p = gen.prepare(&family, Delivery::Resident).expect("a build");
        wall.push(t.elapsed_ms());
        drop(p);
        let_go();
    }
    for _ in 0..5 {
        assert!(super::profile::start(&gpu), "timestamps");
        let t = Clock::now();
        let p = gen.prepare(&family, Delivery::Resident).expect("a build");
        stamped_wall.push(t.elapsed_ms());
        let stamps = super::profile::finish(&gpu);
        sum.push(stamps.iter().map(|(_, a, z)| (z - a) * 1e-6).sum::<f64>());
        let a = stamps.iter().map(|s| s.1).fold(f64::MAX, f64::min);
        let z = stamps.iter().map(|s| s.2).fold(f64::MIN, f64::max);
        span.push((z - a) * 1e-6);
        drop(p);
        let_go();
    }
    let device_peak = gpu.ledger.take_peak_census().0 as f64 / 1e6;
    let rss = std::fs::read_to_string("/proc/self/status").ok().and_then(|s| {
        let line = s.lines().find(|l| l.starts_with("VmHWM:"))?.to_owned();
        line.split_whitespace().nth(1)?.parse::<f64>().ok()
    });
    let walls: Vec<f64> = wall.iter().map(|w| w.round()).collect();
    println!(
        "OAK {{\"values\":\"{path}\",\"age\":{},\"leaves\":{leaves},\"first_ms\":{first:.0},\"warm_wall_ms\":{:.0},\"warm_walls\":{walls:?},\"stamped_wall_ms\":{:.0},\"gpu_stamped_sum_ms\":{:.0},\"gpu_span_ms\":{:.0},\"device_peak_mb\":{device_peak:.0},\"process_peak_rss_mb\":{:.0}}}",
        family.age,
        median(wall),
        median(stamped_wall),
        median(sum),
        median(span),
        rss.unwrap_or(0.0) / 1024.0
    );
}
