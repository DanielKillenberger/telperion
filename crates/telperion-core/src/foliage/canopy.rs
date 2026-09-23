//! The canopy's value table and the twig layer's spacing, shared by the leaf
//! plan, the station preparation and the placement itself. No placement code
//! lives here, so the plan can be built with the placement compiled out.
use super::{range, rosette};
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
    /// Metres between leaves along a shoot, as a share of the tree's
    /// height. Raising it spreads the leaves further apart, so the crown
    /// carries fewer of them.
    pub spacing: f64,
    /// The degrees each successive leaf is turned around its shoot.
    /// Raising it turns the next leaf further round, so the leaves spiral
    /// differently.
    pub divergence: f64,
    /// How many extra leaves are gathered at the end of a shoot that has
    /// no twig layer. Raising it packs a denser tuft at the tip.
    pub clump: u32,
    /// How far back from the tip that tuft is scattered, as a share of the
    /// shoot's length. Raising it spreads the tuft further down the shoot.
    pub clump_span: f64,
    /// How far a leaf turns away from the trunk. Raising it points the
    /// leaves outward, away from the tree's axis.
    pub outward: f64,
    /// How far a leaf turns toward the sky. Raising it tips the leaves up.
    pub upward: f64,
    /// Lean along the shoot, as a fraction of the radial off the wood.
    pub forward_lean: f64,
    /// Further lean along the shoot on radials that face upward.
    pub lean_rise: f64,
    /// The station sits on the shoot axis at 0 and on the wood's own contact
    /// surface at 1; the surface is built whenever it is positive.
    pub surface_contact: f64,
    /// The degrees a leaf may be turned at random from where it was
    /// placed. Raising it leaves the crown less combed.
    pub scatter: f64,
    /// The size every leaf is drawn at, as a multiple of the element's own
    /// dimensions. Raising it enlarges every leaf.
    pub size: f64,
    /// How far leaf size varies leaf to leaf, as a share of that size.
    /// Raising it mixes larger and smaller leaves more widely.
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
    /// How deep a lateral may be and still start a limb system of its own.
    /// Raising it parts the crown into more and smaller leaf masses; it
    /// does nothing until `limb_clumping` is above zero.
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
    /// Fronds the rosette bears at the apex of each stem. At zero no rosette
    /// stands and the canopy clothes wood as it always did; any rise makes the
    /// rosette the tree's only foliage.
    #[cfg_attr(feature = "json", serde(default))]
    pub rosette_fronds: u32,
    /// The degrees each successive frond is turned about the apex.
    #[cfg_attr(
        feature = "json",
        serde(default = "crate::ranges::default_rosette_divergence")
    )]
    pub rosette_divergence: f64,
    /// Degrees from the axis the youngest frond stands: 0 upright, 90 level,
    /// 180 hanging.
    #[cfg_attr(
        feature = "json",
        serde(default = "crate::ranges::default_rosette_pitch")
    )]
    pub rosette_pitch: f64,
    /// How many degrees further than the youngest the oldest frond leans, so
    /// the crown opens from a spike to a skirt.
    #[cfg_attr(
        feature = "json",
        serde(default = "crate::ranges::default_rosette_pitch_spread")
    )]
    pub rosette_pitch_spread: f64,
    /// Metres below the apex the frond insertions are spread down the axis. At
    /// zero every frond leaves one point.
    #[cfg_attr(feature = "json", serde(default))]
    pub rosette_depth: f64,
    /// Leaflets one placement carries along its rachis. One is the single
    /// blade every family drew.
    #[cfg_attr(
        feature = "json",
        serde(default = "crate::ranges::default_leaflet_count")
    )]
    pub leaflet_count: u32,
    /// Metres of rachis the leaflets are strung along. At zero the placement
    /// is one blade whatever the count says.
    #[cfg_attr(feature = "json", serde(default))]
    pub rachis_length: f64,
    /// The degrees a leaflet leaves its rachis.
    #[cfg_attr(
        feature = "json",
        serde(default = "crate::ranges::default_leaflet_pitch")
    )]
    pub leaflet_pitch: f64,
    /// How far the rachis bends out of the straight line from its station, as
    /// a share of its length. Positive arches up, negative droops.
    #[cfg_attr(feature = "json", serde(default))]
    pub rachis_arch: f64,
    /// Whether a single leaflet closes the rachis's end, blended 0 to 1: the
    /// last leaflet turns from standing off the rachis to lying along it.
    #[cfg_attr(feature = "json", serde(default))]
    pub terminal_leaflet: f64,
    /// Bases of shed fronds the stem keeps below its crown, clothing the
    /// trunk. At zero the trunk is bare and the bark is what it always was;
    /// any rise carries the crown's own spiral down it.
    #[cfg_attr(feature = "json", serde(default))]
    pub leaf_bases: u32,
    /// Metres a retained base stands out from the bark. At zero no base is
    /// drawn whatever the count says.
    #[cfg_attr(feature = "json", serde(default))]
    pub leaf_base_length: f64,
    /// How thick a base is where it leaves the bark, as a share of the stem's
    /// own radius there. Raising it leaves a broader boot.
    #[cfg_attr(
        feature = "json",
        serde(default = "crate::ranges::default_leaf_base_radius")
    )]
    pub leaf_base_radius: f64,
    /// Degrees from the stem's axis a base points: 0 flat against the trunk,
    /// 90 square out of it, 180 turned back down.
    #[cfg_attr(
        feature = "json",
        serde(default = "crate::ranges::default_leaf_base_pitch")
    )]
    pub leaf_base_pitch: f64,
    /// How far the lowest and oldest base is worn back against the newest, in
    /// both its length and its girth. At zero every base stands full down the
    /// whole trunk, and any rise wears the foot away.
    #[cfg_attr(feature = "json", serde(default))]
    pub leaf_base_weathering: f64,
    /// Leaflets at a frond's base borne as spines rather than blades. At zero
    /// the frond carries blades all the way down; any rise hardens that many
    /// of them.
    #[cfg_attr(feature = "json", serde(default))]
    pub acanthophylls: u32,
    /// The share of a leaflet's own size a spine is drawn at. At zero no spine
    /// is drawn whatever the count says.
    #[cfg_attr(
        feature = "json",
        serde(default = "crate::ranges::default_acanthophyll_length")
    )]
    pub acanthophyll_length: f64,
    /// The degrees a spine leaves the rachis, in place of the leaflet's own
    /// pitch.
    #[cfg_attr(
        feature = "json",
        serde(default = "crate::ranges::default_acanthophyll_pitch")
    )]
    pub acanthophyll_pitch: f64,
    /// Dead fronds a rosette keeps below its living crown, continuing the
    /// crown's own spiral down the stem. At zero no frond is kept and the
    /// crown ends at its oldest living frond; any rise hangs that many.
    #[cfg_attr(feature = "json", serde(default))]
    pub skirt_fronds: u32,
    /// Degrees from the axis a dead frond hangs: 0 upright, 90 level, 180
    /// collapsed straight down against the stem.
    #[cfg_attr(
        feature = "json",
        serde(default = "crate::ranges::default_skirt_pitch")
    )]
    pub skirt_pitch: f64,
    /// A dead frond's length as a share of a living one's, rachis and
    /// leaflets alike. At zero no dead frond is drawn whatever the count says.
    #[cfg_attr(
        feature = "json",
        serde(default = "crate::ranges::default_skirt_length")
    )]
    pub skirt_length: f64,
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
            // Neutral: no rosette stands and no placement groups until a
            // table states a frond count and a rachis to string leaflets on.
            rosette_fronds: 0,
            rosette_divergence: crate::ranges::default_rosette_divergence(),
            rosette_pitch: crate::ranges::default_rosette_pitch(),
            rosette_pitch_spread: crate::ranges::default_rosette_pitch_spread(),
            rosette_depth: 0.,
            leaflet_count: crate::ranges::default_leaflet_count(),
            rachis_length: 0.,
            leaflet_pitch: crate::ranges::default_leaflet_pitch(),
            rachis_arch: 0.,
            terminal_leaflet: 0.,
            // Neutral: the trunk carries no retained base and no frond bears a
            // spine until a table states a count and the reach to draw it at.
            leaf_bases: 0,
            leaf_base_length: 0.,
            leaf_base_radius: crate::ranges::default_leaf_base_radius(),
            leaf_base_pitch: crate::ranges::default_leaf_base_pitch(),
            leaf_base_weathering: 0.,
            acanthophylls: 0,
            acanthophyll_length: crate::ranges::default_acanthophyll_length(),
            acanthophyll_pitch: crate::ranges::default_acanthophyll_pitch(),
            // Neutral: no dead frond is kept below the living crown.
            skirt_fronds: 0,
            skirt_pitch: crate::ranges::default_skirt_pitch(),
            skirt_length: crate::ranges::default_skirt_length(),
            max_instances: usize::MAX,
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "json", derive(serde::Serialize, serde::Deserialize))]
pub struct TwigPlacement {
    /// Metres between the leaf stations along a twig, copied from the twig
    /// layer's own internode length. Raising it spaces the leaves further
    /// apart along the shoot.
    pub internode_length: f64,
    /// How many leaf stations sit at each of those joints, spread around
    /// the shoot. Raising it crowds more leaves onto each joint.
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
    rows(p, twig)
}

/// Every row on its rail, each refused by its own name, with no tree to read
/// them against.
pub(crate) fn rows(p: CanopyParams, twig: Option<TwigPlacement>) -> Result<()> {
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
    rosette::validate(&p)?;
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
