//! Engine-neutral generator parameters. Named preset tables construct this
//! value; generators and accelerated consumers operate on the value itself.
use crate::catalogue::{bounded, input, Blend, Bounds, Dial, Growth, Site};
use crate::{
    branching::SkeletonParams,
    foliage::{CanopyParams, ElementParams},
    material::MaterialParams,
    radius::RadiusParams,
    surface::SurfaceParams,
    Error, Result,
};

crate::catalogue::rows! {
    #[derive(Debug, Clone)]
    pub struct Family in "" {
        /// The specimen's age in years. Only the growth path reads it: the
        /// direct build is the mature tree whatever the row says.
        pub age: f64 = "age" "years" Bounds::closed(0.0, 1_000_000.0) => [Grow] {
            growth: Growth::Only,
            note: "Refused by `Age::from_years` in `params::parse` and `Family::validate`, which \
                also quantises it to the growth path's tick; the direct build is the mature tree \
                whatever it says.",
            blend: Blend::Weighted,
            dial: Dial::Excluded("Identity of a grown specimen, not a look: only the growth path \
                reads it, and the direct build the tuner measures is the mature tree whatever it \
                says (crates/telperion-core/src/branching/specimen/timeline.rs:30)."),
        },
        pub growth: crate::growth::GrowthTraits,
        pub skeleton: SkeletonParams,
        pub radii: RadiusParams,
        pub surface: SurfaceParams,
        pub canopy: CanopyParams,
        pub element: ElementParams,
        pub material: MaterialParams,
        /// How deep into the crown leaves are kept, as a share of the crown's
        /// widest radius; a leaf further in than that is dropped. Raising it
        /// keeps more of the crown's interior foliage, and one keeps it all.
        pub shell_depth: f64 = "shellDepth" "share of the crown's widest radius"
            Bounds::closed(0.0, 1.0) => [Cull] {
            check: input(Site::Shell, 0, "shell depth"),
            growth: Growth::Differs("measured against the envelope at the specimen's age"),
            note: "`foliage::cull` and the GPU executor's preparation check it again; the \
                planned field counts stations before the cull and does not read it.",
            dial: bounded("shell_depth", "how deep into the crown leaves are kept, as a share of \
                its widest radius", [0.15, 0.3]),
        },
    }
}
impl Default for Family {
    fn default() -> Self {
        Self {
            age: 100.0,
            growth: crate::growth::GrowthTraits::default(),
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
            shell_depth: 0.45,
        }
    }
}
impl Family {
    /// Every row judged as the build and the growth path judge it, each
    /// refused by the name they refuse it by, with no attractor scattered, no
    /// node grown and no leaf placed. The checks run in the order the build
    /// reaches them. A scatter that falls short of its count is left to
    /// growth: that depends on the seed as much as on any row.
    pub fn validate(&self) -> Result<()> {
        use crate::foliage::{self, Instances, Reference, TwigPlacement};
        self.material.validate()?;
        crate::growth::Age::from_years(self.age)?;
        self.growth.validate()?;
        crate::branching::validate_skeleton(&self.skeleton, self.radii)?;
        // Growth keeps a zero ceiling as an empty seedling it can resume
        // from; the build has nothing to sweep, so a family refuses it here.
        if self.skeleton.growth.max_nodes == Some(0) {
            return Err(Error::InvalidInput("maxNodes"));
        }
        self.surface.validate()?;
        crate::surface::height(self.skeleton.envelope.height)?;
        foliage::build_element(self.element)?;
        foliage::canopy_rows(self.canopy, Some(TwigPlacement::of(self)?))?;
        Instances::new(Reference::of(self)?).validate()?;
        crate::catalogue::check(Self::ROWS, self, Site::Shell)
    }
}

#[cfg(all(test, feature = "json", feature = "geometry"))]
mod tests;
