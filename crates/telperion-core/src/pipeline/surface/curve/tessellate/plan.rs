//! Which rings a run is drawn through for one view (host decision 2). Each
//! cluster keeps the coarsest ring level whose error stands under half the
//! budget in pixels at its nearest depth; between the rings kept, a curved
//! stretch is subdivided along its Hermite curve until its chords stand
//! within the same half; and each ring takes the fewest sides, three times a
//! power of two, whose polygon stands within the other half of its circle.
use super::{Ring, Viewer};
use crate::{
    math::{Transcendental, Vec3},
    pipeline::surface::curve::{Curve, CurvePoint, LEVELS},
};

/// The most sides a ring is cut into: 3 · 2^7.
const MOST_SIDES: u32 = 384;
/// The most rings one stretch between two kept points is subdivided into.
const MOST_PIECES: usize = 256;

impl Curve {
    /// The rings run `run` is drawn through for `viewer` within `error` px.
    pub(super) fn plan(&self, run: usize, viewer: &Viewer, error: f64) -> Vec<Ring> {
        let r = &self.runs[run];
        let points = self.run_points(r);
        let lobe = if self.lobes > 0 {
            1.0 + self.lobe_depth.abs()
        } else {
            1.0
        };
        // The error is split: half for the polygon round a ring, half for the
        // chords along the curve, so the two together stand within it.
        let sides =
            |p: &CurvePoint| sides(p.radius * lobe * viewer.pixels_at(p.centre), 0.5 * error);
        if let Some(section) = r.section {
            return points
                .iter()
                .enumerate()
                .map(|(i, p)| Ring {
                    point: *p,
                    sides: sides(p),
                    shape: Some((section, i, points.len())),
                })
                .collect();
        }
        let kept = self.kept(run, viewer, 0.5 * error);
        let mut rings = Vec::with_capacity(kept.len());
        for pair in kept.windows(2) {
            let (a, b) = (&self.points[pair[0]], &self.points[pair[1]]);
            let pieces = pieces(a, b, viewer, 0.5 * error);
            for j in 0..pieces {
                let point = hermite(a, b, j as f64 / pieces as f64);
                rings.push(Ring {
                    point,
                    sides: sides(&point),
                    shape: None,
                });
            }
        }
        let last = self.points[*kept.last().expect("a run has a point")];
        rings.push(Ring {
            point: last,
            sides: sides(&last),
            shape: None,
        });
        rings
    }

    /// The points run `run` keeps: in each cluster, every `2^k`-th and its
    /// last, at the coarsest level `k` whose error stands within `error`
    /// pixels at the cluster's nearest depth.
    fn kept(&self, run: usize, viewer: &Viewer, error: f64) -> Vec<usize> {
        let mut kept: Vec<usize> = Vec::new();
        for c in self.clusters.iter().filter(|c| c.run as usize == run) {
            let depth = (c.centre - viewer.eye).dot(viewer.forward) - c.reach;
            let pixels = viewer.pixels_per_metre / depth.max(viewer.near);
            let level = (0..LEVELS)
                .rev()
                .find(|&k| f64::from(c.errors[k]) * pixels <= error)
                .unwrap_or(0);
            let (first, last) = (c.first as usize, (c.first + c.count - 1) as usize);
            for i in (first..last).step_by(1 << level).chain([last]) {
                if kept.last() != Some(&i) {
                    kept.push(i);
                }
            }
        }
        if kept.is_empty() {
            kept.push(self.runs[run].first as usize);
        }
        kept
    }
}

/// The fewest sides, three times a power of two, whose polygon stands within
/// `error` pixels of a circle `rho` pixels in radius:
/// `ceil(π / acos(1 − error / ρ))`, rounded up (host decision 2).
fn sides(rho: f64, error: f64) -> u32 {
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

fn tangent(p: &CurvePoint) -> Vec3 {
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
