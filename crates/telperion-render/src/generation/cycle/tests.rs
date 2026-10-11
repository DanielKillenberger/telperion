//! The device's units and wake held to the CPU's cycle: young trees in
//! seconds, and the busy cycles DESIGN.md section 7 names, measured.
use super::columns::{Cap, Caps, Columns};
use super::compare::{compare, needed, Agreement, Made, Readback};
use super::device::{Kernels, Run, PHASES};
use crate::Gpu;
use telperion_core::presets::Preset;
use telperion_space::{Capture, ChainTables, Request};

/// `preset` grown to `age` at `seed` through the one cycle, its `at`th
/// cycle captured, and its tables; light as the preset sets it.
fn captured(preset: Preset, age: u32, seed: u64, at: u32) -> (Capture, ChainTables) {
    let family = preset.parameters();
    let species = family.space.species.expect("an engine species");
    let request = Request {
        age,
        seed,
        budget: 20_000_000,
        light: family.space.light,
    };
    let c = telperion_space::grow_capture_units_at(&species, request, at).expect("a capture");
    (c, ChainTables::cycle(&species, age).expect("tables"))
}

/// A first guess at the arenas, grown to what the counts name until the
/// cycle fits: a run that overflowed is run again from the start.
fn fitted(gpu: &Gpu, k: &Kernels, cols: &Columns) -> (Run, Readback) {
    let mut caps = Caps {
        axes: cols.axes + 4 * cols.live + 1024,
        units: cols.units + 2 * cols.live + 1024,
        phyts: 16 * cols.live + 1024,
        sleepers: cols.sleepers + cols.live + 1024,
        next: 4 * cols.live + 1024,
        woken: 4 * cols.sleepers + 64,
    };
    for _ in 0..4 {
        let run = Run::new(gpu, k, cols, caps).expect("buffers");
        let mut e = gpu.encoder(&Default::default());
        run.record(gpu, k, &mut e);
        gpu.submit([e.finish()]);
        let out = run.read(gpu).expect("a readback");
        let need = needed(cols, out.made());
        let fits = need.axes <= caps.axes
            && need.units <= caps.units
            && need.phyts <= caps.phyts
            && need.sleepers <= caps.sleepers
            && need.next <= caps.next
            && need.woken <= caps.woken;
        if fits {
            return (run, out);
        }
        caps = Caps {
            axes: caps.axes.max(need.axes),
            units: caps.units.max(need.units),
            phyts: caps.phyts.max(need.phyts),
            sleepers: caps.sleepers.max(need.sleepers),
            next: caps.next.max(need.next),
            woken: caps.woken.max(need.woken),
        };
    }
    panic!("the arenas never held the cycle");
}

/// The captured cycle on the device, compared with the CPU's, the
/// comparison printed.
fn replay(
    gpu: &Gpu,
    k: &Kernels,
    name: &str,
    (c, tables): &(Capture, ChainTables),
) -> (Run, Made, Columns, Agreement) {
    let cap = Cap(c);
    let cols = Columns::new(&cap, tables);
    let (run, out) = fitted(gpu, k, &cols);
    let agreement = compare(&cap, &cols, &out);
    let m = out.made();
    println!(
        "{name}: cycle {}, {} axes, {} living apexes, {} sleeping buds waking of {}; made {} axes, {} phytomers, {} units, {} sleeping buds, {} living apexes, {} woken",
        cols.cycle, cols.axes, cols.live, m.waking, cols.sleepers, m.axes, m.phyts, m.units, m.sleepers, m.next, m.woken
    );
    println!("{}", agreement.report());
    (run, m, cols, agreement)
}

fn device() -> Option<(Gpu, Kernels)> {
    let gpu = super::super::test_gpu()?;
    let started = std::time::Instant::now();
    let k = Kernels::new(&gpu);
    gpu.device.poll(wgpu::PollType::wait_indefinitely()).expect("compiled");
    println!("compile: {:.2} s", started.elapsed().as_secs_f64());
    Some((gpu, k))
}

#[test]
#[ignore = "red on contract.wgsl's division and contraction until the host rules (UNITS.md)"]
fn young_cycles_agree_with_the_cpu() {
    let _held = super::super::whole_tree();
    let Some((gpu, k)) = device() else {
        return;
    };
    let mut differ = Vec::new();
    for (preset, name, age, at) in [
        (Preset::EuropeanBeech, "european-beech", 6, 6),
        (Preset::Telperion, "telperion", 8, 8),
        (Preset::EuropeanBeech, "european-beech", 12, 8),
        (Preset::Telperion, "telperion", 14, 10),
        (Preset::Ordinary, "ordinary", 16, 12),
        (Preset::NorwaySpruce, "norway-spruce", 24, 16),
    ] {
        let name = format!("{name} {age} at {at}");
        if !replay(&gpu, &k, &name, &captured(preset, age, 42, at))
            .3
            .exact()
        {
            differ.push(name);
        }
    }
    assert!(
        differ.is_empty(),
        "the device's cycle is not the CPU's: {differ:?}"
    );
}

/// The median of `v`.
fn median(mut v: Vec<f64>) -> f64 {
    v.sort_by(f64::total_cmp);
    v[v.len() / 2]
}

/// Nine runs of the cycle, each pass stamped: each phase's median ms.
fn timed(gpu: &Gpu, k: &Kernels, run: &Run) -> Vec<(&'static str, f64)> {
    let mut each: Vec<Vec<f64>> = vec![Vec::new(); PHASES.len()];
    let mut whole = Vec::new();
    for _ in 0..9 {
        assert!(super::super::profile::start(gpu), "timestamps");
        let mut e = gpu.encoder(&Default::default());
        run.record(gpu, k, &mut e);
        gpu.submit([e.finish()]);
        let stamps = super::super::profile::finish(gpu);
        let total: usize = PHASES.iter().map(|p| p.1).sum();
        assert_eq!(stamps.len(), total, "a stamp a pass");
        let mut at = 0;
        for (k, (_, passes)) in PHASES.iter().enumerate() {
            let ns: f64 = stamps[at..at + passes].iter().map(|s| s.2 - s.1).sum();
            each[k].push(ns * 1e-6);
            at += passes;
        }
        whole.push((stamps[total - 1].2 - stamps[0].1) * 1e-6);
    }
    let mut out: Vec<(&str, f64)> = PHASES
        .iter()
        .zip(each)
        .map(|(p, v)| (p.0, median(v)))
        .collect();
    out.push(("first to last", median(whole)));
    out
}

#[test]
#[ignore = "a measurement"]
fn busy_cycles_agree_and_are_timed() {
    let _held = super::super::whole_tree();
    let Some((gpu, k)) = device() else {
        return;
    };
    let mut differ = Vec::new();
    for (preset, name, age) in [
        (Preset::Telperion, "telperion", 82),
        (Preset::EuropeanBeech, "european-beech", 40),
    ] {
        let started = std::time::Instant::now();
        let capture = captured(preset, age, 42, age);
        println!(
            "{name} {age}: captured in {:.1} s",
            started.elapsed().as_secs_f64()
        );
        let (run, m, cols, agreement) = replay(&gpu, &k, &format!("{name} {age}"), &capture);
        if !agreement.exact() {
            differ.push(name);
        }
        drop(capture);
        let times = timed(&gpu, &k, &run);
        let dispatches: usize = PHASES.iter().map(|p| p.1).sum();
        println!(
            "{name} {age}: {dispatches} dispatches, {:.1} MB resident, {:.2} MB scratch ({} living apexes, {} phytomers and {} axes emitted, {} sleeping buds waking)",
            run.resident as f64 / 1e6,
            run.scratch as f64 / 1e6,
            cols.live,
            m.phyts,
            m.axes,
            m.waking
        );
        for (phase, ms) in times {
            println!("{name} {age}: {phase:>14} {ms:8.3} ms");
        }
    }
    assert!(
        differ.is_empty(),
        "the device's cycle is not the CPU's: {differ:?}"
    );
}
