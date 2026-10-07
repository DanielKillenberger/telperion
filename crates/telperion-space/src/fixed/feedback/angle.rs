//! Angles (CONTRACT-A.md 2.3; Astra's review, points 1 and 2).
//!
//! - **An orientation** is a T, a u32 of a turn's 2^32: an azimuth, a
//!   phyllotactic position, a direction's elevation. It wraps.
//! - **An amount** is an i64 of Q32 turns: a rotation, an elevation's
//!   difference, a rate times a rank, a sag's fall. It never wraps; it is
//!   made an orientation (`wrap`) only where one is read, so a product
//!   keeps its whole turns.
//! - **The difference of two orientations** is the amount in [-1/2, 1/2)
//!   turn (`between`); an interpolation adds a share of it (`toward`).
//! - **A half angle** halves the amount, floored, before any wrap.
//! - **Radians** are Q32; the conversions are one product by a constant
//!   of 60 or 64 fraction bits (`TAU60`, `INV_TAU64`), floored.
//! - **sin, cos and atan2** are Q62 polynomials on the rest from a table's
//!   nearest point (host decision, 2026-10-07; `sincos`, `atan2`).
use super::vector::isqrt;
use super::Fault;
use crate::fixed::ONE32;

/// A quarter, a half and an eighth of a turn, as T.
pub const QUARTER: u32 = 1 << 30;
pub const HALF: u32 = 1 << 31;

/// `2 pi` in Q60 and `1 / (2 pi)` in Q64, each the nearest.
pub const TAU60: i64 = 0x6487_ed51_10b4_611a;
pub const INV_TAU64: u64 = 0x28be_60db_9391_054a;

/// The amounts whose radians stay in an i64: below 2^60, 2^28 turns.
pub const AMOUNT: i64 = 1 << 60;

/// An amount as an orientation.
pub fn wrap(amount: i64) -> u32 {
    amount as u32
}

/// The amount from `from` to `to`, in [-1/2, 1/2) turn.
pub fn between(from: u32, to: u32) -> i64 {
    i64::from(to.wrapping_sub(from) as i32)
}

/// `from` moved `share` (Q32) of the way to `to`, the short way:
/// `from + floor(between share)`.
pub fn toward(from: u32, to: u32, share: i64) -> u32 {
    let step = (i128::from(between(from, to)) * i128::from(share)) >> 32;
    from.wrapping_add(step as u32)
}

/// Half an amount, floored.
pub fn half(amount: i64) -> i64 {
    amount >> 1
}

/// An amount in radians, Q32, floored; `Range` past `AMOUNT`.
pub fn radians(amount: i64) -> Result<i64, Fault> {
    if amount.unsigned_abs() >= AMOUNT as u64 {
        return Err(Fault::Range);
    }
    Ok(((i128::from(amount) * i128::from(TAU60)) >> 60) as i64)
}

/// Radians, Q32, as an amount, floored.
pub fn turns(radians: i64) -> i64 {
    ((i128::from(radians) * i128::from(INV_TAU64)) >> 64) as i64
}

/// `sin` and `cos` coefficients in Q62, the nearest: `(-1)^k / (2k + 1)!`
/// and `(-1)^k / (2k)!` for k from 0 to 5 (host decision, 2026-10-07).
pub const SIN: [i64; 6] = [
    0x4000_0000_0000_0000,
    -0x0aaa_aaaa_aaaa_aaab,
    0x0088_8888_8888_8889,
    -0x0003_4034_0340_3403,
    0x0000_0b8e_f1d2_ab64,
    -0x0000_001a_e645_67f5,
];
pub const COS: [i64; 6] = [
    0x4000_0000_0000_0000,
    -0x2000_0000_0000_0000,
    0x02aa_aaaa_aaaa_aaab,
    -0x0016_c16c_16c1_6c17,
    0x0000_6806_8068_0680,
    -0x0000_0127_e4fb_778a,
];

/// `sqrt(1/2)` in Q62, the nearest.
pub const HALF_ROOT: i64 = 0x2d41_3ccc_fe77_9921;

/// `a b / 2^62`, floored.
fn mul62(a: i64, b: i64) -> i64 {
    ((i128::from(a) * i128::from(b)) >> 62) as i64
}

/// A polynomial in `z` (Q62) by Horner's rule, from its highest term down,
/// one floor a product.
fn horner(c: &[i64; 6], z: i64) -> i64 {
    c.iter().rev().fold(0, |p, &k| k + mul62(z, p))
}

/// `(cos t, sin t)`, Q32, each in [-1, 1]: exact at every quarter turn
/// (host decision, 2026-10-07, CONTRACT-A.md 2.3).
/// 1. The nearest eighth `n = (t + 2^28) >> 29`, and the rest `r = (t - n
///    2^29) as i32`, within 1/16 turn.
/// 2. `θ = floor(r TAU60 / 2^30)` (Q62 radians) and `z = floor(θ^2 / 2^62)`.
/// 3. `sin = floor(θ horner(SIN, z) / 2^62)`, `cos = horner(COS, z)`.
/// 4. An odd eighth turns them by 1/8 turn: `(floor((c - s) H / 2^62),
///    floor((c + s) H / 2^62))`, H = `HALF_ROOT`; then `n >> 1` quarter
///    turns, exactly (a swap and negations).
/// 5. Each floored to Q32 (`>> 30`) and clamped to [-1, 1].
pub fn sincos(t: u32) -> (i64, i64) {
    let n = t.wrapping_add(1 << 28) >> 29;
    let r = t.wrapping_sub(n << 29) as i32;
    let theta = ((i128::from(r) * i128::from(TAU60)) >> 30) as i64;
    let z = mul62(theta, theta);
    let (mut c, mut s) = (horner(&COS, z), mul62(theta, horner(&SIN, z)));
    if n & 1 == 1 {
        (c, s) = (mul62(c - s, HALF_ROOT), mul62(c + s, HALF_ROOT));
    }
    let (c, s) = match n >> 1 {
        0 => (c, s),
        1 => (-s, c),
        2 => (-c, -s),
        _ => (s, -c),
    };
    let q32 = |v: i64| (v >> 30).clamp(-ONE32, ONE32);
    (q32(c), q32(s))
}

/// `atan(j/32)` in turns at 2^62 a turn, j = 0..32, the nearest
/// (`atan-constants.py`; host decision, 2026-10-07).
const ATAN_TABLE: [i64; 33] = [
    0x0, 0x5175f85641189e, 0xa2c350c39626bb,
    0xf3bfe26b0d0241, 0x144447507776687, 0x1942b2da6d3f8eb,
    0x1e34ff3a10b9cbb, 0x23190c955d95868, 0x27ece16d7b8e7a3,
    0x2caeae551d62da2, 0x315cd0aa131dd86, 0x35f5d453a87bf6f,
    0x3a7874905e7569e, 0x3ee39be4fd329a4, 0x43366344a4fb911,
    0x4770108d568f037, 0x4b90147677cc21a, 0x4f96080f4f0d62a,
    0x5381a9ea8f1f77c, 0x5752db122231dc9, 0x5b099bdbb7f0790,
    0x5ea608b37a1c304, 0x622856eef2a18f9, 0x6590d1b6d25b882,
    0x68dfd7131067bcd, 0x6c15d521e396c23, 0x6f33477f7409c90,
    0x7238b4e1dba3856, 0x7526aceb22c6ed4, 0x77fdc63158f85d0,
    0x7abe9c7bafc8e9f, 0x7d69cf329142823, 0x800000000000000,
];

/// `atan u`'s series in turns, `(-1)^k / ((2k + 1) 2pi)`, k = 0..3, Q62.
const ATAN_POLY: [i64; 4] = [0xa2f9836e4e44153, -0x36532bcf6f6c071, 0x20984d7c760d9dd, -0x17483758e69c030];

/// The orientation of `(x, y)` (host decision, 2026-10-07: a
/// table-reduced polynomial in place of CORDIC), specified to the bit:
/// 1. **Axes, exact:** `y = 0` gives 0 (a half turn where `x < 0`;
///    `atan2(0, 0) = 0`); `x = 0` a quarter (three where `y < 0`).
/// 2. The magnitudes' lesser over their greater, `t = floor(lo 2^62 /
///    hi)` in Q62, in [0, 1].
/// 3. The nearest thirty-second `j = (t + 2^56) >> 57`, `c = j 2^57`; the
///    rest `u = trunc((t - c) 2^62 / (2^62 + floor(t c / 2^62)))`, below
///    1/64 in magnitude: `atan t = atan(j/32) + atan u`.
/// 4. `a = A_j + floor(u P(z) / 2^62)` in turns at 2^62, `z = floor(u^2 /
///    2^62)`, `P` Horner over `ATAN_POLY` from its fourth coefficient down.
/// 5. Where the greater was `y`, `a = 2^60 - a`; where `x < 0`, `2^61 - a`;
///    where `y < 0`, `-a`.
/// 6. To the nearest T, ties up: `(a + 2^29) >> 30`, wrapped.
pub fn atan2(y: i64, x: i64) -> u32 {
    match (y.signum(), x.signum()) {
        (0, -1) => return HALF,
        (0, _) => return 0,
        (1, 0) => return QUARTER,
        (-1, 0) => return QUARTER.wrapping_mul(3),
        _ => {}
    }
    let (mx, my) = (x.unsigned_abs(), y.unsigned_abs());
    let (lo, hi, steep) = if my > mx { (mx, my, true) } else { (my, mx, false) };
    let t = ((u128::from(lo) << 62) / u128::from(hi)) as i64;
    let j = ((t + (1 << 56)) >> 57) as usize;
    let c = (j as i64) << 57;
    let den = (1i128 << 62) + ((i128::from(t) * i128::from(c)) >> 62);
    let u = ((i128::from(t - c) << 62) / den) as i64;
    let z = ((i128::from(u) * i128::from(u)) >> 62) as i64;
    let p = ATAN_POLY
        .iter()
        .rev()
        .fold(0i64, |p, &k| k + ((i128::from(z) * i128::from(p)) >> 62) as i64);
    let mut a = ATAN_TABLE[j] + ((i128::from(u) * i128::from(p)) >> 62) as i64;
    if steep {
        a = (1 << 60) - a;
    }
    if x < 0 {
        a = (1 << 61) - a;
    }
    if y < 0 {
        a = -a;
    }
    ((a + (1 << 29)) >> 30) as u32
}

/// `asin x` of a Q32 `x`, clamped to [-1, 1], as an orientation in
/// [-1/4, 1/4] turn: `atan2(x, isqrt(1 - x^2))`.
pub fn asin(x: i64) -> u32 {
    let x = x.clamp(-ONE32, ONE32);
    atan2(x, rest(x))
}

/// `acos c` of a Q32 `c`, clamped to [-1, 1], in [0, 1/2] turn.
pub fn acos(c: i64) -> u32 {
    let c = c.clamp(-ONE32, ONE32);
    atan2(rest(c), c)
}

/// `sqrt(1 - x^2)` in Q32, floored, of a Q32 `x` in [-1, 1].
fn rest(x: i64) -> i64 {
    let one = 1u128 << 64;
    isqrt(one - (i128::from(x) * i128::from(x)) as u128) as i64
}
