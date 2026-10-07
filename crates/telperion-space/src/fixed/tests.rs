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

/// The squaring and root procedures the polynomials replaced (host
/// decision, 2026-10-07), kept to hold the polynomials' error bounds to
/// theirs.
mod replaced {
    pub fn log2_frac(mut m: u64) -> u64 {
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
        frac
    }

    /// `2^-f` of a Q32 fraction, Q62, by the 32 roots `2^-(2^-(i + 1))`,
    /// computed here in f64-free integer form from their literals.
    pub fn exp2_frac(f: u64) -> u64 {
        const ROOTS: [u64; 32] = [
            0x2d41_3ccc_fe77_9921, 0x35d1_3f32_b5a7_5abd, 0x3ab0_31b9_f749_0e4b,
            0x3d49_5f45_4921_b30b, 0x3ea0_ecb6_dc8a_80ce, 0x3f4f_8303_3d21_b05d,
            0x3fa7_8457_1ee3_e212, 0x3fd3_b2d6_5447_b229, 0x3fe9_d594_8723_6bb4,
            0x3ff4_e9d4_703d_f842, 0x3ffa_74ac_bdf6_c9d5, 0x3ffd_3a46_ffc6_e30f,
            0x3ffe_9d1f_a801_0199, 0x3fff_4e8e_de05_3ad4, 0x3fff_a747_3183_76ac,
            0x3fff_d3a3_8961_e6fe, 0x3fff_e9d1_c0d8_fd14, 0x3fff_f4e8_df76_80c4,
            0x3fff_fa74_6f7d_c0eb, 0x3fff_fd3a_37af_8097, 0x3fff_fe9d_1bd3_e854,
            0x3fff_ff4e_8de8_fe2c, 0x3fff_ffa7_46f4_4196, 0x3fff_ffd3_a37a_116b,
            0x3fff_ffe9_d1bd_04dd, 0x3fff_fff4_e8de_8178, 0x3fff_fffa_746f_407e,
            0x3fff_fffd_3a37_a030, 0x3fff_fffe_9d1b_d014, 0x3fff_ffff_4e8d_e809,
            0x3fff_ffff_a746_f404, 0x3fff_ffff_d3a3_7a02,
        ];
        let mut r = 1u64 << 62;
        for (i, &root) in ROOTS.iter().enumerate() {
            if f & (1 << (31 - i)) != 0 {
                r = ((u128::from(r) * u128::from(root)) >> 62) as u64;
            }
        }
        r
    }
}

/// The polynomials' error against the reals is no looser than the
/// procedures' they replaced: log2's in Q32 units over mantissas, 2^-f's
/// relative over fractions (f64 references, whose own error is far
/// below both).
#[test]
fn the_polynomials_are_no_looser_than_the_procedures() {
    let mut z = 0xfeed_beefu64;
    let mut next = move || {
        z = z.wrapping_add(0x9e37_79b9_7f4a_7c15);
        let mut x = z;
        x = (x ^ (x >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        x = (x ^ (x >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        x ^ (x >> 31)
    };
    let (mut new_log, mut old_log, mut new_exp, mut old_exp) = (0f64, 0f64, 0f64, 0f64);
    let one = (1u64 << 62) as f64;
    for k in 0..2_000_000u64 {
        let m = (1u64 << 62) | (next() >> 2);
        let m = if k % 64 == 0 { (1 << 62) + (k / 64 % 64 << 56) } else { m };
        let want = (m as f64 / one).log2() * 4_294_967_296.0;
        new_log = new_log.max((log2_frac(m) as f64 - want).abs());
        old_log = old_log.max((replaced::log2_frac(m) as f64 - want).abs());
        let f = next() >> 32;
        let want = (-(f as f64) / 4_294_967_296.0).exp2();
        let rel = |r: u64| (r as f64 / one - want).abs() / want;
        new_exp = new_exp.max(rel(exp2_neg((1 << 32) | f as i64) << 1));
        old_exp = old_exp.max(rel(replaced::exp2_frac(f)));
    }
    println!("log2: {new_log:.3} units (squarings {old_log:.3}); 2^-f: {new_exp:e} (roots {old_exp:e})");
    assert!(new_log <= old_log.max(1.0), "log2 {new_log} against {old_log}");
    assert!(new_exp <= old_exp.max(1e-17), "2^-f {new_exp} against {old_exp}");
}
