//! fn-277: the device draws the counts of organs and the cauline share as
//! the CPU does, one law on both executors (host, 2026-10-14). A crown
//! beside cauline wood (the birch with twelve fronds, which the device drew
//! before fn-277), a fractional leaflet count on the oak's wood, and three
//! frames of the walk from the oak to the date palm: the device stations
//! each run of wood and each short shoot as the CPU does (within the
//! station agreement's tolerance), and places as many leaflets in the
//! crown, last, where the CPU places them, and its wood's leaves where the
//! CPU's stand: a compound leaf on wood as one tuft at its point (fn-309).
use super::super::{io, Generator};
use super::build::instances;
use super::stations::GW;
use crate::Renderer;
use std::collections::{BTreeSet, HashMap};
use telperion_core::foliage::{Leaf, Reference};
use telperion_core::math::Vec3;
use telperion_core::{
    blend, pipeline::executor, presets::by_identity, presets::Family, tree::cluster_key,
};

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
    // The film's frame 526 (fn-309): the oak's wood bearing compound
    // leaves of about five leaflets on a 2.4 m rachis nobody draws.
    let film = |mut f: Family| {
        (f.age, f.skeleton.seed) = (40.0, 7);
        f
    };
    let frame = blend::families(&film(oak), &film(palm), 0.34).unwrap();
    out.push(("oak to palm at 0.34, 40, seed 7".to_string(), frame));
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

/// The wood's leaves by their runs' keys, the device's then the CPU's.
type ByKey = HashMap<u32, u64>;

fn agrees(gpu: &crate::Gpu, name: &str, f: &Family) -> (ByKey, ByKey) {
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
    let wood = |n: usize, l: &[Leaf]| l[..tail(n)].to_vec();
    let (mine_wood, theirs_wood) = (
        wood(leaves.len(), &leaves),
        wood(theirs.leaves.len(), &theirs.leaves),
    );
    stand(
        name,
        f,
        (&mine, &mine_wood),
        (&theirs.reference, &theirs_wood),
    );
    (dev, cpu)
}

/// fn-309 R2: each leaf on the device's wood stands where one of the
/// CPU's does, within `APART` of the height, and turns from the nearest
/// in heading as little as the crown's do (each executor scatters its
/// own). A compound leaf on wood stands its leaflets at its point on
/// both, as one tuft; a device stringing them along the undrawn rachis
/// leaves them off every CPU leaf. Leaves are matched by place, not by
/// order: wood is stationed alike run by run within one run in a
/// hundred, so as many leaves may stand unmatched. Measured: none off
/// on every family but the film's frame, whose one placement at the
/// stem's top (y 9.52 m) stands 0.44 m from the CPU's with its five
/// leaflets, and as far with one leaflet a placement: older than fn-309.
fn stand(name: &str, f: &Family, (mine, dev): Shelf, (theirs, cpu): Shelf) {
    let tol = APART * theirs.extent.y.max(1e-9);
    let cell = |v: Vec3| [v.x, v.y, v.z].map(|c| (c / tol).floor() as i64);
    let mut grid: HashMap<[i64; 3], Vec<Leaf>> = HashMap::new();
    for l in cpu {
        grid.entry(cell(theirs.position(*l))).or_default().push(*l);
    }
    let axis = |r: &Reference, l: Leaf| {
        let m = r.unpack(l);
        Vec3::new(m[4].into(), m[5].into(), m[6].into()).normalized()
    };
    let (mut unmatched, mut turns) = (0usize, Vec::new());
    for l in dev {
        let (at, [x, y, z]) = (mine.position(*l), cell(mine.position(*l)));
        let near: Vec<Leaf> = (0..27)
            .filter_map(|n| grid.get(&[x + n % 3 - 1, y + n / 3 % 3 - 1, z + n / 9 - 1]))
            .flatten()
            .filter(|c| theirs.position(**c).distance(at) <= tol)
            .copied()
            .collect();
        let turn = near
            .iter()
            .map(|c| axis(mine, *l).dot(axis(theirs, *c)).clamp(-1.0, 1.0).acos())
            .fold(f64::INFINITY, f64::min);
        match turn.is_finite() {
            true => turns.push(turn.to_degrees()),
            false => unmatched += 1,
        }
    }
    let turned = turns.iter().sum::<f64>() / turns.len().max(1) as f64;
    println!(
        "{name}: wood leaves {} / {}, {unmatched} off every CPU leaf, turned {turned:.1} degrees",
        dev.len(),
        cpu.len()
    );
    assert!(unmatched * 100 <= dev.len(), "{name}: {unmatched} off");
    assert!(
        turned <= 0.8 * f.canopy.scatter + 1.0,
        "{name}: turned {turned} degrees"
    );
}

/// One executor's leaves and the box they are stored in.
type Shelf<'a> = (&'a Reference, &'a [Leaf]);

/// fn-297 R2: a stem's tip that is both a short shoot's tip and a
/// rosette's apex bears both, as the CPU places them: its cluster among
/// the wood's, its rosette in the crown. The date palm at 40 with its
/// trunk's age clustering one leaf at the cauline share whole reaches it;
/// codex's case set the chain's first age, which the palm passes through,
/// and reached no cluster. The plain palm is the control.
#[test]
fn a_tip_that_is_both_bears_its_cluster_and_its_rosette() {
    let Some(gpu) = crate::generation::test_gpu() else {
        return;
    };
    let mut plain = by_identity("date-palm").unwrap();
    plain.age = 40.0;
    let mut both = plain.clone();
    both.canopy.cauline = 1.0;
    let species = both.space.species.as_mut().expect("an engine species");
    // The palm's one age is the chain's trunk (`chain::TRUNK`).
    species.states[2].cluster_leaves = 1.0;
    // The state is reached: a cluster's tip is a stem's apex.
    let grown = executor::grow(&both).unwrap().expansion().unwrap();
    let tree = grown.tree();
    let apices = tree.stem_apices();
    let tips: Vec<u32> = (tree.clusters.iter())
        .map(|c| c.node as usize)
        .filter(|n| apices.contains(n))
        .map(|n| cluster_key(tree.key(n)))
        .collect();
    assert!(!tips.is_empty(), "no cluster's tip is a stem's apex");
    agrees(&gpu, "date palm", &plain);
    let (dev, cpu) = agrees(&gpu, "date palm, a tip that is both", &both);
    for key in tips {
        assert!(cpu.contains_key(&key), "the CPU's cluster at the apex");
        assert_eq!(dev.get(&key), cpu.get(&key), "the apex's cluster");
    }
}

/// fn-297 R4 B.3: a leaf count past 32 bits, which the stations count in
/// a 64-bit total, is refused by name, whatever the budget; one below
/// passes. No allowed parameter reaches 2^32 leaves on a device.
#[test]
fn a_leaf_count_past_u32_is_refused_by_name() {
    let below = u64::from(u32::MAX);
    assert_eq!(instances(below, usize::MAX).unwrap(), u32::MAX);
    for count in [below + 1, u64::MAX] {
        let error = instances(count, usize::MAX).unwrap_err().to_string();
        assert!(error.contains("foliage instance budget"), "{error}");
    }
    assert_eq!(instances(7, 7).unwrap(), 7);
    assert!(instances(8, 7).is_err(), "past the canopy's budget");
}
