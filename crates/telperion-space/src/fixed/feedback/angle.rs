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
//! - **sin, cos and atan2** are CORDIC on 62 fraction bits, 40 iterations,
//!   with the literal table and gain below (`feedback-constants.py`).
use super::vector::isqrt;
use super::Fault;
use crate::fixed::ONE32;

/// A quarter, a half and an eighth of a turn, as T.
pub const QUARTER: u32 = 1 << 30;
pub const HALF: u32 = 1 << 31;

/// `2 pi` in Q60 and `1 / (2 pi)` in Q64, each the nearest.
pub const TAU60: i64 = 0x6487_ed51_10b4_611a;
pub const INV_TAU64: u64 = 0x28be_60db_9391_054a;

/// CORDIC's iterations.
pub const STEPS: usize = 40;

/// `atan(2^-i)` in turns, 2^62 a turn, each the nearest.
pub const ATAN: [i64; STEPS] = [
    0x800000000000000,
    0x4b90147677cc21a,
    0x27ece16d7b8e7a3,
    0x144447507776687,
    0xa2c350c39626bb,
    0x5175f85641189e,
    0x28bd87970a098a,
    0x145f15447510ac,
    0xa2f94d1b430ce,
    0x517cbaecc2ace,
    0x28be600246e9f,
    0x145f3052a032e,
    0xa2f98337fb18,
    0x517cc1b05cbd,
    0x28be60daba44,
    0x145f306dae9f,
    0xa2f9836e17f,
    0x517cc1b7205,
    0x28be60db92b,
    0x145f306dc9b,
    0xa2f9836e4e,
    0x517cc1b727,
    0x28be60db94,
    0x145f306dca,
    0xa2f9836e5,
    0x517cc1b72,
    0x28be60db9,
    0x145f306dd,
    0xa2f9836e,
    0x517cc1b7,
    0x28be60dc,
    0x145f306e,
    0xa2f9837,
    0x517cc1b,
    0x28be60e,
    0x145f307,
    0xa2f983,
    0x517cc2,
    0x28be61,
    0x145f30,
];

/// CORDIC's gain over its iterations, `prod (1 + 2^-2i)^-1/2`, Q62.
pub const K: i64 = 0x26dd_3b6a_10d7_969a;

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

/// `(cos t, sin t)`, Q32, each in [-1, 1]: exact at every quarter turn.
pub fn sincos(t: u32) -> (i64, i64) {
    // The nearest quarter, and the rest in [-1/8, 1/8) turn.
    let q = t.wrapping_add(1 << 29) >> 30;
    let r = t.wrapping_sub(q << 30) as i32;
    let (c, s) = if r == 0 { (ONE32, 0) } else { rotated(r) };
    match q {
        0 => (c, s),
        1 => (-s, c),
        2 => (-c, -s),
        _ => (s, -c),
    }
}

/// CORDIC in rotation mode from `(K, 0)` through `r` (T units): every
/// step updates x and y from their old values together, its direction
/// positive where the angle left is none or more.
fn rotated(r: i32) -> (i64, i64) {
    let (mut x, mut y, mut z) = (K, 0i64, i64::from(r) << 30);
    for (i, &a) in ATAN.iter().enumerate() {
        let (dx, dy) = (y >> i, x >> i);
        if z >= 0 {
            (x, y, z) = (x - dx, y + dy, z - a);
        } else {
            (x, y, z) = (x + dx, y - dy, z + a);
        }
    }
    let q32 = |v: i64| (v >> 30).clamp(-ONE32, ONE32);
    (q32(x), q32(y))
}

/// The orientation of `(x, y)`: exact on the axes, `atan2(0, 0)` none.
pub fn atan2(y: i64, x: i64) -> u32 {
    match (y.signum(), x.signum()) {
        (0, -1) => return HALF,
        (0, _) => return 0,
        (1, 0) => return QUARTER,
        (-1, 0) => return QUARTER.wrapping_mul(3),
        _ => {}
    }
    // The larger magnitude brought to [2^60, 2^61), each magnitude shifted
    // toward zero, so CORDIC's growth stays inside an i64.
    let top = 63 - x.unsigned_abs().max(y.unsigned_abs()).leading_zeros() as i32;
    let scale = |v: i64| {
        let m = v.unsigned_abs();
        let m = if top <= 60 {
            m << (60 - top)
        } else {
            m >> (top - 60)
        } as i64;
        if v < 0 {
            -m
        } else {
            m
        }
    };
    let (mut x, mut y) = (scale(x), scale(y));
    let base = if x < 0 {
        (x, y) = (-x, -y);
        HALF
    } else {
        0
    };
    let mut z = 0i64;
    for (i, &a) in ATAN.iter().enumerate() {
        let (dx, dy) = (y >> i, x >> i);
        if y >= 0 {
            (x, y, z) = (x + dx, y - dy, z + a);
        } else {
            (x, y, z) = (x - dx, y + dy, z - a);
        }
    }
    base.wrapping_add(((z + (1 << 29)) >> 30) as u32)
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
