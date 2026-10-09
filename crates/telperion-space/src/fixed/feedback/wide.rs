//! Exact intermediates (CONTRACT-A.md 2.3; Astra's review, point 1): a
//! product of four Q32 factors, a Q32 position times a W mass and a W
//! numerator shifted for a quotient each need more than 128 bits, so each
//! is taken exactly in 256 and rounded once, then narrowed with a check.
//! Every function here is specified by its result; the algorithm is free.
use super::Fault;

#[cfg(test)]
mod tests;

/// A two's complement 256-bit integer, `hi 2^128 + lo`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct I256 {
    pub hi: i128,
    pub lo: u128,
}

impl I256 {
    pub fn of(x: i128) -> Self {
        Self {
            hi: x >> 127,
            lo: x as u128,
        }
    }

    pub fn negative(self) -> bool {
        self.hi < 0
    }

    fn neg(self) -> Self {
        let lo = (!self.lo).wrapping_add(1);
        Self {
            hi: (!self.hi).wrapping_add(i128::from(lo == 0)),
            lo,
        }
    }

    /// The magnitude, as unsigned halves.
    fn magnitude(self) -> (u128, u128) {
        let m = if self.negative() { self.neg() } else { self };
        (m.hi as u128, m.lo)
    }

    fn signed(sign: bool, (hi, lo): (u128, u128)) -> Self {
        let m = Self { hi: hi as i128, lo };
        if sign {
            m.neg()
        } else {
            m
        }
    }

    /// `a + b`, wrapping in 256 bits (exact for sums of a few products).
    pub fn add(self, b: Self) -> Self {
        let (lo, carry) = self.lo.overflowing_add(b.lo);
        Self {
            hi: self.hi.wrapping_add(b.hi).wrapping_add(i128::from(carry)),
            lo,
        }
    }

    /// `a b`, exact: its magnitude is at most 2^254.
    pub fn mul(a: i128, b: i128) -> Self {
        let (x, y) = (a.unsigned_abs(), b.unsigned_abs());
        let (x0, x1, y0, y1) = (x as u64 as u128, x >> 64, y as u64 as u128, y >> 64);
        let (p00, p01, p10, p11) = (x0 * y0, x0 * y1, x1 * y0, x1 * y1);
        let (mid, c1) = p01.overflowing_add(p10);
        let (lo, c2) = p00.overflowing_add(mid << 64);
        let hi = p11 + (mid >> 64) + (u128::from(c1) << 64) + u128::from(c2);
        Self::signed((a < 0) != (b < 0), (hi, lo))
    }

    /// `floor(self / 2^n)`, `n` below 256.
    pub fn floor_shift(self, n: u32) -> Self {
        match n {
            0 => self,
            1..=127 => Self {
                hi: self.hi >> n,
                lo: (self.lo >> n) | ((self.hi as u128) << (128 - n)),
            },
            _ => Self {
                hi: self.hi >> 127,
                lo: (self.hi >> (n - 128)) as u128,
            },
        }
    }

    /// The value as an i128, or `Range`.
    pub fn narrow(self) -> Result<i128, Fault> {
        let lo = self.lo as i128;
        if self.hi == lo >> 127 {
            Ok(lo)
        } else {
            Err(Fault::Range)
        }
    }

    /// The value as an i64, or `Range`.
    pub fn narrow64(self) -> Result<i64, Fault> {
        i64::try_from(self.narrow()?).map_err(|_| Fault::Range)
    }

    /// `self / d`, truncated toward zero (both executors' integer
    /// division); `Zero` where `d` is none.
    pub fn quotient(self, d: i128) -> Result<Self, Fault> {
        if d == 0 {
            return Err(Fault::Zero);
        }
        let (hi, lo) = self.magnitude();
        let q = divide(limbs([lo, hi]), limbs([d.unsigned_abs(), 0]));
        let half = |h: usize| (0..4).fold(0u128, |w, k| w | u128::from(q[4 * h + k]) << (32 * k));
        Ok(Self::signed(self.negative() != (d < 0), (half(1), half(0))))
    }
}

/// A 256-bit magnitude, low half first, as eight 32-bit limbs, the least
/// significant first.
fn limbs(x: [u128; 2]) -> [u32; 8] {
    std::array::from_fn(|k| (x[k / 4] >> (32 * (k % 4))) as u32)
}

/// `u / v` of limbs, `v` not zero, truncated: Knuth's algorithm D (TAOCP
/// 4.3.1, as Hacker's Delight's `divmnu`). Each quotient limb is estimated
/// from the two leading limbs of the remainder over the normalised
/// divisor's leading limb, corrected against its next limb, and added back
/// once in the rare case it is still one too large. Every product and
/// division is in 64 bits, which Wasm has natively.
fn divide(u: [u32; 8], v: [u32; 8]) -> [u32; 8] {
    const B: u64 = 1 << 32;
    let len = |x: &[u32; 8]| 8 - x.iter().rev().take_while(|&&l| l == 0).count();
    let (m, n) = (len(&u), len(&v));
    let mut q = [0u32; 8];
    if m < n {
        return q;
    }
    if n == 1 {
        let (d, mut r) = (u64::from(v[0]), 0u64);
        for j in (0..m).rev() {
            let t = (r << 32) | u64::from(u[j]);
            (q[j], r) = ((t / d) as u32, t % d);
        }
        return q;
    }
    // Normalised: the divisor's leading limb has its top bit set.
    let s = v[n - 1].leading_zeros();
    let shifted = |x: &[u32], k: usize| {
        let below = if k == 0 {
            0
        } else {
            u64::from(x[k - 1]) >> (32 - s)
        };
        ((u64::from(x.get(k).copied().unwrap_or(0)) << s) | below) as u32
    };
    let vn: [u32; 8] = std::array::from_fn(|k| shifted(&v, k));
    let mut un: [u32; 9] = std::array::from_fn(|k| shifted(&u, k));
    let (top, next) = (u64::from(vn[n - 1]), u64::from(vn[n - 2]));
    for j in (0..=m - n).rev() {
        let lead = (u64::from(un[j + n]) << 32) | u64::from(un[j + n - 1]);
        let (mut qhat, mut rhat) = (lead / top, lead % top);
        while qhat >= B || qhat * next > (rhat << 32) | u64::from(un[j + n - 2]) {
            qhat -= 1;
            rhat += top;
            if rhat >= B {
                break;
            }
        }
        // Multiply and subtract.
        let mut k = 0i64;
        for i in 0..n {
            let p = qhat * u64::from(vn[i]);
            let t = i64::from(un[i + j]) - k - (p & 0xffff_ffff) as i64;
            un[i + j] = t as u32;
            k = (p >> 32) as i64 - (t >> 32);
        }
        let t = i64::from(un[j + n]) - k;
        un[j + n] = t as u32;
        q[j] = qhat as u32;
        if t < 0 {
            // One too large: add the divisor back.
            q[j] = q[j].wrapping_sub(1);
            let mut c = 0u64;
            for i in 0..n {
                let t = u64::from(un[i + j]) + u64::from(vn[i]) + c;
                un[i + j] = t as u32;
                c = t >> 32;
            }
            un[j + n] = un[j + n].wrapping_add(c as u32);
        }
    }
    q
}

/// `floor(a b / 2^n)`, exact, as an i128: a W product (`n` 64), a Q32 by
/// W one (`n` 32).
pub fn mul_shr(a: i128, b: i128, n: u32) -> Result<i128, Fault> {
    I256::mul(a, b).floor_shift(n).narrow()
}

/// `floor(f0 f1 f2 f3 / 2^n)`, exact, as an i64: a product of Q32 factors
/// rounded once (a missing factor is the integer 1).
pub fn prod_shr(f: [i64; 4], n: u32) -> Result<i64, Fault> {
    let pair = |a: i64, b: i64| i128::from(a) * i128::from(b);
    I256::mul(pair(f[0], f[1]), pair(f[2], f[3]))
        .floor_shift(n)
        .narrow64()
}

/// `a 2^n / b`, truncated toward zero, as an i128: a W over W quotient in
/// Q32 (`n` 32). `n` is at most 126.
pub fn ratio(a: i128, b: i128, n: u32) -> Result<i128, Fault> {
    debug_assert!(n <= 126, "a quotient's shift");
    I256::mul(a, 1 << n).quotient(b)?.narrow()
}

/// `floor(x / 2^n)`: an arithmetic shift, so a tiny negative is -1.
pub fn floor_shift(x: i128, n: u32) -> i128 {
    x >> n.min(127)
}

/// `x / 2^n` truncated toward zero: the magnitude shifted, its sign kept,
/// so a shift is symmetric about zero (a lever's small components).
pub fn zero_shift(x: i128, n: u32) -> i128 {
    if n == 0 {
        return x;
    }
    let m = (x.unsigned_abs() >> n.min(127)) as i128;
    if x < 0 {
        -m
    } else {
        m
    }
}
