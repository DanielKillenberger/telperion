//! The thresholds a decision computes per bud (FINE-CONTRACT.md D5, D6,
//! D10, D11, D15 and D18). Each has an integer form, which both executors
//! run on the draw's bits, ticks and table values, and a form the CPU
//! grower calls with the species' reals, which computes the table values
//! (once per species point on the GPU's side) and calls the integer one.
//! Where a setting decides alone (a whole unit's survival, a hazard that
//! neither rises nor runs over part of a cycle), the threshold is the
//! setting's own, as D1 gives it.
use super::{
    exp, exp2, ln_ratio, log2, logit, logit_draw, mul, mul62, neg_log2, presence, prob, q32, q62,
    scaled, threshold, ticks, CYCLE, INF, ONE32, P_ONE,
};
use crate::lineage::{self, Draw, ONE};
use crate::presence::{Windows, FLOOR, RATE, SPAN};
use crate::species::{PaState, Zone};

/// D5: a unit's survival `p` over `share` of its cycle, as a draw's
/// threshold: `p^share`.
pub(crate) fn survival(p: f64, share: f64) -> u64 {
    if share >= 1.0 {
        return lineage::threshold(p);
    }
    survival_q(neg_log2(q62(p)), ticks(share))
}

/// D5's integer form: a survival of `-log2` chance `l`, Q32, over `share`
/// ticks.
pub fn survival_q(l: i64, share: u64) -> u64 {
    threshold(prob(scaled(l, share)))
}

/// D6: the chance an apex aborts after a unit that ends `grown` cycles of
/// its axis's growth, exposed `exposure` of a cycle, as a draw's
/// threshold: `1 - (1 - a)^(grown^rise exposure)`.
pub(crate) fn abortion(state: &PaState, grown: f64, exposure: f64) -> u64 {
    let a = state.abortion;
    if a <= 0.0 {
        return 0;
    }
    if !state.rises() && exposure >= 1.0 {
        return lineage::threshold(a);
    }
    let rise = state.rises().then(|| q32(state.abortion_rise));
    abortion_q(neg_log2(q62(1.0 - a)), rise, ticks(grown), ticks(exposure))
}

/// D6's integer form: persistence of `-log2` chance `l` a cycle, Q32, its
/// hazard rising by `rise` (Q32) with `grown` ticks, over `exposure`
/// ticks.
pub fn abortion_q(l: i64, rise: Option<i64>, grown: u64, exposure: u64) -> u64 {
    let mut l = l;
    if let Some(rise) = rise.filter(|_| l < INF) {
        l = match grown {
            // grown^rise: none for a rising hazard, unbounded for a falling.
            0 if rise > 0 => 0,
            0 => INF,
            _ => mul(l, exp2(mul(rise, log2(grown) - (20 << 32)))),
        };
    }
    ONE - (prob(scaled(l, exposure)) >> 9)
}

/// D10: the presence, Q32, of an apex's move on to its next age `pa` by
/// the draw `d` under `go`, the move deciding `stake` of the wood a bud of
/// that age is expected to grow from `used` of the way through `cycle`.
/// Under one, the apex all but stopped and also grows its relay.
pub(crate) fn moved(
    windows: &Windows,
    (pa, cycle, used): (usize, u32, f64),
    go: f64,
    d: Draw,
    stake: f64,
) -> u64 {
    let woods = (windows.log_wood(pa, cycle), windows.log_wood(pa, cycle + 1));
    let window = (q32(RATE * stake), q32(FLOOR * stake), q32(SPAN));
    moved_q(
        q62(go),
        d.0,
        (q32(woods.0), q32(woods.1)),
        ticks(used),
        window,
    )
}

/// D10's integer form: `go` Q62; the logs of the wood at the cycle and
/// the next, Q32 (`-INF` for none); `used` ticks; the window's rate, floor
/// and span, Q32, the first two scaled by the stake.
pub fn moved_q(go: u64, d: u64, woods: (i64, i64), used: u64, window: (i64, i64, i64)) -> u64 {
    if go >= P_ONE || d == 0 {
        return ONE32 as u64;
    }
    let lead = logit(go) - logit_draw(d);
    // The log of the wood interpolated by the used ticks, or the wood
    // itself where either is none.
    let used = i128::from(used);
    let lerp = |a: i64, b: i64| a + ((i128::from(b - a) * used) >> 20) as i64;
    let (a, b) = woods;
    let wood = if a <= -INF || b <= -INF {
        lerp(exp(a), exp(b))
    } else {
        exp(lerp(a, b))
    };
    let (rate, floor, span) = window;
    presence(lead, mul(rate, wood).clamp(floor, span))
}

/// The window, Q32, of a node or bud deciding `wood`: a node's presence
/// (D18 reads it) is its draw's lead past its bound over it.
pub(crate) fn node_window(wood: f64) -> i64 {
    q32((RATE * wood).clamp(FLOOR, SPAN))
}

/// A node's lead past a uniform law's `bound`: `ln u - ln bound`.
#[cfg(test)]
pub(crate) fn past(d: Draw, bound: f64) -> i64 {
    past_q(d.0, q62(bound))
}

/// `past`'s integer form, `bound` Q62.
pub fn past_q(d: u64, bound: u64) -> i64 {
    if bound == 0 {
        return INF;
    }
    ln_ratio(d << 9, bound)
}

/// A node's lead past a Poisson law's cumulative `bound`, in log-odds.
#[cfg(test)]
pub(crate) fn above(d: Draw, bound: f64) -> i64 {
    above_q(d.0, q62(bound))
}

/// A uniform law's node's presence over `window`, Q32: `node`'s of
/// `past(d, bound)`, whole without its logs where bounds of them settle
/// it (`presence::of_under`).
pub(crate) fn node_past(d: Draw, bound: f64, window: i64) -> u64 {
    let bound = q62(bound);
    if window > 0 && bound != 0 && d.0 != 0 {
        let (low, _) = presence::log2_bounds(d.0 << 9);
        if presence::ln(low - presence::kept_log2(bound)) >= window {
            return ONE32 as u64;
        }
    }
    super::presence(past_q(d.0, bound), window)
}

/// A Poisson law's node's presence over `window`, Q32: `node`'s of
/// `above(d, bound)`, settled as `node_past` settles one.
pub(crate) fn node_above(d: Draw, bound: f64, window: i64) -> u64 {
    let bound = q62(bound);
    if window > 0 && bound != 0 && presence::bounded(d.0) {
        let (low, _) = presence::logit_draw_bounds(d.0);
        if low - presence::kept_logit(bound) >= window {
            return ONE32 as u64;
        }
    }
    super::presence(above_q(d.0, bound), window)
}

/// `above`'s integer form, `bound` Q62.
pub fn above_q(d: u64, bound: u64) -> i64 {
    if bound == 0 {
        return INF;
    }
    logit_draw(d) - logit(bound)
}

/// D11: the years after its node grew that a sleeping bud wakes, on the
/// tick grid, for the draw `d`: the zone's delay, on the grid, then the
/// exponential wait `-ln(1 - u) / rate`, floored to a tick. Unbounded when
/// it never wakes.
pub(crate) fn wake(zone: &Zone, d: Draw) -> f64 {
    if zone.rate <= 0.0 {
        return f64::INFINITY;
    }
    let delay = ticks(super::grid(zone.delay));
    wake_q(delay, q32(1.0 / zone.rate), d.0).map_or(f64::INFINITY, |t| t as f64 / CYCLE)
}

/// D11's integer form: `delay` ticks, `per` years a nat, Q32; the waking
/// time in ticks, none past any age.
pub fn wake_q(delay: u64, per: i64, d: u64) -> Option<u64> {
    let wait = mul(ln_ratio(ONE, ONE - d), per);
    (wait < INF).then(|| delay + (wait >> 12) as u64)
}

/// D15: whether a chain's own axis is kept, its borne buds all dead by its
/// threshold with `-log2` chance `dead` (summed over them, in any order),
/// conditioned for the chain's first axis on its bearer having been kept
/// on its other buds' chances alone, all dead with `-log2` chance
/// `context`: kept unless `d` falls under `1 - lives`.
pub fn kept(dead: i64, context: Option<i64>, d: u64) -> bool {
    let dead = prob(dead);
    let mut lives = P_ONE - dead;
    if let Some(r) = context {
        // lives / (1 - dead r), and whole where nothing is left to divide.
        let rest = P_ONE - mul62(dead, prob(r));
        lives = if rest == 0 {
            P_ONE
        } else {
            ((u128::from(lives) << 62) / u128::from(rest)).min(u128::from(P_ONE)) as u64
        };
    }
    d >= threshold(P_ONE - lives)
}

/// `-log2` of a chance, Q32, for a sum: `INF` for none.
pub(crate) fn neg_log(p: f64) -> i64 {
    neg_log2(q62(p))
}

/// The sum of two `-log2` chances: their product's, saturated.
pub fn and(a: i64, b: i64) -> i64 {
    a.saturating_add(b).min(INF)
}

/// D18: where along its parent's last growth unit a relay stands at
/// `share`: the node whose span holds that share of the unit's node
/// presences (Q32, `presences`), counted from the unit's first node
/// `first` of `n`; by count where the unit has no presence at all.
pub(crate) fn relay_node(presences: &[u64], first: usize, n: usize, share: f64) -> usize {
    relay_node_q(presences, first, n, q32(share.clamp(0.0, 1.0)) as u64)
}

/// D18's integer form, `share` Q32.
pub fn relay_node_q(presences: &[u64], first: usize, n: usize, share: u64) -> usize {
    let share = u128::from(share);
    let total: u128 = presences.iter().map(|&p| u128::from(p)).sum();
    if total > 0 {
        let target = share * total;
        let mut before = 0u128;
        for (j, &p) in presences.iter().enumerate() {
            let p = u128::from(p);
            if p > 0 && (before + p) << 32 >= target {
                return first + j;
            }
            before += p;
        }
        return n - 1;
    }
    // first + share (n - first), rounded up, among the unit's nodes.
    let f = share * (n - first) as u128;
    let k = first + f.div_ceil(1 << 32) as usize;
    k.clamp(first + 1, n) - 1
}
