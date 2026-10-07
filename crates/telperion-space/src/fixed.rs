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
//! - `log2` and `exp2_neg` are table-reduced Q62 polynomials, each
//!   specified to the bit (host decision, 2026-10-07: in place of 32
//!   squarings and 32 roots; `log2_frac`, `exp2_neg`). Every product is 64
//!   by 64 bits, its high part kept by a flooring shift.
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
pub(crate) const LN2: i128 = 0x2c5c_85fd_f473_de6a;
pub(crate) const LOG2E: i128 = 0x5c55_1d94_ae0b_f85d;

/// The high part of a Q62 product: `a b / 2^62`, truncated.
pub fn mul62(a: u64, b: u64) -> u64 {
    ((u128::from(a) * u128::from(b)) >> 62) as u64
}

/// A probability as Q62, floored: none for none or NaN, one from one.
pub fn q62(p: f64) -> u64 {
    if p.is_nan() || p <= 0.0 {
        return 0;
    }
    if p >= 1.0 {
        return P_ONE;
    }
    (p * P_ONE as f64).floor() as u64
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
    let e = 63 - n.leading_zeros();
    // The mantissa in Q62, in [1, 2); a 64-bit `n` loses its last bit.
    let m = if e <= 62 { n << (62 - e) } else { n >> 1 };
    (i64::from(e) << 32) | log2_frac(m) as i64
}

/// `-log2 r_j`, Q62, the nearest, of the reciprocals below
/// (`log-exp-constants.py`; host decision, 2026-10-07: log2 and exp2_neg
/// as table-reduced polynomials).
const LOGS: [i64; 64] = [
    0x00b7f285b778428b, 0x02239a3aa266825e,
    0x0389bf5721556bdf, 0x04ea8bf749fc70b6,
    0x0646285b918cd6ac, 0x079cbb045fe96c87,
    0x08ee68cbaa95bdaf, 0x0a3b54fcc1fba698,
    0x0b83a16a77813f96, 0x0cc76e83bf386f81,
    0x0e06db66eca6f720, 0x0f4205f3a7352665,
    0x10790adbb030096f, 0x11ac05b291f07053,
    0x12db10fc4d9aaf6f, 0x1406463b1b044976,
    0x152dbdfc4c96b37e, 0x16518fe4677ba6e5,
    0x1771d2ba7efb3be5, 0x188e9c72e0b225a5,
    0x19a802391e232f34, 0x1abe18797f1f48e1,
    0x1bd0f2e9e79030ab, 0x1ce0a4923a587cc9,
    0x1ded3fd442364c4e, 0x1ef6d67328e2207d,
    0x1ffd799a83ff9ab9, 0x210139e4fee07aa4,
    0x22022762a791d4f6, 0x2300519ee52ed49e,
    0x23fbc7a61d024c85, 0x24f4980b0b97f00c,
    0x25ead0ebd67e4c28, 0x26de7ff6db22c480,
    0x27cfb26f3ee02cd6, 0x28be7531440e714a,
    0x29aad4b6679d8895, 0x2a94dd194a871b64,
    0x2b7c9a196a286957, 0x2c62171eaa609081,
    0x2d455f3cb41e1411, 0x2e267d362ad8f7cb,
    0x2f057b7fbb4caef6, 0x2fe26443059e2ba3,
    0x30bd416164f64c22, 0x31961c7696777b66,
    0x326cfedb41565d3b, 0x3341f1a761c09fbd,
    0x3414fdb4982259cd, 0x34e62ba05e418f7a,
    0x35b583ce239065a3, 0x36830e69520117b4,
    0x374ed3673c92c81d, 0x3818da88f8bba6c5,
    0x38e12b5d23c38ec0, 0x39a7cd4195120d85,
    0x3a6cc764fe63aa53, 0x3b3020c87acc2986,
    0x3bf1e0410d5e5897, 0x3cb20c7910459cad,
    0x3d70abf19511f61a, 0x3e2dc503b6ec6076,
    0x3ee95de1df5f5baf, 0x3fa37c98fe55f286,
];

/// `r_j = round(2^62 / c_j)`, `c_j = 1 + (j + 1/2) / 64`.
const RECIP: [u64; 64] = [
    0x3f80fe03f80fe040, 0x3e88cb3c9484e2b0,
    0x3d980f6603d980f6, 0x3cae759203cae759,
    0x3bcbadc7f10d148e, 0x3aef6ca970586723,
    0x3a196b1edd80e866, 0x3949660abdc32204,
    0x387f1e0387f1e038, 0x37ba5713280dee96,
    0x36fad87bb4671656, 0x36406c80d901b203,
    0x358ae0358ae0358b, 0x34da034da034da03,
    0x342da7f2f4960343, 0x3385a29dc9420338,
    0x32e1c9f01970e4f8, 0x3241f693a1c451ab,
    0x31a6031a6031a603, 0x310dcbe157609329,
    0x30792ef5657dba52, 0x2fe80bfa02fe80c0,
    0x2f5a4411c1d986a9, 0x2ecfb9c869536203,
    0x2e4850fe8dbd780c, 0x2dc3eed6866f8d96,
    0x2d4279a2a6e52088, 0x2cc3d8d4a245f203,
    0x2c47f4ee02c47f4f, 0x2bceb771a02bceb7,
    0x2b580ad602b580ad, 0x2ae3da78a0d67344,
    0x2a721291e81fd58e, 0x2a02a02a02a02a03,
    0x2995710e4b5edcea, 0x292a73c765a0cdd4,
    0x28c1978feb9f3438, 0x285acc4baa3f0ddf,
    0x27f6027f6027f602, 0x27932b48f757ce88,
    0x2732385830fec66e, 0x26d31be7bc0e8f2a,
    0x2675c8b6af7963c2, 0x261a320261a32026,
    0x25c04b8097012e02, 0x2568095a02568096,
    0x2511602511602511, 0x24bc44e10125e227,
    0x2468acf13579be02, 0x24168e18cf81b10f,
    0x23c5de767f70e886, 0x237694808dda5202,
    0x2328a70119453809, 0x22dc0d1284e6f1d7,
    0x2290be1c159a76d2, 0x2246b1ceba6066d4,
    0x21fde021fde021fe, 0x21b641511e8d2b32,
    0x216fcdd84b3b8f27, 0x212a7e720212a7e7,
    0x20e64c148fef8cda, 0x20a32fefae680829,
    0x2061236a3ebc349e, 0x2020202020202020,
];

/// `log2(1 + t)`'s Taylor coefficients `(-1)^(k+1) / (k ln 2)`, k = 1..6, Q62.
const LOG_POLY: [i64; 6] = [
    0x5c551d94ae0bf85e, -0x2e2a8eca5705fc2f,
    0x1ec709dc3a03fd75, -0x171547652b82fe17,
    0x12776c50ef9bfe79, -0x0f6384ee1d01feba,
];

/// `2^(-j/64)`, Q62, the nearest.
const POWERS: [u64; 64] = [
    0x4000000000000000, 0x3f4f83033d21b05d,
    0x3ea0ecb6dc8a80cf, 0x3df437dcc2b44eee,
    0x3d495f454921b30b, 0x3ca05dcf167fec4e,
    0x3bf92e66f736bd73, 0x3b53cc07b6650ae1,
    0x3ab031b9f7490e4c, 0x3a0e5a940f12f71a,
    0x396e41b9df20d22a, 0x38cfe25cafa29694,
    0x383337bb0aa53844, 0x37983d209783a449,
    0x36feede5f6bc8dd5, 0x366745709e2bf5ed,
    0x35d13f32b5a75abd, 0x353cd6aaf3fb7e88,
    0x34aa07647c4ab917, 0x3418ccf6bbcac965,
    0x3389230547e12039, 0x32fb053fbc9c9f17,
    0x326e6f619b8bc9e9, 0x31e35d322aee6c57,
    0x3159ca845541b6b7, 0x30d1b3368925d902,
    0x304b1332999c2516, 0x2fc5e66d9e9cc421,
    0x2f4228e7d6030db0, 0x2ebfd6ac84cf917f,
    0x2e3eebd1d8bee7ba, 0x2dbf6478ca345de4,
    0x2d413cccfe779921, 0x2cc47104aa444922,
    0x2c48fd6074ab0964, 0x2bcede2b5a4290dd,
    0x2b560fba90a852b2, 0x2ade8e6d6a4fb4ce,
    0x2a6856ad3a9f03be, 0x29f364ed3a594d5a,
    0x297fb5aa6c544e3b, 0x290d456b8279a028,
    0x289c10c0c3125a06, 0x282c1443ee5c53f1,
    0x27bd4c982468446b, 0x274fb669cb3fedd4,
    0x26e34e6e7553954e, 0x26781164c82dfe99,
    0x260dfc14636e2a5c, 0x25a50b4dc8061680,
    0x253d3bea3fbdc258, 0x24d68acbc4f9ba4b,
    0x2470f4dceac470ce, 0x240c7710c519ac77,
    0x23a90e62d17354f0, 0x2346b7d6df96ea79,
    0x22e57078faa2f5ba, 0x2285355d525bbf67,
    0x222603a024b6a14a, 0x21c7d865a7a34404,
    0x216ab0d9f3121ec5, 0x210e8a30eb379012,
    0x20b361a62b0ae876, 0x2059347cef00c1dd,
];

/// `e^-y`'s Taylor coefficients `(-1)^k / k!`, k = 0..6, Q62.
const EXP_POLY: [i64; 7] = [
    0x4000000000000000, -0x4000000000000000,
    0x2000000000000000, -0x0aaaaaaaaaaaaaab,
    0x02aaaaaaaaaaaaab, -0x0088888888888889,
    0x0016c16c16c16c17,
];

/// `a b / 2^62`, floored, signed.
fn mul62s(a: i64, b: i64) -> i64 {
    ((i128::from(a) * i128::from(b)) >> 62) as i64
}

/// A polynomial in `t` (Q62) by Horner's rule from its highest
/// coefficient down, one floor a product.
fn horner(c: &[i64], t: i64) -> i64 {
    c.iter().rev().fold(0, |p, &k| k + mul62s(t, p))
}

/// The fraction of `log2 m`, Q32, of a Q62 mantissa `m` in [1, 2) (host
/// decision, 2026-10-07, FINE-CONTRACT.md and CONTRACT-A.md 2.3):
/// 1. `j` the mantissa's top six fraction bits; `t = floor(m r_j / 2^62) -
///    2^62`, the rest `m / c_j - 1` in Q62, below 2^-6.9 in magnitude.
/// 2. `log2 m = L_j + t P(t)`, `P` Horner over `LOG_POLY` from its sixth
///    coefficient down, `t P(t)` floored.
/// 3. Floored to Q32 (`>> 30`) and clamped to [0, 2^32): exact at one.
pub(crate) fn log2_frac(m: u64) -> u64 {
    let j = ((m >> 56) & 63) as usize;
    let t = ((u128::from(m) * u128::from(RECIP[j])) >> 62) as i64 - (1 << 62);
    let log = LOGS[j] + mul62s(t, horner(&LOG_POLY, t));
    (log >> 30).clamp(0, (1 << 32) - 1) as u64
}

/// `2^-x` for `x` in Q32, as a Q62 probability: one for none or less (host
/// decision, 2026-10-07):
/// 1. The whole part `k` shifts at the end (none at 63 or more); of the
///    fraction `f`, `j` its top six bits and `g` the rest, below 2^-6.
/// 2. `y = floor(g LN2 / 2^32)` (Q62), `q = E(y)` Horner over `EXP_POLY`
///    from its seventh coefficient down: `e^-y`, that is `2^-g`.
/// 3. `floor(POWERS[j] q / 2^62) >> k`.
pub fn exp2_neg(x: i64) -> u64 {
    if x <= 0 {
        return P_ONE;
    }
    let whole = x >> 32;
    if whole >= 63 {
        return 0;
    }
    let f = x as u64 & 0xffff_ffff;
    let (j, g) = ((f >> 26) as usize, f & ((1 << 26) - 1));
    let y = ((u128::from(g) * LN2 as u128) >> 32) as i64;
    let q = horner(&EXP_POLY, y);
    (((u128::from(POWERS[j]) * q as u128) >> 62) as u64) >> whole
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
        _ => ((i128::from(log2(a) - log2(b)) * LN2) >> 62) as i64,
    }
}

/// `e^x` for `x` in Q32 (natural), in Q32.
pub fn exp(x: i64) -> i64 {
    if x <= -INF {
        return 0;
    }
    exp2(((i128::from(x) * LOG2E) >> 62) as i64)
}

/// `a b` of two Q32 values, in Q32, floored and saturated.
pub fn mul(a: i64, b: i64) -> i64 {
    ((i128::from(a) * i128::from(b)) >> 32).clamp(-i128::from(INF), i128::from(INF)) as i64
}

/// A probability raised to `share` ticks of a cycle: `-log2 p` scaled by
/// the share, `l s / 2^20`, floored and saturated.
pub fn scaled(l: i64, share: u64) -> i64 {
    if l >= INF {
        return INF;
    }
    ((i128::from(l) * i128::from(share)) >> 20).min(i128::from(INF)) as i64
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
pub mod feedback;
pub mod presence;
pub mod sum;

#[cfg(test)]
mod tests;
