//! The plan's error bounds where the outline is not a plain circle or the
//! stretch not a plain arc (fn-208 review: lobes, twist, parallel tangents).
use super::*;
use crate::pipeline::surface::curve::{CurveRun, CLUSTER};

/// The most a polygon of `n` sides through the lobed outline
/// `ρ(1 + d·cos(kθ))` stands off it, in the outline's own units: each
/// outline point's distance from the chord of its side.
fn polygon_error(rho: f64, lobes: u32, depth: f64, n: u32) -> f64 {
    let at = |t: f64| {
        let r = rho * (1.0 + depth * (f64::from(lobes) * t).cos());
        Vec3::new(r * t.cos(), r * t.sin(), 0.0)
    };
    let step = std::f64::consts::TAU / f64::from(n);
    let mut worst = 0.0f64;
    for side in 0..n {
        let (t0, t1) = (step * f64::from(side), step * f64::from(side + 1));
        let (a, b) = (at(t0), at(t1));
        let along = (b - a).normalized();
        for k in 1..64 {
            let p = at(t0 + (t1 - t0) * f64::from(k) / 64.0) - a;
            worst = worst.max((p - along * p.dot(along)).length());
        }
    }
    worst
}

#[test]
fn a_lobed_ring_takes_the_sides_its_outline_needs() {
    for (rho, lobes, depth) in [(2.0, 12, 0.4), (40.0, 7, 0.11), (300.0, 4, 0.24)] {
        let curve = Curve {
            lobes,
            lobe_depth: depth,
            height: 1.0,
            ..Curve::default()
        };
        let n = sides(rho * (1.0 + depth), 0.25, curve.bend());
        let off = polygon_error(rho, lobes, depth, n);
        assert!(
            off <= 0.25,
            "{lobes} lobes at {rho} px: {n} sides stand {off} px off"
        );
    }
}

fn straight(count: usize, length: f64) -> Vec<CurvePoint> {
    (0..count)
        .map(|i| {
            let along = length * i as f64 / (count - 1) as f64;
            CurvePoint {
                centre: Vec3::new(0.0, along, 0.0),
                radius: 0.1,
                along,
                normal: Vec3::new(1.0, 0.0, 0.0),
                binormal: Vec3::new(0.0, 0.0, -1.0),
            }
        })
        .collect()
}

#[test]
fn twisting_lobes_keep_the_rings_that_hold_their_turn() {
    let run = CurveRun {
        first: 0,
        count: CLUSTER as u32,
        trunk: true,
        section: None,
        largest_radius: 0.1,
    };
    let mut curve = Curve::of_runs(straight(CLUSTER, 1.0), vec![run]);
    let round = curve.clusters[0].errors;
    assert!(
        round.iter().all(|&e| e < 1e-9),
        "a straight round run has no error: {round:?}"
    );
    (curve.lobes, curve.lobe_depth, curve.twist_rate) = (4, 0.24, 1.0);
    let mut made = Vec::new();
    super::super::super::clusters(&curve, 0, &run, &mut made);
    let twisted = made[0].errors;
    // Skipping half the turn's rings at the top level leaves the lobes'
    // whole depth in doubt: 0.1 m × 0.24 × 2.
    assert!(
        twisted[LEVELS - 1] > 0.04,
        "the twist was not measured: {twisted:?}"
    );
    assert!(twisted[1] > 0.0 && twisted[1] < twisted[LEVELS - 1]);
}

#[test]
fn parallel_tangents_on_a_bent_stretch_are_still_cut() {
    // The middle stretch of (-1,-1), (0,0), (1,0), (2,1): both ends' tangents
    // point along (cos π/8, sin π/8), but the Hermite curve between them
    // bends off the chord by 36 mm.
    let tangent = Vec3::new(
        (std::f64::consts::PI / 8.0).cos(),
        (std::f64::consts::PI / 8.0).sin(),
        0.0,
    );
    let normal = Vec3::new(0.0, 0.0, 1.0).cross(tangent);
    let point = |x: f64, y: f64| CurvePoint {
        centre: Vec3::new(x, y, 0.0),
        radius: 0.01,
        along: x,
        normal,
        binormal: tangent.cross(normal),
    };
    let (a, b) = (point(0.0, 0.0), point(1.0, 0.0));
    assert!(super::tangent(&a).distance(tangent) < 1e-12);
    let viewer = Viewer {
        eye: Vec3::new(0.5, 0.0, 10.0),
        forward: Vec3::new(0.0, 0.0, -1.0),
        pixels_per_metre: 1000.0,
        near: 0.01,
        orthographic: true,
        planes: None,
    };
    let m = pieces(&a, &b, &viewer, 0.25, 0.0);
    let worst = (0..=200)
        .map(|k| {
            let t = f64::from(k) / 200.0;
            let p = hermite(&a, &b, t).centre;
            // The piece's chord this point falls on.
            let j = ((t * m as f64).floor() as usize).min(m - 1);
            let (c0, c1) = (
                hermite(&a, &b, j as f64 / m as f64).centre,
                hermite(&a, &b, (j + 1) as f64 / m as f64).centre,
            );
            let along = (c1 - c0).normalized();
            let q = p - c0;
            (q - along * q.dot(along)).length()
        })
        .fold(0.0, f64::max);
    assert!(
        worst * 1000.0 <= 0.25,
        "{m} pieces stand {} px off",
        worst * 1000.0
    );
}
