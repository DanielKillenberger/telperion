//! The whole grower on the device from the seed held to the CPU's
//! (`grow_capture_grown`): young trees of every shipped preset, to the
//! ruled standard (`ruled.rs`, DESIGN.md section 9).
use super::build::{Build, Kit, Tables};
use super::build_grow::{grow_now as grow, Sizes};
use super::columns::Cap;
use super::grown::{cpu_snapshot, Snapshot};
use super::grown_compare::by_lineage;
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
    grown_from(species, family.space.light, (age, seed))
}

/// `species`'s build of `age` from `seed`, lit as `light`.
pub(super) fn grown_from(species: telperion_space::Species, light: telperion_space::Light, (age, seed): (u32, u64)) -> Grown {
    let request = Request { age, seed, budget: 20_000_000, light };
    Grown {
        tables: telperion_space::grow_capture_tables(&species, request).expect("tables"),
        chain: ChainTables::cycle(&species, age).expect("chain tables"),
        species,
        request,
    }
}

pub(super) fn device() -> Option<(Gpu, Kit)> {
    let gpu = super::super::test_gpu()?;
    let started = std::time::Instant::now();
    let kit = Kit::new(&gpu);
    gpu.device.poll(wgpu::PollType::wait_indefinitely()).expect("compiled");
    println!("kernels compiled in {:.1} s", started.elapsed().as_secs_f64());
    Some((gpu, kit))
}

/// The device's tree after `cycle` of `g`'s age, from the seed.
/// The device's tree after `cycle` of `g`'s age, from the seed, its
/// tables from the least a build starts from (`Sizes::least`), grown as
/// the counts ask.
pub(super) fn built(gpu: &Gpu, kit: &Kit, g: &Grown, cycle: u32) -> (Build, Snapshot) {
    built_with(gpu, kit, g, (cycle, false))
}

/// `built`, its retirement logging its verdicts where `log`.
pub(super) fn built_with(gpu: &Gpu, kit: &Kit, g: &Grown, (cycle, log): (u32, bool)) -> (Build, Snapshot) {
    let mut t = Tables::new(gpu, &g.tables, &g.chain).expect("tables");
    t.log = log;
    let (b, sizes, grew) = grow(gpu, kit, &t, &g.tables, (Sizes::least(), cycle)).unwrap_or_else(|e| panic!("cycle {cycle}: {e}"));
    println!("grown to {:?} with {:?}, begun again {} times ({:?}), moved {}; workgroups given {} for {} needed", sizes.caps, sizes.bounds, grew.again, grew.why, grew.moves, grew.over.given, grew.over.needed);
    let snap = b.snapshot(gpu, cycle).expect("a readback");
    (b, snap)
}

/// The CPU's tree of `g` at `age` and the device's, compared: what breaks
/// the ruled standard, with the report printed.
pub(super) fn held_to_cpu(gpu: &Gpu, kit: &Kit, g: &Grown, age: u32) -> Vec<String> {
    let capture = telperion_space::grow_capture_grown(&g.species, g.request).expect("the CPU's tree");
    let cpu = cpu_snapshot(&Cap(&capture));
    let (_, dev) = built(gpu, kit, g, age);
    let by = by_lineage(&dev, &cpu);
    println!("{} axes, {} phytomers\n{}", cpu.lineage.len(), cpu.grown, by.report());
    super::ruled::findings(&by)
}

#[test]
fn young_trees_grow_on_the_device_as_on_the_cpu() {
    let _held = super::super::whole_tree();
    let Some((gpu, kit)) = device() else {
        return;
    };
    let mut apart = Vec::new();
    // `CYCLE_ONLY` names one preset, for a first run of a change.
    let only = std::env::var("CYCLE_ONLY").ok();
    for (preset, name, age) in YOUNG.into_iter().filter(|y| only.as_deref().is_none_or(|o| o == y.1)) {
        print!("{name} {age}: ");
        let found = held_to_cpu(&gpu, &kit, &grown(preset, age, 42), age);
        apart.extend(found.into_iter().map(|f| format!("{name}: {f}")));
    }
    assert!(apart.is_empty(), "the device's trees are not the CPU's: {apart:#?}");
}
