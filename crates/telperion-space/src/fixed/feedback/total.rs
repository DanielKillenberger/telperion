//! Exact sums with a refusal independent of order (CONTRACT-A.md 2.3;
//! Astra's review, point 3): a wrapped sum alone cannot show its true
//! value (eight terms of 2^125 wrap to none), so each W sum also carries
//! the sum of its terms' magnitudes, saturating. Both are the same in any
//! grouping: wrapping addition is exact modulo 2^128, and a saturating sum
//! of non-negative terms is the lesser of its true value and the bound.
//! Where the magnitudes stay below 2^127, the wrapped value is the true
//! one; otherwise the sum refuses, whatever the cut, order or chunking.
use super::Fault;

/// The magnitudes' bound: at or past it the sum refuses.
pub const BOUND: u128 = 1 << 127;

/// A W sum and its terms' magnitudes.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Total {
    value: i128,
    magnitude: u128,
}

impl Total {
    pub fn add(&mut self, term: i128) {
        self.merge(Self {
            value: term,
            magnitude: term.unsigned_abs(),
        });
    }

    /// Adds another partial.
    pub fn merge(&mut self, other: Self) {
        self.value = self.value.wrapping_add(other.value);
        self.magnitude = self.magnitude.saturating_add(other.magnitude);
    }

    /// The exact sum, or `Range` where its terms' magnitudes reach the
    /// bound.
    pub fn get(self) -> Result<i128, Fault> {
        if self.magnitude >= BOUND {
            return Err(Fault::Range);
        }
        Ok(self.value)
    }
}

impl std::iter::Sum<i128> for Total {
    fn sum<I: Iterator<Item = i128>>(terms: I) -> Self {
        let mut t = Self::default();
        terms.for_each(|x| t.add(x));
        t
    }
}
