//! The feedback contract's arithmetic (fn-211 task 8, CONTRACT-A.md, as
//! revised after Astra's review): the operations option A's feedback loop
//! runs on both executors, each with one result. The GPU mirrors this
//! module word for word (`telperion-render`'s `fine/feedback*.wgsl`), and
//! its conformance probe holds every result and error status to the CPU's.
//! No tree reads it yet: the contract's first step is this probe and the
//! feedback slice (`lattice`, `shed`), before the reference adopts it.
//!
//! - **Formats** (CONTRACT-A.md 2.2): Q32 in an i64 (`n / 2^32`), W in an
//!   i128 (`n / 2^64`), an orientation T in a u32 (a turn is 2^32), a
//!   signed angular amount in an i64 of Q32 turns.
//! - **Every result is the exact one, rounded once** by the rule its
//!   function names (floor, toward zero, or nearest), from intermediates
//!   wide enough to hold it (`wide`): nothing is checked after it wrapped.
//! - **A value outside its format** is a `Fault`, never a wrapped or
//!   saturated value; the site that meets one refuses the tree by name.
pub mod angle;
pub mod edge;
pub mod lattice;
pub mod power;
pub mod shed;
pub mod slice;
pub mod total;
pub mod vector;
pub mod wide;

#[cfg(test)]
mod tests;

/// Why an operation has no result in its format.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Fault {
    /// The exact result lies outside the format it is narrowed to.
    Range,
    /// A quotient by none.
    Zero,
}

impl Fault {
    /// The status word both executors report: 1 out of range, 2 by none.
    pub fn code(self) -> u64 {
        match self {
            Self::Range => 1,
            Self::Zero => 2,
        }
    }
}

/// The status word of a result: 0 where it has one.
pub fn status<T>(r: &Result<T, Fault>) -> u64 {
    r.as_ref().err().map_or(0, |f| f.code())
}
