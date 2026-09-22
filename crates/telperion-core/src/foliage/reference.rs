//! The box a family's leaves are quantised against, read off its parameters.
//!
//! Not off the tree. `timeline::Placement` caches a shoot's leaves once and
//! shows them again at every later age, while the tree's own bounds grow with
//! it: words quantised against one age's box would decode against a different
//! one at the next, and every cached leaf would need requantising whenever the
//! box moved. A box the parameters alone decide is the same box at every age,
//! so a leaf written at one age reads correctly at all of them.
use super::packed::Reference;
use crate::{
    envelope::Envelope, foliage::CanopyParams, math::Vec3, radius::RadiusParams,
    surface::SurfaceParams, twigs::TwigParams, Result,
};

impl Reference {
    /// The box every station of this family stands in, at every age.
    ///
    /// The authored shell bounds the wood: no node stands above the height, no
    /// node stands below the ground, and none stands further from the axis
    /// than the widest lobe of the silhouette. A curtain hangs into the band
    /// below the crown's base, whose own floor is a clearance above the
    /// ground, so the ground is the floor the box has to hold - and it is the
    /// floor at every age, where the authored crown base is not: a juvenile
    /// crown takes its base from its own fraction of the height.
    ///
    /// A station then stands off the wood it sits on by that wood's radius,
    /// carried out to the swept surface where the row seats it there and out
    /// again along a short shoot where the row grows one. `reach` is the
    /// furthest any of those three carries it, and the box is the shell grown
    /// by it on every side.
    pub fn of(family: &crate::presets::Family) -> Result<Self> {
        let twigs = family.skeleton.twigs.resolved()?;
        Ok(Self::from_params(
            family.skeleton.envelope,
            &twigs,
            family.radii,
            &family.surface,
            family.canopy,
        ))
    }

    /// `of` for a caller holding the five tables rather than the family. The
    /// twig rows must already be resolved.
    pub fn from_params(
        envelope: Envelope,
        twigs: &TwigParams,
        radii: RadiusParams,
        surface: &SurfaceParams,
        canopy: CanopyParams,
    ) -> Self {
        let reach = reach(envelope, twigs, radii, surface, canopy);
        let radius = envelope.max_radius() * (1.0 + envelope.irregularity) + reach;
        Self::spanning(
            Vec3::new(-radius, -reach, -radius),
            Vec3::new(radius, envelope.height + reach, radius),
        )
    }
}

/// How far outside the authored shell a station can stand.
fn reach(
    envelope: Envelope,
    twigs: &TwigParams,
    radii: RadiusParams,
    surface: &SurfaceParams,
    canopy: CanopyParams,
) -> f64 {
    // The root is the thickest node there is: the pipe model normalises it to
    // exactly this and every node below it carries less.
    let trunk = radii.trunk_radius.max(4e-6) * envelope.height.max(0.0);
    // The thickest wood a station sits on. With a twig layer the stations
    // belong to twig wood, whose diameter is a row of its own; where the
    // canopy also clothes slender wood, or clothes terminal runs without a
    // twig layer, the run reaches back to its attachment and that is trunk
    // wood at worst.
    let wood = if canopy.shoot_radius > 0.0 {
        trunk
    } else {
        twigs.twig.diameter / 2.0
    };
    // Seating a station on the swept surface walks it out along the same
    // radial to wherever the sweep's own lobes, fork swell and basal flare put
    // that wood's skin. The product is associated as it always was: the box
    // quantises every leaf, and one ulp moves every committed digest.
    let seated = if canopy.surface_contact > 0.0 {
        wood * surface.fork_swell * (1.0 + surface.lobe_depth) * surface.flare_radius
    } else {
        wood
    };
    // A short shoot stands on limb wood up to its own share of the stem and
    // carries its cluster a spur's length further out again.
    let spur = if canopy.short_shoot_spacing > 0.0 {
        canopy.short_shoot_radius * trunk + canopy.short_shoot_length
    } else {
        0.0
    };
    seated.max(spur).max(0.0)
}
