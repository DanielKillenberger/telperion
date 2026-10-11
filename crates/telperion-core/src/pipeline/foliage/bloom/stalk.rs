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
//!
//! A floret's pedicel is in each of its levels. The rachis is drawn by its
//! own instance alone, so its levels stand in front of the floret's, at
//! deviations no floret is ever drawn at (`RACHIS_LEVEL`), coarsest first:
//! a ribbon facing the eye over two of the axis's segments, over four and
//! over all eight, then the tube. Selection gives a rachis under three
//! pixels wide the coarsest ribbon whose chords stand under half a pixel
//! off the axis the pedicels are seated on, as every level is chosen by
//! its error, and the tube above. No floret runs the rachis's vertices,
//! nor a rachis the petals'.
use super::super::Element;
use crate::math::{Transcendental, Vec3};
use std::f64::consts::TAU;

/// The sides of a stalk's section.
pub const SIDES: u32 = 3;
/// The rachis's rings, one at each of the axis's points (`bloom.rs`'s
/// `SEGMENTS` + 1); a pedicel takes the first and the last of them.
pub const RINGS: u32 = 9;
/// The surface coordinate's `x` on a pedicel's vertex, a rachis tube's and
/// a rachis ribbon's (two sides, its edges).
pub const PEDICEL: f32 = -1.0;
pub const RACHIS: f32 = -2.0;
pub const RIBBON: f32 = -3.0;
/// The size a stalk's vertices stand at in the element, in metres.
const TINY: f64 = 1e-6;
/// The coarsest rachis level's deviation, and each finer one's a tenth of
/// the one before: past any a floret's size on screen meets, so selection
/// gives them only the rachis instances it places there.
pub const RACHIS_LEVEL: f64 = 1e30;
/// The rachis's levels.
pub const RACHIS_LEVELS: usize = 4;

/// The stalks' triangles: the pedicel's, in every floret level and in the
/// element's own, and the rachis's levels, coarsest first.
pub(super) struct Stalks {
    pub pedicel: Vec<u32>,
    pub rachis: [Vec<u32>; RACHIS_LEVELS],
}

/// The stalks appended to `e`: one pedicel's tube, among its triangles,
/// and one rachis's levels, at the origin.
pub(super) fn append(e: &mut Element) -> Stalks {
    let pedicel = tube(e, (PEDICEL, SIDES), &[0, RINGS - 1]);
    e.indices.extend_from_slice(&pedicel);
    let rings: Vec<u32> = (0..RINGS).collect();
    let rachis = [
        tube(e, (RIBBON, 2), &[0, 4, 8]),
        tube(e, (RIBBON, 2), &[0, 2, 4, 6, 8]),
        tube(e, (RIBBON, 2), &rings),
        tube(e, (RACHIS, SIDES), &rings),
    ];
    Stalks { pedicel, rachis }
}

/// A tube of `sides` sides through `rings`, its vertices named `kind`; a
/// ribbon is a tube of two sides, one quad a segment.
fn tube(e: &mut Element, (kind, sides): (f32, u32), rings: &[u32]) -> Vec<u32> {
    let first = e.positions.len() as u32;
    for ring in rings {
        for side in 0..sides {
            let (sin, cos) = (TAU * f64::from(side) / f64::from(SIDES)).sin_cos_fixed();
            let along = f64::from(*ring) / f64::from(RINGS - 1);
            e.positions.push(Vec3::new(cos, along, sin) * TINY);
            e.coords.extend([kind, (ring * SIDES + side) as f32]);
        }
    }
    let mut indices = Vec::new();
    // A ribbon's two edges make one quad a segment, not a tube's two.
    let quads = if sides == 2 { 1 } else { sides };
    for r in 0..rings.len() as u32 - 1 {
        for j in 0..quads {
            let at = |ring: u32, side: u32| first + ring * sides + side % sides;
            let (a, b) = (at(r, j), at(r, j + 1));
            let (c, d) = (at(r + 1, j), at(r + 1, j + 1));
            indices.extend([a, b, d, a, d, c]);
        }
    }
    indices
}
