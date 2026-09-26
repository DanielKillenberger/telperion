//! Named families. Change the seed separately to draw another specimen.
mod materials;
mod originals;
mod species;
use crate::{
    bias::{BiasParams, SupernaturalParams},
    branching::HabitParams,
    envelope::Envelope,
    foliage::CanopyParams,
    radius::RadiusParams,
    surface::SurfaceParams,
};
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Preset {
    Ordinary,
    OregonWhiteOak,
    NorwaySpruce,
    EuropeanBeech,
    SilverBirch,
    DatePalm,
    Telperion,
    Laurelin,
}
pub use crate::Family;
use crate::{Error, Result};

// Numeric ABI IDs remain stable for existing callers; catalogue order is irrelevant.
pub const CATALOGUE: &[(u32, &str, &str, &str)] = &[
    (0, "ordinary", "Ordinary", "Natural baseline"),
    (
        3,
        "oregon-white-oak",
        "Oregon white oak",
        "Quercus garryana",
    ),
    (4, "norway-spruce", "Norway spruce", "Picea abies"),
    (6, "silver-birch", "Silver birch", "Betula pendula"),
    (7, "date-palm", "Date palm", "Phoenix dactylifera"),
    (1, "telperion", "Telperion", "The silver tree"),
    (2, "laurelin", "Laurelin", "The golden tree"),
];
/// Tables still being judged. Their ABI ids are reserved, and they are not
/// listed, served by id or built by name: the core's tests and the species
/// runner reach them through `Preset`. The European beech ships when fn-62
/// accepts it. The date palm shipped when the owner accepted it (fn-82).
pub const IN_WORK: &[(u32, &str, &str, &str)] =
    &[(5, "european-beech", "European beech", "Fagus sylvatica")];
/// The family a shipped identity serves: the catalogue's tables, with the
/// growth default an unset turn limit falls back to. Every binding resolves
/// a species id through here.
pub fn by_identity(id: &str) -> Result<Family> {
    if !CATALOGUE.iter().any(|entry| entry.1 == id) {
        return Err(Error::InvalidInput("preset identity"));
    }
    let mut f = Preset::from_id(id)
        .ok_or(Error::InvalidInput("preset identity"))?
        .parameters();
    f.skeleton
        .growth
        .max_turn_per_step
        .get_or_insert(crate::colonization::GrowthConfig::default().max_turn_per_step);
    Ok(f)
}

impl Preset {
    /// Stable research identity; synthetic families have no botanical profile.
    pub fn profile_id(self) -> Option<&'static str> {
        match self {
            Self::OregonWhiteOak => Some("oregon-white-oak"),
            Self::NorwaySpruce => Some("norway-spruce"),
            Self::EuropeanBeech => Some("european-beech"),
            Self::SilverBirch => Some("silver-birch"),
            Self::DatePalm => Some("date-palm"),
            _ => None,
        }
    }

    /// Native selection is explicit and independent of catalogue order or seed.
    pub fn from_id(id: &str) -> Option<Self> {
        match id {
            "ordinary" => Some(Self::Ordinary),
            "oregon-white-oak" => Some(Self::OregonWhiteOak),
            "norway-spruce" => Some(Self::NorwaySpruce),
            "european-beech" => Some(Self::EuropeanBeech),
            "silver-birch" => Some(Self::SilverBirch),
            "date-palm" => Some(Self::DatePalm),
            "telperion" => Some(Self::Telperion),
            "laurelin" => Some(Self::Laurelin),
            _ => None,
        }
    }

    pub fn parameters(self) -> Family {
        let mut p = Family::default();
        if self == Self::OregonWhiteOak {
            originals::oregon_white_oak(&mut p);
            return p;
        }
        if self == Self::NorwaySpruce {
            originals::norway_spruce(&mut p);
            return p;
        }
        if self == Self::EuropeanBeech {
            species::european_beech(&mut p);
            return p;
        }
        if self == Self::SilverBirch {
            species::silver_birch(&mut p);
            return p;
        }
        if self == Self::DatePalm {
            species::date_palm(&mut p);
            return p;
        }
        if self == Self::Ordinary {
            return p;
        }
        let silver = self == Self::Telperion;
        p.material = materials::radiant(silver);
        p.skeleton.seed = if silver { 1 } else { 2 };
        // The Two Trees are an order of magnitude taller than a forest tree,
        // and every spacing here is a length in metres.
        p.skeleton.habit = HabitParams {
            reach_probe_steps: crate::ranges::default_reach_probe_steps(),
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
            stems: 1,
            stem_divergence: 0.0,
            stem_lean: 0.0,
            stem_lean_spread: 0.0,
            stem_fork_height: 0.0,
        };
        p.skeleton.envelope = if silver {
            Envelope {
                height: 148.0,
                crown_base: 0.38,
                spread: 0.24,
                fullness: 0.58,
                shoulder: 1.5,
                irregularity: 0.0,
                lobe_scale: 0.5,
            }
        } else {
            Envelope {
                height: 132.0,
                crown_base: 0.24,
                spread: 0.58,
                fullness: 0.38,
                shoulder: 3.2,
                irregularity: 0.0,
                lobe_scale: 0.5,
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
                    ..Default::default()
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
                    ..Default::default()
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
                ..Default::default()
            }
        } else {
            RadiusParams {
                trunk_radius: 0.055,
                fork_exponent: 2.7,
                length_taper: 0.35,
                ..Default::default()
            }
        };
        p.surface = SurfaceParams {
            socket_containment: crate::ranges::default_socket_containment(),
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
            clump_system_order: crate::ranges::default_clump_system_order(),
            clump_neighbours: crate::ranges::default_clump_neighbours(),
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
