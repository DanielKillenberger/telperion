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
use super::{
    range,
    station::{matrix, reserve},
    CanopyParams, Instances,
};
use crate::branching::MAX_LEAF_BASES;
use crate::math::Transcendental;
use crate::{math::Vec3, rng::Rng, tree::Tree, Error, Result};

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
    for (v, l, h, n) in [
        (p.rosette_divergence, -1e9, 1e9, "rosette divergence"),
        (p.rosette_pitch, 0., 180., "rosette pitch"),
        (p.rosette_pitch_spread, 0., 180., "rosette pitch spread"),
        (p.rosette_depth, 0., 100., "rosette depth"),
        (p.rachis_length, 0., 1e3, "rachis length"),
        (p.leaflet_pitch, 0., 90., "leaflet pitch"),
        (p.rachis_arch, -1., 1., "rachis arch"),
        (p.terminal_leaflet, 0., 1., "terminal leaflet"),
        (p.leaf_base_length, 0., 10., "leaf base length"),
        (p.leaf_base_radius, 0., 1., "leaf base radius"),
        (p.leaf_base_pitch, 0., 180., "leaf base pitch"),
        (p.leaf_base_weathering, 0., 1., "leaf base weathering"),
        (p.acanthophyll_length, 0., 1., "acanthophyll length"),
        (p.acanthophyll_pitch, 0., 90., "acanthophyll pitch"),
    ] {
        range(v, l, h, n)?;
    }
    if p.rosette_fronds > MAX_FRONDS {
        return Err(Error::InvalidInput("rosette fronds"));
    }
    if !(1..=MAX_LEAFLETS).contains(&p.leaflet_count) {
        return Err(Error::InvalidInput("leaflet count"));
    }
    if p.leaf_bases > MAX_LEAF_BASES {
        return Err(Error::InvalidInput("leaf bases"));
    }
    if p.acanthophylls > MAX_LEAFLETS {
        return Err(Error::InvalidInput("acanthophylls"));
    }
    Ok(())
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

/// Leaves the rosettes place before any cull: apices by fronds by leaflets.
pub(super) fn count(tree: &Tree, p: &CanopyParams) -> Result<usize> {
    if !bearing(p) {
        return Ok(0);
    }
    let budget = || Error::ResourceLimit("foliage instance budget");
    (p.rosette_fronds as usize)
        .checked_mul(leaflets(p))
        .and_then(|per| rosettes(tree).len().checked_mul(per))
        .filter(|total| *total <= p.max_instances)
        .ok_or_else(budget)
}

/// Hang every rosette's fronds into an already-sized crown.
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
    let fronds = p.rosette_fronds;
    let per = leaflets(p);
    // The youngest frond stands at the pitch the row states and the oldest
    // that much further out again, so the crown opens from a spike to a skirt.
    let oldest = f64::from(fronds.saturating_sub(1));
    for rosette in rosettes(tree) {
        reserve(out, fronds as usize * per, *p)?;
        let birth = tree.nodes[rosette.apex].identity.birth_order();
        let (normal, binormal) = frame(rosette.axis);
        for k in 0..fronds {
            let mut rng = Rng::new(key(seed, birth, k));
            let age = if oldest > 0. {
                f64::from(k) / oldest
            } else {
                0.
            };
            let (sin, cos) = (f64::from(k) * p.rosette_divergence.to_radians()).sin_cos_fixed();
            let radial = normal * cos + binormal * sin;
            let pitch = (p.rosette_pitch + p.rosette_pitch_spread * age).to_radians();
            let (lean, upright) = pitch.sin_cos_fixed();
            let heading = (rosette.axis * upright + radial * lean).normalized();
            let at = rosette.at - rosette.axis * (p.rosette_depth * age);
            fan(at, heading, radial, rosette.axis, *p, &mut rng, out)?;
        }
        if let Some(owners) = owners.as_mut() {
            owners.resize(out.leaves.len(), rosette.apex as u32);
        }
    }
    Ok(())
}

/// `clothe` for the growth path, which places its own recorded leaves first
/// and draws the rosette live from the wood on screen.
pub fn place_rosette(tree: &Tree, seed: u32, p: CanopyParams, out: &mut Instances) -> Result<()> {
    tree.validate_solved()?;
    range(p.size, 0., 1000., "foliage size")?;
    validate(&p)?;
    clothe(tree, seed, &p, out, None)
}

/// The matrices one placement stands for: one where the grouping is off, else
/// `leaflet_count` leaflets along the rachis that leaves `point` on `heading`.
///
/// The rachis runs a `rachis_length` from the station, bending out of that
/// straight line by its arch as it goes, and each leaflet leaves it by
/// `leaflet_pitch` to alternating sides. The last of them closes the rachis's
/// end as `terminal_leaflet` blends its pitch back toward the rachis itself.
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
    // The frond's own plane: the rachis runs along `heading`, the leaflets
    // leave it to either side, and the arch lifts it out of the line between.
    let (lift, side) = frame(heading);
    let length = p.rachis_length;
    let pitch = p.leaflet_pitch.to_radians();
    for leaf in 0..count {
        let t = (leaf + 1) as f64 / count as f64;
        let at = point + heading * (length * t) + lift * (p.rachis_arch * length * t * t);
        // The rachis's own direction where this leaflet leaves it.
        let run = (heading + lift * (2. * p.rachis_arch * t)).normalized();
        let closing = if leaf + 1 == count {
            1. - p.terminal_leaflet
        } else {
            1.
        };
        let hand = if leaf % 2 == 0 { 1. } else { -1. };
        // A basal leaflet borne as a spine leaves the rachis at its own pitch
        // and is drawn at its own share of the size the leaflet would have
        // had. No draw moves: the spine is the same instance, scaled.
        let borne = spine(leaf, &p);
        let pitch = borne.map_or(pitch, |(_, steeper)| steeper);
        let axis = run.rotate(lift, hand * pitch * closing);
        let mut placed = matrix(at, axis, run, side, p, rng)?;
        if let Some((share, _)) = borne {
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

/// The share of its own size a basal leaflet is drawn at and the radians it
/// leaves the rachis on, where the rows bear it as a spine. None everywhere
/// else, so a frond that bears none is scaled by nothing at all.
fn spine(index: usize, p: &CanopyParams) -> Option<(f64, f64)> {
    let borne = (index as u64) < u64::from(p.acanthophylls) && p.acanthophyll_length > 0.;
    borne.then(|| (p.acanthophyll_length, p.acanthophyll_pitch.to_radians()))
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

/// SplitMix64's finaliser over the seed, the apex's birth order and the
/// frond's place on the spiral: the frond's own stream.
fn key(seed: u32, birth: u64, frond: u32) -> u32 {
    let mut z = (u64::from(seed ^ 0x1d0f_3a57) << 32 | u64::from(frond))
        ^ birth.wrapping_mul(0x9e37_79b9_7f4a_7c15);
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    (z ^ (z >> 31)) as u32
}
