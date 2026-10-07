//! The contract's zero and endpoint cases (CONTRACT-A.md 2.5 and 2.7;
//! Astra's review, point 1), each as today's f64 code takes it.
use super::angle::{atan2, sincos};
use super::power::log2_w;
use crate::fixed::{exp2, exp2_neg, log2, mul, INF, LN2, ONE32};

/// `min(years / ripening, 1)`, Q32, of whole years (Q32): one where the
/// ripening is none or less (`girth::ripe`).
pub fn ripe(years: i64, ripening: i64) -> i64 {
    if ripening <= 0 {
        return ONE32;
    }
    ((i128::from(years) << 32) / i128::from(ripening)).min(i128::from(ONE32)) as i64
}

/// D5 under shade: a unit's `-log2` survival `l` (Q32) at `light` (Q32)
/// with shade response `phi` (Q32), `l light^-phi`, saturated at `INF`.
/// Dormant where `phi` is none or the light whole; a certain survival
/// (`l` none) stays certain at any light, none included
/// (`grow/advance.rs`); an unbounded factor (no light) makes any other
/// survival none.
pub fn shade(l: i64, light: i64, phi: i64) -> i64 {
    if phi <= 0 || light >= ONE32 || l <= 0 || l >= INF {
        return l;
    }
    if light <= 0 {
        return INF;
    }
    // -phi log2 light, the light's log being its integer's less 32.
    let factor = exp2(mul(phi, (32 << 32) - log2(light as u64)));
    if factor >= INF {
        return INF;
    }
    mul(l, factor).min(INF)
}

/// Sag's `settle`, `2 atan(tan(fall / 2) e^-k)` for `fall` in [0, 1/2]
/// turn and `x = k log2 e` (Q32), taken without a tangent:
/// `2 atan2(sin(fall / 2) e^-k, cos(fall / 2))`. An upright lever (`fall`
/// a half turn) stays a half turn; `atan2(0, 0)` is none.
pub fn settle(fall: u32, x: i64) -> u32 {
    let (c, s) = sincos(fall >> 1);
    let y = ((i128::from(s) * i128::from(exp2_neg(x))) >> 62) as i64;
    atan2(y, c) << 1
}

/// `ln(tan(fall / 2) / tan(high / 2))`, Q32, for angles in [0, 1/2]
/// turn: the log ratio of the exact products `sin_f cos_h` and
/// `cos_f sin_h`, each factor at least none; `INF` where the second is none (an upright `fall`),
/// `-INF` where the first is.
pub fn tangent_ratio(fall: u32, high: u32) -> i64 {
    let (cf, sf) = sincos(fall >> 1);
    let (ch, sh) = sincos(high >> 1);
    let product = |a: i64, b: i64| (a.max(0) as u128) * (b.max(0) as u128);
    let (a, b) = (product(sf, ch), product(cf, sh));
    match (a, b) {
        (0, _) => -INF,
        (_, 0) => INF,
        _ => ((i128::from(log2_w(a) - log2_w(b)) * LN2) >> 62) as i64,
    }
}
