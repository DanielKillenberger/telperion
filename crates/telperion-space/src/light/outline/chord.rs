//! A sky ray's chord through one cone section of the crown's outline
//! (fn-226 R3; R4-INVENTORY.md section 3), in contract A's Q32: the ray
//! `P + u d` is inside the section's cone where `A u² + 2 B u + C <= 0`,
//! solved in closed form with one integer square root, and clipped to the
//! section's heights. No transcendental.
use crate::fixed::feedback::vector::isqrt;
use crate::fixed::{mul, INF};

/// A cone section of the outline: its heights `z[0] < z[1]` and its
/// radii there, Q32.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) struct Section {
    pub z: [i64; 2],
    pub r: [i64; 2],
}

/// `n 2^32 / d` of Q32 values, truncated, saturated to Q32's range.
fn over(n: i128, d: i128) -> i128 {
    ((n << 32) / d).clamp(-i128::from(INF), i128::from(INF))
}

/// The length along `d` (Q32, in units of `d`) of the ray from `p` along
/// `d` (`d[2] > 0`) inside section `s`: zero where it misses the section,
/// is tangent to it, or lies wholly above it.
pub(crate) fn chord(s: &Section, p: [i64; 3], d: [i64; 3]) -> i64 {
    let ([z0, z1], [r0, r1]) = (s.z, s.r);
    let h = z1 - z0;
    if h <= 0 || d[2] <= 0 {
        return 0;
    }
    let lo = over(i128::from(z0 - p[2]), i128::from(d[2])).max(0);
    let hi = over(i128::from(z1 - p[2]), i128::from(d[2]));
    if hi <= lo {
        return 0;
    }
    // The cone's radius at height z is `e + b (z - p_z)`, so along the ray
    // it is `e + g u`; within the section it is never negative.
    let b = over(i128::from(r1 - r0), i128::from(h)) as i64;
    let e = r0 + mul(b, p[2] - z0);
    let g = mul(b, d[2]);
    let a = mul(d[0], d[0]) + mul(d[1], d[1]) - mul(g, g);
    let half = mul(p[0], d[0]) + mul(p[1], d[1]) - mul(e, g);
    let c = mul(p[0], p[0]) + mul(p[1], p[1]) - mul(e, e);
    inside([a, half, c].map(i128::from), (lo, hi)) as i64
}

/// The length of `{u : a u² + 2 half u + c <= 0}` (Q32 coefficients)
/// within `[lo, hi]`.
pub(crate) fn inside([a, half, c]: [i128; 3], (lo, hi): (i128, i128)) -> i128 {
    let within = |from: i128, to: i128| (to.min(hi) - from.max(lo)).max(0);
    if a == 0 {
        if half == 0 {
            // Constant: the ray is inside over the whole span, or outside.
            return if c <= 0 { hi - lo } else { 0 };
        }
        // Linear (a ray along a cone's generator): `2 half u + c <= 0`.
        let root = over(-c, 2 * half);
        return if half > 0 {
            within(lo, root)
        } else {
            within(root, hi)
        };
    }
    // The discriminant `half² - a c`, Q64.
    let disc = half * half - a * c;
    if disc <= 0 {
        // No root, or a tangent's one: inside nowhere where the cone
        // opens away from the ray, everywhere where it closes on it.
        return if a > 0 { 0 } else { hi - lo };
    }
    let root = isqrt(disc as u128) as i128;
    let (u1, u2) = (over(-half - root, a), over(-half + root, a));
    let (u1, u2) = (u1.min(u2), u1.max(u2));
    if a > 0 {
        within(u1, u2)
    } else {
        within(lo, u1) + within(u2, hi)
    }
}
