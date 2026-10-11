//! fn-255's light candidate (R3, DESIGN.md section 3): a cycle's leaves
//! held as a hierarchy of foliage volumes, each a compact ellipsoidal
//! kernel matched to its leaves' area, centroid and second moments, and
//! read along the sky's directions in closed form.
//!
//! - **Moments** are raw sums, `m0 = Σ A`, `m1 = Σ A x`, `m2 = Σ A (x xᵀ +
//!   s² I)`: two summaries merge by adding, so leaf area is conserved and
//!   a summary fades with its areas. Each leaf lends its own variance
//!   `s²`, from its PA's leaf size, so a point, a line or a plane of
//!   leaves still spans a volume and its density stays finite.
//! - **A kernel** is `ρ(x) = m0 (15 / 8π) / √det(7S) · max(0, 1 − (x − μ)ᵀ
//!   (7S)⁻¹ (x − μ))`, whose covariance is `S` (the profile `1 − r²` on the
//!   unit ball has variance 1/7 an axis). Along a ray its density is a
//!   quadratic in the ray's length, integrated exactly between the roots.
mod tree;

pub use tree::Candidate;
pub(crate) use tree::Volumes;

use crate::structure::Vec3;
use std::f64::consts::PI;

/// The upper triangle of a symmetric 3×3 matrix: xx, xy, xz, yy, yz, zz.
type Sym = [f64; 6];

/// A summary of leaf area: its raw moments.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub(crate) struct Moments {
    pub m0: f64,
    pub m1: [f64; 3],
    pub m2: Sym,
}

impl Moments {
    /// One leaf of `area` at `at`, its own variance `s2` on every axis.
    pub fn leaf(at: Vec3, area: f64, s2: f64) -> Self {
        let x = [at.x, at.y, at.z];
        let mut m2 = outer(x);
        for d in [0, 3, 5] {
            m2[d] += s2;
        }
        Self {
            m0: area,
            m1: x.map(|v| area * v),
            m2: m2.map(|v| area * v),
        }
    }

    pub fn add(&mut self, o: &Self) {
        self.m0 += o.m0;
        for a in 0..3 {
            self.m1[a] += o.m1[a];
        }
        for a in 0..6 {
            self.m2[a] += o.m2[a];
        }
    }


    /// The kernel these moments describe; none without leaf area.
    pub fn kernel(&self) -> Option<Kernel> {
        if !(self.m0 > 0.0) {
            return None;
        }
        let mu = self.m1.map(|v| v / self.m0);
        let centred = outer(mu);
        let s: Sym = std::array::from_fn(|a| self.m2[a] / self.m0 - centred[a]);
        let wide = s.map(|v| 7.0 * v);
        let det = determinant(wide);
        if !(det > 0.0) {
            return None;
        }
        let trace = s[0] + s[3] + s[5];
        Some(Kernel {
            centre: Vec3::new(mu[0], mu[1], mu[2]),
            inverse: inverse(wide, det),
            scale: self.m0 * 15.0 / (8.0 * PI) / det.sqrt(),
            reach: (7.0 * trace).sqrt(),
        })
    }
}

/// A foliage volume: where it stands, `(7S)⁻¹`, its density at its centre
/// and how far it reaches at most (`√(7 tr S)`, at least its longest
/// semi-axis).
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct Kernel {
    pub centre: Vec3,
    inverse: Sym,
    scale: f64,
    pub reach: f64,
}

impl Kernel {
    /// The leaf area a ray from `at` towards unit `toward` passes through,
    /// in square metres a square metre: the density's integral along it.
    pub fn along(&self, at: Vec3, toward: Vec3) -> f64 {
        let u = at - self.centre;
        let (mu, md) = (apply(self.inverse, u), apply(self.inverse, toward));
        let a = toward.dot(md);
        let b = u.dot(md);
        let c = u.dot(mu);
        let disc = b * b - a * (c - 1.0);
        if !(disc > 0.0 && a > 0.0) {
            return 0.0;
        }
        let root = disc.sqrt();
        let (low, high) = (((-b - root) / a).max(0.0), (-b + root) / a);
        if high <= low {
            return 0.0;
        }
        let integral = |t: f64| t * (1.0 - c) - b * t * t - a * t * t * t / 3.0;
        self.scale * (integral(high) - integral(low)).max(0.0)
    }
}

fn outer(x: [f64; 3]) -> Sym {
    [
        x[0] * x[0],
        x[0] * x[1],
        x[0] * x[2],
        x[1] * x[1],
        x[1] * x[2],
        x[2] * x[2],
    ]
}

fn apply(m: Sym, v: Vec3) -> Vec3 {
    Vec3::new(
        m[0] * v.x + m[1] * v.y + m[2] * v.z,
        m[1] * v.x + m[3] * v.y + m[4] * v.z,
        m[2] * v.x + m[4] * v.y + m[5] * v.z,
    )
}

fn determinant(m: Sym) -> f64 {
    let [a, b, c, d, e, f] = m;
    a * (d * f - e * e) - b * (b * f - e * c) + c * (b * e - d * c)
}

fn inverse(m: Sym, det: f64) -> Sym {
    let [a, b, c, d, e, f] = m;
    [
        (d * f - e * e) / det,
        (c * e - b * f) / det,
        (b * e - c * d) / det,
        (a * f - c * c) / det,
        (b * c - a * e) / det,
        (a * d - b * b) / det,
    ]
}

#[cfg(test)]
mod tests;
