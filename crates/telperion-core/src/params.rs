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
        $op!($f, $v, "element", "connectorLength"; element.connector_length);
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
        $op!($f, $v, "canopy", "forwardLean"; canopy.forward_lean);
        $op!($f, $v, "canopy", "leanRise"; canopy.lean_rise);
        $op!($f, $v, "canopy", "surfaceContact"; canopy.surface_contact);
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
        $op!($f, $v, "element", "lobeCount"; element.lobe_count);
        $op!($f, $v, "element", "lobeDepth"; element.lobe_depth);
        $op!($f, $v, "element", "sectionRoundness"; element.section_roundness);
        $op!($f, $v, "element", "card"; element.card);
        $op!($f, $v, "material", "barkRed"; material.bark_red);
        $op!($f, $v, "material", "barkGreen"; material.bark_green);
        $op!($f, $v, "material", "barkBlue"; material.bark_blue);
        $op!($f, $v, "material", "barkRoughness"; material.bark_roughness);
        $op!($f, $v, "material", "leafFrontRed"; material.leaf_front_red);
        $op!($f, $v, "material", "leafFrontGreen"; material.leaf_front_green);
        $op!($f, $v, "material", "leafFrontBlue"; material.leaf_front_blue);
        $op!($f, $v, "material", "leafBackRed"; material.leaf_back_red);
        $op!($f, $v, "material", "leafBackGreen"; material.leaf_back_green);
        $op!($f, $v, "material", "leafBackBlue"; material.leaf_back_blue);
        $op!($f, $v, "material", "hueRangeLow"; material.hue_range_low);
        $op!($f, $v, "material", "hueRangeHigh"; material.hue_range_high);
        $op!($f, $v, "material", "brightnessRangeLow"; material.brightness_range_low);
        $op!($f, $v, "material", "brightnessRangeHigh"; material.brightness_range_high);
        $op!($f, $v, "material", "interiorDarkening"; material.interior_darkening);
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
                    match k.as_str() {
                        "habit" => "unknown habit trait",
                        "element" => "unknown element trait",
                        "canopy" => "unknown canopy trait",
                        "material" => "unknown material trait",
                        _ => unknown,
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
    // The material row has no builder of its own to judge it: no mesh depends
    // on a colour, so nothing downstream would ever look. The wire is its
    // consumer, and the wire is where a value off its range or a range that
    // runs backwards is refused, by the name of the field that was wrong.
    f.material.validate()?;
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
// The wire's own tests - every control of every family through the schema in
// both directions, and every refusal by the name of what was refused - live
// beside this file rather than in it, so neither outgrows the line rule.
#[cfg(test)]
mod tests;
