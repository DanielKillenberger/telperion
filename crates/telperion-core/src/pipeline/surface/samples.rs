//! Seeding one run of the sweep: the ring centres, radii and path distances a
//! run is swept over. A trunk run starts at the root it stands on — buried by
//! the flare's own depth and widened by it — and every other run starts sunk
//! into the socket of the wood it forks off. A run that carries on through a
//! fork of stems eases the trunk's girth into the stem it follows.
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
    let girths = girths(tree, path_nodes, &paths.forks, distance);
    if run.trunk {
        trunk_run(tree, path_nodes, girths, distance, burial, &flare, samples);
    } else {
        branch_run(tree, params, path_nodes, girths, distance, &flare, samples);
    }
    if let Some(section) = super::section::of(tree, paths, run) {
        super::section::reshape(section, samples);
    }
}

/// Each node's girth along a run, from its first node: the node's own radius,
/// except over the fork's diameter past a fork of stems the run carries on
/// through, where it eases from the trunk's girth at the fork into the stem's
/// own. The run's first node is where it attaches, never a fork it passes.
fn girths<'a>(
    tree: &'a Tree,
    path_nodes: &'a [usize],
    forks: &'a [bool],
    distance: &'a [f64],
) -> impl Iterator<Item = f64> + 'a {
    let nodes = &tree.nodes;
    let mut fork = None;
    path_nodes.iter().enumerate().map(move |(k, &i)| {
        let own = nodes[i].radius;
        let girth = fork.map_or(own, |f: usize| {
            let trunk = nodes[f].radius;
            let t = (distance[i] - distance[f]) / (2.0 * trunk).max(1e-9);
            let held = if t < 1.0 {
                (1.0 - t) * (1.0 - t) * (1.0 + 2.0 * t)
            } else {
                0.0
            };
            own + (trunk - own).max(0.0) * held
        });
        if k > 0 && forks[i] {
            fork = Some(i);
        }
        girth
    })
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
