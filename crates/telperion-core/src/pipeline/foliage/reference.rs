//! The box a family's leaves are quantised against.
//!
//! The direct build reads it off the grown tree: the twig layer asks the
//! crown nothing, so the authored shell no longer bounds the wood, and the
//! wood's own extent grown by the station reach does. Validation, which grows
//! no tree, reads a box off the parameters alone.
use super::packed::Reference;
use crate::{
    envelope::Envelope, math::Vec3, pipeline::foliage::CanopyParams,
    pipeline::radius::RadiusParams, pipeline::surface::SurfaceParams, pipeline::twigs::TwigParams,
    tree::Tree, Result,
};

/// How far a station can stand from the wood axis it sits on, read off a
/// family's rows: out to the skin of the wood it is seated on, then on along
/// down a rosette or out along a rachis.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct Reach {
    /// The reach from the thickest wood the rows let a station sit on.
    authored: f64,
    /// Whether a station is seated on the swept skin rather than the axis.
    seats: bool,
    /// The swept skin's fork swell, lobe depth and basal flare.
    skin: [f64; 3],
    /// How far a rosette carries a station past its wood.
    beyond: f64,
    /// How far a rachis carries its leaflets past the station.
    rachis: f64,
}

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
        reach(envelope, twigs, radii, surface, canopy)
    }

    /// How far a station seated on wood of radius `wood` stands from its
    /// axis. The product is associated as it always was: the box quantises
    /// every leaf, and one ulp moves every committed digest.
    fn from_wood(self, wood: f64) -> f64 {
        let [swell, lobe, flare] = self.skin;
        let seated = if self.seats {
            wood * swell * (1.0 + lobe) * flare
        } else {
            wood
        };
        (seated.max(self.beyond) + self.rachis).max(0.0)
    }

    /// How far the box reaches out from a node of wood radius `wood`.
    pub(crate) fn out(self, wood: f64) -> f64 {
        self.from_wood(wood).max(self.authored)
    }
}

impl Reference {
    /// The box a family is validated against before any tree exists: the
    /// authored shell, from the ground to the height and out to the widest
    /// lobe of the silhouette, grown by the reach on every side.
    pub(crate) fn of(family: &crate::presets::Family) -> Result<Self> {
        Ok(Self::authored(family.skeleton.envelope, Reach::of(family)?))
    }

    /// The authored box for an envelope and a reach.
    pub(crate) fn authored(envelope: Envelope, reach: Reach) -> Self {
        let reach = reach.authored;
        let radius = envelope.max_radius() * (1.0 + envelope.irregularity) + reach;
        Self::spanning(
            Vec3::new(-radius, -reach, -radius),
            Vec3::new(radius, envelope.height + reach, radius),
        )
    }

    /// The box the direct build quantises against: the grown tree's wood,
    /// every node out by the reach of a station seated on that node's own
    /// widest radius, and never less than the reach the rows state.
    pub(crate) fn grown(tree: &Tree, reach: Reach) -> Self {
        let (lo, hi) = reach.wood_box(tree, |_| true);
        Self::spanning(lo, hi)
    }
}

impl Reach {
    /// The corners of the box of the nodes of `tree` that `keep` keeps,
    /// each out by the reach of a station seated on its widest radius, and
    /// the root's position: boxes of parts of a tree combine by their
    /// corners, exactly (fn-212 task 4).
    pub(crate) fn wood_box(self, tree: &Tree, keep: impl Fn(usize) -> bool) -> (Vec3, Vec3) {
        let first = tree.nodes.first().map_or(Vec3::ZERO, |n| n.position);
        let kept = tree.nodes.iter().enumerate().filter(|&(i, _)| keep(i));
        kept.fold((first, first), |(lo, hi), (_, n)| {
            let wood = n.radius.max(n.start_radius).max(n.base_radius);
            let out = self.out(wood);
            let (a, b) = (
                n.position - Vec3::new(out, out, out),
                n.position + Vec3::new(out, out, out),
            );
            (
                Vec3::new(lo.x.min(a.x), lo.y.min(a.y), lo.z.min(a.z)),
                Vec3::new(hi.x.max(b.x), hi.y.max(b.y), hi.z.max(b.z)),
            )
        })
    }

    /// The terms of a station's reach from wood of radius `w`, for an
    /// executor that folds its own nodes' box: `max(max(w * scale, beyond)
    /// + rachis, 0)`, never under `authored`; `[scale, beyond, rachis,
    /// authored]`.
    pub(crate) fn terms(self) -> [f64; 4] {
        let [swell, lobe, flare] = self.skin;
        let scale = if self.seats {
            swell * (1.0 + lobe) * flare
        } else {
            1.0
        };
        [scale, self.beyond, self.rachis, self.authored]
    }
}

/// How far outside the authored shell a station can stand.
fn reach(
    envelope: Envelope,
    twigs: &TwigParams,
    radii: RadiusParams,
    surface: &SurfaceParams,
    canopy: CanopyParams,
) -> Reach {
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
    // A rachis carries its leaflets a length further out from the station,
    // arched out of that line again, and a rosette spreads its insertions,
    // and its skirt's, down the axis below the apex. Both are off at the neutral rows, so the
    // box every shipped preset quantises against is the box it was.
    let rachis = if canopy.leaflet_count > 1.0 && canopy.rachis_length > 0.0 {
        canopy.rachis_length * (1.0 + canopy.rachis_arch.abs())
    } else {
        0.0
    };
    let apex = if canopy.rosette_fronds > 0.0 {
        super::rosette::deepest(&canopy)
    } else {
        0.0
    };
    // Seating a station on the swept surface walks it out along the same
    // radial to wherever the sweep's own lobes, fork swell and basal flare put
    // that wood's skin.
    let mut reach = Reach {
        authored: 0.0,
        seats: canopy.surface_contact > 0.0,
        skin: [surface.fork_swell, surface.lobe_depth, surface.flare_radius],
        beyond: apex,
        rachis,
    };
    reach.authored = reach.from_wood(wood);
    reach
}
