//! Lengths and unit vectors (CONTRACT-A.md 2.3; Astra's review, points 1
//! and 2). A vector's components are brought to a common scale before the
//! root, the largest magnitude to [2^61, 2^62), so a small vector keeps
//! its direction (raw `(1, 1, 0)` is the unit diagonal, not `(1, 1, 0)`)
//! and a W lever is normalised from W, never shortened to Q32 first.
//! Every scaling shifts magnitudes, toward zero, so it is symmetric about
//! zero; the quotient is truncated toward zero.
use crate::fixed::{log2, INF};

/// `floor(sqrt n)`.
pub fn isqrt(n: u128) -> u128 {
    if n == 0 {
        return 0;
    }
    // Newton from above: the first iterate not below its successor.
    let mut x = 1u128 << (128 - n.leading_zeros()).div_ceil(2);
    loop {
        let y = (x + n / x) >> 1;
        if y >= x {
            return x;
        }
        x = y;
    }
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
