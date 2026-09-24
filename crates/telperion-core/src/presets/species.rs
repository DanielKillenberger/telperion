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
        reach_probe_steps: crate::ranges::default_reach_probe_steps(),
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
        stem_fork_height: 0.0,
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
        reach_probe_steps: crate::ranges::default_reach_probe_steps(),
        apical_dominance: 0.45,
        whorl_strength: 0.2,
        leader_internode: 1.6,
        // From inside the lower crown the four limbs a station threw at 62
        // degrees read as rings of near-horizontal spokes, one ring every
        // internode from 2 m to 9 m (owner, round 26: "way too many branches
        // in the middle"). A pair a station, leaving at 45 degrees, climbs
        // beside the stem the way the photographs' limbs do, and the droop
        // the owner sees in them is `rise_secondary` on the shoots they
        // carry, not the angle they leave at. At seed 1 the lower crown's
        // primaries fall from 27 to 13 and the tree from 174 to 82.
        laterals_per_station: 2,
        lateral_pitch: 45.0,
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
        // Where they part is the two photographs' disagreement: S-WHOLE's
        // pair leaves the ground as two, and S-BARE's stands on one trunk
        // that forks at its lowest limbs, about a third of its visible
        // height up. Both part at or under the crown's base and never in
        // the crown, and the owner asked for the second stem to move up, so
        // the table states the row's top, half the bole, as far toward
        // S-BARE as it reaches: one trunk to a metre, then the pair, the
        // trunk carrying on into the upright stem and the leaning one
        // leaving from its side.
        stems: 2,
        stem_divergence: 0.0,
        stem_lean: 28.0,
        stem_lean_spread: 1.0,
        stem_fork_height: 0.5,
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
    // Eight laterals a station at 0.6 of the parent's length filled the
    // interior with twigs no view from outside showed: 54,500 branches at
    // seed 1, most of them at orders 4 and 5, and a leaf mass the S-WHOLE
    // pair reads 12 grey levels darker in the middle than the photograph.
    // Seven at 0.55 leave 33,500 and put that centre on the photograph's own
    // 83, which is where the leaf spacing below was re-read and left alone.
    p.skeleton.twigs.laterals = 7;
    p.skeleton.twigs.length_ratio = 0.55;
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
    // S-LEAF: a triangular blade, widest a fifth of the way up over a broad,
    // nearly straight base, with a short pointed tip. Eight shallow notches along
    // each margin stand for its teeth: the lobe rows cut symmetric notches,
    // not the forward-leaning double teeth the photograph shows, which are
    // fn-60's. Thirty-two stations keep the outline from reading as facets.
    p.element = ElementParams {
        length: 0.055,
        width: 0.045,
        connector_length: 0.022,
        widest_at: 0.2,
        base_fullness: 0.3,
        tip_sharpness: 1.1,
        lobe_count: 8,
        lobe_depth: 0.1,
        section_roundness: 0.0,
        axial_segments: 32,
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

pub(super) fn date_palm(p: &mut Family) {
    // Corner's unbranched, monopodial habit (fn-108): a single stem that
    // stations no laterals and whose leader persists. The values from here
    // down are tuning revision 3 of fn-80's live run unless a comment says
    // otherwise; the owner accepted the palm on them in the harness on
    // 2026-09-24 (fn-82).
    p.skeleton.habit.stems = 1;
    p.skeleton.habit.lateral_orders = 0;
    p.skeleton.habit.apical_dominance = 0.85;
    p.skeleton.habit.laterals_per_station = 1;
    p.skeleton.habit.rise_secondary = -1.0;
    p.skeleton.habit.crookedness = 1.5;
    p.skeleton.habit.attractor_weight = 0.625;
    p.skeleton.bias.gravitropism = 0.25;
    p.skeleton.bias.lean = 0.02;
    p.skeleton.bias.supernatural.writhe_wavelength = 0.35;
    // A narrow envelope whose crown starts at 45 % of the height.
    p.skeleton.envelope.height = 22.86;
    p.skeleton.envelope.crown_base = 0.45;
    p.skeleton.envelope.spread = 0.1;
    p.skeleton.envelope.shoulder = 0.45;
    p.skeleton.twigs.twig.diameter = 0.007;
    p.skeleton.twigs.hang = 2.75;
    p.skeleton.twigs.sag = 1.0;
    p.skeleton.twigs.max_droop = 8.6;
    p.skeleton.twigs.pendulous_length = 2.75;
    p.skeleton.twigs.curtain_drop = 0.15;
    p.skeleton.twigs.curtain_step_clearance = 0.725;
    p.radii.trunk_radius = 0.013;
    p.radii.length_taper = 0.25;
    p.surface.flare_radius = 2.0;
    // The crown (fn-109): a rosette of pinnate fronds at the apex and
    // nothing borne below it.
    p.canopy.rosette_fronds = 42;
    p.canopy.rosette_pitch = 12.5;
    p.canopy.rosette_pitch_spread = 105.0;
    p.canopy.rosette_depth = 0.35;
    p.canopy.leaflet_count = 113;
    p.canopy.rachis_length = 7.0;
    p.canopy.leaflet_pitch = 55.0;
    p.canopy.rachis_arch = -0.6;
    p.canopy.terminal_leaflet = 1.0;
    p.canopy.size = 2.35;
    p.canopy.upward = 0.225;
    // The leaflet: long, narrow and sharp-tipped, with enough axial
    // stations to bend smoothly along its length.
    p.element.length = 0.6197;
    p.element.width = 0.0725;
    p.element.tip_sharpness = 2.6;
    p.element.section_roundness = 0.15;
    p.element.axial_segments = 13;
    // The trunk organs (fn-110): the boots of shed fronds clothing the bole
    // on the crown's own spiral, worn back toward the ground, and the eight
    // basal leaflets of a Phoenix frond borne as spines. Revision 3 packed
    // 256 boots (owner's call).
    p.canopy.leaf_bases = 256;
    p.canopy.leaf_base_length = 0.255;
    p.canopy.leaf_base_radius = 0.25;
    p.canopy.leaf_base_pitch = 55.0;
    p.canopy.leaf_base_weathering = 0.75;
    // The lattice (fn-144): the boots packed edge to edge as flat-faced
    // diamonds; a flat section crowded least at every width.
    p.canopy.leaf_base_width = 0.9;
    p.canopy.leaf_base_flatness = 1.0;
    p.canopy.acanthophylls = 8;
    p.canopy.acanthophyll_length = 0.3;
    p.canopy.acanthophyll_pitch = 75.0;
    // The skirt (fn-120): the oldest fronds kept after they die, collapsed
    // against the upper trunk below the living crown. Growers trim the lower
    // leaves (F1), so a partial ring rather than a full petticoat. Set before
    // tuning; revision 3 left them.
    p.canopy.skirt_fronds = 16;
    p.canopy.skirt_pitch = 165.0;
    p.canopy.skirt_length = 0.85;
    // The skirt hangs against the trunk, the deepest place inside the
    // crown's envelope, where the default shell cull drops most of it.
    p.shell_depth = 0.85;
    // Bark: a warm brown, with plate, fissure, crest and weathering relief.
    p.material.bark_red = 0.325;
    p.material.bark_green = 0.1975;
    p.material.bark_blue = 0.135;
    p.material.bark_roughness = 0.7775;
    p.material.furrow_strength = 0.85;
    p.material.plate_scale = 0.1125;
    p.material.plate_furrow_width = 0.0375;
    p.material.ridge_scale = 0.01;
    p.material.fissure_red = 0.4375;
    p.material.fissure_strength = 0.3;
    p.material.crest_red = 0.04375;
    p.material.crest_strength = 0.225;
    p.material.depth_strength = 0.075;
    p.material.orientation_red = 0.03;
    p.material.orientation_strength = 0.225;
    p.material.weathering_red = 0.00875;
    p.material.weathering_strength = 0.3;
    // Leaflets: a dull grey-green, the same on both faces, varying little.
    p.material.leaf_front_red = 0.12;
    p.material.leaf_front_green = 0.16;
    p.material.leaf_front_blue = 0.1175;
    p.material.leaf_back_red = 0.12;
    p.material.leaf_back_green = 0.15375;
    p.material.leaf_back_blue = 0.11625;
    p.material.hue_range_low = -0.0275;
    p.material.hue_range_high = 0.0275;
    p.material.brightness_range_low = -0.04;
    p.material.brightness_range_high = 0.03625;
    // Dead fronds of the skirt, brown to grey; set before tuning.
    p.material.leaf_dead_red = 0.34;
    p.material.leaf_dead_green = 0.29;
    p.material.leaf_dead_blue = 0.22;
}
