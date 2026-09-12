//! Monthly age controls shared by every numeric family. No simulation rate lives here.
mod clock;
use crate::{math::Transcendental, Error, Result};
pub(crate) use clock::Age;

/// Ages in years, quantized to one billionth of a month at the API boundary.
pub const MAX_AGE: f64 = 1_000_000.0;
/// Geometric growth work available over a specimen's life (not a node limit).
const LIFETIME_UNITS: f64 = 250_000.0;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GrowthTraits {
    /// Chapman–Richards rate, in inverse years.
    pub rate: f64,
    /// Chapman–Richards shape; values above one give a sigmoidal height curve.
    pub shape: f64,
    /// Consecutive active months below the habit shedding threshold, in years.
    pub shedding_tolerance: f64,
    /// Annual loss of the habit apical control (zero retains its authored value).
    pub apical_control_loss: f64,
}
impl Default for GrowthTraits {
    fn default() -> Self {
        Self {
            rate: 0.08,
            shape: 2.0,
            shedding_tolerance: 2.0,
            apical_control_loss: 0.0,
        }
    }
}
impl GrowthTraits {
    pub fn validate(self) -> Result<()> {
        for (field, value, lo, hi) in [
            ("growth.rate", self.rate, 0.001, 10.0),
            ("growth.shape", self.shape, 1.0, 8.0),
            (
                "growth.sheddingTolerance",
                self.shedding_tolerance,
                0.0,
                MAX_AGE,
            ),
            (
                "growth.apicalControlLoss",
                self.apical_control_loss,
                0.0,
                10.0,
            ),
        ] {
            if !value.is_finite() || !(lo..=hi).contains(&value) {
                return Err(Error::InvalidValue {
                    field,
                    value: value.to_string(),
                });
            }
        }
        Ok(())
    }
    pub(crate) fn fraction(self, month: u64) -> f64 {
        let f = (1.0 - (-self.rate * month as f64 / 12.0).exp_fixed()).powf_fixed(self.shape);
        if f >= 1.0 - 0.5 / LIFETIME_UNITS {
            1.0
        } else {
            f
        }
    }
    /// Rounded cumulative work makes every quantum belong to one fixed month.
    pub(crate) fn budget(self, month: u64) -> usize {
        let units = |m| (self.fraction(m) * LIFETIME_UNITS).round() as usize;
        units(month) - units(month - 1)
    }
    /// First month receiving the last quantum. Computed once, without stepping years.
    pub(crate) fn mature_month(self) -> u64 {
        let mut lo = 0;
        let mut hi = (MAX_AGE * 12.0) as u64;
        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            if self.fraction(mid) == 1.0 {
                hi = mid;
            } else {
                lo = mid + 1;
            }
        }
        lo
    }
}
