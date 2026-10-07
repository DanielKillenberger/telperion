//! Q32 vectors for the layout on contract A (CONTRACT-A.md 2.3 and 2.4;
//! host decision on 2.14: one fixed-point layout for the rough and the
//! final lay). A vector is three Q32 components; the tree's f64 fields
//! hold their exact images (host, task 9), so `q3` of a laid value and
//! `v3` of a computed one are exact. Every product rounds once (floor),
//! every sum is exact.
use crate::fixed::feedback::angle::{sincos, wrap};
use crate::fixed::feedback::vector;
use crate::fixed::feedback::wide::I256;
use crate::fixed::{mul, q32, ONE32};
use crate::structure::Vec3;

pub(crate) type Q3 = [i64; 3];

pub(crate) const UP: Q3 = [0, 0, ONE32];

pub(crate) fn q3(v: Vec3) -> Q3 {
    [q32(v.x), q32(v.y), q32(v.z)]
}

pub(crate) fn v3(q: Q3) -> Vec3 {
    let r = |x: i64| x as f64 / ONE32 as f64;
    Vec3::new(r(q[0]), r(q[1]), r(q[2]))
}

pub(crate) fn add(a: Q3, b: Q3) -> Q3 {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
}

pub(crate) fn sub(a: Q3, b: Q3) -> Q3 {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

/// `v s`, each component floored.
pub(crate) fn scaled(v: Q3, s: i64) -> Q3 {
    v.map(|c| mul(c, s))
}

/// The exact dot product, Q64.
fn dot_wide(a: Q3, b: Q3) -> i128 {
    (0..3).map(|k| i128::from(a[k]) * i128::from(b[k])).sum()
}

/// `a · b`, one floor.
pub(crate) fn dot(a: Q3, b: Q3) -> i64 {
    (dot_wide(a, b) >> 32) as i64
}

/// The exact cross product, Q64.
fn cross_wide(a: Q3, b: Q3) -> [i128; 3] {
    let p = |i: usize, j: usize| i128::from(a[i]) * i128::from(b[j]);
    [p(1, 2) - p(2, 1), p(2, 0) - p(0, 2), p(0, 1) - p(1, 0)]
}

/// `a × b`, each component one floor.
pub(crate) fn cross(a: Q3, b: Q3) -> Q3 {
    cross_wide(a, b).map(|c| (c >> 32) as i64)
}

pub(crate) fn length(v: Q3) -> i64 {
    vector::length(v)
}

pub(crate) fn unit(v: Q3) -> Option<Q3> {
    vector::unit(v)
}

/// `v` rotated by the amount `angle` (Q32 turns) about the unit `pivot`
/// (Rodrigues): `v cos + (p × v) sin + p (p · v)(1 - cos)`, each of the
/// three one floor a component from exact intermediates, added exactly.
pub(crate) fn rotated(v: Q3, pivot: Q3, angle: i64) -> Q3 {
    let (cos, sin) = sincos(wrap(angle));
    let across = cross_wide(pivot, v);
    let along = dot_wide(pivot, v);
    let narrow = |x: I256| x.narrow64().expect("a rotated vector inside Q32");
    std::array::from_fn(|k| {
        let a = mul(v[k], cos);
        let b = narrow(I256::mul(across[k], i128::from(sin)).floor_shift(64));
        let fall = i128::from(pivot[k]) * i128::from(ONE32 - cos);
        let c = narrow(I256::mul(along, fall).floor_shift(128));
        a + b + c
    })
}

/// A species' angle in radians as an amount, Q32 turns: converted once,
/// the nearest.
pub(crate) fn amount(radians: f64) -> i64 {
    q32(radians / std::f64::consts::TAU)
}
