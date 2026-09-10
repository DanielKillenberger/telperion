//! JSON wire mirror of the parameter family, shared by the C-ABI binding and
//! the renderer. Gated behind the `json` feature so the default core stays
//! serde-free.
use crate::{
    presets::{Family, Preset},
    Error, Result,
};
use serde_json::{json, Value};

// This table is the wire schema: it also emits the browser's preset metadata.
macro_rules! fields {
    ($f:ident, $v:ident, $op:ident) => {
        $op!($f, $v, "skeleton", "habit", "apicalDominance"; skeleton.habit.apical_dominance);
        $op!($f, $v, "skeleton", "habit", "whorlStrength"; skeleton.habit.whorl_strength);
        $op!($f, $v, "skeleton", "habit", "leaderInternode"; skeleton.habit.leader_internode);
        $op!($f, $v, "skeleton", "habit", "lateralsPerStation"; skeleton.habit.laterals_per_station);
        $op!($f, $v, "skeleton", "habit", "lateralPitch"; skeleton.habit.lateral_pitch);
        $op!($f, $v, "skeleton", "habit", "pitchVariation"; skeleton.habit.pitch_variation);
        $op!($f, $v, "skeleton", "habit", "risePrimary"; skeleton.habit.rise_primary);
        $op!($f, $v, "skeleton", "habit", "riseSecondary"; skeleton.habit.rise_secondary);
        $op!($f, $v, "skeleton", "habit", "crookedness"; skeleton.habit.crookedness);
        $op!($f, $v, "skeleton", "habit", "lateralSpacing"; skeleton.habit.lateral_spacing);
        $op!($f, $v, "skeleton", "habit", "lateralLengthRatio"; skeleton.habit.lateral_length_ratio);
        $op!($f, $v, "skeleton", "habit", "lateralOrders"; skeleton.habit.lateral_orders);
        $op!($f, $v, "skeleton", "habit", "attractorWeight"; skeleton.habit.attractor_weight);
        $op!($f, $v, "skeleton", "habit", "twigTipTaper"; skeleton.habit.twig_tip_taper);
        $op!($f, $v, "skeleton", "habit", "sheddingThreshold"; skeleton.habit.shedding_threshold);
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
        .get_or_insert(crate::colonization::GrowthConfig::default().max_turn_per_step);
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
    fn known(v: &Value, schema: &Value, unknown: &'static str) -> Result<()> {
        let map = v.as_object().ok_or(Error::InvalidInput("family object"))?;
        for (k, value) in map {
            let s = schema.get(k).ok_or(Error::InvalidInput(unknown))?;
            if s.is_object() {
                known(
                    value,
                    s,
                    if k == "habit" {
                        "unknown habit trait"
                    } else {
                        unknown
                    },
                )?;
            }
        }
        Ok(())
    }
    known(v, &schema, "unknown family parameter")?;
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

// Serialization lives behind the `json` feature; the default core stays serde-free.
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
enum_wire!(crate::foliage::ElementAnatomy, {
    GenericBlade => "genericBlade", LobedBlade => "lobedBlade", FourSidedNeedle => "fourSidedNeedle"
});
enum_wire!(crate::foliage::Attachment, {
    Generic => "generic", Alternate => "alternate", RadialNeedles => "radialNeedles"
});
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
            // Every habit trait is a flat numeric row, set one at a time.
            for (trait_name, set) in [
                ("apicalDominance", json!(0.75)),
                ("whorlStrength", json!(0.4)),
                ("lateralsPerStation", json!(4)),
                ("riseSecondary", json!(-0.5)),
                ("crookedness", json!(13.0)),
                ("lateralOrders", json!(2)),
                ("attractorWeight", json!(0.0)),
            ] {
                value["skeleton"]["habit"][trait_name] = set;
                assert_eq!(value, metadata(&parse(&value).unwrap()));
            }
        }
        assert!(preset(999).is_err());
        assert!(parse(&json!("missing")).is_err());
        // A kind tag is a retired shape, not a parameter: the closed schema
        // refuses it by name, and every trait keeps its range.
        for bad in [
            json!({"kind":"spreading"}),
            json!({"kind":"tiered","tiers":4}),
            json!({"scaffoldLimbs":6}),
        ] {
            assert_eq!(
                parse(&json!({"skeleton":{"habit":bad}})).err(),
                Some(Error::InvalidInput("unknown habit trait"))
            );
        }
        for (trait_name, bad, message) in [
            ("apicalDominance", json!(1.5), "apical dominance"),
            ("whorlStrength", json!(-0.1), "whorl strength"),
            ("leaderInternode", json!(0.0), "leader internode"),
            ("lateralsPerStation", json!(0), "laterals per station"),
            ("crookedness", json!(90.0), "crookedness"),
            ("riseSecondary", json!(-2.0), "secondary rise per order"),
            ("attractorWeight", json!(2.0), "attractor weight"),
        ] {
            let mut habit = json!({});
            habit[trait_name] = bad;
            assert_eq!(
                parse(&json!({"skeleton":{"habit":habit}}))
                    .and_then(|f| f.skeleton.habit.validate()),
                Err(Error::InvalidInput(message))
            );
        }
        assert!(parse(&json!({"skeleton":{"bias":{"writheAmplitude":0.1}}})).is_err());
        assert!(parse(&json!({"skeleton":{"bias":{"supernatural":{"enabled":1}}}})).is_err());
    }
}
