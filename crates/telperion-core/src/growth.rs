//! Annual age controls shared by every numeric family. No simulation rate lives here.
mod clock;
use crate::catalogue::{value, Blend, Bounds, Dial, Growth, Site};
use crate::{math::Transcendental, Error, Result};
pub(crate) use clock::Age;

/// Ages in years, quantized to one twelve-billionth of a year at the API boundary.
pub const MAX_AGE: f64 = 1_000_000.0;

crate::catalogue::rows! {
    #[derive(Debug, Clone, Copy, PartialEq)]
    #[cfg_attr(feature = "json", derive(serde::Serialize, serde::Deserialize))]
    pub struct GrowthTraits in "/growth" {
        /// Geometric work quanta available over this specimen's life.
        #[cfg_attr(feature = "json", serde(default = "crate::ranges::default_work_budget"))]
        pub work_budget: u32 = "workBudget" "work quanta"
            crate::ranges::POSITIVE_COUNT.bounds() => [Grow] {
            check: value(Site::Growth, 0, "growth.workBudget"),
            growth: Growth::Only,
            note: "Also sets when growth snaps to maturity, so it moves the growth path's mature \
                year.",
            blend: Blend::Count,
            dial: Dial::Excluded("Growth-path only, and a work budget."),
        },
        /// Chapman–Richards rate, in inverse years.
        pub rate: f64 = "rate" "per year" Bounds::closed(0.001, 10.0) => [Grow] {
            check: value(Site::Growth, 1, "growth.rate"),
            growth: Growth::Only,
            note: "Also ages shoot vigour for survival and bud sampling.",
            blend: Blend::Weighted,
            dial: Dial::Excluded("Growth-path only: `mesh::build` and `branching::generate` \
                never read the growth traits (crates/telperion-core/src/mesh.rs:73)."),
        },
        /// Chapman–Richards shape; values above one give a sigmoidal height curve.
        pub shape: f64 = "shape" "-" Bounds::closed(1.0, 8.0) => [Grow] {
            check: value(Site::Growth, 2, "growth.shape"),
            growth: Growth::Only,
            blend: Blend::Weighted,
            dial: Dial::Excluded("Growth-path only, as above."),
        },
        /// Consecutive active slices below the habit shedding threshold, in years.
        pub shedding_tolerance: f64 = "sheddingTolerance" "years"
            Bounds::closed(0.0, 1_000_000.0) => [Grow] {
            check: value(Site::Growth, 5, "growth.sheddingTolerance"),
            growth: Growth::Only,
            applies: "`sheddingThreshold` zero",
            blend: Blend::Weighted,
            dial: Dial::Excluded("Growth-path only, as above."),
        },
        /// Annual loss of the habit apical control (zero retains its authored value).
        pub apical_control_loss: f64 = "apicalControlLoss" "per year"
            Bounds::closed(0.0, 10.0) => [Grow] {
            check: value(Site::Growth, 6, "growth.apicalControlLoss"),
            growth: Growth::Only,
            note: "Divides `apicalDominance` each year and adds the released share to \
                `lateralLengthRatio`.",
            blend: Blend::Weighted,
            dial: Dial::Excluded("Growth-path only, as above."),
        },
        /// Years of annual foliage cohorts held by a living shoot; zero bears none.
        pub leaf_lifetime: f64 = "leafLifetime" "years" Bounds::closed(0.0, 1_000_000.0) => [Grow] {
            check: value(Site::Growth, 3, "growth.leafLifetime"),
            growth: Growth::Only,
            applies: "zero bears no leaves on the growth path",
            blend: Blend::Weighted,
            dial: Dial::Excluded("Growth-path only, as above."),
        },
        /// Minimum thickening in metres before recording another annual radius frame.
        pub resize_tolerance: f64 = "resizeTolerance" "m" Bounds::closed(0.0, 1.0) => [Grow] {
            check: value(Site::Growth, 4, "growth.resizeTolerance"),
            growth: Growth::Only,
            blend: Blend::Weighted,
            dial: Dial::Excluded("Growth-path only, and an internal recording tolerance rather \
                than a look."),
        },
    }
}

#[cfg(test)]
mod limit_tests {
    use super::*;
    #[test]
    fn lifetime_work_uses_the_authored_budget() {
        for work_budget in [10, 250_000, 500_000] {
            let t = GrowthTraits {
                work_budget,
                ..Default::default()
            };
            t.validate().unwrap();
            let work: usize = (1..=t.mature_slice()).map(|slice| t.budget(slice)).sum();
            assert_eq!(work, work_budget as usize);
        }
    }
}
impl Default for GrowthTraits {
    fn default() -> Self {
        Self {
            work_budget: crate::ranges::default_work_budget(),
            rate: 0.08,
            shape: 2.0,
            shedding_tolerance: 2.0,
            apical_control_loss: 0.0,
            leaf_lifetime: 1.0,
            // 0.1 mm keeps twig-scale detail while suppressing sub-visible
            // annual frames; 1 mm saved little build time in the native study.
            resize_tolerance: 0.0001,
        }
    }
}
impl GrowthTraits {
    pub fn validate(self) -> Result<()> {
        crate::catalogue::check(Self::CHECKS, &self, Site::Growth)
    }
    pub(crate) fn fraction(self, slice: u64) -> f64 {
        let f = (1.0 - (-self.rate * slice as f64).exp_fixed()).powf_fixed(self.shape);
        if f >= 1.0 - 0.5 / self.work_budget as f64 {
            1.0
        } else {
            f
        }
    }
    /// Rounded cumulative work makes every quantum belong to one fixed slice.
    pub(crate) fn budget(self, slice: u64) -> usize {
        let units = |m| (self.fraction(m) * self.work_budget as f64).round() as usize;
        units(slice) - units(slice - 1)
    }
    /// First slice receiving the last quantum. Computed once, without stepping years.
    pub(crate) fn mature_slice(self) -> u64 {
        let mut lo = 0;
        let mut hi = MAX_AGE as u64;
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
