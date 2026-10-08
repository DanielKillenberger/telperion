//! Logs and powers of W values (CONTRACT-A.md 2.3; Astra's review, point
//! 2): `fixed`'s squarings and roots, with W's range at both ends. A W
//! power is the Q62 mantissa of its fraction shifted once by its whole
//! part, so a small one keeps W's two extra fraction bits and nothing
//! underflows before W's last place.
use super::Fault;
use crate::fixed::{exp2_neg, log2_frac, INF, ONE32, P_ONE};

/// `log2` of the W value `n / 2^64`, Q32: `-INF` for none. The mantissa
/// is `n`'s top 63 bits (truncated) as Q62, then `fixed::log2`'s
/// squarings.
pub fn log2_w(n: u128) -> i64 {
    if n == 0 {
        return -INF;
    }
    let e = 127 - n.leading_zeros();
    let m = if e >= 62 {
        n >> (e - 62)
    } else {
        n << (62 - e)
    } as u64;
    ((i64::from(e) - 64) << 32) + log2_frac(m) as i64
}

/// `2^y` of a Q32 `y`, in W, floored: `2^(k + f)` taken as the mantissa
/// `2^(f - 1)` (`exp2_neg`, Q62) shifted by `k + 3`, or `2^62` by `k + 2`
/// where the fraction is none. `Range` where it reaches 2^126.
pub fn exp2_w(y: i64) -> Result<u128, Fault> {
    let (whole, frac) = (y >> 32, y & (ONE32 - 1));
    let (m, s) = if frac == 0 {
        (P_ONE, whole + 2)
    } else {
        (exp2_neg(ONE32 - frac), whole + 3)
    };
    match s {
        64.. => Err(Fault::Range),
        0..=63 => Ok(u128::from(m) << s),
        -63..=-1 => Ok(u128::from(m) >> -s),
        _ => Ok(0),
    }
}
