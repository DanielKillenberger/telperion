//! fn-212 task 6's measurements on the engine trees at 80 (host decisions
//! 2026-10-07; `cargo test --profile ci -p telperion-core
//! stations_measured_at_80 -- --ignored --nocapture`): the bearers whose
//! bearing children change when a seam no longer counts, the fine bearing
//! runs near their frame seed's jump, and the leaves' draws in law under
//! the global stream and under each run's own; and the stations every
//! bearing run carries counted by its chord (before) and by its grown wood
//! (after, `grown_stations_at_80`).
use super::*;
use crate::pipeline::executor;
use crate::tree::{leaf_stream, mix32};

/// A draw of the leaves' streams, as `pack.wgsl` takes it: a uniform in
/// [0, 1) from the stream's `ordinal`th step.
fn draw(stream: u32, ordinal: u32) -> f64 {
    let z = mix32(stream.wrapping_add(ordinal.wrapping_add(1).wrapping_mul(0x9e37_79b9)));
    f64::from(z) / 4_294_967_296.0
}

/// The largest gap between the empirical distribution of `u` and the
/// uniform one (Kolmogorov-Smirnov).
fn ks(mut u: Vec<f64>) -> f64 {
    u.sort_by(f64::total_cmp);
    let n = u.len() as f64;
    u.iter().enumerate().fold(0.0, |d, (i, &x)| {
        d.max((x - i as f64 / n).abs())
            .max(((i + 1) as f64 / n - x).abs())
    })
}

#[test]
#[ignore]
fn stations_measured_at_80() {
    for id in ["european-beech", "norway-spruce"] {
        let mut family = crate::presets::by_identity(id).unwrap();
        family.age = 80.0;
        family.skeleton.seed = 1;
        let x = executor::grow(&family).unwrap().expansion().unwrap();
        let (tree, leaves, twig) = (x.tree(), x.leaves(), x.twig());
        let p = leaves.canopy;
        // Bearers whose bearing children change, and of them those whose
        // own run now carries on.
        let bearing = bearing_of(tree, p);
        let (old, new) = (
            bearing_children(tree, &bearing, true),
            bearing_children(tree, &bearing, false),
        );
        let changed: Vec<usize> = (0..old.len()).filter(|&i| old[i].0 != new[i].0).collect();
        let carries = changed
            .iter()
            .filter(|&&i| {
                bearing(i)
                    && old[i].0 != 1
                    && new[i].0 == 1
                    && tree.nodes[i].branch == tree.nodes[new[i].1].branch
            })
            .count();
        // The fine bearing runs' first tangents against their seed (the
        // attachment's side), and against the old seed's near-ties.
        let fine = tree.in_fine_roots();
        let runs = bearing_runs(tree, p);
        let (mut seeded, mut near, mut tie) = (0usize, [0usize; 3], 0usize);
        for run in &runs {
            let Some(side) = tree.seed_side(&fine, run[0], run[1]) else {
                continue;
            };
            let t = tree.nodes[run[1]].position - tree.nodes[run[0]].position;
            let t = t.normalized();
            seeded += 1;
            let off = side
                .normalized()
                .dot(t)
                .abs()
                .clamp(0.0, 1.0)
                .acos()
                .to_degrees();
            for (k, lim) in [10.0, 5.0, 1.0].iter().enumerate() {
                near[k] += usize::from(off < *lim);
            }
            let mut a = [t.x.abs(), t.y.abs(), t.z.abs()];
            a.sort_by(f64::total_cmp);
            tie += usize::from(a[1] - a[0] < 1e-2);
        }
        // Each leaf's scale draw and first scatter draw, under the global
        // stream and under each run's own.
        let plan = runs_of(tree, leaves, twig);
        let stride = if p.scatter > 0.0 { 4 } else { 1 };
        let scale_at = if p.scatter > 0.0 { 3 } else { 0 };
        let global = leaves.seed ^ 0x2c9e_1a7f;
        let (mut before, mut after) = ([Vec::new(), Vec::new()], [Vec::new(), Vec::new()]);
        let mut at = 0u32;
        for (first, count) in plan {
            let stream = leaf_stream(leaves.seed, tree.key(first));
            for k in 0..count {
                let (g, r) = ((at + k) * stride, k * stride);
                before[0].push(draw(global, g + scale_at));
                after[0].push(draw(stream, r + scale_at));
                if stride == 4 {
                    before[1].push(draw(global, g));
                    after[1].push(draw(stream, r));
                }
            }
            at += count;
        }
        println!(
            "{id} 80 1: bearers changed {} (of them carrying their run on now {carries}); \
             fine bearing runs {seeded}, their first tangent within 10/5/1 deg of the seed side {near:?}, \
             near-ties of the old seed {tie}; leaves {}: KS of the scale draw against uniform before {:.2e} after {:.2e}, \
             of the scatter draw before {:.2e} after {:.2e}",
            changed.len(),
            before[0].len(),
            ks(before[0].clone()),
            ks(after[0].clone()),
            ks(before[1].clone()),
            ks(after[1].clone()),
        );
    }
}

/// Each run's first node and its station count.
fn runs_of(
    tree: &Tree,
    leaves: crate::pipeline::executor::LeafInput,
    twig: TwigPlacement,
) -> Vec<(usize, u32)> {
    let runs = runs(tree, leaves.envelope, leaves.canopy, Some(twig))
        .unwrap()
        .unwrap();
    runs.iter().map(|r| (r.nodes[1], r.count)).collect()
}

/// Stations by the chord against stations by the grown wood (host
/// decision 2026-10-07, whole-internode ties): per tree, the bearing runs
/// whose count moves, the stations before and after, and the runs whose
/// segments split elsewhere (`cargo test --profile ci -p telperion-core
/// grown_stations_at_80 -- --ignored --nocapture`).
#[test]
#[ignore]
fn grown_stations_at_80() {
    for id in ["european-beech", "norway-spruce"] {
        for seed in [1, 7] {
            let mut family = crate::presets::by_identity(id).unwrap();
            family.age = 80.0;
            family.skeleton.seed = seed;
            let x = executor::grow(&family).unwrap().expansion().unwrap();
            let (tree, leaves, twig) = (x.tree(), x.leaves(), x.twig());
            let runs = runs(tree, leaves.envelope, leaves.canopy, Some(twig))
                .unwrap()
                .unwrap();
            let unit = twig.internode_length;
            let (mut before, mut after, mut moved, mut split) = (0u64, 0u64, 0usize, 0usize);
            let (mut longer, mut shorter) = (0usize, 0usize);
            for run in &runs {
                // The chord's count and split, as `runs` took them before.
                let mut chord = vec![0.0];
                for pair in run.nodes.windows(2) {
                    let step = tree.nodes[pair[0]]
                        .position
                        .distance(tree.nodes[pair[1]].position);
                    chord.push(chord.last().unwrap() + step);
                }
                let length = *chord.last().unwrap();
                let internodes = (length / unit - 1e-9).ceil().max(1.0) as u32;
                let old = Run {
                    nodes: run.nodes.clone(),
                    along: chord.iter().map(|c| c * 1e6).collect(),
                    internodes,
                    count: internodes * twig.stations_per_internode,
                    leaflets: run.leaflets,
                };
                before += u64::from(old.count);
                after += u64::from(run.count);
                moved += usize::from(old.count != run.count);
                longer += usize::from(run.count > old.count);
                shorter += usize::from(run.count < old.count);
                let parts = |r: &Run| -> Vec<(u32, u32)> {
                    (0..r.nodes.len() - 1)
                        .map(|s| r.segment_stations(s, twig))
                        .collect()
                };
                split += usize::from(old.count == run.count && parts(&old) != parts(run));
            }
            println!(
                "{id} 80 {seed}: bearing runs {}; stations by the chord {before}, by the grown wood \
                 {after} ({:+.3} %); runs counted differently {moved} (more {longer}, fewer \
                 {shorter}); runs of equal count split elsewhere {split}",
                runs.len(),
                (after as f64 / before as f64 - 1.0) * 100.0,
            );
        }
    }
}
