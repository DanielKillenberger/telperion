//! The pipe model's section across exponents (fn-209, host decision 15).
//! A subtree carries its base section S in its own PA's exponent e_c; a
//! bearer of exponent e_p sums it as its radius to e_p, S^(e_p / e_c),
//! whose expectation is not the mean's power where the exponents differ
//! (Astra on decision 12). It is taken by the delta method from S's mean
//! and mean square among the subtrees kept, so the shed, whose section is
//! none, do not stand in the spread.
use super::{Row, SECTION, SECTION2};
use crate::transcendental::Transcendental;

/// E[S^r; event] by the second-order delta method, from S's partial
/// moments `s1` = E[S; kept] and `s2` = E[S²; kept] on an event of chance
/// `kept`. Kept on the side of μ^r that Jensen's inequality puts it, so a
/// wide spread never turns it over.
pub(crate) fn delta(kept: f64, s1: f64, s2: f64, r: f64) -> f64 {
    if kept <= 0.0 || s1 <= 0.0 {
        return 0.0;
    }
    if r == 1.0 {
        return s1;
    }
    let mean = s1 / kept;
    let spread = (s2 / kept - mean * mean).max(0.0);
    let power = mean.powf_fixed(r);
    let second = power * (1.0 + r * (r - 1.0) / 2.0 * spread / (mean * mean));
    let taken = if r < 1.0 {
        second.clamp(0.0, power)
    } else {
        second.max(power)
    };
    kept * taken
}

/// A borne subtree's section handed to a bearer of exponent `e`, at its
/// table row: E[X; T <= t] and E[X²; T <= t] for X = S^(e / its own).
pub(super) fn handed(row: Row, own: f64, e: f64) -> (f64, f64) {
    let (_, kept, Some(b)) = row else {
        return (0.0, 0.0);
    };
    let r = e / own;
    (
        delta(kept, b[SECTION], b[SECTION2], r),
        delta(kept, b[SECTION], b[SECTION2], 2.0 * r),
    )
}

/// One zone's section on its factor: each node bears its own pipe `own`
/// (as o^e and o^2e) and `places` places, each a factor `phi` and, in
/// one of them, the moments `slots[k]` (E[X], E[X²] on it). Over the node
/// count's law `pmf`, each node's factor `f`: E[Π f · S] and E[Π f · S²].
pub(super) fn zoned(
    pmf: &[f64],
    (f, phi, places): (f64, f64, i32),
    own: (f64, f64),
    slots: &[(f64, f64)],
) -> (f64, f64) {
    let one = |p: i32| if p >= 0 { phi.powi(p) } else { 0.0 };
    let sum1: f64 = slots.iter().map(|s| s.0).sum();
    let sum2: f64 = slots.iter().map(|s| s.1).sum();
    let pairs = sum1 * sum1 - slots.iter().map(|s| s.0 * s.0).sum::<f64>();
    let node1 = f * own.0 + one(places - 1) * sum1;
    let node2 = f * own.1
        + 2.0 * own.0 * one(places - 1) * sum1
        + one(places - 1) * sum2
        + one(places - 2) * pairs;
    // E[N f^(N-1)] and E[N (N-1) f^(N-2)].
    let (mut c1, mut c2) = (0.0, 0.0);
    for (n, &p) in pmf.iter().enumerate().skip(1) {
        let n = n as i32;
        c1 += p * f64::from(n) * f.powi(n - 1);
        if n >= 2 {
            c2 += p * f64::from(n * (n - 1)) * f.powi(n - 2);
        }
    }
    (c1 * node1, c1 * node2 + c2 * node1 * node1)
}
