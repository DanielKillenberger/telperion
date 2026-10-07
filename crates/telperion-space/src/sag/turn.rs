//! Sag's turn in contract A (CONTRACT-A.md 2.5): the torque, the ground's
//! share of a lever, and the beam's turn under it, taken in Q32 logs and
//! amounts.
use super::W3;
use crate::fixed::feedback::angle::{acos, asin, between, radians, turns};
use crate::fixed::feedback::edge::{settle, tangent_ratio};
use crate::fixed::feedback::vector::unit_w;
use crate::fixed::{exp2, log2, mul, q32, INF, LOG2E, ONE32};

/// The torque gravity puts on a `lever` (W): `(S - W e) × (-z)`,
/// horizontal, the axis the wood turns down about, its length the moment.
pub(crate) fn torque(lever: W3) -> W3 {
    [-lever[1], lever[0], 0]
}

/// The share of a lever the ground leaves to the wood before it, Q32:
/// where the wood from a phytomer's base `height` above the ground,
/// turned rigidly to its pose now along `chord`, would go below it, the
/// ground carries that share, and the rest, nearer, bears with a lever as
/// much shorter (host, 2026-10-05). Measured from the base, so it falls by
/// degree as the phytomer's own end reaches the ground.
pub(crate) fn held(height: i64, chord: [i64; 3]) -> i64 {
    let low = height + chord[2];
    if low >= 0 {
        return ONE32;
    }
    if height <= 0 {
        return 0;
    }
    let free = ((i128::from(height) << 32) / i128::from(height - low)) as i64;
    mul(free, free)
}

/// The log2 of a positive Q32, Q32; `-INF` for none.
pub(crate) fn log2_q(x: i64) -> i64 {
    if x <= 0 {
        return -INF;
    }
    log2(x as u64) - (32 << 32)
}

/// The turn, an amount (Q32 turns), of a phytomer `length` metres long
/// and `radius` thick under `lever` (W, turned into its frame now) and its
/// PA's `sag`, its moment never more than the cap whose log2 is `cap`, the
/// moment its unbent lever puts on it (host, 2026-10-05: the correction
/// only ever shrinks the lever). The beam's curvature is sag times the
/// moment over the radius to the fourth, taken in logs. The load, carried
/// rigidly, falls `fall` from hanging straight below the phytomer's end,
/// and its moment is its reach times sin(fall), so dfall/ds = -c
/// min(reach sin(fall), cap). Where the cap holds, the fall closes at a
/// constant rate; elsewhere tan(fall / 2) shrinks by e^(-c reach s),
/// taken without a tangent (`edge::settle`). Integrated exactly over the
/// phytomer: a light load turns as the small-deflection beam does, a heavy
/// one turns until it hangs and never past, and upright wood never
/// buckles (CONTRACT-A.md 2.5).
pub(crate) fn turn(lever: W3, cap: i64, sag: f64, radius: f64, length: f64) -> i64 {
    let (u, reach) = unit_w(lever);
    let (r, left) = (q32(radius), q32(length));
    if reach <= -INF || cap <= -INF || r <= 0 || left <= 0 {
        return 0;
    }
    // The PA's sag enters as its log, converted once (a Q32 of a value as
    // small as 1e-8 would keep two digits of it).
    let c = q32(sag.log2()) - 4 * log2_q(r);
    let start = between(0, acos(-u[2]));
    let low = between(0, asin(exp2((cap - reach).min(0))));
    let high = (1 << 31) - low;
    // `fall` settled over `s` metres: 2 atan(tan(fall / 2) e^(-c reach s)).
    let settled = |fall: i64, s: i64| {
        if s <= 0 {
            return fall;
        }
        let k = exp2(c + reach + log2_q(s));
        let x = ((i128::from(k) * LOG2E) >> 62) as i64;
        i64::from(settle(fall as u32, x))
    };
    let (mut fall, mut left) = (start, left);
    // Near upright the turned lever is the smaller, until its moment
    // reaches the cap.
    if fall > high {
        let ln = tangent_ratio(fall as u32, high as u32);
        let s = if ln >= INF {
            INF
        } else {
            mul(ln, exp2(-(c + reach)))
        };
        if s >= left {
            return start - settled(fall, left);
        }
        (fall, left) = (high, left - s);
    }
    // Where the cap holds, the moment is the unbent one.
    if fall > low {
        let rate = c + cap;
        let s = mul(radians(fall - low).expect("an angle"), exp2(-rate));
        if s >= left {
            return start - (fall - turns(exp2(rate + log2_q(left))));
        }
        (fall, left) = (low, left - s);
    }
    start - settled(fall, left)
}
