//! D16 in integers (CONTRACT-A.md 2.7; Astra's review, point 6): a
//! lateral's remembered balance, its hazard's probability in Q62, the
//! integer threshold its draw is held to, and the Q32 kept share its lead
//! past that threshold leaves it, as every other presence takes one
//! (FINE-CONTRACT.md, host decision 8). A reference change: today's
//! balance decides on an f64 probability and leads from it.
use super::Fault;
use crate::fixed::presence::{of, over, times};
use crate::fixed::{exp2_neg, mul, threshold, LOG2E, ONE32, P_ONE};

/// One lateral's balance inputs, Q32 unless named: its collected light
/// `q` and cost `c`, its memory, its PA's hazard and tolerance, its 53-bit
/// draw, its window (`Windows::window_q` at the cycle, stake one) and its
/// bearer's kept share.
#[derive(Debug, Clone, Copy, Default)]
pub struct Bud {
    pub q: i64,
    pub c: i64,
    pub memory: i64,
    pub hazard: i64,
    pub tolerance: i64,
    pub draw: u64,
    pub window: i64,
    pub keep: u64,
}

/// What the balance makes of it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Balance {
    pub memory: i64,
    /// The draw's threshold: shed where the draw lies below it.
    pub threshold: u64,
    pub shed: bool,
    /// The kept share, Q32: the bearer's where shed or undecided.
    pub kept: u64,
}

/// `(q - c) / (q + c)`, Q32 toward zero; one where `q + c` is none or less.
fn now(q: i64, c: i64) -> i64 {
    let total = i128::from(q) + i128::from(c);
    if total <= 0 {
        return ONE32;
    }
    (((i128::from(q) - i128::from(c)) << 32) / total) as i64
}

/// D16: the memory halves toward the balance now (floored); the hazard is
/// the PA's times the memory's shortfall; shed where the draw lies below
/// `threshold(1 - e^-hazard)`, the probability `P_ONE - 2^-(hazard log2
/// e)` in Q62 (none where the hazard is none or less); else the kept share
/// is the bearer's times the presence of the draw's lead over its window.
pub fn balance(b: &Bud) -> Result<Balance, Fault> {
    if b.q < 0 || b.c < 0 {
        return Err(Fault::Range);
    }
    let memory = ((i128::from(b.memory) + i128::from(now(b.q, b.c))) >> 1) as i64;
    let hazard = mul(b.hazard, (b.tolerance - memory).max(0));
    let undecided = Balance {
        memory,
        threshold: 0,
        shed: false,
        kept: b.keep,
    };
    if hazard <= 0 {
        return Ok(undecided);
    }
    let x = ((i128::from(hazard) * LOG2E) >> 62) as i64;
    let t = threshold(P_ONE - exp2_neg(x));
    let shed = b.draw < t;
    let kept = if shed {
        b.keep
    } else {
        times(b.keep, of(over(t, b.draw), b.window))
    };
    Ok(Balance {
        threshold: t,
        shed,
        kept,
        ..undecided
    })
}
