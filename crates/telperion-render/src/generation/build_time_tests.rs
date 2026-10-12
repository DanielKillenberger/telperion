//! fn-256 task 3: each shipped preset's resident build, from its family
//! to draw-ready, timed warm (its second build in the process) at its
//! default age and seed: the same call before and after the one cycle
//! took the harness. `BUILD_PRESETS` names a comma list to time alone,
//! `BUILD_AGE` an age for them all; each build's peak owned bytes are
//! printed by label (fn-256 task 5's census).
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
    println!("| Preset | Age | Cold ms | Warm ms | Instances | Device peak MB |");
    println!("|---|--:|--:|--:|--:|--:|");
    for id in ids {
        let mut family = telperion_core::presets::by_identity(id).unwrap();
        if let Some(age) = std::env::var("BUILD_AGE").ok().and_then(|a| a.parse().ok()) {
            family.age = age;
        }
        let mut ms = Vec::new();
        let mut count = 0;
        let mut peak = 0;
        for _ in 0..2 {
            gpu.ledger.take_peak_census();
            let t = super::Clock::now();
            let prepared = match gen.prepare(&family, Delivery::Resident) {
                Ok(p) => p,
                Err(e) => {
                    println!("| {id} | {} | refused: {e} | | |", family.age);
                    census(id, gpu.ledger.take_peak_census());
                    break;
                }
            };
            ms.push(t.elapsed_ms());
            count = prepared.count();
            let labelled = gpu.ledger.take_peak_census();
            peak = labelled.0;
            census(id, labelled);
            drop(prepared);
            gpu.device.poll(wgpu::PollType::wait_indefinitely()).unwrap();
        }
        if ms.len() < 2 {
            continue;
        }
        println!("| {id} | {} | {:.0} | {:.0} | {count} | {:.0} |", family.age, ms[0], ms[1], peak as f64 / 1e6);
    }
}

/// A build's peak owned bytes by label, the largest first.
fn census(id: &str, (peak, by): (u64, Vec<(&'static str, u64)>)) {
    let mb = |b: u64| b as f64 / 1e6;
    let rows: Vec<String> = by.iter().take(16).map(|(l, b)| format!("{l} {:.0}", mb(*b))).collect();
    println!("census {id}: peak {:.0} MB: {}", mb(peak), rows.join(", "));
}
