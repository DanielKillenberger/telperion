//! The habit trait table: every architectural dial the scaffold reads, numeric
//! and present on every family. Named models are regions of this space.
use crate::catalogue::{bounded, input, tuned, value, Blend, Bounds, Dial, Growth, Site};
use crate::Result;

crate::catalogue::rows! {
    /// Architectural traits, all numeric and present on every family. Named models
    /// are regions of this space, never variants of it.
    #[derive(Debug, Clone, Copy, PartialEq)]
    #[cfg_attr(feature = "json", derive(serde::Serialize, serde::Deserialize))]
    pub struct HabitParams in "/skeleton/habit" {
        /// Samples available to find an axis's room within the crown.
        #[cfg_attr(feature = "json", serde(default = "crate::ranges::default_reach_probe_steps"))]
        pub reach_probe_steps: u32 = "reachProbeSteps" "samples"
            crate::ranges::POSITIVE_COUNT.bounds() => [Grow] {
            check: value(Site::Habit, 0, "reachProbeSteps"),
            applies: "needs `lateralOrders` of one or more: only first-order laterals probe \
                their room",
            blend: Blend::Count,
            dial: Dial::Excluded("A sampling budget: samples spent finding an axis's room, not a \
                look the owner can ask for (crates/telperion-core/src/branching/traits.rs:15)."),
        },
        /// How far the leader persists into the crown, 0 to 1.
        pub apical_dominance: f64 = "apicalDominance" "share" Bounds::closed(0.0, 1.0) => [Grow] {
            check: input(Site::Habit, 1, "apical dominance"),
            growth: Growth::Differs("also splits the structural and local budget, and \
                `apicalControlLoss` divides it each year"),
            note: "The leader runs to `bole + (height - bole)·apicalDominance`, the bole being \
                `max(trunkHeight, height·crownBase)`.",
            dial: bounded("apical_dominance", "how far the leader keeps going into the crown",
                [0.15, 0.3]),
        },
        /// Clustering of laterals at a station against scattering along the axis.
        pub whorl_strength: f64 = "whorlStrength" "share" Bounds::closed(0.0, 1.0) => [Grow] {
            check: input(Site::Habit, 2, "whorl strength"),
            applies: "needs `lateralOrders` of one or more",
            dial: bounded("whorl_strength", "how tightly laterals cluster at one station rather \
                than scattering along the axis", [0.15, 0.3]),
        },
        /// Spacing between lateral stations on the leader, in metres.
        pub leader_internode: f64 = "leaderInternode" "m" Bounds::above(0.0) => [Grow] {
            check: input(Site::Habit, 3, "leader internode"),
            note: "Also the unit stems are placed in.",
            dial: tuned("leader_internode", "metres between lateral stations on the leader",
                [0.05, 3.45], [0.5, 1.0], "capped").span([0.05, 3.45])
                .cap("capped both ways: the generator validates no closed range here (only \
                    positive and finite); the row stays at the preset span until the generator \
                    authors one"),
        },
        /// Laterals borne by one station of the leader.
        pub laterals_per_station: u32 = "lateralsPerStation" "laterals"
            Bounds::closed(1.0, 12.0) => [Grow] {
            check: input(Site::Habit, 4, "laterals per station"),
            applies: "needs `lateralOrders` of one or more; leader stations only",
            blend: Blend::Count,
            dial: tuned("limbs", "limbs born at each station", [1.0, 4.0], [1.0, 2.0], "authored"),
        },
        /// Initial angle of a lateral from its parent axis, in degrees; on the
        /// upright leader this is degrees from vertical.
        pub lateral_pitch: f64 = "lateralPitch" "degrees" Bounds::closed(0.0, 180.0) => [Grow] {
            check: input(Site::Habit, 5, "lateral pitch"),
            applies: "needs `lateralOrders` of one or more",
            blend: Blend::Degrees,
            dial: bounded("lateral_pitch", "the degrees a lateral leaves its parent axis, from \
                vertical on the leader", [20.0, 40.0]).span([0.0, 118.0]),
        },
        /// Spread of the lateral pitch, in degrees.
        pub pitch_variation: f64 = "pitchVariation" "degrees" Bounds::closed(0.0, 90.0) => [Grow] {
            check: input(Site::Habit, 6, "lateral pitch variation"),
            applies: "needs `lateralOrders` of one or more",
            blend: Blend::Degrees,
            dial: bounded("pitch_variation", "the degrees that departure angle varies lateral to \
                lateral", [5.0, 10.0]).span([0.0, 28.0]),
        },
        /// Signed bend over the length of a first-order axis; positive rises.
        pub rise_primary: f64 = "risePrimary" "-" Bounds::closed(-1.0, 1.0) => [Grow] {
            check: input(Site::Habit, 7, "primary rise per order"),
            note: "Acts at orders zero and one, so it bends leaning stems; an upright stem has \
                no rise to take.",
            dial: bounded("rise_primary", "the bend over a first-order limb's length; positive \
                rises", [0.025, 0.05]).span([-0.03, 0.17]),
        },
        /// Signed bend over the length of a deeper axis; negative hangs.
        pub rise_secondary: f64 = "riseSecondary" "-" Bounds::closed(-1.0, 1.0) => [Grow] {
            check: input(Site::Habit, 8, "secondary rise per order"),
            applies: "needs `lateralOrders` of two or more",
            dial: bounded("rise_secondary", "the bend over a deeper axis's length; negative \
                hangs", [0.25, 0.5]),
        },
        /// Heading change between successive growth units, in degrees.
        pub crookedness: f64 = "crookedness" "degrees" Bounds::closed(0.0, 60.0) => [Grow] {
            check: input(Site::Habit, 9, "crookedness"),
            note: "Starts above `trunkHeight` in the scaffold, and also makes the local twig \
                layer wander.",
            blend: Blend::Degrees,
            dial: tuned("crookedness", "limb turning and zigzag amount",
                [0.0, 15.0], [3.0, 6.0], "authored"),
        },
        /// Spacing between lateral stations away from the leader, in metres.
        pub lateral_spacing: f64 = "lateralSpacing" "m" Bounds::above(0.0) => [Grow] {
            check: input(Site::Habit, 10, "lateral spacing"),
            applies: "needs `lateralOrders` of one or more",
            dial: tuned("lateral_spacing", "metres between lateral stations away from the leader",
                [1e-06, 2.925], [0.5, 1.0], "capped").span([1e-06, 2.925])
                .cap("capped both ways: the generator validates no closed range here (only \
                    positive and finite); the row stays at the preset span until the generator \
                    authors one"),
        },
        /// Length of a lateral against its supporting axis.
        pub lateral_length_ratio: f64 = "lateralLengthRatio" "share"
            Bounds::closed(0.0, 1.0) => [Grow] {
            check: input(Site::Habit, 11, "lateral length ratio"),
            growth: Growth::Differs("`apicalControlLoss` adds the dominance it releases to it"),
            applies: "needs `lateralOrders` of two or more: first-order laterals take their \
                reach from the probe",
            dial: bounded("lateral_length_ratio", "a lateral's length against the axis that \
                bears it", [0.15, 0.3]),
        },
        /// Depth of rule-built orders below the leader.
        pub lateral_orders: u32 = "lateralOrders" "orders" Bounds::closed(0.0, 8.0) => [Grow] {
            check: input(Site::Habit, 12, "lateral orders"),
            note: "At zero every other lateral row lies dormant.",
            blend: Blend::Count,
            dial: bounded("lateral_orders", "how many rule-built orders grow below the leader",
                [1.0, 2.0]),
        },
        /// Weight of the attractor pull beside the axis's own rule heading. At
        /// zero no pull points are scattered and every axis holds its own rule
        /// heading, and any rise switches the pull on.
        pub attractor_weight: f64 = "attractorWeight" "share" Bounds::closed(0.0, 1.0) => [Grow] {
            check: input(Site::Habit, 13, "attractor weight"),
            note: "At zero no pull point is scattered, so `attractors`, \
                `samplingAttemptsPerAttractor`, `influenceRadius` and `killDistance` lie \
                dormant.",
            dial: bounded("attractor_weight", "how far the pull points steer an axis beside its \
                own rule heading; at zero no pull points are scattered and every axis holds its \
                own rule heading, and any rise switches the pull on", [0.15, 0.3]),
        },
        /// Distal twig radius against the nominal twig radius.
        pub twig_tip_taper: f64 = "twigTipTaper" "share" Bounds::closed(0.0, 1.0).open() => [Grow] {
            check: input(Site::Habit, 14, "twig tip taper"),
            note: "Scales half the twig diameter at every childless tip, structural or twig.",
            dial: tuned("taper", "remaining wood thickness toward the crown edge; lower means \
                thinner tips", [0.05, 0.6], [0.1, 0.2], "authored"),
        },
        /// Monthly vigour threshold; zero disables shedding. The legacy envelope
        /// builder interprets it as shell depth.
        pub shedding_threshold: f64 = "sheddingThreshold" "share of height·spread"
            Bounds::closed(0.0, 1.0) => [Grow] {
            check: input(Site::Habit, 15, "shedding threshold"),
            growth: Growth::Differs("an annual vigour threshold, not a shell depth (fn-161 \
                splits the two)"),
            applies: "zero sheds nothing",
            note: "On the direct build it is the shed shell's depth, a share of `height·spread`, \
                apart from `shellDepth`.",
            dial: bounded("shedding_threshold", "the vigour below which a shoot is shed; zero \
                sheds nothing", [0.15, 0.3]),
        },
        /// Stems leaving the root. One is the single trunk every tree was, to the
        /// byte; a birch, a hazel or a coppiced oak stands on more.
        pub stems: u32 = "stems" "stems" Bounds::closed(1.0, 6.0) => [Grow] {
            check: input(Site::Habit, 16, "stems"),
            note: "At one the four stem rows lie dormant.",
            blend: Blend::Count,
            dial: bounded("stems", "stems leaving the root", [1.0, 2.0]),
        },
        /// Degrees of bearing between neighbouring stems, about a bearing the seed
        /// alone decides. Inert at one stem, which has no neighbour. At zero every
        /// stem leaves the root on one bearing, and any rise starts to fan them
        /// apart.
        pub stem_divergence: f64 = "stemDivergence" "degrees" Bounds::closed(0.0, 120.0) => [Grow] {
            check: input(Site::Habit, 17, "stem divergence"),
            applies: "one stem",
            note: "With more than one stem, refused where it and `stemLean` are both zero \
                (`stems_placed`).",
            blend: Blend::Degrees,
            dial: tuned("stem_divergence", "the degrees of bearing between neighbouring stems of \
                a clump; at zero every stem leaves the root on one bearing, and any rise starts \
                to fan them apart", [0.0, 15.0], [2.5, 5.0], "validated bound"),
        },
        /// Degrees from vertical the outermost stems tilt away from the root; the
        /// ones between tilt in proportion to how far out they stand. Inert at one
        /// stem, which stands at the centre and so tilts by none of it. At zero
        /// every stem stands upright, and any rise starts the tilt.
        pub stem_lean: f64 = "stemLean" "degrees" Bounds::closed(0.0, 45.0) => [Grow] {
            check: input(Site::Habit, 18, "stem lean"),
            applies: "one stem",
            note: "With more than one stem, refused at zero (`stems_placed`); `risePrimary` \
                bends the leaning stems.",
            blend: Blend::Degrees,
            dial: bounded("stem_lean", "the degrees from vertical the outermost stems tilt away \
                from the root; at zero every stem stands upright, and any rise starts the tilt",
                [5.0, 10.0]).span([0.0, 42.0]),
        },
        /// How unequally a clump's stems lean, 0 to 1. None of it is the lean
        /// above, shared about the clump's centre; all of it leans the stems in
        /// their order instead, the first upright and the last by all of
        /// `stem_lean`. Inert at one stem, which has nothing to lean against.
        pub stem_lean_spread: f64 = "stemLeanSpread" "share" Bounds::closed(0.0, 1.0) => [Grow] {
            check: input(Site::Habit, 19, "stem lean spread"),
            applies: "one stem, or `stemLean` zero",
            dial: bounded("stem_lean_spread", "how unequally a clump's stems lean", [0.15, 0.3]),
        },
        /// Where a clump's later stems leave the first, as a share of the bole's
        /// height, 0 to 0.5. None of it parts them at the ground; half of it parts
        /// them halfway up the bole, with one trunk below. Inert at one stem.
        pub stem_fork_height: f64 = "stemForkHeight" "share of the bole"
            Bounds::closed(0.0, 0.5) => [Grow] {
            check: input(Site::Habit, 20, "stem fork height"),
            applies: "one stem",
            note: "The bole is `max(trunkHeight, height·crownBase)`.",
            dial: bounded("stem_fork_height", "how far up the bole a clump's later stems part \
                from the first; at zero every stem leaves the root, and any rise starts the one \
                trunk below the fork", [0.1, 0.2]),
        },
    }
}
impl Default for HabitParams {
    fn default() -> Self {
        Self {
            reach_probe_steps: crate::ranges::default_reach_probe_steps(),
            apical_dominance: 0.5,
            whorl_strength: 0.3,
            leader_internode: 1.5,
            laterals_per_station: 3,
            lateral_pitch: 60.0,
            pitch_variation: 15.0,
            rise_primary: 0.05,
            rise_secondary: 0.0,
            crookedness: 12.0,
            lateral_spacing: 0.9,
            lateral_length_ratio: 0.4,
            lateral_orders: 3,
            attractor_weight: 1.0,
            twig_tip_taper: 1.0,
            shedding_threshold: 0.45,
            stems: 1,
            stem_divergence: 0.0,
            stem_lean: 0.0,
            stem_lean_spread: 0.0,
            stem_fork_height: 0.0,
        }
    }
}
impl HabitParams {
    pub fn validate(&self) -> Result<()> {
        crate::catalogue::check(Self::CHECKS, self, Site::Habit)
    }
}
