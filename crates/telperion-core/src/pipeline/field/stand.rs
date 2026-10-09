//! A coarse tree's stand-in in the field (fn-214, host decision
//! 2026-10-09): its expected leaves spread evenly through its capsule, so
//! a cell of it holds λ = leaves × cell volume / capsule volume of them,
//! and is foliage with the chance 1 − e^(−λ) that at least one lands
//! there. The draw is keyed by the stand's key and the cell, so a cell
//! answers alike at every query. No cell below the ground answers.
use crate::{math::Vec3, tree::mix32};

#[derive(Debug, Clone, Copy)]
pub(super) struct Fill {
    key: u32,
    /// Expected leaves per cubic metre of the capsule.
    density: f64,
    leaves: f64,
}

impl Fill {
    /// `leaves` spread through the capsule about `a`, `b` of radius `reach`.
    pub(super) fn of(key: u32, leaves: f64, a: Vec3, b: Vec3, reach: f64) -> Self {
        let pi = std::f64::consts::PI;
        let volume = pi * reach * reach * (b - a).length() + 4.0 / 3.0 * pi * reach.powi(3);
        let density = if volume > 0.0 {
            leaves / volume
        } else {
            f64::INFINITY
        };
        Self {
            key,
            density,
            leaves,
        }
    }

    /// The leaves expected in the cell of `half` extent about `center`
    /// given that it is foliage, or `None` where it is not.
    pub(super) fn answer(&self, center: Vec3, half: f64) -> Option<f64> {
        if center.y < 0.0 || half <= 0.0 || self.leaves <= 0.0 {
            return None;
        }
        let lambda = self.density * (2.0 * half).powi(3);
        let chance = -(-lambda).exp_m1();
        let cell = |v: f64| (v / (2.0 * half)).floor() as i64 as u32;
        let draw = mix32(mix32(mix32(self.key ^ cell(center.x)) ^ cell(center.y)) ^ cell(center.z));
        (f64::from(draw) / 4_294_967_296.0 < chance).then(|| {
            if lambda.is_finite() {
                lambda / chance
            } else {
                self.leaves
            }
        })
    }
}
