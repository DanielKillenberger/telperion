//! Which rings a cluster is drawn through for one view (host decisions 2, 4
//! and 14), cluster by cluster as the GPU passes run it.
//! - A cluster outside the frustum is not drawn.
//! - A ring standing at least `RIBBON` pixels in radius is a tube's, a
//!   thinner one a ribbon's: a stretch between two tube rings is drawn as a
//!   tube, any other as a ribbon facing the eye (host decision 4). A shaped
//!   run is always a tube.
//! - It keeps the coarsest nested ring level whose error stands within half
//!   the error at its nearest depth; between the rings kept a curved stretch
//!   is cut along its Hermite curve until its chords sag within that half.
//! - Each tube ring takes the fewest sides, three times a power of two,
//!   whose polygon stands within the other half (host decision 14).
//! - A cluster draws its own first and last points, so neighbours share
//!   identical rings and meet without a crack.
use super::{Ring, Viewer};
use crate::{
    math::{Transcendental, Vec3},
    pipeline::surface::curve::{Curve, CurveCluster, CurvePoint, LEVELS},
};

/// The most sides a ring is cut into: 3 · 2^7.
pub(super) const MOST_SIDES: u32 = 384;
/// The most pieces one stretch between two kept points is cut into.
const MOST_PIECES: usize = 256;
/// Below this radius in pixels a ring is a ribbon's (host decision 4).
pub const RIBBON: f64 = 1.0;

/// A cluster's rings for one view.
#[derive(Debug, Clone)]
pub(super) struct Plan {
    pub(super) rings: Vec<Ring>,
}

impl Curve {
    /// How cluster `index` is drawn for `viewer` within `error` pixels, or
    /// nothing outside the frustum.
    pub(super) fn plan(&self, index: usize, viewer: &Viewer, error: f64) -> Option<Plan> {
        let c = &self.clusters[index];
        if !visible(c, viewer) {
            return None;
        }
        let run = &self.runs[c.run as usize];
        let (lobe, bend) = (self.lobe(), self.bend());
        let shaped = run.section.is_some();
        let ring = |point: CurvePoint, slope: f64, shape: Option<(u32, usize, usize)>| {
            let rho = point.radius * lobe * viewer.pixels_at(point.centre);
            let sides = sides(rho, 0.5 * error, bend);
            Ring {
                point,
                slope,
                sides,
                shape,
                tube: shaped || rho >= RIBBON,
            }
        };
        let first = c.first as usize;
        if let Some(section) = run.section {
            let rings = (0..c.count as usize)
                .map(|i| {
                    let slope = self.slope(first + i, run.first, run.count);
                    ring(
                        self.points[first + i],
                        slope,
                        Some((section, i, c.count as usize)),
                    )
                })
                .collect();
            return Some(Plan { rings });
        }
        let kept = kept(c, viewer, 0.5 * error);
        let mut rings = Vec::with_capacity(kept.len());
        for pair in kept.windows(2) {
            let (ga, gb) = (first + pair[0], first + pair[1]);
            let (a, b) = (&self.points[ga], &self.points[gb]);
            let (sa, sb) = (
                self.slope(ga, run.first, run.count),
                self.slope(gb, run.first, run.count),
            );
            let pieces = pieces(a, b, viewer, 0.5 * error, self.phase(a, b));
            for j in 0..pieces {
                let t = j as f64 / pieces as f64;
                rings.push(ring(hermite(a, b, t), sa + (sb - sa) * t, None));
            }
        }
        let g = first + kept[kept.len() - 1];
        rings.push(ring(
            self.points[g],
            self.slope(g, run.first, run.count),
            None,
        ));
        Some(Plan { rings })
    }

    /// What the most any view could ask of this curve rests on, computed
    /// once a tree (fn-208, host decision 23): see `Demand::at`.
    pub fn demand(&self) -> Demand {
        let mut d = Demand::default();
        for c in &self.clusters {
            d.clusters += 1;
            if self.runs[c.run as usize].section.is_some() {
                d.shaped += u64::from(c.count);
                continue;
            }
            let span = &self.points[c.first as usize..(c.first + c.count) as usize];
            for w in span.windows(2) {
                d.stretches += 1;
                d.root_sag += sag(&w[0], &w[1], self.phase(&w[0], &w[1])).sqrt();
            }
        }
        d
    }

    /// How much more a lobed ring's outline bends than its circle's: a
    /// polygon's sag over `Δθ` is at most `Δθ²/8` of its outline's second
    /// derivative, `R(1 + |d|(n + 1)²)` for `R(1 + d·cos(nθ))`, against the
    /// circle `R(1 + |d|)` the radius in pixels already holds. 1 unlobed.
    pub(super) fn bend(&self) -> f64 {
        if self.lobes > 0 && self.lobe_depth != 0.0 {
            let n = f64::from(self.lobes) + 1.0;
            (1.0 + self.lobe_depth.abs() * n * n) / (1.0 + self.lobe_depth.abs())
        } else {
            1.0
        }
    }

    /// How far the lobes turn between `a` and `b`, in lobe radians, times
    /// the root of their depth: `R·phase²` is what the stretch's outline,
    /// interpolated between its ends' phases, can sag. 0 untwisted.
    pub(super) fn phase(&self, a: &CurvePoint, b: &CurvePoint) -> f64 {
        let (twist, depth) = self.twisting();
        twist * (b.along - a.along).abs() * depth.sqrt()
    }

    fn lobe(&self) -> f64 {
        if self.lobes > 0 {
            1.0 + self.lobe_depth.abs()
        } else {
            1.0
        }
    }

    /// How fast the radius changes along the curve at point `g` of a run of
    /// `count` points from `first`: from its neighbours on the run, so every
    /// cluster that draws the point reads the same slope.
    fn slope(&self, g: usize, first: u32, count: u32) -> f64 {
        let (lo, hi) = (first as usize, (first + count - 1) as usize);
        let (a, b) = (
            &self.points[g.saturating_sub(1).max(lo)],
            &self.points[(g + 1).min(hi)],
        );
        let run = b.along - a.along;
        if run.abs() < 1e-12 {
            0.0
        } else {
            (b.radius - a.radius) / run
        }
    }
}

/// Whether any of cluster `c` stands inside the view's frustum.
fn visible(c: &CurveCluster, viewer: &Viewer) -> bool {
    viewer.planes.is_none_or(|planes| {
        planes
            .iter()
            .all(|p| p[0] * c.centre.x + p[1] * c.centre.y + p[2] * c.centre.z + p[3] >= -c.reach)
    })
}

/// Pixels a metre spans at the cluster's nearest depth.
fn nearest(c: &CurveCluster, viewer: &Viewer) -> f64 {
    if viewer.orthographic {
        return viewer.pixels_per_metre;
    }
    let depth = (c.centre - viewer.eye).dot(viewer.forward) - c.reach;
    viewer.pixels_per_metre / depth.max(viewer.near)
}

/// The cluster's points kept, as indices from its first: every `2^k`-th and
/// its last, at the coarsest level `k` whose error stands within `error`
/// pixels at its nearest depth.
fn kept(c: &CurveCluster, viewer: &Viewer, error: f64) -> Vec<usize> {
    let pixels = nearest(c, viewer);
    let level = (0..LEVELS)
        .rev()
        .find(|&k| f64::from(c.errors[k]) * pixels <= error)
        .unwrap_or(0);
    let last = c.count as usize - 1;
    let mut kept: Vec<usize> = (0..last).step_by(1 << level).collect();
    kept.push(last);
    kept
}

/// The fewest sides, three times a power of two, whose polygon stands within
/// `error` pixels of a circle `rho` pixels in radius:
/// `ceil(π / acos(1 − error / ρ))`, rounded up (host decisions 2 and 14).
/// A lobed outline, which bends `bend` times more, also needs
/// `2π / √(8·error / (ρ·bend))`.
pub(super) fn sides(rho: f64, error: f64, bend: f64) -> u32 {
    let mut exact = if rho <= error {
        3.0
    } else {
        std::f64::consts::PI / (1.0 - error / rho).acos_fixed()
    };
    if bend > 1.0 {
        exact = exact.max(std::f64::consts::TAU * (rho * bend / (8.0 * error)).sqrt());
    }
    let mut n = 3;
    while f64::from(n) < exact && n < MOST_SIDES {
        n *= 2;
    }
    n
}

/// How many pieces the stretch from `a` to `b` is cut into so its chords
/// stand within `error` pixels of the curve, the most of three bounds:
/// - a stretch of length `L` turning through `θ` sags `Lθ/8` off its chord,
///   `Lθ/(8m²)` cut in `m`;
/// - a cubic sags at most `max|B''|/(8m²)`, `B''` six times its control
///   points' second differences at either end, which holds where the end
///   tangents are parallel but the curve is not straight;
/// - twisting lobes, `phase` lobe radians a metre of radius apart at its
///   ends, stand `R·|d|·(nΔφ)²/8` off their interpolated outline.
fn pieces(a: &CurvePoint, b: &CurvePoint, viewer: &Viewer, error: f64, phase: f64) -> usize {
    let pixels = viewer.pixels_at(a.centre).max(viewer.pixels_at(b.centre));
    let m = (sag(a, b, phase) * pixels / (8.0 * error)).sqrt().ceil();
    (m as usize).clamp(1, MOST_PIECES)
}

/// What `pieces` cuts against, in metres: the stretch's sag uncut, times 8.
fn sag(a: &CurvePoint, b: &CurvePoint, phase: f64) -> f64 {
    let (ta, tb) = (tangent(a), tangent(b));
    let turn = ta.dot(tb).clamp(-1.0, 1.0).acos_fixed();
    let length = b.centre.distance(a.centre);
    // The control points' second differences, from the chord so that a
    // straight stretch reads zero without cancelling world coordinates.
    let chord = b.centre - a.centre;
    let bent = (chord - (ta * 2.0 + tb) * (length / 3.0))
        .length()
        .max((chord - (ta + tb * 2.0) * (length / 3.0)).length());
    let radius = a.radius.max(b.radius);
    (length * turn).max(6.0 * bent) + radius * phase * phase
}

/// The sums a curve's most demanding view is bounded by (`Curve::demand`).
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Demand {
    pub clusters: u64,
    /// Rings of shaped runs, which are never cut.
    pub shaped: u64,
    /// Stretches between neighbouring points of round runs, and the sum of
    /// the roots of their sags.
    pub stretches: u64,
    pub root_sag: f64,
}

impl Demand {
    /// The most a view could ask that sees none of the curve nearer than
    /// `pixels` a metre, within `error` pixels: rings, vertices, tube
    /// indices and ribbon indices. Every point kept, every stretch cut as
    /// `pieces` would at `pixels` (`ceil(√x) ≤ √x + 1`, so the roots' sum
    /// bounds them all), every ring a tube of `MOST_SIDES` capped at both
    /// ends.
    pub fn at(&self, pixels: f64, error: f64) -> [u64; 4] {
        let k = (pixels.max(0.0) / (8.0 * 0.5 * error)).sqrt();
        let cut = (self.root_sag * k).ceil() as u64 + self.stretches;
        let cut = cut.min(self.stretches * MOST_PIECES as u64);
        let rings = self.shaped + cut + self.clusters;
        let sides = u64::from(MOST_SIDES);
        // A ring writes its polygon, three ribbon vertices and two cap
        // centres; a strip to the last ring and two cap fans, 12 sides of
        // tube indices; a ribbon quad, 12.
        [rings, rings * (sides + 5), rings * 12 * sides, rings * 12]
    }
}

pub(super) fn tangent(p: &CurvePoint) -> Vec3 {
    p.normal.cross(p.binormal)
}

/// The point a share `t` along the Hermite curve from `a` to `b`: its
/// centre and tangent on the curve, its radius and distance along between
/// theirs, and its frame turned onto the tangent from theirs.
fn hermite(a: &CurvePoint, b: &CurvePoint, t: f64) -> CurvePoint {
    if t == 0.0 {
        return *a;
    }
    let length = b.centre.distance(a.centre);
    let (ta, tb) = (tangent(a) * length, tangent(b) * length);
    let (t2, t3) = (t * t, t * t * t);
    let centre = a.centre * (2.0 * t3 - 3.0 * t2 + 1.0)
        + ta * (t3 - 2.0 * t2 + t)
        + b.centre * (3.0 * t2 - 2.0 * t3)
        + tb * (t3 - t2);
    let along = a.centre * (6.0 * t2 - 6.0 * t)
        + ta * (3.0 * t2 - 4.0 * t + 1.0)
        + b.centre * (6.0 * t - 6.0 * t2)
        + tb * (3.0 * t2 - 2.0 * t);
    let tangent = along.normalized();
    let mixed = a.normal.lerp(b.normal, t);
    let normal = (mixed - tangent * mixed.dot(tangent)).normalized();
    CurvePoint {
        centre,
        radius: a.radius + (b.radius - a.radius) * t,
        along: a.along + (b.along - a.along) * t,
        normal,
        binormal: tangent.cross(normal),
    }
}

#[cfg(test)]
#[path = "plan_tests.rs"]
mod tests;
