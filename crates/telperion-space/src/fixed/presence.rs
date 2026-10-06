//! Host decision 8 (FINE-CONTRACT.md, D17): every presence the shedding
//! sums read is a Q32 integer, `p` being `n / 2^32`, taken by one
//! procedure on both executors. Shedding by fade decides topology from
//! those sums, so they are integers too: living times in Q32.32 (a `u64`
//! of cycles), presences in Q32, every product floored.
//!
//! - **A lead** is the draw's log-odds past the threshold its decision
//!   took, Q32 (`under`, `over`), so a presence and its decision agree.
//! - **A window** is the draw's share of the tree's expected wood times
//!   the window's rate, clamped to its floor and span, Q32 (`window`,
//!   `window_at`); a presence is its lead over its window (`presence`).
//! - **A product** of presences is floored at each step, in the order the
//!   grower takes them (`times`).
//! - A species' real (a share, a reach, a continuation) enters as `q`,
//!   floored, computed once per species point.
use super::{exp, logit, logit_draw, mul, presence, INF, ONE32};

/// Q32's one, as a presence.
pub const ONE: u64 = ONE32 as u64;

/// `a b` of two Q32 values, the second any Q32 (a time or a presence), in
/// the second's scale, floored.
pub fn times(a: u64, b: u64) -> u64 {
    ((u128::from(a) * u128::from(b)) >> 32) as u64
}

/// A non-negative real as Q32, floored: exact where the real is a Q32
/// value or a time on the tick grid.
pub fn q(x: f64) -> u64 {
    if x.is_nan() || x <= 0.0 {
        return 0;
    }
    (x * ONE as f64).floor() as u64
}

/// A presence from a real that holds one exactly.
pub fn real(p: u64) -> f64 {
    super::real(p)
}

/// The lead, in log-odds Q32, of a 53-bit draw `d` under the threshold
/// `t` it had to stay under: unbounded where the element is certain.
pub fn under(t: u64, d: u64) -> i64 {
    if t >= 1 << 53 || d == 0 {
        return INF;
    }
    logit(t << 9) - logit_draw(d)
}

/// The lead of a draw `d` over the threshold `t` it had to reach.
pub fn over(t: u64, d: u64) -> i64 {
    if t == 0 {
        return INF;
    }
    logit_draw(d) - logit(t << 9)
}

/// A window, Q32: `rate` times the wood's share `wood` (Q32), clamped to
/// `floor` and `span`; the rate and floor already scaled by the stake.
pub fn window(wood: i64, (rate, floor, span): (i64, i64, i64)) -> i64 {
    mul(rate, wood).clamp(floor, span)
}

/// The wood's share from `used` ticks of the way between the cycle's and
/// the next's, by their logs (Q32, `-INF` for none), or by the woods
/// themselves where either is none (D10's interpolation).
pub fn wood_at(woods: (i64, i64), used: u64) -> i64 {
    let used = i128::from(used);
    let lerp = |a: i64, b: i64| a + ((i128::from(b - a) * used) >> 20) as i64;
    let (a, b) = woods;
    if a <= -INF || b <= -INF {
        lerp(exp(a), exp(b))
    } else {
        exp(lerp(a, b))
    }
}

/// A draw's presence: its lead over its window.
pub fn of(lead: i64, window: i64) -> u64 {
    presence(lead, window)
}
