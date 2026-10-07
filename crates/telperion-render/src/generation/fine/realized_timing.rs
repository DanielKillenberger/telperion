//! The realized GPU executor at 40 and 80 against the CPU executor
//! (fn-209 F1 task 6's acceptance): the beech and spruce, seeds 1 and 7,
//! topology by lineage and geometry within fn-125's screen in two orders
//! and batches, and the whole tree's build timed cold and warm on the
//! device and on 1 and all CPU cores, round trips counted: `cargo test
//! --profile ci -p telperion-render realized_at_40_and_80 -- --ignored
//! --nocapture`, with `FINE_GPU_LOCK` naming the GPU lock, taken only
//! around the device's builds.
use super::realized_tests::{request, settled_against, Apart};
use super::roots::kernels;
use std::time::Instant;
use telperion_space::{grow_fine, root_axes, Plan, Species};

/// fn-125's screen, in metres.
const MOST: f64 = 50e-6;
const RMS: f64 = 10e-6;

/// The GPU lock named by `FINE_GPU_LOCK`, held while the guard lives.
fn locked() -> Option<std::fs::File> {
    let path = std::env::var("FINE_GPU_LOCK").ok()?;
    let file = std::fs::OpenOptions::new()
        .create(true)
        .truncate(false)
        .write(true)
        .open(path)
        .expect("the lock file");
    file.lock().expect("the GPU lock");
    Some(file)
}

/// The CPU executor's build of the tree on `threads` cores, cold and
/// then the best of two warm, in milliseconds.
fn cpu(species: &Species, at: (u32, u64), threads: usize) -> (f64, f64) {
    let plan = Plan {
        threads,
        cut: cut(),
        ..Plan::default()
    };
    let mut times = (0..3).map(|_| {
        let t = Instant::now();
        grow_fine(species, request(at.0, at.1), plan).expect("grows");
        t.elapsed().as_secs_f64() * 1e3
    });
    let cold = times.next().expect("a build");
    (cold, times.fold(f64::INFINITY, f64::min))
}

/// The cut: `FINE_CUT`, or the default plan's.
pub(super) fn cut() -> f64 {
    let at = std::env::var("FINE_CUT").ok();
    at.and_then(|c| c.parse().ok())
        .unwrap_or(Plan::default().cut)
}

fn ms(a: &Apart) -> String {
    let c = &a.cost;
    format!(
        "{:.0} ({:.0}, of it {:.0} fading / {:.0}; {} trips)",
        a.wall, c.grow, c.fade, c.lay, c.trips
    )
}

#[test]
#[ignore = "a measurement over the old trees"]
fn realized_at_40_and_80() {
    let Some(gpu) = crate::generation::test_gpu() else {
        return;
    };
    let k = {
        let _lock = locked();
        kernels(&gpu)
    };
    let load = std::fs::read_to_string("/proc/loadavg").unwrap_or_default();
    println!("load at the start: {}; cut {}", load.trim(), cut());
    println!("| Tree | Roots | Phytomers | Tips most / RMS µm | Radii most rel | GPU cold ms (device phase 1 / phases 2 to 4; trips) | GPU warm ms | CPU all cores cold / warm ms | CPU 1 core cold / warm ms | Statuses |");
    println!("|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|");
    let mut outside = Vec::new();
    for (name, species) in [
        ("beech", telperion_space::beech()),
        ("spruce", telperion_space::spruce()),
    ] {
        for age in [40, 80] {
            for seed in [1, 7] {
                let at = (age, seed);
                let plan = Plan {
                    cut: cut(),
                    ..Plan::default()
                };
                let only = std::env::var("FINE_ONLY").unwrap_or_default();
                if !only.is_empty() && only != format!("{name} {age} {seed}") {
                    continue;
                }
                let fine = grow_fine(&species, request(age, seed), plan).expect("grows");
                let want = root_axes(&fine);
                drop(fine);
                let (all_cold, all_warm) = cpu(&species, at, 0);
                let (one_cold, one_warm) = cpu(&species, at, 1);
                let forward: Vec<usize> = (0..want.len()).collect();
                let reverse: Vec<usize> = forward.iter().rev().copied().collect();
                let lock = locked();
                let cold =
                    settled_against(&gpu, &k, (&species, at, plan), &want, (&forward, 1 << 16));
                // Sown while the structure above the cut grows (task 9).
                let warm = settled_against(&gpu, &k, (&species, at, plan), &want, (&[], 2048));
                let other =
                    settled_against(&gpu, &k, (&species, at, plan), &want, (&reverse, 4096));
                drop(lock);
                assert_eq!(
                    warm.digest, other.digest,
                    "{name} {at:?}: any order and batch"
                );
                println!(
                    "| {name} {age} {seed} | {} | {} | {:.1} / {:.2} | {:.1e} | {} | {} | {all_cold:.0} / {all_warm:.0} | {one_cold:.0} / {one_warm:.0} | {} |",
                    want.len(),
                    warm.phytomers,
                    warm.most * 1e6,
                    warm.rms() * 1e6,
                    warm.radius,
                    ms(&cold),
                    ms(&warm),
                    warm.statuses,
                );
                println!(
                    "  grounded {}, farthest of theirs {:.1} µm",
                    warm.grounded,
                    warm.landed_most * 1e6
                );
                println!(
                    "  worst {:?}, waves {}, chains {}, again {}",
                    warm.worst, warm.cost.waves, warm.cost.chains, warm.cost.again
                );
                if warm.most > MOST || warm.rms() > RMS {
                    outside.push(format!("{name} {age} {seed}"));
                }
            }
        }
    }
    let load = std::fs::read_to_string("/proc/loadavg").unwrap_or_default();
    println!("load at the end: {}", load.trim());
    assert!(outside.is_empty(), "outside fn-125's screen: {outside:?}");
}
