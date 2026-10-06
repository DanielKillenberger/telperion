//! The thresholds a decision computes per bud (FINE-CONTRACT.md D5, D6,
//! D10, D11, D15 and D18), each by `fixed.rs`'s procedure from the draw's
//! bits, times on the tick grid and the species' settings, so the two
//! executors take them alike. Where a setting decides alone (a whole
//! unit's survival, a hazard that neither rises nor runs over part of a
//! cycle), the threshold is the setting's own, as D1 gives it.
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
    threshold(prob(scaled(neg_log2(q62(p)), ticks(share))))
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
    let mut l = neg_log2(q62(1.0 - a));
    if state.rises() && l < INF {
        let t = ticks(grown);
        l = match t {
            // grown^rise: none for a rising hazard, unbounded for a falling.
            0 if state.abortion_rise > 0.0 => 0,
            0 => INF,
            _ => {
                let lt = log2(t) - (20 << 32);
                mul(l, exp2(mul(q32(state.abortion_rise), lt)))
            }
        };
    }
    let persist = prob(scaled(l, ticks(exposure)));
    ONE - (persist >> 9)
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
    let go = q62(go);
    if go >= P_ONE || d.0 == 0 {
        return ONE32 as u64;
    }
    let lead = logit(go) - logit_draw(d.0);
    presence(lead, window_at(windows, (pa, cycle, used), stake))
}

/// `Windows::decided`'s window, in Q32 log-odds, over the wood a bud of
/// `pa` is expected to grow from `used` of the way through `cycle`: its
/// log interpolated between the cycle's and the next's, or the wood
/// itself where either is none.
fn window_at(windows: &Windows, (pa, cycle, used): (usize, u32, f64), stake: f64) -> i64 {
    let (a, b) = (windows.log_wood(pa, cycle), windows.log_wood(pa, cycle + 1));
    let used = i128::from(ticks(used) as i64);
    let lerp = |a: i64, b: i64| a + ((i128::from(b - a) * used) >> 20) as i64;
    let wood = if a == f64::NEG_INFINITY || b == f64::NEG_INFINITY {
        lerp(exp(q32(a)), exp(q32(b)))
    } else {
        exp(lerp(q32(a), q32(b)))
    };
    let window = mul(q32(RATE * stake), wood);
    window.clamp(q32(FLOOR * stake), q32(SPAN))
}

/// A node's presence, Q32 (D18 reads it): its draw's lead `lead` past its
/// bound, Q32, over the window of the `wood` it decides.
pub(crate) fn node(lead: i64, wood: f64) -> u64 {
    let window = (RATE * wood).clamp(FLOOR, SPAN);
    presence(lead, q32(window))
}

/// A node's lead past a uniform law's `bound`: `ln u - ln bound`.
pub(crate) fn past(d: Draw, bound: f64) -> i64 {
    let bound = q62(bound);
    if bound == 0 {
        return INF;
    }
    ln_ratio(d.0 << 9, bound)
}

/// A node's lead past a Poisson law's cumulative `bound`, in log-odds.
pub(crate) fn above(d: Draw, bound: f64) -> i64 {
    let bound = q62(bound);
    if bound == 0 {
        return INF;
    }
    logit_draw(d.0) - logit(bound)
}

/// D11: the years after its node grew that a sleeping bud wakes, on the
/// tick grid, for the draw `d`: the zone's delay, on the grid, then the
/// exponential wait `-ln(1 - u) / rate`, floored to a tick. Unbounded when
/// it never wakes.
pub(crate) fn wake(zone: &Zone, d: Draw) -> f64 {
    if zone.rate <= 0.0 {
        return f64::INFINITY;
    }
    let nats = ln_ratio(ONE, ONE - d.0);
    let wait = mul(nats, q32(1.0 / zone.rate));
    if wait >= INF {
        return f64::INFINITY;
    }
    super::grid(zone.delay) + (wait >> 12) as f64 / CYCLE
}

/// D15: whether a chain's own axis is kept, its borne buds all dead by its
/// threshold with `-log2` chance `dead` (summed over them, in any order),
/// conditioned for the chain's first axis on its bearer having been kept
/// on its other buds' chances alone, all dead with `-log2` chance
/// `context`: kept unless `d` falls under `1 - lives`.
pub(crate) fn kept(dead: i64, context: Option<i64>, d: Draw) -> bool {
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
    d.0 >= threshold(P_ONE - lives)
}

/// `-log2` of a chance, Q32, for a sum: `INF` for none.
pub(crate) fn neg_log(p: f64) -> i64 {
    neg_log2(q62(p))
}

/// The sum of two `-log2` chances: their product's, saturated.
pub(crate) fn and(a: i64, b: i64) -> i64 {
    a.saturating_add(b).min(INF)
}

/// D18: where along its parent's last growth unit a relay stands at
/// `share`: the node whose span holds that share of the unit's node
/// presences (Q32, `presences`), counted from the unit's first node
/// `first` of `n`; by count where the unit has no presence at all.
pub(crate) fn relay_node(presences: &[u64], first: usize, n: usize, share: f64) -> usize {
    let share = u128::from(q32(share.clamp(0.0, 1.0)) as u64);
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
