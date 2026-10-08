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
/// steps: the same bits as any exact algorithm, without a 128-bit division
/// (20 ns against Newton's 27 on the 124-bit sums `unit` takes).
pub fn isqrt(n: u128) -> u128 {
    let wide = |x: u64| u128::from(x) * u128::from(x);
    // The estimate from the two halves' hardware conversions: within 2^11
    // of the root, which lies below 2^64.
    let f = (n >> 64) as u64 as f64 * 18_446_744_073_709_551_616.0 + n as u64 as f64;
    let mut s = f.sqrt() as u64;
    if s > 0 {
        // One Newton step from the exact residual (below 2^76 in magnitude).
        let r = n.wrapping_sub(wide(s)) as i128;
        let step = ((r >> 16) as i64 as f64 * 65_536.0 / (2.0 * s as f64)) as i64;
        s = s.saturating_add_signed(step);
    }
    while wide(s) > n {
        s -= 1;
    }
    while s < u64::MAX && wide(s + 1) <= n {
        s += 1;
    }
    u128::from(s)
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

/// The unit components, Q32, each `c 2^32 / root` toward zero (a 128-bit
/// numerator over a 64-bit divisor: the hardware's fast path).
fn unit_of(c: [i64; 3], root: u64) -> [i64; 3] {
    c.map(|x| ((i128::from(x) << 32) / i128::from(root)) as i64)
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
/// `unit(from × to)`, its cosine and sine `(from · to, |from × to|)`,
/// unit by construction up to Q32 rounding, so made unit by one Newton
/// step, `(3 - (c^2 + s^2)) / 2` (host decision, 2026-10-07: no
/// normalisation where a vector is unit by construction). None where the
/// two are parallel or opposed exactly.
pub fn carry(from: [i64; 3], to: [i64; 3]) -> Option<Turn> {
    use crate::fixed::{mul, ONE32};
    let axis = cross(from, to);
    let pivot = unit(axis)?;
    let (c, s) = (dot(from, to), length(axis));
    let norm = ((i128::from(c) * i128::from(c) + i128::from(s) * i128::from(s)) >> 32) as i64;
    let step = (3 * ONE32 - norm) >> 1;
    Some(Turn {
        pivot,
        cos: mul(c, step),
        sin: mul(s, step),
    })
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
