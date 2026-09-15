//! Fixed shoot anatomy and finite authoring rails for the local branch law.
use crate::math::Transcendental;
use crate::{Error, Result};
pub const MAX_LEVELS: usize = 12;
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
    pub length_ratio: f64,
    pub ratio_power: f64,
    pub internode_factor: f64,
    pub laterals: u32,
    /// Twig-law generations of branching, 1 to 6. A lateral born at or past
    /// this generation is a twig whatever the pipe model left its radius, so
    /// the twig layer's depth is a row a table states rather than a
    /// consequence of how thick the wood is. `MAX_LEVELS` stays behind it as
    /// the structural stop the rail can no longer reach.
    pub generations: u32,
    pub limb_radius: f64,
    pub reach: f64,
    pub angle: f64,
    pub angle_variation: f64,
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
}
impl Default for TwigParams {
    fn default() -> Self {
        Self {
            twig: TwigAnatomy::default(),
            length_ratio: 0.4,
            ratio_power: 1.3,
            internode_factor: 2.5,
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
        }
    }
}
impl TwigParams {
    pub fn resolved(mut self) -> Result<Self> {
        let t = &mut self.twig;
        if ![
            t.diameter,
            t.length,
            t.internode_length,
            t.bearing_diameter,
            self.length_ratio,
            self.ratio_power,
            self.internode_factor,
            self.limb_radius,
            self.reach,
            self.angle,
            self.angle_variation,
            self.vigour_variation,
            self.divergence,
        ]
        .iter()
        .all(|v| v.is_finite())
        {
            return Err(Error::InvalidInput("twig parameters"));
        }
        t.diameter = t.diameter.clamp(1e-6, 1e6);
        t.length = t.length.clamp(1e-6, 1e6);
        t.internode_length = t.internode_length.clamp(1e-6, 1e6);
        t.bearing_diameter = t.bearing_diameter.clamp(1e-6, 1e6);
        t.stations_per_internode = t.stations_per_internode.clamp(1, 32);
        self.length_ratio = self.length_ratio.clamp(0.05, 1.0);
        self.ratio_power = self.ratio_power.clamp(0.0, 8.0);
        self.internode_factor = self.internode_factor.clamp(0.05, 32.0);
        self.laterals = self.laterals.min(7);
        self.limb_radius = self.limb_radius.clamp(0.0, 1.0);
        self.reach = self.reach.clamp(0.0, 0.9);
        self.angle = self.angle.clamp(0.0, 90.0);
        self.angle_variation = self.angle_variation.clamp(0.0, 90.0);
        self.vigour_variation = self.vigour_variation.clamp(0.0, 0.95);
        // A generation count outside the rail is a table with a mistake in
        // it, not a value to round into range: the depth of the twig layer is
        // what the table is choosing here.
        if !(1..=MAX_GENERATIONS).contains(&self.generations) {
            return Err(Error::InvalidInput("twig generations"));
        }
        // The curtain's five rows are refused rather than clamped: a table
        // that asks for a droop or a separation outside the rail is a table
        // with a mistake in it, and the mistake is named.
        for (v, low, high, row) in [
            (self.hang, 0.0, 3.0, "hang"),
            (self.pendulous_length, 0.05, 5.0, "pendulous length"),
            (self.pendulous_radius, 0.0, 1.0, "pendulous radius"),
            (self.curtain_separation, 1.0, 45.0, "curtain separation"),
            (self.sag, 0.0, 1.0, "sag"),
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
                .max(length / 32.0))
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
