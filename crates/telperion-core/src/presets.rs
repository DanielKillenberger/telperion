//! Named families. Change the seed separately to draw another specimen.
use crate::{
    bias::BiasParams,
    branching::SkeletonParams,
    envelope::Envelope,
    foliage::{CanopyParams, ElementParams},
    radius::RadiusParams,
    surface::SurfaceParams,
};
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Preset {
    Ordinary,
    Telperion,
    Laurelin,
}
#[derive(Debug, Clone)]
pub struct Family {
    pub skeleton: SkeletonParams,
    pub radii: RadiusParams,
    pub surface: SurfaceParams,
    pub canopy: CanopyParams,
    pub element: ElementParams,
    pub shell_depth: f64,
}
impl Default for Family {
    fn default() -> Self {
        Self {
            skeleton: SkeletonParams::default(),
            radii: RadiusParams::default(),
            surface: SurfaceParams::default(),
            canopy: CanopyParams::default(),
            element: ElementParams::default(),
            shell_depth: 0.45,
        }
    }
}
impl Preset {
    pub fn parameters(self) -> Family {
        let mut p = Family::default();
        if self == Self::Ordinary {
            return p;
        }
        let silver = self == Self::Telperion;
        p.skeleton.seed = if silver { 1 } else { 2 };
        p.skeleton.envelope = if silver {
            Envelope {
                height: 148.0,
                crown_base: 0.38,
                spread: 0.24,
                fullness: 0.58,
                shoulder: 1.5,
            }
        } else {
            Envelope {
                height: 132.0,
                crown_base: 0.24,
                spread: 0.58,
                fullness: 0.38,
                shoulder: 3.2,
            }
        };
        p.skeleton.attractors = if silver { 1600 } else { 1060 };
        p.skeleton.bias = if silver {
            BiasParams {
                gravitropism: 0.95,
                lean: 0.04,
                writhe_amplitude: 0.11,
                writhe_wavelength: 0.34,
                spiral_rate: 2.6,
            }
        } else {
            BiasParams {
                gravitropism: 0.55,
                lean: 0.06,
                writhe_amplitude: 0.05,
                writhe_wavelength: 0.8,
                spiral_rate: 0.6,
            }
        };
        p.skeleton.twigs.twig.length = 0.5;
        p.skeleton.twigs.internode_factor = if silver { 3.5 } else { 6.0 };
        p.skeleton.twigs.laterals = if silver { 4 } else { 2 };
        p.skeleton.growth.max_turn_per_step = Some(if silver { 26.0 } else { 46.0 });
        p.radii = if silver {
            RadiusParams {
                trunk_radius: 0.05,
                fork_exponent: 2.15,
                length_taper: 0.75,
            }
        } else {
            RadiusParams {
                trunk_radius: 0.055,
                fork_exponent: 2.7,
                length_taper: 0.35,
            }
        };
        p.surface = SurfaceParams {
            radial_segments: 12,
            lobes: if silver { 7 } else { 4 },
            lobe_depth: if silver { 0.11 } else { 0.24 },
            twist_rate: if silver { 2.4 } else { 0.8 },
            flare_radius: if silver { 2.0 } else { 3.1 },
            flare_falloff: 0.022,
            flare_depth: 0.004,
            fork_socket: 0.5,
            fork_swell: 1.35,
        };
        p.canopy = CanopyParams {
            shoot_radius: if silver { 0.1 } else { 0.14 },
            spacing: if silver { 0.0045 } else { 0.0065 },
            divergence: if silver { 137.508 } else { 99.502 },
            clump: if silver { 6 } else { 9 },
            clump_span: if silver { 0.28 } else { 0.36 },
            outward: if silver { 0.42 } else { 0.72 },
            upward: if silver { 0.55 } else { 0.22 },
            scatter: if silver { 14.0 } else { 22.0 },
            size: if silver { 1.0 } else { 1.5 },
            size_variation: if silver { 0.28 } else { 0.4 },
            ..Default::default()
        };
        p
    }
}
