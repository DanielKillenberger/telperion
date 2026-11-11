//! fn-354: the drawing eases a change of section along a stem. Where the
//! wood thins from one node to the next going up one stem, the drawn girth
//! falls no faster than 1 in 1; a lateral's base is drawn as it was.
use super::{paths::paths, samples::sample_path, Sample, SurfaceParams};
use crate::{
    math::Vec3,
    tree::{Node, Tree},
};

/// Node spacing along the stem.
const STEP: f64 = 0.05;

/// A stem of 0.5 m for a metre, then 0.1 m for two, one axis's run; and a
/// lateral of 0.02 m leaving it at its tenth node, wider (`wide`) or not.
fn stepped(wide: bool) -> Tree {
    let mut nodes = vec![Node {
        radius: 0.5,
        start_radius: 0.5,
        ..Node::root()
    }];
    let stem = 1u32;
    for k in 0..60 {
        let i = nodes.len();
        let radius = if k < 20 { 0.5 } else { 0.1 };
        nodes.push(Node {
            position: Vec3::Y * (STEP * (k + 1) as f64),
            parent: Some((i - 1) as u32),
            radius,
            start_radius: radius,
            base_radius: 0.5,
            branch: stem,
            ..Node::root()
        });
    }
    let radius = if wide { 0.3 } else { 0.02 };
    for k in 0..10 {
        let i = nodes.len();
        let parent = if k == 0 { 10 } else { i - 1 };
        nodes.push(Node {
            position: nodes[10].position + Vec3::X * (STEP * (k + 1) as f64),
            parent: Some(parent as u32),
            radius,
            start_radius: radius,
            base_radius: radius,
            branch: 61,
            ..Node::root()
        });
    }
    Tree {
        crossover: nodes.len(),
        nodes,
        ..Tree::default()
    }
}

/// A surface with no flare and no swell at a socket, so the radii read are
/// the girths themselves.
fn plain() -> SurfaceParams {
    SurfaceParams {
        flare_radius: 1.0,
        fork_swell: 1.0,
        ..SurfaceParams::default()
    }
}

/// Each run's first node and its rings, one a node.
fn rings(tree: &Tree) -> Vec<(usize, Vec<Sample>)> {
    let paths = paths(tree).unwrap();
    let mut distance = vec![0.0; tree.nodes.len()];
    for (i, n) in tree.nodes.iter().enumerate().skip(1) {
        let p = n.parent.unwrap() as usize;
        distance[i] = distance[p] + tree.nodes[p].position.distance(n.position);
    }
    let mut samples = Vec::new();
    paths
        .runs
        .iter()
        .map(|run| {
            sample_path(tree, 18.0, &plain(), &paths, run, &distance, &mut samples);
            let n = run.end - run.start;
            (
                paths.nodes[run.start],
                samples[samples.len() - n..].to_vec(),
            )
        })
        .collect()
}

/// Along the stem the drawn girth never falls faster than 1 in 1: the drop
/// of 0.4 m is a shoulder 0.4 m long, and the wood below it and well above
/// it is drawn at its own radius.
#[test]
fn a_stem_never_thins_faster_than_one_in_one() {
    let tree = stepped(false);
    let runs = rings(&tree);
    let (_, stem) = runs.iter().find(|(first, _)| *first == 0).unwrap();
    for pair in stem.windows(2) {
        let fall = pair[0].r - pair[1].r;
        assert!(
            fall <= pair[1].d - pair[0].d + 1e-12,
            "{fall} at {}",
            pair[1].d
        );
    }
    let at = |y: f64| stem.iter().find(|s| (s.p.y - y).abs() < 1e-9).unwrap().r;
    assert_eq!(at(1.0), 0.5, "below the drop");
    assert!((at(1.2) - 0.3).abs() < 1e-9, "the shoulder: {}", at(1.2));
    assert_eq!(at(1.6), 0.1, "above the shoulder");
}

/// A lateral's base, thinner or wider than its bearer's shoulder, is drawn
/// at its own wood: the easing never runs from a bearer into a lateral.
#[test]
fn a_laterals_base_is_untouched() {
    for wide in [false, true] {
        let tree = stepped(wide);
        let runs = rings(&tree);
        let (_, lateral) = runs.iter().find(|(first, _)| *first == 10).unwrap();
        let own = tree.nodes[61].radius;
        let radii: Vec<f64> = lateral[1..].iter().map(|s| s.r).collect();
        assert!(radii.iter().all(|&r| r == own), "{wide}: {radii:?}");
    }
}
