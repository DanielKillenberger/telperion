//! The contract's fixed point against the reals it stands for: close to
//! them, monotone, and exact where it must be.
use super::*;

fn real32(x: i64) -> f64 {
    x as f64 / ONE32 as f64
}

#[test]
fn log2_is_within_a_few_units_of_its_last_place() {
    let mut n = 1u64;
    while n < u64::MAX / 5 {
        for m in [n, n + 1, n.saturating_mul(3) / 2] {
            let err = (real32(log2(m)) - (m as f64).log2()).abs();
            assert!(err < 4.0 / ONE32 as f64, "log2 {m}: {err}");
        }
        n += n / 2 + 1;
    }
    assert_eq!(log2(1), 0);
    assert_eq!(log2(1 << 40), 40 << 32);
    assert_eq!(log2(0), -INF);
}

/// `log2` as the contract first wrote it: the reference its faster form
/// is held to, bit for bit.
fn log2_reference(n: u64) -> i64 {
    if n == 0 {
        return -INF;
    }
    let e = 63 - n.leading_zeros();
    let mut m = if e <= 62 { n << (62 - e) } else { n >> 1 };
    let mut frac = 0u64;
    for _ in 0..32 {
        let square = (u128::from(m) * u128::from(m)) >> 62;
        frac <<= 1;
        if square >= 1 << 63 {
            frac |= 1;
            m = (square >> 1) as u64;
        } else {
            m = square as u64;
        }
    }
    (i64::from(e) << 32) | frac as i64
}

#[test]
fn log2_and_its_pairs_are_the_reference_to_the_bit() {
    let mut x = 0x9e37_79b9_7f4a_7c15u64;
    let edges = [
        1,
        2,
        3,
        (1 << 53) - 1,
        1 << 62,
        (1 << 63) - 1,
        1 << 63,
        u64::MAX,
    ];
    let mut values: Vec<u64> = edges.to_vec();
    for _ in 0..200_000 {
        x = x.wrapping_mul(6_364_136_223_846_793_005).wrapping_add(1);
        values.push(x >> (x % 64));
    }
    for pair in values.windows(2) {
        let (a, b) = (pair[0].max(1), pair[1].max(1));
        assert_eq!(log2(a), log2_reference(a), "log2 {a}");
        assert_eq!(log2_pair(a, b), (log2_reference(a), log2_reference(b)));
    }
}

#[test]
fn exp2_neg_is_close_and_monotone() {
    for k in 0..20_000i64 {
        let x = (k * 0x9e37_79b1) % (70 * ONE32);
        let got = exp2_neg(x) as f64 / P_ONE as f64;
        let want = (-real32(x)).exp2();
        assert!(
            (got - want).abs() <= want * 1e-15 + 1e-18,
            "2^-{x}: {got} {want}"
        );
    }
    let mut last = P_ONE;
    for x in (0..ONE32 * 3).step_by(9_973_331) {
        let v = exp2_neg(x);
        assert!(v <= last, "2^-x falls");
        last = v;
    }
    assert_eq!(exp2_neg(0), P_ONE);
    assert_eq!(exp2_neg(ONE32), P_ONE / 2);
    assert_eq!(exp2_neg(63 * ONE32), 0);
}

#[test]
fn exp2_and_exp_hold_on_both_sides_of_one() {
    for y in [-5.5, -1.0, -0.25, 0.0, 0.3, 1.0, 2.75, 10.0, 20.5] {
        let got = real32(exp2(q32(y)));
        assert!(
            (got - y.exp2()).abs() <= y.exp2() * 1e-8 + 1e-9,
            "2^{y}: {got}"
        );
        let got = real32(exp(q32(y)));
        assert!(
            (got - y.exp()).abs() <= y.exp() * 1e-8 + 1e-9,
            "e^{y}: {got}"
        );
    }
    assert_eq!(exp2(60 * ONE32), INF);
    assert_eq!(exp(-INF), 0);
}

#[test]
fn a_threshold_is_the_real_decision_on_q62() {
    // d / 2^53 < n / 2^62  <=>  d < ceil(n / 2^9).
    for n in [0u64, 1, 511, 512, 513, 1 << 40, P_ONE - 1, P_ONE] {
        let t = threshold(n);
        if t > 0 {
            assert!(u128::from(t - 1) << 9 < u128::from(n), "{n}");
        }
        assert!(u128::from(t) << 9 >= u128::from(n), "{n}");
    }
    assert_eq!(threshold(P_ONE), 1 << 53);
}

#[test]
fn a_share_of_a_survival_is_the_real_power() {
    for (p, share) in [(0.9, 0.5), (0.5, 0.25), (0.999, 0.01), (0.2, 0.75)] {
        let ticks = (share * CYCLE) as u64;
        let got = prob(scaled(neg_log2(q62(p)), ticks)) as f64 / P_ONE as f64;
        let want: f64 = p.powf(share);
        assert!((got - want).abs() < 1e-9, "{p}^{share}: {got}");
    }
    assert_eq!(prob(scaled(neg_log2(0), 5)), 0);
}

#[test]
fn ratios_logits_and_presences() {
    let got = real32(ln_ratio(3, 7));
    assert!((got - (3.0f64 / 7.0).ln()).abs() < 1e-8);
    let d = 1u64 << 51;
    assert!((real32(logit_draw(d)) - (0.25f64 / 0.75).ln()).abs() < 1e-8);
    assert!((real32(logit(q62(0.8))) - 4f64.ln()).abs() < 1e-8);
    assert_eq!(presence(INF, 5), ONE32 as u64);
    assert_eq!(presence(5, 0), ONE32 as u64);
    assert_eq!(presence(-1, 5), 0);
    assert_eq!(presence(ONE32, 2 * ONE32), (ONE32 / 2) as u64);
    assert_eq!(presence(3 * ONE32, 2 * ONE32), ONE32 as u64);
}

#[test]
fn time_on_the_grid() {
    assert_eq!(grid(1.37), (1.37 * CYCLE).round() / CYCLE);
    assert_eq!(ticks(grid(2.5)), 5 << 19);
    assert_eq!(grid(0.999_999_9), 1.0);
    assert_eq!(grid(f64::INFINITY), f64::INFINITY);
}

/// Host decision 8: a draw's lead past its decision's own threshold is
/// the real log-odds lead to within a few units of Q32's last place, and
/// unbounded where the element is certain; products floor.
#[test]
fn presences_lead_from_their_thresholds() {
    use presence::{over, times, under, ONE};
    let logit = |p: f64| (p / (1.0 - p)).ln();
    for k in 1..2_000u64 {
        let p = k as f64 / 2_001.0;
        let t = crate::lineage::threshold(p);
        let d = (k * 0x9e37_79b9_7f4a) % t.max(1);
        let u = d as f64 / (1u64 << 53) as f64;
        if d > 0 {
            let want = logit(t as f64 / (1u64 << 53) as f64) - logit(u);
            assert!((real32(under(t, d)) - want).abs() < 1e-7, "{p} {d}");
        }
        assert!(over(t, t) >= 0 && under(t, t - 1) >= 0);
    }
    assert_eq!(under(1 << 53, 5), INF);
    assert_eq!(under(5, 0), INF);
    assert_eq!(over(0, 5), INF);
    assert_eq!(times(ONE, ONE), ONE);
    assert_eq!(times(3, ONE / 2), 1);
}
