//! fn-362: a run is cut with as many sides as its widest drawn ring needs
//! for its polygon to stand within `FINEST` of the circle, `r (1 − cos(π /
//! n)) ≤ FINEST`, and never more than the surface's count (host). Every
//! ring of a run takes that one count.
use super::{build, curve, SurfaceParams};
use crate::{
    math::Vec3,
    pipeline::space::FINEST,
    tree::{Node, Tree},
};
use std::f64::consts::{PI, TAU};

/// Node spacing along a stem.
const STEP: f64 = 0.05;

/// Round wood of twenty sides with no flare, burial or swell, so a ring's
/// drawn radius is its node's radius.
fn plain() -> SurfaceParams {
    SurfaceParams {
        radial_segments: 20,
        lobes: 0,
        lobe_depth: 0.0,
        flare_radius: 1.0,
        flare_depth: 0.0,
        fork_swell: 1.0,
        ..SurfaceParams::default()
    }
}

/// An upright stem of `count` nodes, each node's radius read off its
/// place, and a codominant sibling of ten nodes of each radius in `sides`
/// leaning out of every second node.
fn stem(count: usize, radius: impl Fn(usize) -> f64, sides: &[f64]) -> Tree {
    let mut nodes = vec![Node {
        radius: radius(0),
        start_radius: radius(0),
        ..Node::root()
    }];
    let push = |nodes: &mut Vec<Node>, parent: usize, at: Vec3, r: f64| {
        let i = nodes.len();
        nodes.push(Node {
            position: at,
            parent: Some(parent as u32),
            radius: r,
            start_radius: r,
            base_radius: r,
            branch: i as u32,
            stem: true,
            ..Node::root()
        });
        i
    };
    for k in 1..count {
        push(&mut nodes, k - 1, Vec3::Y * (STEP * k as f64), radius(k));
    }
    for (j, &r) in sides.iter().enumerate() {
        let at = 2 * (j + 1);
        let turn = j as f64;
        let lean = Vec3::new(turn.cos() * 0.5, 0.8, turn.sin() * 0.5).normalized();
        let mut side = at;
        for k in 0..10 {
            let to = nodes[at].position + lean * (STEP * (k + 1) as f64);
            side = push(&mut nodes, side, to, r);
            if k == 0 {
                nodes[side].codominant = Some(1.0);
            }
        }
    }
    Tree {
        crossover: nodes.len(),
        nodes,
        ..Tree::default()
    }
}

/// Each run's ring radii and the sides of each of its rings, read off the
/// mesh: a ring vertex's bark angle is its place round a ring of `n`, so
/// the second vertex of a run gives `n`, and every ring of the run must lay
/// its vertices at `k / n` of a turn.
fn sides_by_run(tree: &Tree, params: &SurfaceParams) -> Vec<(Vec<f64>, Vec<usize>)> {
    let mesh = build(tree, 18.0, params).unwrap();
    let curve = curve(tree, 18.0, params).unwrap();
    let flat = mesh.flattened();
    let angle = |v: usize| f64::from(flat.coords[v * 2 + 1]);
    let mut at = 0;
    let mut out = Vec::new();
    for run in &curve.runs {
        let points = curve.run_points(run);
        let mut sides = Vec::new();
        for _ in points {
            let n = (TAU / angle(at + 1)).round() as usize;
            for k in 0..n {
                let off = (angle(at + k) - k as f64 / n as f64 * TAU).abs();
                assert!(
                    off < 1e-3,
                    "vertex {k} of a ring of {n} stands at {}",
                    angle(at + k)
                );
            }
            sides.push(n);
            at += n;
        }
        out.push((points.iter().map(|p| p.radius).collect(), sides));
        at += 2;
    }
    assert_eq!(at, flat.positions.len() / 3, "every vertex is a run's");
    out
}

/// R2: a run of 1.5 mm is cut with 7 sides and a run of 1 m with the
/// surface's 20.
#[test]
fn a_twig_takes_the_sides_its_radius_needs_and_a_trunk_the_cap() {
    for (radius, want) in [(1.5e-3, 7), (1.0, 20)] {
        let tree = stem(12, |_| radius, &[]);
        let runs = sides_by_run(&tree, &plain());
        assert_eq!(runs.len(), 1);
        assert!(
            runs[0].1.iter().all(|&n| n == want),
            "a run of {radius} m is cut with {:?} sides, not {want}",
            runs[0].1
        );
    }
}

/// R2: a run thinning from 3 cm to 0.3 mm keeps the sides of its widest
/// ring, and every ring of every run of a mixed tree stands within
/// `FINEST` of its circle or has the cap's sides.
#[test]
fn every_ring_of_a_run_has_its_widest_rings_sides_within_finest() {
    let thinning = |k: usize| 0.03 * (0.01_f64).powf(k as f64 / 29.0);
    let tree = stem(
        30,
        thinning,
        &[2.5e-4, 5e-4, 1e-3, 1.5e-3, 2e-3, 5e-3, 1e-2, 2e-2],
    );
    let params = plain();
    let runs = sides_by_run(&tree, &params);
    assert_eq!(runs.len(), 9);
    let mut counts = Vec::new();
    for (radii, sides) in &runs {
        let n = sides[0];
        assert!(sides.iter().all(|&s| s == n), "one count a run: {sides:?}");
        for &r in radii {
            let flat = r * (1.0 - (PI / n as f64).cos());
            assert!(
                n == 20 || flat <= FINEST,
                "a ring of {r} m on {n} sides is {flat} m flat"
            );
        }
        counts.push(n);
    }
    counts.sort_unstable();
    assert_eq!(counts, [3, 4, 5, 7, 7, 12, 16, 20, 20]);
}
