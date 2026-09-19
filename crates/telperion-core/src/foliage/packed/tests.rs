use super::*;

/// A column-major transform with these columns and this translation.
fn transform(columns: [Vec3; 3], scale: f64, at: Vec3) -> [f32; 16] {
    let mut m = [0.0f32; 16];
    for (c, v) in columns.iter().enumerate() {
        m[c * 4] = (v.x * scale) as f32;
        m[c * 4 + 1] = (v.y * scale) as f32;
        m[c * 4 + 2] = (v.z * scale) as f32;
    }
    m[12] = at.x as f32;
    m[13] = at.y as f32;
    m[14] = at.z as f32;
    m[15] = 1.0;
    m
}

/// A right-handed orthonormal basis about `axis`, the way `station::matrix`
/// builds one: face perpendicular to the axis, side their cross product.
fn basis(axis: Vec3, hint: Vec3) -> [Vec3; 3] {
    let axis = axis.normalized();
    let mut face = hint - axis * hint.dot(axis);
    if face.length_squared() <= 1e-12 {
        face = axis.perpendicular();
    }
    let face = face.normalized();
    let side = axis.cross(face).normalized();
    [side, axis, face]
}

#[test]
fn a_leaf_is_twelve_bytes() {
    assert_eq!(std::mem::size_of::<Leaf>(), 12);
    assert_eq!(WORDS, 3);
    assert_eq!(std::mem::size_of::<[Leaf; 7]>(), 7 * 12);
}

#[test]
fn a_half_float_round_trips_every_scale_a_leaf_can_carry() {
    for millis in 1..=20_000u32 {
        let v = f64::from(millis) / 1000.0;
        let back = f64::from(half_value(half_bits(v)));
        assert!(
            (back - v).abs() <= v * 1e-3,
            "half float {v} came back {back}"
        );
    }
    assert_eq!(half_value(half_bits(0.0)), 0.0);
    assert_eq!(half_value(half_bits(1.0)), 1.0);
    // Past the format's reach it saturates rather than reaching infinity, so
    // no stored scale can ever decode to a transform nothing can draw.
    assert!(half_value(half_bits(1e30)).is_finite());
}

#[test]
fn a_rotation_round_trips_inside_the_stated_bound() {
    let mut worst: f64 = 0.0;
    for i in 0..64 {
        for j in 0..64 {
            let a = f64::from(i) / 64.0 * std::f64::consts::TAU;
            let b = f64::from(j) / 64.0 * std::f64::consts::PI - std::f64::consts::FRAC_PI_2;
            let axis = Vec3::new(b.cos() * a.cos(), b.sin(), b.cos() * a.sin());
            let columns = basis(axis, Vec3::Y + Vec3::X * 0.31);
            let word = rotation_word(&columns);
            let back = rotation_columns(word);
            for (before, after) in columns.iter().zip(&back) {
                let angle = before.dot(*after).clamp(-1.0, 1.0).acos();
                worst = worst.max(angle);
            }
        }
    }
    // The bound is the measured worst case, not the typical one. Ten bits a
    // component over a sweep of 4,096 orientations is 0.00250 rad at worst
    // with the encoder choosing the nearest of its cell's eight corners, and
    // 0.00316 rad with each component rounded on its own. Thirty bits cannot
    // reach the 0.002 rad the spec first asked for, because what the dropped
    // component reconstructs to is amplified by one over itself and that is
    // as large as two near half a turn; the owner moved R2 to 0.0026 on
    // 2026-09-19 rather than widen the rotation, since the twelve bytes are
    // what this encoding exists for. At 0.149 degrees it carries a 70 mm
    // leaf's tip 0.18 mm, inside the position budget already allowed. The
    // number held here is the measured one, so a later change that makes the
    // encoding worse still fails.
    assert!(worst <= 2.6e-3, "worst rotation error {worst} rad");
}

#[test]
fn a_position_round_trips_inside_half_a_step() {
    let reference = Reference::spanning(Vec3::new(-14.0, -1.0, -14.0), Vec3::new(14.0, 33.0, 14.0));
    let step = reference.step();
    let columns = basis(Vec3::Y, Vec3::X);
    for k in 0..10_000u32 {
        let t = f64::from(k) / 10_000.0;
        let at = Vec3::new(
            -14.0 + 28.0 * t,
            -1.0 + 34.0 * (1.0 - t) * 0.999,
            -14.0 + 28.0 * (t * 7.0).fract(),
        );
        let m = transform(columns, 0.07, at);
        // The stored translation was already an f32 before it was quantised,
        // so the round trip is measured against that, not against the f64 the
        // test wrote: half a code step is the whole of what quantising costs.
        let at = Vec3::new(f64::from(m[12]), f64::from(m[13]), f64::from(m[14]));
        let back = reference.position(reference.pack(&m));
        assert!((back.x - at.x).abs() <= step.x / 2.0 + 1e-12, "x at {at:?}");
        assert!((back.y - at.y).abs() <= step.y / 2.0 + 1e-12, "y at {at:?}");
        assert!((back.z - at.z).abs() <= step.z / 2.0 + 1e-12, "z at {at:?}");
    }
}

#[test]
fn a_box_of_no_extent_decodes_every_leaf_to_its_corner() {
    let reference = Reference::default();
    let leaf = reference.pack(&transform(basis(Vec3::Y, Vec3::X), 1.0, Vec3::ZERO));
    assert_eq!(reference.position(leaf), Vec3::ZERO);
    assert!(reference.contains(Vec3::ZERO));
}

#[test]
fn a_reversed_span_is_read_in_the_order_that_makes_a_box() {
    let a = Reference::spanning(Vec3::new(3.0, 4.0, 5.0), Vec3::new(-1.0, -2.0, -3.0));
    assert_eq!(a.min, Vec3::new(-1.0, -2.0, -3.0));
    assert_eq!(a.extent, Vec3::new(4.0, 6.0, 8.0));
    assert!(a.is_finite());
}

#[test]
fn a_box_with_a_nan_side_is_not_a_box() {
    let mut a = Reference::spanning(Vec3::ZERO, Vec3::new(1.0, 1.0, 1.0));
    a.extent.y = f64::NAN;
    assert!(!a.is_finite());
}

#[test]
fn the_scale_is_read_back_off_the_word_that_carries_it() {
    let reference = Reference::spanning(Vec3::ZERO, Vec3::new(2.0, 2.0, 2.0));
    for scale in [0.004, 0.07, 1.0, 12.5] {
        let leaf = reference.pack(&transform(basis(Vec3::Y, Vec3::X), scale, Vec3::ZERO));
        let back = reference.scale(leaf);
        assert!(
            (back - scale).abs() <= scale * 1e-3,
            "scale {scale} came back {back}"
        );
        // The rebuilt columns carry that same scale, so the transform the
        // renderer multiplies by is the one the generator wrote.
        let m = reference.unpack(leaf);
        let column = Vec3::new(f64::from(m[4]), f64::from(m[5]), f64::from(m[6]));
        assert!((column.length() - scale).abs() <= scale * 2e-3);
    }
}

#[test]
fn the_columns_come_back_in_the_order_they_went_in() {
    let reference = Reference::spanning(Vec3::ZERO, Vec3::new(4.0, 4.0, 4.0));
    let columns = basis(Vec3::new(0.3, 0.9, -0.2), Vec3::Y);
    let at = Vec3::new(1.0, 2.0, 3.0);
    let m = reference.unpack(reference.pack(&transform(columns, 0.05, at)));
    for (c, expected) in columns.iter().enumerate() {
        let got = Vec3::new(
            f64::from(m[c * 4]),
            f64::from(m[c * 4 + 1]),
            f64::from(m[c * 4 + 2]),
        )
        .normalized();
        assert!(
            got.dot(*expected) > 0.999,
            "column {c} came back {got:?} for {expected:?}"
        );
    }
    assert_eq!(m[3], 0.0);
    assert_eq!(m[7], 0.0);
    assert_eq!(m[11], 0.0);
    assert_eq!(m[15], 1.0);
}
