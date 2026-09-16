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
    // The winter photograph (B-BARE, fn-54): a straight trunk clear to about
    // a fifth of the height, a core that stays the thickest wood to about
    // half of it and runs on toward the top, and limbs that leave it 25 to
    // 40 degrees from vertical and rise straight alongside it before they
    // spread; their side branches rise with them, so the fine wood is an
    // upward broom and not a tangle. Stations 2.6 m apart from a crown base
    // of 1.9 m put the first limbs at 2.6 m, two a station at 28 degrees,
    // barely bending (0.1) and hardly crooked. The leader runs to four fifths of
    // the height; limbs this steep carry less of the crown's top than
    // round 11's level ones did, so the core holds its girth without them
    // leaving at 65 degrees. Side branches run two thirds of their limb and
    // leave it level with its own heading, which is what widens the lower
    // crown: the limbs themselves stay close to the core.
    p.skeleton.habit = HabitParams {
        apical_dominance: 0.8,
        whorl_strength: 0.15,
        leader_internode: 2.6,
        laterals_per_station: 2,
        lateral_pitch: 28.0,
        pitch_variation: 6.0,
        rise_primary: 0.1,
        rise_secondary: 0.0,
        crookedness: 3.0,
        lateral_spacing: 2.0,
        lateral_length_ratio: 0.65,
        lateral_orders: 3,
        attractor_weight: 0.0,
        twig_tip_taper: 0.25,
        shedding_threshold: 0.0,
        stems: 1,
        stem_divergence: 0.0,
        stem_lean: 0.0,
        stem_lean_spread: 0.0,
    };
    // An upright oval, widest a little below the middle of the crown and
    // rounding to its top (B-BARE: width over height about 0.74). The shell
    // starts at 0.06 of the height, so no wood and no leaf stands under 1.9 m.
    p.skeleton.envelope = Envelope {
        height: 32.0,
        crown_base: 0.06,
        spread: 0.36,
        fullness: 0.3,
        shoulder: 1.8,
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
    // The twig layer starts on wood under 0.28 of the trunk's radius, so the
    // side branches of steep limbs carry shoots along most of their length
    // and the crown's inside is not bare wood under a shell of leaves.
    p.skeleton.twigs.limb_radius = 0.28;
    // Four twig laterals a station fill the crown; a twig a quarter of its
    // parent's length pays for them and for the wider twig layer, so the
    // heaviest protocol seed stays well under the node ceiling.
    p.skeleton.twigs.length_ratio = 0.23;
    // Short shoots carry most of the leaves, so the long shoots only have
    // to spread the crown: each stands out at 40 degrees, rising with the
    // broom its limb makes, turned by the golden angle, with a leaf every
    // 5 cm. Two-ranked at 32 degrees with a leaf every 2 cm, every spray was
    // a flat frond and the crown read as a hemlock (owner and host, round 11).
    p.skeleton.twigs.angle = 40.0;
    p.skeleton.twigs.divergence = 137.5;
    p.skeleton.twigs.twig.internode_length = 0.05;
    p.skeleton.twigs.twig.bearing_diameter = 0.03;
    // A trunk a little over a metre through at breast height, a third of
    // the default taper per metre, and a fork exponent of 2.9: a parent's
    // radius is the n-th root of the sum of its children's n-th powers, so a
    // higher n lets the core keep its girth ("much more thick core trunks
    // for almost the entire height", owner, round 5c). Round 6 found a cliff
    // at 2.8 under limbs at 48 degrees, where a few degrees of limb angle
    // collapsed the twig layer; under these steep limbs every protocol seed
    // grows its whole twig layer.
    p.radii.trunk_radius = 0.016;
    p.radii.length_taper = 0.2;
    p.radii.fork_exponent = 2.9;
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
    // Most of a beech's leaves stand in clusters all along its limbs and
    // deep inside the crown, on any wood under 0.7 of the trunk's radius,
    // which twig wood alone left bare. A cluster of eight leaves fanned a
    // half circle every 3 cm, held 20 cm off the wood: longer than a spur,
    // it stands where a leafy side shoot holds its leaves, so each limb
    // wears a sleeve of rosettes. Five leaves every 2.5 cm, 5 cm off, lined
    // every shoot with a comb of level leaves, and the crown's edge read as
    // fern fronds against the sky (host, round 17). They stand where the
    // slender wood's own row of leaves stood, so that row is off.
    p.canopy.shoot_radius = 0.0;
    p.canopy.short_shoot_spacing = 0.03;
    p.canopy.short_shoot_radius = 0.7;
    p.canopy.short_shoot_length = 0.2;
    p.canopy.short_shoot_leaves = 8;
    p.canopy.short_shoot_spread = 90.0;
    // Each limb system keeps a leaf mass of its own (fn-54): the leaves are
    // thinned within a quarter of the way from the wall between two
    // systems to their centres, so the crown breaks into rounded clumps
    // with gaps between them, as B-WHOLE's does; the spurs stand a
    // centimetre closer than round 18's to give back what the gaps take.
    p.canopy.limb_clumping = 0.25;
    // A beech leaf is held flat and turned every way about its shoot;
    // leaning along it (0.45) laid the leaves down the twig like needles.
    // Scattered 80 degrees rather than 45, fewer lie edge-on to an eye
    // below the crown, where a level leaf reads as a needle.
    p.canopy.forward_lean = 0.1;
    p.canopy.outward = 0.0;
    p.canopy.upward = 0.3;
    // A tenth larger with less spread, so the largest leaf stays inside
    // the sourced 4 to 10 cm.
    p.canopy.size = 1.1;
    p.canopy.divergence = 180.0;
    p.canopy.scatter = 80.0;
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
    // longer holds a hanging shoot. The whole drop, down to a clearance that
    // is the crown's own base: in both photographs the two stems stand clear
    // under the curtain for their first two metres, and one floor lower than
    // that curtains them where the lowest limbs hang beside them.
    p.skeleton.twigs.curtain_drop = 1.0;
    p.skeleton.twigs.curtain_clearance = 1.8;
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
