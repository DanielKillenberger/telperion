//! The apical rosette, and the pinnate grouping that makes one placement a
//! frond.
//!
//! A rosette stands at the apex of every stem - the childless node of an
//! order-zero axis - and bears its fronds on a phyllotactic spiral about that
//! axis. Like the short shoots it is a placement source over wood the skeleton
//! already has: no node, no run, every draw keyed by the bearing apex's own
//! identity and the seed, and the apices visited in identity order, so no
//! storage or build order moves a frond. Where a rosette stands it is the
//! tree's only source: a stem that bears a frond crown bears nothing along its
//! length.
//!
//! The frond itself is a grouping at the placement, never a compound element:
//! one placement becomes `leaflet_count` leaflets strung along a rachis,
//! alternating sides, so the element stays one blade and a leaflet stays one
//! instance. That grouping is the placement's, not the rosette's, so every
//! source expands through [`fan`] and a family that states leaflets without a
//! rosette draws compound leaves on the wood it already clothed.
//!
//! Below the living crown a rosette may keep its dead: the skirt continues the
//! same spiral down the stem, each dead frond hung at the skirt's own pitch
//! and drawn at its share of a living frond's length, and every leaf of it is
//! marked withered so the crown draws it in the dead colour.
//!
//! The rows, their rails and the apices are read with the placement compiled
//! out, so the leaf plan can tell a frond crown from a clothed one; only the
//! placing itself needs the `geometry` feature.
use super::{range, CanopyParams};
#[cfg(feature = "geometry")]
use super::{
    station::{matrix, reserve},
    Instances,
};
use crate::rng::Rng;
use crate::{
    math::{Transcendental, Vec3},
    tree::Tree,
    Error, Result,
};

/// The most fronds one rosette bears, and the most leaflets one rachis
/// carries: rails wide enough for any crown a table has asked for.
pub const MAX_FRONDS: u32 = 128;
pub const MAX_LEAFLETS: u32 = 256;

/// One rosette: the apex it crowns, the point it leaves and the axis it
/// stands on.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rosette {
    /// The apex node the fronds are borne at.
    pub apex: usize,
    pub at: Vec3,
    pub axis: Vec3,
}

/// Whether this table stands a rosette at all. At zero fronds the canopy
/// clothes wood as it always did.
pub(super) fn bearing(p: &CanopyParams) -> bool {
    p.rosette_fronds > 0
}

/// Dead fronds each rosette keeps: none unless a table states a count and a
/// length to draw them at.
pub(super) fn skirt(p: &CanopyParams) -> u32 {
    if p.skirt_length > 0. {
        p.skirt_fronds
    } else {
        0
    }
}

/// Metres below the apex the deepest frond, living or dead, leaves the axis:
/// the living crown's own depth unless a skirt continues its spacing further.
pub(super) fn deepest(p: &CanopyParams) -> f64 {
    let kept = skirt(p);
    if kept == 0 {
        return p.rosette_depth;
    }
    p.rosette_depth * (f64::from(p.rosette_fronds + kept - 1) / spacing(p))
}

/// The steps of the living crown's spacing its depth is cut into: one short
/// of its fronds, and one where a lone frond has no spacing of its own.
fn spacing(p: &CanopyParams) -> f64 {
    f64::from(p.rosette_fronds.saturating_sub(1)).max(1.)
}

/// Leaflets one placement carries: the grouping is off until a table states
/// both a count above one and a rachis to string them along.
pub(super) fn leaflets(p: &CanopyParams) -> usize {
    if p.leaflet_count > 1 && p.rachis_length > 0. {
        p.leaflet_count as usize
    } else {
        1
    }
}

/// Every row on its rail, each refused by its own name.
pub(crate) fn validate(p: &CanopyParams) -> Result<()> {
    crate::catalogue::check(CanopyParams::ROWS, p, crate::catalogue::Site::Rosette)
}

/// Every stem apex this tree offers, in identity order. The apex is the last
/// node of the stem run, so a tree whose twig layer still stands above it
/// reads the same apex as one whose apical twigs have been cleared.
pub fn rosettes(tree: &Tree) -> Vec<Rosette> {
    tree.stem_apices()
        .into_iter()
        .map(|i| {
            let node = &tree.nodes[i];
            let from = tree.nodes[node.parent.unwrap() as usize].position;
            let along = node.position - from;
            Rosette {
                apex: i,
                at: node.position,
                axis: if along.length_squared() > 1e-18 {
                    along.normalized()
                } else {
                    Vec3::Y
                },
            }
        })
        .collect()
}

#[cfg(feature = "geometry")]
/// Leaves the rosettes place before any cull: apices by fronds by leaflets.
pub(super) fn count(tree: &Tree, p: &CanopyParams) -> Result<usize> {
    if !bearing(p) {
        return Ok(0);
    }
    let budget = || Error::ResourceLimit("foliage instance budget");
    (p.rosette_fronds as usize + skirt(p) as usize)
        .checked_mul(leaflets(p))
        .and_then(|per| rosettes(tree).len().checked_mul(per))
        .filter(|total| *total <= p.max_instances)
        .ok_or_else(budget)
}

/// One frond on a rosette's spiral: where it leaves the axis, the way its
/// rachis heads, the radial it leans out along, the canopy it is drawn at (a
/// dead frond at its share of a living one) and its place `k` on the spiral,
/// which keys its draws.
#[derive(Debug, Clone, Copy)]
pub(super) struct Frond {
    pub(super) k: u32,
    pub(super) at: Vec3,
    pub(super) heading: Vec3,
    pub(super) radial: Vec3,
    pub(super) canopy: CanopyParams,
    pub(super) dead: bool,
}

/// Every frond of one rosette, living then dead, in spiral order. The living
/// crown opens from a spike to a skirt: the youngest frond stands at the
/// pitch the row states and the oldest that much further out again. The
/// skirt continues the same spiral and spacing past the oldest living frond,
/// each dead frond hung at the skirt's pitch and drawn at its share of a
/// living frond's length. The placement and the leaf plan both read this.
pub(super) fn fronds(rosette: &Rosette, p: &CanopyParams) -> impl Iterator<Item = Frond> {
    let (fronds, steps) = (p.rosette_fronds, spacing(p));
    let oldest = f64::from(fronds.saturating_sub(1));
    let dead = CanopyParams {
        size: p.size * p.skirt_length,
        rachis_length: p.rachis_length * p.skirt_length,
        ..*p
    };
    let (normal, binormal) = frame(rosette.axis);
    let (rosette, p) = (*rosette, *p);
    (0..fronds + skirt(&p)).map(move |k| {
        let (sin, cos) = (f64::from(k) * p.rosette_divergence.to_radians()).sin_cos_fixed();
        let radial = normal * cos + binormal * sin;
        let living = k < fronds;
        let (pitch, depth) = if living {
            let age = if oldest > 0. {
                f64::from(k) / oldest
            } else {
                0.
            };
            let pitch = (p.rosette_pitch + p.rosette_pitch_spread * age).to_radians();
            (pitch, p.rosette_depth * age)
        } else {
            (
                p.skirt_pitch.to_radians(),
                p.rosette_depth * (f64::from(k) / steps),
            )
        };
        let (lean, upright) = pitch.sin_cos_fixed();
        Frond {
            k,
            at: rosette.at - rosette.axis * depth,
            heading: (rosette.axis * upright + radial * lean).normalized(),
            radial,
            canopy: if living { p } else { dead },
            dead: !living,
        }
    })
}

#[cfg(feature = "geometry")]
/// Hang every rosette's fronds into an already-sized crown, the skirt's in
/// the dead colour.
pub(super) fn clothe(
    tree: &Tree,
    seed: u32,
    p: &CanopyParams,
    out: &mut Instances,
    mut owners: Option<&mut Vec<u32>>,
) -> Result<()> {
    if !bearing(p) || p.size == 0. {
        return Ok(());
    }
    let per = leaflets(p);
    for rosette in rosettes(tree) {
        reserve(out, (p.rosette_fronds + skirt(p)) as usize * per, *p)?;
        let birth = tree.nodes[rosette.apex].identity.birth_order();
        let mut withered = None;
        for frond in fronds(&rosette, p) {
            if frond.dead && withered.is_none() {
                withered = Some(out.leaves.len());
            }
            let mut rng = stream(seed, birth, &frond);
            let (at, heading, radial) = (frond.at, frond.heading, frond.radial);
            fan(
                at,
                heading,
                radial,
                rosette.axis,
                frond.canopy,
                &mut rng,
                out,
            )?;
        }
        if let Some(from) = withered {
            out.wither(from);
        }
        if let Some(owners) = owners.as_mut() {
            owners.resize(out.leaves.len(), rosette.apex as u32);
        }
    }
    Ok(())
}

#[cfg(feature = "geometry")]
/// `clothe` for the growth path, which places its own recorded leaves first
/// and draws the rosette live from the wood on screen.
pub fn place_rosette(tree: &Tree, seed: u32, p: CanopyParams, out: &mut Instances) -> Result<()> {
    tree.validate_solved()?;
    range(p.size, 0., 1000., "foliage size")?;
    validate(&p)?;
    clothe(tree, seed, &p, out, None)
}

#[cfg(feature = "geometry")]
/// The matrices one placement stands for: one where the grouping is off, else
/// `leaflet_count` leaflets along the rachis that leaves `point` on `heading`
/// (`leaflet::leaflets`), each turned in the frond's plane.
pub(super) fn fan(
    point: Vec3,
    heading: Vec3,
    tangent: Vec3,
    normal: Vec3,
    p: CanopyParams,
    rng: &mut Rng,
    out: &mut Instances,
) -> Result<()> {
    let count = leaflets(&p);
    if count == 1 {
        out.push(&matrix(point, heading, tangent, normal, p, rng)?);
        return Ok(());
    }
    let (_, side) = frame(heading);
    for leaflet in super::leaflet::leaflets(point, heading, p, count) {
        let mut placed = matrix(leaflet.at, leaflet.axis, leaflet.run, side, p, rng)?;
        if let Some(share) = leaflet.share {
            for column in 0..3 {
                for row in 0..3 {
                    placed[column * 4 + row] *= share as f32;
                }
            }
        }
        out.push(&placed);
    }
    Ok(())
}

/// Two unit vectors square to `axis` and to each other: the frame a spiral is
/// turned in and a rachis is arched out of. One frame a stem, taken on the
/// apex's own axis: the reference is a vanishing vector on a near-vertical
/// trunk, so a frame per node would scatter a lattice's phase.
pub fn frame(axis: Vec3) -> (Vec3, Vec3) {
    let up = Vec3::Y - axis * axis.y;
    let normal = if up.length_squared() > 1e-12 {
        up.normalized()
    } else {
        axis.perpendicular()
    };
    (normal, axis.cross(normal))
}

/// The stream one frond draws from: its leaflets' scatter and size, in order.
pub(super) fn stream(seed: u32, birth: u64, frond: &Frond) -> Rng {
    Rng::new(key(seed, birth, frond.k))
}

/// SplitMix64's finaliser over the seed, the apex's birth order and the
/// frond's place on the spiral: the frond's own stream.
fn key(seed: u32, birth: u64, frond: u32) -> u32 {
    let mut z = (u64::from(seed ^ 0x1d0f_3a57) << 32 | u64::from(frond))
        ^ birth.wrapping_mul(0x9e37_79b9_7f4a_7c15);
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    (z ^ (z >> 31)) as u32
}
