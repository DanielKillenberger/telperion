//! Botanical species value tables. Legendary and ordinary families stay above.
use super::materials;
use crate::{
    bias::BiasParams, branching::HabitParams, envelope::Envelope, foliage::ElementParams,
    presets::Family,
};

fn oak_growth(p: &mut Family, age: f64) {
    p.age = age;
    p.growth = crate::growth::GrowthTraits::default();
}

pub(super) fn european_beech(p: &mut Family) {
    // Mature open-grown Fagus sylvatica. Envelope height is the 32 m
    // reference at 120 yr; oak growth traits, not a beech calibration.
    oak_growth(p, 120.0);
    p.skeleton.habit = HabitParams {
        apical_dominance: 0.2,
        whorl_strength: 0.15,
        leader_internode: 2.2,
        laterals_per_station: 4,
        lateral_pitch: 50.0,
        pitch_variation: 12.0,
        rise_primary: 0.08,
        rise_secondary: 0.0,
        crookedness: 8.0,
        lateral_spacing: 1.8,
        lateral_length_ratio: 0.42,
        lateral_orders: 3,
        attractor_weight: 0.0,
        twig_tip_taper: 0.25,
        shedding_threshold: 0.0,
    };
    p.skeleton.envelope = Envelope {
        height: 32.0,
        crown_base: 0.12,
        spread: 0.5,
        fullness: 0.7,
        shoulder: 1.8,
    };
    p.skeleton.bias = BiasParams::NONE;
    p.skeleton.twigs.laterals = 4;
    p.skeleton.twigs.length_ratio = 0.42;
    p.skeleton.twigs.twig.bearing_diameter = 0.03;
    p.radii.trunk_radius = 0.014;
    p.element = ElementParams {
        length: 0.07,
        width: 0.045,
        connector_length: 0.008,
        widest_at: 0.45,
        base_fullness: 0.75,
        tip_sharpness: 0.7,
        lobe_count: 0,
        lobe_depth: 0.0,
        section_roundness: 0.0,
        axial_segments: 8,
        ..Default::default()
    };
    p.shell_depth = 1.0;
    p.canopy.forward_lean = 0.2;
    p.canopy.outward = 0.0;
    p.canopy.upward = 0.0;
    p.canopy.divergence = 180.0;
    p.canopy.size_variation = 0.2;
    p.material = materials::beech();
}

pub(super) fn silver_birch(p: &mut Family) {
    // Mature open-grown Betula pendula. Envelope height is the 18 m
    // reference at 70 yr; oak growth traits, pendulous laterals.
    oak_growth(p, 70.0);
    p.skeleton.habit = HabitParams {
        apical_dominance: 0.45,
        whorl_strength: 0.2,
        leader_internode: 1.6,
        laterals_per_station: 4,
        lateral_pitch: 55.0,
        pitch_variation: 14.0,
        rise_primary: 0.05,
        rise_secondary: -0.4,
        crookedness: 10.0,
        lateral_spacing: 1.2,
        lateral_length_ratio: 0.4,
        lateral_orders: 3,
        attractor_weight: 0.0,
        twig_tip_taper: 0.35,
        shedding_threshold: 0.0,
    };
    p.skeleton.envelope = Envelope {
        height: 18.0,
        crown_base: 0.16,
        spread: 0.38,
        fullness: 0.35,
        shoulder: 1.3,
    };
    p.skeleton.bias = BiasParams::NONE;
    p.skeleton.twigs.laterals = 5;
    p.skeleton.twigs.length_ratio = 0.4;
    p.skeleton.twigs.twig.diameter = 0.003;
    p.skeleton.twigs.twig.bearing_diameter = 0.02;
    p.radii.trunk_radius = 0.01;
    p.element = ElementParams {
        length: 0.055,
        width: 0.045,
        connector_length: 0.022,
        widest_at: 0.28,
        base_fullness: 0.55,
        tip_sharpness: 1.8,
        lobe_count: 0,
        lobe_depth: 0.0,
        section_roundness: 0.0,
        axial_segments: 8,
        ..Default::default()
    };
    p.shell_depth = 1.0;
    p.canopy.forward_lean = 0.15;
    p.canopy.outward = 0.0;
    p.canopy.upward = 0.0;
    p.canopy.divergence = 180.0;
    p.canopy.size_variation = 0.2;
    p.material = materials::birch();
}
