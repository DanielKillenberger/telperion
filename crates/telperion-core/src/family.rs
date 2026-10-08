//! Engine-neutral generator parameters. Named preset tables construct this
//! value; generators and accelerated consumers operate on the value itself.
use crate::catalogue::{bounded, input, value, Blend, Bounds, Dial, Growth, Site};
use crate::{
    branching::SkeletonParams,
    foliage::{BloomParams, CanopyParams, ElementParams},
    material::MaterialParams,
    radius::RadiusParams,
    surface::SurfaceParams,
    Error, Result,
};

crate::catalogue::rows! {
    #[derive(Debug, Clone)]
    pub struct Family in "" {
        /// The specimen's age in years: the engine grows the family's species
        /// for this many years, so the tree is the one it is at that age
        /// (fn-198). A whole number of years.
        pub age: f64 = "age" "years" Bounds::closed(0.0, 1_000_000.0) => [Grow] {
            wire: 0,
            check: value(Site::Age, 0, "age"),
            growth: Growth::Same,
            note: "Refused off its range in `params::parse` and `Family::validate`, and refused \
                by name at the skeleton where it is not a whole number of years: the engine \
                grows a year a cycle. The harness opens an engine preset at 40 (fn-198 host \
                decision 5).",
            blend: Blend::Count,
            dial: Dial::Excluded("The age the tree is drawn at, which the viewer sets; the \
                tuner judges a species at its ages, never by stepping one."),
        },
        pub skeleton: SkeletonParams,
        pub radii: RadiusParams,
        pub surface: SurfaceParams,
        pub canopy: CanopyParams,
        pub element: ElementParams,
        pub material: MaterialParams,
        /// What flowers where the engine's ages flower (fn-216).
        pub bloom: BloomParams,
        /// The tree-space engine's species, the ages that are stem and the
        /// site's light: the indexed `/space` rows (`presets/space.rs`).
        pub space: crate::presets::space::Space,
        /// How deep into the crown leaves were kept, as a share of the crown's
        /// widest radius. Checked and read by nothing past the check: the
        /// cull it set kept every leaf on every engine preset and was removed
        /// (fn-198 R1, fn-212 R2); whether the row goes is the owner's call.
        pub shell_depth: f64 = "shellDepth" "share of the crown's widest radius"
            Bounds::closed(0.0, 1.0) => [Cull] {
            wire: 236,
            check: input(Site::Shell, 0, "shell depth"),
            growth: Growth::Differs("measured against the envelope at the specimen's age"),
            note: "The leaves stage checks it again; no cull reads it since fn-212 R2, and \
                the planned field does not read it.",
            dial: bounded("shell_depth", "how deep into the crown leaves are kept, as a share of \
                its widest radius", [0.15, 0.3]),
        },
    }
}
impl Default for Family {
    fn default() -> Self {
        Self {
            age: 100.0,
            skeleton: SkeletonParams::default(),
            radii: RadiusParams::default(),
            surface: SurfaceParams {
                lobe_depth: 0.0,
                twist_rate: 0.0,
                ..Default::default()
            },
            canopy: CanopyParams::default(),
            element: ElementParams::default(),
            material: MaterialParams::default(),
            bloom: BloomParams::default(),
            space: Default::default(),
            shell_depth: 0.45,
        }
    }
}
impl Family {
    /// Every row judged as the build judges it, each refused by the name it
    /// refuses it by, with no attractor scattered, no node grown and no leaf
    /// placed. The checks run in the order the build reaches them. A scatter
    /// that falls short of its count is left to growth: that depends on the
    /// seed as much as on any row.
    pub fn validate(&self) -> Result<()> {
        use crate::foliage::{Instances, Reference, TwigPlacement};
        use crate::pipeline::{build_element, canopy_rows, height, validate_skeleton};
        self.material.validate()?;
        crate::catalogue::check(Self::CHECKS, self, Site::Age)?;
        // The engine species, as the engine judges it before it grows.
        if let Some(species) = &self.space.species {
            species.validate().map_err(|e| Error::InvalidValue {
                field: "engine",
                value: e.to_string(),
            })?;
        }
        validate_skeleton(&self.skeleton, self.radii)?;
        // A zero ceiling grows nothing; the build has nothing to sweep, so a
        // family refuses it here.
        if self.skeleton.growth.max_nodes == Some(0) {
            return Err(Error::InvalidInput("maxNodes"));
        }
        self.surface.validate()?;
        height(self.skeleton.envelope.height)?;
        build_element(self.element)?;
        crate::catalogue::check(BloomParams::CHECKS, &self.bloom, Site::Bloom)?;
        canopy_rows(self.canopy, Some(TwigPlacement::of(self)?))?;
        Instances::new(Reference::of(self)?).validate()?;
        crate::catalogue::check(Self::CHECKS, self, Site::Shell)
    }
}

#[cfg(all(test, feature = "json", feature = "geometry"))]
mod tests;
