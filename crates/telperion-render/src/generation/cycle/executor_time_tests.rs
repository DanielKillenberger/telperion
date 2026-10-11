//! fn-256 task 2's acceptance, measured (`#[ignore = "a measurement"]`):
//! every preset at its default age, seed 42, and the oak at 80, seed 1,
//! grown with retirement on and its retired subtrees let go at each
//! move, then settled and laid; Telperion 82 and the oak 80 also exact.
//! Per phase the GPU time of the build's and the settle's stamped
//! dispatches, the dispatches a build and a cycle, the longest with its
//! kernel, the unstamped wall, and the device memory at its peak.
use super::build::{Kit, Tables};
use super::build_grow::{grow, Sizes};
use super::grown_tests::{device, grown};
use super::settle_device::Kernels;
use super::settle_run::settle;
use super::settle_tests::constants;
use crate::Gpu;
use std::time::Instant;
use telperion_core::presets::Preset;
use telperion_space::SettleTo;

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

fn let_go(gpu: &Gpu) {
    gpu.device.poll(wgpu::PollType::wait_indefinitely()).expect("let go");
}

/// A phase, its stamped milliseconds and its dispatches.
type Phase = (&'static str, f64, usize);

/// Each phase's stamped milliseconds and dispatches, in their order, and
/// the longest dispatch with its kernel.
fn phases(stamped: &[&'static str], stamps: &[(String, f64, f64)]) -> (Vec<Phase>, (f64, String)) {
    assert_eq!(stamps.len(), stamped.len(), "a dispatch a stamp");
    let mut phases: Vec<(&str, f64, usize)> = Vec::new();
    let mut longest = (0.0, String::new());
    for (phase, (kernel, a, z)) in stamped.iter().zip(stamps) {
        let ms = (z - a) * 1e-6;
        match phases.iter_mut().find(|p| p.0 == *phase) {
            Some(p) => (p.1, p.2) = (p.1 + ms, p.2 + 1),
            None => phases.push((phase, ms, 1)),
        }
        if ms > longest.0 {
            longest = (ms, format!("{phase} {kernel}"));
        }
    }
    (phases, longest)
}

/// The longest stamp with its kernel, and the stamps' sum in ms.
fn by_kernel(stamps: &[(String, f64, f64)]) -> ((f64, String), f64) {
    let mut longest = (0.0, String::new());
    let mut sum = 0.0;
    for (kernel, a, z) in stamps {
        let ms = (z - a) * 1e-6;
        sum += ms;
        if ms > longest.0 {
            longest = (ms, kernel.clone());
        }
    }
    (longest, sum)
}

/// One preset two ways. From the least sizes with its retired subtrees
/// let go at each move (the only place they are let go): its memory at
/// its peak, its every dispatch stamped by kernel (begun-again and moved
/// work included), and its unstamped wall with the settle. Then at the
/// sizes a build without letting go settles on, so it never begins again
/// or moves: each phase's stamped time, the build's and the settle's.
fn measured(gpu: &Gpu, (kit, k): (&Kit, &Kernels), (preset, name, age, seed): (Preset, &str, u32, u64)) {
    let g = grown(preset, age, seed);
    let t = Tables::new(gpu, &g.tables, &g.chain).expect("tables");
    let label = format!("{name} {age} seed {seed}");
    let pa = constants(&g);
    let mut least = Sizes::least();
    least.compact = true;
    gpu.ledger.take_peak();
    assert!(super::super::profile::start(gpu), "timestamps");
    let (b, sizes, grew) = grow(gpu, kit, &t, &g.tables, (least, age)).expect("a build");
    let stamps = super::super::profile::finish(gpu);
    let grown_peak = gpu.ledger.take_peak();
    let (resident, scratch) = b.bytes(&t);
    let (rows, records) = (b.held.phyts, b.retire.rec_n);
    let s = settle(gpu, (kit, k), (&b, &t), (&pa, SettleTo::Laid)).expect("a settle");
    let_go(gpu);
    let settle_peak = gpu.ledger.take_peak();
    let ((ms, kernel), sum) = by_kernel(&stamps);
    println!(
        "{label}: letting go, begun again {}, moved {}; {} dispatches, {sum:.1} ms stamped, longest {ms:.3} ms ({kernel}); arena {rows} rows of {}, {records} records; resident {:.0} MB, scratch {:.0} MB; peak growing {:.0} MB, settling {:.0} MB (the settle's own {:.0} MB); laid {} axes, {} phytomers, error {:?}",
        grew.again, grew.moves, stamps.len(), sizes.caps.phyts, resident as f64 / 1e6, scratch as f64 / 1e6,
        grown_peak as f64 / 1e6, settle_peak as f64 / 1e6, s.bytes() as f64 / 1e6, s.table.kept, s.table.phyts, s.error
    );
    drop((s, b));
    let_go(gpu);
    let mut walls = Vec::new();
    for _ in 0..3 {
        let started = Instant::now();
        let (b, ..) = grow(gpu, kit, &t, &g.tables, (least, age)).expect("a build");
        let grew_s = started.elapsed().as_secs_f64();
        let s = settle(gpu, (kit, k), (&b, &t), (&pa, SettleTo::Laid)).expect("a settle");
        let_go(gpu);
        walls.push((started.elapsed().as_secs_f64(), grew_s));
        drop((s, b));
        let_go(gpu);
    }
    walls.sort_by(|a, b| a.0.total_cmp(&b.0));
    println!("{label}: letting go, unstamped wall {:.3} s, growing {:.3} s of it (median of 3)", walls[1].0, walls[1].1);
    let (b, sizes, _) = grow(gpu, kit, &t, &g.tables, (Sizes::least(), age)).expect("a build");
    drop(b);
    let_go(gpu);
    assert!(super::super::profile::start(gpu), "timestamps");
    let (b, _, grew) = grow(gpu, kit, &t, &g.tables, (sizes, age)).expect("a build");
    let stamps = super::super::profile::finish(gpu);
    assert_eq!((grew.again, grew.moves), (0, 0), "built at its sizes");
    let (built, long_b) = phases(&grew.stamped, &stamps);
    assert!(super::super::profile::start(gpu), "timestamps");
    let s = settle(gpu, (kit, k), (&b, &t), (&pa, SettleTo::Laid)).expect("a settle");
    let_go(gpu);
    let sstamps = super::super::profile::finish(gpu);
    let (settled, long_s) = phases(&s.stamped, &sstamps);
    let total = |p: &[(&str, f64, usize)]| p.iter().map(|p| p.1).sum::<f64>();
    println!(
        "{label}: at its sizes, build {} dispatches ({:.0} a cycle), {:.1} ms stamped, longest {:.3} ms ({}); settle {} dispatches, {:.1} ms stamped, longest {:.3} ms ({})",
        stamps.len(), stamps.len() as f64 / f64::from(age), total(&built), long_b.0, long_b.1,
        sstamps.len(), total(&settled), long_s.0, long_s.1
    );
    for (phase, ms, n) in built.iter().chain(&settled) {
        println!("{label}: {phase:>24} {ms:9.2} ms in {n} dispatches");
    }
    drop((s, b));
    let_go(gpu);
}

#[test]
#[ignore = "a measurement"]
fn the_executor_is_measured() {
    let _held = super::super::whole_tree();
    let mut cases: Vec<_> = PRESETS.iter().map(|&(p, n)| (p, n, p.parameters().age as u32, 42)).collect();
    cases.push((Preset::OregonWhiteOak, "oregon-white-oak", 80, 1));
    let Some((gpu, kit)) = device() else {
        return;
    };
    let k = Kernels::new(&gpu);
    for case in &cases {
        measured(&gpu, (&kit, &k), *case);
    }
}
