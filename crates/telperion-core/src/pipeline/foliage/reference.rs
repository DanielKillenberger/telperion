//! The box a family's leaves are quantised against.
//!
//! The direct build reads it off the grown tree: the twig layer asks the
//! crown nothing, so the authored shell no longer bounds the wood, and the
//! wood's own extent grown by the station reach does. The growth path keeps
//! its wall and a box read off the parameters alone: `timeline::Placement`
//! caches a shoot's leaves once and shows them again at every later age,
//! while the tree's own bounds grow with it, so words quantised against one
//! age's box would decode against a different one at the next. A box the
//! parameters alone decide is the same box at every age.
use super::packed::Reference;
use crate::{
    envelope::Envelope, math::Vec3, pipeline::foliage::CanopyParams,
    pipeline::radius::RadiusParams, pipeline::surface::SurfaceParams, pipeline::twigs::TwigParams,
    tree::Tree, Result,
};

/// How far a station can stand from the wood axis it sits on, read off a
/// family's rows.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct Reach(f64);

impl Reach {
    /// The reach of this family's resolved twig rows and its other tables.
    pub(crate) fn of(family: &crate::presets::Family) -> Result<Self> {
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
    pub(crate) fn from_params(
        envelope: Envelope,
        twigs: &TwigParams,
        radii: RadiusParams,
        surface: &SurfaceParams,
        canopy: CanopyParams,
    ) -> Self {
        Self(reach(envelope, twigs, radii, surface, canopy))
    }
}

impl Reference {
    /// The box every station of this family stands in at every age of the
    /// growth path, and the one a family is validated against before any
    /// tree exists.
    ///
    /// On the growth path the authored shell bounds the wood: no node stands
    /// above the height, no node stands below the ground, and none stands
    /// further from the axis than the widest lobe of the silhouette. A curtain
    /// hangs into the band below the crown's base, whose own floor is a
    /// clearance above the ground, so the ground is the floor the box has to
    /// hold - and it is the floor at every age, where the authored crown base
    /// is not: a juvenile crown takes its base from its own fraction of the
    /// height. The box is that shell grown by the reach on every side.
    pub(crate) fn of(family: &crate::presets::Family) -> Result<Self> {
        Ok(Self::authored(family.skeleton.envelope, Reach::of(family)?))
    }

    /// The authored box for an envelope and a reach.
    pub(crate) fn authored(envelope: Envelope, reach: Reach) -> Self {
        let reach = reach.0;
        let radius = envelope.max_radius() * (1.0 + envelope.irregularity) + reach;
        Self::spanning(
            Vec3::new(-radius, -reach, -radius),
            Vec3::new(radius, envelope.height + reach, radius),
        )
    }

    /// The box the direct build quantises against: the grown tree's wood
    /// extent, every node's position, grown by the reach on every side.
    pub(crate) fn grown(tree: &Tree, reach: Reach) -> Self {
        let first = tree.nodes.first().map_or(Vec3::ZERO, |n| n.position);
        let (lo, hi) = tree.nodes.iter().fold((first, first), |(lo, hi), n| {
            let p = n.position;
            (
                Vec3::new(lo.x.min(p.x), lo.y.min(p.y), lo.z.min(p.z)),
                Vec3::new(hi.x.max(p.x), hi.y.max(p.y), hi.z.max(p.z)),
            )
        });
        let out = Vec3::new(reach.0, reach.0, reach.0);
        Self::spanning(lo - out, hi + out)
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
    // A rachis carries its leaflets a length further out from the station,
    // arched out of that line again, and a rosette spreads its insertions,
    // and its skirt's, down the axis below the apex. Both are off at the neutral rows, so the
    // box every shipped preset quantises against is the box it was.
    let rachis = if canopy.leaflet_count > 1 && canopy.rachis_length > 0.0 {
        canopy.rachis_length * (1.0 + canopy.rachis_arch.abs())
    } else {
        0.0
    };
    let apex = if canopy.rosette_fronds > 0 {
        super::rosette::deepest(&canopy)
    } else {
        0.0
    };
    (seated.max(spur).max(apex) + rachis).max(0.0)
}
