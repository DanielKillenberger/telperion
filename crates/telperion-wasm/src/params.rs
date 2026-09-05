use serde_json::{json, Value};
use telperion_core::{
    presets::{Family, Preset},
    Error, Result,
};

// This table is the wire schema: it also emits the browser's preset metadata.
macro_rules! fields {
    ($f:ident, $v:ident, $op:ident) => {
        $op!($f, $v, "skeleton", "habit"; skeleton.habit);
        $op!($f, $v, "element", "anatomy"; element.anatomy);
        $op!($f, $v, "element", "connectorLength"; element.connector_length);
        $op!($f, $v, "canopy", "attachment"; canopy.attachment);
        $op!($f, $v, "skeleton", "seed"; skeleton.seed);
        $op!($f, $v, "skeleton", "attractors"; skeleton.attractors);
        $op!($f, $v, "skeleton", "step"; skeleton.step);
        $op!($f, $v, "skeleton", "envelope", "height"; skeleton.envelope.height);
        $op!($f, $v, "skeleton", "envelope", "crownBase"; skeleton.envelope.crown_base);
        $op!($f, $v, "skeleton", "envelope", "spread"; skeleton.envelope.spread);
        $op!($f, $v, "skeleton", "envelope", "fullness"; skeleton.envelope.fullness);
        $op!($f, $v, "skeleton", "envelope", "shoulder"; skeleton.envelope.shoulder);
        $op!($f, $v, "skeleton", "bias", "gravitropism"; skeleton.bias.gravitropism);
        $op!($f, $v, "skeleton", "bias", "lean"; skeleton.bias.lean);
        $op!($f, $v, "skeleton", "bias", "supernatural", "enabled"; skeleton.bias.supernatural.enabled);
        $op!($f, $v, "skeleton", "bias", "supernatural", "writheAmplitude"; skeleton.bias.supernatural.writhe_amplitude);
        $op!($f, $v, "skeleton", "bias", "supernatural", "writheWavelength"; skeleton.bias.supernatural.writhe_wavelength);
        $op!($f, $v, "skeleton", "bias", "supernatural", "spiralRate"; skeleton.bias.supernatural.spiral_rate);
        $op!($f, $v, "skeleton", "twigs", "twig", "diameter"; skeleton.twigs.twig.diameter);
        $op!($f, $v, "skeleton", "twigs", "twig", "length"; skeleton.twigs.twig.length);
        $op!($f, $v, "skeleton", "twigs", "twig", "internodeLength"; skeleton.twigs.twig.internode_length);
        $op!($f, $v, "skeleton", "twigs", "twig", "stationsPerInternode"; skeleton.twigs.twig.stations_per_internode);
        $op!($f, $v, "skeleton", "twigs", "twig", "bearingDiameter"; skeleton.twigs.twig.bearing_diameter);
        $op!($f, $v, "skeleton", "twigs", "lengthRatio"; skeleton.twigs.length_ratio);
        $op!($f, $v, "skeleton", "twigs", "ratioPower"; skeleton.twigs.ratio_power);
        $op!($f, $v, "skeleton", "twigs", "internodeFactor"; skeleton.twigs.internode_factor);
        $op!($f, $v, "skeleton", "twigs", "laterals"; skeleton.twigs.laterals);
        $op!($f, $v, "skeleton", "twigs", "limbRadius"; skeleton.twigs.limb_radius);
        $op!($f, $v, "skeleton", "twigs", "reach"; skeleton.twigs.reach);
        $op!($f, $v, "skeleton", "twigs", "angle"; skeleton.twigs.angle);
        $op!($f, $v, "skeleton", "twigs", "angleVariation"; skeleton.twigs.angle_variation);
        $op!($f, $v, "skeleton", "twigs", "vigourVariation"; skeleton.twigs.vigour_variation);
        $op!($f, $v, "skeleton", "twigs", "divergence"; skeleton.twigs.divergence);
        $op!($f, $v, "skeleton", "growth", "influenceRadius"; skeleton.growth.influence_radius);
        $op!($f, $v, "skeleton", "growth", "killDistance"; skeleton.growth.kill_distance);
        $op!($f, $v, "skeleton", "growth", "stepDistance"; skeleton.growth.step_distance);
        $op!($f, $v, "skeleton", "growth", "trunkHeight"; skeleton.growth.trunk_height);
        $op!($f, $v, "skeleton", "growth", "maxNodes"; skeleton.growth.max_nodes);
        $op!($f, $v, "skeleton", "growth", "maxTurnPerStep"; skeleton.growth.max_turn_per_step);
        $op!($f, $v, "radii", "trunkRadius"; radii.trunk_radius);
        $op!($f, $v, "radii", "forkExponent"; radii.fork_exponent);
        $op!($f, $v, "radii", "lengthTaper"; radii.length_taper);
        $op!($f, $v, "surface", "radialSegments"; surface.radial_segments);
        $op!($f, $v, "surface", "lobes"; surface.lobes);
        $op!($f, $v, "surface", "lobeDepth"; surface.lobe_depth);
        $op!($f, $v, "surface", "twistRate"; surface.twist_rate);
        $op!($f, $v, "surface", "flareRadius"; surface.flare_radius);
        $op!($f, $v, "surface", "flareFalloff"; surface.flare_falloff);
        $op!($f, $v, "surface", "flareDepth"; surface.flare_depth);
        $op!($f, $v, "surface", "forkSocket"; surface.fork_socket);
        $op!($f, $v, "surface", "forkSwell"; surface.fork_swell);
        $op!($f, $v, "canopy", "shootRadius"; canopy.shoot_radius);
        $op!($f, $v, "canopy", "spacing"; canopy.spacing);
        $op!($f, $v, "canopy", "divergence"; canopy.divergence);
        $op!($f, $v, "canopy", "clump"; canopy.clump);
        $op!($f, $v, "canopy", "clumpSpan"; canopy.clump_span);
        $op!($f, $v, "canopy", "outward"; canopy.outward);
        $op!($f, $v, "canopy", "upward"; canopy.upward);
        $op!($f, $v, "canopy", "scatter"; canopy.scatter);
        $op!($f, $v, "canopy", "size"; canopy.size);
        $op!($f, $v, "canopy", "sizeVariation"; canopy.size_variation);
        $op!($f, $v, "canopy", "maxInstances"; canopy.max_instances);
        $op!($f, $v, "element", "length"; element.length);
        $op!($f, $v, "element", "width"; element.width);
        $op!($f, $v, "element", "widestAt"; element.widest_at);
        $op!($f, $v, "element", "baseFullness"; element.base_fullness);
        $op!($f, $v, "element", "tipSharpness"; element.tip_sharpness);
        $op!($f, $v, "element", "cup"; element.cup);
        $op!($f, $v, "element", "curl"; element.curl);
        $op!($f, $v, "element", "axialSegments"; element.axial_segments);
        $op!($f, $v, "element", "crossSegments"; element.cross_segments);
        $op!($f, $v, "element", "card"; element.card);
        $op!($f, $v, "shellDepth"; shell_depth);
    };
}
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
    (1, "telperion", "Telperion", "The silver tree"),
    (2, "laurelin", "Laurelin", "The golden tree"),
];
pub fn preset(id: u32) -> Result<Family> {
    let identity = CATALOGUE
        .iter()
        .find(|entry| entry.0 == id)
        .ok_or(Error::InvalidInput("preset id"))?
        .1;
    by_identity(identity)
}
pub fn by_identity(id: &str) -> Result<Family> {
    let mut f = Preset::from_id(id)
        .ok_or(Error::InvalidInput("preset identity"))?
        .parameters();
    f.skeleton
        .growth
        .max_turn_per_step
        .get_or_insert(telperion_core::colonization::GrowthConfig::default().max_turn_per_step);
    Ok(f)
}
pub fn metadata(f: &Family) -> Value {
    let mut v = json!({});
    macro_rules! emit {
        ($f:ident, $v:ident, $($key:literal),+; $($field:ident).+) => {
            $v$([$key])+ = Wire::encode(&$f.$($field).+);
        };
    }
    fields!(f, v, emit);
    v
}
pub fn parse(v: &Value) -> Result<Family> {
    if let Some(id) = v.as_str() {
        return by_identity(id);
    }
    let mut f = preset(0)?;
    let schema = metadata(&f);
    fn known(v: &Value, schema: &Value) -> Result<()> {
        let map = v.as_object().ok_or(Error::InvalidInput("family object"))?;
        for (k, value) in map {
            let s = schema
                .get(k)
                .ok_or(Error::InvalidInput("unknown family parameter"))?;
            if s.is_object() && k != "habit" {
                known(value, s)?;
            }
        }
        Ok(())
    }
    known(v, &schema)?;
    macro_rules! read {
        ($f:ident, $v:ident, $($key:literal),+; $($field:ident).+) => {
            if let Some(value) = $v.pointer(concat!($("/", $key),+)) {
                $f.$($field).+ = Wire::decode(value.clone())?;
            }
        };
    }
    fields!(f, v, read);
    Ok(f)
}

// Serialization belongs to the binding; native core remains serde-free.
trait Wire: Sized {
    fn encode(&self) -> Value;
    fn decode(value: Value) -> Result<Self>;
}
macro_rules! scalar_wire {
    ($($t:ty),+) => { $(impl Wire for $t {
        fn encode(&self) -> Value { json!(self) }
        fn decode(value: Value) -> Result<Self> {
            serde_json::from_value(value).map_err(|_| Error::InvalidInput("parameter type or range"))
        }
    })+ };
}
scalar_wire!(
    f64,
    u32,
    usize,
    bool,
    Option<f64>,
    Option<u32>,
    Option<usize>
);
macro_rules! enum_wire {
    ($t:ty, {$($variant:ident => $name:literal),+}) => {
        impl Wire for $t {
            fn encode(&self) -> Value { json!(match self { $(Self::$variant => $name),+ }) }
            fn decode(value: Value) -> Result<Self> {
                match value.as_str() { $(Some($name) => Ok(Self::$variant)),+,
                    _ => Err(Error::InvalidInput("unknown anatomy or attachment")) }
            }
        }
    };
}
enum_wire!(telperion_core::foliage::ElementAnatomy, {
    GenericBlade => "genericBlade", LobedBlade => "lobedBlade", FourSidedNeedle => "fourSidedNeedle"
});
enum_wire!(telperion_core::foliage::Attachment, {
    Generic => "generic", Alternate => "alternate", RadialNeedles => "radialNeedles"
});
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase", deny_unknown_fields)]
enum Habit {
    Colonizing {},
    #[serde(rename_all = "camelCase")]
    Spreading {
        scaffold_limbs: u32,
        subdivisions: u32,
        crookedness: f64,
    },
    #[serde(rename_all = "camelCase")]
    Tiered {
        tiers: u32,
        branches_per_tier: u32,
        secondary_spacing: f64,
        secondary_length: f64,
        upturn: f64,
    },
}
impl Wire for telperion_core::branching::BranchHabit {
    fn encode(&self) -> Value {
        let wire = match *self {
            Self::Colonizing => Habit::Colonizing {},
            Self::Spreading(p) => Habit::Spreading {
                scaffold_limbs: p.scaffold_limbs,
                subdivisions: p.subdivisions,
                crookedness: p.crookedness,
            },
            Self::Tiered(p) => Habit::Tiered {
                tiers: p.tiers,
                branches_per_tier: p.branches_per_tier,
                secondary_spacing: p.secondary_spacing,
                secondary_length: p.secondary_length,
                upturn: p.upturn,
            },
        };
        json!(wire)
    }
    fn decode(value: Value) -> Result<Self> {
        use telperion_core::branching::{SpreadingHabit, TieredHabit};
        let wire =
            serde_json::from_value(value).map_err(|_| Error::InvalidInput("habit parameters"))?;
        let habit = match wire {
            Habit::Colonizing {} => Self::Colonizing,
            Habit::Spreading {
                scaffold_limbs,
                subdivisions,
                crookedness,
            } => Self::Spreading(SpreadingHabit {
                scaffold_limbs,
                subdivisions,
                crookedness,
            }),
            Habit::Tiered {
                tiers,
                branches_per_tier,
                secondary_spacing,
                secondary_length,
                upturn,
            } => Self::Tiered(TieredHabit {
                tiers,
                branches_per_tier,
                secondary_spacing,
                secondary_length,
                upturn,
            }),
        };
        habit.validate()?;
        Ok(habit)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn catalogue_roundtrips_all_controls_and_identities() {
        for &(abi, id, _, _) in CATALOGUE.iter().rev() {
            let mut value = metadata(&preset(abi).unwrap());
            assert_eq!(value, metadata(&parse(&json!(id)).unwrap()));
            assert_eq!(value, metadata(&parse(&value).unwrap()));
            value["skeleton"]["bias"]["supernatural"] = json!({"enabled":false,"writheAmplitude":0.12,"writheWavelength":0.4,"spiralRate":3.0});
            value["element"]["connectorLength"] = json!(0.002);
            for habit in [
                json!({"kind":"colonizing"}),
                json!({"kind":"spreading","scaffoldLimbs":6,"subdivisions":2,"crookedness":13.0}),
                json!({"kind":"tiered","tiers":4,"branchesPerTier":3,"secondarySpacing":0.2,"secondaryLength":0.4,"upturn":0.1}),
            ] {
                value["skeleton"]["habit"] = habit;
                assert_eq!(value, metadata(&parse(&value).unwrap()));
            }
        }
        assert!(preset(999).is_err());
        assert!(parse(&json!("missing")).is_err());
        for bad in [
            json!({"kind":"unknown"}),
            json!({"kind":"colonizing","crookedness":3}),
            json!({"kind":"spreading","scaffoldLimbs":0,"subdivisions":2,"crookedness":13}),
            json!({"kind":"tiered","tiers":4,"branchesPerTier":3,"secondarySpacing":0,"secondaryLength":0.4,"upturn":0.1}),
        ] {
            assert!(parse(&json!({"skeleton":{"habit":bad}})).is_err());
        }
        assert!(parse(&json!({"skeleton":{"bias":{"writheAmplitude":0.1}}})).is_err());
        assert!(parse(&json!({"skeleton":{"bias":{"supernatural":{"enabled":1}}}})).is_err());
    }
}
