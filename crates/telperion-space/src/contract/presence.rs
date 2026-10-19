//! Host decision 8 (FINE-CONTRACT.md, D17), in f32 (fn-254): every presence
//! the shedding sums read is taken by one procedure on both executors.
//! Shedding by fade decides topology from those sums; their living times
//! are cycles after a whole birth cycle, so no time grows large in an f32.
//!
//! - **A lead** is the draw's log-odds past the threshold its decision
//!   took (`under`, `over`), so a presence and its decision agree.
//! - **A window** is the draw's share of the tree's expected wood times the
//!   window's rate, clamped to its floor and span (`window`, `wood_at`); a
//!   presence is its lead over its window (`of`).
//! - **A product** of presences is taken in the order the grower takes
//!   them (`times`).
use super::{logit53, presence, INF, ONE as DRAWS};

/// A whole presence.
pub const ONE: f32 = 1.0;

/// `a b` of two presences, or of a presence and a time.
pub fn times(a: f32, b: f32) -> f32 {
    a * b
}

/// A non-negative real as an executor's value: none for a negative or NaN.
pub fn q(x: f64) -> f32 {
    if x > 0.0 {
        x as f32
    } else {
        0.0
    }
}

/// A presence as a real: exact.
pub fn real(p: f32) -> f64 {
    f64::from(p)
}

/// The lead, in log-odds, of a 53-bit draw `d` under the threshold `t` it
/// had to stay under: unbounded where the element is certain.
pub fn under(t: u64, d: u64) -> f32 {
    if t >= DRAWS || d == 0 {
        return INF;
    }
    logit53(t) - logit53(d)
}

/// The lead of a draw `d` over the threshold `t` it had to reach.
pub fn over(t: u64, d: u64) -> f32 {
    if t == 0 {
        return INF;
    }
    logit53(d) - logit53(t)
}

/// A draw `d`'s presence under the threshold `t` over `window`.
pub fn of_under(t: u64, d: u64, window: f32) -> f32 {
    of(under(t, d), window)
}

/// A draw `d`'s presence over the threshold `t` over `window`.
pub fn of_over(t: u64, d: u64, window: f32) -> f32 {
    of(over(t, d), window)
}

/// A window: `rate` times the wood's share `wood`, clamped to `floor` and
/// `span`; the rate and floor already scaled by the stake.
pub fn window(wood: f32, (rate, floor, span): (f32, f32, f32)) -> f32 {
    (rate * wood).clamp(floor, span)
}

/// The wood's share from `used` ticks of the way between the cycle's and
/// the next's, by their logs (`-INF` for none), or by the woods themselves
/// where either is none (D10's interpolation).
pub fn wood_at((a, b): (f32, f32), used: u64) -> f32 {
    let u = used as f32 * (1.0 / super::CYCLE as f32);
    // Each end weighed by its share, so neither cancels as `u` nears one.
    let lerp = |a: f32, b: f32| a * (1.0 - u) + b * u;
    let exp = |x: f32| if x <= -INF { 0.0 } else { super::exp(x) };
    if a <= -INF || b <= -INF {
        lerp(exp(a), exp(b))
    } else {
        super::exp(lerp(a, b))
    }
}

/// A draw's presence: its lead over its window.
pub fn of(lead: f32, window: f32) -> f32 {
    presence(lead, window)
}

/// What a child carries up into its bearer's living time (D17): the
/// bearer's own time `before` to the child's birth, in cycles, plus the
/// child's `weight` times the `gap` whole cycles from there and the child's
/// own living time `lived`.
pub fn carry(before: f32, weight: f32, gap: u32, lived: f32) -> f32 {
    before + weight * (gap as f32 + lived)
}

/// D17: an axis's fade, its delay plus a cycle `delay`, born in `birth`,
/// its subtree living `lived` cycles past its birth and its most present
/// living apex `alive`, at the tree's `age`: the delay left, clamped to
/// that apex and to one. Shed where it is none.
pub fn fade(delay: f32, birth: u32, (lived, alive): (f32, f32), age: u32) -> f32 {
    let idle = (i64::from(age) - i64::from(birth)) as f32 - lived;
    (delay - idle).clamp(alive.min(ONE), ONE)
}
