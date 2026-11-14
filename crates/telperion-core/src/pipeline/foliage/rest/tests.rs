use super::*;
use crate::pipeline::foliage::{build_element, transform_point};

/// What "on the ground" admits after a float32 store (`ground_reach`).
const BELOW: f64 = 1e-4;

fn element() -> Element {
    build_element(Default::default()).unwrap()
}

/// A leaf at `at` along `axis`, its face toward `face`, at `scale`.
fn placed(at: Vec3, axis: Vec3, face: Vec3, scale: f64) -> [f32; 16] {
    let (axis, face) = (axis.normalized(), face.normalized());
    let side = axis.cross(face);
    let mut m = [0f32; 16];
    for (c, v) in [side, axis, face].iter().enumerate() {
        m[c * 4] = (v.x * scale) as f32;
        m[c * 4 + 1] = (v.y * scale) as f32;
        m[c * 4 + 2] = (v.z * scale) as f32;
    }
    (m[12], m[13], m[14], m[15]) = (at.x as f32, at.y as f32, at.z as f32, 1.0);
    m
}

fn column(m: &[f32; 16], c: usize) -> Vec3 {
    Vec3::new(m[c * 4].into(), m[c * 4 + 1].into(), m[c * 4 + 2].into())
}

/// The lowest of the wedge's corners under `m`.
fn lowest(m: &[f32; 16], w: &Wedge) -> f64 {
    w.corners
        .iter()
        .map(|c| transform_point(m, *c).y)
        .fold(f64::INFINITY, f64::min)
}

#[test]
fn a_wedge_bounds_every_vertex_of_its_element() {
    let e = element();
    let w = Wedge::of(&e);
    let base = w.corners[0].y;
    let rise = |y: f64| (w.corners[2].z - w.corners[0].z) / (w.corners[2].y - base) * (y - base);
    for v in &e.positions {
        assert!(v.x >= w.corners[0].x && v.x <= w.corners[1].x, "{v:?}");
        assert!(v.y >= base && v.y <= w.corners[2].y, "{v:?}");
        assert!(v.z <= w.corners[4].z, "{v:?}");
        assert!(v.z >= w.corners[0].z + rise(v.y) - 1e-12, "{v:?}");
    }
    assert!(w.depth >= 0.0 && w.depth == -w.corners[0].z);
}

#[test]
fn a_leaf_clear_of_the_ground_keeps_every_bit() {
    let w = Wedge::of(&element());
    let m = placed(Vec3::new(1.0, 0.5, 0.0), -Vec3::Y, Vec3::X, 1.0);
    assert!(lowest(&m, &w) > 0.0);
    assert_eq!(leaf(m, &w, 1e-4).map(f32::to_bits), m.map(f32::to_bits));
}

#[test]
fn a_hanging_leaf_rests_by_no_more_than_lying_flat() {
    let w = Wedge::of(&element());
    let step = 2e-4;
    for (axis, face) in [
        (-Vec3::Y, Vec3::X),
        (Vec3::new(0.3, -1.0, 0.2), Vec3::new(1.0, 0.2, -0.1)),
        (Vec3::new(1.0, -0.4, 0.0), Vec3::new(0.0, -0.3, 1.0)),
    ] {
        let face = (face - axis.normalized() * face.dot(axis.normalized())).normalized();
        let m = placed(Vec3::new(2.0, 0.05, -1.0), axis, face, 1.0);
        assert!(lowest(&m, &w) < 0.0);
        let r = leaf(m, &w, step);
        // Translation and scale are kept.
        assert_eq!(&r[12..], &m[12..]);
        for c in 0..3 {
            assert!((column(&r, c).length() - column(&m, c).length()).abs() < 1e-6);
        }
        // The lowest corner lies on the ground, cleared by the encoder's
        // margin.
        let reach = w.corners.iter().map(|c| c.length()).fold(0.0, f64::max);
        let low = lowest(&r, &w);
        assert!((low - margin(step, reach)).abs() < BELOW, "{low}");
        // No larger a turn than laying the leaf face up.
        let turn = |a: Vec3, b: Vec3| a.normalized().dot(b.normalized()).clamp(-1.0, 1.0).acos();
        let angle = (0..3)
            .map(|c| turn(column(&m, c), column(&r, c)))
            .fold(0.0, f64::max);
        assert!(angle <= turn(face, Vec3::Y) + 1e-6, "{angle}");
    }
}

#[test]
fn a_leaf_on_the_finest_wood_lets_its_stalk_down_no_further_than_its_depth() {
    let w = Wedge::of(&element());
    let h = 2e-4;
    let m = placed(Vec3::new(0.0, h, 0.0), -Vec3::Y, Vec3::Z, 1.0);
    let step = 1e-4;
    let r = leaf(m, &w, step);
    assert!(lowest(&r, &w) >= h - w.depth - 1e-6);
    // Only the stalk dips: the tip's corners clear the ground by the margin.
    let reach = w.corners.iter().map(|c| c.length()).fold(0.0, f64::max);
    for c in &w.corners[2..4] {
        assert!(transform_point(&r, *c).y >= margin(step, reach) - 1e-6);
    }
    // Lying flat qualifies, so the leaf is not stood up.
    assert!(column(&r, 1).normalized().y < 0.5);
}

#[test]
fn where_no_direction_clears_the_lowest_corner_is_lifted_highest() {
    // A box round its own pivot, an inch off its nearest face in -y.
    let (lo, hi) = (Vec3::new(-1.0, -0.025, -1.0), Vec3::new(1.0, 2.0, 1.0));
    let v = std::array::from_fn(|k| {
        Vec3::new(
            if k & 1 == 0 { lo.x } else { hi.x },
            if k & 2 == 0 { lo.y } else { hi.y },
            if k & 4 == 0 { lo.z } else { hi.z },
        )
    });
    assert_eq!(upright(&v, &[0.0; 8]), None);
    assert_eq!(best(&v), Vec3::Y);
}
