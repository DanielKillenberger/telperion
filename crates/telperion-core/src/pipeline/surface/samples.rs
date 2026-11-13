//! Seeding one run of the sweep: the ring centres, radii and path distances a
//! run is swept over. A trunk run starts at the root it stands on — buried by
//! the flare's own depth and widened by it — and every other run starts sunk
//! into the socket of the wood it forks off. Wherever a run's wood thins, at
//! a fork of stems or anywhere else, the drawn girth eases into it.
use super::{
    paths::{Paths, Run},
    Sample, SurfaceParams,
};
use crate::{math::Transcendental, math::Vec3, tree::Tree};

pub(super) fn sample_path(
    tree: &Tree,
    height: f64,
    params: &SurfaceParams,
    paths: &Paths,
    run: &Run,
    distance: &[f64],
    samples: &mut Vec<Sample>,
) {
    samples.clear();
    let path_nodes = &paths.nodes[run.start..run.end];
    let burial = params.flare_depth * height;
    let flare = |y: f64| {
        1.0 + (params.flare_radius - 1.0)
            * (-y.max(0.0) / (params.flare_falloff * height)).exp_fixed()
    };
    let girths = girths(tree, (path_nodes, run.trunk), distance);
    if run.trunk {
        trunk_run(tree, path_nodes, girths, distance, burial, &flare, samples);
    } else {
        branch_run(tree, params, path_nodes, girths, distance, &flare, samples);
    }
    if let Some(section) = super::section::of(tree, paths, run) {
        super::section::reshape(section, samples);
    }
}

/// Each node's girth along a run, from its first node (fn-360, host): every
/// segment across which the wood falls lays a hold of that fall, fading
/// from the ring it fell from over twice the radius it fell from or over
/// what is left of the run, whichever is shorter, and the holds add, `g(s) =
/// r(s) + Σ δ_j · w((s − s_(j−1)) / ℓ_j)`, `ℓ_j = min(2a_j, S_end −
/// s_(j−1))`, with `w` the ease curve. So a fall eases out after the segment
/// it happens across, and a run's last ring carries no hold. One rule for
/// every drop, a fork's, a growth unit's or a relay's, across a lateral that
/// takes over a bearer that ends too. Each girth reads only the run's radii
/// and distances up to it and where the run ends, and it never falls faster
/// than 1 in 1; the floor outranks the run's end. It is never below the
/// node's radius, and is the radius where no drop lies within reach. A
/// branch run's first node past where it attaches starts its own run, at
/// its own base.
fn girths<'a>(
    tree: &'a Tree,
    (path_nodes, trunk): (&'a [usize], bool),
    distance: &'a [f64],
) -> impl Iterator<Item = f64> + 'a {
    let radius = move |k: usize| tree.nodes[path_nodes[k]].radius;
    let at = move |k: usize| distance[path_nodes[k]];
    let end = at(path_nodes.len() - 1);
    // Twice the widest radius from the run's base: no fall further back
    // than that holds anything.
    let mut reach = 0.0_f64;
    // The girth drawn at the ring before, for the 1-in-1 floor.
    let mut below = 0.0_f64;
    (0..path_nodes.len()).map(move |k| {
        let base = if trunk || k == 0 { 0 } else { 1 };
        if k == base {
            reach = 0.0;
            below = 0.0;
        }
        let own = radius(k);
        reach = reach.max(2.0 * own);
        let mut girth = own;
        for j in (base + 1..=k).rev() {
            let gap = at(k) - at(j - 1);
            if gap >= reach {
                break;
            }
            let (from, to) = (radius(j - 1), radius(j));
            if from > to {
                let fade = (2.0 * from).min(end - at(j - 1));
                girth += (from - to) * ease(gap / fade);
            }
        }
        // The 1-in-1 floor (fn-354, host): kept, since without it a twig of
        // the oak at 200 falls 1.57 in 1 between two close rings (fn-360).
        if k > base {
            girth = girth.max(below - (at(k) - at(k - 1)));
        }
        below = girth;
        girth
    })
}

/// The ease curve: 1 at 0, 0 from 1 on, flat at both ends.
fn ease(t: f64) -> f64 {
    if t < 1.0 {
        (1.0 - t) * (1.0 - t) * (1.0 + 2.0 * t)
    } else {
        0.0
    }
}

fn trunk_run(
    tree: &Tree,
    path_nodes: &[usize],
    girths: impl Iterator<Item = f64>,
    distance: &[f64],
    burial: f64,
    flare: &impl Fn(f64) -> f64,
    samples: &mut Vec<Sample>,
) {
    let nodes = &tree.nodes;
    let root = &nodes[path_nodes[0]];
    if burial > 0.0 {
        samples.push(Sample {
            p: Vec3::new(root.position.x, root.position.y - burial, root.position.z),
            r: root.radius * flare(root.position.y),
            d: 0.0,
        });
    }
    for (&i, girth) in path_nodes.iter().zip(girths) {
        samples.push(Sample {
            p: nodes[i].position,
            r: girth * flare(nodes[i].position.y),
            d: distance[i],
        });
    }
}

fn branch_run(
    tree: &Tree,
    params: &SurfaceParams,
    path_nodes: &[usize],
    girths: impl Iterator<Item = f64>,
    distance: &[f64],
    flare: &impl Fn(f64) -> f64,
    samples: &mut Vec<Sample>,
) {
    let nodes = &tree.nodes;
    let depth = params.lobe_depth;
    let segments = params.radial_segments.max(params.lobes * 4) as usize;
    let swell = params.fork_swell;
    let attach = path_nodes[0];
    let first = path_nodes[1];
    let pr = nodes[attach].radius;
    let away = (nodes[first].position - nodes[attach].position).normalized();
    let inscribed = pr * (1.0 - depth) * (std::f64::consts::PI / segments as f64).cos_fixed();
    let sink = (params.fork_socket * pr).min(params.socket_containment * inscribed);
    let contained = (inscribed * inscribed - sink * sink).max(0.0).sqrt() / (1.0 + depth);
    samples.push(Sample {
        p: nodes[attach].position + away * (-sink),
        r: (nodes[first].start_radius * swell).min(contained) * flare(nodes[attach].position.y),
        d: distance[attach],
    });
    for (&i, girth) in path_nodes.iter().zip(girths).skip(1) {
        let swelling =
            1.0 + (swell - 1.0) * (-(distance[i] - distance[attach]) / pr.max(1e-9)).exp_fixed();
        samples.push(Sample {
            p: nodes[i].position,
            r: girth * swelling * flare(nodes[i].position.y),
            d: distance[i],
        });
    }
}
