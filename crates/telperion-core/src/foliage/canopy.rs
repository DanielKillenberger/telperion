//! The canopy's value table and the twig layer's spacing, shared by the leaf
//! plan, the station preparation and the placement itself. No placement code
//! lives here, so the plan can be built with the placement compiled out.
mod params;
use super::{range, rosette};
use crate::{catalogue::Site, envelope::Envelope, tree::Tree, Error, Result};
pub use params::CanopyParams;

/// The closest two short shoots may stand, in metres, and the furthest: a
/// walk from none thins in from here, where no tree has wood enough for one.
pub const SHORT_SHOOT_SPACING: (f64, f64) = (0.01, 1000.);
/// The most leaves one short shoot's cluster carries.
pub const MAX_SHORT_SHOOT_LEAVES: u32 = 8;

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
    crate::catalogue::check(CanopyParams::CHECKS, &p, Site::Canopy)?;
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
    crate::catalogue::check(CanopyParams::CHECKS, p, Site::ShortShoots)
}
