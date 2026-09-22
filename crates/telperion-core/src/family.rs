//! Engine-neutral generator parameters. Named preset tables construct this
//! value; generators and accelerated consumers operate on the value itself.
use crate::{
    branching::SkeletonParams,
    foliage::{CanopyParams, ElementParams},
    material::MaterialParams,
    radius::RadiusParams,
    surface::SurfaceParams,
};

#[derive(Debug, Clone)]
pub struct Family {
    /// The specimen's age in years. Only the growth path reads it: the
    /// direct build is the mature tree whatever the row says.
    pub age: f64,
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
    pub shell_depth: f64,
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
