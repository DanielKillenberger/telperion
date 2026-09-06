//! Named families. Change the seed separately to draw another specimen.
use crate::{
    bias::{BiasParams, SupernaturalParams},
    branching::{BranchHabit, SkeletonParams, SpreadingHabit, TieredHabit},
    envelope::Envelope,
    foliage::{Attachment, CanopyParams, ElementAnatomy, ElementParams},
    radius::RadiusParams,
    surface::SurfaceParams,
};
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Preset {
    Ordinary,
    OregonWhiteOak,
    NorwaySpruce,
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
            surface: SurfaceParams {
                lobe_depth: 0.0,
                twist_rate: 0.0,
                ..Default::default()
            },
            canopy: CanopyParams::default(),
            element: ElementParams::default(),
            shell_depth: 0.45,
        }
    }
}
impl Preset {
    /// Stable research identity; synthetic families have no botanical profile.
    pub fn profile_id(self) -> Option<&'static str> {
        match self {
            Self::OregonWhiteOak => Some("oregon-white-oak"),
            Self::NorwaySpruce => Some("norway-spruce"),
            _ => None,
        }
    }

    /// Native selection is explicit and independent of catalogue order or seed.
    pub fn from_id(id: &str) -> Option<Self> {
        match id {
            "ordinary" => Some(Self::Ordinary),
            "oregon-white-oak" => Some(Self::OregonWhiteOak),
            "norway-spruce" => Some(Self::NorwaySpruce),
            "telperion" => Some(Self::Telperion),
            "laurelin" => Some(Self::Laurelin),
            _ => None,
        }
    }

    pub fn parameters(self) -> Family {
        let mut p = Family::default();
        if self == Self::OregonWhiteOak {
            // Mature, open-grown Quercus garryana. Metre dimensions are
            // calibrated against the frozen profile, not inferred from seed.
            p.skeleton.habit = BranchHabit::Spreading(SpreadingHabit {
                subdivisions: 5,
                ..Default::default()
            });
            p.skeleton.envelope = Envelope {
                height: 24.0,
                crown_base: 0.16,
                spread: 0.55,
                fullness: 0.55,
                shoulder: 2.2,
            };
            p.skeleton.bias = BiasParams::NONE;
            p.skeleton.twigs.laterals = 4;
            p.skeleton.twigs.length_ratio = 0.45;
            p.skeleton.twigs.twig.bearing_diameter = 0.03;
            p.radii.trunk_radius = 0.018;
            p.element = ElementParams {
                anatomy: ElementAnatomy::LobedBlade,
                length: 0.10,
                width: 0.075,
                connector_length: 0.012,
                ..Default::default()
            };
            // Retain interior leaf-bearing shoots in the healthy open-grown crown.
            p.shell_depth = 1.0;
            p.canopy.attachment = Attachment::Alternate;
            p.canopy.divergence = 180.0;
            p.canopy.size_variation = 0.2;
            return p;
        }
        if self == Self::NorwaySpruce {
            // Open-grown landscape Picea abies; one needle per local station.
            p.skeleton.habit = BranchHabit::Tiered(TieredHabit {
                secondary_spacing: 0.20,
                ..Default::default()
            });
            p.skeleton.envelope = Envelope {
                height: 15.0,
                crown_base: 0.04,
                spread: 0.31,
                fullness: 0.15,
                shoulder: 1.0,
            };
            p.skeleton.bias = BiasParams::NONE;
            p.skeleton.twigs.twig.diameter = 0.002;
            p.skeleton.twigs.twig.internode_length = 0.0025;
            p.skeleton.twigs.twig.bearing_diameter = 0.02;
            p.radii.trunk_radius = 0.015;
            p.element = ElementParams {
                anatomy: ElementAnatomy::FourSidedNeedle,
                length: 0.018,
                width: 0.0015,
                connector_length: 0.001,
                ..Default::default()
            };
            p.shell_depth = 1.0;
            // Evergreen foliage also persists on slender supporting branchlets.
            p.canopy.shoot_radius = 0.025;
            p.canopy.attachment = Attachment::RadialNeedles;
            p.canopy.size_variation = 0.2;
            return p;
        }
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
                supernatural: SupernaturalParams {
                    enabled: true,
                    writhe_amplitude: 0.11,
                    writhe_wavelength: 0.34,
                    spiral_rate: 2.6,
                },
            }
        } else {
            BiasParams {
                gravitropism: 0.55,
                lean: 0.06,
                supernatural: SupernaturalParams {
                    enabled: true,
                    writhe_amplitude: 0.05,
                    writhe_wavelength: 0.8,
                    spiral_rate: 0.6,
                },
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
