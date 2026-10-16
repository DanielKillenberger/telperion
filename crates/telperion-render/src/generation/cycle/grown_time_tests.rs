//! Whole builds on the device, measured (`#[ignore = "a measurement"]`):
//! every preset at its default age, seed 42 (the oak also at 80, seed
//! 1), built plain; Telperion and the oak also built exact. Per phase the
//! GPU time of its stamped dispatches over the whole build, the
//! dispatches, the longest with its kernel, the bytes resident and as
//! scratch, and each module's compile.
use super::build::{Kit, Tables};
use super::build_arena::hint;
use super::build_grow::{grow_now as grow, Sizes};
use super::census as c;
use super::grown_tests::grown_from;
use super::{device, glue, hier_build, lay_device, light_device, vm};
use crate::Gpu;
use std::time::Instant;
use telperion_core::presets::Preset;

const PRESETS: [(Preset, &str); 8] = [
    (Preset::EuropeanBeech, "european-beech"),
    (Preset::OregonWhiteOak, "oregon-white-oak"),
    (Preset::NorwaySpruce, "norway-spruce"),
    (Preset::DatePalm, "date-palm"),
    (Preset::SilverBirch, "silver-birch"),
    (Preset::Ordinary, "ordinary"),
    (Preset::Laurelin, "laurelin"),
    (Preset::Telperion, "telperion"),
];

/// Each module's compile, the device waited for after each.
fn compiled(gpu: &Gpu) -> Kit {
    let timed = |name: &str, started: Instant| {
        gpu.device
            .poll(wgpu::PollType::wait_indefinitely())
            .expect("compiled");
        println!("compile {name}: {:.2} s", started.elapsed().as_secs_f64());
    };
    let s = Instant::now();
    let units = device::Kernels::new(gpu);
    timed("units and wake", s);
    let s = Instant::now();
    let layout = lay_device::Kernels::new(gpu);
    timed("layout", s);
    let s = Instant::now();
    let hier = hier_build::Kernels::new(gpu);
    timed("hierarchy", s);
    let s = Instant::now();
    let light = light_device::Kernels::new(gpu);
    timed("light", s);
    let s = Instant::now();
    let (glue, vm) = (glue::Kernels::new(gpu), vm::Kernels::new(gpu));
    timed("glue and sizing", s);
    let s = Instant::now();
    let retire = super::retire_device::Kernels::new(gpu);
    timed("retirement", s);
    Kit {
        units,
        layout,
        hier,
        light,
        glue,
        vm,
        retire,
        noop: super::dispatch_probe::Noop::new(gpu),
    }
}

/// One preset's build, grown as a build grows it (`executor::growth`) in
/// the arena its hint sizes, then again with every dispatch stamped.
fn measured(gpu: &Gpu, kit: &Kit, (preset, name, age, seed): (Preset, &str, u32, u64)) {
    let growth =
        telperion_core::pipeline::executor::growth(&preset.parameters()).expect("a growth");
    let g = grown_from(growth.species, growth.request.light, (age, seed));
    let t = Tables::new(gpu, &g.tables, &g.chain).expect("tables");
    let label = format!("{name} {age} seed {seed}");
    let hinted = Sizes::hinted(hint(&g.species, age).expect("a hint")).expect("sized");
    let started = Instant::now();
    let (b, sizes, grew) = grow(gpu, kit, &t, &g.tables, (hinted, age)).expect("a build");
    let cold = started.elapsed().as_secs_f64();
    let census = b.census.clone();
    drop(b);
    println!(
        "{label}: sized in {cold:.2} s, moved {}; {} axes, {} phytomers, {} units, {} living; {:?}",
        grew.moves,
        census[c::AXES],
        census[c::GROWN],
        census[c::UNITS],
        census[c::NEXT],
        sizes
    );
    assert!(super::super::profile::start(gpu), "timestamps");
    let started = Instant::now();
    let (b, _, grew) = grow(gpu, kit, &t, &g.tables, (sizes, age)).expect("a build");
    let wall = started.elapsed().as_secs_f64();
    let stamps = super::super::profile::finish(gpu);
    assert_eq!(stamps.len(), grew.stamped.len(), "a phase a stamp");
    assert_eq!(grew.moves, 0, "built at its sizes");
    let mut phases: Vec<(&str, f64, usize)> = Vec::new();
    let mut longest = (0.0, String::new());
    for (phase, (kernel, a, z)) in grew.stamped.iter().zip(&stamps) {
        let ms = (z - a) * 1e-6;
        match phases.iter_mut().find(|p| p.0 == *phase) {
            Some(p) => (p.1, p.2) = (p.1 + ms, p.2 + 1),
            None => phases.push((phase, ms, 1)),
        }
        if ms > longest.0 {
            longest = (ms, format!("{phase} {kernel}"));
        }
    }
    let total: f64 = phases.iter().map(|p| p.1).sum();
    let span = (stamps.last().map_or(0.0, |s| s.2) - stamps.first().map_or(0.0, |s| s.1)) * 1e-6;
    let (resident, scratch) = b.bytes(&t);
    println!(
        "{label}: {} dispatches, {total:.1} ms stamped, {span:.1} ms first to last, wall {wall:.2} s; longest {:.3} ms ({}); resident {:.1} MB, scratch {:.1} MB",
        stamps.len(), longest.0, longest.1, resident as f64 / 1e6, scratch as f64 / 1e6
    );
    println!(
        "{label}: {:.0} dispatches a cycle; workgroups given {} for {} needed",
        stamps.len() as f64 / f64::from(age),
        grew.over.given,
        grew.over.needed
    );
    for (phase, ms, n) in phases {
        println!("{label}: {phase:>24} {ms:9.2} ms in {n} dispatches");
    }
    drop(b);
    let mut walls = Vec::new();
    for _ in 0..3 {
        let started = Instant::now();
        let (_, _, g) = grow(gpu, kit, &t, &g.tables, (sizes, age)).expect("a build");
        walls.push((
            started.elapsed().as_secs_f64(),
            g.recording,
            g.waiting,
            g.moving,
        ));
    }
    walls.sort_by(|a, b| a.0.total_cmp(&b.0));
    let (w, r, q, m) = walls[1];
    println!("{label}: unstamped wall {w:.3} s (median of 3): recording {r:.3} s, waiting for the census {q:.3} s, moving {m:.3} s");
    if std::env::var("CYCLE_KERNELS").is_ok() {
        kernels(&label, &grew.stamped, &stamps);
    }
}

/// Each kernel's stamped time, dispatches, and those under 10 µs, by
/// phase, the costliest first (`CYCLE_KERNELS` set).
fn kernels(label: &str, phases: &[&str], stamps: &[(String, f64, f64)]) {
    let mut by: Vec<(String, f64, usize, usize)> = Vec::new();
    for (phase, (kernel, a, z)) in phases.iter().zip(stamps) {
        let (name, ms) = (format!("{phase} {kernel}"), (z - a) * 1e-6);
        let tiny = usize::from(ms < 0.01);
        match by.iter_mut().find(|k| k.0 == name) {
            Some(k) => (k.1, k.2, k.3) = (k.1 + ms, k.2 + 1, k.3 + tiny),
            None => by.push((name, ms, 1, tiny)),
        }
    }
    by.sort_by(|a, b| b.1.total_cmp(&a.1));
    for (name, ms, n, tiny) in &by {
        println!("{label}: kernel {name:>40} {ms:9.2} ms in {n} dispatches, {tiny} under 10 µs");
    }
}

/// Telperion alone, for the performance pass; `CYCLE_TIMED` names
/// another tree as `preset:age:seed`.
#[test]
#[ignore = "a measurement"]
fn telperion_is_timed() {
    let _held = super::super::whole_tree();
    let Some(gpu) = super::super::test_gpu() else {
        return;
    };
    let kit = compiled(&gpu);
    let named = std::env::var("CYCLE_TIMED").unwrap_or_else(|_| "telperion:82:42".into());
    let [id, age, seed] = named.split(':').collect::<Vec<_>>()[..] else {
        panic!("CYCLE_TIMED is preset:age:seed")
    };
    let preset = Preset::from_id(id).expect("a shipped preset");
    measured(
        &gpu,
        &kit,
        (
            preset,
            id,
            age.parse().expect("an age"),
            seed.parse().expect("a seed"),
        ),
    );
}

/// The three builds DESIGN.md section 7a budgets, plain.
#[test]
#[ignore = "a measurement"]
fn budgeted_builds_are_timed() {
    let _held = super::super::whole_tree();
    let Some(gpu) = super::super::test_gpu() else {
        return;
    };
    let kit = compiled(&gpu);
    let cases = [
        (Preset::Telperion, "telperion", 82, 42),
        (Preset::OregonWhiteOak, "oregon-white-oak", 80, 1),
        (Preset::EuropeanBeech, "european-beech", 40, 42),
    ];
    for case in cases {
        measured(&gpu, &kit, case);
    }
}

#[test]
#[ignore = "a measurement"]
fn whole_builds_are_timed() {
    let _held = super::super::whole_tree();
    let Some(gpu) = super::super::test_gpu() else {
        return;
    };
    let l = gpu.device.limits();
    println!(
        "limits: buffer {} B, storage binding {} B",
        l.max_buffer_size, l.max_storage_buffer_binding_size
    );
    let kit = compiled(&gpu);
    let mut cases: Vec<_> = PRESETS
        .iter()
        .map(|&(p, n)| (p, n, p.parameters().age as u32, 42))
        .collect();
    cases.push((Preset::OregonWhiteOak, "oregon-white-oak", 80, 1));
    for case in &cases {
        measured(&gpu, &kit, *case);
    }
}

/// Each module's compile on its own: run with the driver's disk cache
/// off (`__GL_SHADER_DISK_CACHE=0`) for the cold figure.
#[test]
#[ignore = "a measurement"]
fn the_kernels_compile() {
    let _held = super::super::whole_tree();
    let Some(gpu) = super::super::test_gpu() else {
        return;
    };
    let started = Instant::now();
    drop(compiled(&gpu));
    println!("compile in all: {:.2} s", started.elapsed().as_secs_f64());
}
