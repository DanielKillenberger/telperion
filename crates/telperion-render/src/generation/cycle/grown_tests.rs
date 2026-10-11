//! The whole grower on the device from the seed held to the CPU's
//! (`grow_capture_grown`): young trees of every shipped preset, exact in
//! every word where the kernels are built exact.
use super::build::{Build, Kit, Tables};
use super::build_grow::{grow, Sizes};
use super::columns::Cap;
use super::grown::{cpu_snapshot, Snapshot};
use super::grown_compare::{by_lineage, exact};
use super::Arith;
use crate::Gpu;
use telperion_core::presets::Preset;
use telperion_space::{Capture, ChainTables, Request};

/// The shipped presets, each young enough to grow in seconds.
pub(super) const YOUNG: [(Preset, &str, u32); 8] = [
    (Preset::EuropeanBeech, "european-beech", 8),
    (Preset::OregonWhiteOak, "oregon-white-oak", 9),
    (Preset::NorwaySpruce, "norway-spruce", 12),
    (Preset::DatePalm, "date-palm", 8),
    (Preset::SilverBirch, "silver-birch", 8),
    (Preset::Ordinary, "ordinary", 10),
    (Preset::Laurelin, "laurelin", 8),
    (Preset::Telperion, "telperion", 8),
];

/// What a preset's build reads: its tables, chain tables and request.
pub(super) struct Grown {
    pub tables: Capture,
    pub chain: ChainTables,
    pub species: telperion_space::Species,
    pub request: Request,
}

pub(super) fn grown(preset: Preset, age: u32, seed: u64) -> Grown {
    let family = preset.parameters();
    let species = family.space.species.expect("an engine species");
    let request = Request { age, seed, budget: 20_000_000, light: family.space.light };
    Grown {
        tables: telperion_space::grow_capture_tables(&species, request).expect("tables"),
        chain: ChainTables::cycle(&species, age).expect("chain tables"),
        species,
        request,
    }
}

pub(super) fn device(arith: Arith) -> Option<(Gpu, Kit)> {
    let gpu = super::super::test_gpu()?;
    let started = std::time::Instant::now();
    let kit = Kit::new(&gpu, arith);
    gpu.device.poll(wgpu::PollType::wait_indefinitely()).expect("compiled");
    println!("{arith:?} kernels compiled in {:.1} s", started.elapsed().as_secs_f64());
    Some((gpu, kit))
}

/// The device's tree after `cycle` of `g`'s age, from the seed.
/// The device's tree after `cycle` of `g`'s age, from the seed, its
/// tables from the least a build starts from (`Sizes::least`), grown as
/// the counts ask.
pub(super) fn built(gpu: &Gpu, kit: &Kit, g: &Grown, cycle: u32) -> (Build, Snapshot) {
    let t = Tables::new(gpu, &g.tables, &g.chain).expect("tables");
    let (b, sizes, grew) = grow(gpu, kit, &t, &g.tables, (Sizes::least(), cycle)).unwrap_or_else(|e| panic!("cycle {cycle}: {e}"));
    println!("grown to {:?} with {:?}, begun again {} times ({:?}), moved {}; workgroups given {} for {} needed", sizes.caps, sizes.bounds, grew.again, grew.why, grew.moves, grew.over.given, grew.over.needed);
    let snap = b.snapshot(gpu, cycle).expect("a readback");
    (b, snap)
}

#[test]
fn young_trees_grow_on_the_device_as_on_the_cpu() {
    let _held = super::super::whole_tree();
    let Some((gpu, kit)) = device(Arith::Exact) else {
        return;
    };
    let mut differ = Vec::new();
    // `CYCLE_ONLY` names one preset, for a first run of a change.
    let only = std::env::var("CYCLE_ONLY").ok();
    for (preset, name, age) in YOUNG.into_iter().filter(|y| only.as_deref().is_none_or(|o| o == y.1)) {
        let g = grown(preset, age, 42);
        let capture = telperion_space::grow_capture_grown(&g.species, g.request).expect("the CPU's tree");
        let cpu = cpu_snapshot(&Cap(&capture));
        let (_, dev) = built(&gpu, &kit, &g, age);
        let a = exact(&dev, &cpu);
        println!("{name} {age}: {} axes, {} phytomers\n{}", cpu.lineage.len(), cpu.grown, a.report());
        if !a.exact() {
            println!("{}", by_lineage(&dev, &cpu).report());
            differ.push(name);
        }
    }
    assert!(differ.is_empty(), "the device's trees are not the CPU's: {differ:?}");
}

/// Built plain (`Arith::Plain`, COUNCIL.md's tolerance standard), the
/// same trees: the first cycle and field where each parts from the CPU's,
/// and at its age the agreement by lineage. Reported, not bounded: only
/// that every build completes is held.
#[test]
fn young_trees_built_plain_are_reported() {
    let _held = super::super::whole_tree();
    let Some((gpu, kit)) = device(Arith::Plain) else {
        return;
    };
    for (preset, name, age) in YOUNG {
        let g = grown(preset, age, 42);
        let last = telperion_space::grow_capture_grown(&g.species, g.request).expect("the CPU's tree");
        let (mut first, mut whole) = (None, None);
        for c in 1..=age {
            let at = telperion_space::grow_capture_grown_at(&g.species, g.request, c).expect("the CPU's tree");
            let (dev, cpu) = (built(&gpu, &kit, &g, c).1, cpu_snapshot(&Cap(&at)));
            let a = exact(&dev, &cpu);
            let named = |f: &(String, usize, usize, String)| format!("cycle {c}, {} ({} of {} rows; {})", f.0, f.2, f.1, f.3);
            if first.is_none() {
                first = a.fields.iter().find(|f| f.2 > 0).map(named);
            }
            let ints = super::grown::integer_fields(&dev);
            if let Some(f) = a.fields.iter().find(|f| f.2 > 0 && (f.0 == "phytomers grown" || ints.contains(&f.0))) {
                whole = Some(named(f));
                break;
            }
        }
        let dev = built(&gpu, &kit, &g, age).1;
        let cpu = cpu_snapshot(&Cap(&last));
        println!("{name} {age}: first parts at {}", first.as_deref().unwrap_or("none"));
        println!("{name} {age}: an integer first parts at {}", whole.as_deref().unwrap_or("none"));
        println!("{}", by_lineage(&dev, &cpu).report());
    }
}
