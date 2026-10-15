//! The decision contract's arithmetic (fn-254, after fn-209 F1): what a
//! bud's decisions read, in ordinary f32 on the CPU and on the device
//! (`contract.wgsl`), by the same operations. Identity stays integer:
//!
//! - **Time** is a count of ticks of 2^-20 cycle. The CPU holds it in an
//!   f64, where every count below 2^53 and every sum and difference of two
//!   is exact; `grid` puts a lifespan or a waking time on the grid where it
//!   enters. A part of a cycle finer than a tick is a count of 2^-32 of a
//!   cycle. Neither is ever accumulated in an f32.
//! - **A draw** is 53 bits of a lineage hash (`lineage::Key::draw`). It
//!   falls under a probability `p` where its uniform `d / 2^53`, exact,
//!   lies below `p`: the integer test `d < threshold(p)`, `threshold(p)`
//!   being `ceil(p · 2^53)`, exact for every f32 and f64. A probability of
//!   none or NaN passes no draw and one of one or more every draw.
//! - **A probability, a log, a lead, a window and a presence** are f32. A
//!   log of none is `-INF` and of a certainty's complement `INF`, a finite
//!   sentinel tested by name at every site, as the device must (WGSL may
//!   assume no infinities); a lead is a difference of log-odds; a presence
//!   is in [0, 1].
//!
//! - **Logarithms and powers** are the contract's own (`log2`, `exp2`):
//!   plain f32 arithmetic, the same operations on both executors, never a
//!   platform's or a GPU's own, whose accuracy near one is an absolute
//!   2^-21 and differs from vendor to vendor.
//!
//! Values computed once per species point (the closed form's tables, a
//! species' settings) are inputs: the CPU computes them in f64 and hands
//! both executors the same values, rounded once to f32 where they enter.

pub mod decide;
pub mod presence;
pub mod shed;

/// Ticks a cycle: 2^20.
pub const CYCLE: f64 = (1u64 << 20) as f64;

/// A log beyond any finite one: a probability of none.
pub const INF: f32 = 1e30;

/// 2^53: a draw's threshold where its probability is certain.
pub const ONE: u64 = 1 << 53;

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

/// A probability's threshold against a 53-bit draw: `ceil(p · 2^53)`,
/// none for none or NaN, `ONE` for one or more. The scaling by 2^53 is
/// exact, so the integer test is the real one to the bit.
pub fn threshold(p: f32) -> u64 {
    crate::lineage::threshold(f64::from(p))
}

/// `log2 x` of a positive finite `x`: its exponent and, of its mantissa
/// `m` brought within [1/sqrt 2, sqrt 2), `ln m` by the series of `2
/// atanh s`, `s = (m - 1) / (m + 1)`, to its ninth power, so a value near
/// one keeps its relative precision.
pub fn log2(x: f32) -> f32 {
    let (x, below) = if x < f32::MIN_POSITIVE {
        (x * 16_777_216.0, 24)
    } else {
        (x, 0)
    };
    let bits = x.to_bits();
    let mut e = ((bits >> 23) & 0xff) as i32 - 127 - below;
    let mut m = f32::from_bits((bits & 0x7f_ffff) | 0x3f80_0000);
    if m > std::f32::consts::SQRT_2 {
        m *= 0.5;
        e += 1;
    }
    let s = (m - 1.0) / (m + 1.0);
    let z = s * s;
    let ln = s * (2.0 + z * (2.0 / 3.0 + z * (0.4 + z * (2.0 / 7.0 + z * (2.0 / 9.0)))));
    e as f32 + ln * std::f32::consts::LOG2_E
}

/// `2^x`: none below 2^-126, the largest below 2^128 past it; `x` split
/// into its nearest whole `n` and the rest `f` in [-1/2, 1/2], `2^f` by
/// its Taylor series to the seventh power.
// The series' coefficients are written out as the shaders write them.
#[allow(clippy::approx_constant)]
pub fn exp2(x: f32) -> f32 {
    if x < -126.0 {
        return 0.0;
    }
    let n = (x.min(127.0) + 0.5).floor();
    let f = x.min(127.0) - n;
    let p = 1.0
        + f * (0.693_147_2
            + f * (0.240_226_5
                + f * (0.055_504_11
                    + f * (0.009_618_129
                        + f * (0.001_333_355_8 + f * (0.000_154_035_3 + f * 0.000_015_252_734))))));
    p * f32::from_bits(((n as i32 + 127) as u32) << 23)
}

/// `ln x`, by `log2`.
pub fn ln(x: f32) -> f32 {
    log2(x) * std::f32::consts::LN_2
}

/// `e^x`, by `exp2`.
pub fn exp(x: f32) -> f32 {
    exp2(x * std::f32::consts::LOG2_E)
}

/// A log as the executors hold it: an unbounded one as the sentinel.
pub fn log32(x: f64) -> f32 {
    if x <= -f64::from(INF) {
        -INF
    } else if x >= f64::from(INF) {
        INF
    } else {
        x as f32
    }
}

/// `-log2 p`: `INF` for none or NaN, none for one or more.
pub fn neg_log2(p: f32) -> f32 {
    if !(p > 0.0) {
        return INF;
    }
    if p >= 1.0 {
        return 0.0;
    }
    -log2(p)
}

/// `-log2 p` of a chance the CPU holds in f64 (a species' setting, or one
/// it computes from them): its log taken before it is rounded to f32, so a
/// chance near one keeps its distance from one, which an f32 of the chance
/// itself would round away (fn-254: an apex's shaded survival stepped by
/// percents as light moved by parts in a billion).
pub fn neg_log2_of(p: f64) -> f32 {
    if !(p > 0.0) {
        return INF;
    }
    if p >= 1.0 {
        return 0.0;
    }
    use crate::transcendental::Transcendental;
    (-p.log2_fixed()) as f32
}

/// The log-odds of a chance the CPU holds in f64, taken before it is
/// rounded (as `neg_log2_of`): `-INF` for none, `INF` for one.
pub fn logit_of(p: f64) -> f32 {
    if !(p > 0.0) {
        return -INF;
    }
    if p >= 1.0 {
        return INF;
    }
    use crate::transcendental::Transcendental;
    (p / (1.0 - p)).ln_fixed() as f32
}

/// `ln p` of a chance the CPU holds in f64: `-INF` for none.
pub fn ln_of(p: f64) -> f32 {
    if !(p > 0.0) {
        return -INF;
    }
    use crate::transcendental::Transcendental;
    p.ln_fixed() as f32
}

/// The probability `2^-l` of a `-log2`: one for none or less, none for
/// `INF`.
pub fn prob(l: f32) -> f32 {
    if l <= 0.0 {
        return 1.0;
    }
    if l >= INF {
        return 0.0;
    }
    exp2(-l)
}

/// `1 - 2^-l`, the complement of `prob`: none for none or less, one for
/// `INF`. Under 1/8 it is the series of `1 - e^-y`, `y = l ln 2`, to its
/// fifth power, so a chance near none keeps every bit of the f32 that
/// holds it, as `1 - prob(l)` would not; past it, that difference.
pub fn fails(l: f32) -> f32 {
    if l <= 0.0 {
        return 0.0;
    }
    if l >= INF {
        return 1.0;
    }
    if l < 0.125 {
        let y = l * std::f32::consts::LN_2;
        return y * (1.0 - y * (0.5 - y * (1.0 / 6.0 - y * (1.0 / 24.0 - y * (1.0 / 120.0)))));
    }
    1.0 - exp2(-l)
}

/// `-ln(1 - x)` for `x` in [0, 1): under 1/8 its series to the ninth
/// power, so a small `x` keeps every bit (fn-254 review: `1 - x` would
/// round it away); past it, the log of the difference.
pub fn neg_ln1m(x: f32) -> f32 {
    if !(x > 0.0) {
        return 0.0;
    }
    if x >= 1.0 {
        return INF;
    }
    if x < 0.125 {
        let mut sum = 0.0f32;
        for k in (1..=9).rev() {
            sum = sum * x + 1.0 / k as f32;
        }
        return sum * x;
    }
    -ln(1.0 - x)
}

/// `floor(p · 2^53)` for an f32 `p` in [0, 1]: exact.
pub fn floor53(p: f32) -> u64 {
    (f64::from(p) * ONE as f64) as u64
}

/// A `-log2` chance a cycle over `share`, in 2^-32 of a cycle: `l` scaled
/// by it, `INF` staying unbounded whatever the share.
pub fn scaled(l: f32, share: u64) -> f32 {
    if l >= INF {
        return INF;
    }
    l * (share as f32 * FINE)
}

/// 2^-32: a fine share's unit, a cycle's part.
const FINE: f32 = 1.0 / 4_294_967_296.0;

/// A presence: `lead / window`, in [0, 1]. Whole where the lead is
/// unbounded or the window none; none where the lead is none or less.
pub fn presence(lead: f32, window: f32) -> f32 {
    if lead >= INF || window <= 0.0 {
        return 1.0;
    }
    if lead <= 0.0 {
        return 0.0;
    }
    (lead / window).min(1.0)
}

/// The log-odds of a probability, `ln(p / (1 - p))`: `-INF` for none,
/// `INF` for one.
pub fn logit(p: f32) -> f32 {
    if !(p > 0.0) {
        return -INF;
    }
    if p >= 1.0 {
        return INF;
    }
    ln(p / (1.0 - p))
}

/// The log-odds of a 53-bit integer `d` against 2^53, `ln(d / (2^53 -
/// d))`, its count and its complement's each rounded once: a draw's, or a
/// threshold's as its probability's.
pub fn logit53(d: u64) -> f32 {
    if d == 0 {
        return -INF;
    }
    if d >= ONE {
        return INF;
    }
    ln(d as f32 / (ONE - d) as f32)
}

/// D1's hash, for the GPU's mirror: the key one step further, `part`.
pub fn child(key: u64, part: u64) -> u64 {
    crate::lineage::Key(key).child(part).0
}

/// D1's draw of a key: its 53 bits.
pub fn draw(key: u64) -> u64 {
    crate::lineage::Key(key).draw().0
}

/// The 53 bits of the internode draw of the node keyed `key` (fn-222),
/// which `lineage::internode` scales its length by.
pub fn internode_draw(key: u64) -> u64 {
    use crate::lineage::{Key, INTERNODE};
    Key(key).child(INTERNODE).draw().0
}

/// The 53 bits of the insertion draw of the axis whose lineage is `key`
/// (fn-224), which `lineage::insertion` offsets its angle by.
pub fn insertion_draw(key: u64) -> u64 {
    use crate::lineage::{Key, INSERTION};
    Key(key).child(INSERTION).draw().0
}

/// The 53 bits of the tropism draw of the axis whose lineage is `key`
/// (fn-224), which `lineage::tropism` scales its tropism by.
pub fn tropism_draw(key: u64) -> u64 {
    use crate::lineage::{Key, TROPISM};
    Key(key).child(TROPISM).draw().0
}

#[cfg(test)]
mod tests;
