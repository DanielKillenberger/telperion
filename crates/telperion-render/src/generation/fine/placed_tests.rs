//! fn-212 task 6: the fine roots' seated leaves placed on the device, from
//! the station segments `stations.wgsl` built and on the vertices of the
//! fine block's rings (`rings.wgsl`, the positions pass's `emit`), against
//! the pipeline's own resident leaves (its segments placed on its rings),
//! leaf by leaf, matched by their run's key and their station along the
//! run: equal in number, and within fn-125's bound (`generation/tests.rs`:
//! two code steps of the leaf box, 0.005 in a column's direction, a scale
//! within 1/1024).
use super::super::{data, io, Generator, Metrics};
use super::dress::{Dressed, Dresser};
use super::fixture::{with_dressed, Fixture};
use super::rings::Ringer;
use super::roots::{kernels, Kernels};
use super::stations::{Stationer, GW};
use super::stations_tests::station_rows;
use super::sweep::{rows, Sweeper};
use crate::Gpu;
use std::borrow::Cow;
use std::collections::{HashMap, HashSet};
use telperion_core::foliage::{prepared::PreparedStations, Instances};
use telperion_core::math::Vec3;

/// What the placement probe measured on one tree.
// Read through its `Debug` print.
#[allow(dead_code)]
#[derive(Debug, Default)]
pub(super) struct Seated {
    /// The device's leaves, and the reference's leaves of the device's keys.
    pub leaves: (u32, u64),
    /// Leaves compared by key and station.
    pub compared: u64,
    /// Leaves left out: the device's on a run the reference has no run for,
    /// and on a run whose stations differ in number from its reference's;
    /// the reference's on a run the device has no run for
    /// (`stations_tests`' residual classes).
    pub left: (u64, u64, u64),
    /// Compared leaves outside the bound: position, direction, scale.
    pub over: (u64, u64, u64),
    /// Compared leaves whose direction is over the bound, by class: their
    /// orientation words one code step apart (`code_step`), and their face
    /// turned about an axis within the bound (`compare`).
    pub steps: (u64, u64),
    /// Of the leaves over in direction, those whose segment's normal is
    /// 1 mrad or more from the reference's, and the most it is.
    pub framed: u64,
    pub frame: f64,
    /// The most apart: position (m), a column's direction, scale
    /// (relative); and the position bound.
    pub position: f64,
    pub direction: f64,
    pub scale: f64,
    pub bound: f64,
    /// The device placement's flags (transform, contact, box).
    pub flags: u32,
    pub ms: f64,
}

/// The pipeline's resident leaves of `x`, its segments' keys, first
/// stations, first leaves and counts.
fn reference(gen: &Generator, x: &telperion_core::pipeline::executor::Expansion) -> Reference {
    let (_, shared) = x.curve_with_contacts().unwrap();
    let theirs = x.compact_stations(&shared).unwrap().unwrap();
    let segments = (theirs.segments.iter())
        .map(|s| Seg {
            words: [s.key, s.run_station, s.first, s.count],
            start: s.endpoints[0],
            normal: s.frame[1],
        })
        .collect();
    let mut m = Metrics::default();
    let wood = pollster::block_on(gen.emit_positions(shared.into_surface(), &mut m));
    let wood = wood.unwrap().expect("the reference's ring positions");
    let p = PreparedStations {
        segments: theirs.segments,
        count: theirs.count,
        ring_size: theirs.ring_size,
        rings: Cow::Borrowed(&wood.positions),
    };
    let (l, e) = (x.leaves(), x.element());
    let r = pollster::block_on(gen.compute_buffer_async(p, &l, x.twig(), e, x.reference(), &mut m));
    let r = r.unwrap();
    let leaves = pollster::block_on(io::read_leaves_async(&gen.gpu, r.leaves.buffer(), r.count));
    Reference {
        leaves: leaves.unwrap(),
        segments,
    }
}

struct Reference {
    leaves: Vec<[u32; 3]>,
    segments: Vec<Seg>,
}

/// A station segment as matching reads it: its run's key, its first
/// station along the run, its first leaf and its leaves; and where it
/// starts.
#[derive(Clone, Copy)]
struct Seg {
    words: [u32; 4],
    start: Vec3,
    normal: Vec3,
}

/// One run's leaves by station, from its first, where it starts, and
/// each leaf's segment's normal.
struct Run {
    key: u32,
    station: u32,
    start: Vec3,
    leaves: Vec<u32>,
    normals: Vec<Vec3>,
}

/// The runs `segments` hold: a run's segments follow one another, each
/// beginning where the one before ends.
fn runs(segments: &[Seg]) -> Vec<Run> {
    let mut out: Vec<Run> = Vec::new();
    for s in segments {
        let [key, station, first, count] = s.words;
        let on = out
            .last()
            .filter(|r| r.key == key && r.station + r.leaves.len() as u32 == station);
        if on.is_none() {
            let start = s.start;
            let (leaves, normals) = (Vec::new(), Vec::new());
            out.push(Run {
                key,
                station,
                start,
                leaves,
                normals,
            });
        }
        let run = out.last_mut().unwrap();
        run.leaves.extend(first..first + count);
        run.normals.extend((0..count).map(|_| s.normal));
    }
    out
}

/// The fine roots of `fx`, drawn as `dressed`, stationed and placed on the
/// device, against the reference's resident leaves.
fn seated(
    gpu: &Gpu,
    gen: &Generator,
    (sw, st, rg): (&Sweeper, &Stationer, &Ringer),
    (fx, dressed): (&Fixture, &Dressed),
    theirs: Reference,
) -> Seated {
    let x = &fx.reference;
    let (curve, shared) = x.curve_with_contacts().unwrap();
    let surface = &fx.family.surface;
    let (segments, angular) = (shared.surface().segments, shared.surface().angular.clone());
    drop(shared);
    let t = std::time::Instant::now();
    let swept = sw.sweep(gpu, dressed, rows(surface, curve.height));
    let device = st.stations(gpu, &swept, dressed.ids, station_rows(x, segments));
    // The rings read the run table and the points alone.
    swept.nodes.destroy();
    swept.cluster_buffer.destroy();
    let twist = std::f64::consts::TAU * surface.twist_rate / curve.height;
    let rung = rg.rings(gpu, &swept, segments, twist as f32);
    let rings = (&rung.buffer, rung.rings, rung.vertices);
    let positions = gen.ring_vertices(rings, segments, &angular).unwrap();
    let (l, e) = (x.leaves(), x.element());
    let counts = (device.stations, device.segments, segments);
    let config = data::config(&l, x.twig(), counts, e, x.reference());
    let placed =
        (gen.place_leaves(config, (&device.buffer, &positions), e, device.stations)).unwrap();
    let summary = io::read(gpu, &placed.summary, 64).unwrap();
    let ms = t.elapsed().as_secs_f64() * 1e3;
    let leaves = io::read_leaves_async(gpu, &placed.leaves, device.stations);
    let leaves = pollster::block_on(leaves).unwrap();
    let words = device.segments as usize * GW;
    let raw = io::read(gpu, &device.buffer, (words.max(4) * 4) as u64).unwrap();
    let f = |w: u32| f64::from(f32::from_bits(w));
    let mine: Vec<Seg> = (bytemuck::cast_slice::<u8, u32>(&raw).chunks(GW))
        .take(device.segments as usize)
        .map(|g| Seg {
            words: [g[27], g[26], g[24], g[25]],
            start: Vec3::new(f(g[0]), f(g[1]), f(g[2])),
            normal: Vec3::new(f(g[12]), f(g[13]), f(g[14])),
        })
        .collect();
    let keys: HashSet<u32> = mine.iter().map(|g| g.words[0]).collect();
    let refs: Vec<Seg> = (theirs.segments.iter())
        .filter(|g| keys.contains(&g.words[0]))
        .copied()
        .collect();
    let theirs_runs = runs(&refs);
    let mut by: HashMap<(u32, u32), Vec<usize>> = HashMap::new();
    for (i, r) in theirs_runs.iter().enumerate() {
        by.entry((r.key, r.station)).or_default().push(i);
    }
    let box_ = x.reference();
    let (mut a, mut b) = (Instances::new(box_), Instances::new(box_));
    (a.leaves, b.leaves) = (leaves, theirs.leaves);
    let step = box_.step().length();
    let mut out = Seated {
        leaves: (
            device.stations,
            refs.iter().map(|g| u64::from(g.words[3])).sum(),
        ),
        bound: step * 2.0 + 32.0 * f64::from(f32::EPSILON) * 4.0,
        flags: u32::from_ne_bytes(summary[4..8].try_into().unwrap()),
        ms,
        ..Seated::default()
    };
    let mut used = vec![false; theirs_runs.len()];
    for run in runs(&mine) {
        let near = by.get(&(run.key, run.station)).and_then(|c| {
            let off = |&i: &usize| (theirs_runs[i].start - run.start).length();
            c.iter()
                .filter(|&&i| !used[i])
                .min_by(|x, y| off(x).total_cmp(&off(y)))
        });
        let Some(&i) = near else {
            out.left.0 += run.leaves.len() as u64;
            continue;
        };
        used[i] = true;
        let theirs = &theirs_runs[i].leaves;
        if theirs.len() != run.leaves.len() {
            out.left.1 += run.leaves.len() as u64;
            continue;
        }
        let normals = &theirs_runs[i].normals;
        for (k, (&d, &r)) in run.leaves.iter().zip(theirs).enumerate() {
            let (d, r) = (d as usize, r as usize);
            let step = code_step(a.leaves[d][0], b.leaves[r][0]);
            let was = out.over.1;
            compare(&mut out, (a.matrix(d), b.matrix(r)), step);
            if out.over.1 > was {
                let turn = run.normals[k].normalized().dot(normals[k].normalized());
                let turn = turn.clamp(-1.0, 1.0).acos();
                out.framed += u64::from(turn >= 0.001);
                out.frame = out.frame.max(turn);
            }
            let off = a.position(d).distance(b.position(r));
            out.position = out.position.max(off);
            out.over.0 += u64::from(off > out.bound);
            out.compared += 1;
        }
    }
    out.left.2 = (theirs_runs.iter().zip(&used))
        .filter(|(_, &u)| !u)
        .map(|(r, _)| r.leaves.len() as u64)
        .sum();
    out
}

/// Whether two orientation words (`pack.wgsl`'s `rotation_word`) differ by
/// at most one code in each kept component, the same one dropped: a
/// rounding to the 10-bit grid that inputs a few microradians apart land
/// either side of (the f32-against-f64 class).
fn code_step(a: u32, b: u32) -> bool {
    let field = |w: u32, k: u32| ((w >> (10 * k)) & 0x3ff) as i32;
    a >> 30 == b >> 30 && (0..3).all(|k| (field(a, k) - field(b, k)).abs() <= 1)
}

/// One leaf's columns against the reference's: their directions and
/// lengths. A direction over the bound is counted by class: an orientation
/// one code step apart; or a face and side turned about an axis within the
/// bound. `pack.wgsl` takes a leaf's face from `up - axis * axis.y` before
/// its scatter turns it, so where that axis stands near the vertical a
/// small difference in it turns the face about it many times as far; the
/// scatter's draws are the reference's to the bit (its scale shows it).
fn compare(out: &mut Seated, (a, b): ([f32; 16], [f32; 16]), step: bool) {
    let column = |m: &[f32; 16], c: usize| {
        Vec3::new(m[c * 4] as f64, m[c * 4 + 1] as f64, m[c * 4 + 2] as f64)
    };
    let (mut dir, mut scale) = (0.0f64, 0.0f64);
    let mut axis = 0.0f64;
    for c in 0..3 {
        let (x, y) = (column(&a, c), column(&b, c));
        let d = x.normalized().distance(y.normalized());
        dir = dir.max(d);
        axis = if c == 1 { d } else { axis };
        scale = scale.max((x.length() - y.length()).abs() / y.length());
    }
    out.direction = out.direction.max(dir);
    out.scale = out.scale.max(scale);
    let over = dir >= 0.005;
    let turned = !step && axis < 0.005;
    out.steps.0 += u64::from(over && step);
    out.steps.1 += u64::from(over && turned);
    out.over.1 += u64::from(over && !step && !turned);
    out.over.2 += u64::from(scale > 1.0 / 1024.0);
}

/// The preset `id` at `at`: its reference's resident leaves made first,
/// so the settled device tree and the reference's rings never share the
/// device's memory, then its fine roots placed on the device.
fn probe(gpu: &Gpu, k: &Kernels, gen: &Generator, d: &Dresser, id: &str, at: (u32, u32)) -> Seated {
    let mut family = telperion_core::presets::by_identity(id).unwrap();
    (family.age, family.skeleton.seed) = (f64::from(at.0), at.1);
    let x = telperion_core::pipeline::executor::grow(&family).unwrap();
    let theirs = reference(gen, &x.expansion().unwrap());
    let tools = (&Sweeper::new(gpu), &Stationer::new(gpu), &Ringer::new(gpu));
    with_dressed(gpu, k, d, id, at, |fx, dressed| {
        seated(gpu, gen, tools, (fx, dressed), theirs)
    })
}

fn generator(gpu: &Gpu) -> Generator {
    let make = Generator::create(gpu.clone(), std::sync::Arc::new(()), true);
    pollster::block_on(make).unwrap()
}

/// The beech and the spruce at 30: the fine roots' seated leaves on the
/// device, leaf for leaf the pipeline's within fn-125's bound.
#[test]
fn fine_roots_are_seated_on_the_device() {
    let Some(gpu) = crate::generation::test_gpu() else {
        return;
    };
    let (k, gen, d) = (kernels(&gpu), generator(&gpu), Dresser::new(&gpu));
    for id in ["european-beech", "norway-spruce"] {
        let s = probe(&gpu, &k, &gen, &d, id, (30, 1));
        println!("{id} (30, 1): {s:?}");
        assert_eq!(s.flags, 0, "{id}: placement flags");
        assert_eq!(u64::from(s.leaves.0), s.leaves.1, "{id}: leaves");
        assert_eq!(s.compared, s.leaves.1, "{id}: every leaf compared");
        assert_eq!(s.over, (0, 0, 0), "{id}: within the bound");
    }
}

/// The probe's fixtures at 40 and 80 (`cargo test --profile ci -p
/// telperion-render seated_at_40_and_80 -- --ignored --nocapture`, under
/// the GPU lock). Reported, not held.
#[test]
#[ignore]
fn seated_at_40_and_80() {
    let Some(gpu) = crate::generation::test_gpu() else {
        return;
    };
    let (k, gen, d) = (kernels(&gpu), generator(&gpu), Dresser::new(&gpu));
    // One tree where `PROBE` names it (`norway-spruce:80:1`).
    let only = std::env::var("PROBE").ok();
    for id in ["european-beech", "norway-spruce"] {
        for at in [(40, 1), (40, 7), (80, 1), (80, 7)] {
            let name = format!("{id}:{}:{}", at.0, at.1);
            if only.as_ref().is_some_and(|o| *o != name) {
                continue;
            }
            println!("{id} {at:?}: {:?}", probe(&gpu, &k, &gen, &d, id, at));
        }
    }
}
