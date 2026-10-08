//! Q32 vectors for the layout on contract A (CONTRACT-A.md 2.3 and 2.4;
//! host decision on 2.14: one fixed-point layout for the rough and the
//! final lay). A vector is three Q32 components; the tree's f64 fields
//! hold their exact images (host, task 9), so `q3` of a laid value and
//! `v3` of a computed one are exact. Every product rounds once (floor),
//! every sum is exact.
use crate::fixed::feedback::angle::{sincos, wrap};
use crate::fixed::feedback::vector::{self, turned, Turn};
use crate::fixed::{mul, q32, ONE32};
use crate::structure::Vec3;

pub(crate) type Q3 = [i64; 3];
pub(crate) use vector::{carry, turned as turned_by};

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

/// `a · b`, one floor.
pub(crate) fn dot(a: Q3, b: Q3) -> i64 {
    vector::dot(a, b)
}

/// `a × b`, each component one floor.
pub(crate) fn cross(a: Q3, b: Q3) -> Q3 {
    vector::cross(a, b)
}

pub(crate) fn length(v: Q3) -> i64 {
    vector::length(v)
}

pub(crate) fn unit(v: Q3) -> Option<Q3> {
    vector::unit(v)
}

/// The rotation by the stated amount `angle` (Q32 turns) about the unit
/// `pivot`: one sin and cos, reused for every vector it turns.
pub(crate) fn turn(pivot: Q3, angle: i64) -> Turn {
    let (cos, sin) = sincos(wrap(angle));
    Turn { pivot, cos, sin }
}

/// `v` rotated by the amount `angle` about the unit `pivot`.
pub(crate) fn rotated(v: Q3, pivot: Q3, angle: i64) -> Q3 {
    turned(v, &turn(pivot, angle))
}

/// A species' angle in radians as an amount, Q32 turns: converted once,
/// the nearest.
pub(crate) fn amount(radians: f64) -> i64 {
    q32(radians / std::f64::consts::TAU)
}
