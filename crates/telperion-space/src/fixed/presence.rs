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
use super::{exp, log2, logit, logit_draw, mul, presence, wide_signed, INF, LN2, ONE32};

/// Q32's one, as a presence.
pub const ONE: u64 = ONE32 as u64;

/// `a b` of two Q32 values, the second any Q32 (a time or a presence), in
/// the second's scale, floored.
pub fn times(a: u64, b: u64) -> u64 {
    (super::wide(a, b) >> 32) as u64
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

/// A draw `d`'s presence under the threshold `t` over `window`: its
/// lead's (`under`). Bounds of the draw's log-odds settle most presences
/// whole without the logs, which are most of growth's arithmetic on one
/// core; the presence is the same either way.
pub fn of_under(t: u64, d: u64, window: i64) -> u64 {
    if window > 0 && t < 1 << 53 && bounded(d) {
        let (_, high) = logit_draw_bounds(d);
        if threshold_logit(t) - high >= window {
            return ONE;
        }
    }
    of(under(t, d), window)
}

/// A draw `d`'s presence over the threshold `t` over `window`: its
/// lead's (`over`), settled as `of_under` settles one.
pub fn of_over(t: u64, d: u64, window: i64) -> u64 {
    if window > 0 && t != 0 && bounded(d) {
        let (low, _) = logit_draw_bounds(d);
        if low - threshold_logit(t) >= window {
            return ONE;
        }
    }
    of(over(t, d), window)
}

/// Whether a 53-bit draw and its complement each have the nine bits
/// `log2_bounds` reads.
pub(super) fn bounded(d: u64) -> bool {
    d >= 1 << 8 && (1 << 53) - d >= 1 << 8
}

/// Bounds of `logit_draw(d)`: `ln_ratio` of the bounds of its two logs,
/// whose difference it is monotone in.
pub(super) fn logit_draw_bounds(d: u64) -> (i64, i64) {
    let ((a_low, a_high), (b_low, b_high)) = (log2_bounds(d), log2_bounds((1 << 53) - d));
    (ln(a_low - b_high), ln(a_high - b_low))
}

/// `ln_ratio` of a difference of two logs: monotone in it.
pub(super) fn ln(log2s: i64) -> i64 {
    (wide_signed(log2s, LN2) >> 62) as i64
}

/// Bounds of `log2 n`, `n` of nine bits or more: the logs of `n` with all
/// but its leading nine bits cleared and of the next such integer, which
/// `log2`, monotone, puts it between. Each is a grid point's log, its
/// exponent added: the same mantissa, so the same squarings.
pub(super) fn log2_bounds(n: u64) -> (i64, i64) {
    let e = 63 - n.leading_zeros();
    let k = ((n >> (e - 8)) & 0xff) as usize;
    let exponent = i64::from(e - 8) << 32;
    (exponent + GRID[k], exponent + GRID[k + 1])
}

/// `log2 (256 + k)` for k to 256, Q32.
const GRID: [i64; 257] = {
    let mut grid = [0; 257];
    let mut k = 0;
    while k < 257 {
        grid[k] = log2(256 + k as u64);
        k += 1;
    }
    grid
};

/// The log-odds of a draw's threshold `t`, Q32.
fn threshold_logit(t: u64) -> i64 {
    kept_logit(t << 9)
}

/// The log-odds of a Q62 probability `p`, Q32 (`logit`). A tree meets
/// few: most are a species' settings, so the last taken are kept, by the
/// core that took them, and a log a draw is not taken twice for one.
pub(super) fn kept_logit(p: u64) -> i64 {
    kept(p, logit)
}

/// `log2` of a Q62 probability `p`, kept as `kept_logit` keeps one.
pub(super) fn kept_log2(p: u64) -> i64 {
    kept(p | 1 << 63, |key| log2(key & !(1 << 63)))
}

/// `log(key)`, from the last 256 the core took. A key holds a Q62 value,
/// at most 2^62, and the 63rd bit names the log; none is `u64::MAX`, so
/// an empty slot matches none.
fn kept(key: u64, log: impl Fn(u64) -> i64) -> i64 {
    use std::cell::Cell;
    const SLOTS: usize = 256;
    thread_local! {
        static KEPT: [Cell<(u64, i64)>; SLOTS] =
            const { [const { Cell::new((u64::MAX, 0)) }; SLOTS] };
    }
    let slot = (key.wrapping_mul(0x9e37_79b9_7f4a_7c15) >> 56) as usize;
    KEPT.with(|kept| {
        let (k, value) = kept[slot].get();
        if k == key {
            return value;
        }
        let value = log(key);
        kept[slot].set((key, value));
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
