//! Lengths and unit vectors (CONTRACT-A.md 2.3; Astra's review, points 1
//! and 2). A vector's components are brought to a common scale before the
//! root, the largest magnitude to [2^61, 2^62), so a small vector keeps
//! its direction (raw `(1, 1, 0)` is the unit diagonal, not `(1, 1, 0)`)
//! and a W lever is normalised from W, never shortened to Q32 first.
//! Every scaling shifts magnitudes, toward zero, so it is symmetric about
//! zero; the quotient is truncated toward zero.
use crate::fixed::{log2, INF};

/// `floor(sqrt n)`, specified by its result (CONTRACT-A.md 2.3). Taken as
/// an f64 estimate, one correction from its exact residual, then exact
/// steps: the same bits as any exact algorithm, without a 128-bit division.
pub fn isqrt(n: u128) -> u128 {
    // The root of a u128 lies below 2^64; the estimate is within 2^11.
    let mut s = ((n as f64).sqrt() as u128).min(u128::from(u64::MAX));
    if s > 0 {
        let sq = s * s;
        let step = |d: u128| ((d as f64) / (2.0 * s as f64)).round() as u128;
        s = if n >= sq { s + step(n - sq) } else { s - step(sq - n).min(s) };
        s = s.min(u128::from(u64::MAX));
    }
    while s * s > n {
        s -= 1;
    }
    while (s + 1).checked_mul(s + 1).is_some_and(|q| q <= n) {
        s += 1;
    }
    s
}

/// The length of a Q32 vector, Q32, floored: the root of its squares'
/// exact sum.
pub fn length(v: [i64; 3]) -> i64 {
    let sum: u128 = v.iter().map(|&c| u128::from(c.unsigned_abs()).pow(2)).sum();
    isqrt(sum) as i64
}

/// A magnitude shifted left by `s`, or right by `-s` toward zero.
fn scaled(m: u128, s: i32) -> u128 {
    if s >= 0 {
        m << s
    } else {
        m >> (-s).min(127)
    }
}

/// The components brought to the common scale, with the shift taken, and
/// their root: none for the zero vector.
fn normal(v: [i128; 3]) -> Option<([i64; 3], u64, i32)> {
    let most = v.iter().map(|c| c.unsigned_abs()).max().unwrap_or(0);
    if most == 0 {
        return None;
    }
    let shift = 61 - (127 - most.leading_zeros() as i32);
    let c = v.map(|c| {
        let m = scaled(c.unsigned_abs(), shift) as i64;
        if c < 0 {
            -m
        } else {
            m
        }
    });
    // Each square is below 2^124, so the sum is below 2^126.
    let sum: u128 = c.iter().map(|&x| u128::from(x.unsigned_abs()).pow(2)).sum();
    Some((c, isqrt(sum) as u64, shift))
}

/// The unit components, Q32, each `c 2^32 / root` toward zero.
fn unit_of(c: [i64; 3], root: u64) -> [i64; 3] {
    c.map(|x| over_root(x, root))
}

/// `trunc(c 2^32 / root)` for `|c| < 2^63`, `root` in `[2^61, 2^63)`: an
/// f64 estimate made exact by 96-bit products, without a 128-bit division.
fn over_root(c: i64, root: u64) -> i64 {
    let (num, d) = (u128::from(c.unsigned_abs()) << 32, u128::from(root));
    let mut q = (c.unsigned_abs() as f64 * 4_294_967_296.0 / root as f64) as u128;
    while q * d > num {
        q -= 1;
    }
    while (q + 1) * d <= num {
        q += 1;
    }
    if c < 0 {
        -(q as i64)
    } else {
        q as i64
    }
}

/// The unit vector along a Q32 vector: none for the zero vector exactly.
pub fn unit(v: [i64; 3]) -> Option<[i64; 3]> {
    let (c, root, _) = normal(v.map(i128::from))?;
    Some(unit_of(c, root))
}

/// The unit vector along a W vector, Q32, and the log2 of its length (Q32,
/// of the real length): `([0; 3], -INF)` for the zero vector.
pub fn unit_w(v: [i128; 3]) -> ([i64; 3], i64) {
    match normal(v) {
        None => ([0; 3], -INF),
        Some((c, root, shift)) => {
            let log = log2(root) - (i64::from(shift + 64) << 32);
            (unit_of(c, root), log)
        }
    }
}

/// A rotation built from vectors (host decision, 2026-10-07: trig only
/// where an angle is a stated quantity): its unit pivot and its Q32 cosine
/// and sine, each pair normalised in fixed point.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Turn {
    pub pivot: [i64; 3],
    pub cos: i64,
    pub sin: i64,
}

/// `a · b` of Q32 vectors: the exact sum, one floor.
pub fn dot(a: [i64; 3], b: [i64; 3]) -> i64 {
    ((0..3)
        .map(|k| i128::from(a[k]) * i128::from(b[k]))
        .sum::<i128>()
        >> 32) as i64
}

/// `a × b`, exact, Q64.
fn cross_wide(a: [i64; 3], b: [i64; 3]) -> [i128; 3] {
    let p = |i: usize, j: usize| i128::from(a[i]) * i128::from(b[j]);
    [p(1, 2) - p(2, 1), p(2, 0) - p(0, 2), p(0, 1) - p(1, 0)]
}

/// `a × b`, each component one floor.
pub fn cross(a: [i64; 3], b: [i64; 3]) -> [i64; 3] {
    cross_wide(a, b).map(|c| (c >> 32) as i64)
}

/// The least rotation taking unit `from` to unit `to`: about
/// `unit(from × to)`, its cosine and sine the unit of `(from · to,
/// |from × to|)`. None where the two are parallel or opposed exactly.
pub fn carry(from: [i64; 3], to: [i64; 3]) -> Option<Turn> {
    let axis = cross(from, to);
    let pivot = unit(axis)?;
    let [cos, sin, _] = unit([dot(from, to), length(axis), 0])?;
    Some(Turn { pivot, cos, sin })
}

/// `v` turned by `t` (Rodrigues): `v cos + (p × v) sin + p (p · v)(1 -
/// cos)`, each term one floor a component from exact intermediates (the
/// last a 256-bit product shifted by 96), the three added exactly.
pub fn turned(v: [i64; 3], t: &Turn) -> [i64; 3] {
    use super::wide::I256;
    use crate::fixed::{mul, ONE32};
    let across = cross_wide(t.pivot, v);
    let along: i128 = (0..3)
        .map(|k| i128::from(t.pivot[k]) * i128::from(v[k]))
        .sum();
    let narrow = |x: I256| x.narrow64().expect("a turned vector inside Q32");
    std::array::from_fn(|k| {
        let a = mul(v[k], t.cos);
        let b = narrow(I256::mul(across[k], i128::from(t.sin)).floor_shift(64));
        let fall = i128::from(t.pivot[k]) * i128::from(ONE32 - t.cos);
        let c = narrow(I256::mul(along, fall).floor_shift(96));
        a + b + c
    })
}
