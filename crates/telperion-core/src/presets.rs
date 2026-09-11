//! Named families. Change the seed separately to draw another specimen.
use crate::{
    bias::{BiasParams, SupernaturalParams},
    branching::{HabitParams, SkeletonParams},
    envelope::Envelope,
    foliage::{CanopyParams, ElementParams},
    material::MaterialParams,
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
    pub material: MaterialParams,
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
            material: MaterialParams::default(),
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
            p.skeleton.habit = HabitParams {
                apical_dominance: 0.1,
                whorl_strength: 0.1,
                leader_internode: 2.0,
                laterals_per_station: 5,
                lateral_pitch: 55.0,
                pitch_variation: 20.0,
                rise_primary: 0.12,
                rise_secondary: 0.0,
                crookedness: 24.0,
                lateral_spacing: 1.6,
                lateral_length_ratio: 0.45,
                lateral_orders: 3,
                attractor_weight: 0.0,
                twig_tip_taper: 0.25,
                shedding_threshold: 0.0,
            };
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
            // Five lobes on an envelope whose crests sit where the retired
            // five-lobe table reached; the sinuses cut deeper than that table
            // did, which is the shape a Quercus garryana leaf actually holds.
            p.element = ElementParams {
                length: 0.10,
                width: 0.075,
                connector_length: 0.012,
                widest_at: 0.55,
                base_fullness: 0.6,
                tip_sharpness: 0.6,
                lobe_count: 5,
                lobe_depth: 0.7,
                section_roundness: 0.0,
                // Four stations to a half-lobe, landing exactly on every crest
                // and every sinus: fewer and the margin is drawn as the zigzag
                // between them rather than as the curve through them.
                axial_segments: 40,
                ..Default::default()
            };
            // Retain interior leaf-bearing shoots in the healthy open-grown crown.
            p.shell_depth = 1.0;
            // Blades alternate along the shoot and lean a quarter of the
            // radial toward its tip; nothing pulls them outward or up.
            p.canopy.forward_lean = 0.25;
            p.canopy.outward = 0.0;
            p.canopy.upward = 0.0;
            p.canopy.divergence = 180.0;
            p.canopy.size_variation = 0.2;
            // Pale grey-brown furrowed bark; a dark glossy blade over a
            // markedly paler underside. Linear, from the frozen profile's
            // prose, and calibrated against the photographs in fn-14.6.
            p.material = MaterialParams {
                bark_red: 0.254,
                bark_green: 0.220,
                bark_blue: 0.178,
                bark_roughness: 0.85,
                leaf_front_red: 0.028,
                leaf_front_green: 0.102,
                leaf_front_blue: 0.016,
                leaf_back_red: 0.153,
                leaf_back_green: 0.254,
                leaf_back_blue: 0.112,
                hue_range_low: -0.03,
                hue_range_high: 0.03,
                brightness_range_low: -0.15,
                brightness_range_high: 0.15,
                interior_darkening: 0.55,
            };
            return p;
        }
        if self == Self::NorwaySpruce {
            // Open-grown landscape Picea abies; one needle per local station.
            p.skeleton.habit = HabitParams {
                apical_dominance: 1.0,
                whorl_strength: 1.0,
                leader_internode: 0.9,
                laterals_per_station: 5,
                lateral_pitch: 88.0,
                pitch_variation: 4.0,
                rise_primary: 0.12,
                rise_secondary: -0.8,
                crookedness: 0.0,
                lateral_spacing: 0.15,
                lateral_length_ratio: 0.30,
                lateral_orders: 4,
                attractor_weight: 0.0,
                twig_tip_taper: 0.25,
                shedding_threshold: 0.0,
            };
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
            // A shaft that holds its width to the distal point, rolled all
            // the way round: four sides, four cross segments, no seam vertex
            // spent on a seam that is not there.
            p.element = ElementParams {
                length: 0.018,
                width: 0.0015,
                connector_length: 0.001,
                widest_at: 0.2,
                base_fullness: 0.2,
                tip_sharpness: 0.2,
                lobe_count: 0,
                lobe_depth: 0.0,
                section_roundness: 1.0,
                cross_segments: 4,
                ..Default::default()
            };
            p.shell_depth = 1.0;
            // Evergreen foliage also persists on slender supporting branchlets,
            // seated on the wood itself, upper needles leaning toward the tip.
            p.canopy.shoot_radius = 0.025;
            p.canopy.forward_lean = 0.05;
            p.canopy.lean_rise = 1.2;
            p.canopy.surface_contact = 1.0;
            p.canopy.outward = 0.0;
            p.canopy.upward = 0.0;
            p.canopy.size_variation = 0.2;
            // Reddish-brown scaly bark; a needle darker and bluer than any
            // blade, its underside paler where the stomatal bands run.
            p.material = MaterialParams {
                bark_red: 0.147,
                bark_green: 0.078,
                bark_blue: 0.045,
                bark_roughness: 0.9,
                leaf_front_red: 0.018,
                leaf_front_green: 0.056,
                leaf_front_blue: 0.028,
                leaf_back_red: 0.109,
                leaf_back_green: 0.195,
                leaf_back_blue: 0.138,
                hue_range_low: -0.02,
                hue_range_high: 0.02,
                brightness_range_low: -0.10,
                brightness_range_high: 0.10,
                interior_darkening: 0.7,
            };
            return p;
        }
        if self == Self::Ordinary {
            return p;
        }
        let silver = self == Self::Telperion;
        p.skeleton.seed = if silver { 1 } else { 2 };
        // The Two Trees are an order of magnitude taller than a forest tree,
        // and every spacing here is a length in metres.
        p.skeleton.habit = HabitParams {
            apical_dominance: 0.15,
            whorl_strength: 0.2,
            leader_internode: 10.0,
            laterals_per_station: 4,
            lateral_pitch: 60.0,
            pitch_variation: 15.0,
            rise_primary: 0.05,
            rise_secondary: 0.0,
            crookedness: 12.0,
            lateral_spacing: 10.0,
            lateral_length_ratio: 0.45,
            lateral_orders: 3,
            attractor_weight: 1.0,
            twig_tip_taper: 1.0,
            shedding_threshold: 0.45,
        };
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
