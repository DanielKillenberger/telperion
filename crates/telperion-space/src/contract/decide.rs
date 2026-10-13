//! The thresholds a decision computes per bud (FINE-CONTRACT.md D5, D6,
//! D10, D11, D15 and D18), in f32 (fn-254). Each has a form both executors
//! run on the draw's bits, ticks and table values, and a form the CPU
//! grower calls with the species' reals, which rounds them once to f32 (the
//! values the device's tables hold) and calls the first. Where a setting
//! decides alone (a whole unit's survival, a hazard that neither rises nor
//! runs over part of a cycle), the threshold is the setting's own, as D1
//! gives it.
use super::{exp2, fails, floor53, ln, ln_of, log2, logit53, logit_of, neg_ln1m, neg_log2, neg_log2_of};
use super::{presence, prob, scaled};
use super::{threshold, ticks, CYCLE, INF, ONE};
use crate::lineage::{self, Draw};
use crate::presence::{Windows, FLOOR, RATE, SPAN};
use crate::species::{PaState, Zone};

/// D5: a unit's survival `p` over `share` of its cycle and `rest` more in
/// 2^-32 of a cycle, as a draw's threshold: `p^share`. `rest` is what a
/// failed unit's relay's first unit holds below a tick (fn-199 B2).
pub(crate) fn survival(p: f64, share: f64, rest: u64) -> u64 {
    if share >= 1.0 {
        return lineage::threshold(p);
    }
    survival_fine(neg_log2_of(p), (ticks(share) << 12) + rest)
}

/// D5 over a share in 2^-32 of a cycle (fn-199 B2): a survival of `-log2`
/// chance `l` a cycle. The threshold of `2^-x`, `ceil(2^-x 2^53)`: where
/// survival is likelier than not, the complement of its failure's,
/// `2^53 - floor(fails(x) 2^53)`, so every bit of a small chance of
/// failing is kept; where not, `2^-x`'s own, so every bit of a small
/// chance of surviving is (fn-254 review). The two meet at a half.
pub fn survival_fine(l: f32, share: u64) -> u64 {
    let x = scaled(l, share);
    if x >= 1.0 {
        return threshold(prob(x));
    }
    ONE - floor53(fails(x))
}

/// `-log2` of a draw's uniform, `d / 2^53`: `INF` for none. Near one it
/// is taken from the uniform's distance from one, which the uniform
/// itself would round away (fn-254 review).
pub fn draw_log2(d: u64) -> f32 {
    if d > ONE / 2 {
        return neg_ln1m((ONE - d) as f32 * (1.0 / ONE as f32)) * std::f32::consts::LOG2_E;
    }
    neg_log2(d as f32 * (1.0 / ONE as f32))
}

/// D5 extended (fn-199 R1, host 2026-10-07): the share of its cycle an
/// apex lived whose unit of `share` failed on draw `d` under survival `p` a
/// cycle, in ticks: `v^t = u` gives `t = log2 u / log2 p`, at most the
/// unit's share, so a draw at the survival's bound lives it whole.
pub(crate) fn lived_at(l: f32, share: f64, d: Draw) -> u64 {
    lived_ticks(l, ticks(share.min(1.0)), d.0)
}

/// A unit's survival as the device takes it (fn-256's spike contract,
/// COUNCIL.md "Decided by the spike"): its PA's viability `v` as the f32
/// its table holds, raised by shade to `v^(light^-φ)` in f32 by the
/// contract's `exp2` and `log2`; whole where `φ` is none or the light full;
/// in no light at all, `v^∞`: none, or certain where `v` is.
pub fn shaded(v: f32, phi: f32, light: f32) -> f32 {
    if phi > 0.0 && light <= 0.0 {
        return if v >= 1.0 { v } else { 0.0 };
    }
    if phi > 0.0 && light < 1.0 {
        crate::contract::exp2(crate::contract::log2(v) * crate::contract::exp2(-phi * crate::contract::log2(light)))
    } else {
        v
    }
}

/// D5 of an f32 survival `p`, as the device decides it: a whole cycle's
/// threshold of `p` itself, a part's from its `-log2`.
pub(crate) fn survival32(p: f32, share: f64, rest: u64) -> u64 {
    if share >= 1.0 {
        return crate::contract::threshold(p);
    }
    survival_fine(crate::contract::neg_log2(p), (ticks(share) << 12) + rest)
}

/// The cycles lived, `-log2(d / 2^53) / l`, where under a whole cycle:
/// none where the survival is certain or the draw at its bound.
fn lived_part(l: f32, d: u64) -> Option<f32> {
    let u = draw_log2(d);
    (u < l).then(|| u / l)
}

/// D5's lived time: a survival of `-log2` chance `l` a cycle, a unit of
/// `share` ticks, the 53-bit draw `d`; the ticks of `lived_part`, rounded
/// up, at most `share`: a draw at the bound lives the whole unit, so its
/// relay's rest is none, not a tick.
pub fn lived_ticks(l: f32, share: u64, d: u64) -> u64 {
    if l <= 0.0 {
        return share;
    }
    if l >= INF {
        return 0;
    }
    match lived_part(l, d) {
        None => share.min(1 << 20),
        Some(t) => ((t * CYCLE as f32).ceil() as u64).min(share),
    }
}

/// D5's lived time before it is put on the tick grid (fn-199 B2), in 2^-32
/// of a cycle: `lived_part`, floored, at most `share` ticks. What
/// `lived_ticks` rounds up past it, under a tick, is the part of its cycle
/// the relay's first unit holds below its ticks, which its thresholds read,
/// so a lived time crossing a tick moves none of them.
pub fn lived_fine(l: f32, share: u64, d: u64) -> u64 {
    if l <= 0.0 {
        return share << 12;
    }
    if l >= INF {
        return 0;
    }
    match lived_part(l, d) {
        None => share.min(1 << 20) << 12,
        Some(t) => ((t * 4_294_967_296.0) as u64).min(share << 12),
    }
}

/// The rest below a tick of a failed unit's relay's first unit: what
/// `lived_ticks` rounds `lived_fine` up by, in 2^-32 of a cycle.
pub(crate) fn lived_rest_at(l: f32, share: f64, d: Draw) -> u64 {
    let share = ticks(share.min(1.0));
    (lived_ticks(l, share, d.0) << 12) - lived_fine(l, share, d.0)
}

/// D6: the chance an apex aborts after a unit that ends `grown` cycles of
/// its axis's growth, exposed `exposure` of a cycle, as a draw's threshold:
/// `1 - (1 - a)^(grown^rise exposure)`. `rest` adds to the exposure in
/// 2^-32 of a cycle, as in `survival`.
pub(crate) fn abortion(state: &PaState, grown: f64, (exposure, rest): (f64, u64)) -> u64 {
    let a = state.abortion;
    if a <= 0.0 {
        return 0;
    }
    let exposure = (ticks(exposure) << 12) + rest;
    if !state.rises() && exposure >= 1 << 32 {
        return lineage::threshold(a);
    }
    let rise = state.rises().then(|| state.abortion_rise as f32);
    abortion_fine(neg_log2_of(1.0 - a), rise, ticks(grown), exposure)
}

/// D6 over an exposure in 2^-32 of a cycle (fn-199 B2): persistence of
/// `-log2` chance `l` a cycle, its hazard rising by `rise` with `grown`
/// ticks; the threshold of its abortion's chance, `fails`.
pub fn abortion_fine(l: f32, rise: Option<f32>, grown: u64, exposure: u64) -> u64 {
    let mut l = l;
    if let Some(rise) = rise.filter(|_| l < INF) {
        l = match grown {
            // grown^rise: none for a rising hazard, unbounded for a falling.
            0 if rise > 0.0 => 0.0,
            0 => INF,
            _ => l * exp2(rise * log2(grown as f32 * (1.0 / CYCLE as f32))),
        };
    }
    threshold(fails(scaled(l, exposure)))
}

/// D10: the presence of an apex's move on to its next age `pa` by the draw
/// `d` under `go`, the move deciding `stake` of the wood a bud of that age
/// is expected to grow from `used` of the way through `cycle`. Under one,
/// the apex all but stopped and also grows its relay.
pub(crate) fn moved(
    windows: &Windows,
    (pa, cycle, used): (usize, u32, f64),
    go: f64,
    d: Draw,
    stake: f64,
) -> f32 {
    let log = |c: u32| super::log32(windows.log_wood(pa, c));
    let window = ((RATE * stake) as f32, (FLOOR * stake) as f32, SPAN as f32);
    moved_f(logit_of(go), d.0, (log(cycle), log(cycle + 1)), ticks(used), window)
}

/// D10 on the executors' values: the log-odds of `go` (`logit_of`); the
/// logs of the wood at the cycle and the next (`-INF` for none); `used`
/// ticks; the window's rate, floor and span, the first two scaled by the
/// stake.
pub fn moved_f(go: f32, d: u64, woods: (f32, f32), used: u64, window: (f32, f32, f32)) -> f32 {
    if go >= INF || d == 0 {
        return 1.0;
    }
    let lead = go - logit53(d);
    let (rate, floor, span) = window;
    let wood = super::presence::wood_at(woods, used);
    presence(lead, (rate * wood).clamp(floor, span))
}

/// The window of a node or bud deciding `wood`: a node's presence (D18
/// reads it) is its draw's lead past its bound over it.
pub(crate) fn node_window(wood: f64) -> f32 {
    (RATE * wood).clamp(FLOOR, SPAN) as f32
}

/// A node's lead past a uniform law's bound of log `own` (`ln_of`): `ln u
/// - ln bound`, `u` the draw's uniform; unbounded for a bound of none.
pub fn past(d: u64, own: f32) -> f32 {
    if own <= -INF {
        return INF;
    }
    if d == 0 {
        return -INF;
    }
    ln(d as f32 * (1.0 / ONE as f32)) - own
}

/// A node's lead past a Poisson law's cumulative bound of log-odds `own`
/// (`logit_of`); unbounded for a bound of none.
pub fn above(d: u64, own: f32) -> f32 {
    if own <= -INF {
        return INF;
    }
    logit53(d) - own
}

/// A uniform law's node's presence over `window`.
pub(crate) fn node_past(d: Draw, bound: f64, window: f32) -> f32 {
    presence(past(d.0, ln_of(bound)), window)
}

/// A Poisson law's node's presence over `window`.
pub(crate) fn node_above(d: Draw, bound: f64, window: f32) -> f32 {
    presence(above(d.0, logit_of(bound)), window)
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
    wake_ticks(delay, (1.0 / zone.rate) as f32, d.0).map_or(f64::INFINITY, |t| t as f64 / CYCLE)
}

/// D11 on the executors' values: `delay` ticks, `per` years a nat; the
/// waking time in ticks, the wait floored to a tick, none past any age.
pub fn wake_ticks(delay: u64, per: f32, d: u64) -> Option<u64> {
    // -ln(1 - u): a small uniform kept whole by the series, a large one
    // from its integer complement, which its f32 would round to one
    // (fn-254 review).
    let nats = if d > ONE / 2 {
        -ln((ONE - d) as f32 * (1.0 / ONE as f32))
    } else {
        neg_ln1m(d as f32 * (1.0 / ONE as f32))
    };
    let wait = nats * per * CYCLE as f32;
    (wait < 4_611_686_018_427_387_904.0).then(|| delay + wait as u64)
}

/// D15: whether a chain's own axis is kept, its borne buds all dead by its
/// threshold with `-log2` chance `dead` (summed over them, in any order),
/// conditioned for the chain's first axis on its bearer having been kept on
/// its other buds' chances alone, all dead with `-log2` chance `context`:
/// kept unless `d` falls under the chance all die. Conditioned, that is
/// `2^-dead (1 - 2^-r) / (1 - 2^-(dead + r))`, each factor taken whole so
/// no difference of near-equal chances rounds it away (fn-254 review);
/// none where nothing could die.
pub fn kept(dead: f32, context: Option<f32>, d: u64) -> bool {
    // The chance all die and its complement, the chance one lives:
    // `2^-dead`, `1 - 2^-dead`; conditioned, each over `1 - 2^-(dead + r)`.
    let (all_die, lives) = match context {
        None => (prob(dead), fails(dead)),
        Some(r) => {
            let rest = fails(and(dead, r));
            if rest <= 0.0 {
                (0.0, 1.0)
            } else {
                (prob(dead) * fails(r) / rest, fails(dead) / rest)
            }
        }
    };
    // The threshold from whichever is the smaller, so a near-certain death
    // keeps its small chance of living (fn-254 review).
    let t = if all_die <= 0.5 {
        threshold(all_die)
    } else {
        ONE - floor53(lives.min(1.0))
    };
    d >= t
}

/// The sum of two `-log2` chances: their product's.
pub fn and(a: f32, b: f32) -> f32 {
    if a >= INF || b >= INF {
        return INF;
    }
    (a + b).min(INF)
}

/// D18: where along its parent's last growth unit a relay stands at
/// `share`: the node whose span holds that share of the unit's node
/// presences (`presences`), counted from the unit's first node `first` of
/// `n`; by count where the unit has no presence at all.
pub(crate) fn relay_node(presences: &[f32], first: usize, n: usize, share: f64) -> usize {
    relay_node_f(presences, first, n, share.clamp(0.0, 1.0) as f32)
}

/// D18 on the executors' values, the presences summed in order.
pub fn relay_node_f(presences: &[f32], first: usize, n: usize, share: f32) -> usize {
    let total: f32 = presences.iter().sum();
    if total > 0.0 {
        let target = share * total;
        let mut before = 0.0;
        for (j, &p) in presences.iter().enumerate() {
            if p > 0.0 && before + p >= target {
                return first + j;
            }
            before += p;
        }
        return n - 1;
    }
    // first + share (n - first), rounded up, among the unit's nodes.
    let k = first + (share * (n - first) as f32).ceil() as usize;
    k.clamp(first + 1, n) - 1
}
