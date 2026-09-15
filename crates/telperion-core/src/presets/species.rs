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
    // beech's crown is dense and the cap is a value like any other.
    // Round 5b, the owner on the round-5 pairs: "clearly not structurally
    // sound. The reference grows relatively straight up and out. Our
    // generation bends too much." The winter photograph's trunk runs up
    // through the crown and its limbs leave it steeply and keep rising;
    // ours split low and arced out nearly flat. So the leader keeps its
    // dominance, the limbs leave at 38 degrees and rise, and the axes are
    // barely crooked.
    p.skeleton.habit = HabitParams {
        // Round 6: with two limbs a station the leader was one of four
        // co-dominant axes by mid-crown, and the core thinned where the
        // photograph's is still thick. At 0.57 - the side of the bound a
        // broadleaf stays on - the leader runs to 96 per cent of the height
        // and carries half again the radius at three fifths of it. Above it,
        // at 0.59, two of the protocol's own seeds collapse a wood triangle
        // in float32 when the surface is built.
        apical_dominance: 0.57,
        whorl_strength: 0.15,
        leader_internode: 2.2,
        laterals_per_station: 2,
        // Round 6b, on the photograph: its limbs leave the trunk steeply and
        // every order of them keeps rising. Ours left at 42 degrees and the
        // second order did not rise at all, so the lowest limbs swept out and
        // down. At 32 degrees with a rising second order the low limbs climb
        // through the crown instead of around its outside, which is also what
        // carries twigs into the crown's interior.
        lateral_pitch: 32.0,
        pitch_variation: 10.0,
        rise_primary: 0.3,
        rise_secondary: 0.4,
        crookedness: 6.0,
        // Round 6, the owner on the 5c winter pair: "Fewer larger branches
        // compared to ours which has many more thinner ones directly attached
        // to the trunk." Half as many limbs a station, a station further
        // apart, and each limb more than half its parent's length.
        lateral_spacing: 2.2,
        lateral_length_ratio: 0.6,
        lateral_orders: 3,
        attractor_weight: 0.0,
        twig_tip_taper: 0.25,
        shedding_threshold: 0.0,
    };
    p.skeleton.envelope = Envelope {
        height: 32.0,
        crown_base: 0.12,
        spread: 0.52,
        shoulder: 1.8,
        // Round 6b: the crown read as a broad fan widest high. The widest
        // point moves down the crown's own depth to the photograph's.
        fullness: 0.55,
        // A beech's crown is lumpy where its limbs end and hollow where they
        // do not, and the round-3 pair read as an oval against it. Four or
        // five broad lobes around a wavelength most of the tree's own height,
        // at not quite a fifth of the radius.
        irregularity: 0.18,
        lobe_scale: 0.7,
    };
    p.skeleton.bias = BiasParams::NONE;
    // Round 6, fn-45. Two rows buy the girth the owner asked for. The twig
    // law is two generations deep and says so: a lateral born at generation
    // two is a twig whatever the pipe model left its radius, so the depth no
    // longer follows the wood. And three laterals a station instead of five
    // is where the budget for a Murray's-law core comes from - the beech
    // trades twig density for girth, which is what the photograph shows.
    p.skeleton.twigs.generations = 2;
    p.skeleton.twigs.laterals = 3;
    p.skeleton.twigs.length_ratio = 0.40;
    // Round 6b: a local shoot left its limb at 45 degrees, which on a limb
    // rising at 60 put the shoot itself near horizontal. At 32 it follows the
    // limb up.
    p.skeleton.twigs.angle = 32.0;
    p.skeleton.twigs.twig.bearing_diameter = 0.03;
    p.radii.trunk_radius = 0.014;
    // Round 5c, the owner: "much more thick core trunks for almost the
    // entire height of the tree. Our tree thins out too quickly." The beech
    // sheds two thirds of the default taper per metre, which is what keeps
    // the core traceable into the top fifth of the tree.
    p.radii.length_taper = 0.2;
    // Round 6: a parent's radius is the n-th root of the sum of its
    // children's n-th powers, so a higher n lets the leader keep the girth
    // its branches take. Near Murray's 3 the core runs thick up through the
    // crown. It costs nodes, and the twig laterals above pay for them.
    p.radii.fork_exponent = 2.8;
    // A beech stands on a modest flare, not the oak's buttress.
    p.surface.flare_radius = 1.5;
    p.element = ElementParams {
        length: 0.08,
        width: 0.05,
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
    // Round 6b: leaves follow twig wood one for one, and twig wood grows
    // only from scaffold tips and from wood under `limb_radius` of the
    // trunk's, so the leaf mass was a shell over a bare vase. Clothing every
    // shoot under a twentieth of the trunk's radius leafs the crown's inside
    // without growing a node for it - twice the leaves at the same wood.
    p.canopy.shoot_radius = 0.05;
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
        lateral_pitch: 62.0,
        pitch_variation: 14.0,
        rise_primary: 0.02,
        rise_secondary: -0.85,
        crookedness: 10.0,
        lateral_spacing: 1.2,
        lateral_length_ratio: 0.4,
        lateral_orders: 3,
        attractor_weight: 0.0,
        twig_tip_taper: 0.35,
        shedding_threshold: 0.0,
    };
    // The curtain is what reaches the ground, so the envelope has to leave it
    // somewhere to reach: almost no bare trunk, a narrower crown carrying its
    // mass up top, and squarer shoulders than the round-3 oval.
    p.skeleton.envelope = Envelope {
        height: 18.0,
        crown_base: 0.015,
        spread: 0.36,
        fullness: 0.6,
        shoulder: 1.6,
        // The birch's own lumps are finer and shallower than the beech's: a
        // narrow crown of hanging shoots, with the shell they hang from
        // ragged rather than lobed.
        irregularity: 0.15,
        lobe_scale: 0.45,
    };
    p.skeleton.bias = BiasParams::NONE;
    p.skeleton.twigs.laterals = 8;
    p.skeleton.twigs.length_ratio = 0.6;
    p.skeleton.twigs.twig.diameter = 0.003;
    p.skeleton.twigs.twig.length = 0.45;
    p.skeleton.twigs.twig.internode_length = 0.012;
    p.skeleton.twigs.twig.bearing_diameter = 0.02;
    // The weeping birch's own curtain: a full hang, shoots running three and a
    // half metres unbranched instead of the twig's own quarter, every shoot
    // under a descending limb hanging, and nine degrees between neighbours
    // where the spruce stands them four apart - a birch's curtain is long and
    // open, not the spruce's dense fringe.
    p.skeleton.twigs.hang = 2.4;
    p.skeleton.twigs.pendulous_length = 3.5;
    p.skeleton.twigs.pendulous_radius = 1.0;
    p.skeleton.twigs.curtain_separation = 9.0;
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
    // Sky through the crown: a curtain three and a half metres long fills the
    // shell twice over, and the photograph's birch is a light tree. Retaining
    // less than half the shell's depth is what lets the light back in.
    p.shell_depth = 0.45;
    p.canopy.forward_lean = 0.6;
    p.canopy.lean_rise = 0.3;
    p.canopy.clump = 8;
    p.canopy.clump_span = 0.5;
    p.canopy.outward = 0.0;
    // Leaves hang under the shoots they are strung along, which is the signed
    // half of the row a leaf could not reach before.
    p.canopy.upward = -0.35;
    p.canopy.divergence = 180.0;
    p.canopy.size_variation = 0.2;
    p.material = materials::birch();
}
