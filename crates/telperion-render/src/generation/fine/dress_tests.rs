//! fn-212 task 4: fine roots drawn in the tree's frame on the device, and
//! the whole-tree folds over them combined with the pipeline's own,
//! against the pipeline's CPU reference build of the same tree: the same
//! nodes within fn-125's screen, the folds within it, and the envelope and
//! leaf box the pipeline reads. The folds combine exactly
//! (`telperion-core`, `parts_of_a_tree_combine_to_the_whole`).
use super::dress::{Dressed, Dresser, CAP, CA_WORDS, CV, LANDED, MADE};
use super::fixture::{with_dressed, Fixture};
use super::roots::{kernels, Kernels};
use crate::Gpu;
use std::collections::HashMap;
use telperion_core::math::Vec3;
use telperion_core::pipeline::executor::{beyond, bins, envelope, Folds};

/// fn-125's screen, in metres.
const MOST: f64 = 50e-6;

/// What the probe measured on one tree.
// Read through its `Debug` print.
#[allow(dead_code)]
#[derive(Debug, Default)]
pub(super) struct Probe {
    /// The fine roots' nodes drawn on the device, and the reference's.
    pub nodes: (usize, usize),
    /// The device's nodes with no node of the reference's fine roots
    /// within the screen, or none of the reference's at all; and the
    /// reference's fine roots' nodes with no device node.
    pub strays: (usize, usize, usize),
    /// The roots laid on the CPU, and the roots routed there before
    /// upload by D3(a)'s bound.
    pub landed: usize,
    /// The folds combined from the device's and the pipeline's own nodes'
    /// against the whole reference tree's: the farthest apart of the
    /// height, the crown's lowest and the widest reach and its height,
    /// metres.
    pub folds: f64,
    /// The combined envelope against the reference's: its height and
    /// widest reach (m), its shares, and its shoulder (relative).
    pub envelope: (f64, f64, f64),
    /// The combined leaf box's corners against the reference's (m).
    pub boxed: f64,
    /// The crown nodes the device handed on, and whether more were kept.
    pub kept: (usize, bool),
    /// Milliseconds: the device's drawing and folds, round trips included.
    pub ms: f64,
}

/// How many of `points` have none of `against` within the screen.
pub(super) fn strays(points: &[Vec3], against: &[Vec3]) -> usize {
    let cell = |v: Vec3| {
        let c = |x: f64| (x / 1e-3).floor() as i64;
        (c(v.x), c(v.y), c(v.z))
    };
    let mut grid: HashMap<(i64, i64, i64), Vec<Vec3>> = HashMap::new();
    for &v in against {
        grid.entry(cell(v)).or_default().push(v);
    }
    let near = |v: Vec3| {
        let (x, y, z) = cell(v);
        (-1..=1).any(|a| {
            (-1..=1).any(|b| {
                (-1..=1).any(|c| {
                    grid.get(&(x + a, y + b, z + c))
                        .is_some_and(|vs| vs.iter().any(|&w| (w - v).length() <= MOST))
                })
            })
        })
    };
    points.iter().filter(|&&v| !near(v)).count()
}

fn apart(a: Vec3, b: Vec3) -> f64 {
    (a - b).length()
}

/// The fine roots of the preset `id` at `age` and `seed` settled on the
/// device, drawn and folded there, against the pipeline's build.
pub(super) fn probe(gpu: &Gpu, k: &Kernels, d: &Dresser, id: &str, at: (u32, u32)) -> Probe {
    with_dressed(gpu, k, d, id, at, |fx, dressed| folded(gpu, d, fx, dressed))
}

/// The device's folds against the pipeline's, on a fixture drawn.
pub(super) fn folded(gpu: &Gpu, d: &Dresser, fx: &Fixture, dressed: &Dressed) -> Probe {
    let (reference, fine) = (&fx.reference, &fx.fine);
    let tree = reference.tree();
    let d0 = std::time::Instant::now();
    // The device's folds combined with the pipeline's own nodes'.
    let device = pollster::block_on(dressed.extent(gpu, d)).unwrap();
    let cpu = Folds::of(&tree.nodes, |i| !fine[i]);
    let folds = cpu.with(device);
    let shape = folds.shape();
    let terms = reference.wood_reach().unwrap();
    let (device_bins, device_box) =
        pollster::block_on(dressed.crowns(gpu, d, shape, terms)).unwrap();
    let own = || {
        let nodes = tree.nodes.iter().enumerate();
        nodes.filter(|&(i, _)| !fine[i]).map(|(_, n)| n.position)
    };
    let most: Vec<f64> = (bins(own(), shape).into_iter())
        .zip(device_bins)
        .map(|(a, b)| a.max(b))
        .collect();
    let (kept, over) = pollster::block_on(dressed.sift(gpu, d, shape, &beyond(&most))).unwrap();
    let ms = d0.elapsed().as_secs_f64() * 1e3;
    let grown = envelope(folds, own().chain(kept.iter().copied()));
    let (lo, hi) = reference.wood_box(|i| !fine[i]).unwrap();
    let (dl, dh) = device_box;
    let lo = Vec3::new(lo.x.min(dl.x), lo.y.min(dl.y), lo.z.min(dl.z));
    let hi = Vec3::new(hi.x.max(dh.x), hi.y.max(dh.y), hi.z.max(dh.z));
    // Against the reference: its envelope, its box, its fine part's folds.
    let want = reference.leaves().envelope;
    let boxed = telperion_core::foliage::Reference::spanning(lo, hi);
    let rb = reference.reference();
    let whole = Folds::of(&tree.nodes, |_| true);
    let fold_apart = |a: Folds, b: Folds| {
        let w = |f: Folds| f.widest.unwrap_or_default();
        [
            (a.height - b.height).abs(),
            (a.low - b.low).abs(),
            (w(a).0 - w(b).0).abs(),
            (w(a).1 - w(b).1).abs(),
        ]
        .into_iter()
        .fold(0.0, f64::max)
    };
    // Every drawn node, read back for the comparison alone.
    let mut nodes = Vec::new();
    for w in dressed.waves.iter().filter(|w| !w.stubs) {
        let (buffer, axes, phyts) = (&w.buffer, &w.axes, &w.phyts);
        let words = *axes as usize * CA_WORDS + *phyts as usize * CV;
        let raw = super::super::io::read(gpu, buffer, (words.max(4) * 4) as u64).unwrap();
        let floats: &[f32] = bytemuck::cast_slice(&raw);
        for r in floats[*axes as usize * CA_WORDS..]
            .chunks(CV)
            .take(*phyts as usize)
        {
            let flags = r[6].to_bits();
            if flags & MADE != 0 && flags & LANDED == 0 {
                nodes.push(Vec3::new(f64::from(r[0]), f64::from(r[1]), f64::from(r[2])));
            }
        }
    }
    let theirs: Vec<Vec3> = (tree.nodes.iter().enumerate())
        .filter(|&(i, _)| fine[i])
        .map(|(_, n)| n.position)
        .collect();
    let all: Vec<Vec3> = tree.nodes.iter().map(|n| n.position).collect();
    Probe {
        nodes: (nodes.len(), theirs.len()),
        strays: (
            strays(&nodes, &theirs),
            strays(&nodes, &all),
            strays(&theirs, &nodes),
        ),
        landed: fx.landed,
        folds: fold_apart(folds, whole),
        envelope: (
            (grown.height - want.height)
                .abs()
                .max((grown.spread * grown.height - want.spread * want.height).abs()),
            [
                grown.crown_base - want.crown_base,
                grown.fullness - want.fullness,
            ]
            .into_iter()
            .fold(0.0, |m: f64, v| m.max(v.abs())),
            (grown.shoulder / want.shoulder - 1.0).abs(),
        ),
        boxed: apart(boxed.min, rb.min).max(apart(boxed.max(), rb.max())),
        kept: (kept.len(), over),
        ms: ms + fx.dress_ms,
    }
}

pub(super) fn checked(name: &str, at: (u32, u32), p: &Probe) {
    println!("{name} {at:?}: {p:?}");
    assert!(
        !p.kept.1 && p.kept.0 < CAP as usize,
        "{name} {at:?}: the sift's bound"
    );
    assert!(p.folds <= MOST, "{name} {at:?}: the device's folds");
    assert!(p.envelope.0 <= MOST, "{name} {at:?}: the envelope");
    assert!(p.boxed <= 2.0 * MOST, "{name} {at:?}: the leaf box");
}

/// The probe's fixtures, beech and spruce at 40 and 80, seeds 1 and 7
/// (`cargo test --profile ci -p telperion-render dressed_at_40_and_80 --
/// --ignored --nocapture`, under the GPU lock).
#[test]
#[ignore]
fn dressed_at_40_and_80() {
    let Some(gpu) = crate::generation::test_gpu() else {
        return;
    };
    let (k, d) = (kernels(&gpu), Dresser::new(&gpu));
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
            let p = probe(&gpu, &k, &d, id, at);
            checked(id, at, &p);
        }
    }
}

/// fn-212 task 11, Astra's finding 3: of the nodes tied at the widest
/// reach, one supplies every coordinate of the widest node.
#[test]
fn one_widest_node_supplies_all_its_coordinates() {
    let Some(gpu) = crate::generation::test_gpu() else {
        return;
    };
    let d = Dresser::new(&gpu);
    let r = 2.0f32;
    let nodes: Vec<[f32; 3]> = (0..1u32 << 16)
        .map(|k| {
            let y = (k % 997) as f32 * 0.01;
            match k % 4 {
                0 => [r, y, 0.0],
                1 => [0.0, y, r],
                2 => [-r, y, 0.0],
                _ => [0.0, y, -r],
            }
        })
        .collect();
    for _ in 0..8 {
        let dressed = Dressed::of_nodes(&gpu, &nodes).unwrap();
        let folds = pollster::block_on(dressed.extent(&gpu, &d)).unwrap();
        let (reach, y) = folds.widest.expect("a widest node");
        assert_eq!(reach, f64::from(r), "the widest node's coordinates mixed");
        // The least id among the tied: node 0, at height 0.
        assert_eq!(y, 0.0);
    }
}
