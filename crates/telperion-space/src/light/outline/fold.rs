//! The outline's shape folded from points (fn-226 R3): core's `Folds`
//! and least holding shoulder (`pipeline/input/folds.rs`, `shoulder.rs`)
//! in the engine's frame, z up, reach off the seed's vertical. A crown
//! point is one off the stem. The shoulder is a second pass over the
//! crown points, against the first pass's base, height, reach and
//! fullness, on the same `quadrant` every bud reads.
use super::quadrant;
use crate::fixed::q32;
use crate::structure::Vec3;
use crate::transcendental::Transcendental;

/// The shoulders searched, from nearly a cone's point to nearly a
/// cylinder, and the search's halvings (core's `shoulder.rs`).
const NARROWEST: f64 = 1.0 / 16.0;
const WIDEST: f64 = 64.0;
const HALVINGS: usize = 40;
/// Bins of the way from the widest height to an end, for the front.
const BINS: usize = 1024;

/// The folds of a cycle's points: the highest, the lowest crown point,
/// the widest reach and its height, and the crown points kept for the
/// shoulder.
#[derive(Debug, Clone)]
pub(crate) struct Fold {
    height: f64,
    low: f64,
    widest: Option<(f64, f64)>,
    crown: Vec<(f64, f64)>,
}

impl Default for Fold {
    fn default() -> Self {
        Self {
            height: 0.0,
            low: f64::INFINITY,
            widest: None,
            crown: Vec::new(),
        }
    }
}

/// A point's reach off the seed's vertical.
fn reach(p: Vec3) -> f64 {
    p.x.hypot_fixed(p.y)
}

impl Fold {
    /// Folds in the point `p`, in the crown or on the stem.
    pub fn add(&mut self, p: Vec3, crown: bool) {
        let r = reach(p);
        self.height = self.height.max(p.z);
        if crown {
            self.low = self.low.min(p.z);
            self.crown.push((p.z, r));
        }
        if self.widest.is_none_or(|w| r >= w.0) {
            self.widest = Some((r, p.z));
        }
    }

    /// The base, height, widest reach and fullness.
    fn folds(&self) -> [f64; 4] {
        let height = self.height;
        let base = self.low.min(height);
        let (radius, at) = self.widest.unwrap_or((0.0, height));
        let span = height - base;
        let full = if span > 0.0 {
            ((at - base) / span).clamp(0.0, 1.0)
        } else {
            0.0
        };
        [base, height, radius, full]
    }

    /// The shape, Q32: base, height, widest reach, fullness and the least
    /// shoulder that holds every crown point.
    pub fn shape(&self) -> [i64; 5] {
        let folds = self.folds();
        let [base, height, radius, full] = folds.map(q32);
        let front = self.front(folds);
        let holds = |s: f64| {
            let s = q32(s);
            front.iter().all(|&(p, q)| quadrant(q32(p), s) >= q32(q))
        };
        let s = if holds(NARROWEST) {
            NARROWEST
        } else if !holds(WIDEST) {
            WIDEST
        } else {
            let (mut lo, mut hi) = (NARROWEST, WIDEST);
            for _ in 0..HALVINGS {
                let mid = (lo * hi).sqrt();
                if holds(mid) {
                    hi = mid;
                } else {
                    lo = mid;
                }
            }
            hi
        };
        [base, height, radius, full, q32(s)]
    }

    /// Every crown point strictly inside the span as (p, q): its way from
    /// the widest height to the end it lies toward, its reach as a share
    /// of the widest; only those no other passes in both.
    fn front(&self, [base, height, radius, full]: [f64; 4]) -> Vec<(f64, f64)> {
        if !(radius > 0.0 && height > base) {
            return Vec::new();
        }
        let point = |&(z, r): &(f64, f64)| -> Option<(f64, f64)> {
            if z <= base || z >= height || r <= 0.0 {
                return None;
            }
            let t = (z - base) / (height - base);
            let p = if t < full {
                1.0 - t / full
            } else {
                (t - full) / (1.0 - full)
            };
            (p > 0.0).then_some((p.min(1.0), r / radius))
        };
        let bin = |p: f64| ((p * BINS as f64) as usize).min(BINS - 1);
        let mut most = vec![0.0f64; BINS];
        for (p, q) in self.crown.iter().filter_map(point) {
            most[bin(p)] = most[bin(p)].max(q);
        }
        let mut beyond = vec![0.0f64; BINS];
        for b in (0..BINS - 1).rev() {
            beyond[b] = beyond[b + 1].max(most[b + 1]);
        }
        let mut kept: Vec<(f64, f64)> = (self.crown.iter().filter_map(point))
            .filter(|&(p, q)| q > beyond[bin(p)])
            .collect();
        kept.sort_by(|a, b| b.0.total_cmp(&a.0).then(b.1.total_cmp(&a.1)));
        let mut front = Vec::new();
        let mut far = 0.0f64;
        for (p, q) in kept {
            if q > far {
                front.push((p, q));
                far = q;
            }
        }
        front
    }
}
