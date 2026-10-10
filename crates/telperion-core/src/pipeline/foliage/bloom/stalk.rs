//! The flowers' stalks in the organ element (fn-253): the pedicel each
//! floret stands on and the rachis its inflorescence hangs from, drawn in
//! the organ's one draw. Their shape is not the element's to scale: a
//! stalk runs between ends the floret's transforms never move (its seat on
//! the rachis or the wood, and the floret's base), at a width in metres
//! the floret's size never touches. So every stalk vertex stands within
//! `TINY` of the element's origin, on its own section's shape so that its
//! triangles keep an area, where no bound, sphere or deviation of the
//! floret reads it; its surface coordinate names it instead: `x` its kind,
//! `y` its ring along the stalk times `SIDES` plus its side. The organ's
//! vertex stage lays it from the instance (`foliage.wgsl`).
use super::super::Element;
use crate::math::{Transcendental, Vec3};
use std::f64::consts::TAU;

/// The sides of a stalk's section.
pub const SIDES: u32 = 3;
/// The rachis's rings, one at each of the axis's points (`bloom.rs`'s
/// `SEGMENTS` + 1); a pedicel takes the first and the last of them.
pub const RINGS: u32 = 9;
/// The surface coordinate's `x` on a pedicel's vertex and a rachis's.
pub const PEDICEL: f32 = -1.0;
pub const RACHIS: f32 = -2.0;
/// The size a stalk's vertices stand at in the element, in metres.
const TINY: f64 = 1e-6;

/// The stalks appended to `e`: one pedicel's tube and one rachis's, at the
/// origin. Returns their triangles, which every level draws.
pub(super) fn append(e: &mut Element) -> Vec<u32> {
    let mut indices = tube(e, PEDICEL, &[0, RINGS - 1]);
    let rings: Vec<u32> = (0..RINGS).collect();
    indices.extend(tube(e, RACHIS, &rings));
    e.indices.extend_from_slice(&indices);
    indices
}

/// A tube of `SIDES` sides through `rings`, its vertices named `kind`.
fn tube(e: &mut Element, kind: f32, rings: &[u32]) -> Vec<u32> {
    let first = e.positions.len() as u32;
    for ring in rings {
        for side in 0..SIDES {
            let (sin, cos) = (TAU * f64::from(side) / f64::from(SIDES)).sin_cos_fixed();
            let along = f64::from(*ring) / f64::from(RINGS - 1);
            e.positions.push(Vec3::new(cos, along, sin) * TINY);
            e.coords.extend([kind, (ring * SIDES + side) as f32]);
        }
    }
    let mut indices = Vec::new();
    for r in 0..rings.len() as u32 - 1 {
        for j in 0..SIDES {
            let at = |ring: u32, side: u32| first + ring * SIDES + side % SIDES;
            let (a, b) = (at(r, j), at(r, j + 1));
            let (c, d) = (at(r + 1, j), at(r + 1, j + 1));
            indices.extend([a, b, d, a, d, c]);
        }
    }
    indices
}
