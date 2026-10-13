//! The apical rosette, and the pinnate grouping that makes one placement a
//! frond.
//!
//! A rosette stands at the apex of every stem - the childless node of an
//! order-zero axis - and bears its fronds on a phyllotactic spiral about that
//! axis. It is a placement source over wood the skeleton
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
use super::CanopyParams;
#[cfg(feature = "geometry")]
use super::{
    station::{matrix, reserve},
    Instances,
};
use crate::rng::Rng;
#[cfg(feature = "geometry")]
use crate::Error;
use crate::{
    math::{Transcendental, Vec3},
    tree::Tree,
    Result,
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
    // The skirt's last frond fades a place further down (fn-270 R5b).
    p.rosette_depth * (f64::from(p.rosette_fronds + kept) / spacing(p))
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
    crate::catalogue::check(CanopyParams::CHECKS, p, crate::catalogue::Site::Rosette)
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
    slots(p, Crown::of(tree, p))
        .checked_mul(leaflets(p))
        .and_then(|per| rosettes(tree).len().checked_mul(per))
        .filter(|total| *total <= p.max_instances)
        .ok_or_else(budget)
}

/// The fronds a rosette draws: its living and dead ones, and within the
/// year one more while the oldest leaves the skirt as a new one emerges
/// (fn-270 R5b, `fronds`).
pub(super) fn slots(p: &CanopyParams, crown: Crown) -> usize {
    p.rosette_fronds as usize + skirt(p) as usize + usize::from(crown.made().fract() > 0.0)
}

/// How many fronds a crown has made by the year `year` it is drawn in, `y`
/// a year (fn-270 R5b): its whole years', and this year's as they emerge,
/// evenly over its stem's elongation from budburst; and the share of its
/// whole size frond `serial` stands at, a frond of this year's expanding
/// from its emergence. A tree no engine grew makes none.
#[derive(Debug, Clone, Copy)]
pub struct Crown {
    year: Option<crate::tree::Year>,
    per_year: f64,
}

impl Crown {
    pub(crate) fn of(tree: &Tree, p: &CanopyParams) -> Self {
        Self::new(tree.year, p)
    }

    /// The crown of a tree drawn in `year` (`executor::year`), for an
    /// executor that holds no tree.
    pub fn new(year: Option<crate::tree::Year>, p: &CanopyParams) -> Self {
        Self { year, per_year: p.rosette_year }
    }

    /// The rows the device draws the crown by (`stations.wgsl`'s
    /// `rosette`): the fronds made, a year's, the whole years before the
    /// last (negative where no engine grew the tree) and the days into
    /// it; its stem's budburst, elongation and expansion, and the fronds
    /// it draws past its living and dead ones (`slots`).
    pub fn rows(&self) -> [[f64; 4]; 2] {
        let extra = f64::from(u8::from(self.made().fract() > 0.0));
        match (self.clock(), self.year) {
            (Some((whole, tau)), Some(y)) => {
                [[self.made(), self.per_year, whole, tau], [y.budburst, y.elongation, y.expansion, extra]]
            }
            _ => [[0.0, self.per_year, -1.0, 0.0], [0.0; 4]],
        }
    }

    /// The years grown before the last, and how far into it.
    fn clock(&self) -> Option<(f64, f64)> {
        let y = self.year?;
        let whole = y.age.ceil().max(1.0) - 1.0;
        Some((whole, (y.age - whole) * telperion_space::unfold::YEAR))
    }

    /// Fronds made: whole years' and this year's emerged.
    pub fn made(&self) -> f64 {
        let (Some((whole, tau)), Some(y)) = (self.clock(), self.year) else {
            return 0.0;
        };
        let emerged = match y.elongation > 0.0 {
            true => ((tau - y.budburst) / y.elongation).clamp(0.0, 1.0),
            false => f64::from(u8::from(tau > y.budburst)),
        };
        self.per_year * (whole + emerged)
    }

    /// The share of its whole size frond `serial` stands at.
    pub fn unfolded(&self, serial: f64) -> f64 {
        let (Some((whole, tau)), Some(y)) = (self.clock(), self.year) else {
            return 1.0;
        };
        let new = serial + 1.0 - self.per_year * whole;
        if new <= 0.0 || self.per_year <= 0.0 {
            return 1.0;
        }
        let emerges = y.budburst + y.elongation * (new / self.per_year).min(1.0);
        telperion_space::unfold::run(tau - emerges, y.expansion)
    }
}

/// One frond on a rosette's spiral: where it leaves the axis, the way its
/// rachis heads, the radial it leans out along, the canopy it is drawn at (a
/// dead frond at its share of a living one, an emerging one at its share of
/// its whole size) and its serial among the fronds its crown has made,
/// which keys its draws.
#[derive(Debug, Clone, Copy)]
pub(super) struct Frond {
    pub(super) serial: i64,
    pub(super) at: Vec3,
    pub(super) heading: Vec3,
    pub(super) radial: Vec3,
    pub(super) canopy: CanopyParams,
    pub(super) dead: bool,
}

/// Every frond of one rosette, newest first (`slots`), each at its place
/// on the spiral: the fronds made so far (`crown`), the newest at the apex
/// and each older one a place further down, by degree as the next
/// emerges. The living crown opens from a spike to a skirt: the youngest
/// frond stands at the pitch the row states and the oldest that much
/// further out again. The skirt continues the same spiral and spacing past
/// the oldest living frond, each dead frond hung at the skirt's pitch and
/// drawn at its share of a living frond's length; a frond turns from the
/// one to the other over its last place in the living crown, and fades
/// over the skirt's last. Its turn on the spiral is its serial's, so a
/// frond holds still as the crown grows above it. The placement and the
/// leaf plan both read this.
pub(super) fn fronds(rosette: &Rosette, p: &CanopyParams, crown: Crown) -> impl Iterator<Item = Frond> {
    let (fronds, steps) = (p.rosette_fronds, spacing(p));
    let oldest = f64::from(fronds.saturating_sub(1));
    let total = f64::from(fronds + skirt(p));
    let made = crown.made();
    let newest = made.ceil() as i64 - 1;
    let (normal, binormal) = frame(rosette.axis);
    let (rosette, p) = (*rosette, *p);
    (0..slots(&p, crown) as i64).map(move |i| {
        let serial = newest - i;
        let k = (made - 1.0 - serial as f64).max(0.0);
        let (sin, cos) = (serial as f64 * p.rosette_divergence.to_radians()).sin_cos_fixed();
        let radial = normal * cos + binormal * sin;
        let living = |k: f64| {
            let age = if oldest > 0. { k.min(oldest) / oldest } else { 0. };
            ((p.rosette_pitch + p.rosette_pitch_spread * age).to_radians(), p.rosette_depth * age, 1.0)
        };
        let dead = |k: f64| (p.skirt_pitch.to_radians(), p.rosette_depth * (k / steps), p.skirt_length);
        // How far it has turned from living to dead over its last living
        // place, and how far it has left the skirt.
        let turned = if skirt(&p) > 0 { (k - oldest).clamp(0.0, 1.0) } else { 0.0 };
        let (a, b) = (living(k), dead(k.max(f64::from(fronds))));
        let mix = |x: f64, y: f64| x + (y - x) * turned;
        let (pitch, depth, length) = (mix(a.0, b.0), mix(a.1, b.1), mix(a.2, b.2));
        let share = crown.unfolded(serial as f64) * (total - k).clamp(0.0, 1.0);
        let canopy = CanopyParams { size: p.size * length * share, rachis_length: p.rachis_length * length * share, ..p };
        let (lean, upright) = pitch.sin_cos_fixed();
        Frond {
            serial,
            at: rosette.at - rosette.axis * depth,
            heading: (rosette.axis * upright + radial * lean).normalized(),
            radial,
            canopy,
            dead: turned >= 0.5,
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
    let crown = Crown::of(tree, p);
    for rosette in rosettes(tree) {
        reserve(out, slots(p, crown) * per, *p)?;
        let stem = stem_key(tree, &rosette);
        for frond in fronds(&rosette, p, crown) {
            let from = out.leaves.len();
            let mut rng = stream(seed, stem, &frond);
            let (at, heading, radial) = (frond.at, frond.heading, frond.radial);
            fan(
                at,
                heading,
                radial,
                rosette.axis,
                (frond.canopy, per),
                &mut rng,
                out,
            )?;
            if frond.dead {
                out.wither(from);
            }
        }
        if let Some(owners) = owners.as_mut() {
            owners.resize(out.leaves.len(), rosette.apex as u32);
        }
    }
    Ok(())
}

#[cfg(feature = "geometry")]
/// The matrices one placement stands for: one where the grouping is off, else
/// `count` leaflets along the rachis that leaves `point` on `heading`
/// (`leaflet::leaflets`), each turned in the frond's plane. The count is
/// the family's (`leaflets`), so a frond drawn at no size still stands its
/// leaflets (fn-270 R5b).
pub(super) fn fan(
    point: Vec3,
    heading: Vec3,
    tangent: Vec3,
    normal: Vec3,
    (p, count): (CanopyParams, usize),
    rng: &mut Rng,
    out: &mut Instances,
) -> Result<()> {
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

/// What keys a rosette's fronds: its stem's first node's identity, which
/// stays as the stem grows (fn-270 R5b).
pub(super) fn stem_key(tree: &Tree, rosette: &Rosette) -> u64 {
    tree.nodes[tree.nodes[rosette.apex].branch as usize].identity.birth_order()
}

/// The stream one frond draws from: its leaflets' scatter and size, in order.
pub(super) fn stream(seed: u32, stem: u64, frond: &Frond) -> Rng {
    Rng::new(key(seed, stem, frond.serial as u32))
}

/// SplitMix64's finaliser over the seed, the stem's key and the frond's
/// serial: the frond's own stream.
fn key(seed: u32, birth: u64, frond: u32) -> u32 {
    let mut z = (u64::from(seed ^ 0x1d0f_3a57) << 32 | u64::from(frond))
        ^ birth.wrapping_mul(0x9e37_79b9_7f4a_7c15);
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    (z ^ (z >> 31)) as u32
}
