//! fn-354: the drawing eases a change of section along a run (host,
//! corrected 11 Nov evening). Where the wood thins from one node to the
//! next going up the run, across a lateral that takes over a bearer that
//! ends, the drawn girth falls no faster than 1 in 1; a lateral that starts
//! its own run is drawn at its own base.
use super::{paths::paths, samples::sample_path, Sample, SurfaceParams};
use crate::{
    math::Vec3,
    tree::{Node, Tree},
};

/// Node spacing along the stem.
const STEP: f64 = 0.05;

/// A stem of 0.5 m for a metre and a lateral of 0.02 m (`wide`: 0.3 m)
/// leaving it at its tenth node. Above the metre the run goes on at 0.1 m
/// for two more: the same axis, or (`taken`) a lateral that takes over
/// the stem, which ends there.
fn stepped(wide: bool, taken: bool) -> Tree {
    let mut nodes = vec![Node {
        radius: 0.5,
        start_radius: 0.5,
        ..Node::root()
    }];
    let mut push = |nodes: &mut Vec<Node>, parent: usize, at: Vec3, radius: f64, branch: u32| {
        let i = nodes.len();
        nodes.push(Node {
            position: at,
            parent: Some(parent as u32),
            radius,
            start_radius: radius,
            base_radius: radius,
            branch,
            ..Node::root()
        });
        i
    };
    let mut at = 0;
    for k in 0..20 {
        at = push(&mut nodes, at, Vec3::Y * (STEP * (k + 1) as f64), 0.5, 1);
    }
    let lateral = if wide { 0.3 } else { 0.02 };
    let mut side = 10;
    for k in 0..10 {
        let to = nodes[10].position + Vec3::X * (STEP * (k + 1) as f64);
        side = push(&mut nodes, side, to, lateral, 100);
    }
    let branch = if taken { 200 } else { 1 };
    for k in 20..60 {
        at = push(
            &mut nodes,
            at,
            Vec3::Y * (STEP * (k + 1) as f64),
            0.1,
            branch,
        );
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

/// Along the run the drawn girth never falls faster than 1 in 1, within
/// one axis and across a takeover alike: the drop of 0.4 m is a shoulder
/// 0.4 m long, and the wood below it and well above it is drawn at its own
/// radius.
#[test]
fn a_run_never_thins_faster_than_one_in_one() {
    for taken in [false, true] {
        let tree = stepped(false, taken);
        let runs = rings(&tree);
        let (_, stem) = runs.iter().find(|(first, _)| *first == 0).unwrap();
        for pair in stem.windows(2) {
            let fall = pair[0].r - pair[1].r;
            assert!(
                fall <= pair[1].d - pair[0].d + 1e-12,
                "{taken}: {fall} at {}",
                pair[1].d
            );
        }
        let at = |y: f64| stem.iter().find(|s| (s.p.y - y).abs() < 1e-9).unwrap().r;
        assert_eq!(at(1.0), 0.5, "{taken}: below the drop");
        assert!(
            (at(1.2) - 0.3).abs() < 1e-9,
            "{taken}: the shoulder: {}",
            at(1.2)
        );
        assert_eq!(at(1.6), 0.1, "{taken}: above the shoulder");
    }
}

/// A lateral that starts its own run, thinner or wider than the stem's
/// shoulder, is drawn at its own wood from its base.
#[test]
fn a_lateral_starting_its_own_run_keeps_its_base() {
    for wide in [false, true] {
        let tree = stepped(wide, false);
        let runs = rings(&tree);
        let (_, lateral) = runs.iter().find(|(first, _)| *first == 10).unwrap();
        let own = tree.nodes[21].radius;
        let radii: Vec<f64> = lateral[1..].iter().map(|s| s.r).collect();
        assert!(radii.iter().all(|&r| r == own), "{wide}: {radii:?}");
    }
}
