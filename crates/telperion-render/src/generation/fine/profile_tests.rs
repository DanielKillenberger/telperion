//! The realized GPU executor's build profiled by kernel (fn-209 F1 task
//! 8): each tree built once warm, then again with every pass's GPU
//! timestamps, summed by entry point; the device's busy time against the
//! build's wall time shows what the host's round trips and allocations
//! cost. `cargo test --profile ci -p telperion-render growth_by_kernel --
//! --ignored --nocapture`, `FINE_ONLY="spruce 80 1"` to take one tree,
//! `FINE_GPU_LOCK` naming the GPU lock.
use super::device::Device;
use super::realized_tests::request;
use super::roots::{kernels, Kernels, Tables};
use crate::generation::profile;
use crate::Gpu;
use std::collections::BTreeMap;
use std::time::Instant;
use telperion_space::{settle, ChainTables, Expected, GeometryTables, Plan, Species};

/// One build of `species` at `at` on the device, batch 65,536; its wall
/// milliseconds and the device's phase costs.
fn build(gpu: &Gpu, k: &Kernels, species: &Species, at: (u32, u64)) -> (f64, super::roots::Cost) {
    let expected = Expected::new(species, at.0, 0);
    let chain = ChainTables::new(species, at.0).expect("tables");
    let geo = GeometryTables::new(species, at.0).expect("tables");
    let t = Instant::now();
    let tables = Tables::new(gpu, &chain, &geo);
    let mut dev = Device::new(gpu, k, &tables, 1 << 16);
    settle(species, request(at.0, at.1), &expected, Plan::default(), &mut dev).expect("settles");
    (t.elapsed().as_secs_f64() * 1e3, dev.cost)
}

/// Each entry point's passes, GPU milliseconds summed and the longest.
fn by_kernel(passes: &[(String, f64, f64)]) -> BTreeMap<&str, (usize, f64, f64)> {
    let mut out: BTreeMap<&str, (usize, f64, f64)> = BTreeMap::new();
    for (label, start, end) in passes {
        let e = out.entry(label.as_str()).or_default();
        let ms = (end - start) / 1e6;
        *e = (e.0 + 1, e.1 + ms, e.2.max(ms));
    }
    out
}

#[test]
#[ignore = "a measurement"]
fn growth_by_kernel() {
    let Some(gpu) = crate::generation::test_gpu() else {
        return;
    };
    let only = std::env::var("FINE_ONLY").unwrap_or_default();
    let lock = std::env::var("FINE_GPU_LOCK").ok().map(|path| {
        let file = std::fs::File::create(path).expect("the lock file");
        file.lock().expect("the GPU lock");
        file
    });
    let k = kernels(&gpu);
    let load = std::fs::read_to_string("/proc/loadavg").unwrap_or_default();
    println!("load at the start: {}", load.trim());
    for (name, species) in [
        ("beech", telperion_space::beech()),
        ("spruce", telperion_space::spruce()),
    ] {
        for at in [(40, 1), (80, 1), (80, 7)] {
            let tree = format!("{name} {} {}", at.0, at.1);
            if !only.is_empty() && only != tree {
                continue;
            }
            let (warm, _) = build(&gpu, &k, &species, at);
            assert!(profile::start(&gpu), "timestamps");
            let (wall, cost) = build(&gpu, &k, &species, at);
            let passes = profile::finish(&gpu);
            let busy: f64 = passes.iter().map(|p| (p.2 - p.1) / 1e6).sum();
            let span = passes.last().map_or(0.0, |l| l.2) - passes.first().map_or(0.0, |f| f.1);
            println!(
                "{tree}: warm {warm:.0} ms, profiled {wall:.0} ms (grow {:.0}, fade {:.0}, lay {:.0}; {} trips, {} waves, {} runs, {} chains, {} again); {} passes, GPU busy {busy:.0} ms over a span of {:.0} ms",
                cost.grow, cost.fade, cost.lay, cost.trips, cost.waves, cost.runs, cost.chains, cost.again,
                passes.len(), span / 1e6
            );
            for (label, (n, ms, most)) in by_kernel(&passes) {
                println!("  {label:>12}: {n:>5} passes, {ms:>8.1} ms, longest {most:.2} ms");
            }
        }
    }
    drop(lock);
}
