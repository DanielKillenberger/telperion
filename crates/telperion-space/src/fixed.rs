//! The decision contract's arithmetic (fn-209 F1, FINE-CONTRACT.md, host
//! decision 1): time in whole ticks of 2^-20 cycle, and every threshold a
//! decision computes per bud taken by one fixed-point procedure, so the
//! CPU reference executor and the GPU executor, which mirrors this file in
//! 32-bit words, take the same decision for the same draw.
//!
//! - **Time** is a count of ticks. The CPU holds it in an f64, where every
//!   count below 2^53 and every sum and difference of two is exact, so its
//!   comparisons are the integer ones; `grid` puts a lifespan or a waking
//!   time on the grid where it enters.
//! - **A probability** is Q62: `p` is `n / 2^62` (`q62`). Its threshold
//!   against a 53-bit draw is `ceil(n / 2^9)` (`threshold`).
//! - **A log** is Q32 in an i64: base 2 (`log2`) or natural (`ln_ratio`),
//!   saturated at `INF`, so sums of logs are exact and order-free.
//! - `log2` takes an integer's mantissa through 32 squarings; `exp2_neg`
//!   multiplies the constants `2^-(2^-i)` its fraction's bits select. Every
//!   product is 64 by 64 bits, its high part kept by a truncating shift.
//!
//! Values computed once per species point (the closed form's tables, a
//! species' settings) are inputs: the CPU computes them in f64 and hands
//! both executors the same integers.

/// Ticks a cycle: 2^20.
pub const CYCLE: f64 = (1u64 << 20) as f64;

/// The time on the tick grid nearest `t` (`round`, half away from zero).
pub fn grid(t: f64) -> f64 {
    if !t.is_finite() {
        return t;
    }
    (t * CYCLE).round() / CYCLE
}

/// A time on the grid as its ticks.
pub fn ticks(t: f64) -> u64 {
    debug_assert_eq!(grid(t), t, "a time off the tick grid");
    (t * CYCLE) as u64
}

/// A certain probability, Q62.
pub const P_ONE: u64 = 1 << 62;
/// Q32's one.
pub const ONE32: i64 = 1 << 32;
/// A log beyond any finite one this contract meets: a probability of none.
pub const INF: i64 = 1 << 62;

/// `ln 2` and `log2 e` in Q62.
const LN2: i64 = 0x2c5c_85fd_f473_de6a;
const LOG2E: i64 = 0x5c55_1d94_ae0b_f85d;

/// `2^-(2^-(i + 1))` in Q62, floored.
const ROOTS: [u64; 32] = [
    0x2d41_3ccc_fe77_9921,
    0x35d1_3f32_b5a7_5abd,
    0x3ab0_31b9_f749_0e4b,
    0x3d49_5f45_4921_b30b,
    0x3ea0_ecb6_dc8a_80ce,
    0x3f4f_8303_3d21_b05d,
    0x3fa7_8457_1ee3_e212,
    0x3fd3_b2d6_5447_b229,
    0x3fe9_d594_8723_6bb4,
    0x3ff4_e9d4_703d_f842,
    0x3ffa_74ac_bdf6_c9d5,
    0x3ffd_3a46_ffc6_e30f,
    0x3ffe_9d1f_a801_0199,
    0x3fff_4e8e_de05_3ad4,
    0x3fff_a747_3183_76ac,
    0x3fff_d3a3_8961_e6fe,
    0x3fff_e9d1_c0d8_fd14,
    0x3fff_f4e8_df76_80c4,
    0x3fff_fa74_6f7d_c0eb,
    0x3fff_fd3a_37af_8097,
    0x3fff_fe9d_1bd3_e854,
    0x3fff_ff4e_8de8_fe2c,
    0x3fff_ffa7_46f4_4196,
    0x3fff_ffd3_a37a_116b,
    0x3fff_ffe9_d1bd_04dd,
    0x3fff_fff4_e8de_8178,
    0x3fff_fffa_746f_407e,
    0x3fff_fffd_3a37_a030,
    0x3fff_fffe_9d1b_d014,
    0x3fff_ffff_4e8d_e809,
    0x3fff_ffff_a746_f404,
    0x3fff_ffff_d3a3_7a02,
];

/// The whole product of two integers. Wasm has no 64-by-64-bit product
/// and takes a 128-bit one by a call, most of growth's arithmetic on one
/// core there, so it builds the product from 32-bit halves instead: the
/// same integer either way (test).
pub fn wide(a: u64, b: u64) -> u128 {
    if cfg!(target_arch = "wasm32") {
        by_halves(a, b)
    } else {
        u128::from(a) * u128::from(b)
    }
}

/// `wide` of two signed integers.
pub fn wide_signed(a: i64, b: i64) -> i128 {
    if cfg!(target_arch = "wasm32") {
        signed_by_halves(a, b)
    } else {
        i128::from(a) * i128::from(b)
    }
}

/// `a b` by halves of their two's complements, less what each sign adds.
fn signed_by_halves(a: i64, b: i64) -> i128 {
    let (au, bu) = (a as u64, b as u64);
    let mut p = by_halves(au, bu);
    if a < 0 {
        p = p.wrapping_sub(u128::from(bu) << 64);
    }
    if b < 0 {
        p = p.wrapping_sub(u128::from(au) << 64);
    }
    p as i128
}

/// `a b` from the four products of their 32-bit halves.
fn by_halves(a: u64, b: u64) -> u128 {
    let (ah, al, bh, bl) = (a >> 32, a & 0xffff_ffff, b >> 32, b & 0xffff_ffff);
    let (mid, carry) = (ah * bl).overflowing_add(al * bh);
    (u128::from(ah * bh) << 64)
        + (u128::from(carry) << 96)
        + (u128::from(mid) << 32)
        + u128::from(al * bl)
}

/// The high part of a Q62 product: `a b / 2^62`, truncated.
pub fn mul62(a: u64, b: u64) -> u64 {
    (wide(a, b) >> 62) as u64
}

/// A probability as Q62, floored: none for none or NaN, one from one.
pub fn q62(p: f64) -> u64 {
    if p.is_nan() || p <= 0.0 {
        return 0;
    }
    if p >= 1.0 {
        return P_ONE;
    }
    // Positive, so the cast's truncation is the floor, without a call.
    (p * P_ONE as f64) as u64
}

/// A real as Q32, to the nearest (saturated at `INF`).
pub fn q32(x: f64) -> i64 {
    if x.is_nan() {
        return 0;
    }
    (x * ONE32 as f64).round().clamp(-INF as f64, INF as f64) as i64
}

/// A Q62 probability's threshold against a 53-bit draw: the draws under
/// it are those whose `d / 2^53` lies below `p / 2^62`.
pub fn threshold(p: u64) -> u64 {
    (p.min(P_ONE) + 511) >> 9
}

/// `log2 n` in Q32; `-INF` for none.
pub fn log2(n: u64) -> i64 {
    if n == 0 {
        return -INF;
    }
    let (e, mut m) = mantissa(n);
    let mut frac = 0u64;
    for _ in 0..32 {
        frac = (frac << 1) | square(&mut m);
    }
    (e << 32) | frac as i64
}

/// `log2` of two non-zero integers at once: the same steps, the two
/// chains of squarings interleaved so one core runs them side by side.
fn log2_pair(a: u64, b: u64) -> (i64, i64) {
    let ((ea, mut ma), (eb, mut mb)) = (mantissa(a), mantissa(b));
    let (mut fa, mut fb) = (0u64, 0u64);
    for _ in 0..32 {
        fa = (fa << 1) | square(&mut ma);
        fb = (fb << 1) | square(&mut mb);
    }
    ((ea << 32) | fa as i64, (eb << 32) | fb as i64)
}

/// A non-zero `n`'s exponent and its mantissa in Q62, in [1, 2), held
/// doubled (Q63, its last bit clear) so a square's high word is the Q62
/// square; a 64-bit `n` loses its last bit.
fn mantissa(n: u64) -> (i64, u64) {
    let zeros = n.leading_zeros();
    (i64::from(63 - zeros), (n << zeros) & !1)
}

/// One step of `log2`: the mantissa squared in Q62, truncated, and halved
/// back into [1, 2) where it reached 2; the bit is whether it did.
fn square(m: &mut u64) -> u64 {
    // The high word of the doubled mantissa's square: the Q62 square.
    let square = (wide(*m, *m) >> 64) as u64;
    let bit = square >> 63;
    // Halved and doubled, or doubled: its last bit stays clear.
    *m = if bit == 1 { square & !1 } else { square << 1 };
    bit
}

/// `2^-x` for `x` in Q32, as a Q62 probability: one for none or less.
pub fn exp2_neg(x: i64) -> u64 {
    if x <= 0 {
        return P_ONE;
    }
    let whole = x >> 32;
    if whole >= 63 {
        return 0;
    }
    let frac = x as u64 & 0xffff_ffff;
    let mut r = P_ONE;
    for (i, &root) in ROOTS.iter().enumerate() {
        if frac & (1 << (31 - i)) != 0 {
            r = mul62(r, root);
        }
    }
    r >> whole
}

/// `2^y` for `y` in Q32, in Q32, saturated at `INF`.
pub fn exp2(y: i64) -> i64 {
    if y <= 0 {
        return (exp2_neg(-y) >> 30) as i64;
    }
    // 2^y = 2^(n + 1) 2^-(1 - f), its integer part n and fraction f.
    let n = (y >> 32) + 1;
    if n >= 62 {
        return INF;
    }
    let below = exp2_neg(ONE32 - (y & 0xffff_ffff));
    ((u128::from(below) << n) >> 30).min(INF as u128) as i64
}

/// `-log2 p` of a Q62 probability, in Q32: `INF` for none.
pub fn neg_log2(p: u64) -> i64 {
    if p == 0 {
        return INF;
    }
    (62 << 32) - log2(p.min(P_ONE))
}

/// The probability `2^-l` of a `-log2`, Q62.
pub fn prob(l: i64) -> u64 {
    if l >= INF {
        return 0;
    }
    exp2_neg(l)
}

/// `ln(a / b)` in Q32: `-INF` where `a` is none, `INF` where `b` is.
pub fn ln_ratio(a: u64, b: u64) -> i64 {
    match (a, b) {
        (0, _) => -INF,
        (_, 0) => INF,
        _ => {
            let (la, lb) = log2_pair(a, b);
            (wide_signed(la - lb, LN2) >> 62) as i64
        }
    }
}

/// `e^x` for `x` in Q32 (natural), in Q32.
pub fn exp(x: i64) -> i64 {
    if x <= -INF {
        return 0;
    }
    exp2((wide_signed(x, LOG2E) >> 62) as i64)
}

/// `a b` of two Q32 values, in Q32, floored and saturated.
pub fn mul(a: i64, b: i64) -> i64 {
    (wide_signed(a, b) >> 32).clamp(-i128::from(INF), i128::from(INF)) as i64
}

/// A probability raised to `share` ticks of a cycle: `-log2 p` scaled by
/// the share, `l s / 2^20`, floored and saturated.
pub fn scaled(l: i64, share: u64) -> i64 {
    if l >= INF {
        return INF;
    }
    (wide_signed(l, share as i64) >> 20).min(i128::from(INF)) as i64
}

/// A presence's fixed point: `lead / window`, in [0, 1], Q32. Whole
/// where the lead is unbounded or the window none.
pub fn presence(lead: i64, window: i64) -> u64 {
    if lead >= INF || window <= 0 {
        return ONE32 as u64;
    }
    if lead <= 0 {
        return 0;
    }
    // A lead past its window is whole, and a lead under one (Q32)
    // divides in 64 bits: the same quotient, without a 128-bit division.
    if lead >= window {
        return ONE32 as u64;
    }
    if lead < ONE32 {
        return ((lead as u64) << 32) / window as u64;
    }
    ((i128::from(lead) << 32) / i128::from(window)).min(i128::from(ONE32)) as u64
}

/// A Q32 presence as a real: exact.
pub fn real(q: u64) -> f64 {
    q as f64 / ONE32 as f64
}

/// The log-odds of a 53-bit draw `d`, `ln(d / (2^53 - d))`, in Q32.
pub fn logit_draw(d: u64) -> i64 {
    ln_ratio(d, (1u64 << 53) - d)
}

/// The log-odds of a Q62 probability, in Q32.
pub fn logit(p: u64) -> i64 {
    ln_ratio(p, P_ONE - p.min(P_ONE))
}

/// D1's hash, for the GPU's mirror: the key one step further, `part`.
pub fn child(key: u64, part: u64) -> u64 {
    crate::lineage::Key(key).child(part).0
}

/// D1's draw of a key: its 53 bits.
pub fn draw(key: u64) -> u64 {
    crate::lineage::Key(key).draw().0
}

pub mod decide;
pub mod presence;

#[cfg(test)]
mod tests;
