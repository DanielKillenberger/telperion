//! f32 vectors and angles for the layout (fn-254, after CONTRACT-A.md 2.3
//! and 2.4; host decision on 2.14: one layout for the rough and the final
//! lay). A vector is three f32 components; the tree's f64 fields hold their
//! exact images, so `q3` of a laid value and `v3` of a computed one are
//! exact. An angle is an amount in turns: whole turns kept until an
//! orientation is read, which `wrap` brings within half a turn.
use crate::structure::Vec3;
use crate::transcendental::Transcendental;

pub(crate) type Q3 = [f32; 3];

pub(crate) const UP: Q3 = [0.0, 0.0, 1.0];

/// A turn in radians.
const TAU: f32 = std::f32::consts::TAU;

pub(crate) fn q3(v: Vec3) -> Q3 {
    [v.x as f32, v.y as f32, v.z as f32]
}

pub(crate) fn v3(q: Q3) -> Vec3 {
    Vec3::new(f64::from(q[0]), f64::from(q[1]), f64::from(q[2]))
}

pub(crate) fn add(a: Q3, b: Q3) -> Q3 {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
}

pub(crate) fn sub(a: Q3, b: Q3) -> Q3 {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

/// `v s`.
pub(crate) fn scaled(v: Q3, s: f32) -> Q3 {
    v.map(|c| c * s)
}

pub(crate) fn dot(a: Q3, b: Q3) -> f32 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

pub(crate) fn cross(a: Q3, b: Q3) -> Q3 {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

pub(crate) fn length(v: Q3) -> f32 {
    dot(v, v).sqrt()
}

/// The unit vector along `v`: none for the zero vector exactly.
pub(crate) fn unit(v: Q3) -> Option<Q3> {
    let l = length(v);
    (l > 0.0).then(|| scaled(v, 1.0 / l))
}

/// A rotation: its unit pivot and its cosine and sine.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct Turn {
    pub pivot: Q3,
    pub cos: f32,
    pub sin: f32,
}

/// `v` turned by `t` (Rodrigues): `v cos + (p × v) sin + p (p · v)(1 -
/// cos)`.
pub(crate) fn turned_by(v: Q3, t: &Turn) -> Q3 {
    let across = cross(t.pivot, v);
    let fall = dot(t.pivot, v) * (1.0 - t.cos);
    std::array::from_fn(|k| v[k] * t.cos + across[k] * t.sin + t.pivot[k] * fall)
}

/// The least rotation taking unit `from` to unit `to`: about `unit(from
/// × to)`, its cosine and sine `(from · to, |from × to|)` (host decision,
/// 2026-10-07: trig only where an angle is a stated quantity). None where
/// the two are parallel or opposed exactly.
pub(crate) fn carry(from: Q3, to: Q3) -> Option<Turn> {
    let axis = cross(from, to);
    let pivot = unit(axis)?;
    let (c, s) = (dot(from, to), length(axis));
    let norm = (c * c + s * s).sqrt();
    Some(Turn {
        pivot,
        cos: c / norm,
        sin: s / norm,
    })
}

/// An amount within half a turn of none: the orientation it reads.
pub(crate) fn wrap(turns: f32) -> f32 {
    turns - (turns + 0.5).floor()
}

/// The amount from orientation `from` to `to`, the short way.
pub(crate) fn between(from: f32, to: f32) -> f32 {
    wrap(to - from)
}

/// The cosine and sine of an amount in turns.
pub(crate) fn sincos(turns: f32) -> (f32, f32) {
    let (sin, cos) = (wrap(turns) * TAU).sin_cos_fixed();
    (cos, sin)
}

/// The orientation of `(x, y)`, in turns within half a turn; none for
/// none.
pub(crate) fn atan2(y: f32, x: f32) -> f32 {
    y.atan2_fixed(x) / TAU
}

/// `asin x`, clamped, in turns.
pub(crate) fn asin(x: f32) -> f32 {
    x.clamp(-1.0, 1.0).asin_fixed() / TAU
}

/// `acos c`, clamped, in turns.
pub(crate) fn acos(c: f32) -> f32 {
    c.clamp(-1.0, 1.0).acos_fixed() / TAU
}

/// An amount in radians.
pub(crate) fn radians(turns: f32) -> f32 {
    turns * TAU
}

/// Radians as an amount.
pub(crate) fn turns(radians: f32) -> f32 {
    radians / TAU
}

/// The rotation by the stated amount `angle` (turns) about the unit
/// `pivot`: one sin and cos, reused for every vector it turns.
pub(crate) fn turn(pivot: Q3, angle: f32) -> Turn {
    let (cos, sin) = sincos(angle);
    Turn { pivot, cos, sin }
}

/// `v` rotated by the amount `angle` about the unit `pivot`.
pub(crate) fn rotated(v: Q3, pivot: Q3, angle: f32) -> Q3 {
    turned_by(v, &turn(pivot, angle))
}

/// A species' angle in radians as an amount, in turns: converted once.
pub(crate) fn amount(radians: f64) -> f32 {
    (radians / std::f64::consts::TAU) as f32
}
