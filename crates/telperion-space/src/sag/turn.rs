//! Sag's turn (CONTRACT-A.md 2.5, in f32 since fn-254): the torque, the
//! ground's share of a lever, and the beam's turn under it, taken in logs
//! and amounts in turns.
use super::W3;
use crate::contract::{exp2, log2, INF};
use crate::geometry::q::{asin, atan2, between, radians, sincos, turns, unit};
use crate::transcendental::Transcendental;

/// The torque gravity puts on a `lever`: `(S - W e) × (-z)`, horizontal,
/// the axis the wood turns down about, its length the moment.
pub(crate) fn torque(lever: W3) -> W3 {
    [-lever[1], lever[0], 0.0]
}

/// The share of a lever the ground leaves to the wood before it: where
/// the wood from a phytomer's base `height` above the ground, turned
/// rigidly to its pose now along `chord`, would go below it, the ground
/// carries that share, and the rest, nearer, bears with a lever as much
/// shorter (host, 2026-10-05). Measured from the base, so it falls by
/// degree as the phytomer's own end reaches the ground.
pub(crate) fn held(height: f32, chord: [f32; 3]) -> f32 {
    let low = height + chord[2];
    if low >= 0.0 {
        return 1.0;
    }
    if height <= 0.0 {
        return 0.0;
    }
    let free = height / (height - low);
    free * free
}

/// The log2 of a positive value; `-INF` for none.
pub(crate) fn log2_q(x: f32) -> f32 {
    if x <= 0.0 {
        return -INF;
    }
    log2(x)
}

/// The unit vector along a lever and the log2 of its length: `-INF` for
/// none.
pub(crate) fn unit_w(v: W3) -> (W3, f32) {
    match unit(v) {
        None => ([0.0; 3], -INF),
        Some(u) => (u, log2(crate::geometry::q::length(v))),
    }
}

/// `2 atan(tan(fall / 2) e^-k)` for `fall` in [0, 1/2] turn and `x = k
/// log2 e`, taken without a tangent: `2 atan2(sin(fall / 2) e^-k, cos(fall
/// / 2))`. An upright lever (`fall` a half turn) stays a half turn;
/// `atan2(0, 0)` is none.
fn settle(fall: f32, x: f32) -> f32 {
    let (c, s) = sincos(fall * 0.5);
    let y = s * exp2(-x);
    if y == 0.0 && c <= 0.0 {
        return 0.0;
    }
    atan2(y, c) * 2.0
}

/// `ln(tan(fall / 2) / tan(high / 2))` for angles in [0, 1/2] turn: the log
/// ratio of the products `sin_f cos_h` and `cos_f sin_h`, each factor at
/// least none; `INF` where the second is none (an upright `fall`), `-INF`
/// where the first is.
fn tangent_ratio(fall: f32, high: f32) -> f32 {
    let (cf, sf) = sincos(fall * 0.5);
    let (ch, sh) = sincos(high * 0.5);
    let (a, b) = (sf.max(0.0) * ch.max(0.0), cf.max(0.0) * sh.max(0.0));
    if a <= 0.0 {
        return -INF;
    }
    if b <= 0.0 {
        return INF;
    }
    (log2(a) - log2(b)) * std::f32::consts::LN_2
}

/// The turn, an amount in turns, of a phytomer `length` metres long and
/// `radius` thick under `lever` (turned into its frame now) and its PA's
/// `sag`, its moment never more than the cap whose log2 is `cap`, the
/// moment its unbent lever puts on it (host, 2026-10-05: the correction
/// only ever shrinks the lever). The beam's curvature is sag times the
/// moment over the radius to the fourth, taken in logs. The load, carried
/// rigidly, falls `fall` from hanging straight below the phytomer's end,
/// and its moment is its reach times sin(fall), so dfall/ds = -c
/// min(reach sin(fall), cap). Where the cap holds, the fall closes at a
/// constant rate; elsewhere tan(fall / 2) shrinks by e^(-c reach s), taken
/// without a tangent (`settle`). Integrated exactly over the phytomer: a
/// light load turns as the small-deflection beam does, a heavy one turns
/// until it hangs and never past, and upright wood never buckles.
pub(crate) fn turn(lever: W3, cap: f32, sag: f64, radius: f64, length: f64) -> f32 {
    let (u, reach) = unit_w(lever);
    let (r, left) = (radius as f32, length as f32);
    if reach <= -INF || cap <= -INF || r <= 0.0 || left <= 0.0 {
        return 0.0;
    }
    // The PA's sag enters as its log, converted once.
    let c = sag.log2_fixed() as f32 - 4.0 * log2_q(r);
    // The fall from hanging, in [0, 1/2] turn: by its sine and cosine, so
    // an upright lever is a half turn and never wraps to minus one, and a
    // small lean near upright keeps its size (fn-254 review).
    let start = atan2(u[0].hypot_fixed(u[1]), -u[2]);
    let low = between(0.0, asin(exp2((cap - reach).min(0.0))));
    let high = 0.5 - low;
    // `fall` settled over `s` metres: 2 atan(tan(fall / 2) e^(-c reach s)).
    let settled = |fall: f32, s: f32| {
        if s <= 0.0 {
            return fall;
        }
        let k = exp2(c + reach + log2_q(s));
        settle(fall, k * std::f32::consts::LOG2_E)
    };
    let (mut fall, mut left) = (start, left);
    // Near upright the turned lever is the smaller, until its moment
    // reaches the cap.
    if fall > high {
        let ln = tangent_ratio(fall, high);
        let s = if ln >= INF {
            INF
        } else {
            ln * exp2(-(c + reach))
        };
        if s >= left {
            return start - settled(fall, left);
        }
        (fall, left) = (high, left - s);
    }
    // Where the cap holds, the moment is the unbent one.
    if fall > low {
        let rate = c + cap;
        let s = radians(fall - low) * exp2(-rate);
        if s >= left {
            return start - (fall - turns(exp2(rate + log2_q(left))));
        }
        (fall, left) = (low, left - s);
    }
    start - settled(fall, left)
}
