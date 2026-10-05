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
        let lobe = self.lobe();
        let shaped = run.section.is_some();
        let ring = |point: CurvePoint, slope: f64, shape: Option<(u32, usize, usize)>| {
            let rho = point.radius * lobe * viewer.pixels_at(point.centre);
            let sides = sides(rho, 0.5 * error);
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
            let pieces = pieces(a, b, viewer, 0.5 * error);
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
pub(super) fn sides(rho: f64, error: f64) -> u32 {
    let exact = if rho <= error {
        3.0
    } else {
        std::f64::consts::PI / (1.0 - error / rho).acos_fixed()
    };
    let mut n = 3;
    while f64::from(n) < exact && n < MOST_SIDES {
        n *= 2;
    }
    n
}

/// How many pieces the stretch from `a` to `b` is cut into so its chords
/// stand within `error` pixels of the curve: a stretch of length `L` turning
/// through `θ` sags `Lθ/8` off its chord, `Lθ/(8m²)` cut in `m`.
fn pieces(a: &CurvePoint, b: &CurvePoint, viewer: &Viewer, error: f64) -> usize {
    let (ta, tb) = (tangent(a), tangent(b));
    let turn = ta.dot(tb).clamp(-1.0, 1.0).acos_fixed();
    let length = b.centre.distance(a.centre);
    let pixels = viewer.pixels_at(a.centre).max(viewer.pixels_at(b.centre));
    let m = (length * turn * pixels / (8.0 * error)).sqrt().ceil();
    (m as usize).clamp(1, MOST_PIECES)
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
