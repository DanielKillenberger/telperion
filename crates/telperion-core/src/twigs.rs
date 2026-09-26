//! Fixed shoot anatomy and finite authoring rails for the local branch law.
use crate::catalogue::{bounded, input, tuned, value, Blend, Bounds, Dial, Growth, Site};
use crate::math::Transcendental;
use crate::Result;
/// Top of the `generations` rail, and its neutral.
pub const MAX_GENERATIONS: u32 = 6;
crate::catalogue::rows! {
    #[derive(Debug, Clone, Copy, PartialEq)]
    #[cfg_attr(feature = "json", derive(serde::Serialize, serde::Deserialize))]
    pub struct TwigAnatomy in "/skeleton/twigs/twig" {
        /// The finished thickness of a twig in metres. Wood at or below half
        /// of it is drawn as a twig, so raising it thickens the twig layer and
        /// hands more of the fine wood to it.
        pub diameter: f64 = "diameter" "m" Bounds::closed(1e-6, 1e6) => [Grow, Plan] {
            check: value(Site::Twig, 0, "twig diameter"),
            note: "Wood at or below half of it is a twig; it is also the twig radius, the cap on \
                childless tips (with `twigTipTaper`), and the leaf box's size where \
                `canopy.shootRadius` is zero.",
            dial: bounded("twig_diameter", "the finished thickness of a twig, in metres",
                [0.001, 0.002]).span([0.0005, 0.0065]),
        },
        /// The length in metres a twig shoot grows before it stops, and the
        /// whole of one internode on leaf-bearing wood. Raising it lengthens
        /// every twig, so the crown carries a deeper, shaggier skin.
        pub length: f64 = "length" "m" Bounds::closed(1e-6, 1e6) => [Grow] {
            check: value(Site::Twig, 1, "twig length"),
            growth: Growth::Differs("also sets how far a waiting shoot reaches"),
            note: "The internode floor on bearing wood; a structural run the shell stops is \
                trimmed by it.",
            dial: bounded("twig_length", "the metres a twig shoot grows before it stops",
                [0.05, 0.1]).span([0.15, 0.55]),
        },
        /// Metres between the joints on wood thicker than the bearing
        /// diameter, and the spacing of the stations a leaf sits on. Raising
        /// it gives longer segments, so laterals and leaves sit further apart.
        pub internode_length: f64 = "internodeLength" "m"
            Bounds::closed(1e-6, 1e6) => [Grow, Plan, Expand] {
            check: value(Site::Twig, 2, "twig internodeLength"),
            note: "Also the spacing of leaf stations; the canopy check bounds it again at [1e-6, \
                1e6] (`twig internode`).",
            dial: bounded("twig_internode_length", "metres between the joints on wood thicker \
                than the bearing diameter", [0.015, 0.03]).span([1e-06, 0.08875]),
        },
        /// How many leaf stations sit at each joint, each turned its own share
        /// of a full turn around the shoot. Raising it crowds more leaves onto
        /// the same joints.
        pub stations_per_internode: u32 = "stationsPerInternode" "stations"
            Bounds::closed(1.0, 32.0) => [Plan, Expand] {
            check: value(Site::Twig, 4, "twig stationsPerInternode"),
            note: "Conflicting bounds: the canopy check admits 1 to 64 (`twig stations`), and \
                this check's 1 to 32 answers first. The skeleton never reads it; with \
                `canopy.divergence` it sets the phyllotaxis.",
            blend: Blend::Count,
            dial: bounded("twig_stations_per_internode", "leaf stations at each joint, spread \
                around the shoot", [1.0, 2.0]).span([1.0, 4.0]),
        },
        /// The thickness in metres at or below which a shoot bears leaves and
        /// side shoots of its own. Raising it lets thicker wood bear, so
        /// foliage reaches further back down the branch.
        pub bearing_diameter: f64 = "bearingDiameter" "m" Bounds::closed(1e-6, 1e6) => [Grow] {
            check: value(Site::Twig, 3, "twig bearingDiameter"),
            growth: Growth::Differs("also marks leaf-bearing wood and caps `canopy.shootRadius`"),
            note: "Wood at or below half of it takes one lateral per internode, ignoring \
                `laterals`, and `twig.length` as its internode floor.",
            dial: bounded("twig_bearing_diameter", "the thickness in metres at or below which a \
                shoot bears leaves and side shoots", [0.01, 0.02]).span([0.005, 0.065]),
        },
    }
}
impl Default for TwigAnatomy {
    fn default() -> Self {
        Self {
            diameter: 0.005,
            length: 0.25,
            internode_length: 0.02,
            stations_per_internode: 1,
            bearing_diameter: 0.05,
        }
    }
}
crate::catalogue::rows! {
    #[derive(Debug, Clone, Copy, PartialEq)]
    #[cfg_attr(feature = "json", derive(serde::Serialize, serde::Deserialize))]
    pub struct TwigParams in "/skeleton/twigs" {
        pub twig: TwigAnatomy,
        /// A shoot's length as a share of the one that bore it. Raising it
        /// makes each generation of twigs longer relative to its parent.
        pub length_ratio: f64 = "lengthRatio" "share"
            crate::ranges::LENGTH_RATIO.bounds() => [Grow] {
            check: value(Site::Twig, 0, "twig lengthRatio"),
            note: "With `ratioPower` it sets the child radius; a varied draw is clamped to the \
                bounds.",
            dial: bounded("twig_length_ratio", "a shoot's length as a share of the one that bore \
                it", [0.15, 0.3]),
        },
        /// How much thinner a shoot is than its parent for the same drop in
        /// length. Raising it leaves side shoots finer.
        pub ratio_power: f64 = "ratioPower" "-" Bounds::closed(0.0, 8.0) => [Grow] {
            check: value(Site::Twig, 1, "twig ratioPower"),
            dial: bounded("twig_ratio_power", "how much finer a shoot is than its parent for the \
                same drop in length", [1.0, 2.0]),
        },
        /// The fewest of its own diameters a segment of wood may span. Raising
        /// it makes segments longer for the same thickness, so there are fewer
        /// joints and the wood reads straighter.
        pub internode_factor: f64 = "internodeFactor" "-" Bounds::closed(0.05, 32.0) => [Grow] {
            check: value(Site::Twig, 2, "twig internodeFactor"),
            dial: bounded("twig_internode_factor", "the fewest of its own diameters a segment of \
                wood may span", [0.5, 1.0]).span([1.25, 3.75]),
        },
        /// The ceiling on segments one length of wood may be cut into. It
        /// binds only where the two lengths above would cut more, and there it
        /// caps the cost rather than states a look.
        #[cfg_attr(feature = "json", serde(default = "crate::ranges::default_max_internodes"))]
        pub max_internodes: u32 = "maxInternodes" "internodes"
            crate::ranges::POSITIVE_COUNT.bounds() => [Grow] {
            check: value(Site::Twig, 9, "twig maxInternodes"),
            blend: Blend::Count,
            dial: Dial::Excluded("A cost cap: it binds only where the two internode lengths \
                would cut more segments (crates/telperion-core/src/twigs.rs:221)."),
        },
        /// How many side shoots leave each station along a twig. Raising it
        /// crowds more twigs onto the same length of wood.
        pub laterals: u32 = "laterals" "laterals" Bounds::closed(0.0, 7.0) => [Grow] {
            check: value(Site::Twig, 3, "twig laterals"),
            growth: Growth::Differs("also sizes the rollback checkpoint"),
            note: "Held as a bitmask, which is why it stops at 7; ignored on bearing wood; at \
                zero non-bearing shoots may sleep.",
            blend: Blend::Count,
            dial: bounded("twig_laterals", "side shoots leaving each station along a twig",
                [1.0, 2.0]),
        },
        /// Twig-law generations of branching, 1 to 6. A lateral born at or past
        /// this generation is a twig whatever the pipe model left its radius, so
        /// the twig layer's depth is a row a table states rather than a
        /// consequence of how thick the wood is.
        pub generations: u32 = "generations" "generations"
            Bounds::closed(1.0, MAX_GENERATIONS as f64) => [Grow] {
            check: input(Site::Twig, 13, "twig generations"),
            blend: Blend::Count,
            dial: bounded("twig_generations", "generations of twig branching below the limbs",
                [1.0, 2.0]),
        },
        /// The share of the trunk's radius at or below which wood starts
        /// bearing twigs. Raising it lets twigs start on thicker wood, so
        /// they reach further back toward the trunk.
        pub limb_radius: f64 = "limbRadius" "share of root radius"
            Bounds::closed(0.0, 1.0) => [Grow] {
            check: value(Site::Twig, 4, "twig limbRadius"),
            note: "Measured against the thickest stem where laterals are seeded and against the \
                root node where they advance.",
            dial: bounded("twig_limb_radius", "the share of the trunk's radius at or below which \
                wood starts bearing twigs", [0.15, 0.3]),
        },
        /// How deep the outer skin of the crown is that only twigs may fill,
        /// as a share of the crown: the scaffold is grown into what is left
        /// inside it. Raising it holds the structural wood further in and
        /// leaves a deeper twig layer.
        pub reach: f64 = "reach" "share" Bounds::closed(0.0, 0.9) => [Grow] {
            check: value(Site::Twig, 5, "twig reach"),
            note: "Shrinks the scaffold's room and, with `attractorWeight` above zero, the \
                attractor volume; the twig layer itself is not bound by it.",
            dial: bounded("twig_reach", "the depth of the crown's outer skin that only twigs \
                fill", [0.15, 0.3]),
        },
        /// The degrees a side shoot leaves its parent. Raising it swings
        /// twigs further out toward square with the branch.
        pub angle: f64 = "angle" "degrees" crate::ranges::ANGLE.bounds() => [Grow] {
            check: value(Site::Twig, 6, "twig angle"),
            note: "Also sets the laterals' separation, `min(angle, maxTurnPerStep)`; a varied \
                draw is clamped.",
            blend: Blend::Degrees,
            dial: bounded("twig_angle", "the degrees a side shoot leaves its parent",
                [1.5, 3.0]).span([37.5, 47.5]),
        },
        /// How many degrees that departure angle varies shoot to shoot.
        /// Raising it makes the twig layer less uniform.
        pub angle_variation: f64 = "angleVariation" "degrees"
            crate::ranges::ANGLE.bounds() => [Grow] {
            check: value(Site::Twig, 7, "twig angleVariation"),
            blend: Blend::Degrees,
            dial: bounded("twig_angle_variation", "the degrees that departure angle varies shoot \
                to shoot", [1.5, 3.0]).span([5.0, 15.0]),
        },
        /// How much shoot length varies shoot to shoot. Raising it gives a
        /// more uneven, less combed twig layer.
        pub vigour_variation: f64 = "vigourVariation" "share" Bounds::closed(0.0, 0.95) => [Grow] {
            check: value(Site::Twig, 8, "twig vigourVariation"),
            dial: bounded("twig_vigour_variation", "how far shoot length varies shoot to shoot",
                [0.15, 0.3]),
        },
        /// The degrees each successive shoot is turned around the wood that
        /// bears it. Raising it swings the next shoot further around, so the
        /// twigs spiral differently.
        pub divergence: f64 = "divergence" "degrees" Bounds::FINITE => [Grow] {
            check: input(Site::Twig, 12, "twig divergence"),
            blend: Blend::Degrees,
            dial: tuned("twig_divergence", "the degrees each successive shoot is turned around \
                the wood that bears it",
                [68.752, 206.256], [20.0, 40.0], "capped").span([68.752, 206.256])
                .cap("capped both ways: the generator validates no closed range here (only \
                    finite); the row stays at the preset span until the generator authors one"),
        },
        /// How strongly a shoot hangs, 0 to 3. At 0 nothing hangs and the local
        /// law is the ordinary one; at 1 a curtain takes its full droop.
        pub hang: f64 = "hang" "-" Bounds::closed(0.0, 3.0) => [Grow] {
            check: input(Site::Twig, 14, "hang"),
            note: "Gates the curtain: the pendulous, separation, sag, variation, droop and \
                step-clearance rows do nothing at zero. Above one it scales droop and \
                separation, while the floor, drop and length terms take one.",
            dial: bounded("twig_hang", "how strongly a shoot hangs; zero hangs nothing",
                [0.5, 1.0]),
        },
        /// Metres a pendulous shoot grows before it stops, and the length its
        /// droop reaches the cap over.
        pub pendulous_length: f64 = "pendulousLength" "m" Bounds::closed(0.05, 5.0) => [Grow] {
            check: input(Site::Twig, 15, "pendulous length"),
            applies: "`hang` zero",
            note: "Without `sag` it caps a hanging run; with it, it is the run's length.",
            dial: bounded("twig_pendulous_length", "the metres a hanging shoot grows before it \
                stops", [1.0, 2.0]),
        },
        /// Fraction of the root radius at or below which a station's shoots hang.
        pub pendulous_radius: f64 = "pendulousRadius" "share of stem radius"
            Bounds::closed(0.0, 1.0) => [Grow] {
            check: input(Site::Twig, 16, "pendulous radius"),
            applies: "`hang` zero",
            dial: bounded("twig_pendulous_radius", "the share of the root radius at or below \
                which a station's shoots hang", [0.15, 0.3]),
        },
        /// Degrees between neighbouring shoots in a curtain.
        pub curtain_separation: f64 = "curtainSeparation" "degrees"
            Bounds::closed(1.0, 45.0) => [Grow] {
            check: input(Site::Twig, 17, "curtain separation"),
            applies: "`hang` zero",
            blend: Blend::Degrees,
            dial: bounded("twig_curtain_separation", "the degrees between neighbouring shoots in \
                a curtain", [1.5, 3.0]).span([1.5, 11.5]),
        },
        /// How far toward straight down a hanging shoot's course has turned by the
        /// end of its pendulous length, 0 to 1: at 0 the shoot holds the direction
        /// it departed with and the curtain is a set of rods, at 1 it hangs
        /// vertical, and the turn is spread along the run as an arc steepest at the
        /// wood that bears it.
        pub sag: f64 = "sag" "share" Bounds::closed(0.0, 1.0) => [Grow] {
            check: input(Site::Twig, 18, "sag"),
            applies: "`hang` zero",
            dial: bounded("twig_sag", "how far a hanging shoot's course has turned toward \
                straight down by its end; at zero a hanging shoot holds the direction it left on \
                and the curtain is a set of rods, and any rise starts the turn", [0.15, 0.3]),
        },
        /// How much shorter than the pendulous length a hanging shoot may run, 0
        /// to 1: each shoot's own length is the pendulous length times one minus
        /// this times a draw in 0 to 1 keyed by the shoot and the seed. At 0 every
        /// shoot has the one length the table states.
        pub pendulous_variation: f64 = "pendulousVariation" "share"
            Bounds::closed(0.0, 1.0) => [Grow] {
            check: input(Site::Twig, 19, "pendulous variation"),
            applies: "`hang` zero",
            dial: bounded("twig_pendulous_variation", "how much shorter than the pendulous \
                length a hanging shoot may run; at zero every shoot runs the one length the \
                table states, and any rise starts the variation", [0.15, 0.3]),
        },
        /// How far below the shell's lower surface a hanging shoot may fall, 0 to
        /// 1, as a share of the way from that surface down to the clearance: at 0
        /// the shell binds a hanging shoot as it binds every other, at 1 the shoot
        /// may fall to the clearance. Only a shoot that hangs, only under the
        /// crown's footprint.
        pub curtain_drop: f64 = "curtainDrop" "share" Bounds::closed(0.0, 1.0) => [Grow] {
            check: input(Site::Twig, 20, "curtain drop"),
            note: "Above zero admits wood below the shell and stops shoots sleeping.",
            dial: bounded("twig_curtain_drop", "how far below the crown's own surface a hanging \
                shoot may fall; at zero the shell binds a hanging shoot as it binds every other, \
                and any rise starts the fall below it", [0.15, 0.3]),
        },
        /// Metres above the ground no hanging shoot falls below, 0 to 5. Never
        /// above the crown's own base, whatever the row says.
        pub curtain_clearance: f64 = "curtainClearance" "m" Bounds::closed(0.0, 5.0) => [Grow] {
            check: input(Site::Twig, 21, "curtain clearance"),
            applies: "unless `sag` or `curtainDrop` is above zero",
            note: "Capped by `trunkHeight` in the floor and by `height·crownBase` in the band.",
            dial: bounded("twig_curtain_clearance", "the metres above the ground no hanging \
                shoot falls below", [1.0, 2.0]),
        },
        /// The furthest a hanging shoot may bend toward straight down.
        /// Raising it lets curtains hang more heavily.
        #[cfg_attr(feature = "json", serde(default = "crate::ranges::default_max_droop"))]
        pub max_droop: f64 = "maxDroop" "-" Bounds::closed(0.0, 10.0) => [Grow] {
            check: value(Site::Twig, 10, "twig maxDroop"),
            applies: "`hang` zero",
            dial: bounded("twig_max_droop", "the furthest a hanging shoot may bend toward \
                straight down", [1.5, 3.0]),
        },
        /// How close to the ground a hanging curtain may reach before it
        /// stops growing. Raising it lets curtains hang nearer the floor.
        #[cfg_attr(
            feature = "json",
            serde(default = "crate::ranges::default_curtain_step_clearance")
        )]
        pub curtain_step_clearance: f64 = "curtainStepClearance" "share"
            Bounds::closed(0.0, 1.0) => [Grow] {
            check: value(Site::Twig, 11, "twig curtainStepClearance"),
            applies: "`hang` zero",
            dial: bounded("twig_curtain_step_clearance", "how close to the ground a hanging \
                curtain may reach before it stops", [0.15, 0.3]),
        },
    }
}
impl Default for TwigParams {
    fn default() -> Self {
        Self {
            twig: TwigAnatomy::default(),
            length_ratio: 0.4,
            ratio_power: 1.3,
            internode_factor: 2.5,
            max_internodes: crate::ranges::default_max_internodes(),
            laterals: 2,
            // Neutral: the deepest shipped tree branches four generations, so
            // the top of the rail reproduces every one of them to the byte.
            generations: MAX_GENERATIONS,
            limb_radius: 0.1,
            reach: 0.2,
            angle: 45.0,
            angle_variation: 10.0,
            vigour_variation: 0.15,
            divergence: 137.508,
            // Neutral: no tree hangs until a table says so. The other three
            // state the magnitudes the curtain had while it was a constant,
            // so a table that turns hang on reproduces them.
            hang: 0.0,
            pendulous_length: 0.25,
            pendulous_radius: 1.0,
            curtain_separation: 4.0,
            // Neutral: nothing gives in to its own weight until a table says
            // so, so a curtain is the straight rod it was before this row.
            sag: 0.0,
            // Neutral: every hanging shoot has the one pendulous length.
            pendulous_variation: 0.0,
            // Neutral: the shell holds the curtain, as it did before the row.
            curtain_drop: 0.0,
            curtain_clearance: 0.5,
            max_droop: crate::ranges::default_max_droop(),
            curtain_step_clearance: crate::ranges::default_curtain_step_clearance(),
        }
    }
}
impl TwigParams {
    pub fn resolved(self) -> Result<Self> {
        crate::catalogue::check(TwigAnatomy::ROWS, &self.twig, Site::Twig)?;
        crate::catalogue::check(Self::ROWS, &self, Site::Twig)?;
        Ok(self)
    }
    pub(crate) fn internodes(self, radius: f64, length: f64) -> usize {
        let floor = if radius <= self.twig.bearing_diameter / 2.0 {
            self.twig.length
        } else {
            self.twig.internode_length
        };
        (length
            / floor
                .max(self.internode_factor * 2.0 * radius)
                .max(length / self.max_internodes as f64))
        .round()
        .max(1.0) as usize
    }
}
pub fn branch_length(radius: f64) -> f64 {
    148.0 / 7.4_f64.powf_fixed(2.0 / 3.0) * radius.max(0.0).powf_fixed(2.0 / 3.0)
}
pub fn child_radius(radius: f64, ratio: f64, power: f64) -> f64 {
    radius * ratio.powf_fixed(power)
}

#[cfg(test)]
mod limit_tests {
    use super::*;
    #[test]
    fn internode_budget_is_authored() {
        for max_internodes in [16, 32, 64, 100] {
            let t = TwigParams {
                max_internodes,
                ..Default::default()
            }
            .resolved()
            .unwrap();
            assert_eq!(t.internodes(0.1, 1000.), max_internodes as usize);
        }
    }
}
