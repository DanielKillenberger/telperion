//! The wood's per-vertex data in four bytes a field, and the octahedral
//! folding the curve's packed frame shares. A normal is folded onto the
//! octahedron and kept as two snorm16; bark coords are two unorm16, the
//! distance along over the wood's longest and the angle over a turn.
use crate::math::Vec3;
use std::f64::consts::TAU;

/// A unit vector folded onto the octahedron, each component quantised to a
/// signed integer of `bits` bits.
pub(crate) fn octahedral(v: Vec3, bits: u32) -> [i32; 2] {
    let l1 = v.x.abs() + v.y.abs() + v.z.abs();
    let (mut x, mut y) = (v.x / l1, v.y / l1);
    if v.z < 0.0 {
        let (ox, oy) = (x, y);
        x = (1.0 - oy.abs()) * ox.signum();
        y = (1.0 - ox.abs()) * oy.signum();
    }
    let top = f64::from((1u32 << (bits - 1)) - 1);
    let q = |c: f64| (c.clamp(-1.0, 1.0) * top).round() as i32;
    [q(x), q(y)]
}

/// The unit vector two `bits`-bit octahedral components stand for.
pub(crate) fn unoctahedral(c: [i32; 2], bits: u32) -> Vec3 {
    let top = f64::from((1u32 << (bits - 1)) - 1);
    let (x, y) = (f64::from(c[0]) / top, f64::from(c[1]) / top);
    let z = 1.0 - x.abs() - y.abs();
    let (x, y) = if z < 0.0 {
        ((1.0 - y.abs()) * x.signum(), (1.0 - x.abs()) * y.signum())
    } else {
        (x, y)
    };
    Vec3::new(x, y, z).normalized()
}

/// A wood normal as the chunk keeps it.
pub fn normal(n: Vec3) -> [i16; 2] {
    octahedral(n, 16).map(|c| c as i16)
}

/// The unit normal a chunk's two components stand for.
pub fn unnormal(c: [i16; 2]) -> Vec3 {
    unoctahedral(c.map(i32::from), 16)
}

const TOP: f64 = u16::MAX as f64;

/// Bark coords as the chunk keeps them: metres along the wood over
/// `along`, the wood's longest, and the angle round it over a turn.
pub fn coord(d: f64, angle: f64, along: f64) -> [u16; 2] {
    let q = |t: f64| (t.clamp(0.0, 1.0) * TOP).round() as u16;
    let d = if along > 0.0 { d / along } else { 0.0 };
    [q(d), q(angle / TAU)]
}

/// The metres along and the angle two coord components stand for.
pub fn uncoord(c: [u16; 2], along: f64) -> [f64; 2] {
    [f64::from(c[0]) / TOP * along, f64::from(c[1]) / TOP * TAU]
}

#[cfg(test)]
mod tests;
