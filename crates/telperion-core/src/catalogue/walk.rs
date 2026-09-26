//! Ordinary blending: every row whose entry names a walk of its own moves by
//! it, independently of every other row. Coupled rows are left to
//! `blend::families`, and a kept row stays the first family's.
use super::{entries, Blend};
use crate::Family;

/// Writes into `f` every ordinary row walked from `a` to `b` at `t`.
pub fn walk(a: &Family, b: &Family, t: f64, f: &mut Family) {
    for entry in entries() {
        let (Some(x), Some(y)) = (entry.get(a).number(), entry.get(b).number()) else {
            continue;
        };
        let value = match entry.info().blend {
            Blend::Linear => linear(x, y, t),
            Blend::Weighted => weighted(x, y, t),
            Blend::Degrees => degrees(x, y, t),
            Blend::Count | Blend::Many => linear(x, y, t).round(),
            Blend::Down => linear(x, y, t).floor(),
            Blend::Up => linear(x, y, t).ceil(),
            Blend::Density => density(x, y, t),
            Blend::Kept | Blend::Coupled => continue,
        };
        entry.set(f).put(value);
    }
}

/// Linear, clamped to the two ends so rounding cannot leave the range both
/// ends were valid in.
pub fn weighted(a: f64, b: f64, t: f64) -> f64 {
    let value = (1.0 - t) * a + t * b;
    if a.is_finite() && b.is_finite() {
        value.clamp(a.min(b), a.max(b))
    } else {
        value
    }
}

pub fn linear(a: f64, b: f64, t: f64) -> f64 {
    a + (b - a) * t
}

/// Degrees along the shorter arc, so two bearings either side of the wrap meet
/// across it rather than sweeping the long way round. A walk between two
/// bearings that are themselves positive stays positive, which is the range
/// every angle in a family is validated against.
pub fn degrees(a: f64, b: f64, t: f64) -> f64 {
    let delta = (b - a + 180.0).rem_euclid(360.0) - 180.0;
    let turned = a + delta * t;
    if turned < 0.0 && a >= 0.0 && b >= 0.0 {
        turned + 360.0
    } else {
        turned
    }
}

/// Shoots per metre walk linearly, and the spacing is what they leave: a walk
/// from zero, which grows none, starts past the furthest the rail allows and
/// closes in, with no frame where the wood is suddenly crowded.
pub fn density(a: f64, b: f64, t: f64) -> f64 {
    let per = |spacing: f64| if spacing > 0.0 { 1.0 / spacing } else { 0.0 };
    let walked = linear(per(a), per(b), t);
    if walked > 0.0 {
        (1.0 / walked).min(crate::foliage::SHORT_SHOOT_SPACING.1)
    } else {
        0.0
    }
}
