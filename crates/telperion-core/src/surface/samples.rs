//! Seeding one run of the sweep: the ring centres, radii and path distances a
//! run is swept over. A trunk run starts at the root it stands on — buried by
//! the flare's own depth and widened by it — and every other run starts sunk
//! into the socket of the wood it forks off.
use super::{Sample, SurfaceParams};
use crate::{math::Transcendental, math::Vec3, tree::Tree};

pub(super) fn sample_path(
    tree: &Tree,
    height: f64,
    params: &SurfaceParams,
    path_nodes: &[usize],
    trunk: bool,
    distance: &[f64],
    samples: &mut Vec<Sample>,
) {
    samples.clear();
    let burial = params.flare_depth * height;
    let flare = |y: f64| {
        1.0 + (params.flare_radius - 1.0)
            * (-y.max(0.0) / (params.flare_falloff * height)).exp_fixed()
    };
    if trunk {
        trunk_run(tree, path_nodes, distance, burial, &flare, samples);
    } else {
        branch_run(tree, params, path_nodes, distance, &flare, samples);
    }
}

fn trunk_run(
    tree: &Tree,
    path_nodes: &[usize],
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
    for &i in path_nodes {
        samples.push(Sample {
            p: nodes[i].position,
            r: nodes[i].radius * flare(nodes[i].position.y),
            d: distance[i],
        });
    }
}

fn branch_run(
    tree: &Tree,
    params: &SurfaceParams,
    path_nodes: &[usize],
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
    let sink = (params.fork_socket * pr).min(0.9 * inscribed);
    let contained = (inscribed * inscribed - sink * sink).max(0.0).sqrt() / (1.0 + depth);
    samples.push(Sample {
        p: nodes[attach].position + away * (-sink),
        r: (nodes[first].start_radius * swell).min(contained) * flare(nodes[attach].position.y),
        d: distance[attach],
    });
    for &i in &path_nodes[1..] {
        let swelling =
            1.0 + (swell - 1.0) * (-(distance[i] - distance[attach]) / pr.max(1e-9)).exp_fixed();
        samples.push(Sample {
            p: nodes[i].position,
            r: nodes[i].radius * swelling * flare(nodes[i].position.y),
            d: distance[i],
        });
    }
}
