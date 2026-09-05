//! Fixed shoot anatomy and finite authoring rails for the local branch law.
use crate::{Error, Result};
pub const MAX_LEVELS: usize = 12;
#[derive(Debug, Clone, Copy, PartialEq)]
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
pub struct TwigParams {
    pub twig: TwigAnatomy,
    pub length_ratio: f64,
    pub ratio_power: f64,
    pub internode_factor: f64,
    pub laterals: u32,
    pub limb_radius: f64,
    pub reach: f64,
    pub angle: f64,
    pub angle_variation: f64,
    pub vigour_variation: f64,
    pub divergence: f64,
}
impl Default for TwigParams {
    fn default() -> Self {
        Self {
            twig: TwigAnatomy::default(),
            length_ratio: 0.4,
            ratio_power: 1.3,
            internode_factor: 2.5,
            laterals: 2,
            limb_radius: 0.1,
            reach: 0.2,
            angle: 45.0,
            angle_variation: 10.0,
            vigour_variation: 0.15,
            divergence: 137.508,
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
    148.0 / 7.4_f64.powf(2.0 / 3.0) * radius.max(0.0).powf(2.0 / 3.0)
}
pub fn child_radius(radius: f64, ratio: f64, power: f64) -> f64 {
    radius * ratio.powf(power)
}
