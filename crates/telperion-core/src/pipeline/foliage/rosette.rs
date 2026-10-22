//! The apical rosette, and the pinnate grouping that makes one placement a
//! frond.
//!
//! A rosette stands at the apex of every stem - the childless node of an
//! order-zero axis - and bears its fronds on a phyllotactic spiral about that
//! axis. It is a placement source over wood the skeleton
//! already has: no node, no run, every draw keyed by the bearing apex's own
//! identity and the seed, and the apices visited in identity order, so no
//! storage or build order moves a frond. A rosette stands beside whatever
//! wood the canopy's cauline share clothes (fn-277): a palm's share is none.
//!
//! A count of organs walks between two whole numbers by degree: each apex
//! bears the whole fronds and dead fronds either side of the row by its
//! own draw, and each placement the whole leaflets and spines either side
//! of theirs (`telperion_space::organs`), so the expected count is the
//! row's.
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
use super::organs::{apex, frond_key, frond_leaflets, placement};
use super::{
    crown::{Crown, Split},
    CanopyParams,
};
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

/// Whether this table stands a rosette at all: any fronds.
pub(super) fn bearing(p: &CanopyParams) -> bool {
    p.rosette_fronds > 0.
}

/// Dead fronds each rosette keeps: none unless a table states a count and a
/// length to draw them at.
pub(super) fn skirt(p: &CanopyParams) -> f64 {
    if p.skirt_length > 0. {
        p.skirt_fronds
    } else {
        0.
    }
}

/// Metres below the apex the deepest frond, living or dead, of any apex
/// leaves the axis: the living crown's own depth unless a skirt continues
/// its spacing further, over every living count an apex may draw (either
/// whole beside the row's) and the most dead fronds it keeps.
pub(super) fn deepest(p: &CanopyParams) -> f64 {
    let kept = skirt(p).ceil();
    let living = [p.rosette_fronds.floor(), p.rosette_fronds.ceil()];
    // The skirt's last frond fades a place further down (fn-270 R5b).
    let depth = |fronds: f64| {
        let p = CanopyParams {
            rosette_fronds: fronds,
            ..*p
        };
        p.rosette_depth * ((fronds + kept) / spacing(&p))
    };
    match kept == 0. {
        true => p.rosette_depth,
        false => living
            .into_iter()
            .filter(|&f| f > 0.)
            .map(depth)
            .fold(p.rosette_depth, f64::max),
    }
}

/// The steps of the living crown's spacing its depth is cut into: one short
/// of its fronds, and one where a lone frond has no spacing of its own.
fn spacing(p: &CanopyParams) -> f64 {
    (p.rosette_fronds - 1.).max(1.)
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
/// Leaves the rosettes place before any cull: every apex's fronds by each
/// one's leaflets.
pub(super) fn count(tree: &Tree, p: &CanopyParams) -> Result<usize> {
    if !bearing(p) {
        return Ok(0);
    }
    let budget = || Error::ResourceLimit("foliage instance budget");
    let mut total = 0usize;
    for rosette in rosettes(tree) {
        let stem = stem_node(tree, &rosette);
        let p = apex(p, stem);
        // An apex that draws no living frond stands no rosette.
        if !bearing(&p) {
            continue;
        }
        let crown = Crown::of(tree, &p);
        for serial in crown.serials() {
            let leaves = frond_leaflets(&p, crown.leaflets(serial as f64), frond_key(stem, serial));
            total = total
                .checked_add(leaves)
                .filter(|t| *t <= p.max_instances)
                .ok_or_else(budget)?;
        }
    }
    Ok(total)
}

/// One frond on a rosette's spiral: where it leaves the axis, the way its
/// rachis heads, the radial it leans out along, the canopy it is drawn at (a
/// dead frond at its share of a living one, an emerging one at its share of
/// its whole size, one made while the crown established at the size and
/// with the leaflets it was made at) and its serial among the fronds its crown has made,
/// which keys its draws.
#[derive(Debug, Clone, Copy)]
pub(super) struct Frond {
    pub(super) serial: i64,
    pub(super) at: Vec3,
    pub(super) heading: Vec3,
    pub(super) radial: Vec3,
    pub(super) canopy: CanopyParams,
    /// Its leaflets as strips of one blade while it was made young.
    pub(super) split: Option<Split>,
    pub(super) dead: bool,
}

/// Every frond of one rosette, newest first (`slots`), each at its place
/// on the spiral: the fronds made so far (`crown`), the newest at the apex
/// and each older one a place further down for every frond grown above
/// it, by degree as each unfolds, so a new frond grows small at the
/// centre and the crown below it moves out only as it grows (host,
/// fn-270 R5b). The living crown opens from a spike to a skirt: the youngest
/// frond stands at the pitch the row states and the oldest that much
/// further out again. The skirt continues the same spiral and spacing past
/// the oldest living frond, each dead frond hung at the skirt's pitch and
/// drawn at its share of a living frond's length; a frond turns from the
/// one to the other over its last place in the living crown, and fades
/// over the skirt's last. Its turn on the spiral is its serial's, so a
/// frond holds still as the crown grows above it. The placement and the
/// leaf plan both read this.
pub(super) fn fronds(
    rosette: &Rosette,
    p: &CanopyParams,
    crown: Crown,
    blade: f64,
) -> impl Iterator<Item = Frond> {
    let (fronds, steps) = (p.rosette_fronds, spacing(p));
    let oldest = (fronds - 1.).max(0.);
    let total = fronds + skirt(p);
    let (normal, binormal) = frame(rosette.axis);
    let (rosette, p) = (*rosette, *p);
    let mut above = 0.0;
    crown.serials().map(move |serial| {
        let unfolded = crown.unfolded(serial as f64);
        let established = crown.established(serial as f64);
        let k = above;
        above += unfolded;
        let (sin, cos) = (serial as f64 * p.rosette_divergence.to_radians()).sin_cos_fixed();
        let radial = normal * cos + binormal * sin;
        let living = |k: f64| {
            let age = if oldest > 0. {
                k.min(oldest) / oldest
            } else {
                0.
            };
            (
                (p.rosette_pitch + p.rosette_pitch_spread * age).to_radians(),
                p.rosette_depth * age,
                1.0,
            )
        };
        let dead = |k: f64| {
            (
                p.skirt_pitch.to_radians(),
                p.rosette_depth * (k / steps),
                p.skirt_length,
            )
        };
        // How far it has turned from living to dead over its last living
        // place, and how far it has left the skirt.
        let turned = if skirt(&p) > 0. {
            (k - oldest).clamp(0.0, 1.0)
        } else {
            0.0
        };
        let (a, b) = (living(k), dead(k.max(fronds)));
        let mix = |x: f64, y: f64| x + (y - x) * turned;
        let (pitch, depth, length) = (mix(a.0, b.0), mix(a.1, b.1), mix(a.2, b.2));
        let share = unfolded * established * (total - k).clamp(0.0, 1.0);
        let canopy = CanopyParams {
            size: p.size * length * share,
            rachis_length: p.rachis_length * length * share,
            ..p
        };
        let (canopy, split) = crown.segments(canopy, &p, serial as f64, blade);
        let (lean, upright) = pitch.sin_cos_fixed();
        Frond {
            serial,
            at: rosette.at - rosette.axis * depth,
            heading: (rosette.axis * upright + radial * lean).normalized(),
            radial,
            canopy,
            split,
            dead: turned >= 0.5,
        }
    })
}

/// One frond as placement draws it: the rosette it stands in and its
/// stem's key, the apex's own whole rows, the frond, the whole leaflets it
/// carries and the table they are drawn by.
#[derive(Debug, Clone, Copy)]
pub(super) struct Placed {
    pub(super) rosette: Rosette,
    pub(super) stem: u64,
    pub(super) apex: CanopyParams,
    pub(super) frond: Frond,
    pub(super) count: usize,
    pub(super) table: CanopyParams,
}

/// Every frond of every rosette with what placement draws it by, rosettes
/// in identity order and fronds newest first: the leaflets and the frond's
/// axis both read this. None where the table stands no rosette or draws its
/// leaves at no size.
pub(super) fn placed<'t>(
    tree: &'t Tree,
    p: &CanopyParams,
    blade: f64,
) -> impl Iterator<Item = Placed> + 't {
    let all = match bearing(p) && p.size != 0. {
        true => rosettes(tree),
        false => Vec::new(),
    };
    let p = *p;
    all.into_iter().flat_map(move |rosette| {
        let (stem, node) = (stem_key(tree, &rosette), stem_node(tree, &rosette));
        let at = apex(&p, node);
        // An apex that draws no living frond stands no rosette.
        let crown = bearing(&at).then(|| Crown::of(tree, &at));
        crown
            .into_iter()
            .flat_map(move |crown| fronds(&rosette, &at, crown, blade))
            .map(move |frond| {
                let key = frond_key(node, frond.serial);
                Placed {
                    rosette,
                    stem,
                    apex: at,
                    frond,
                    count: frond_leaflets(&at, frond.canopy.leaflet_count, key),
                    table: placement(&frond.canopy, key),
                }
            })
    })
}

#[cfg(feature = "geometry")]
/// Hang every rosette's fronds into an already-sized crown, the skirt's in
/// the dead colour.
pub(super) fn clothe(
    tree: &Tree,
    seed: u32,
    p: &CanopyParams,
    (blade, wedge): (f64, &super::rest::Wedge),
    out: &mut Instances,
    mut owners: Option<&mut Vec<u32>>,
) -> Result<()> {
    for Placed {
        rosette,
        stem,
        apex,
        frond,
        count,
        table,
    } in placed(tree, p, blade)
    {
        let from = out.leaves.len();
        let mut rng = stream(seed, stem, &frond);
        let (at, heading, radial) = (frond.at, frond.heading, frond.radial);
        reserve(out, count, apex)?;
        fan(
            at,
            heading,
            radial,
            rosette.axis,
            (
                table,
                count,
                frond.split,
                Some(super::rachis::girth(&table, &apex)),
            ),
            (&mut rng, None),
            (wedge, out),
        )?;
        if frond.dead {
            out.wither(from);
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
/// the placement's (`leaflets`), so a frond drawn at no size still stands
/// its leaflets (fn-270 R5b). The first leaflet draws on `rng`; the rest
/// on `more` where given, so a placement's leaflets move no other
/// placement's draws on a shared stream (fn-277), else on `rng`. The
/// rachis is `girth` thick where it leaves `point` (`leaflet::rachis`):
/// a frond's own, or none for a placement on wood, which draws no rachis
/// and stands its leaflets at `point`. Each matrix is rested on the ground
/// by `wedge` as it is stored (`rest::leaf`).
pub(super) fn fan(
    point: Vec3,
    heading: Vec3,
    tangent: Vec3,
    normal: Vec3,
    (p, count, split, girth): (CanopyParams, usize, Option<Split>, Option<f64>),
    (rng, mut more): (&mut Rng, Option<&mut Rng>),
    (wedge, out): (&super::rest::Wedge, &mut Instances),
) -> Result<()> {
    let step = out.reference.step().y;
    let rest = |m: [f32; 16]| super::rest::leaf(m, wedge, step);
    if count == 1 {
        let length = split.map_or(1.0, |s| lone(&s, &p));
        out.push(&rest(matrix(
            point,
            heading,
            tangent,
            normal,
            CanopyParams {
                size: p.size * length,
                ..p
            },
            rng,
        )?));
        return Ok(());
    }
    let (_, side) = frame(heading);
    for (k, leaflet) in
        super::leaflet::leaflets(point, heading, (p, girth), count, split).enumerate()
    {
        let rng = match more.as_deref_mut() {
            Some(more) if k > 0 => more,
            _ => &mut *rng,
        };
        let mut placed = matrix(leaflet.at, leaflet.axis, leaflet.run, side, p, rng)?;
        if let Some(share) = leaflet.share {
            for column in 0..3 {
                for row in 0..3 {
                    placed[column * 4 + row] *= share as f32;
                }
            }
        }
        out.push(&rest(placed));
    }
    Ok(())
}

/// A lone placement's length as a share of a mature leaflet's, where it
/// is a young frond's entire blade (`Split::segment`).
pub(super) fn lone(split: &Split, p: &CanopyParams) -> f64 {
    split.segment(0, 1, p.rachis_length, p.size * split.unit).1
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
/// What keys the counts of a rosette's organs (fn-277): the key of its
/// stem's first node, up its stem from the apex to the node the root or
/// wood that is no stem bears, as both executors walk it.
pub(crate) fn stem_node(tree: &Tree, rosette: &Rosette) -> u32 {
    let mut first = rosette.apex;
    while let Some(p) = tree.nodes[first].parent.map(|p| p as usize) {
        let root = tree.nodes[p].parent.is_none();
        if root || !tree.nodes[p].stem {
            break;
        }
        first = p;
    }
    tree.key(first)
}

pub(crate) fn stem_key(tree: &Tree, rosette: &Rosette) -> u64 {
    tree.nodes[tree.nodes[rosette.apex].branch as usize]
        .identity
        .birth_order()
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
