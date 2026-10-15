//! fn-277: the device draws the counts of organs and the cauline share as
//! the CPU does, one law on both executors (host, 2026-10-14). A crown
//! beside cauline wood (the birch with twelve fronds, which the device drew
//! before fn-277), a fractional leaflet count on the oak's wood, and three
//! frames of the walk from the oak to the date palm: the device stations
//! each run of wood and each short shoot as the CPU does (within the
//! station agreement's tolerance), and places as many leaflets in the
//! crown, last, where the CPU places them.
use super::super::{io, Generator};
use super::stations::GW;
use crate::Renderer;
use std::collections::{BTreeSet, HashMap};
use telperion_core::{blend, pipeline::executor, presets::by_identity, presets::Family};

/// Young, as the rest of the gate's agreement is held.
const AGE: f64 = 12.0;

fn preset(id: &str) -> Family {
    let mut f = by_identity(id).unwrap();
    f.age = AGE;
    f
}

/// The families the host named: each with a name for the report.
fn families() -> Vec<(String, Family)> {
    let mut crowned = preset("silver-birch");
    crowned.canopy.rosette_fronds = 12.0;
    // The oak's rachis is none, which strings no leaflets; a short one
    // lets its fractional count draw them.
    let mut leaflets = preset("oregon-white-oak");
    (leaflets.canopy.leaflet_count, leaflets.canopy.rachis_length) = (1.3, 0.1);
    let mut out = vec![
        ("birch, twelve fronds".to_string(), crowned),
        ("oak, 1.3 leaflets".to_string(), leaflets),
    ];
    let (oak, palm) = (preset("oregon-white-oak"), preset("date-palm"));
    for t in [0.01, 0.3, 0.9] {
        out.push((
            format!("oak to palm at {t}"),
            blend::families(&oak, &palm, t).unwrap(),
        ));
    }
    out
}

/// The farthest a device leaflet may stand from the CPU's, a share of the
/// tree's height (`agree_tests`' `ROSETTE_APART`).
const APART: f64 = 1e-4;

/// Every family: wood stationed alike run by run (within one run in a
/// hundred, as `every_preset_stations_its_runs_as_the_cpu_does`), every
/// leaf placed, and the crown's leaflets where the CPU's stand.
#[test]
fn the_device_draws_counts_of_organs_as_the_cpu_does() {
    let Some(gpu) = crate::generation::test_gpu() else {
        return;
    };
    for (name, f) in families() {
        agrees(&gpu, &name, &f);
    }
}

fn agrees(gpu: &crate::Gpu, name: &str, f: &Family) {
    let renderer = Renderer::new(gpu.clone(), crate::STILL_FORMAT);
    let gen = Generator::new(&renderer).unwrap();
    let kit = gen.kit();
    let built =
        pollster::block_on(super::build::build(gpu, &gen, &kit.kernels, &kit.tools, f)).unwrap();
    // The device's wood by its runs' keys, and its crown.
    let d = &built.device;
    let raw = io::read(
        gpu,
        &d.buffer,
        ((d.segments as usize * GW).max(4) * 4) as u64,
    )
    .unwrap();
    let (mut dev, mut crown) = (HashMap::<u32, u64>::new(), 0u64);
    for g in bytemuck::cast_slice::<u8, u32>(&raw).chunks(GW) {
        match f32::from_bits(g[21]) < -1.5 {
            true => crown += u64::from(g[25]),
            false => *dev.entry(g[27]).or_default() += u64::from(g[25]),
        }
    }
    // The CPU's wood: the same grown tree, its crown set aside. Not grown
    // again: a crown establishes its stem as it grows (fn-275).
    let grown = executor::grow(f).unwrap().expansion().unwrap();
    let mut wood = f.clone();
    wood.canopy.rosette_fronds = 0.0;
    let x = executor::expand(grown.tree().clone(), &wood).unwrap();
    let (_, shared) = x.curve_with_contacts().unwrap();
    let mut cpu = HashMap::<u32, u64>::new();
    for s in &x
        .compact_stations(&shared)
        .unwrap()
        .expect("wood the plan stations")
        .segments
    {
        *cpu.entry(s.key).or_default() += u64::from(s.count);
    }
    let keys: BTreeSet<u32> = dev.keys().chain(cpu.keys()).copied().collect();
    let differ = keys.iter().filter(|k| dev.get(k) != cpu.get(k)).count();
    let (dev_wood, cpu_wood): (u64, u64) = (dev.values().sum(), cpu.values().sum());
    // Every leaf, and the crown's last on both.
    let resident = built.prepared.resident.as_ref().unwrap();
    let leaves = pollster::block_on(io::read_leaves_async(
        gpu,
        resident.leaves.buffer(),
        resident.count,
    ))
    .unwrap();
    let mine = built.prepared.mesh.foliage.instances.reference;
    let theirs = grown.mesh().unwrap().foliage.instances;
    let height = theirs.reference.extent.y.max(1e-9);
    let tail = |n: usize| n - crown as usize;
    let apart = (leaves[tail(leaves.len())..]
        .iter()
        .zip(&theirs.leaves[tail(theirs.leaves.len())..]))
    .map(|(a, b)| mine.position(*a).distance(theirs.reference.position(*b)) / height)
    .fold(0.0f64, f64::max);
    println!(
        "{name}: wood {dev_wood} / {cpu_wood} on {} runs, {differ} differ; crown {crown}; leaves {} / {}; crown apart {apart:e}",
        keys.len(),
        leaves.len(),
        theirs.leaves.len()
    );
    assert!(differ * 100 <= keys.len(), "{name}: {differ} runs differ");
    // Every leaf is the wood's or the crown's, the crowns alike.
    assert_eq!(
        leaves.len() as u64,
        dev_wood + crown,
        "{name}: the device's leaves"
    );
    assert_eq!(
        theirs.leaves.len() as u64,
        cpu_wood + crown,
        "{name}: the CPU's leaves"
    );
    assert!(apart <= APART, "{name}: the crown {apart} apart");
}
