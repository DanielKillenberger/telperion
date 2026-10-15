//! fn-270 R6, R7's measure: each preset grown in through the growth
//! entry as the page grows it, `GROW_STEPS` draws a year, each draw held
//! as a page holds the tree it drew until the next replaces it. Beside
//! it, on the same queue, another renderer draws the preset's whole tree
//! frame after frame, as the page's frame loop draws while its growth is
//! awaited (the whole tree: the dearest frame the run draws). Per preset:
//! growth per year, the draw (settle and placement) at the whole year and
//! within it, each frame's time from its submit to its completion while
//! growing and idle, and the ledger's peak. Run by name:
//!
//!   GROW_STEPS=4 GROW_PRESETS=telperion,oregon-white-oak cargo test --profile ci \
//!     -p telperion-render --lib grows_in_and_draws_while_growing -- --ignored --nocapture
use super::super::{Delivery, Generator};
use crate::headless::Frame;
use crate::{hero_pose, Gpu, Renderer, GROUND_REACH};
use pollster::block_on;
use std::sync::atomic::{AtomicU8, Ordering};
use std::sync::Arc;
use std::time::Instant;
use telperion_core::Family;

const SIZE: (u32, u32) = (1280, 720);
/// Frames drawn before growth starts, idle.
const IDLE: usize = 60;
const WARMING: u8 = 0;
const GROWING: u8 = 1;
const DONE: u8 = 2;

/// One frame drawn at the hero pose of what is up, in milliseconds from
/// its submit to the queue's completing it.
fn frame(renderer: &mut Renderer, target: &Frame) -> f64 {
    let bounds = renderer.bounds().expect("a tree is up");
    let camera = hero_pose(bounds, f64::from(SIZE.0) / f64::from(SIZE.1), GROUND_REACH);
    let started = Instant::now();
    renderer.draw(&camera, SIZE, target.target());
    let done = Arc::new(std::sync::atomic::AtomicBool::new(false));
    let flag = done.clone();
    renderer
        .gpu()
        .queue
        .on_submitted_work_done(move || flag.store(true, Ordering::Release));
    while !done.load(Ordering::Acquire) {
        renderer.gpu().device.poll(wgpu::PollType::Poll).unwrap();
        std::thread::yield_now();
    }
    started.elapsed().as_secs_f64() * 1e3
}

/// A frame loop on its own renderer drawing `family`'s whole tree: idle
/// frames until `IDLE`, then frames while `phase` reads growing.
fn frames(gpu: Gpu, family: Family, phase: Arc<AtomicU8>) -> (Vec<f64>, Vec<f64>) {
    let mut renderer = Renderer::new(gpu, crate::STILL_FORMAT);
    let gen = Generator::new(&renderer).unwrap();
    let whole = gen.prepare(&family, Delivery::Resident).unwrap();
    renderer.submit_prepared(whole).unwrap();
    renderer.set_family(&family);
    let target = Frame::new(&renderer, "growing frame", SIZE);
    let idle: Vec<f64> = (0..IDLE).map(|_| frame(&mut renderer, &target)).collect();
    phase.store(GROWING, Ordering::Release);
    let mut busy = vec![];
    while phase.load(Ordering::Acquire) == GROWING {
        busy.push(frame(&mut renderer, &target));
    }
    (idle, busy)
}

/// The `p`th percentile of `v`, sorted in place.
fn pct(v: &mut [f64], p: f64) -> f64 {
    v.sort_by(f64::total_cmp);
    v.get(((v.len() as f64 - 1.0) * p).round() as usize)
        .copied()
        .unwrap_or(f64::NAN)
}

#[test]
#[ignore = "fn-270 R6/R7 measure: run by name"]
fn grows_in_and_draws_while_growing() {
    let Some(gpu) = crate::generation::test_gpu() else {
        return;
    };
    let steps: u32 = std::env::var("GROW_STEPS").map_or(1, |s| s.parse().unwrap());
    let only = std::env::var("GROW_PRESETS").ok();
    for &(_, id, _, _) in telperion_core::presets::CATALOGUE {
        if only
            .as_ref()
            .is_some_and(|o| !o.split(',').any(|p| p == id))
        {
            continue;
        }
        let family = telperion_core::presets::by_identity(id).unwrap();
        let phase = Arc::new(AtomicU8::new(WARMING));
        let looping = {
            let (gpu, family, phase) = (gpu.clone(), family.clone(), phase.clone());
            std::thread::spawn(move || frames(gpu, family, phase))
        };
        while phase.load(Ordering::Acquire) == WARMING {
            std::thread::sleep(std::time::Duration::from_millis(5));
        }
        let renderer = Renderer::new(gpu.clone(), crate::STILL_FORMAT);
        let gen = Generator::new(&renderer).unwrap();
        block_on(gen.prepare_programs()).unwrap();
        gpu.ledger.take_peak();
        let mut tree = block_on(gen.begin_growth(&family)).unwrap();
        let horizon = tree.horizon();
        let (mut grow, mut whole, mut within, mut held) = (0.0, vec![], vec![], None);
        for year in 1..=horizon {
            for k in 1..=steps {
                // The seedling stands only from its first budburst.
                if year == 1 && k < steps {
                    continue;
                }
                let at = f64::from(year) - 1.0 + f64::from(k) / f64::from(steps);
                let t = Instant::now();
                block_on(gen.grow_to(&mut tree, at)).unwrap();
                grow += t.elapsed().as_secs_f64() * 1e3;
                let t = Instant::now();
                let drawn = block_on(gen.settle(&tree, at)).unwrap();
                let ms = t.elapsed().as_secs_f64() * 1e3;
                if k == steps {
                    whole.push(ms)
                } else {
                    within.push(ms)
                }
                // Held as the page holds the tree it drew, until the next.
                held = Some(drawn);
            }
        }
        phase.store(DONE, Ordering::Release);
        let (mut idle, mut busy) = looping.join().unwrap();
        drop(held);
        let peak = gpu.ledger.take_peak() as f64 / 1e6;
        let mean = |v: &[f64]| v.iter().sum::<f64>() / v.len().max(1) as f64;
        let max = |v: &[f64]| v.iter().copied().fold(0.0, f64::max);
        println!(
            "GROWN {id} {horizon} y x{steps}: grow {:.1} ms/y; draw whole {:.0} ms (max {:.0}), within {:.0} ms (max {:.0}); frames while growing {} p50 {:.1} p95 {:.1} max {:.1} ms; idle p50 {:.1} p95 {:.1}; peak {peak:.0} MB",
            grow / f64::from(horizon), mean(&whole), max(&whole), mean(&within), max(&within),
            busy.len(), pct(&mut busy, 0.5), pct(&mut busy, 0.95), pct(&mut busy, 1.0), pct(&mut idle, 0.5), pct(&mut idle, 0.95),
        );
    }
}
