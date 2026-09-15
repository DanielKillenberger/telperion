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
    // Round 6, read against both photographs: a leader that carries the
    // crown to the top, limbs leaving it along the whole height and rising
    // straight up and out (owner, round 5b), and in leaf one full oval on
    // that trunk. The scaffold ends its leader at crownBase + (1 -
    // crownBase) * apicalDominance of the height, so 0.9 runs it to nine
    // tenths; below 0.6 it stopped at three fifths and forked. With the
    // leader carrying the top, the limbs can leave at 48 degrees and bend
    // up over their run (0.45) without the low ones climbing beside it to
    // the crown's top, which at 32 degrees made a vase of seven stems under
    // an umbrella of leaves. Their side branches are held nearly level, and
    // two limbs a station, 2.2 m apart, keep them few.
    p.skeleton.habit = HabitParams {
        apical_dominance: 0.9,
        whorl_strength: 0.15,
        leader_internode: 2.2,
        laterals_per_station: 2,
        lateral_pitch: 48.0,
        pitch_variation: 10.0,
        rise_primary: 0.45,
        rise_secondary: 0.1,
        crookedness: 6.0,
        lateral_spacing: 2.2,
        lateral_length_ratio: 0.6,
        lateral_orders: 3,
        attractor_weight: 0.0,
        twig_tip_taper: 0.25,
        shedding_threshold: 0.0,
        stems: 1,
        stem_divergence: 0.0,
        stem_lean: 0.0,
        stem_lean_spread: 0.0,
    };
    // The crown starts low, is widest a little under half its depth and
    // rounds off at the top rather than squaring into a shoulder.
    p.skeleton.envelope = Envelope {
        height: 32.0,
        crown_base: 0.05,
        spread: 0.52,
        fullness: 0.48,
        shoulder: 1.5,
        // A beech's crown is lumpy where its limbs end and hollow where they
        // do not. Four or five broad lobes around a wavelength most of the
        // tree's own height, at not quite a fifth of the radius.
        irregularity: 0.18,
        lobe_scale: 0.7,
    };
    p.skeleton.bias = BiasParams::NONE;
    // The twig law is two generations deep and says so: a lateral born at
    // generation two is a twig whatever the pipe model left its radius.
    p.skeleton.twigs.generations = 2;
    p.skeleton.twigs.laterals = 4;
    // Four twig laterals a station fill the crown; a shorter twig pays for
    // them and for the longer leader, so the heaviest protocol seed keeps a
    // sixth of the ceiling in hand.
    p.skeleton.twigs.length_ratio = 0.30;
    // A local shoot follows its limb rather than standing off it at 45,
    // and its own shoots are two-ranked: a beech's spray is flat, where one
    // turned by the golden angle read as a bottlebrush frond.
    p.skeleton.twigs.angle = 32.0;
    p.skeleton.twigs.divergence = 180.0;
    p.skeleton.twigs.twig.bearing_diameter = 0.03;
    p.radii.trunk_radius = 0.014;
    // Two thirds of the default taper per metre, and a fork exponent of
    // 2.6: a parent's radius is the n-th root of the sum of its children's
    // n-th powers, so a higher n lets the leader keep its girth. At 2.8 the
    // wood sat on a cliff - a few degrees of limb angle collapsed the local
    // layer to a third of its nodes - and 2.6 keeps the core off it.
    p.radii.length_taper = 0.2;
    p.radii.fork_exponent = 2.6;
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
    // Leaves follow twig wood one for one; clothing every shoot under a
    // twentieth of the trunk's radius leafs the crown's inside without
    // growing a node for it.
    p.canopy.shoot_radius = 0.05;
    // A beech leaf leans forward along its shoot and its blade is held
    // flat, and it is large against the spacing of its shoots. Standing
    // straight out on both sides of a two-ranked shoot, as at a lean of
    // 0.2, every spray read as a fern's comb of pinnae.
    p.canopy.forward_lean = 0.45;
    p.canopy.outward = 0.0;
    p.canopy.upward = 0.3;
    // A tenth larger with less spread, so the largest leaf stays inside
    // the sourced 4 to 10 cm.
    p.canopy.size = 1.1;
    p.canopy.divergence = 180.0;
    p.canopy.scatter = 30.0;
    p.canopy.size_variation = 0.12;
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
        // The photograph's birch stands on two stems that part at the ground:
        // one near vertical, the other leaning well out before it rises into
        // the crown. The whole spread stands the first upright and leans the
        // second by all of the lean, so the pair is that and not an even V.
        // An upright stem has no bearing to part from, so a divergence would
        // only turn the leaning one about the seed's own bearing; it is left
        // at none. Twenty-eight degrees is the lean the photographs' leaning
        // stem shows across the view; the curtain leaves only the first
        // couple of metres of stem bare, and less put the two of them still
        // inside one another's bark over that stretch.
        stems: 2,
        stem_divergence: 0.0,
        stem_lean: 28.0,
        stem_lean_spread: 1.0,
    };
    // A narrower crown carrying its mass up top, with squarer shoulders than
    // the round-3 oval. The crown's base is also the curtain's floor: a shoot
    // that hangs by its own weight falls to it, and in both photographs the
    // two stems stand clear under the curtain for their first two metres.
    p.skeleton.envelope = Envelope {
        height: 18.0,
        crown_base: 0.10,
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
    // A leaf every 60 mm of shoot: two stems carry two crowns' worth of
    // shoots inside one shell, and at the single stem's 36 mm the curtain
    // closed back into a mass with no sky through it.
    p.skeleton.twigs.twig.internode_length = 0.06;
    p.skeleton.twigs.twig.bearing_diameter = 0.02;
    // The weeping birch's own curtain: a full hang, shoots running three and a
    // half metres unbranched instead of the twig's own quarter, every shoot
    // under a descending limb hanging, and nine degrees between neighbours
    // where the spruce stands them four apart - a birch's curtain is long and
    // open, not the spruce's dense fringe.
    p.skeleton.twigs.hang = 2.4;
    p.skeleton.twigs.pendulous_length = 2.5;
    p.skeleton.twigs.pendulous_radius = 1.0;
    p.skeleton.twigs.curtain_separation = 9.0;
    // And the curtain hangs: in S-WHOLE and S-BARE the shoots arch out of the
    // crown and fall nearly vertical over most of their length, so the row
    // states the whole of the way there. A birch's shoot runs only a share of
    // its pendulous length before the branch law stops it, and a shoot that
    // runs less has turned less, so anything short of the row's own end
    // leaves the curtain standing out of the crown rather than falling from it.
    p.skeleton.twigs.sag = 1.0;
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
