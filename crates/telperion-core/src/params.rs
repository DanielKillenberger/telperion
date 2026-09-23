//! JSON wire mirror of the parameter family, shared by the C-ABI binding and
//! the renderer. Gated behind the `json` feature so the default core stays
//! serde-free.
use crate::{presets::Family, Error, Result};
use serde_json::{json, Value};

// This table is the wire schema: it also emits the browser's preset metadata.
macro_rules! fields {
    ($f:ident, $v:ident, $op:ident) => {
        $op!($f, $v, "age"; age);
        $op!($f, $v, "growth", "rate"; growth.rate);
        $op!($f, $v, "growth", "shape"; growth.shape);
        $op!($f, $v, "growth", "leafLifetime"; growth.leaf_lifetime);
        $op!($f, $v, "growth", "resizeTolerance"; growth.resize_tolerance);
        $op!($f, $v, "growth", "workBudget"; growth.work_budget);
        $op!($f, $v, "growth", "sheddingTolerance"; growth.shedding_tolerance);
        $op!($f, $v, "growth", "apicalControlLoss"; growth.apical_control_loss);
        $op!($f, $v, "skeleton", "habit", "apicalDominance"; skeleton.habit.apical_dominance);
        $op!($f, $v, "skeleton", "habit", "reachProbeSteps"; skeleton.habit.reach_probe_steps);
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
        $op!($f, $v, "skeleton", "habit", "stems"; skeleton.habit.stems);
        $op!($f, $v, "skeleton", "habit", "stemDivergence"; skeleton.habit.stem_divergence);
        $op!($f, $v, "skeleton", "habit", "stemLean"; skeleton.habit.stem_lean);
        $op!($f, $v, "skeleton", "habit", "stemLeanSpread"; skeleton.habit.stem_lean_spread);
        $op!($f, $v, "skeleton", "habit", "stemForkHeight"; skeleton.habit.stem_fork_height);
        $op!($f, $v, "element", "connectorLength"; element.connector_length);
        $op!($f, $v, "skeleton", "seed"; skeleton.seed);
        $op!($f, $v, "skeleton", "attractors"; skeleton.attractors);
        $op!($f, $v, "skeleton", "samplingAttemptsPerAttractor"; skeleton.sampling_attempts_per_attractor);
        $op!($f, $v, "skeleton", "step"; skeleton.step);
        $op!($f, $v, "skeleton", "envelope", "height"; skeleton.envelope.height);
        $op!($f, $v, "skeleton", "envelope", "crownBase"; skeleton.envelope.crown_base);
        $op!($f, $v, "skeleton", "envelope", "spread"; skeleton.envelope.spread);
        $op!($f, $v, "skeleton", "envelope", "fullness"; skeleton.envelope.fullness);
        $op!($f, $v, "skeleton", "envelope", "shoulder"; skeleton.envelope.shoulder);
        $op!($f, $v, "skeleton", "envelope", "irregularity"; skeleton.envelope.irregularity);
        $op!($f, $v, "skeleton", "envelope", "lobeScale"; skeleton.envelope.lobe_scale);
        $op!($f, $v, "skeleton", "bias", "gravitropism"; skeleton.bias.gravitropism);
        $op!($f, $v, "skeleton", "bias", "lean"; skeleton.bias.lean);
        $op!($f, $v, "skeleton", "bias", "supernatural", "enabled"; skeleton.bias.supernatural.enabled);
        $op!($f, $v, "skeleton", "bias", "supernatural", "writheAmplitude"; skeleton.bias.supernatural.writhe_amplitude);
        $op!($f, $v, "skeleton", "bias", "supernatural", "writheWavelength"; skeleton.bias.supernatural.writhe_wavelength);
        $op!($f, $v, "skeleton", "bias", "supernatural", "maxWritheMagnitude"; skeleton.bias.supernatural.max_writhe_magnitude);
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
        $op!($f, $v, "skeleton", "twigs", "generations"; skeleton.twigs.generations);
        $op!($f, $v, "skeleton", "twigs", "maxInternodes"; skeleton.twigs.max_internodes);
        $op!($f, $v, "skeleton", "twigs", "maxDroop"; skeleton.twigs.max_droop);
        $op!($f, $v, "skeleton", "twigs", "curtainStepClearance"; skeleton.twigs.curtain_step_clearance);
        $op!($f, $v, "skeleton", "twigs", "limbRadius"; skeleton.twigs.limb_radius);
        $op!($f, $v, "skeleton", "twigs", "reach"; skeleton.twigs.reach);
        $op!($f, $v, "skeleton", "twigs", "angle"; skeleton.twigs.angle);
        $op!($f, $v, "skeleton", "twigs", "angleVariation"; skeleton.twigs.angle_variation);
        $op!($f, $v, "skeleton", "twigs", "vigourVariation"; skeleton.twigs.vigour_variation);
        $op!($f, $v, "skeleton", "twigs", "divergence"; skeleton.twigs.divergence);
        $op!($f, $v, "skeleton", "twigs", "hang"; skeleton.twigs.hang);
        $op!($f, $v, "skeleton", "twigs", "pendulousLength"; skeleton.twigs.pendulous_length);
        $op!($f, $v, "skeleton", "twigs", "pendulousRadius"; skeleton.twigs.pendulous_radius);
        $op!($f, $v, "skeleton", "twigs", "curtainSeparation"; skeleton.twigs.curtain_separation);
        $op!($f, $v, "skeleton", "twigs", "sag"; skeleton.twigs.sag);
        $op!($f, $v, "skeleton", "twigs", "pendulousVariation"; skeleton.twigs.pendulous_variation);
        $op!($f, $v, "skeleton", "twigs", "curtainDrop"; skeleton.twigs.curtain_drop);
        $op!($f, $v, "skeleton", "twigs", "curtainClearance"; skeleton.twigs.curtain_clearance);
        $op!($f, $v, "skeleton", "growth", "influenceRadius"; skeleton.growth.influence_radius);
        $op!($f, $v, "skeleton", "growth", "killDistance"; skeleton.growth.kill_distance);
        $op!($f, $v, "skeleton", "growth", "stepDistance"; skeleton.growth.step_distance);
        $op!($f, $v, "skeleton", "growth", "trunkHeight"; skeleton.growth.trunk_height);
        $op!($f, $v, "skeleton", "growth", "maxNodes"; skeleton.growth.max_nodes);
        $op!($f, $v, "skeleton", "growth", "maxTurnPerStep"; skeleton.growth.max_turn_per_step);
        $op!($f, $v, "radii", "trunkRadius"; radii.trunk_radius);
        $op!($f, $v, "radii", "forkExponent"; radii.fork_exponent);
        $op!($f, $v, "radii", "lengthTaper"; radii.length_taper);
        $op!($f, $v, "radii", "maxTaperExponent"; radii.max_taper_exponent);
        $op!($f, $v, "surface", "radialSegments"; surface.radial_segments);
        $op!($f, $v, "surface", "lobes"; surface.lobes);
        $op!($f, $v, "surface", "lobeDepth"; surface.lobe_depth);
        $op!($f, $v, "surface", "twistRate"; surface.twist_rate);
        $op!($f, $v, "surface", "flareRadius"; surface.flare_radius);
        $op!($f, $v, "surface", "flareFalloff"; surface.flare_falloff);
        $op!($f, $v, "surface", "flareDepth"; surface.flare_depth);
        $op!($f, $v, "surface", "forkSocket"; surface.fork_socket);
        $op!($f, $v, "surface", "socketContainment"; surface.socket_containment);
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
        $op!($f, $v, "canopy", "shortShootSpacing"; canopy.short_shoot_spacing);
        $op!($f, $v, "canopy", "shortShootRadius"; canopy.short_shoot_radius);
        $op!($f, $v, "canopy", "shortShootLength"; canopy.short_shoot_length);
        $op!($f, $v, "canopy", "shortShootLeaves"; canopy.short_shoot_leaves);
        $op!($f, $v, "canopy", "shortShootSpread"; canopy.short_shoot_spread);
        $op!($f, $v, "canopy", "limbClumping"; canopy.limb_clumping);
        $op!($f, $v, "canopy", "clumpSystemOrder"; canopy.clump_system_order);
        $op!($f, $v, "canopy", "clumpNeighbours"; canopy.clump_neighbours);
        $op!($f, $v, "canopy", "rosetteFronds"; canopy.rosette_fronds);
        $op!($f, $v, "canopy", "rosetteDivergence"; canopy.rosette_divergence);
        $op!($f, $v, "canopy", "rosettePitch"; canopy.rosette_pitch);
        $op!($f, $v, "canopy", "rosettePitchSpread"; canopy.rosette_pitch_spread);
        $op!($f, $v, "canopy", "rosetteDepth"; canopy.rosette_depth);
        $op!($f, $v, "canopy", "leafletCount"; canopy.leaflet_count);
        $op!($f, $v, "canopy", "rachisLength"; canopy.rachis_length);
        $op!($f, $v, "canopy", "leafletPitch"; canopy.leaflet_pitch);
        $op!($f, $v, "canopy", "rachisArch"; canopy.rachis_arch);
        $op!($f, $v, "canopy", "terminalLeaflet"; canopy.terminal_leaflet);
        $op!($f, $v, "canopy", "leafBases"; canopy.leaf_bases);
        $op!($f, $v, "canopy", "leafBaseLength"; canopy.leaf_base_length);
        $op!($f, $v, "canopy", "leafBaseRadius"; canopy.leaf_base_radius);
        $op!($f, $v, "canopy", "leafBasePitch"; canopy.leaf_base_pitch);
        $op!($f, $v, "canopy", "leafBaseWeathering"; canopy.leaf_base_weathering);
        $op!($f, $v, "canopy", "acanthophylls"; canopy.acanthophylls);
        $op!($f, $v, "canopy", "acanthophyllLength"; canopy.acanthophyll_length);
        $op!($f, $v, "canopy", "acanthophyllPitch"; canopy.acanthophyll_pitch);
        $op!($f, $v, "canopy", "skirtFronds"; canopy.skirt_fronds);
        $op!($f, $v, "canopy", "skirtPitch"; canopy.skirt_pitch);
        $op!($f, $v, "canopy", "skirtLength"; canopy.skirt_length);
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
        $op!($f, $v, "material", "shootRed"; material.shoot_red);
        $op!($f, $v, "material", "shootGreen"; material.shoot_green);
        $op!($f, $v, "material", "shootBlue"; material.shoot_blue);
        $op!($f, $v, "material", "shootRadius"; material.shoot_radius);
        $op!($f, $v, "material", "leafFrontRed"; material.leaf_front_red);
        $op!($f, $v, "material", "leafFrontGreen"; material.leaf_front_green);
        $op!($f, $v, "material", "leafFrontBlue"; material.leaf_front_blue);
        $op!($f, $v, "material", "leafBackRed"; material.leaf_back_red);
        $op!($f, $v, "material", "leafBackGreen"; material.leaf_back_green);
        $op!($f, $v, "material", "leafBackBlue"; material.leaf_back_blue);
        $op!($f, $v, "material", "leafDeadRed"; material.leaf_dead_red);
        $op!($f, $v, "material", "leafDeadGreen"; material.leaf_dead_green);
        $op!($f, $v, "material", "leafDeadBlue"; material.leaf_dead_blue);
        $op!($f, $v, "material", "hueRangeLow"; material.hue_range_low);
        $op!($f, $v, "material", "hueRangeHigh"; material.hue_range_high);
        $op!($f, $v, "material", "brightnessRangeLow"; material.brightness_range_low);
        $op!($f, $v, "material", "brightnessRangeHigh"; material.brightness_range_high);
        $op!($f, $v, "material", "interiorDarkening"; material.interior_darkening);
        $op!($f, $v, "material", "ridgeScale"; material.ridge_scale);
        $op!($f, $v, "material", "plateScale"; material.plate_scale);
        $op!($f, $v, "material", "furrowStrength"; material.furrow_strength);
        $op!($f, $v, "material", "roughnessDetail"; material.roughness_detail);
        $op!($f, $v, "material", "veinScale"; material.vein_scale);
        $op!($f, $v, "material", "veinContrast"; material.vein_contrast);
        $op!($f, $v, "material", "transmissionStrength"; material.transmission_strength);
        $op!($f, $v, "material", "transmissionRed"; material.transmission_red);
        $op!($f, $v, "material", "transmissionGreen"; material.transmission_green);
        $op!($f, $v, "material", "transmissionBlue"; material.transmission_blue);
        $op!($f, $v, "material", "thickness"; material.thickness);
        $op!($f, $v, "material", "fissureRed"; material.fissure_red);
        $op!($f, $v, "material", "fissureGreen"; material.fissure_green);
        $op!($f, $v, "material", "fissureBlue"; material.fissure_blue);
        $op!($f, $v, "material", "fissureStrength"; material.fissure_strength);
        $op!($f, $v, "material", "crestRed"; material.crest_red);
        $op!($f, $v, "material", "crestGreen"; material.crest_green);
        $op!($f, $v, "material", "crestBlue"; material.crest_blue);
        $op!($f, $v, "material", "crestStrength"; material.crest_strength);
        $op!($f, $v, "material", "barkMottleScale"; material.bark_mottle_scale);
        $op!($f, $v, "material", "barkMottleStrength"; material.bark_mottle_strength);
        $op!($f, $v, "material", "cavityStrength"; material.cavity_strength);
        $op!($f, $v, "material", "bladeMottleScale"; material.blade_mottle_scale);
        $op!($f, $v, "material", "bladeMottleStrength"; material.blade_mottle_strength);
        $op!($f, $v, "material", "marginWidth"; material.margin_width);
        $op!($f, $v, "material", "marginRed"; material.margin_red);
        $op!($f, $v, "material", "marginGreen"; material.margin_green);
        $op!($f, $v, "material", "marginBlue"; material.margin_blue);
        $op!($f, $v, "material", "cuticleGloss"; material.cuticle_gloss);
        $op!($f, $v, "material", "skyOcclusionStrength"; material.sky_occlusion_strength);
        $op!($f, $v, "material", "plateCellScale"; material.plate_cell_scale);
        $op!($f, $v, "material", "plateElongation"; material.plate_elongation);
        $op!($f, $v, "material", "plateDome"; material.plate_dome);
        $op!($f, $v, "material", "plateEdgeLift"; material.plate_edge_lift);
        $op!($f, $v, "material", "plateFurrowWidth"; material.plate_furrow_width);
        $op!($f, $v, "material", "plateEdgeShape"; material.plate_edge_shape);
        $op!($f, $v, "material", "plateIdentity"; material.plate_identity);
        $op!($f, $v, "material", "weatheringStrength"; material.weathering_strength);
        $op!($f, $v, "material", "weatheringRed"; material.weathering_red);
        $op!($f, $v, "material", "weatheringGreen"; material.weathering_green);
        $op!($f, $v, "material", "weatheringBlue"; material.weathering_blue);
        $op!($f, $v, "material", "orientationStrength"; material.orientation_strength);
        $op!($f, $v, "material", "orientationRed"; material.orientation_red);
        $op!($f, $v, "material", "orientationGreen"; material.orientation_green);
        $op!($f, $v, "material", "orientationBlue"; material.orientation_blue);
        $op!($f, $v, "material", "directionalOcclusion"; material.directional_occlusion);
        $op!($f, $v, "material", "depthStrength"; material.depth_strength);
        $op!($f, $v, "material", "canopyNormal"; material.canopy_normal);
        $op!($f, $v, "material", "lightWrap"; material.light_wrap);
        $op!($f, $v, "material", "diffuseTransmission"; material.diffuse_transmission);
        $op!($f, $v, "material", "leafSheen"; material.leaf_sheen);
        $op!($f, $v, "material", "crownShade"; material.crown_shade);
        $op!($f, $v, "material", "lichenScale"; material.lichen_scale);
        $op!($f, $v, "material", "lichenCoverage"; material.lichen_coverage);
        $op!($f, $v, "material", "lichenRed"; material.lichen_red);
        $op!($f, $v, "material", "lichenGreen"; material.lichen_green);
        $op!($f, $v, "material", "lichenBlue"; material.lichen_blue);
        $op!($f, $v, "material", "lichenStrength"; material.lichen_strength);
        $op!($f, $v, "material", "lenticelDensity"; material.lenticel_density);
        $op!($f, $v, "material", "lenticelLength"; material.lenticel_length);
        $op!($f, $v, "material", "lenticelStrength"; material.lenticel_strength);
        $op!($f, $v, "material", "lenticelTint"; material.lenticel_tint);
        $op!($f, $v, "material", "peelCurl"; material.peel_curl);
        $op!($f, $v, "material", "peelRed"; material.peel_red);
        $op!($f, $v, "material", "peelGreen"; material.peel_green);
        $op!($f, $v, "material", "peelBlue"; material.peel_blue);
        $op!($f, $v, "material", "lobeShade"; material.lobe_shade);
        $op!($f, $v, "material", "barkReflectance"; material.bark_reflectance);
        $op!($f, $v, "material", "leafReflectance"; material.leaf_reflectance);
        $op!($f, $v, "material", "barkGrainScale"; material.bark_grain_scale);
        $op!($f, $v, "material", "barkGrainStrength"; material.bark_grain_strength);
        $op!($f, $v, "material", "bladeGrainScale"; material.blade_grain_scale);
        $op!($f, $v, "material", "bladeGrainStrength"; material.blade_grain_strength);

        $op!($f, $v, "shellDepth"; shell_depth);
    };
}
pub use crate::presets::{by_identity, CATALOGUE, IN_WORK};
pub fn preset(id: u32) -> Result<Family> {
    let identity = CATALOGUE
        .iter()
        .find(|entry| entry.0 == id)
        .ok_or(Error::InvalidInput("preset id"))?
        .1;
    by_identity(identity)
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
                $f.$($field).+ = Wire::decode(value.clone())
                    .map_err(|_| Error::InvalidInput(concat!($("/", $key),+)))?;
            }
        };
    }
    fields!(f, v, read);
    // The material row has no builder of its own to judge it: no mesh depends
    // on a colour, so nothing downstream would ever look. The wire is its
    // consumer, and the wire is where a value off its range or a range that
    // runs backwards is refused, by the name of the field that was wrong.
    f.material.validate()?;
    crate::growth::Age::from_years(f.age)?;
    f.growth.validate()?;
    Ok(f)
}

/// A family with some of its rows restated: `overrides` is a partial wire
/// object laid over the family's own wire, then read back through `parse`, so
/// an unknown key or a value off its range is refused by name. A value trial
/// states only the rows it moves.
pub fn overlay(f: &Family, overrides: &Value) -> Result<Family> {
    let mut wire = metadata(f);
    lay(&mut wire, overrides)?;
    parse(&wire)
}
fn lay(wire: &mut Value, over: &Value) -> Result<()> {
    let map = over
        .as_object()
        .ok_or(Error::InvalidInput("family object"))?;
    for (key, value) in map {
        match (wire.get_mut(key), value.is_object()) {
            (Some(slot), true) if slot.is_object() => lay(slot, value)?,
            (Some(slot), _) => *slot = value.clone(),
            (None, _) => return Err(Error::InvalidInput("unknown family parameter")),
        }
    }
    Ok(())
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
