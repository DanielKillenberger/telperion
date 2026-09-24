//! A run drawn as a cell of its stem's lattice rather than as a round ring.
//!
//! Wood is swept as round rings. A retained leaf base packed into its trunk's
//! lattice is not round: it is the cell the crown's spiral gives it on the
//! bark, wrapped on the stem and carried outward as a wedge. The tree keeps
//! that cell beside the run it shapes, so the sweep, the prepared surface and
//! the contact surface all draw the same shape from the same table, and a
//! tree with no section is swept exactly as it always was.
use crate::math::{Transcendental, Vec3};

/// One ring of a shaped run: how far out from the axis it stands, how far up
/// the axis it has risen, and the share of the cell it is drawn at.
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "json", derive(serde::Serialize, serde::Deserialize))]
pub struct SectionRing {
    pub reach: f64,
    pub rise: f64,
    pub scale: f64,
}

/// The cell one run is drawn as. The cell is spanned by two half-diagonals in
/// (radians round the axis, metres along it); a ring's vertices stand on the
/// cylinder of that ring's reach, so a cell keeps its angle outward and the
/// neighbours that meet on the bark meet at every reach. Flatness blends each
/// ring from the ellipse through the cell's corners to the cell itself.
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "json", derive(serde::Serialize, serde::Deserialize))]
pub struct Section {
    /// The last node of the run this section draws.
    pub node: u32,
    /// The point on the stem's axis the run leaves from.
    pub origin: Vec3,
    pub axis: Vec3,
    /// The way the run stands out from the axis, square to it.
    pub radial: Vec3,
    /// Square to both, the way the angle round the axis grows.
    pub across: Vec3,
    pub corners: [[f64; 2]; 2],
    pub flatness: f64,
    /// Inside the trunk, on the bark and at the outer end.
    pub rings: [SectionRing; 3],
}

impl Section {
    /// The ring a shaped run's sample `k` of `samples` is drawn as. A run that
    /// leaves the axis at a stem node has no sample inside the trunk and
    /// starts on the bark.
    pub fn ring(&self, k: usize, samples: usize) -> &SectionRing {
        &self.rings[k + 3 - samples.clamp(2, 3)]
    }

    /// The centre of a ring: on the radial, at its reach and its rise.
    pub fn centre(&self, ring: &SectionRing) -> Vec3 {
        self.origin + self.axis * ring.rise + self.radial * ring.reach
    }

    /// A ring's vertex at the unit circle's point `(cos, sin)`: blended toward
    /// the diamond through the same corners, carried onto the cell, and wrapped
    /// onto the cylinder the ring stands on.
    pub fn vertex(&self, ring: &SectionRing, cos: f64, sin: f64) -> Vec3 {
        let diamond = 1. / (cos.abs() + sin.abs());
        let blend = 1. + self.flatness * (diamond - 1.);
        let (u, v) = (cos * blend * ring.scale, sin * blend * ring.scale);
        let [a, b] = self.corners;
        let turn = a[0] * u + b[0] * v;
        let up = a[1] * u + b[1] * v;
        let (sin, cos) = turn.sin_cos_fixed();
        self.origin
            + self.axis * (ring.rise + up)
            + (self.radial * cos + self.across * sin) * ring.reach
    }

    /// A bound on how far a ring's vertices stand from its centre: the ellipse
    /// through the corners reaches no further than both half-diagonals
    /// together. What the sweep ranks a run by.
    pub fn extent(&self, ring: &SectionRing) -> f64 {
        let square = |c: [f64; 2]| (c[0] * ring.reach).powi(2) + c[1] * c[1];
        (square(self.corners[0]) + square(self.corners[1])).sqrt() * ring.scale
    }

    pub(crate) fn is_finite(&self) -> bool {
        let rings = self
            .rings
            .iter()
            .all(|r| [r.reach, r.rise, r.scale].iter().all(|v| v.is_finite()));
        rings
            && self.origin.is_finite()
            && self.axis.is_finite()
            && self.radial.is_finite()
            && self.across.is_finite()
            && self.flatness.is_finite()
            && self.corners.iter().flatten().all(|v| v.is_finite())
    }
}
