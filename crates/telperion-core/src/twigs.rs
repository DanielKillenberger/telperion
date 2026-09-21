//! Fixed shoot anatomy and finite authoring rails for the local branch law.
use crate::math::Transcendental;
use crate::{Error, Result};
/// Top of the `generations` rail, and its neutral.
pub const MAX_GENERATIONS: u32 = 6;
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "json", derive(serde::Serialize, serde::Deserialize))]
pub struct TwigAnatomy {
    pub diameter: f64,
    pub length: f64,
    pub internode_length: f64,
    pub stations_per_internode: u32,
    pub bearing_diameter: f64,
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
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "json", derive(serde::Serialize, serde::Deserialize))]
pub struct TwigParams {
    pub twig: TwigAnatomy,
    /// A shoot's length as a share of the one that bore it. Raising it
    /// makes each generation of twigs longer relative to its parent.
    pub length_ratio: f64,
    /// How much thinner a shoot is than its parent for the same drop in
    /// length. Raising it leaves side shoots finer.
    pub ratio_power: f64,
    pub internode_factor: f64,
    #[cfg_attr(
        feature = "json",
        serde(default = "crate::ranges::default_max_internodes")
    )]
    pub max_internodes: u32,
    /// How many side shoots leave each station along a twig. Raising it
    /// crowds more twigs onto the same length of wood.
    pub laterals: u32,
    /// Twig-law generations of branching, 1 to 6. A lateral born at or past
    /// this generation is a twig whatever the pipe model left its radius, so
    /// the twig layer's depth is a row a table states rather than a
    /// consequence of how thick the wood is.
    pub generations: u32,
    /// The share of the trunk's radius at or below which wood starts
    /// bearing twigs. Raising it lets twigs start on thicker wood, so
    /// they reach further back toward the trunk.
    pub limb_radius: f64,
    pub reach: f64,
    /// The degrees a side shoot leaves its parent. Raising it swings
    /// twigs further out toward square with the branch.
    pub angle: f64,
    /// How many degrees that departure angle varies shoot to shoot.
    /// Raising it makes the twig layer less uniform.
    pub angle_variation: f64,
    /// How much shoot length varies shoot to shoot. Raising it gives a
    /// more uneven, less combed twig layer.
    pub vigour_variation: f64,
    pub divergence: f64,
    /// How strongly a shoot hangs, 0 to 3. At 0 nothing hangs and the local
    /// law is the ordinary one; at 1 a curtain takes its full droop.
    pub hang: f64,
    /// Metres a pendulous shoot grows before it stops, and the length its
    /// droop reaches the cap over.
    pub pendulous_length: f64,
    /// Fraction of the root radius at or below which a station's shoots hang.
    pub pendulous_radius: f64,
    /// Degrees between neighbouring shoots in a curtain.
    pub curtain_separation: f64,
    /// How far toward straight down a hanging shoot's course has turned by the
    /// end of its pendulous length, 0 to 1: at 0 the shoot holds the direction
    /// it departed with and the curtain is a set of rods, at 1 it hangs
    /// vertical, and the turn is spread along the run as an arc steepest at the
    /// wood that bears it.
    pub sag: f64,
    /// How much shorter than the pendulous length a hanging shoot may run, 0
    /// to 1: each shoot's own length is the pendulous length times one minus
    /// this times a draw in 0 to 1 keyed by the shoot and the seed. At 0 every
    /// shoot has the one length the table states.
    pub pendulous_variation: f64,
    /// How far below the shell's lower surface a hanging shoot may fall, 0 to
    /// 1, as a share of the way from that surface down to the clearance: at 0
    /// the shell binds a hanging shoot as it binds every other, at 1 the shoot
    /// may fall to the clearance. Only a shoot that hangs, only under the
    /// crown's footprint.
    pub curtain_drop: f64,
    /// Metres above the ground no hanging shoot falls below, 0 to 5. Never
    /// above the crown's own base, whatever the row says.
    pub curtain_clearance: f64,
    /// The furthest a hanging shoot may bend toward straight down.
    /// Raising it lets curtains hang more heavily.
    #[cfg_attr(feature = "json", serde(default = "crate::ranges::default_max_droop"))]
    pub max_droop: f64,
    /// How close to the ground a hanging curtain may reach before it
    /// stops growing. Raising it lets curtains hang nearer the floor.
    #[cfg_attr(
        feature = "json",
        serde(default = "crate::ranges::default_curtain_step_clearance")
    )]
    pub curtain_step_clearance: f64,
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
        use crate::ranges::*;
        let t = &self.twig;
        for (value, range, field) in [
            (t.diameter, ANATOMY, "twig diameter"),
            (t.length, ANATOMY, "twig length"),
            (t.internode_length, ANATOMY, "twig internodeLength"),
            (t.bearing_diameter, ANATOMY, "twig bearingDiameter"),
            (
                t.stations_per_internode as f64,
                STATIONS,
                "twig stationsPerInternode",
            ),
            (self.length_ratio, LENGTH_RATIO, "twig lengthRatio"),
            (self.ratio_power, RATIO_POWER, "twig ratioPower"),
            (
                self.internode_factor,
                INTERNODE_FACTOR,
                "twig internodeFactor",
            ),
            (self.laterals as f64, LATERALS, "twig laterals"),
            (self.limb_radius, UNIT, "twig limbRadius"),
            (self.reach, REACH, "twig reach"),
            (self.angle, ANGLE, "twig angle"),
            (self.angle_variation, ANGLE, "twig angleVariation"),
            (self.vigour_variation, VIGOUR, "twig vigourVariation"),
            (
                self.max_internodes as f64,
                POSITIVE_COUNT,
                "twig maxInternodes",
            ),
            (self.max_droop, MAX_DROOP, "twig maxDroop"),
            (
                self.curtain_step_clearance,
                UNIT,
                "twig curtainStepClearance",
            ),
        ] {
            range.check(value, field)?;
        }
        if !self.divergence.is_finite() {
            return Err(Error::InvalidInput("twig divergence"));
        }
        // A generation count outside the rail is a table with a mistake in
        // it, not a value to round into range: the depth of the twig layer is
        // what the table is choosing here.
        if !(1..=MAX_GENERATIONS).contains(&self.generations) {
            return Err(Error::InvalidInput("twig generations"));
        }
        // The curtain's eight rows are refused rather than clamped: a table
        // that asks for a droop or a separation outside the rail is a table
        // with a mistake in it, and the mistake is named.
        for (v, low, high, row) in [
            (self.hang, 0.0, 3.0, "hang"),
            (self.pendulous_length, 0.05, 5.0, "pendulous length"),
            (self.pendulous_radius, 0.0, 1.0, "pendulous radius"),
            (self.curtain_separation, 1.0, 45.0, "curtain separation"),
            (self.sag, 0.0, 1.0, "sag"),
            (self.pendulous_variation, 0.0, 1.0, "pendulous variation"),
            (self.curtain_drop, 0.0, 1.0, "curtain drop"),
            (self.curtain_clearance, 0.0, 5.0, "curtain clearance"),
        ] {
            if !v.is_finite() || !(low..=high).contains(&v) {
                return Err(Error::InvalidInput(row));
            }
        }
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
