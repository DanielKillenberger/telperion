//! fn-256 task 3, the render gate's agreement: the device's whole tree
//! against the CPU's reference (the CPU's cycle, converted and drawn
//! through `prepare_whole`), by lineage. Each node the device's dress
//! drew is matched to the CPU conversion's by its phytomer's key (a
//! relay's join by its node key), and each run's stations by its first
//! node's key. Integers exact; floats within the lineage tolerance
//! (fn-256 DESIGN.md 9.1). `AGREE_PRESETS` and `AGREE_AGE` widen them.
use super::super::cycle::tree::{self, Kernels};
use super::super::{io, Generator};
use super::fold::{CA_WORDS, CV};
use super::stations::GW;
use crate::Renderer;
use std::collections::{BTreeSet, HashMap};
use telperion_core::math::Vec3;
use telperion_core::pipeline::executor;
use telperion_core::presets::Family;
use telperion_core::surface::{CurvePoint, CLUSTER_WORDS, POINT_WORDS};

/// The age the gate holds every preset the device draws at: young.
const AGE: u32 = 12;

/// The preset at `age`.
fn family(id: &str, age: u32) -> Family {
    let mut family = telperion_core::presets::by_identity(id).unwrap();
    family.age = f64::from(age);
    family
}

/// The preset at `age`, its retained leaf bases set aside: the dress
/// draws the tree before they are hung (`boots_tests` holds them,
/// DESIGN.md 12.8).
fn bare(id: &str, age: u32) -> Family {
    let mut family = family(id, age);
    family.canopy.leaf_bases = 0;
    family
}

/// Every shipped preset, or `AGREE_PRESETS`'s.
fn presets() -> Vec<String> {
    match std::env::var("AGREE_PRESETS") {
        Ok(ids) => ids.split(',').map(String::from).collect(),
        Err(_) => (telperion_core::presets::CATALOGUE.iter()).map(|c| c.1.to_string()).collect(),
    }
}

fn age() -> u32 {
    std::env::var("AGREE_AGE").ok().and_then(|a| a.parse().ok()).unwrap_or(AGE)
}

/// A node as the device's dress drew it.
#[derive(Debug, Clone, Copy)]
pub(in crate::generation) struct Node {
    pub position: [f32; 3],
    pub radii: [f32; 3],
    pub stem: bool,
    pub key: u32,
    /// Its phytomer's key, or none for a relay's join.
    pub identity: Option<u64>,
    pub parent: Option<u32>,
    /// Its leaves' share of their whole size (fn-270 R5), and its wood's
    /// retentions now and a year before and the share a leaf kept only
    /// then is drawn at (R5a).
    pub leaf: f32,
    pub retention: (u32, u32, f32),
}

/// Every node the device drew for `f`.
fn device_nodes(gpu: &crate::Gpu, k: &Kernels, f: &Family) -> Vec<Node> {
    let growth = executor::growth(f).unwrap();
    let (dressed, _) = pollster::block_on(tree::drawn(gpu, k, &growth)).unwrap();
    drawn_nodes(gpu, &dressed)
}

/// Every node of `dressed`.
pub(in crate::generation) fn drawn_nodes(gpu: &crate::Gpu, dressed: &super::fold::Dressed) -> Vec<Node> {
    let mut waves: Vec<Vec<Vec<u32>>> = Vec::new();
    for w in &dressed.waves {
        let words = w.axes as usize * CA_WORDS + w.phyts as usize * CV;
        let bytes = io::read(gpu, &w.buffer, (words.max(4) * 4) as u64).unwrap();
        let all: &[u32] = bytemuck::cast_slice(&bytes);
        let records = all[w.axes as usize * CA_WORDS..].chunks(CV).map(<[u32]>::to_vec);
        waves.push(records.collect());
    }
    let keys: HashMap<u32, u32> = (waves.iter().flatten())
        .filter(|r| r[6] & 2 != 0)
        .map(|r| (r[14], r[24]))
        .collect();
    let mut out = Vec::new();
    for wave in waves.iter().skip(1) {
        for (i, r) in wave.iter().enumerate() {
            if r[6] & 2 == 0 || r[7] != i as u32 {
                continue;
            }
            let f = |k: usize| f32::from_bits(r[k]);
            out.push(Node {
                position: [f(0), f(1), f(2)],
                radii: [f(3), f(4), f(5)],
                stem: r[6] & 128 != 0,
                key: r[24],
                identity: (r[26] | r[27] != 0).then(|| u64::from(r[26]) | u64::from(r[27]) << 32),
                parent: keys.get(&r[16]).copied(),
                leaf: f(32),
                retention: (r[31], r[33], f(34)),
            });
        }
    }
    out
}

/// How the device's nodes stood against the CPU conversion's: the most
/// apart a position (m) and a radius (m) stood.
#[derive(Debug, Default)]
struct Drawn {
    device: usize,
    cpu: usize,
    matched: usize,
    position: f64,
    radius: f64,
    stems: usize,
    parents: usize,
}

fn drawn(gpu: &crate::Gpu, k: &Kernels, f: &Family) -> Drawn {
    let dev = device_nodes(gpu, k, f);
    let x = executor::grow(f).unwrap().expansion().unwrap();
    let tree = x.tree();
    let (mut by_key, mut by_identity) = (HashMap::new(), HashMap::new());
    for i in 1..tree.nodes.len() {
        by_key.insert(tree.key(i), i);
        by_identity.insert(tree.nodes[i].identity.birth_order(), i);
    }
    let mut out = Drawn { device: dev.len(), cpu: tree.nodes.len() - 1, ..Drawn::default() };
    for d in &dev {
        let found = match d.identity {
            Some(k) => by_identity.get(&k),
            None => by_key.get(&d.key),
        };
        let Some(&i) = found else { continue };
        out.matched += 1;
        let n = &tree.nodes[i];
        let p = [n.position.x, n.position.y, n.position.z];
        let dp = (0..3).map(|c| (f64::from(d.position[c]) - p[c]).abs()).fold(0.0, f64::max);
        let r = [n.radius, n.start_radius, n.base_radius];
        let dr = (0..3).map(|c| (f64::from(d.radii[c]) - r[c]).abs()).fold(0.0, f64::max);
        (out.position, out.radius) = (out.position.max(dp), out.radius.max(dr));
        out.stems += usize::from(d.stem != n.stem);
        let parent = n.parent.filter(|&q| q != 0).map(|q| tree.key(q as usize));
        out.parents += usize::from(n.parent != Some(0) && parent != d.parent);
    }
    out
}

/// Every preset young: each node the device drew is the CPU conversion's,
/// by lineage, within the lineage tolerance.
#[test]
fn every_preset_is_drawn_as_the_cpu_converts_it() {
    let Some(gpu) = crate::generation::test_gpu() else {
        return;
    };
    let k = Kernels::new(&gpu);
    for id in presets() {
        let d = drawn(&gpu, &k, &bare(&id, age()));
        println!("{id} {}: {d:?}", age());
        assert_eq!(d.device, d.cpu, "{id}: nodes");
        // A relay's join keyed by its node's 32-bit key may meet another's.
        assert!(d.matched * 1000 >= d.device * 999, "{id}: matched {d:?}");
        assert!(d.position < 1e-3 && d.radius < 1e-5, "{id}: {d:?}");
        assert_eq!(d.stems, 0, "{id}: stems");
        assert!(d.parents * 1000 <= d.device, "{id}: parents {d:?}");
    }
}

/// The stations the device and the CPU place, and the runs (by the key
/// of each run's first node) whose stations differ in number.
fn stations(gpu: &crate::Gpu, f: &Family) -> (u64, u64, usize) {
    let renderer = Renderer::new(gpu.clone(), crate::STILL_FORMAT);
    let gen = Generator::new(&renderer).unwrap();
    let kit = gen.kit();
    let built = pollster::block_on(super::build::build(gpu, &gen, &kit.kernels, &kit.tools, f)).unwrap();
    let d = &built.device;
    let words = d.segments as usize * GW;
    let raw = io::read(gpu, &d.buffer, (words.max(4) * 4) as u64).unwrap();
    let mut dev: HashMap<u32, u64> = HashMap::new();
    for g in bytemuck::cast_slice::<u8, u32>(&raw).chunks(GW) {
        *dev.entry(g[27]).or_default() += u64::from(g[25]);
    }
    let x = executor::grow(f).unwrap().expansion().unwrap();
    let (_, shared) = x.curve_with_contacts().unwrap();
    // A rosette's leaves stand on no station of the CPU's: none to count
    // against (`every_rosette_is_placed_as_the_cpu_places_it`).
    let Some(theirs) = x.compact_stations(&shared).unwrap() else {
        return (0, 0, 0);
    };
    let mut cpu: HashMap<u32, u64> = HashMap::new();
    for s in &theirs.segments {
        *cpu.entry(s.key).or_default() += u64::from(s.count);
    }
    let keys: BTreeSet<u32> = dev.keys().chain(cpu.keys()).copied().collect();
    let differ = keys.iter().filter(|k| dev.get(k) != cpu.get(k)).count();
    (dev.values().sum(), cpu.values().sum(), differ)
}

/// Every preset young: the device stations its runs as the CPU does.
#[test]
fn every_preset_stations_its_runs_as_the_cpu_does() {
    let Some(gpu) = crate::generation::test_gpu() else {
        return;
    };
    for id in presets() {
        let (dev, cpu, differ) = stations(&gpu, &family(&id, age()));
        println!("{id} {}: stations {dev} / {cpu}, runs differing {differ}", age());
        assert_eq!(dev, cpu, "{id}: stations");
        assert!(differ * 100 <= cpu as usize, "{id}: {differ} runs differ");
    }
}

/// Every preset whose canopy stands a rosette: the device places every
/// leaflet where the CPU does, in the CPU's order (apex, frond, leaflet),
/// within the lineage tolerance, and withers the same ones. The draws
/// are each executor's own, as a station's are.
#[test]
fn every_rosette_is_placed_as_the_cpu_places_it() {
    let Some(gpu) = crate::generation::test_gpu() else {
        return;
    };
    for id in presets() {
        let f = family(&id, age());
        if f.canopy.rosette_fronds == 0 {
            continue;
        }
        let renderer = Renderer::new(gpu.clone(), crate::STILL_FORMAT);
        let gen = Generator::new(&renderer).unwrap();
        let kit = gen.kit();
        let built = pollster::block_on(super::build::build(&gpu, &gen, &kit.kernels, &kit.tools, &f)).unwrap();
        let resident = built.prepared.resident.as_ref().unwrap();
        let leaves = pollster::block_on(io::read_leaves_async(&gpu, resident.leaves.buffer(), resident.count)).unwrap();
        let mine = built.prepared.mesh.foliage.instances.reference;
        let theirs = executor::grow(&f).unwrap().expansion().unwrap().mesh().unwrap().foliage.instances;
        assert_eq!(leaves.len(), theirs.leaves.len(), "{id}: leaves");
        let height = theirs.reference.extent.y.max(1e-9);
        let (mut apart, mut withered) = (0.0f64, 0usize);
        for (a, b) in leaves.iter().zip(&theirs.leaves) {
            apart = apart.max(mine.position(*a).distance(theirs.reference.position(*b)) / height);
            withered += usize::from(telperion_core::foliage::withered(*a) != telperion_core::foliage::withered(*b));
        }
        // The sizes are drawn on each executor's stream: alike in the mean.
        let mean = |s: &mut dyn Iterator<Item = f64>| s.sum::<f64>() / leaves.len() as f64;
        let size = mean(&mut leaves.iter().map(|l| mine.scale(*l)));
        let theirs_size = mean(&mut theirs.leaves.iter().map(|l| theirs.reference.scale(*l)));
        let sized = (size / theirs_size - 1.0).abs();
        println!("{id} {}: {} leaves, apart {apart:e} of the height, withering differs on {withered}, mean size apart {sized:e}", age(), leaves.len());
        assert_eq!(withered, 0, "{id}: withered");
        assert!(apart <= ROSETTE_APART, "{id}: {apart}");
        assert!(sized <= 0.02, "{id}: mean size {size} against {theirs_size}");
    }
}

/// The farthest a device leaflet may stand from the CPU's, a share of the
/// tree's height: each box quantises a position to 1/65,535 of its extent
/// (1.5e-5 of the height each), and the trees agree by lineage. Measured
/// 3.2e-5 at 12 and 4.1e-5 at 40 on the date palm; about twice that.
const ROSETTE_APART: f64 = 1e-4;

/// The device's curve against the CPU's, run by run (matched at their
/// first point within a millimetre): runs matched, unmatched, with their
/// points differing in number, and the most apart a matched point stood
/// (m) and its radius (relative). Read through its `Debug` print.
#[allow(dead_code)]
#[derive(Debug, Default)]
struct Runs {
    runs: (usize, usize),
    matched: usize,
    unmatched: usize,
    count: usize,
    /// The CPU's trunk runs the device drew point for point, of all.
    trunks: (usize, usize),
    centre: f64,
    radius: f64,
}

/// A millimetre's cell at the scale `s` the tree is drawn at.
fn cell(v: Vec3, s: f64) -> (i64, i64, i64) {
    let c = |x: f64| (x / (1e-3 * s)).floor() as i64;
    (c(v.x), c(v.y), c(v.z))
}

fn curve(gpu: &crate::Gpu, f: &Family) -> Runs {
    let renderer = Renderer::new(gpu.clone(), crate::STILL_FORMAT);
    let gen = Generator::new(&renderer).unwrap();
    let kit = gen.kit();
    let built = pollster::block_on(super::build::build(gpu, &gen, &kit.kernels, &kit.tools, f)).unwrap();
    let s = executor::growth(f).unwrap().scale;
    let wood = built.prepared.wood.as_ref().unwrap();
    let (points, clusters, p, k, _) = wood.blocks();
    let read = |b: &wgpu::Buffer, n: u64| -> Vec<u32> {
        bytemuck::cast_slice(&io::read(gpu, b, (n.max(1) * 4).next_multiple_of(4)).unwrap()).to_vec()
    };
    let points = read(points, p * POINT_WORDS as u64);
    let clusters = read(clusters, u64::from(k) * CLUSTER_WORDS as u64);
    let point = |i: u32| CurvePoint::unpack(points[i as usize * POINT_WORDS..][..POINT_WORDS].try_into().unwrap());
    let mut runs: Vec<(u32, u32)> = Vec::new();
    for c in clusters.chunks(CLUSTER_WORDS) {
        if runs.last() != Some(&(c[2], c[3])) {
            runs.push((c[2], c[3]));
        }
    }
    let theirs = executor::grow(f).unwrap().expansion().unwrap().curve().unwrap();
    let mut by: HashMap<(i64, i64, i64), Vec<usize>> = HashMap::new();
    for (j, r) in theirs.runs.iter().enumerate().filter(|(_, r)| r.count > 1) {
        by.entry(cell(theirs.points[r.first as usize + 1].centre, s)).or_default().push(j);
    }
    let trunks = theirs.runs.iter().filter(|r| r.trunk).count();
    let mut out = Runs { runs: (runs.len(), theirs.runs.len()), trunks: (0, trunks), ..Runs::default() };
    let mut used = vec![false; theirs.runs.len()];
    for &(first, count) in runs.iter().filter(|r| r.1 > 1) {
        let at = point(first + 1).centre;
        let (x, y, z) = cell(at, s);
        let near = (-1..=1).flat_map(|a| (-1..=1).flat_map(move |b| (-1..=1).map(move |c| (x + a, y + b, z + c))));
        let off = |j: usize| theirs.points[theirs.runs[j].first as usize + 1].centre.distance(at);
        let found = (near.flat_map(|c| by.get(&c).into_iter().flatten().copied()))
            .filter(|&j| !used[j] && off(j) <= 1e-3 * s)
            .min_by(|&a, &b| off(a).total_cmp(&off(b)));
        let Some(j) = found else {
            out.unmatched += 1;
            continue;
        };
        used[j] = true;
        let want = theirs.run_points(&theirs.runs[j]);
        if want.len() != count as usize {
            out.count += 1;
            continue;
        }
        out.matched += 1;
        out.trunks.0 += usize::from(theirs.runs[j].trunk);
        for (i, c) in want.iter().enumerate() {
            let m = point(first + i as u32);
            out.centre = out.centre.max(m.centre.distance(c.centre));
            out.radius = out.radius.max((m.radius - c.radius).abs() / c.radius.max(1e-9));
        }
    }
    out
}

/// Every preset: the device's curve draws by the section's rows the CPU's
/// does (its lobes, their depth, the twist and the height).
#[test]
fn every_preset_draws_its_wood_by_the_cpu_rows() {
    let Some(gpu) = crate::generation::test_gpu() else {
        return;
    };
    for id in presets() {
        let f = family(&id, age());
        let renderer = Renderer::new(gpu.clone(), crate::STILL_FORMAT);
        let gen = Generator::new(&renderer).unwrap();
        let kit = gen.kit();
        let built = pollster::block_on(super::build::build(&gpu, &gen, &kit.kernels, &kit.tools, &f)).unwrap();
        let theirs = executor::grow(&f).unwrap().expansion().unwrap().curve().unwrap();
        let want = [theirs.lobe_depth as f32, theirs.twist_rate as f32, theirs.height as f32, 0.0];
        let (shape, lobes) = built.prepared.wood.as_ref().unwrap().rows();
        assert_eq!((shape[0], shape[1], lobes), (want[0], want[1], theirs.lobes), "{id}");
        // The height is the tree's: the device's tree and the CPU's agree
        // by lineage.
        assert!((shape[2] - want[2]).abs() <= 1e-3 * want[2], "{id}: {shape:?} {want:?}");
    }
}

/// Every preset young: the device sweeps its wood as the CPU does, run
/// for run, point for point within the lineage tolerance.
#[test]
fn every_preset_sweeps_its_wood_as_the_cpu_does() {
    let Some(gpu) = crate::generation::test_gpu() else {
        return;
    };
    for id in presets() {
        let r = curve(&gpu, &family(&id, age()));
        println!("{id} {}: {r:?}", age());
        assert!(r.unmatched + r.count <= r.matched / 100, "{id}: {r:?}");
        // The tolerance above never covers a trunk: one run, all the flare.
        assert_eq!(r.trunks.0, r.trunks.1, "{id}: {r:?}");
        assert!(r.centre < 1e-3 && r.radius < 1e-3, "{id}: {r:?}");
    }
}
