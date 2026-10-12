//! fn-261 R1, fn-256 R2a: the same binary builds the same tree twice.
//! Every preset the device draws, built resident in two fresh contexts
//! (device, generator, kernels and tools each made anew), keeps its
//! station segments, leaves and masses byte for byte.
use super::super::{io, Generator};
use super::build::{build, Tools};
use super::super::cycle::tree::Kernels;
use super::stations::GW;
use telperion_core::presets::Family;

/// What one build hands the renderer, as bytes: the station segments,
/// the leaves and their masses.
#[derive(PartialEq)]
struct Snapshot {
    segments: Vec<u8>,
    leaves: Vec<[u32; 3]>,
    masses: Vec<u8>,
}

/// `family` built once in a context of its own.
fn snapshot(family: &Family) -> Option<Snapshot> {
    let gpu = crate::generation::test_gpu()?;
    let make = Generator::create(gpu.clone(), std::sync::Arc::new(()), true);
    let gen = pollster::block_on(make).unwrap();
    let (k, tools) = (Kernels::new(&gpu), Tools::new(&gpu));
    let built = pollster::block_on(build(&gpu, &gen, &k, &tools, family)).unwrap();
    let resident = built.prepared.resident.as_ref().unwrap();
    let leaves = io::read_leaves_async(&gpu, resident.leaves.buffer(), resident.count);
    let leaves = pollster::block_on(leaves).unwrap();
    let d = &built.device;
    let segments = io::read(&gpu, &d.buffer, (d.segments as usize * GW * 4).max(16) as u64).unwrap();
    let masses = resident.masses.buffer();
    Some(Snapshot {
        segments,
        leaves,
        masses: io::read(&gpu, masses, masses.size()).unwrap(),
    })
}

/// Which parts of two builds differ, by name.
fn differing(a: &Snapshot, b: &Snapshot) -> Vec<&'static str> {
    [
        ("segments", a.segments == b.segments),
        ("leaves", a.leaves == b.leaves),
        ("masses", a.masses == b.masses),
    ]
    .into_iter()
    .filter_map(|(name, same)| (!same).then_some(name))
    .collect()
}

/// Every shipped preset the device draws builds the same tree each time,
/// at its default age and seed: twice here, `REPEATS` times where it is
/// set (`REPEATS=10`, under the GPU lock, for the evidence).
#[test]
fn every_device_preset_builds_the_same_leaves_each_time() {
    let repeats: usize = std::env::var("REPEATS").map_or(2, |r| r.parse().unwrap());
    for &(_, id, _, _) in telperion_core::presets::CATALOGUE {
        let family = telperion_core::presets::by_identity(id).unwrap();
        if !telperion_core::pipeline::executor::stations(&family) {
            continue;
        }
        let Some(first) = snapshot(&family) else {
            return;
        };
        for k in 1..repeats {
            let again = snapshot(&family).unwrap();
            assert_eq!(differing(&first, &again), Vec::<&str>::new(), "{id}: build {k}");
        }
        println!("{id}: {repeats} builds alike, {} leaves", first.leaves.len());
    }
}
