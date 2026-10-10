//! fn-253 R4: every drawn floret hangs from wood through its stalk, on
//! both executors. Each executor's organ words are decoded as the organ's
//! vertex stage reads them (`foliage.wgsl`'s `pedicel_vertex` and
//! `rachis_vertex`): a floret's pedicel seat is its base a pedicel back
//! along its axis, and must lie inside its own rachis's tube, laid from
//! the rachis instance by `bloom.rs`'s axis, which every rachis level runs
//! through ring by ring (`stalk.rs`); its own rachis is the first
//! marked instance after it in its run, as selection and the draw find it
//! (`unit.wgsl`); each rachis must stand on a shoot tip of the tree. With
//! the rachis at zero, the seat is the tip itself.
//! The trees carry non-unit floret sizes (taper, scatter) and, at a
//! stagger of one, florets the season has grown only part way or dropped.
use super::resident::{build, Tools};
use super::roots::kernels;
use crate::generation::{io, Generator};
use crate::Gpu;
use std::collections::HashMap;
use telperion_core::foliage::{BloomParams, Leaf, Reference, WITHERED};
use telperion_core::math::Vec3;
use telperion_core::pipeline::executor;
use telperion_core::presets::Family;
use telperion_core::tree::Tree;

/// `bloom.rs`'s axis segments.
const SEGMENTS: usize = 8;

/// One executor's florets against its rachises, as drawn.
#[derive(Debug, Default)]
struct Hung {
    florets: usize,
    rachises: usize,
    /// Florets drawn below their full size.
    partial: usize,
    /// The farthest a pedicel's seat stands outside its rachis's tube, and
    /// a rachis's seat from a shoot tip, in metres (zero or under: inside).
    outside: f64,
    seat: f64,
    /// Florets with no rachis within reach.
    loose: usize,
}

fn column(m: &[f32; 16], c: usize) -> Vec3 {
    Vec3::new(m[c * 4].into(), m[c * 4 + 1].into(), m[c * 4 + 2].into())
}

/// The rachis standing at `seat` along `heading`: its points, as the
/// vertex stage lays them.
fn axis(seat: Vec3, heading: Vec3, p: &BloomParams) -> [Vec3; SEGMENTS + 1] {
    let toward = if p.droop >= 0.0 { -Vec3::Y } else { Vec3::Y };
    let mut points = [seat; SEGMENTS + 1];
    for k in 0..SEGMENTS {
        let rest = 1.0 - (k as f64 + 0.5) / SEGMENTS as f64;
        let turned = p.droop.abs() * (1.0 - rest.powi(3));
        let t = heading.lerp(toward, turned);
        let t = if t.length_squared() > 1e-24 { t.normalized() } else { toward };
        points[k + 1] = points[k] + t * (p.rachis / SEGMENTS as f64);
    }
    points
}

/// How far `q` stands outside the tube through `points`, which narrows from
/// `radius` to `end`.
fn outside(q: Vec3, points: &[Vec3; SEGMENTS + 1], radius: f64, end: f64) -> f64 {
    (0..SEGMENTS)
        .map(|k| {
            let (a, b) = (points[k], points[k + 1]);
            let d = b - a;
            let t = ((q - a).dot(d) / d.length_squared().max(1e-30)).clamp(0.0, 1.0);
            let along = (k as f64 + t) / SEGMENTS as f64;
            q.distance(a + d * t) - (radius + (end - radius) * along)
        })
        .fold(f64::INFINITY, f64::min)
}

/// `words`, packed against `r`, hung from `tree`'s tips by `p`.
fn hung(words: &[Leaf], r: Reference, tree: &Tree, p: &BloomParams) -> Hung {
    let mut out = Hung::default();
    let cell = (p.rachis + p.pedicel).max(1e-3);
    let key = |v: Vec3| {
        let c = v * (1.0 / cell);
        (c.x.floor() as i64, c.y.floor() as i64, c.z.floor() as i64)
    };
    let near = |grid: &HashMap<_, Vec<usize>>, v: Vec3| -> Vec<usize> {
        let (x, y, z) = key(v);
        let mut found = Vec::new();
        for d in -1..=1 {
            for e in -1..=1 {
                for f in -1..=1 {
                    found.extend(grid.get(&(x + d, y + e, z + f)).into_iter().flatten());
                }
            }
        }
        found
    };
    // The tips that bear, by cell.
    let mut tips: HashMap<_, Vec<usize>> = HashMap::new();
    for b in &tree.blooms {
        let i = b.node as usize;
        tips.entry(key(tree.nodes[i].position)).or_default().push(i);
    }
    let end = 0.5 * p.stamen_width;
    // Each floret waits for the rachis that closes its run.
    let mut run: Vec<Vec3> = Vec::new();
    for w in words {
        let m = r.unpack(*w);
        if w[2] & WITHERED == 0 {
            let along = column(&m, 1);
            out.partial += usize::from(along.length() < 0.99);
            out.florets += 1;
            let seat = column(&m, 3) - along.normalized() * p.pedicel;
            if p.rachis > 0.0 {
                run.push(seat);
                continue;
            }
            let gap = (near(&tips, seat).iter())
                .map(|&i| tree.nodes[i].position.distance(seat) - tree.nodes[i].radius)
                .fold(f64::INFINITY, f64::min);
            match gap.is_finite() {
                true => out.outside = out.outside.max(gap),
                false => out.loose += 1,
            }
            continue;
        }
        let seat = column(&m, 3);
        let heading = column(&m, 1);
        let tip = (near(&tips, seat).iter())
            .map(|&i| tree.nodes[i].position.distance(seat))
            .fold(f64::INFINITY, f64::min);
        out.seat = out.seat.max(tip);
        out.rachises += 1;
        let points = axis(seat, heading.normalized(), p);
        for q in run.drain(..) {
            out.outside = out.outside.max(outside(q, &points, heading.length(), end));
        }
    }
    out.loose += run.len();
    out
}

/// `family` built on the real path and as the CPU whole tree: each one's
/// organ hung from the tree.
fn both(gpu: &Gpu, family: &Family) -> (Hung, Hung, Reference) {
    let make = Generator::create(gpu.clone(), std::sync::Arc::new(()), true);
    let gen = pollster::block_on(make).unwrap();
    let k = kernels(gpu);
    let built = pollster::block_on(build(gpu, &gen, &k, &Tools::new(gpu), family)).unwrap();
    let organ = built.prepared.organ.as_ref().expect("the tree flowers");
    let r = (built.prepared.mesh.organ.as_ref().unwrap().instances).reference;
    let mine = pollster::block_on(io::read_leaves_async(gpu, organ.leaves.buffer(), organ.count));
    let mine = mine.unwrap();
    drop(built);
    let x = executor::grow(family).unwrap().expansion().unwrap();
    let (p, _, _) = x.bloom();
    let theirs = x.organ_in(r).unwrap().instances.leaves;
    (hung(&mine, r, x.tree(), &p), hung(&theirs, r, x.tree(), &p), r)
}

/// Laurelin young, her florets opening one after another, at her own rows
/// and with no rachis.
#[test]
fn every_drawn_floret_hangs_from_wood_through_its_stalk() {
    let Some(gpu) = crate::generation::test_gpu() else {
        return;
    };
    for rachis in [None, Some(0.0)] {
        let mut family = telperion_core::presets::Preset::Laurelin.parameters();
        (family.age, family.skeleton.seed) = (30.0, 1);
        family.supernatural.bloom.stagger = Some(1.0);
        if let Some(r) = rachis {
            family.supernatural.bloom.rachis = Some(r);
        }
        let (device, cpu, r) = both(&gpu, &family);
        // Two codes of quantisation at each end of a stalk.
        let codes = 2.0 * r.step().length();
        println!("rachis {rachis:?}: device {device:?}, cpu {cpu:?}, codes {codes}");
        for (name, h) in [("device", &device), ("cpu", &cpu)] {
            assert!(h.florets > 1000 && h.partial > 100, "{name}: {h:?}");
            assert_eq!(h.loose, 0, "{name}: a floret in the air");
            assert!(h.outside <= codes, "{name}: a stalk off its seat by {}", h.outside);
            assert!(h.seat <= codes, "{name}: a rachis off its tip by {}", h.seat);
        }
        assert_eq!((device.florets, device.rachises), (cpu.florets, cpu.rachises));
        if rachis.is_some() {
            assert_eq!(cpu.rachises, 0, "no rachis drawn at zero");
        } else {
            assert!(cpu.rachises > 100);
        }
    }
}
