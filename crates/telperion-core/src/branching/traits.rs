//! The habit trait table: every architectural dial the scaffold reads, numeric
//! and present on every family. Named models are regions of this space.
use crate::{Error, Result};

/// Architectural traits, all numeric and present on every family. Named models
/// are regions of this space, never variants of it.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HabitParams {
    /// How far the leader persists into the crown, 0 to 1.
    pub apical_dominance: f64,
    /// Clustering of laterals at a station against scattering along the axis.
    pub whorl_strength: f64,
    /// Spacing between lateral stations on the leader, in metres.
    pub leader_internode: f64,
    /// Laterals borne by one station of the leader.
    pub laterals_per_station: u32,
    /// Initial angle of a lateral from its parent axis, in degrees; on the
    /// upright leader this is degrees from vertical.
    pub lateral_pitch: f64,
    /// Spread of the lateral pitch, in degrees.
    pub pitch_variation: f64,
    /// Signed bend over the length of a first-order axis; positive rises.
    pub rise_primary: f64,
    /// Signed bend over the length of a deeper axis; negative hangs.
    pub rise_secondary: f64,
    /// Heading change between successive growth units, in degrees.
    pub crookedness: f64,
    /// Spacing between lateral stations away from the leader, in metres.
    pub lateral_spacing: f64,
    /// Length of a lateral against its supporting axis.
    pub lateral_length_ratio: f64,
    /// Depth of rule-built orders below the leader.
    pub lateral_orders: u32,
    /// Weight of the attractor pull beside the axis's own rule heading.
    pub attractor_weight: f64,
    /// Distal twig radius against the nominal twig radius.
    pub twig_tip_taper: f64,
    /// Interior shedding after the scaffold; 0 skips the pass.
    pub shedding_threshold: f64,
}
impl Default for HabitParams {
    fn default() -> Self {
        Self {
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
        }
    }
}
impl HabitParams {
    pub fn validate(&self) -> Result<()> {
        let unit = |v: f64| v.is_finite() && (0.0..=1.0).contains(&v);
        let signed = |v: f64| v.is_finite() && (-1.0..=1.0).contains(&v);
        let positive = |v: f64| v.is_finite() && v > 0.0;
        for (valid, trait_name) in [
            (unit(self.apical_dominance), "apical dominance"),
            (unit(self.whorl_strength), "whorl strength"),
            (positive(self.leader_internode), "leader internode"),
            (
                (1..=12).contains(&self.laterals_per_station),
                "laterals per station",
            ),
            (
                self.lateral_pitch.is_finite() && (0.0..=180.0).contains(&self.lateral_pitch),
                "lateral pitch",
            ),
            (
                self.pitch_variation.is_finite() && (0.0..=90.0).contains(&self.pitch_variation),
                "lateral pitch variation",
            ),
            (signed(self.rise_primary), "primary rise per order"),
            (signed(self.rise_secondary), "secondary rise per order"),
            (
                self.crookedness.is_finite() && (0.0..=60.0).contains(&self.crookedness),
                "crookedness",
            ),
            (positive(self.lateral_spacing), "lateral spacing"),
            (unit(self.lateral_length_ratio), "lateral length ratio"),
            (self.lateral_orders <= 8, "lateral orders"),
            (unit(self.attractor_weight), "attractor weight"),
            (unit(self.twig_tip_taper), "twig tip taper"),
            (unit(self.shedding_threshold), "shedding threshold"),
        ] {
            if !valid {
                return Err(Error::InvalidInput(trait_name));
            }
        }
        Ok(())
    }
}
