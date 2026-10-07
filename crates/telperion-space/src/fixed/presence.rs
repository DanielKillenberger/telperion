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
    // Positive, so the cast's truncation is the floor, without a call.
    (x * ONE as f64) as u64
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
    threshold_logit(t) - logit_draw(d)
}

/// The lead of a draw `d` over the threshold `t` it had to reach.
pub fn over(t: u64, d: u64) -> i64 {
    if t == 0 {
        return INF;
    }
    logit_draw(d) - threshold_logit(t)
}

/// The log-odds of a draw's threshold `t`, Q32. A tree meets few: most
/// are a species' settings, so the last taken are kept, by the core that
/// took them, and a log a draw is not taken twice for one.
fn threshold_logit(t: u64) -> i64 {
    use std::cell::Cell;
    const SLOTS: usize = 256;
    thread_local! {
        // No threshold reaches `u64::MAX`, so an empty slot matches none.
        static KEPT: [Cell<(u64, i64)>; SLOTS] =
            const { [const { Cell::new((u64::MAX, 0)) }; SLOTS] };
    }
    let slot = (t.wrapping_mul(0x9e37_79b9_7f4a_7c15) >> 56) as usize;
    KEPT.with(|kept| {
        let (key, value) = kept[slot].get();
        if key == t {
            return value;
        }
        let value = logit(t << 9);
        kept[slot].set((t, value));
        value
    })
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

/// What a child carries up into its bearer's living time (D17): the
/// bearer's own time `before` to the child's birth (Q32.32), plus the
/// child's `weight` (Q32) times the `gap` whole cycles from there and the
/// child's own living time `lived` (Q32.32).
pub fn carry(before: u64, weight: u64, gap: u64, lived: u64) -> u64 {
    before + times(weight, (gap << 32) + lived)
}

/// D17: an axis's fade, Q32, its delay plus a cycle `delay` (Q32), born
/// in `birth`, its subtree living `lived` (Q32.32) past its birth and its
/// most present living apex `alive` (Q32), at the tree's `age`: the delay
/// left, clamped to that apex and to one. Shed where it is none.
pub fn fade(delay: u64, birth: u32, (lived, alive): (u64, u64), age: u32) -> u64 {
    let until = (i64::from(birth) << 32) + lived as i64;
    let t = delay as i64 - ((i64::from(age) << 32) - until);
    t.clamp(alive.min(ONE) as i64, ONE as i64) as u64
}
