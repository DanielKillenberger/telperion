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
    // A leader that carries the crown to the top, limbs leaving it one or
    // two at a time along its length and rising (owner, round 5b), and in
    // leaf one full oval on that trunk. The scaffold ends its leader at
    // crownBase + (1 - crownBase) * apicalDominance of the height, so 0.9
    // runs it to nine tenths. The leader is only as thick as what it
    // carries: a first-order limb runs to the shell and its tips grow with
    // the square of that run, so a steep limb from low on the bole carries
    // a share of the crown's top and the leader's girth with it. Leaving at
    // 65 degrees and bending up over their run (0.45), the low limbs meet
    // the shell short and the heaviest leave from a quarter to a half of
    // the height; at 48 degrees eight of them climbed to within a metre of
    // the leader's top and it was lost among them above a quarter. Side
    // branches half their limb's length, held nearly level, and two limbs
    // a station, 2.2 m apart, keep them few.
    p.skeleton.habit = HabitParams {
        apical_dominance: 0.9,
        whorl_strength: 0.15,
        leader_internode: 2.2,
        laterals_per_station: 2,
        lateral_pitch: 65.0,
        pitch_variation: 10.0,
        rise_primary: 0.45,
        rise_secondary: 0.1,
        crookedness: 6.0,
        lateral_spacing: 2.2,
        lateral_length_ratio: 0.5,
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
    // Fewer, heavier scaffold tips leave more of the scaffold above a tenth
    // of the trunk's radius, where no twig starts; the twig layer starts on
    // wood under 0.17 of it, so the crown keeps its shoots.
    p.skeleton.twigs.limb_radius = 0.17;
    // Four twig laterals a station fill the crown; a shorter twig pays for
    // them and for the longer leader, so the heaviest protocol seed keeps a
    // fifth of the ceiling in hand.
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
        // The birch's own lumps are finer than the beech's: a narrow crown of
        // hanging shoots, with the shell they hang from ragged rather than
        // lobed, lumps a quarter of the height across and a third of the
        // radius deep. The curtain falls past the shell's lower surface
        // (the drop below), so the lumps shape the crown the strands hang
        // from rather than the hem they end in.
        irregularity: 0.35,
        lobe_scale: 0.25,
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
    // The weeping birch's own curtain: a full hang, shoots running metres
    // unbranched instead of the twig's own quarter (their length is stated
    // with its variation below), every shoot under a descending limb hanging, and nine degrees between neighbours
    // where the spruce stands them four apart - a birch's curtain is long and
    // open, not the spruce's dense fringe.
    p.skeleton.twigs.hang = 2.4;
    p.skeleton.twigs.pendulous_radius = 1.0;
    p.skeleton.twigs.curtain_separation = 9.0;
    // And the curtain hangs: in S-WHOLE and S-BARE the shoots arch out of the
    // crown and fall nearly vertical over most of their length, so the row
    // states the whole of the way there. A birch's shoot runs only a share of
    // its pendulous length before the branch law stops it, and a shoot that
    // runs less has turned less, so anything short of the row's own end
    // leaves the curtain standing out of the crown rather than falling from it.
    p.skeleton.twigs.sag = 1.0;
    // And no two strands alike: in S-WHOLE and S-BARE they run anything from
    // a hand's width to about three metres, so the curtain ends in a ragged
    // hem, lower under the heavier limbs, and not at one height all round.
    // Each shoot runs its own share of a three-metre pendulous length, down
    // to a twentieth of it.
    p.skeleton.twigs.pendulous_length = 3.0;
    p.skeleton.twigs.pendulous_variation = 0.95;
    // And the curtain hangs below the crown: in S-WHOLE and S-BARE the limbs
    // make the crown's shape and the strands fall past it, so the shell no
    // longer holds a hanging shoot. The clearance is the crown's own base: in
    // both photographs the two stems stand clear under the curtain for their
    // first two metres, and one floor lower than that curtains them where the
    // lowest limbs hang beside them. Six tenths of the drop toward it, not the
    // whole: at the whole drop the band's foot is that one height all round,
    // and every strand long enough stopped on it, 657 strand ends in its
    // lowest ten centimetres, so the hem was a straight line (owner, round
    // 13). Short of the whole, a column's foot is the shell's lower surface
    // lowered by the share, so the hem rises away from the stems with the
    // shell's lumps and the strands end at their own lengths.
    p.skeleton.twigs.curtain_drop = 0.6;
    p.skeleton.twigs.curtain_clearance = 1.8;
    // Two stems share one root through the pipe model, so each is thinner than
    // a single birch's trunk (owner, round 13: "the trunk being too thin"). In
    // S-WHOLE the upright stem is 0.027 of the tree's height across at breast
    // height and the leaning one 0.017; at 0.01 the matched still's stems read
    // 0.018 and 0.017, and at 0.014 they read 0.024 and 0.023. The fork
    // exponent and the taper stay the family's: the stems already narrow to
    // about four fifths of their breast-height girth by a quarter of the
    // height, as S-BARE's stem does, and a lower exponent that thickens the
    // stems alone would put a parent thicker than its children's wood.
    p.radii.trunk_radius = 0.014;
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
