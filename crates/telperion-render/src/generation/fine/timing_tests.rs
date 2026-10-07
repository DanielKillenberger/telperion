//! fn-212 task 7, the 3x measurement as a composite (`cargo test --profile
//! ci -p telperion-render three_x_composite -- --ignored --nocapture`,
//! `PROBE=norway-spruce:80:1`, under the GPU lock, one tree a process).
//!
//! Five builds a side, alternately, one a process (`SIDE`), on the same
//! settled tree:
//! - **today:** the resident path from the converted tree to draw-ready
//!   (the generator's total less its skeleton, the submit and the drain),
//!   as task 2's timing probe measures it;
//! - **the device's:** the pipeline's own stages on the structure above
//!   the cut and the landed roots alone (the converted tree with the
//!   device's roots taken out), the device's dress, folds, sweep,
//!   stations, rings, ring positions and placement, and the copy of the
//!   fine block's leaves and curve into one buffer, each waited on.
//!
//! The conversion sits outside both (task 2's boundary); TASK7.md adds
//! baseline B's to today's side, and its share by nodes to the device's.
//! The handoff into the renderer's curve form is not built: its copy is
//! measured, its conversion kernel is not.
use super::super::{data, io, Delivery, Generator, Metrics};
use super::dress::Dressed;
use super::dress::Dresser;
use super::fixture::{with_dressed, Fixture};
use super::rings::Ringer;
use super::roots::{kernels, Kernels};
use super::stations::Stationer;
use super::stations_tests::station_rows;
use super::sweep::{rows, Sweeper, PW};
use crate::Gpu;
use std::borrow::Cow;
use std::time::Instant;
use telperion_core::foliage::prepared::PreparedStations;
use telperion_core::pipeline::executor::{self, beyond, bins, Folds};
use telperion_core::presets::Family;
use telperion_core::tree::Tree;

fn ms(t: Instant) -> f64 {
    t.elapsed().as_secs_f64() * 1e3
}

fn wait(gpu: &Gpu) {
    gpu.device
        .poll(wgpu::PollType::wait_indefinitely())
        .unwrap();
}

/// `tree` with the nodes `fine` marks taken out: the structure above the
/// cut and the landed roots, as the CPU side converts them.
fn above(tree: &Tree, fine: &[bool]) -> Tree {
    let mut at = vec![u32::MAX; tree.nodes.len()];
    let mut nodes = Vec::new();
    for (i, n) in tree.nodes.iter().enumerate().filter(|&(i, _)| !fine[i]) {
        at[i] = nodes.len() as u32;
        nodes.push(n.clone());
    }
    for n in &mut nodes {
        n.parent = n.parent.map(|p| at[p as usize]);
        n.branch = at[n.branch as usize];
    }
    let keep = |i: &usize| !fine[*i];
    let pick = |v: usize| (0..v).filter(keep);
    let n = tree.nodes.len();
    Tree {
        crossover: nodes.len(),
        seams: (tree.seams.iter())
            .filter(|&&s| !fine[s as usize])
            .map(|&s| at[s as usize])
            .collect(),
        frames: pick(tree.frames.len().min(n))
            .map(|i| tree.frames[i])
            .collect(),
        keys: pick(tree.keys.len().min(n)).map(|i| tree.keys[i]).collect(),
        grown: pick(tree.grown.len().min(n))
            .map(|i| tree.grown[i])
            .collect(),
        pipe: pick(tree.pipe.len().min(n)).map(|i| tree.pipe[i]).collect(),
        nodes,
        ..Tree::default()
    }
}

/// One build of today's resident path: milliseconds from the converted
/// tree to draw-ready.
fn today(gpu: &Gpu, family: &Family) -> f64 {
    // A renderer of its own, dropped with the tree it holds.
    let mut renderer = crate::Renderer::new(gpu.clone(), crate::STILL_FORMAT);
    let gen = Generator::new(&renderer).unwrap();
    let prepared = gen.prepare(family, Delivery::Resident).unwrap();
    let m = &prepared.metrics;
    let inside = m.total_ms - m.skeleton_ms;
    let t = Instant::now();
    renderer.submit_prepared(prepared).unwrap();
    renderer.gpu().queue.submit([]);
    wait(renderer.gpu());
    inside + ms(t)
}

/// The device's side, by stage (ms).
#[derive(Debug, Default)]
#[allow(dead_code)]
struct Candidate {
    cpu: f64,
    dress: f64,
    folds: f64,
    sweep: f64,
    stations: f64,
    rings: f64,
    place: f64,
    copy: f64,
    total: f64,
    /// The share of the tree's nodes the CPU side converts.
    share: f64,
}

/// The CPU side: the pipeline's own stages on `partial`, placed and its
/// curve uploaded; and the rings' angular samples, which the fine block's
/// ring positions read too.
fn cpu_side(gpu: &Gpu, gen: &Generator, fx: &Fixture, partial: Tree) -> (f64, Vec<[f32; 4]>) {
    let t = Instant::now();
    let x = executor::expand(partial, &fx.family).unwrap();
    let (curve, shared) = x.curve_with_contacts().unwrap();
    let theirs = x.compact_stations(&shared).unwrap().unwrap();
    let angular = shared.surface().angular.clone();
    let mut m = Metrics::default();
    let wood = pollster::block_on(gen.emit_positions(shared.into_surface(), &mut m));
    let wood = wood.unwrap().expect("ring positions");
    let p = PreparedStations {
        segments: theirs.segments,
        count: theirs.count,
        ring_size: theirs.ring_size,
        rings: Cow::Borrowed(&wood.positions),
    };
    let (l, e) = (x.leaves(), x.element());
    let r = pollster::block_on(gen.compute_buffer_async(p, &l, x.twig(), e, x.reference(), &mut m));
    drop(r.unwrap());
    let _curve = crate::curve::CurveGpu::new(gpu, &curve);
    wait(gpu);
    (ms(t), angular)
}

/// The device's folds as the pipeline combines them with its own nodes'.
fn folds(gpu: &Gpu, d: &Dresser, fx: &Fixture, dressed: &Dressed) {
    let (x, fine) = (&fx.reference, &fx.fine);
    let tree = x.tree();
    let device = dressed.extent(gpu, d);
    let shape = Folds::of(&tree.nodes, |i| !fine[i]).with(device).shape();
    let (device_bins, _) = dressed.crowns(gpu, d, shape, x.wood_reach().unwrap());
    let own = (tree.nodes.iter().enumerate())
        .filter(|&(i, _)| !fine[i])
        .map(|(_, n)| n.position);
    let most: Vec<f64> = (bins(own, shape).into_iter())
        .zip(device_bins)
        .map(|(a, b)| a.max(b))
        .collect();
    dressed.sift(gpu, d, shape, &beyond(&most));
}

fn candidate(
    gpu: &Gpu,
    gen: &Generator,
    d: &Dresser,
    fx: &Fixture,
    dressed: &Dressed,
) -> Candidate {
    let (sw, st, rg) = (Sweeper::new(gpu), Stationer::new(gpu), Ringer::new(gpu));
    let x = &fx.reference;
    let mut c = Candidate {
        share: fx.fine.iter().filter(|&&f| !f).count() as f64 / fx.fine.len() as f64,
        dress: fx.dress_ms,
        ..Candidate::default()
    };
    let (cpu, angular) = cpu_side(gpu, gen, fx, above(x.tree(), &fx.fine));
    c.cpu = cpu;
    let t = Instant::now();
    folds(gpu, d, fx, dressed);
    c.folds = ms(t);
    let surface = &fx.family.surface;
    let height = x.leaves().envelope.height;
    let t = Instant::now();
    let swept = sw.sweep(gpu, dressed, rows(surface, height));
    c.sweep = ms(t);
    let segments = surface.radial_segments.max(surface.lobes * 4);
    let t = Instant::now();
    let device = st.stations(gpu, &swept, dressed.ids, station_rows(x, segments));
    c.stations = ms(t);
    swept.nodes.destroy();
    let t = Instant::now();
    let twist = std::f64::consts::TAU * surface.twist_rate / height;
    let rung = rg.rings(gpu, &swept, segments, twist as f32);
    let rings = (&rung.buffer, rung.rings, rung.vertices);
    let positions = gen.ring_vertices(rings, segments, &angular).unwrap();
    wait(gpu);
    rung.buffer.destroy();
    c.rings = ms(t);
    let t = Instant::now();
    let (l, e) = (x.leaves(), x.element());
    let counts = (device.stations, device.segments, segments);
    let config = data::config(&l, x.twig(), counts, e, x.reference());
    let placed = gen.place_leaves(config, (&device.buffer, &positions), e, device.stations);
    let placed = placed.unwrap();
    io::read(gpu, &placed.summary, 64).unwrap();
    c.place = ms(t);
    // The fine block's leaves and curve copied into buffers of their own,
    // as the handoff would place them after the CPU block's.
    let t = Instant::now();
    let usage = wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST;
    let sizes = [
        placed.leaves.size(),
        u64::from(swept.points) * PW as u64 * 4,
        swept.cluster_buffer.size(),
    ];
    let into: Vec<wgpu::Buffer> = (sizes.iter())
        .map(|&s| io::buffer(gpu, "handoff", s.max(4), usage, &[]).unwrap())
        .collect();
    let mut e = gpu.device.create_command_encoder(&Default::default());
    let from = [&placed.leaves, &swept.point_buffer, &swept.cluster_buffer];
    for ((f, i), s) in from.iter().zip(&into).zip(sizes) {
        e.copy_buffer_to_buffer(f, 0, i, 0, s);
    }
    gpu.queue.submit([e.finish()]);
    wait(gpu);
    c.copy = ms(t);
    c.total = c.cpu + c.dress + c.folds + c.sweep + c.stations + c.rings + c.place + c.copy;
    c
}

/// One build of one side (`SIDE`: `today` or `device`), one a process: the
/// kernels' pool keeps a settled tree's buffers, and two builds of the
/// spruce at 80 do not fit the device together.
fn probe(gpu: &Gpu, k: &Kernels, gen: &Generator, id: &str, at: (u32, u32)) {
    let side = std::env::var("SIDE").unwrap_or_else(|_| "device".into());
    if side == "today" {
        let mut family = telperion_core::presets::by_identity(id).unwrap();
        (family.age, family.skeleton.seed) = (f64::from(at.0), at.1);
        println!("{id} {at:?}: today {:.1} ms", today(gpu, &family));
        return;
    }
    let d = Dresser::new(gpu);
    let b = with_dressed(gpu, k, &d, id, at, |fx, dressed| {
        candidate(gpu, gen, &d, fx, dressed)
    });
    println!("{id} {at:?}: device {b:?}");
}

#[test]
#[ignore]
fn three_x_composite() {
    let Some(gpu) = crate::generation::test_gpu() else {
        return;
    };
    let k = kernels(&gpu);
    let renderer = crate::Renderer::new(gpu.clone(), crate::STILL_FORMAT);
    let gen = Generator::new(&renderer).unwrap();
    let only = std::env::var("PROBE").unwrap_or_else(|_| "norway-spruce:80:1".into());
    let parts: Vec<&str> = only.split(':').collect();
    let at = (parts[1].parse().unwrap(), parts[2].parse().unwrap());
    probe(&gpu, &k, &gen, parts[0], at);
}
