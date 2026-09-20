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
    pub age: f64,
    pub growth: crate::growth::GrowthTraits,
    pub skeleton: SkeletonParams,
    pub radii: RadiusParams,
    pub surface: SurfaceParams,
    pub canopy: CanopyParams,
    pub element: ElementParams,
    pub material: MaterialParams,
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
