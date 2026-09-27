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
            wire: 9,
            check: value(Site::Habit, 0, "reachProbeSteps"),
            applies: "needs `lateralOrders` of one or more: only first-order laterals probe \
                their room",
            blend: Blend::Count,
            dial: Dial::Excluded("A sampling budget: samples spent finding an axis's room, not a \
                look the owner can ask for (crates/telperion-core/src/branching/traits.rs:15)."),
        },
        /// How far the leader persists into the crown, 0 to 1.
        pub apical_dominance: f64 = "apicalDominance" "share" Bounds::closed(0.0, 1.0) => [Grow] {
            wire: 8,
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
            wire: 10,
            check: input(Site::Habit, 2, "whorl strength"),
            applies: "needs `lateralOrders` of one or more",
            dial: bounded("whorl_strength", "how tightly laterals cluster at one station rather \
                than scattering along the axis", [0.15, 0.3]),
        },
        /// Spacing between lateral stations on the leader, in metres.
        pub leader_internode: f64 = "leaderInternode" "m" Bounds::above(0.0) => [Grow] {
            wire: 11,
            check: input(Site::Habit, 3, "leader internode"),
            dial: tuned("leader_internode", "metres between lateral stations on the leader",
                [0.05, 3.45], [0.5, 1.0], "capped").span([0.05, 3.45])
                .cap("capped both ways: the generator validates no closed range here (only \
                    positive and finite); the row stays at the preset span until the generator \
                    authors one"),
        },
        /// Laterals borne by one station of the leader.
        pub laterals_per_station: u32 = "lateralsPerStation" "laterals"
            Bounds::closed(1.0, 12.0) => [Grow] {
            wire: 12,
            check: input(Site::Habit, 4, "laterals per station"),
            applies: "needs `lateralOrders` of one or more; leader stations only",
            blend: Blend::Count,
            dial: tuned("limbs", "limbs born at each station", [1.0, 4.0], [1.0, 2.0], "authored"),
        },
        /// Initial angle of a lateral from its parent axis, in degrees; on the
        /// upright leader this is degrees from vertical.
        pub lateral_pitch: f64 = "lateralPitch" "degrees" Bounds::closed(0.0, 180.0) => [Grow] {
            wire: 13,
            check: input(Site::Habit, 5, "lateral pitch"),
            applies: "needs `lateralOrders` of one or more",
            blend: Blend::Degrees,
            dial: bounded("lateral_pitch", "the degrees a lateral leaves its parent axis, from \
                vertical on the leader", [20.0, 40.0]).span([0.0, 118.0]),
        },
        /// Spread of the lateral pitch, in degrees.
        pub pitch_variation: f64 = "pitchVariation" "degrees" Bounds::closed(0.0, 90.0) => [Grow] {
            wire: 14,
            check: input(Site::Habit, 6, "lateral pitch variation"),
            applies: "needs `lateralOrders` of one or more",
            blend: Blend::Degrees,
            dial: bounded("pitch_variation", "the degrees that departure angle varies lateral to \
                lateral", [5.0, 10.0]).span([0.0, 28.0]),
        },
        /// Degrees the lateral pitch moves from the crown's base to its top: a
        /// station's laterals leave at `lateralPitch` plus as much of this as
        /// the station stands up the crown, none at its base and all at its
        /// top. Negative sets the upper limbs more upright than the lower;
        /// zero is one pitch everywhere.
        #[cfg_attr(feature = "json", serde(default))]
        pub pitch_by_height: f64 = "pitchByHeight" "degrees"
            Bounds::closed(-180.0, 180.0) => [Grow] {
            wire: 249,
            check: value(Site::Habit, 21, "pitchByHeight"),
            applies: "needs `lateralOrders` of one or more",
            note: "The crown runs from `trunkHeight` to the envelope's height; the pitch it adds \
                to is clamped to 0 to 180 as `lateralPitch` is.",
            dial: bounded("pitch_by_height", "the degrees the lateral pitch moves from the \
                crown's base to its top; negative sets the upper limbs more upright",
                [5.0, 10.0]),
        },
        /// The most a first-order axis may stop short of the shell, as a share
        /// of its room; each axis draws its own share from its own stream, and
        /// what it bears grows within the shell scaled about its station by
        /// the share it kept. Zero reaches the shell.
        #[cfg_attr(feature = "json", serde(default))]
        pub ragged_reach: f64 = "raggedReach" "share of the room"
            Bounds::closed(0.0, 1.0) => [Grow] {
            wire: 250,
            check: value(Site::Habit, 22, "raggedReach"),
            applies: "needs `lateralOrders` of one or more: only first-order laterals probe \
                their room",
            dial: bounded("ragged_reach", "how far short of the crown's edge a limb may stop, \
                each limb by its own share", [0.05, 0.1]),
        },
        /// Signed bend over the length of a first-order axis; positive rises.
        pub rise_primary: f64 = "risePrimary" "-" Bounds::closed(-1.0, 1.0) => [Grow] {
            wire: 15,
            check: input(Site::Habit, 7, "primary rise per order"),
            note: "Acts at orders zero and one, so it bends leaning stems; an upright stem has \
                no rise to take.",
            dial: bounded("rise_primary", "the bend over a first-order limb's length; positive \
                rises", [0.025, 0.05]).span([-0.03, 0.17]),
        },
        /// Signed bend over the length of a deeper axis; negative hangs.
        pub rise_secondary: f64 = "riseSecondary" "-" Bounds::closed(-1.0, 1.0) => [Grow] {
            wire: 16,
            check: input(Site::Habit, 8, "secondary rise per order"),
            applies: "needs `lateralOrders` of two or more",
            dial: bounded("rise_secondary", "the bend over a deeper axis's length; negative \
                hangs", [0.25, 0.5]),
        },
        /// Heading change between successive growth units, in degrees.
        pub crookedness: f64 = "crookedness" "degrees" Bounds::closed(0.0, 60.0) => [Grow] {
            wire: 17,
            check: input(Site::Habit, 9, "crookedness"),
            note: "Starts above `trunkHeight` in the scaffold, and also makes the local twig \
                layer wander.",
            blend: Blend::Degrees,
            dial: tuned("crookedness", "limb turning and zigzag amount",
                [0.0, 15.0], [3.0, 6.0], "authored"),
        },
        /// Spacing between lateral stations away from the leader, in metres.
        pub lateral_spacing: f64 = "lateralSpacing" "m" Bounds::above(0.0) => [Grow] {
            wire: 18,
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
            wire: 19,
            check: input(Site::Habit, 11, "lateral length ratio"),
            growth: Growth::Differs("`apicalControlLoss` adds the dominance it releases to it"),
            applies: "needs `lateralOrders` of two or more: first-order laterals take their \
                reach from the probe",
            dial: bounded("lateral_length_ratio", "a lateral's length against the axis that \
                bears it", [0.15, 0.3]),
        },
        /// Depth of rule-built orders below the leader.
        pub lateral_orders: u32 = "lateralOrders" "orders" Bounds::closed(0.0, 8.0) => [Grow] {
            wire: 20,
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
            wire: 21,
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
            wire: 22,
            check: input(Site::Habit, 14, "twig tip taper"),
            note: "Scales half the twig diameter at every childless tip, structural or twig.",
            dial: tuned("taper", "remaining wood thickness toward the crown edge; lower means \
                thinner tips", [0.05, 0.6], [0.1, 0.2], "authored"),
        },
        /// Monthly vigour threshold; zero disables shedding. The legacy envelope
        /// builder interprets it as shell depth.
        pub shedding_threshold: f64 = "sheddingThreshold" "share of height·spread"
            Bounds::closed(0.0, 1.0) => [Grow] {
            wire: 23,
            check: input(Site::Habit, 15, "shedding threshold"),
            growth: Growth::Differs("an annual vigour threshold, not a shell depth (fn-161 \
                splits the two)"),
            applies: "zero sheds nothing",
            note: "On the direct build it is the shed shell's depth, a share of `height·spread`, \
                apart from `shellDepth`.",
            dial: bounded("shedding_threshold", "the vigour below which a shoot is shed; zero \
                sheds nothing", [0.15, 0.3]),
        },
        /// The chance, 0 to 1, that a structural axis forks codominantly:
        /// each axis draws once from its own stream when it is born. At zero
        /// no axis forks and the tree stands on one stem.
        #[cfg_attr(feature = "json", serde(default))]
        pub codominance: f64 = "codominance" "share" Bounds::closed(0.0, 1.0) => [Grow] {
            wire: 24,
            check: input(Site::Habit, 16, "codominance"),
            note: "At zero the six fork rows lie dormant. Replaces `stems`: a clump is a fork \
                at height zero.",
            dial: bounded("codominance", "how likely a trunk or limb is to fork into near-equal \
                parts; at zero no fork", [0.1, 0.25]),
        },
        /// Where a codominant fork falls, as a share of the tree's height: the
        /// centre of the bell each axis draws its fork height from. Zero is a
        /// fork at the root, which is a clump.
        #[cfg_attr(feature = "json", serde(default))]
        pub fork_height: f64 = "forkHeight" "share of height" Bounds::closed(0.0, 1.0) => [Grow] {
            wire: 25,
            check: input(Site::Habit, 17, "fork height"),
            applies: "`codominance` zero",
            note: "An axis forks where it reaches the drawn height; a height it never reaches, \
                or one at or below its own base, is no fork, except at the root.",
            dial: bounded("fork_height", "how far up the tree codominant forks fall",
                [0.05, 0.1]),
        },
        /// Width of the bell a fork height is drawn from, as a share of the
        /// tree's height. Zero forks every axis that forks at exactly
        /// `forkHeight`.
        #[cfg_attr(feature = "json", serde(default))]
        pub fork_height_spread: f64 = "forkHeightSpread" "share of height"
            Bounds::closed(0.0, 1.0) => [Grow] {
            wire: 26,
            check: input(Site::Habit, 18, "fork height spread"),
            applies: "`codominance` zero",
            dial: bounded("fork_height_spread", "how widely codominant forks scatter about their \
                height", [0.05, 0.1]),
        },
        /// The parts a codominant fork divides into, the primary among them, 2
        /// to 4. The whole part is the full parts; the fraction grows one more
        /// in from the fork, its length and its wood that share of a whole
        /// part's.
        #[cfg_attr(feature = "json", serde(default = "crate::ranges::default_fork_ways"))]
        pub fork_ways: f64 = "forkWays" "parts" Bounds::closed(2.0, 4.0) => [Grow] {
            wire: 27,
            check: input(Site::Habit, 19, "fork ways"),
            applies: "`codominance` zero",
            note: "Replaces `stems`. The parts fill four fixed slots in order, so a part \
                added never moves the others.",
            dial: bounded("fork_ways", "how many near-equal parts a codominant fork divides into",
                [0.5, 1.0]),
        },
        /// Degrees of bearing between neighbouring slots of a fork, slot `k`
        /// standing `k` times this round from the primary's, about a bearing
        /// the axis's own stream decides. At zero every part leaves on one
        /// bearing.
        #[cfg_attr(feature = "json", serde(default = "crate::ranges::default_fork_divergence"))]
        pub fork_divergence: f64 = "forkDivergence" "degrees"
            Bounds::closed(0.0, 120.0) => [Grow] {
            wire: 28,
            check: input(Site::Habit, 20, "fork divergence"),
            applies: "`codominance` zero",
            note: "Defaults to 90 so that `codominance` builds on its own: fn-170's plane \
                candidate stands its parts 120 apart, but at 120 a fourth part returns onto the \
                primary's bearing, and a quarter turn keeps all four apart at every `forkWays`. \
                Refused where two parts of a fork leave on one heading (`forks_placed`). \
                Replaces `stemDivergence`.",
            blend: Blend::Degrees,
            dial: bounded("fork_divergence", "the degrees of bearing between neighbouring parts \
                of a codominant fork", [5.0, 10.0]),
        },
        /// Degrees every sibling of a fork leans from the axis it leaves; the
        /// primary leans by what `forkLeanSpread` leaves it of this.
        #[cfg_attr(feature = "json", serde(default = "crate::ranges::default_fork_lean"))]
        pub fork_lean: f64 = "forkLean" "degrees" Bounds::closed(0.0, 45.0) => [Grow] {
            wire: 251,
            check: input(Site::Habit, 23, "fork lean"),
            applies: "`codominance` zero",
            note: "Defaults to 26 so that `codominance` builds on its own: fn-170's plane \
                candidate's lean, two degrees under the 28 the birch's photographs show. \
                Measured from the heading of the axis at the fork. Replaces `stemLean`.",
            blend: Blend::Degrees,
            dial: bounded("fork_lean", "the degrees the parts of a codominant fork lean from \
                the axis they leave", [5.0, 10.0]),
        },
        /// How far a fork's primary stands back along its axis, 0 to 1. None of
        /// it leans the primary by all of `forkLean`, as every sibling leans;
        /// all of it carries the axis straight on through the fork.
        #[cfg_attr(feature = "json", serde(default))]
        pub fork_lean_spread: f64 = "forkLeanSpread" "share"
            Bounds::closed(0.0, 1.0) => [Grow] {
            wire: 252,
            check: input(Site::Habit, 24, "fork lean spread"),
            applies: "`codominance` zero, or `forkLean` zero",
            note: "Replaces `stemLeanSpread`.",
            dial: bounded("fork_lean_spread", "how far the part carrying the axis on stands \
                straight while the others lean", [0.15, 0.3]),
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
            pitch_by_height: 0.0,
            ragged_reach: 0.0,
            rise_primary: 0.05,
            rise_secondary: 0.0,
            crookedness: 12.0,
            lateral_spacing: 0.9,
            lateral_length_ratio: 0.4,
            lateral_orders: 3,
            attractor_weight: 1.0,
            twig_tip_taper: 1.0,
            shedding_threshold: 0.45,
            codominance: 0.0,
            fork_height: 0.0,
            fork_height_spread: 0.0,
            fork_ways: 2.0,
            fork_divergence: crate::ranges::default_fork_divergence(),
            fork_lean: crate::ranges::default_fork_lean(),
            fork_lean_spread: 0.0,
        }
    }
}
impl HabitParams {
    pub fn validate(&self) -> Result<()> {
        crate::catalogue::check(Self::CHECKS, self, Site::Habit)
    }
}
