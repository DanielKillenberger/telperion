//! Whole builds on the device, measured (`#[ignore = "a measurement"]`):
//! every preset at its default age, seed 42 (the oak also at 80, seed
//! 1), built plain; Telperion and the oak also built exact. Per phase the
//! GPU time of its stamped dispatches over the whole build, the
//! dispatches, the longest with its kernel, the bytes resident and as
//! scratch, and each module's compile.
use super::build::{Kit, Tables};
use super::build_grow::{grow, Sizes};
use super::census as c;
use super::grown_tests::grown;
use super::{device, glue, hier_build, lay_device, light_device, vm, Arith};
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

/// Each module's compile under `arith`, the device waited for after each.
fn compiled(gpu: &Gpu, arith: Arith) -> Kit {
    let timed = |name: &str, started: Instant| {
        gpu.device.poll(wgpu::PollType::wait_indefinitely()).expect("compiled");
        println!("compile {arith:?} {name}: {:.2} s", started.elapsed().as_secs_f64());
    };
    let s = Instant::now();
    let units = device::Kernels::new(gpu, arith);
    timed("units and wake", s);
    let s = Instant::now();
    let layout = lay_device::Kernels::new(gpu, arith);
    timed("layout", s);
    let s = Instant::now();
    let hier = hier_build::Kernels::new(gpu, arith);
    timed("hierarchy", s);
    let s = Instant::now();
    let light = light_device::Kernels::new(gpu, arith);
    timed("light", s);
    let s = Instant::now();
    let (glue, vm) = (glue::Kernels::new(gpu), vm::Kernels::new(gpu));
    timed("glue and sizing", s);
    Kit { units, layout, hier, light, glue, vm }
}

/// One preset's build: sized from the least, then built again at those
/// sizes with every dispatch stamped.
fn measured(gpu: &Gpu, kit: &Kit, (preset, name, age, seed): (Preset, &str, u32, u64), arith: Arith) {
    let g = grown(preset, age, seed);
    let t = Tables::new(gpu, &g.tables, &g.chain).expect("tables");
    let label = format!("{name} {age} seed {seed} {arith:?}");
    let started = Instant::now();
    let (b, sizes, grew) = grow(gpu, kit, &t, &g.tables, (Sizes::least(), age)).expect("a build");
    let cold = started.elapsed().as_secs_f64();
    let census = b.census.clone();
    drop(b);
    println!(
        "{label}: sized in {cold:.2} s, begun again {} ({:?}), moved {}; {} axes, {} phytomers, {} units, {} living; {:?}",
        grew.again, grew.why, grew.moves, census[c::AXES], census[c::GROWN], census[c::UNITS], census[c::NEXT], sizes
    );
    assert!(super::super::profile::start(gpu), "timestamps");
    let started = Instant::now();
    let (b, _, grew) = grow(gpu, kit, &t, &g.tables, (sizes, age)).expect("a build");
    let wall = started.elapsed().as_secs_f64();
    let stamps = super::super::profile::finish(gpu);
    assert_eq!(stamps.len(), grew.stamped.len(), "a phase a stamp");
    assert_eq!((grew.again, grew.moves), (0, 0), "built at its sizes");
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
    for (phase, ms, n) in phases {
        println!("{label}: {phase:>24} {ms:9.2} ms in {n} dispatches");
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
    println!("limits: buffer {} B, storage binding {} B", l.max_buffer_size, l.max_storage_buffer_binding_size);
    let plain = compiled(&gpu, Arith::Plain);
    let mut cases: Vec<_> = PRESETS.iter().map(|&(p, n)| (p, n, p.parameters().age as u32, 42)).collect();
    cases.push((Preset::OregonWhiteOak, "oregon-white-oak", 80, 1));
    for case in &cases {
        measured(&gpu, &plain, *case, Arith::Plain);
    }
    drop(plain);
    let exact = compiled(&gpu, Arith::Exact);
    for case in cases.iter().filter(|c| matches!(c.0, Preset::Telperion | Preset::OregonWhiteOak)) {
        measured(&gpu, &exact, *case, Arith::Exact);
    }
}
