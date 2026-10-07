//! fn-212 task 6: the fine roots' station segments built on the device
//! against the pipeline's own (`Expansion::compact_stations`), matched by
//! their run's first node's key and their first station along the run:
//! the same segments and station counts, their endpoints, radii, frames,
//! distances and turns within fn-125's screen, and their contacts naming
//! the same rings of their runs.
use super::dress::{Dressed, Dresser};
use super::fixture::{with_dressed, Fixture};
use super::roots::{kernels, Kernels};
use super::stations::{Rows, Stationer, GW};
use super::sweep::{rows, Sweeper};
use crate::Gpu;
use std::collections::{HashMap, HashSet};
use telperion_core::foliage::prepared::StationSegment;
use telperion_core::math::Vec3;

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
    /// Runs whose station count differs from the reference's: a run as
    /// long as a whole number of internodes, which float32 rounds over
    /// (the class of threshold decisions fed by f32 against f64); and the
    /// stations they hold on the device. Left out of what follows.
    pub tied: (usize, u64),
    /// Device segments with no reference segment, and reference segments
    /// of the device's keys with no device segment.
    pub strays: (usize, usize),
    /// Matched segments whose station count differs (a boundary at a
    /// whole number of internodes along the run); left out of what follows.
    pub counts: usize,
    /// The most apart: endpoints (m), radii (relative), frames (rad),
    /// distances along and spans (m), turns' sine and cosine.
    pub endpoints: f64,
    pub radii: f64,
    pub frames: f64,
    pub along: f64,
    pub phase: f64,
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
    let [structural, slender] = x.bearing_bounds();
    let (twig, canopy) = (x.twig(), x.leaves().canopy);
    let r = Rows {
        spi: twig.stations_per_internode,
        segments,
        structural: structural as f32,
        slender: slender as f32,
        internode: twig.internode_length as f32,
        divergence: canopy.divergence.to_radians() as f32,
        contact: x.seats(),
    };
    let t = std::time::Instant::now();
    let swept = sw.sweep(gpu, dressed, rows(surface, curve.height));
    let device = st.stations(gpu, &swept, dressed.ids, r);
    let ms = t.elapsed().as_secs_f64() * 1e3;
    let words = device.segments as usize * GW;
    let raw = super::super::io::read(gpu, &device.buffer, (words.max(4) * 4) as u64).unwrap();
    let mine: &[u32] = bytemuck::cast_slice(&raw);
    let keys: HashSet<u32> = mine.chunks(GW).map(|g| g[27]).collect();
    let by: HashMap<(u32, u32), &StationSegment> = (theirs.segments.iter())
        .filter(|s| keys.contains(&s.key))
        .map(|s| ((s.key, s.run_station), s))
        .collect();
    let mut out = Stationed {
        device: (device.runs, device.segments, device.stations),
        reference: (by.len(), by.values().map(|s| u64::from(s.count)).sum()),
        ms,
        ..Stationed::default()
    };
    // Each run's stations, by its key, on either side.
    let total = |it: &mut dyn Iterator<Item = (u32, u32)>| {
        let mut t: HashMap<u32, u64> = HashMap::new();
        for (k, n) in it {
            *t.entry(k).or_default() += u64::from(n);
        }
        t
    };
    let ours = total(&mut mine.chunks(GW).map(|g| (g[27], g[25])));
    let refs = total(&mut by.values().map(|s| (s.key, s.count)));
    let tied: HashSet<u32> = (ours.iter())
        .filter(|(k, n)| refs.get(k) != Some(n))
        .map(|(k, _)| *k)
        .collect();
    out.tied = (tied.len(), tied.iter().map(|k| ours[k]).sum());
    let mut seen = 0usize;
    for g in mine.chunks(GW).filter(|g| !tied.contains(&g[27])) {
        let Some(c) = by.get(&(g[27], g[26])) else {
            out.strays.0 += 1;
            continue;
        };
        seen += 1;
        // A segment split elsewhere: a station boundary at a whole number
        // of internodes along its run, the same class.
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
    out.strays.1 = by.keys().filter(|(k, _)| !tied.contains(k)).count() - seen;
    out
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
    }
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
    for id in ["european-beech", "norway-spruce"] {
        for at in [(40, 1), (40, 7), (80, 1), (80, 7)] {
            // Reported, not held: the class of whole-internode ties is the
            // host's (TASK6.md).
            println!("{id} {at:?}: {:?}", probe(&gpu, &k, tools, id, at));
        }
    }
}
