//! The canopy's value table and the twig layer's spacing, shared by the leaf
//! plan, the station preparation and the placement itself. No placement code
//! lives here, so the plan can be built with the placement compiled out.
use super::range;
use crate::{envelope::Envelope, tree::Tree, Error, Result};

/// The closest two short shoots may stand, in metres, and the furthest: a
/// walk from none thins in from here, where no tree has wood enough for one.
pub const SHORT_SHOOT_SPACING: (f64, f64) = (0.01, 1000.);
/// The most leaves one short shoot's cluster carries.
pub const MAX_SHORT_SHOOT_LEAVES: u32 = 8;

#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "json", derive(serde::Serialize, serde::Deserialize))]
pub struct CanopyParams {
    /// Wood at or below this fraction of the root radius bears foliage of its
    /// own, beside whatever the twig layer marks. Zero leaves the twigs alone
    /// with it; without a twig layer it is what selects the terminal shoots.
    pub shoot_radius: f64,
    pub spacing: f64,
    pub divergence: f64,
    pub clump: u32,
    pub clump_span: f64,
    pub outward: f64,
    pub upward: f64,
    /// Lean along the shoot, as a fraction of the radial off the wood.
    pub forward_lean: f64,
    /// Further lean along the shoot on radials that face upward.
    pub lean_rise: f64,
    /// The station sits on the shoot axis at 0 and on the wood's own contact
    /// surface at 1; the surface is built whenever it is positive.
    pub surface_contact: f64,
    pub scatter: f64,
    pub size: f64,
    pub size_variation: f64,
    /// Metres between short shoots along limb and branch wood: spurs a few
    /// centimetres long, each ending in a cluster of leaves. Zero grows none.
    pub short_shoot_spacing: f64,
    /// Wood thicker than this fraction of the stem's radius carries no short
    /// shoot, and neither does twig wood or anything below the crown base.
    pub short_shoot_radius: f64,
    /// Metres from the bark to the cluster a short shoot carries.
    pub short_shoot_length: f64,
    /// Leaves in one short shoot's cluster, 1 to 8.
    pub short_shoot_leaves: u32,
    /// Degrees either side of its short shoot's bearing a cluster's leaves
    /// fan across, held level: 90 is a half circle, 0 stacks them.
    pub short_shoot_spread: f64,
    /// How far into each limb system the gap between it and its neighbours
    /// reaches, as a share of the way from their shared boundary to the
    /// system's centre: each limb system then keeps a rounded leaf mass of its
    /// own. Zero, the neutral, thins nothing.
    pub limb_clumping: f64,
    #[cfg_attr(
        feature = "json",
        serde(default = "crate::ranges::default_clump_system_order")
    )]
    pub clump_system_order: u32,
    /// Nearest neighbours and cell crossings in the clumping approximation.
    #[cfg_attr(
        feature = "json",
        serde(default = "crate::ranges::default_clump_neighbours")
    )]
    pub clump_neighbours: u32,
    /// Hard total budget. Exceeding it returns an error, never partial foliage.
    #[cfg_attr(feature = "json", serde(with = "crate::specimen::portable::index"))]
    pub max_instances: usize,
}
impl Default for CanopyParams {
    fn default() -> Self {
        Self {
            shoot_radius: 0.,
            spacing: 0.006,
            divergence: 137.508,
            clump: 5,
            clump_span: 0.3,
            outward: 0.6,
            upward: 0.35,
            forward_lean: 0.,
            lean_rise: 0.,
            surface_contact: 0.,
            scatter: 18.,
            size: 1.,
            size_variation: 0.35,
            // Neutral: no short shoot grows until a table states a spacing.
            // The other four are a beech's spur, so a spacing alone reads.
            short_shoot_spacing: 0.,
            short_shoot_radius: 0.15,
            short_shoot_length: 0.04,
            short_shoot_leaves: 3,
            short_shoot_spread: 45.,
            // Neutral: every leaf the stations and the short shoots place.
            limb_clumping: 0.,
            clump_system_order: crate::ranges::default_clump_system_order(),
            clump_neighbours: crate::ranges::default_clump_neighbours(),
            max_instances: usize::MAX,
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "json", derive(serde::Serialize, serde::Deserialize))]
pub struct TwigPlacement {
    pub internode_length: f64,
    pub stations_per_internode: u32,
}
impl Default for TwigPlacement {
    fn default() -> Self {
        Self {
            internode_length: 0.02,
            stations_per_internode: 1,
        }
    }
}
impl TwigPlacement {
    /// The rows a family's own twig table states, resolved: what a caller
    /// clothing that family hands `place`, and what the prediction counts by.
    pub fn of(family: &crate::presets::Family) -> Result<Self> {
        let twig = family.skeleton.twigs.resolved()?.twig;
        Ok(Self {
            internode_length: twig.internode_length,
            stations_per_internode: twig.stations_per_internode,
        })
    }
}

/// Every row on its rail, each refused by its own name, and the tree and
/// shell they are read against.
pub(super) fn validate(
    tree: &Tree,
    envelope: Envelope,
    p: CanopyParams,
    twig: Option<TwigPlacement>,
) -> Result<()> {
    tree.validate_solved()?;
    envelope.validate()?;
    for (v, l, h, n) in [
        (p.shoot_radius, 0., 1., "shoot radius"),
        (p.spacing, 0.001, 1e6, "foliage spacing"),
        (p.divergence, -1e9, 1e9, "divergence"),
        (p.clump_span, 0., 1., "clump span"),
        // Signed: a leaf may lean back down its shoot and turn toward the
        // ground as readily as toward the tip and the sky. Zero is still zero,
        // so every row authored before the rails widened is the row it was.
        (p.outward, -1., 1., "outward"),
        (p.upward, -1., 1., "upward"),
        (p.forward_lean, -1., 1., "forward lean"),
        (p.lean_rise, -2., 2., "lean rise"),
        (p.surface_contact, 0., 1., "surface contact"),
        (p.scatter, 0., 90., "scatter"),
        (p.size, 0., 1000., "foliage size"),
        (p.size_variation, 0., 0.9, "size variation"),
        (p.limb_clumping, 0., 1., "limb clumping"),
    ] {
        range(v, l, h, n)?;
    }
    if p.clump > 64 {
        return Err(Error::InvalidInput("foliage clump"));
    }
    validate_short_shoots(&p)?;
    if let Some(t) = twig {
        range(t.internode_length, 1e-6, 1e6, "twig internode")?;
        if !(1..=64).contains(&t.stations_per_internode) {
            return Err(Error::InvalidInput("twig stations"));
        }
    }
    Ok(())
}

/// The short-shoot rows on their rails.
pub(super) fn validate_short_shoots(p: &CanopyParams) -> Result<()> {
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
