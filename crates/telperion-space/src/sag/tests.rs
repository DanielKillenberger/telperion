//! Sag's turn against today's f64 form (fn-211 task 9).
use super::*;
use std::f64::consts::PI;
fn old(lever: Vec3, cap: f64, sag: f64, radius: f64, length: f64) -> f64 {
    let reach = lever.length();
    if reach <= 0.0 || cap <= 0.0 {
        return 0.0;
    }
    let c = sag / radius.powi(4);
    let start = (-lever.z / reach).clamp(-1.0, 1.0).acos();
    let low = (cap / reach).min(1.0).asin();
    let high = PI - low;
    let settle = |fall: f64, s: f64| 2.0 * ((fall / 2.0).tan() * (-c * reach * s).exp()).atan();
    let (mut fall, mut left) = (start, length);
    if fall > high {
        let s = ((fall / 2.0).tan() / (high / 2.0).tan()).ln() / (c * reach);
        if s >= left {
            return start - settle(fall, left);
        }
        (fall, left) = (high, left - s);
    }
    if fall > low {
        let rate = c * cap;
        let s = (fall - low) / rate;
        if s >= left {
            return start - (fall - rate * left);
        }
        (fall, left) = (low, left - s);
    }
    start - settle(fall, left)
}
/// Random levers, caps, stiffnesses and lengths: the turn in f32 is the
/// f64 form's to within 1e-4 rad.
#[test]
fn the_turn_is_its_real_to_f32() {
    let mut seed = 99u64;
    let mut r = || {
        seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        (seed >> 11) as f64 / (1u64 << 53) as f64
    };
    let mut worst = (0.0f64, String::new());
    for _ in 0..200000 {
        let mag = 10f64.powf(r() * 8.0 - 6.0);
        let l = Vec3::new(r() * 2.0 - 1.0, r() * 2.0 - 1.0, r() * 2.0 - 1.0) * mag;
        let t = Vec3::new(-l.y, l.x, 0.0).length();
        let cap = t * (0.2 + r());
        let (sag, radius, length) = (10f64.powf(r() * 4.0 - 8.0), 10f64.powf(r() * 2.0 - 3.0), r() * 0.5);
        let want = old(l, cap, sag, radius, length);
        let lw = Load::of(0.0, l).moment;
        let capw = unit_w(torque(Load::of(0.0, Vec3::new(l.x, l.y, 0.0) * (cap / t)).moment)).1;
        let got = f64::from(turn(lw, capw, sag, radius, length)) * std::f64::consts::TAU;
        let d = (got - want).abs();
        if d > worst.0 {
            worst = (d, format!("{l:?} cap {cap} sag {sag} r {radius} len {length}: {got} vs {want}"));
        }
    }
    assert!(worst.0 < 1e-4, "{:e} at {}", worst.0, worst.1);
}
