//! Short shoots: the spurs a few centimetres long a broadleaf carries all
//! along its older limbs and deep inside its crown, each ending in a cluster
//! of leaves. They are placements on wood the skeleton already has - no node,
//! no run, the node ceiling untouched - and every draw belongs to the wood
//! that bears it, keyed by its identity and the seed, so no other wood and no
//! storage or build order moves one.
use super::{
    range,
    station::{axis, matrix, reserve},
    CanopyParams, Instances,
};
use crate::math::Transcendental;
use crate::{
    envelope::Envelope,
    math::Vec3,
    rng::Rng,
    tree::{NodeKind, Tree},
    Error, Result,
};
use std::f64::consts::TAU;

/// The closest two short shoots may stand, in metres, and the furthest: a
/// walk from none thins in from here, where no tree has wood enough for one.
pub const SHORT_SHOOT_SPACING: (f64, f64) = (0.01, 1000.);
/// The most leaves one short shoot's cluster carries.
pub const MAX_SHORT_SHOOT_LEAVES: u32 = 8;

/// One short shoot: the wood it stands on, where it leaves the bark and where
/// its cluster sits.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ShortShoot {
    /// The node whose wood, from its parent to it, carries the shoot.
    pub wood: usize,
    pub base: Vec3,
    pub tip: Vec3,
}

/// Every row on its rail, each refused by its own name.
pub(super) fn validate(p: &CanopyParams) -> Result<()> {
    crate::ranges::POSITIVE_COUNT.check(p.clump_neighbours as f64, "clumpNeighbours")?;
    if p.short_shoot_spacing != 0. {
        let (low, high) = SHORT_SHOOT_SPACING;
        range(p.short_shoot_spacing, low, high, "short shoot spacing")?;
    }
    range(p.short_shoot_radius, 0., 1., "short shoot radius")?;
    range(p.short_shoot_length, 0., 0.5, "short shoot length")?;
    range(p.short_shoot_spread, 0., 90., "short shoot spread")?;
    if !(1..=MAX_SHORT_SHOOT_LEAVES).contains(&p.short_shoot_leaves) {
        return Err(Error::InvalidInput("short shoot leaves"));
    }
    Ok(())
}

/// Every short shoot the rows grow on this tree, wood by wood.
pub fn short_shoots(
    tree: &Tree,
    envelope: Envelope,
    seed: u32,
    p: CanopyParams,
) -> Result<Vec<ShortShoot>> {
    checked(tree, envelope, &p)?;
    let mut found = Vec::new();
    each(tree, envelope, seed, &p, |_, shoots| {
        found.extend(shoots.iter().map(|s| s.shoot));
        Ok(())
    })?;
    Ok(found)
}

/// Hang each short shoot's cluster after the placements already made: its
/// leaves held level, fanned across `spread` degrees either side of the
/// bearing the shoot leaves its wood on, so every cluster is one flat spray
/// whichever side of the limb it stands on. Wood is visited in identity
/// order, so a tree stored in any order gets the same matrices in the same
/// order.
pub fn place_short_shoots(
    tree: &Tree,
    envelope: Envelope,
    seed: u32,
    p: CanopyParams,
    out: &mut Instances,
) -> Result<()> {
    checked(tree, envelope, &p)?;
    clothe(tree, envelope, seed, &p, out, None)
}

/// `place_short_shoots` for a crown whose limb systems clump: `owners` names
/// the node that bears each placement already in `out`, and once the short
/// shoots are hung the whole crown is thinned by the rule the one-shot build
/// thins by.
pub fn place_short_shoots_clumped(
    tree: &Tree,
    envelope: Envelope,
    seed: u32,
    p: CanopyParams,
    mut owners: Vec<u32>,
    out: &mut Instances,
) -> Result<()> {
    checked(tree, envelope, &p)?;
    range(p.limb_clumping, 0., 1., "limb clumping")?;
    clothe(tree, envelope, seed, &p, out, Some(&mut owners))?;
    super::clumping::thin(tree, &owners, seed, p, out);
    Ok(())
}

/// What the placement stage checks before either source places a leaf.
fn checked(tree: &Tree, envelope: Envelope, p: &CanopyParams) -> Result<()> {
    tree.validate_solved()?;
    envelope.validate()?;
    range(p.size, 0., 1000., "foliage size")?;
    validate(p)
}

pub(super) fn clothe(
    tree: &Tree,
    envelope: Envelope,
    seed: u32,
    p: &CanopyParams,
    out: &mut Instances,
    mut owners: Option<&mut Vec<u32>>,
) -> Result<()> {
    let leaves = p.short_shoot_leaves;
    let spread = p.short_shoot_spread.to_radians();
    each(tree, envelope, seed, p, |tangent, shoots| {
        reserve(out, shoots.len() * leaves as usize, *p)?;
        for s in shoots {
            let mut rng = s.rng.clone();
            let bearing = level(s.heading)
                .or_else(|| level(tangent))
                .unwrap_or(Vec3::X);
            // The fan turns as a whole by up to one leaf's share of it.
            let turn = rng.range(-1., 1.) * spread / f64::from(leaves);
            for leaf in 0..leaves {
                let across = if leaves > 1 {
                    spread * (2. * f64::from(leaf) / f64::from(leaves - 1) - 1.)
                } else {
                    0.
                };
                let radial = bearing.rotate(Vec3::Y, across + turn);
                let at = s.shoot.tip;
                let lean = axis(at, radial, tangent, *p);
                out.push(&matrix(at, lean, tangent, radial, *p, &mut rng)?);
            }
            if let Some(owners) = owners.as_mut() {
                owners.resize(out.leaves.len(), s.shoot.wood as u32);
            }
        }
        Ok(())
    })
}

/// The horizontal part of a direction, if it has one.
fn level(v: Vec3) -> Option<Vec3> {
    let flat = Vec3::new(v.x, 0., v.z);
    (flat.length_squared() > 1e-12).then(|| flat.normalized())
}

struct Drawn {
    shoot: ShortShoot,
    /// Straight out of the bark, even where the shoot has no length.
    heading: Vec3,
    rng: Rng,
}

/// Visit every eligible piece of wood with its tangent and its short shoots:
/// limb and branch wood no thicker than the row, above the crown base. A
/// shoot stands every `spacing` metres from a phase the wood draws itself,
/// at a bearing and with a cluster its own stream draws.
fn each(
    tree: &Tree,
    envelope: Envelope,
    seed: u32,
    p: &CanopyParams,
    mut visit: impl FnMut(Vec3, &[Drawn]) -> Result<()>,
) -> Result<()> {
    let spacing = p.short_shoot_spacing;
    if spacing == 0. || p.size == 0. || tree.nodes.len() < 2 {
        return Ok(());
    }
    let thickest = tree.stem_radius(|i| tree.nodes[i].radius) * p.short_shoot_radius;
    let floor = envelope.crown_base * envelope.height;
    let mut wood: Vec<usize> = (1..tree.nodes.len())
        .filter(|&i| {
            let n = &tree.nodes[i];
            n.parent.is_some()
                && n.kind != NodeKind::Twig
                && n.radius.max(n.start_radius) <= thickest
        })
        .collect();
    wood.sort_by_key(|&i| tree.nodes[i].identity);
    let mut shoots = Vec::new();
    for i in wood {
        let n = &tree.nodes[i];
        let from = tree.nodes[n.parent.unwrap() as usize].position;
        let length = from.distance(n.position);
        if length <= 1e-9 {
            continue;
        }
        let birth = n.identity.birth_order();
        let phase = Rng::new(key(seed, birth, u32::MAX)).next_f64();
        let count = (length / spacing - phase).ceil().max(0.);
        if !count.is_finite()
            || count > u32::MAX as f64
            || count * p.short_shoot_leaves as f64 > p.max_instances as f64
        {
            return Err(Error::ResourceLimit("foliage instance budget"));
        }
        let tangent = (n.position - from) / length;
        let up = Vec3::Y - tangent * tangent.y;
        let normal = if up.length_squared() > 1e-12 {
            up.normalized()
        } else {
            tangent.perpendicular()
        };
        let binormal = tangent.cross(normal);
        shoots.clear();
        shoots
            .try_reserve(count as usize)
            .map_err(|_| Error::ResourceLimit("foliage allocation"))?;
        for k in 0..count as u32 {
            let t = ((phase + f64::from(k)) * spacing / length).min(1.);
            let mut rng = Rng::new(key(seed, birth, k));
            let (sin, cos) = rng.range(0., TAU).sin_cos_fixed();
            let radial = normal * cos + binormal * sin;
            let wood = n.start_radius + (n.radius - n.start_radius) * t;
            let base = from.lerp(n.position, t) + radial * wood;
            if base.y < floor {
                continue;
            }
            shoots.push(Drawn {
                shoot: ShortShoot {
                    wood: i,
                    base,
                    tip: base + radial * p.short_shoot_length,
                },
                heading: radial,
                rng,
            });
        }
        if !shoots.is_empty() {
            visit(tangent, &shoots)?;
        }
    }
    Ok(())
}

/// SplitMix64's finaliser over the seed, the wood's birth order and the
/// shoot's place along it: the wood's own stream.
fn key(seed: u32, birth: u64, shoot: u32) -> u32 {
    let mut z = (u64::from(seed ^ 0x5f37_59df) << 32 | u64::from(shoot))
        ^ birth.wrapping_mul(0x9e37_79b9_7f4a_7c15);
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    (z ^ (z >> 31)) as u32
}
