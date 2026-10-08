//! Order-free sums (fn-211.4, host decision on probe (a), 2026-10-06;
//! change 2's first part): the three reductions over the whole tree, each
//! PA's Σp and Σp(v/p)^ψ (decision 23), the light lattice's deposits and
//! the leaf-light girth's Σpipe and Σpipe·light^χ, accumulate in fixed
//! point, so any grouping of their terms (a cut, an item order, a
//! chunking, a GPU's) gives the same bits. The GPU mirrors this file.
//!
//! - **Format:** Q64.64 in a two's complement 128-bit integer: a term `x`
//!   is the integer nearest `x · 2^64`, ties to even. Scaling by 2^64 is
//!   exact in binary64, so the only rounding is that one.
//! - **Range:** a term of magnitude 2^63 or more saturates to ±(2^127 − 1)
//!   and an accumulation saturates at those bounds (never met: the
//!   largest of these terms is a lattice node's density, below 2^20).
//! - **Zero and the unrepresentable:** a term below 2^-65 in magnitude is
//!   0; a non-finite term is 0 (no input is non-finite; zero light gives a
//!   light term of exactly 0).
//! - **Back to binary64:** the integer converted to the nearest binary64,
//!   ties to even, then scaled by 2^-64 (exact).
//! - **Partials** add as integers, exactly and in any order.

/// 2^64, the format's one.
const ONE: f64 = 18_446_744_073_709_551_616.0;

/// An order-free sum.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct Sum(i128);

impl Sum {
    /// A term in the format.
    pub fn of(x: f64) -> Self {
        let q = (x * ONE).round_ties_even();
        if !q.is_finite() {
            return Self(0);
        }
        // `as` saturates at the bounds and is exact below them.
        Self(q as i128)
    }

    pub fn add(&mut self, x: f64) {
        self.merge(Self::of(x));
    }

    /// Adds another partial.
    pub fn merge(&mut self, other: Self) {
        self.0 = self.0.saturating_add(other.0);
    }

    pub fn value(self) -> f64 {
        self.0 as f64 / ONE
    }
}

impl std::iter::Sum<f64> for Sum {
    fn sum<I: Iterator<Item = f64>>(terms: I) -> Self {
        let mut s = Self::default();
        terms.for_each(|x| s.add(x));
        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn any_grouping_gives_the_same_bits() {
        let terms: Vec<f64> = (1..2000)
            .map(|k| 1.0 / f64::from(k) + 1e-9 * f64::from(k % 7))
            .collect();
        let forward: Sum = terms.iter().copied().sum();
        let backward: Sum = terms.iter().rev().copied().sum();
        let mut chunked = Sum::default();
        for c in terms.chunks(37) {
            chunked.merge(c.iter().copied().sum());
        }
        assert_eq!(forward, backward);
        assert_eq!(forward, chunked);
        let float: f64 = terms.iter().sum();
        assert!((forward.value() - float).abs() < 1e-12);
    }

    #[test]
    fn its_bounds() {
        assert_eq!(Sum::of(0.0).value(), 0.0);
        assert_eq!(Sum::of(f64::NAN), Sum(0));
        assert_eq!(Sum::of(2f64.powi(-66)), Sum(0));
        assert_eq!(Sum::of(1.5).value(), 1.5);
        assert_eq!(Sum::of(-0.25).value(), -0.25);
        assert_eq!(Sum::of(1e40), Sum(i128::MAX));
        let mut s = Sum(i128::MAX);
        s.add(1.0);
        assert_eq!(s, Sum(i128::MAX));
    }
}
