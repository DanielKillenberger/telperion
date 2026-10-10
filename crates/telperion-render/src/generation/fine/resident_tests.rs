//! fn-212 task 7: final-output parity of a tree built on the real path
//! (`resident::build`) against the CPU path, on the buffers the submit
//! takes, by named class:
//! - **nodes:** the pipeline's part of the tree against the whole tree's
//!   same nodes (by key and place): kind and codominance, which read a
//!   node's children, and position;
//! - **curve:** the renderer's one curve block, run by run against the
//!   reference curve's (matched at their first node), point for point
//!   within fn-125's screen, and its demand;
//! - **leaves:** the one leaf buffer, leaf by leaf (`placed_tests`).
use super::super::{io, Generator};
use super::placed_tests::{compare_leaves, device_segments, reference, Seated, Seg};
use super::resident::{build, Built, Tools};
use super::roots::{kernels, Kernels};
use crate::Gpu;
use std::collections::HashMap;
use telperion_core::math::Vec3;
use telperion_core::pipeline::executor;
use telperion_core::presets::Family;
use telperion_core::surface::{Curve, CurvePoint, CLUSTER_WORDS, POINT_WORDS};
use telperion_core::tree::{NodeKind, Tree};

/// fn-125's screen: metres, and radians for a frame.
const MOST: f64 = 50e-6;
const TURN: f64 = 0.01;

/// The pipeline's part's nodes against the whole tree's.
#[allow(dead_code)]
#[derive(Debug, Default)]
struct Nodes {
    /// The part's nodes, those with no whole-tree node at their key and
    /// place, and those placed off the screen.
    nodes: (usize, usize, usize),
    /// Kinds apart, and of them a whole-tree node whose children are all
    /// the device's (the seam's own class).
    kinds: (usize, usize),
    /// Codominance apart (one has it, the other not); and the part's
    /// nodes whose whole-tree twin bears a codominant child the device
    /// draws, whose fork the sweep eases by it (the seam's own class).
    codominance: (usize, usize),
    /// The farthest apart (m).
    position: f64,
}

/// The curve's runs by class.
#[allow(dead_code)]
#[derive(Debug, Default)]
struct Runs {
    /// The submitted runs: the CPU block's and the device's.
    runs: (usize, usize),
    /// Matched within the screen, point for point.
    within: usize,
    /// Matched, apart at the socket's sample alone (TASK5.md residual 2).
    socket: usize,
    /// Matched, apart beyond the socket's sample.
    parted: usize,
    /// Matched at their first node, their points differing in number (a
    /// threshold decision: the f32-against-f64 class).
    count: usize,
    /// With no reference run at their first node; the reference's runs
    /// matched by none.
    unmatched: usize,
    missing: usize,
    /// The most apart within: centre (m), radius (relative), distance
    /// along (m) and frame (rad); frames over `TURN`.
    centre: f64,
    radius: f64,
    along: f64,
    turn: f64,
    turned: usize,
    /// Matched runs whose radius is off by more than 1e-3 somewhere: those
    /// through a bearer whose fine root the device draws as codominant
    /// (the sweep's fork easing there reads it), and the rest; the most
    /// apart of the rest (relative).
    radii: (usize, usize, f64),
    /// The demand's sums apart: clusters, stretches, and the root sag
    /// (relative).
    demand: (i64, i64, f64),
}

/// What the parity probe measured on one tree.
#[allow(dead_code)]
#[derive(Debug)]
struct Parity {
    nodes: Nodes,
    runs: Runs,
    leaves: Seated,
}

/// A millimetre's cell at the scale `s` the tree is drawn at.
fn cell(v: Vec3, s: f64) -> (i64, i64, i64) {
    let c = |x: f64| (x / (1e-3 * s)).floor() as i64;
    (c(v.x), c(v.y), c(v.z))
}

fn near<'a, T: Copy>(
    map: &'a HashMap<(i64, i64, i64), Vec<T>>,
    (at, s): (Vec3, f64),
) -> impl Iterator<Item = T> + 'a {
    let (x, y, z) = cell(at, s);
    (-1..=1)
        .flat_map(move |a| (-1..=1).flat_map(move |b| (-1..=1).map(move |c| (x + a, y + b, z + c))))
        .flat_map(|c| map.get(&c).into_iter().flatten().copied())
}

/// The part's nodes against the whole tree's (`fine`: the whole tree's
/// nodes the device draws), the screen at the scale `s` they are drawn at.
fn nodes(
    (part, whole, fine): (&Tree, &Tree, &[bool]),
    s: f64,
) -> (Nodes, HashMap<(i64, i64, i64), Vec<Vec3>>) {
    let mut by: HashMap<u32, Vec<usize>> = HashMap::new();
    for i in 0..whole.nodes.len() {
        by.entry(whole.key(i)).or_default().push(i);
    }
    let mut children = vec![(0usize, 0usize); whole.nodes.len()];
    let mut bears = vec![false; whole.nodes.len()];
    for (i, n) in whole.nodes.iter().enumerate().skip(1) {
        let p = n.parent.unwrap() as usize;
        children[p].0 += 1;
        children[p].1 += usize::from(fine[i]);
        bears[p] |= fine[i] && n.codominant.is_some();
    }
    let mut out = Nodes {
        nodes: (part.nodes.len(), 0, 0),
        ..Nodes::default()
    };
    for (i, n) in part.nodes.iter().enumerate() {
        let found = by.get(&part.key(i)).and_then(|c| {
            let off = |&j: &usize| whole.nodes[j].position.distance(n.position);
            c.iter().copied().min_by(|a, b| off(a).total_cmp(&off(b)))
        });
        let Some(j) = found else {
            out.nodes.1 += 1;
            continue;
        };
        let w = &whole.nodes[j];
        let off = w.position.distance(n.position);
        out.position = out.position.max(off);
        out.nodes.2 += usize::from(off > MOST * s);
        if w.kind != n.kind {
            out.kinds.0 += 1;
            let (all, theirs) = children[j];
            out.kinds.1 += usize::from(all > 0 && all == theirs && n.kind == NodeKind::Twig);
        }
        if w.codominant.is_some() != n.codominant.is_some() {
            out.codominance.0 += 1;
        }
        out.codominance.1 += usize::from(bears[j]);
    }
    let mut forks: HashMap<(i64, i64, i64), Vec<Vec3>> = HashMap::new();
    for (j, _) in bears.iter().enumerate().filter(|(_, &b)| b) {
        let at = whole.nodes[j].position;
        forks.entry(cell(at, s)).or_default().push(at);
    }
    (out, forks)
}

fn words(gpu: &Gpu, b: &wgpu::Buffer, n: u64) -> Vec<u32> {
    let raw = io::read(gpu, b, (n.max(1) * 4).next_multiple_of(4)).unwrap();
    bytemuck::cast_slice(&raw).to_vec()
}

/// The submitted curve's runs, by class, against the reference's, the
/// screen at the scale `s` they are drawn at.
fn curve(
    gpu: &Gpu,
    built: &Built,
    (theirs, forks): (&Curve, &HashMap<(i64, i64, i64), Vec<Vec3>>),
    s: f64,
) -> Runs {
    let most = MOST * s;
    let wood = built.prepared.wood.as_ref().unwrap();
    let (points, clusters, p, k, demand) = wood.blocks();
    let points = words(gpu, points, p * POINT_WORDS as u64);
    let clusters = words(gpu, clusters, u64::from(k) * CLUSTER_WORDS as u64);
    let point = |i: u32| {
        let w: &[u32; POINT_WORDS] = points[i as usize * POINT_WORDS..][..POINT_WORDS]
            .try_into()
            .unwrap();
        CurvePoint::unpack(w)
    };
    // Each run once, from its clusters' run words.
    let mut runs: Vec<(u32, u32)> = Vec::new();
    for c in clusters.chunks(CLUSTER_WORDS) {
        if runs.last() != Some(&(c[2], c[3])) {
            runs.push((c[2], c[3]));
        }
    }
    let cpu = built.prepared.mesh.curve.runs.len();
    let mut by: HashMap<(i64, i64, i64), Vec<usize>> = HashMap::new();
    for (j, r) in theirs.runs.iter().enumerate().filter(|(_, r)| r.count > 1) {
        let at = theirs.points[r.first as usize + 1].centre;
        by.entry(cell(at, s)).or_default().push(j);
    }
    let mut used = vec![false; theirs.runs.len()];
    let mut out = Runs {
        runs: (cpu, runs.len() - cpu),
        ..Runs::default()
    };
    for &(first, count) in &runs {
        if count < 2 {
            continue;
        }
        let at = point(first + 1).centre;
        let off = |j: usize| {
            theirs.points[theirs.runs[j].first as usize + 1]
                .centre
                .distance(at)
        };
        let found = near(&by, (at, s))
            .filter(|&j| !used[j] && off(j) <= most)
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
        let apart: Vec<bool> = (want.iter().enumerate())
            .map(|(i, c)| point(first + i as u32).centre.distance(c.centre) > most)
            .collect();
        if apart[1..].iter().any(|&a| a) {
            out.parted += 1;
            continue;
        }
        out.socket += usize::from(apart[0]);
        out.within += usize::from(!apart[0]);
        let rel = |i: usize| {
            let r = want[i].radius;
            (point(first + i as u32).radius - r).abs() / r.max(1e-9)
        };
        let worst = (0..want.len()).map(rel).fold(0.0, f64::max);
        if worst > 1e-3 {
            let forked = want
                .iter()
                .any(|c| near(forks, (c.centre, s)).any(|f: Vec3| f.distance(c.centre) <= most));
            if forked {
                out.radii.0 += 1;
            } else {
                out.radii.1 += 1;
                out.radii.2 = out.radii.2.max(worst);
            }
            continue;
        }
        let mut turned = false;
        for (i, c) in want.iter().enumerate().skip(usize::from(apart[0])) {
            let m = point(first + i as u32);
            out.centre = out.centre.max(m.centre.distance(c.centre));
            out.radius = out
                .radius
                .max((m.radius - c.radius).abs() / c.radius.max(1e-9));
            out.along = out.along.max((m.along - c.along).abs());
            let angle = |a: Vec3, b: Vec3| a.dot(b).clamp(-1.0, 1.0).acos();
            let turn = angle(m.normal, c.normal).max(angle(m.binormal, c.binormal));
            out.turn = out.turn.max(turn);
            turned |= turn > TURN;
        }
        out.turned += usize::from(turned);
    }
    out.missing = (theirs.runs.iter().zip(&used))
        .filter(|(r, &u)| r.count > 1 && !u)
        .count();
    let want = theirs.demand();
    out.demand = (
        demand.clusters as i64 - want.clusters as i64,
        demand.stretches as i64 - want.stretches as i64,
        (demand.root_sag - want.root_sag) / want.root_sag.max(1e-12),
    );
    out
}

/// The whole tree's nodes the device draws: every node of a fine root
/// not laid on the CPU, by the roots `built` laid there.
fn fine_nodes(whole: &Tree, part: &Tree) -> Vec<bool> {
    let landed: std::collections::HashSet<u64> = (part.attachments.iter())
        .filter(|a| !a.seams.is_empty())
        .map(|a| a.lineage)
        .collect();
    let mut fine = vec![false; whole.nodes.len()];
    let kept = (whole.attachments.iter()).filter(|a| !landed.contains(&a.lineage));
    for &s in kept.flat_map(|a| &a.seams) {
        fine[s as usize] = true;
    }
    for i in 1..whole.nodes.len() {
        fine[i] |= fine[whole.nodes[i].parent.unwrap() as usize];
    }
    fine
}

/// `family` on the real path against the CPU path, the reference's
/// resident leaves made first so the two never share the device's memory.
fn parity(gpu: &Gpu, k: &Kernels, gen: &Generator, family: &Family) -> Parity {
    let x = executor::grow(family).unwrap().expansion().unwrap();
    if std::env::var("TRACE").is_ok() {
        println!("bearing bounds {:?}", x.bearing_bounds());
    }
    let theirs = reference(gen, &x);
    let their_curve = x.curve().unwrap();
    let built = pollster::block_on(build(gpu, gen, k, &Tools::new(gpu), family)).unwrap();
    let part = built.expansion.tree();
    let fine = fine_nodes(x.tree(), part);
    let s = family.supernatural.scale();
    let (nodes, forks) = nodes((part, x.tree(), &fine), s);
    let runs = curve(gpu, &built, (&their_curve, &forks), s);
    let resident = built.prepared.resident.as_ref().unwrap();
    let leaves = io::read_leaves_async(gpu, resident.leaves.buffer(), resident.count);
    let leaves = pollster::block_on(leaves).unwrap();
    let cpu = built.cpu_segments.len();
    let first = leaves.len() as u32 - built.device.stations;
    let mut mine: Vec<Seg> = (built.cpu_segments.iter())
        .map(|s| Seg {
            words: [s.key, s.run_station, s.first, s.count],
            start: s.endpoints[0],
            end: s.endpoints[1],
        })
        .collect();
    mine.extend(device_segments(gpu, &built.device, first));
    assert_eq!(mine.len(), cpu + built.device.segments as usize);
    let mine = (&leaves[..], &mine[..], built.expansion.reference());
    let leaves = compare_leaves(
        mine,
        (&theirs.leaves, &theirs.segments, x.reference()),
        x.tree(),
    );
    Parity {
        nodes,
        runs,
        leaves,
    }
}

fn generator(gpu: &Gpu) -> Generator {
    let make = Generator::create(gpu.clone(), std::sync::Arc::new(()), true);
    pollster::block_on(make).unwrap()
}

fn family(id: &str, (age, seed): (u32, u32)) -> Family {
    let mut family = telperion_core::presets::by_identity(id).unwrap();
    (family.age, family.skeleton.seed) = (f64::from(age), seed);
    family
}

/// The beech and the spruce at 30 on the real path, submitted, against
/// the CPU path.
#[test]
fn the_real_path_matches_the_cpu_path() {
    let Some(gpu) = crate::generation::test_gpu() else {
        return;
    };
    let (k, gen) = (kernels(&gpu), generator(&gpu));
    for id in ["european-beech", "norway-spruce"] {
        let p = parity(&gpu, &k, &gen, &family(id, (30, 1)));
        println!("{id} (30, 1): {p:?}");
        assert_eq!(p.leaves.flags, 0, "{id}: placement flags");
        assert_eq!(
            p.runs.unmatched + p.runs.missing,
            0,
            "{id}: every run matched"
        );
    }
}

/// fn-254 R4 check 4, leaf area: every shipped GPU preset at 30 on the
/// real path, its leaves against the CPU path's by key and station: as
/// many, and none apart in place, direction or scale past the screen.
/// Measured: every leaf of all five matched, none apart in scale; the
/// birch's 42,548 apart in place and 5,673 in direction ride its wood's
/// gap.
/// The birch's leaves apart in place and in direction, of 60,084, with
/// its wood's gap: measured 42,548 and 5,673, bound at about a tenth more.
const BIRCH_LEAVES: (u64, u64) = (47_000, 6_300);

#[test]
fn every_gpu_preset_seats_its_leaves_as_the_cpu_path_does() {
    let Some(gpu) = crate::generation::test_gpu() else {
        return;
    };
    let (k, gen) = (kernels(&gpu), generator(&gpu));
    for id in ["european-beech", "norway-spruce", "silver-birch", "telperion", "laurelin"] {
        let p = parity(&gpu, &k, &gen, &family(id, (30, 42)));
        let l = &p.leaves;
        println!(
            "{id} (30, 42): {} of {} leaves, {} compared, left {:?}, over {:?}, flags {}",
            l.leaves.0, l.leaves.1, l.compared, l.left, l.over, l.flags
        );
        assert_eq!(l.leaves.0 as u64, l.leaves.1, "{id}: as many leaves");
        assert_eq!((l.compared, l.left), (l.leaves.1, (0, 0, 0)), "{id}: every leaf compared");
        assert_eq!(l.flags, 0, "{id}: placement flags");
        // No leaf's scale apart: the leaf area agrees leaf by leaf. The
        // birch's places and directions stand apart with its wood, whose
        // gap predates the floats (`preset_agree_tests::BIRCH`): held to
        // what was measured, so a regression still shows.
        let most = if id == "silver-birch" { BIRCH_LEAVES } else { (0, 0) };
        assert!(
            l.over.0 <= most.0 && l.over.1 <= most.1 && l.over.2 == 0,
            "{id}: leaves apart {:?}",
            l.over
        );
    }
}

/// fn-248: where wood sheds its leaves by age, a whole year and a share
/// of one, the device keeps the stations the CPU keeps, each with its
/// own ordinal's leaf.
#[test]
fn the_real_path_keeps_the_stations_the_cpu_path_keeps() {
    let Some(gpu) = crate::generation::test_gpu() else {
        return;
    };
    let (k, gen) = (kernels(&gpu), generator(&gpu));
    for years in [1.0, 1.5] {
        let mut f = family("european-beech", (30, 1));
        let species = f.space.species.as_mut().unwrap();
        for s in &mut species.states {
            s.leaf_years = years;
        }
        let p = parity(&gpu, &k, &gen, &f);
        println!("european-beech (30, 1) at {years} years: {p:?}");
        assert_eq!(p.leaves.flags, 0, "{years}: placement flags");
        assert_eq!(p.runs.unmatched + p.runs.missing, 0, "{years}: every run");
        let (device, reference) = p.leaves.leaves;
        assert_eq!(u64::from(device), reference, "{years}: leaves");
        assert_eq!(p.leaves.left, (0, 0, 0), "{years}: every leaf compared");
        assert_eq!(p.leaves.over.0, 0, "{years}: positions");
    }
}

/// fn-248: an age's internode and pipe overridden by the supernatural
/// group grow the same tree on the device as on the CPU.
#[test]
fn the_real_path_grows_the_overridden_ages_the_cpu_path_grows() {
    let Some(gpu) = crate::generation::test_gpu() else {
        return;
    };
    let (k, gen) = (kernels(&gpu), generator(&gpu));
    let mut f = family("european-beech", (30, 1));
    f.supernatural.enabled = true;
    for age in [4, 6, 7] {
        f.supernatural.internode.set(age, 0.1);
        f.supernatural.pipe.set(age, 0.02);
    }
    // The overrides grow the limbs and boughs longer: the tree is taller.
    let tall = |f: &telperion_core::Family| {
        let x = executor::grow(f).unwrap().expansion().unwrap();
        x.tree()
            .nodes
            .iter()
            .map(|n| n.position.y)
            .fold(0.0, f64::max)
    };
    let natural = tall(&family("european-beech", (30, 1)));
    assert!(tall(&f) > 1.2 * natural, "{} against {natural}", tall(&f));
    let p = parity(&gpu, &k, &gen, &f);
    println!("european-beech (30, 1) overridden: {p:?}");
    assert_eq!(p.leaves.flags, 0, "placement flags");
    assert_eq!(p.runs.unmatched + p.runs.missing, 0, "every run matched");
    let (device, reference) = p.leaves.leaves;
    assert_eq!(u64::from(device), reference, "leaves");
    assert_eq!(p.leaves.left, (0, 0, 0), "every leaf compared");
    assert_eq!(p.leaves.over.0, 0, "positions");
}

/// fn-248: the leaf's size overridden by the supernatural group is the
/// size the device places, as the CPU does.
#[test]
fn the_real_path_places_the_overridden_leaf_size_the_cpu_path_places() {
    let Some(gpu) = crate::generation::test_gpu() else {
        return;
    };
    let (k, gen) = (kernels(&gpu), generator(&gpu));
    let mut f = family("european-beech", (30, 1));
    f.supernatural.enabled = true;
    f.supernatural.canopy.size = Some(2.0 * f.canopy.size);
    let p = parity(&gpu, &k, &gen, &f);
    println!("european-beech (30, 1) larger leaves: {p:?}");
    assert_eq!(p.leaves.flags, 0, "placement flags");
    let (device, reference) = p.leaves.leaves;
    assert_eq!(u64::from(device), reference, "leaves");
    assert_eq!(p.leaves.left, (0, 0, 0), "every leaf compared");
    assert_eq!(p.leaves.over, (0, 0, 0), "position, direction and scale");
}

/// The parity probe at 40 and 80 (`cargo test --profile ci -p
/// telperion-render real_path_at_80 -- --ignored --nocapture`, under the
/// GPU lock, `PROBE=norway-spruce:80:1` for one tree a process, with
/// `:lobed` for its wood two lobes a tenth deep). Reported, not held.
#[test]
#[ignore = "device limits: a tree at 80 on the device"]
fn real_path_at_80() {
    let Some(gpu) = crate::generation::test_gpu() else {
        return;
    };
    let (k, gen) = (kernels(&gpu), generator(&gpu));
    let only = std::env::var("PROBE").unwrap_or_else(|_| "norway-spruce:80:1".into());
    let parts: Vec<&str> = only.split(':').collect();
    let at = (parts[1].parse().unwrap(), parts[2].parse().unwrap());
    let mut family = family(parts[0], at);
    if parts.get(3) == Some(&"lobed") {
        (family.surface.lobes, family.surface.lobe_depth) = (2, 0.1);
    }
    let p = parity(&gpu, &k, &gen, &family);
    println!("{only}: {p:?}");
}

/// fn-217 S1: the device draws its roots' wood at the scale the pipeline
/// drew the tree above the cut, so the young beech on the real path
/// matches the CPU path at both ends of the scale's range, the screen
/// taken at the scale.
#[test]
fn the_real_path_matches_the_cpu_path_at_every_scale() {
    let Some(gpu) = crate::generation::test_gpu() else {
        return;
    };
    let (k, gen) = (kernels(&gpu), generator(&gpu));
    for scale in [0.1, 100.0] {
        let mut f = family("european-beech", (20, 1));
        f.supernatural.enabled = true;
        f.supernatural.scale = scale;
        let p = parity(&gpu, &k, &gen, &f);
        println!("european-beech (20, 1) at {scale}: {p:?}");
        assert_eq!(p.leaves.flags, 0, "{scale}: placement flags");
        assert_eq!(
            p.runs.unmatched + p.runs.missing,
            0,
            "{scale}: every run matched"
        );
        assert_eq!(
            p.nodes.nodes.1 + p.nodes.nodes.2,
            0,
            "{scale}: every node placed"
        );
        assert!(p.runs.runs.1 > 0, "{scale}: the device drew roots");
    }
}
