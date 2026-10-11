//! The settle on the device, measured (`#[ignore = "a measurement"]`):
//! every preset at its default age built plain, seed 42, Telperion 82 and
//! the oak 80 seed 1 also built exact. Per pass the GPU time of its
//! stamped dispatches, the dispatches, the longest with its kernel and
//! every one past a millisecond, the leaf mean's time apart, the growth's
//! bytes freed and the settle's own; and each module's compile.
use super::build::{Kit, Tables};
use super::build_grow::{grow, Sizes};
use super::grown_tests::{device, grown};
use super::settle_device::{self as d, Kernels};
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

/// One preset grown, then its settle stamped dispatch by dispatch.
fn measured(gpu: &Gpu, (kit, k): (&Kit, &Kernels), (preset, name, age, seed): (Preset, &str, u32, u64)) {
    let g = grown(preset, age, seed);
    let t = Tables::new(gpu, &g.tables, &g.chain).expect("tables");
    let (b, ..) = grow(gpu, kit, &t, &g.tables, (Sizes::least(), age)).expect("a build");
    let (resident, scratch) = b.bytes(&t);
    let label = format!("{name} {age} seed {seed}");
    assert!(super::super::profile::start(gpu), "timestamps");
    let started = Instant::now();
    let s = settle(gpu, (kit, k), (&b, &t), (&constants(&g), SettleTo::Laid)).expect("a settle");
    gpu.device.poll(wgpu::PollType::wait_indefinitely()).expect("settled");
    let wall = started.elapsed().as_secs_f64();
    let stamps = super::super::profile::finish(gpu);
    assert_eq!(stamps.len(), s.stamped.len(), "a pass a stamp");
    let mut phases: Vec<(&str, f64, usize)> = Vec::new();
    let mut long = Vec::new();
    for (phase, (kernel, a, z)) in s.stamped.iter().zip(&stamps) {
        let ms = (z - a) * 1e-6;
        match phases.iter_mut().find(|p| p.0 == *phase) {
            Some(p) => (p.1, p.2) = (p.1 + ms, p.2 + 1),
            None => phases.push((phase, ms, 1)),
        }
        long.push((ms, format!("{phase} {kernel}")));
    }
    long.sort_by(|a, b| b.0.total_cmp(&a.0));
    let total: f64 = phases.iter().map(|p| p.1).sum();
    let t2 = s.table;
    println!(
        "{label}: kept {} axes, {} units, {} phytomers in {} depths, error {:?}; {} dispatches, {total:.1} ms stamped, wall {wall:.2} s; longest {:.3} ms ({})",
        t2.kept, t2.units, t2.phyts, s.kept_starts.len() - 1, s.error, stamps.len(), long[0].0, long[0].1
    );
    println!(
        "{label}: growth resident {:.0} MB and scratch {:.0} MB, {:.0} MB of it freed; the settle's own {:.0} MB",
        resident as f64 / 1e6, scratch as f64 / 1e6, s.freed as f64 / 1e6, s.bytes() as f64 / 1e6
    );
    for (phase, ms, n) in phases {
        println!("{label}: {phase:>24} {ms:9.2} ms in {n} dispatches");
    }
    for (ms, what) in long.iter().take_while(|l| l.0 > 1.0) {
        println!("{label}: past 1 ms: {ms:.3} ms {what}");
    }
    drop((s, b));
    gpu.device.poll(wgpu::PollType::wait_indefinitely()).expect("let go");
}

#[test]
#[ignore = "a measurement"]
fn settles_are_timed() {
    let _held = super::super::whole_tree();
    let Some((gpu, kit)) = device() else {
        return;
    };
    let k = Kernels::new(&gpu);
    let mut cases: Vec<_> = PRESETS.iter().map(|&(p, n)| (p, n, p.parameters().age as u32, 42)).collect();
    cases.push((Preset::OregonWhiteOak, "oregon-white-oak", 80, 1));
    for case in &cases {
        measured(&gpu, (&kit, &k), *case);
    }
}

/// Each settle module's compile, and the place module's entry by entry:
/// run with the driver's disk cache off (`__GL_SHADER_DISK_CACHE=0`) for
/// the cold figure.
#[test]
#[ignore = "a measurement"]
fn the_settle_compiles() {
    let _held = super::super::whole_tree();
    let Some(gpu) = super::super::test_gpu() else {
        return;
    };
    let timed = |name: &str, make: &dyn Fn() -> super::super::io::Pass| {
        let started = Instant::now();
        let pass = make();
        gpu.device.poll(wgpu::PollType::wait_indefinitely()).expect("compiled");
        println!("compile {name}: {:.2} s", started.elapsed().as_secs_f64());
        drop(pass);
    };
    let place = [true, true, false, false, false];
    let shed = [true, true, true, false, false, false, true, true];
    timed("shed and order", &|| super::super::io::Pass::new(&gpu, d::source(), &shed, &d::ENTRIES));
    for entry in d::PLACE_ENTRIES {
        timed(&format!("place {entry}"), &|| super::super::io::Pass::new(&gpu, d::place_source(), &place, &[entry]));
    }
}

/// Retirement on a busy build: Telperion 82 grown plain without and with
/// its retired subtrees let go at each move, the arena's rows and tables,
/// the records, the retirement's stamped time and dispatches and its
/// longest, the moves and the build's wall time; then its settle.
#[test]
#[ignore = "a measurement"]
fn retirement_is_measured() {
    let _held = super::super::whole_tree();
    let Some((gpu, kit)) = device() else {
        return;
    };
    let k = Kernels::new(&gpu);
    let g = grown(Preset::Telperion, 82, 42);
    let t = Tables::new(&gpu, &g.tables, &g.chain).expect("tables");
    for compact in [false, true] {
        let mut sizes = Sizes::least();
        sizes.compact = compact;
        let (b, sized, _) = grow(&gpu, &kit, &t, &g.tables, (sizes, 82)).expect("a build");
        drop(b);
        gpu.device.poll(wgpu::PollType::wait_indefinitely()).expect("let go");
        assert!(super::super::profile::start(&gpu), "timestamps");
        let started = Instant::now();
        let (b, last, grew) = grow(&gpu, &kit, &t, &g.tables, (sized, 82)).expect("a build");
        let wall = started.elapsed().as_secs_f64();
        let stamps = super::super::profile::finish(&gpu);
        let (mut ms, mut n, mut longest) = (0.0, 0, 0.0f64);
        for (phase, (_, a, z)) in grew.stamped.iter().zip(&stamps) {
            if *phase == "retirement" {
                let d = (z - a) * 1e-6;
                (ms, n, longest) = (ms + d, n + 1, longest.max(d));
            }
        }
        let (resident, scratch) = b.bytes(&t);
        println!(
            "telperion 82 compact {compact}: arena rows {} of tables {}, records {}; resident {:.0} MB, scratch {:.0} MB; retirement {ms:.1} ms in {n} dispatches, longest {longest:.3} ms; {} moves; build wall {wall:.2} s",
            b.held.phyts, last.caps.phyts, b.retire.rec_n, resident as f64 / 1e6, scratch as f64 / 1e6, grew.moves
        );
        let s = settle(&gpu, (&kit, &k), (&b, &t), (&constants(&g), SettleTo::Laid)).expect("a settle");
        drop(b);
        println!("telperion 82 compact {compact}: settled, {} kept axes, {} kept phytomers, error {:?}", s.table.kept, s.table.phyts, s.error);
    }
}
