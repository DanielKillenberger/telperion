//! fn-261 R1: the same binary builds the same tree twice. The spruce at
//! its default age and seed, built on the real path in two fresh
//! contexts (device, generator, kernels and tools each made anew), keeps
//! its grown tree and its prepared leaves byte for byte.
use super::super::{io, Generator};
use super::placed_tests::device_segments;
use super::resident::{build, Tools};
use super::roots::kernels;
use telperion_core::presets::Family;

/// What one build hands the renderer, as bytes: the pipeline's tree, the
/// station segments of both blocks, the leaves and their masses.
#[derive(PartialEq)]
struct Snapshot {
    tree: String,
    segments: String,
    leaves: Vec<[u32; 3]>,
    masses: Vec<u8>,
}

/// `family` built once in a context of its own.
fn snapshot(family: &Family) -> Option<Snapshot> {
    let gpu = crate::generation::test_gpu()?;
    let make = Generator::create(gpu.clone(), std::sync::Arc::new(()), true);
    let gen = pollster::block_on(make).unwrap();
    let (k, tools) = (kernels(&gpu), Tools::new(&gpu));
    let built = pollster::block_on(build(&gpu, &gen, &k, &tools, family)).unwrap();
    let resident = built.prepared.resident.as_ref().unwrap();
    let leaves = io::read_leaves_async(&gpu, resident.leaves.buffer(), resident.count);
    let leaves = pollster::block_on(leaves).unwrap();
    let first = leaves.len() as u32 - built.device.stations;
    let device = device_segments(&gpu, &built.device, first);
    let cpu = built.cpu_segments.iter().map(|s| format!("{s:?}"));
    let device = device
        .iter()
        .map(|s| format!("{:?} {:?} {:?}", s.words, s.start, s.end));
    let masses = resident.masses.buffer();
    Some(Snapshot {
        tree: format!("{:?}", built.expansion.tree().nodes),
        segments: cpu.chain(device).collect::<Vec<_>>().join("\n"),
        leaves,
        masses: io::read(&gpu, masses, masses.size()).unwrap(),
    })
}

/// Which parts of two builds differ, by name.
fn differing(a: &Snapshot, b: &Snapshot) -> Vec<&'static str> {
    [
        ("tree", a.tree == b.tree),
        ("segments", a.segments == b.segments),
        ("leaves", a.leaves == b.leaves),
        ("masses", a.masses == b.masses),
    ]
    .into_iter()
    .filter_map(|(name, same)| (!same).then_some(name))
    .collect()
}

/// The spruce at its default age and seed, twice.
#[test]
fn the_spruce_builds_the_same_leaves_twice() {
    let family = telperion_core::presets::by_identity("norway-spruce").unwrap();
    let Some(first) = snapshot(&family) else {
        return;
    };
    let second = snapshot(&family).unwrap();
    assert_eq!(differing(&first, &second), Vec::<&str>::new());
}
