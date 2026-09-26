//! The first two species, the oak and the spruce (fn19), as the other
//! species are: one function of rows per family.
use super::materials;
use crate::{
    bias::BiasParams, branching::HabitParams, envelope::Envelope, foliage::ElementParams,
    presets::Family,
};

pub(super) fn oregon_white_oak(p: &mut Family) {
    // Mature, open-grown Quercus garryana. Metre dimensions are
    // calibrated against the frozen profile, not inferred from seed.
    p.skeleton.habit = HabitParams {
        reach_probe_steps: crate::ranges::default_reach_probe_steps(),
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
        stems: 1,
        stem_divergence: 0.0,
        stem_lean: 0.0,
        stem_lean_spread: 0.0,
        stem_fork_height: 0.0,
    };
    p.skeleton.envelope = Envelope {
        height: 24.0,
        crown_base: 0.16,
        spread: 0.55,
        fullness: 0.55,
        shoulder: 2.2,
        irregularity: 0.0,
        lobe_scale: 0.5,
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
    p.material = materials::oak();
}

pub(super) fn norway_spruce(p: &mut Family) {
    // Provisional needle retention; age calibration remains separate.
    p.growth.leaf_lifetime = 6.0;
    // Open-grown landscape Picea abies; one needle per local station.
    p.skeleton.habit = HabitParams {
        reach_probe_steps: crate::ranges::default_reach_probe_steps(),
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
        stems: 1,
        stem_divergence: 0.0,
        stem_lean: 0.0,
        stem_lean_spread: 0.0,
        stem_fork_height: 0.0,
    };
    p.skeleton.envelope = Envelope {
        height: 15.0,
        crown_base: 0.04,
        spread: 0.31,
        fullness: 0.15,
        shoulder: 1.0,
        irregularity: 0.0,
        lobe_scale: 0.5,
    };
    p.skeleton.bias = BiasParams::NONE;
    p.skeleton.twigs.twig.diameter = 0.002;
    p.skeleton.twigs.twig.internode_length = 0.0025;
    p.skeleton.twigs.twig.bearing_diameter = 0.02;
    // The spruce's curtain, as rows. These four are the values the
    // twig layer used while the curtain was a hidden mode: full hang,
    // every shoot under a descending limb, the default twig length as
    // the pendulous run, four degrees between neighbours.
    p.skeleton.twigs.hang = 1.0;
    p.skeleton.twigs.pendulous_length = 0.25;
    p.skeleton.twigs.pendulous_radius = 1.0;
    p.skeleton.twigs.curtain_separation = 4.0;
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
    p.material = materials::spruce();
}
