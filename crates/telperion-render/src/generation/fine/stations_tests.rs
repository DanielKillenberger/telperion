//! fn-212 task 6: the fine roots' station segments built on the device
//! against the pipeline's own (`Expansion::compact_stations`), matched by
//! their run's first node's key and their first station along the run:
//! the same segments and station counts, their endpoints, radii, frames,
//! distances and turns within fn-125's screen, and their contacts naming
//! the same rings of their runs.
use super::dress::{Dressed, Dresser};
use super::fixture::{with_dressed, with_family, Fixture};
use super::roots::{kernels, Kernels};
use super::stations::{station_rows, Stationer, GW};
use super::sweep::{rows, Sweeper};
use crate::Gpu;
use std::collections::{HashMap, HashSet};
use telperion_core::foliage::prepared::StationSegment;
use telperion_core::math::Vec3;
use telperion_core::tree::Tree;

/// fn-125's screen, in metres, and radians for a frame.
const MOST: f64 = 50e-6;
const TURN: f64 = 0.01;

/// What the station probe measured on one tree.
// Read through its `Debug` print.
#[allow(dead_code)]
#[derive(Debug, Default)]
pub(super) struct Stationed {
    /// The device's runs, segments and stations; the reference's fine
    /// segments and stations (those of the device's runs' keys).
    pub device: (u32, u32, u32),
    pub reference: (usize, u64),
    /// Keys whose stations differ in number from the reference's (the
    /// class of threshold decisions fed by f32 against f64), and the
    /// stations they hold on the device. Left out of what follows.
    pub tied: (usize, u64),
    /// Runs whose first station or segment differs from one walk over the
    /// runs in their order (the bases' scan, fn-212 task 9).
    pub scanned: usize,
    /// Device segments with no reference segment, and reference segments
    /// of the device's keys with no device segment.
    pub strays: (usize, usize),
    /// Matched segments whose station count differs (a boundary on the
    /// other side of a node); left out of what follows.
    pub counts: usize,
    /// The most apart: endpoints (m), radii (relative), frames (rad),
    /// distances along and spans (m), turns' sine and cosine.
    pub endpoints: f64,
    pub radii: f64,
    pub frames: f64,
    pub along: f64,
    pub phase: f64,
    /// The keys left over (counted differently or split elsewhere), by
    /// class: the f32-against-f64 class (a node's grown wood at a half
    /// micrometre, `rounding`), and the rest, runs that part from their
    /// reference's elsewhere.
    pub classes: (usize, usize),
    /// Matched contacts whose rings stand elsewhere in their run.
    pub contacts: usize,
    pub ms: f64,
}

fn f(w: u32) -> f64 {
    f64::from(f32::from_bits(w))
}

fn v(w: &[u32]) -> Vec3 {
    Vec3::new(f(w[0]), f(w[1]), f(w[2]))
}

fn angle(a: Vec3, b: Vec3) -> f64 {
    a.dot(b).clamp(-1.0, 1.0).acos()
}

/// A contact's rings as their places in their run: lower, upper and the
/// run's last, from its first.
fn places(c: [u32; 4], segments: u32) -> [u32; 3] {
    [c[0] - c[2], c[1] - c[2], c[3] - c[2]].map(|o| o / segments)
}

/// Whether a reference segment ending at a point lies on wood the device
/// draws (`Fixture::fine`), found by its distal node's position.
pub(super) fn drawn_by_device<'a>(tree: &'a Tree, fine: &'a [bool]) -> impl Fn(Vec3) -> bool + 'a {
    let at: HashMap<[u64; 3], usize> = (tree.nodes.iter().enumerate())
        .map(|(i, n)| {
            (
                [n.position.x, n.position.y, n.position.z].map(f64::to_bits),
                i,
            )
        })
        .collect();
    move |p: Vec3| {
        at.get(&[p.x, p.y, p.z].map(f64::to_bits))
            .is_some_and(|&i| fine[i])
    }
}

/// The fine roots of `fx`, drawn as `dressed`, swept and stationed on the
/// device, against the pipeline's stations.
fn stationed(
    gpu: &Gpu,
    (sw, st): (&Sweeper, &Stationer),
    fx: &Fixture,
    dressed: &Dressed,
) -> Stationed {
    let x = &fx.reference;
    let (curve, shared) = x.curve_with_contacts().unwrap();
    let theirs = x.compact_stations(&shared).unwrap().unwrap();
    let surface = &fx.family.surface;
    let segments = surface.radial_segments.max(surface.lobes * 4);
    let r = station_rows(x, segments, false);
    let t = std::time::Instant::now();
    let swept = pollster::block_on(sw.sweep(gpu, dressed, rows(surface, curve.height))).unwrap();
    let (device, runs) = pollster::block_on(st.stations_and_runs(
        gpu,
        (&swept, &super::steps::Steps::new(gpu)),
        dressed.ids,
        r,
    ))
    .unwrap();
    let ms = t.elapsed().as_secs_f64() * 1e3;
    let scanned = scanned(gpu, &device, &runs);
    let words = device.segments as usize * GW;
    let raw = super::super::io::read(gpu, &device.buffer, (words.max(4) * 4) as u64).unwrap();
    let mine: &[u32] = bytemuck::cast_slice(&raw);
    let keys: HashSet<u32> = mine.chunks(GW).map(|g| g[27]).collect();
    // Every reference segment of the device's keys on wood the device
    // draws, by key and first station: two runs may share a key (32-bit
    // keys collide, with a run the device does not draw too), so a key
    // holds every run's, matched by the nearest first endpoint.
    let drawn = drawn_by_device(x.tree(), &fx.fine);
    let fine: Vec<&StationSegment> = (theirs.segments.iter())
        .filter(|s| keys.contains(&s.key) && drawn(s.endpoints[1]))
        .collect();
    let mut by: HashMap<(u32, u32), Vec<usize>> = HashMap::new();
    for (i, s) in fine.iter().enumerate() {
        by.entry((s.key, s.run_station)).or_default().push(i);
    }
    let mut out = Stationed {
        device: (device.runs, device.segments, device.stations),
        reference: (fine.len(), fine.iter().map(|s| u64::from(s.count)).sum()),
        scanned,
        ms,
        ..Stationed::default()
    };
    // Each key's stations, on either side.
    let total = |it: &mut dyn Iterator<Item = (u32, u32)>| {
        let mut t: HashMap<u32, u64> = HashMap::new();
        for (k, n) in it {
            *t.entry(k).or_default() += u64::from(n);
        }
        t
    };
    let ours = total(&mut mine.chunks(GW).map(|g| (g[27], g[25])));
    let refs = total(&mut fine.iter().map(|s| (s.key, s.count)));
    let tied: HashSet<u32> = (ours.iter())
        .filter(|(k, n)| refs.get(k) != Some(n))
        .map(|(k, _)| *k)
        .collect();
    out.tied = (tied.len(), tied.iter().map(|k| ours[k]).sum());
    let mut matched = vec![false; fine.len()];
    for g in mine.chunks(GW).filter(|g| !tied.contains(&g[27])) {
        let near = by.get(&(g[27], g[26])).and_then(|c| {
            let off = |&i: &usize| (fine[i].endpoints[0] - v(&g[0..])).length();
            c.iter()
                .filter(|&&i| !matched[i])
                .min_by(|a, b| off(a).total_cmp(&off(b)))
        });
        let Some(&i) = near else {
            out.strays.0 += 1;
            continue;
        };
        matched[i] = true;
        let c = fine[i];
        // A segment split elsewhere: a station boundary on the other side
        // of a node, the same class.
        if c.count != g[25] {
            out.counts += 1;
            continue;
        }
        let ends = (v(&g[0..]) - c.endpoints[0])
            .length()
            .max((v(&g[4..]) - c.endpoints[1]).length());
        out.endpoints = out.endpoints.max(ends);
        let rel = |a: f64, b: f64| (a - b).abs() / b.abs().max(1e-9);
        out.radii = out
            .radii
            .max(rel(f(g[3]), c.radii[0]).max(rel(f(g[7]), c.radii[1])));
        for (k, axis) in c.frame.iter().enumerate() {
            out.frames = out.frames.max(angle(v(&g[8 + 4 * k..]), *axis));
        }
        out.along = out
            .along
            .max((f(g[20]) - c.along).abs())
            .max((f(g[21]) - c.span).abs());
        out.phase = out
            .phase
            .max((f(g[22]) - c.phase[0]).abs())
            .max((f(g[23]) - c.phase[1]).abs());
        if let Some(contact) = c.contact {
            let mine = [g[28], g[29], g[30], g[31]];
            out.contacts += usize::from(places(mine, segments) != places(contact, segments));
        }
    }
    let odd = (0..fine.len())
        .filter(|&i| !matched[i])
        .map(|i| fine[i].key);
    let residual: HashSet<u32> = tied.iter().copied().chain(odd).collect();
    let rounding = rounding(x.tree(), &fine, &residual);
    out.classes = (rounding.len(), residual.len() - rounding.len());
    out.strays.1 = (0..fine.len())
        .filter(|&i| !matched[i] && !tied.contains(&fine[i].key))
        .count();
    out
}

/// The runs whose first station or segment differs from one walk over the
/// runs in their order: what the bases' parallel scan must equal.
fn scanned(gpu: &Gpu, device: &super::stations::Stationed, runs: &wgpu::Buffer) -> usize {
    let rw = super::stations::RW as usize;
    let bytes = (device.runs as usize * rw).max(1) * 4;
    let raw = super::super::io::read(gpu, runs, bytes as u64).unwrap();
    let words: &[u32] = bytemuck::cast_slice(&raw);
    let (mut s, mut g, mut off) = (0u32, 0u32, 0usize);
    for run in words.chunks(rw).take(device.runs as usize) {
        off += usize::from(run[4] != s || run[5] != g);
        s += run[1];
        g += run[2];
    }
    off
}

pub(super) fn checked(name: &str, at: (u32, u32), s: &Stationed) {
    println!("{name} {at:?}: {s:?}");
    assert!(s.endpoints <= MOST, "{name} {at:?}: endpoints");
    assert!(s.frames <= TURN, "{name} {at:?}: frames");
    assert!(s.along <= MOST, "{name} {at:?}: along");
}

fn probe(
    gpu: &Gpu,
    k: &Kernels,
    tools: (&Dresser, &Sweeper, &Stationer),
    id: &str,
    at: (u32, u32),
) -> Stationed {
    let (d, sw, st) = tools;
    with_dressed(gpu, k, d, id, at, |fx, dressed| {
        stationed(gpu, (sw, st), fx, dressed)
    })
}

/// The beech and the spruce at 30: the fine roots' stations on the device
/// as the pipeline prepares them.
#[test]
fn fine_roots_are_stationed_on_the_device() {
    let Some(gpu) = crate::generation::test_gpu() else {
        return;
    };
    let k = kernels(&gpu);
    let tools = (
        &Dresser::new(&gpu),
        &Sweeper::new(&gpu),
        &Stationer::new(&gpu),
    );
    for id in ["european-beech", "norway-spruce"] {
        let s = probe(&gpu, &k, tools, id, (30, 1));
        checked(id, (30, 1), &s);
        assert_eq!(s.contacts, 0, "{id}: contacts");
        assert_eq!(
            s.scanned, 0,
            "{id}: the bases' scan is the walk's numbering"
        );
        // Counts from the grown wood are equal (D2).
        assert_eq!(
            (s.tied.0, s.strays, s.counts),
            (0, (0, 0), 0),
            "{id}: counts"
        );
    }
}

/// fn-212 task 11, Astra's finding 6: a large valid divergence turns the
/// device's stations as the pipeline's, its phase reduced before it is
/// narrowed to float32.
#[test]
fn a_large_divergence_turns_the_stations_as_the_pipeline_does() {
    let Some(gpu) = crate::generation::test_gpu() else {
        return;
    };
    let k = kernels(&gpu);
    let (d, sw, st) = (Dresser::new(&gpu), Sweeper::new(&gpu), Stationer::new(&gpu));
    let mut family = telperion_core::presets::by_identity("european-beech").unwrap();
    family.age = 30.0;
    family.canopy.divergence = 1e9;
    let s = with_family(&gpu, &k, &d, family, |fx, dressed| {
        stationed(&gpu, (&sw, &st), fx, dressed)
    });
    assert!(s.device.2 > 0, "no stations");
    assert!(s.phase < 1e-3, "turns apart: {}", s.phase);
}

/// fn-212 task 11, Astra's finding 9: the stations keep their segments
/// and nothing else live; the runs' words, which only a test reads, are
/// let go with the rest of the pass's buffers.
#[test]
fn the_stations_keep_only_their_segments() {
    let Some(gpu) = crate::generation::test_gpu() else {
        return;
    };
    let k = kernels(&gpu);
    let (d, sw, st) = (Dresser::new(&gpu), Sweeper::new(&gpu), Stationer::new(&gpu));
    let steps = super::steps::Steps::new(&gpu);
    with_dressed(&gpu, &k, &d, "european-beech", (30, 1), |fx, dressed| {
        let x = &fx.reference;
        let (curve, _) = x.curve_with_contacts().unwrap();
        let surface = &fx.family.surface;
        let rows = rows(surface, curve.height);
        let swept = pollster::block_on(sw.sweep(&gpu, dressed, rows)).unwrap();
        let r = station_rows(x, surface.radial_segments.max(surface.lobes * 4), false);
        let before = gpu.ledger.live();
        let device = pollster::block_on(st.stations(&gpu, (&swept, &steps), dressed.ids, r));
        let device = device.unwrap();
        assert!(device.stations > 0);
        assert_eq!(gpu.ledger.live() - before, device.buffer.size());
    });
}

/// The probe's fixtures at 40 and 80 (`cargo test --profile ci -p
/// telperion-render stationed_at_40_and_80 -- --ignored --nocapture`,
/// under the GPU lock).
#[test]
#[ignore]
fn stationed_at_40_and_80() {
    let Some(gpu) = crate::generation::test_gpu() else {
        return;
    };
    let k = kernels(&gpu);
    let tools = (
        &Dresser::new(&gpu),
        &Sweeper::new(&gpu),
        &Stationer::new(&gpu),
    );
    // One tree where `PROBE` names it (`norway-spruce:80:1`).
    let only = std::env::var("PROBE").ok();
    for id in ["european-beech", "norway-spruce"] {
        for at in [(40, 1), (40, 7), (80, 1), (80, 7)] {
            let name = format!("{id}:{}:{}", at.0, at.1);
            if only.as_ref().is_some_and(|o| *o != name) {
                continue;
            }
            // Reported, not held: the residuals are the f32-against-f64
            // threshold class, counted per tree (TASK6.md).
            println!("{id} {at:?}: {:?}", probe(&gpu, &k, tools, id, at));
        }
    }
}

/// The residual keys of the f32-against-f64 class: a node of the
/// reference's run whose grown wood lies within float32's reach (0.01 um)
/// of a half micrometre, where the executors round it apart.
fn rounding(tree: &Tree, fine: &[&StationSegment], keys: &HashSet<u32>) -> HashSet<u32> {
    let at: HashMap<[u64; 3], usize> = (tree.nodes.iter().enumerate())
        .map(|(i, n)| {
            (
                [n.position.x, n.position.y, n.position.z].map(f64::to_bits),
                i,
            )
        })
        .collect();
    let node = |p: Vec3| at.get(&[p.x, p.y, p.z].map(f64::to_bits)).copied();
    let near = |n: usize| {
        let u = tree.grown_length(n) * 1e6;
        ((u - u.floor()) - 0.5).abs() < 0.01
    };
    (fine.iter().filter(|s| keys.contains(&s.key)))
        .filter(|s| node(s.endpoints[1]).is_some_and(near))
        .map(|s| s.key)
        .collect()
}
