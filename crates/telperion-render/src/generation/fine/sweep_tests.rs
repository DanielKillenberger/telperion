//! fn-212 task 5: the fine roots' curve swept on the device against the
//! pipeline's CPU reference curve of the same tree: every device run is a
//! reference run of the fine roots' side, point for point within fn-125's
//! screen, its frames within 0.01 rad, its clusters' bounds, radius
//! extrema and error ladders with them, and the runs widest first (D5).
use super::dress::{Dressed, Dresser};
use super::dress_tests::{self, folded, strays};
use super::fixture::{with_dressed, Fixture};
use super::roots::{kernels, Kernels};
use super::sweep::{rows, Sweeper, Swept, KW, PW, TW};
use crate::Gpu;
use std::collections::HashMap;
use telperion_core::math::Vec3;
use telperion_core::surface::{Curve, CurvePoint};

/// fn-125's screen: metres, and radians for a frame.
const MOST: f64 = 50e-6;
const RMS: f64 = 10e-6;
const TURN: f64 = 0.01;

/// What the curve probe measured on one tree.
// Read through its `Debug` print.
#[allow(dead_code)]
#[derive(Debug, Default)]
pub(super) struct Curved {
    /// The device's runs, the reference's fine runs, and the device's runs
    /// with no reference run.
    pub runs: (u32, usize, usize),
    pub points: u32,
    /// The reference curve's runs and points, both sides.
    pub reference: (usize, usize),
    /// The farthest and RMS apart of the centres (m); the most apart of
    /// the radii, relative, and of the distances along (m); the widest
    /// turn of a frame (rad).
    pub centres: (f64, f64),
    pub radius: f64,
    pub along: f64,
    pub turn: f64,
    /// The clusters: count apart, the most apart of centre and reach (m),
    /// of the radius extrema (relative) and of the ladders (m).
    pub clusters: (usize, f64, f64, f64),
    /// Runs out of D5's order (wider after narrower).
    pub disorder: usize,
    /// Matched runs whose points leave the screen: they part from the
    /// reference's at a node.
    pub parted: usize,
    pub ms: f64,
}

fn words(gpu: &Gpu, b: &wgpu::Buffer, n: usize) -> Vec<u32> {
    let raw = super::super::io::read(gpu, b, (n.max(4) * 4) as u64).unwrap();
    bytemuck::cast_slice(&raw).to_vec()
}

fn v(w: &[u32]) -> Vec3 {
    let f = |i: usize| f64::from(f32::from_bits(w[i]));
    Vec3::new(f(0), f(1), f(2))
}

fn angle(a: Vec3, b: Vec3) -> f64 {
    a.dot(b).clamp(-1.0, 1.0).acos()
}

/// The reference's runs, by the cell of their first node (their second
/// point).
fn by_first(curve: &Curve) -> HashMap<(i64, i64, i64), Vec<usize>> {
    let mut out: HashMap<_, Vec<usize>> = HashMap::new();
    for (k, r) in curve.runs.iter().enumerate().filter(|(_, r)| r.count > 1) {
        out.entry(cell(curve.points[r.first as usize + 1].centre))
            .or_default()
            .push(k);
    }
    out
}

fn cell(v: Vec3) -> (i64, i64, i64) {
    let c = |x: f64| (x / 1e-3).floor() as i64;
    (c(v.x), c(v.y), c(v.z))
}

/// The reference run whose first node lies within the screen of `at`.
fn matched(curve: &Curve, runs: &HashMap<(i64, i64, i64), Vec<usize>>, at: Vec3) -> Option<usize> {
    let (x, y, z) = cell(at);
    let near = (-1..=1)
        .flat_map(|a| (-1..=1).flat_map(move |b| (-1..=1).map(move |c| (x + a, y + b, z + c))));
    near.flat_map(|c| runs.get(&c).into_iter().flatten())
        .map(|&k| {
            (
                k,
                (curve.points[curve.runs[k].first as usize + 1].centre - at).length(),
            )
        })
        .filter(|&(_, d)| d <= MOST)
        .min_by(|a, b| a.1.total_cmp(&b.1))
        .map(|(k, _)| k)
}

/// The device's curve against the reference's.
fn compared(gpu: &Gpu, curve: &Curve, s: &Swept, fine_runs: usize) -> Curved {
    let table = words(gpu, &s.table, s.runs as usize * TW);
    let points = words(gpu, &s.point_buffer, s.points as usize * PW);
    let clusters = words(gpu, &s.cluster_buffer, s.clusters as usize * KW);
    let runs = by_first(curve);
    let mut of_run = vec![(0usize, 0usize); curve.runs.len()];
    for (j, c) in curve.clusters.iter().enumerate() {
        let o = &mut of_run[c.run as usize];
        if o.1 == 0 {
            o.0 = j;
        }
        o.1 += 1;
    }
    let mut out = Curved {
        runs: (s.runs, fine_runs, 0),
        points: s.points,
        ..Curved::default()
    };
    let (mut squares, mut n) = (0.0, 0usize);
    let mut last = f32::INFINITY;
    for r in 0..s.runs as usize {
        let [first, cluster, count, widest] = table[r * TW..][..TW] else {
            unreachable!()
        };
        let widest = f32::from_bits(widest);
        out.disorder += usize::from(widest > last);
        last = widest;
        let point = |i: u32| &points[(first + i) as usize * PW..][..PW];
        let Some(k) = matched(curve, &runs, v(point(1))) else {
            out.runs.2 += 1;
            continue;
        };
        let want = &curve.runs[k];
        if want.count != count {
            out.runs.2 += 1;
            continue;
        }
        let theirs: &[CurvePoint] = curve.run_points(want);
        let parts = theirs
            .iter()
            .enumerate()
            .any(|(i, c)| (v(point(i as u32)) - c.centre).length() > MOST);
        if parts {
            out.parted += 1;
            continue;
        }
        for (i, c) in theirs.iter().enumerate() {
            let p = point(i as u32);
            let f = |j: usize| f64::from(f32::from_bits(p[j]));
            let e = (v(p) - c.centre).length();
            out.centres.0 = out.centres.0.max(e);
            (squares, n) = (squares + e * e, n + 1);
            out.radius = out.radius.max((f(3) - c.radius).abs() / c.radius.max(1e-9));
            out.along = out.along.max((f(4) - c.along).abs());
            let turn = angle(v(&p[5..]), c.normal).max(angle(v(&p[8..]), c.binormal));
            out.turn = out.turn.max(turn);
        }
        let theirs = curve.clusters[of_run[k].0..][..of_run[k].1].iter();
        let mine = (0..(count - 1).div_ceil(31) as usize)
            .map(|j| &clusters[(cluster as usize + j) * KW..][..KW])
            .take_while(|c| c[0] as usize == r);
        let (theirs, mine): (Vec<_>, Vec<_>) = (theirs.collect(), mine.collect());
        out.clusters.0 += theirs.len().abs_diff(mine.len());
        for (c, m) in theirs.iter().zip(&mine) {
            let f = |j: usize| f64::from(f32::from_bits(m[j]));
            let apart = (v(&m[3..]) - c.centre).length().max((f(6) - c.reach).abs());
            out.clusters.1 = out.clusters.1.max(apart);
            let rel = |a: f64, b: f64| (a - b).abs() / b.max(1e-9);
            let radii = rel(f(7), c.largest_radius).max(rel(f(8), c.smallest_radius));
            out.clusters.2 = out.clusters.2.max(radii);
            for (l, e) in c.errors.iter().enumerate() {
                out.clusters.3 = out.clusters.3.max((f(9 + l) - f64::from(*e)).abs());
            }
        }
    }
    out.centres.1 = (squares / n.max(1) as f64).sqrt();
    out
}

/// The preset `id` at `at`: its fine roots drawn and swept on the device,
/// against the pipeline's curve.
pub(super) fn curved(
    gpu: &Gpu,
    k: &Kernels,
    d: &Dresser,
    sw: &Sweeper,
    id: &str,
    at: (u32, u32),
) -> Curved {
    with_dressed(gpu, k, d, id, at, |fx, dressed| {
        swept_against(gpu, sw, fx, dressed)
    })
}

/// The fine roots `dressed` drew swept on the device, against the
/// reference's curve.
fn swept_against(gpu: &Gpu, sw: &Sweeper, fx: &Fixture, dressed: &Dressed) -> Curved {
    let curve = fx.reference.curve().unwrap();
    let rows = rows(&fx.family.surface, curve.height);
    let t = std::time::Instant::now();
    let swept = sw.sweep(gpu, dressed, rows);
    let ms = t.elapsed().as_secs_f64() * 1e3;
    // The reference's runs of the device's nodes: its fine roots' side.
    let tree = fx.reference.tree();
    let fine: Vec<Vec3> = (tree.nodes.iter().enumerate())
        .filter(|&(i, _)| fx.fine[i])
        .map(|(_, n)| n.position)
        .collect();
    let firsts: Vec<Vec3> = (curve.runs.iter())
        .filter(|r| r.count > 1)
        .map(|r| curve.points[r.first as usize + 1].centre)
        .collect();
    let fine_runs = firsts.len() - strays(&firsts, &fine);
    Curved {
        ms,
        reference: (curve.runs.len(), curve.points.len()),
        ..compared(gpu, &curve, &swept, fine_runs)
    }
}

fn checked(name: &str, at: (u32, u32), c: &Curved) {
    println!("{name} {at:?}: {c:?}");
    assert_eq!(c.disorder, 0, "{name} {at:?}: D5's order");
    assert!(
        c.centres.0 <= MOST && c.centres.1 <= RMS,
        "{name} {at:?}: centres"
    );
    assert!(c.along <= MOST, "{name} {at:?}: along");
    assert!(c.turn <= TURN, "{name} {at:?}: frames");
}

/// The beech and the spruce at 30: fine roots drawn, folded and swept on
/// the device as the pipeline draws, folds and sweeps them, on one
/// settled tree each (the device holds one tree at a time).
#[test]
fn fine_roots_are_drawn_folded_and_swept_on_the_device() {
    let Some(gpu) = crate::generation::test_gpu() else {
        return;
    };
    let (k, d, sw) = (kernels(&gpu), Dresser::new(&gpu), Sweeper::new(&gpu));
    let at = (30, 1);
    for id in ["european-beech", "norway-spruce"] {
        with_dressed(&gpu, &k, &d, id, at, |fx, dressed| {
            let p = folded(&gpu, &d, fx, dressed);
            dress_tests::checked(id, at, &p);
            assert_eq!((p.strays.1, p.strays.2), (0, 0), "{id}: the nodes drawn");
            let c = swept_against(&gpu, &sw, fx, dressed);
            checked(id, at, &c);
            assert_eq!(c.runs.2, 0, "{id}: every device run a reference run");
            assert_eq!(
                c.runs.0 as usize, c.runs.1,
                "{id}: every fine run on the device"
            );
        });
    }
}

/// The probe's fixtures at 40 and 80 (`cargo test --profile ci -p
/// telperion-render swept_at_40_and_80 -- --ignored --nocapture
/// --test-threads=1`, under
/// the GPU lock).
#[test]
#[ignore]
fn swept_at_40_and_80() {
    let Some(gpu) = crate::generation::test_gpu() else {
        return;
    };
    let (k, d, sw) = (kernels(&gpu), Dresser::new(&gpu), Sweeper::new(&gpu));
    // One tree where `PROBE` names it (`norway-spruce:80:1`).
    let only = std::env::var("PROBE").ok();
    for id in ["european-beech", "norway-spruce"] {
        for at in [(40, 1), (40, 7), (80, 1), (80, 7)] {
            if only
                .as_ref()
                .is_some_and(|o| *o != format!("{id}:{}:{}", at.0, at.1))
            {
                continue;
            }
            checked(id, at, &curved(&gpu, &k, &d, &sw, id, at));
        }
    }
}

/// What the frame seed moved (fn-212, host decision 2026-10-07): for every
/// fine run of the pipeline's curve, the angle from the old seed (the
/// first tangent's `perpendicular`) to the new (the bearer's side laid
/// across it), binned by 15°; the spruce and beech at 80, seed 1
/// (`cargo test --profile ci -p telperion-render seed_twist_at_80 --
/// --ignored --nocapture`).
#[test]
#[ignore]
fn seed_twist_at_80() {
    for id in ["european-beech", "norway-spruce"] {
        let mut family = telperion_core::presets::by_identity(id).unwrap();
        family.age = 80.0;
        family.skeleton.seed = 1;
        let reference = telperion_core::pipeline::executor::grow(&family)
            .unwrap()
            .expansion()
            .unwrap();
        let (tree, curve) = (reference.tree(), reference.curve().unwrap());
        let mut fine = vec![false; tree.nodes.len()];
        for &s in &tree.seams {
            fine[s as usize] = true;
        }
        for i in 1..tree.nodes.len() {
            fine[i] |= fine[tree.nodes[i].parent.unwrap() as usize];
        }
        let key = |v: Vec3| [v.x.to_bits(), v.y.to_bits(), v.z.to_bits()];
        let at: HashMap<[u64; 3], usize> = (tree.nodes.iter().enumerate())
            .map(|(i, n)| (key(n.position), i))
            .collect();
        let mut bins = [0usize; 12];
        for r in curve.runs.iter().filter(|r| r.count > 1) {
            let p = &curve.points[r.first as usize..][..2];
            if !at.get(&key(p[1].centre)).is_some_and(|&i| fine[i]) {
                continue;
            }
            let t = (p[1].centre - p[0].centre).normalized();
            let old = t.perpendicular();
            let old = (old - t * old.dot(t)).normalized();
            let turn = angle(old, p[0].normal).to_degrees();
            bins[((turn / 15.0) as usize).min(11)] += 1;
        }
        println!("{id} 80 1: fine runs by the seed's turn, 15° bins from 0 to 180: {bins:?}");
    }
}
