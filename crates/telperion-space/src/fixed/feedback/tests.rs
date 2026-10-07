//! The feedback contract on the CPU: Astra's counterexamples give their
//! exact integers, and each primitive stays within its stated error of the
//! real it stands for (CONTRACT-A.md 2.3's bounds). The GPU's conformance
//! probe holds the mirror to these functions word for word.
use super::angle::{self, acos, asin, atan2, sincos, HALF, QUARTER};
use super::edge::{ripe, settle, shade, tangent_ratio};
use super::lattice::{deposits, weights, Grid};
use super::power::{exp2_w, log2_w};
use super::shed::{balance, Bud};
use super::total::Total;
use super::vector::{isqrt, length, unit, unit_w};
use super::wide::{floor_shift, mul_shr, prod_shr, ratio, zero_shift};
use super::Fault;
use crate::fixed::{INF, ONE32, P_ONE};
use std::f64::consts::TAU;

const W: f64 = 18_446_744_073_709_551_616.0;

fn real(q: i64) -> f64 {
    q as f64 / ONE32 as f64
}

fn turns(t: u32) -> f64 {
    f64::from(t) / 4_294_967_296.0
}

/// Four Q32 ones multiply to raw 2^128, outside an i128: exact here.
#[test]
fn a_product_of_four_factors_is_exact() {
    assert_eq!(prod_shr([ONE32; 4], 96), Ok(ONE32));
    let big = i64::MAX;
    let want = (i128::from(big) * i128::from(big)) >> 64;
    assert_eq!(prod_shr([big, big, 1 << 32, 1 << 32], 128), Ok(want as i64));
    assert_eq!(prod_shr([big; 4], 96), Err(Fault::Range));
    assert_eq!(prod_shr([-ONE32, ONE32, ONE32, 3], 96), Ok(-3));
    // Floored: -1/2 is -1.
    assert_eq!(prod_shr([-1, 1, 1, 1], 1), Ok(-1));
}

/// A W over W quotient cannot shift its numerator left 32 inside an i128.
#[test]
fn a_w_quotient_is_exact() {
    assert_eq!(ratio(1 << 100, 1 << 99, 32), Ok(2 << 32));
    assert_eq!(ratio(-(1 << 120), 3 << 118, 32), Ok(-(4 << 32) / 3));
    assert_eq!(ratio(1, 0, 32), Err(Fault::Zero));
    assert_eq!(ratio(1 << 120, 1, 32), Err(Fault::Range));
    assert_eq!(mul_shr(1 << 100, 1 << 100, 128), Ok(1 << 72));
    assert_eq!(mul_shr(1 << 100, 1 << 100, 64), Err(Fault::Range));
    assert_eq!(mul_shr(i128::MAX, i128::MAX, 0), Err(Fault::Range));
    assert_eq!(mul_shr(-3, 1, 1), Ok(-2));
}

/// An arithmetic shift makes a tiny negative -1; the magnitude's shift
/// keeps it symmetric.
#[test]
fn signed_shifts() {
    assert_eq!(floor_shift(-1, 32), -1);
    assert_eq!(zero_shift(-1, 32), 0);
    assert_eq!(zero_shift(-(3 << 40), 40), -3);
    assert_eq!(zero_shift(i128::MIN, 0), i128::MIN);
}

/// Eight terms of 2^125 wrap to none; the sum refuses in every grouping,
/// and a sum whose signed partials overflow on the way is exact.
#[test]
fn a_sum_refuses_by_its_magnitudes_in_any_order() {
    let eight: Total = std::iter::repeat_n(1i128 << 125, 8).sum();
    assert_eq!(eight.get(), Err(Fault::Range));
    let terms = [1i128 << 126, -(1 << 126), 1 << 125, -(1 << 125), 5];
    let forward: Total = terms.iter().copied().sum();
    let mut chunked = Total::default();
    for c in terms.rchunks(2) {
        chunked.merge(c.iter().copied().sum());
    }
    assert_eq!(forward, chunked);
    assert_eq!(forward.get(), Err(Fault::Range));
    let small = [(1i128 << 100), -(1 << 101), 7, -3];
    let s: Total = small.iter().copied().sum();
    assert_eq!(s.get(), Ok((1 << 100) - (1 << 101) + 4));
}

/// 2 pi / 2^32 in Q32 is raw 6, a 4.5 % error; the conversion here is one
/// product by a 60-bit constant.
#[test]
fn radians_are_precise() {
    assert_eq!(
        angle::radians(ONE32),
        Ok((TAU * ONE32 as f64).floor() as i64)
    );
    for k in [1i64, 7, 1 << 20, 3 << 31, -(5 << 29), 1 << 59] {
        let got = real(angle::radians(k).unwrap());
        let want = TAU * real(k);
        assert!(
            (got - want).abs() <= want.abs() * 1e-15 + 1.0 / ONE32 as f64,
            "{k}"
        );
        let back = angle::turns(angle::radians(k).unwrap());
        assert!((back - k).abs() <= 1, "{k}");
    }
    assert_eq!(angle::radians(1 << 60), Err(Fault::Range));
}

#[test]
fn angles_wrap_only_where_read() {
    assert_eq!(angle::between(0, HALF), -(1 << 31));
    assert_eq!(angle::between(u32::MAX, 1), 2);
    assert_eq!(angle::toward(u32::MAX - 9, 10, ONE32 / 2), 0);
    // A rate times a rank keeps its whole turns until it is wrapped.
    let amount = prod_shr([3 * ONE32 + ONE32 / 4, 5 * ONE32, 1, 1], 32).unwrap();
    assert_eq!(angle::wrap(amount), QUARTER);
    assert_eq!(angle::half(-3), -2);
}

#[test]
fn sin_and_cos_are_within_a_unit() {
    let mut most = 0f64;
    for k in 0..200_000u64 {
        let t = (k.wrapping_mul(0x9e37_79b9_7f4a_7c15) >> 32) as u32;
        let (c, s) = sincos(t);
        let a = TAU * turns(t);
        most = most
            .max((real(c) - a.cos()).abs())
            .max((real(s) - a.sin()).abs());
    }
    println!("sin, cos: {} units", most * ONE32 as f64);
    assert!(most <= 1.001 / ONE32 as f64, "{}", most * ONE32 as f64);
    assert_eq!(sincos(0), (ONE32, 0));
    assert_eq!(sincos(QUARTER), (0, ONE32));
    assert_eq!(sincos(HALF), (-ONE32, 0));
}

#[test]
fn atan2_is_within_half_a_unit_and_exact_on_the_axes() {
    let mut most = 0f64;
    let mut rng = 0x1234_5678u64;
    for _ in 0..200_000 {
        rng = rng.wrapping_mul(6_364_136_223_846_793_005).wrapping_add(1);
        let (y, x) = (
            (rng as i64) >> (rng % 60),
            ((rng >> 7) as i64) >> ((rng >> 3) % 60),
        );
        if x == 0 && y == 0 {
            continue;
        }
        let want = (y as f64).atan2(x as f64) / TAU;
        let got = turns(atan2(y, x));
        let err = (got - want.rem_euclid(1.0)).abs();
        most = most.max(err.min(1.0 - err));
    }
    println!("atan2: {} units", most * 4_294_967_296.0);
    assert!(most <= 0.55 / 4_294_967_296.0, "{}", most * 4_294_967_296.0);
    assert_eq!(atan2(0, 0), 0);
    assert_eq!(atan2(0, -5), HALF);
    assert_eq!(atan2(7, 0), QUARTER);
    assert_eq!(atan2(-7, 0), 3 * QUARTER);
    assert_eq!(atan2(i64::MIN, i64::MIN), HALF + QUARTER / 2);
}

/// Scalar `asin(cap / reach)` at its ends, and `acos`.
#[test]
fn asin_and_acos_at_their_ends() {
    assert_eq!(asin(ONE32), QUARTER);
    assert_eq!(asin(2 * ONE32), QUARTER);
    assert_eq!(asin(0), 0);
    assert_eq!(asin(-ONE32), 3 * QUARTER);
    assert_eq!(acos(ONE32), 0);
    assert_eq!(acos(-ONE32), HALF);
    assert_eq!(acos(0), QUARTER);
    let got = turns(asin(ONE32 / 2));
    assert!((got - 1.0 / 12.0).abs() < 2e-9, "{got}");
}

/// Raw `(1, 1, 0)` has integer length 1; its unit is the diagonal.
#[test]
fn small_vectors_keep_their_direction() {
    let d = unit([1, 1, 0]).unwrap();
    let half_root = std::f64::consts::FRAC_1_SQRT_2;
    assert!((real(d[0]) - half_root).abs() < 2.0 / ONE32 as f64);
    assert_eq!(d[0], d[1]);
    assert_eq!(d[2], 0);
    assert_eq!(unit([0; 3]), None);
    assert_eq!(unit([-5, 0, 0]), Some([-ONE32, 0, 0]));
    assert_eq!(length([3 << 32, 4 << 32, 0]), 5 << 32);
    assert_eq!(isqrt(u128::MAX), u128::from(u64::MAX));
    assert_eq!(isqrt(15), 3);
    assert_eq!(isqrt(16), 4);
    for k in 0..10_000u128 {
        let n = k * k * 0x9e37_79b9 + k;
        let r = isqrt(n);
        assert!(r * r <= n && (r + 1) * (r + 1) > n);
    }
}

/// A W lever of raw `(1, 0, -1)` is the diagonal down, not `(0, 0, -1)`
/// as an arithmetic shift to Q32 would make it, and its length is kept.
#[test]
fn a_small_lever_is_normalised_from_w() {
    let (u, log) = unit_w([1, 0, -1]);
    assert_eq!(u[0], -u[2]);
    assert!((real(u[0]) - std::f64::consts::FRAC_1_SQRT_2).abs() < 2.0 / ONE32 as f64);
    assert!(
        (real(log) - (0.5 - 64.0)).abs() < 4.0 / ONE32 as f64,
        "{}",
        real(log)
    );
    assert_eq!(unit_w([0; 3]), ([0; 3], -INF));
    let (u, log) = unit_w([i128::MIN, 0, 0]);
    assert_eq!(u, [-ONE32, 0, 0]);
    assert!((real(log) - 63.0).abs() < 4.0 / ONE32 as f64);
}

#[test]
fn w_logs_and_powers() {
    for k in 1..2_000u32 {
        let n = (u128::from(k) * 0x9e37_79b9_7f4a_7c15) << (k % 60);
        let want = (n as f64 / W).log2();
        assert!((real(log2_w(n)) - want).abs() < 4.0 / ONE32 as f64, "{n}");
    }
    assert_eq!(log2_w(1 << 64), 0);
    assert_eq!(log2_w(1), -64 << 32);
    assert_eq!(log2_w(0), -INF);
    assert_eq!(exp2_w(0), Ok(1 << 64));
    // Below Q62's reach but inside W's: 2^-63 is raw 2.
    assert_eq!(exp2_w(-63 << 32), Ok(2));
    assert_eq!(exp2_w(-64 << 32), Ok(1));
    assert_eq!(exp2_w(-65 << 32), Ok(0));
    assert_eq!(exp2_w(62 << 32), Err(Fault::Range));
    for y in [-40.25, -3.5, -0.1, 0.3, 1.0, 17.75, 60.5] {
        let q = crate::fixed::q32(y);
        let (got, want) = (exp2_w(q).unwrap() as f64 / W, real(q).exp2());
        assert!(
            (got - want).abs() <= want * 1e-15 + 1.0 / W,
            "{y}: {got} {want}"
        );
    }
}

/// Seven floored weights and the rest; deposits that conserve area to the
/// last unit, Astra's raw area 1 with thirds included.
#[test]
fn weights_and_deposits_conserve() {
    let third = (1u64 << 31) / 3;
    for t in [[third; 3], [0; 3], [(1 << 31) - 1; 3], [5, 1 << 30, third]] {
        let w = weights(t);
        assert_eq!(w.iter().sum::<u64>(), P_ONE);
        for area in [1i64, 3, ONE32, (1 << 48) - 1] {
            let d = deposits(area, &w);
            assert_eq!(d.iter().sum::<u128>(), (area as u128) << 35);
        }
    }
    let one = Grid::spanning([[0i64; 3]].into_iter()).unwrap();
    assert_eq!(one.size, [2, 2, 2]);
    let none = Grid::spanning(std::iter::empty()).unwrap();
    assert_eq!((none.low, none.size), ([0; 3], [1; 3]));
    assert_eq!(
        Grid::spanning([[1 << 44, 0, 0]].into_iter()),
        Err(Fault::Range)
    );
    let wide = [[-(1 << 43); 3], [(1 << 43) - 1; 3]];
    assert_eq!(Grid::spanning(wide.into_iter()), Err(Fault::Range));
}

/// A field that is one everywhere samples one, inside the box and out.
#[test]
fn sampling_a_constant_field() {
    let grid = Grid::spanning([[-(3 << 31), 5, 7 << 30]].into_iter()).unwrap();
    let light = vec![ONE32; grid.nodes()];
    for at in [
        [-(3 << 31), 5, 7 << 30],
        [1 << 40, 0, 0],
        [-(5 << 30), 1, 2],
    ] {
        assert_eq!(grid.sample(&light, at), ONE32);
    }
}

#[test]
fn zero_and_endpoint_cases() {
    assert_eq!(ripe(3 << 32, 0), ONE32);
    assert_eq!(ripe(3 << 32, -ONE32), ONE32);
    assert_eq!(ripe(1 << 32, 4 << 32), ONE32 / 4);
    // Certain survival stays certain at no light; no response, no change.
    assert_eq!(shade(0, 0, ONE32), 0);
    assert_eq!(shade(ONE32 / 8, 0, ONE32), INF);
    assert_eq!(shade(ONE32 / 8, 0, 0), ONE32 / 8);
    assert_eq!(shade(ONE32 / 8, ONE32, ONE32), ONE32 / 8);
    assert_eq!(shade(ONE32, ONE32 / 2, ONE32), 2 * ONE32);
    // An upright lever stays upright; a level one settles.
    assert_eq!(settle(HALF, ONE32), HALF);
    assert_eq!(settle(0, ONE32), 0);
    assert_eq!(settle(QUARTER, 0), QUARTER);
    assert_eq!(tangent_ratio(HALF, QUARTER), INF);
    assert_eq!(tangent_ratio(0, QUARTER), -INF);
    let got = real(tangent_ratio(QUARTER + QUARTER / 2, QUARTER));
    let want = ((3.0 * TAU / 16.0).tan() / (TAU / 8.0).tan()).ln();
    assert!((got - want).abs() < 1e-8, "{got} {want}");
}

#[test]
fn d16_threshold_and_kept_share() {
    let bud = Bud {
        q: ONE32 / 4,
        c: ONE32,
        memory: 0,
        hazard: 2 * ONE32,
        tolerance: ONE32 / 2,
        draw: 1 << 52,
        window: ONE32,
        keep: ONE32 as u64,
    };
    let b = balance(&bud).unwrap();
    // now = -0.6, memory -0.3, hazard 1.6, p = 1 - e^-1.6.
    assert_eq!(b.memory, ((-(3 * ONE32 / 5) - 1) >> 1));
    let p = 1.0 - (-1.6f64).exp();
    assert!((b.threshold as f64 / (1u64 << 53) as f64 - p).abs() < 1e-8);
    assert!(b.shed);
    let kept = balance(&Bud {
        draw: (1 << 53) - 1,
        ..bud
    })
    .unwrap();
    assert!(!kept.shed && kept.kept > 0 && kept.kept <= ONE32 as u64);
    // No hazard: undecided, the bearer's share kept.
    let calm = balance(&Bud {
        hazard: 0,
        keep: 77,
        ..bud
    })
    .unwrap();
    assert_eq!((calm.threshold, calm.shed, calm.kept), (0, false, 77));
}

/// The slice: the same lattice, light and balances for any deposit order
/// and chunking; the crown shades and the balance both sheds and keeps.
#[test]
fn the_slice_is_the_same_in_any_order_and_chunking() {
    use super::slice::{crown, run};
    let light = crate::light::Light {
        extinction: 0.5,
        sky: 0.5,
    };
    let (leaves, reads) = crown(4_000, 500, 7);
    let n = leaves.len();
    let forward: Vec<usize> = (0..n).collect();
    let reversed: Vec<usize> = (0..n).rev().collect();
    let shuffled: Vec<usize> = (0..n).map(|k| (k * 2_654_435_761) % n).collect();
    let base = run(&light, &leaves, &reads, &forward, n).unwrap();
    for order in [&forward, &reversed, &shuffled] {
        for chunk in [1, 7, 1_000] {
            assert!(run(&light, &leaves, &reads, order, chunk).unwrap() == base);
        }
    }
    let shaded = base.light.iter().filter(|&&l| l < ONE32).count();
    let shed = base.reads.iter().filter(|r| r.1.shed).count();
    assert!(
        shaded > 0 && shed > 0 && shed < reads.len(),
        "{shaded} {shed}"
    );
}

/// The f64-seeded square root gives the bits of the exact reference
/// (Newton from above), and the unit vectors those of the reference root,
/// over every bit length, squares and their neighbours, and millions of
/// random cases.
#[test]
fn the_fast_root_is_the_exact_one() {
    fn newton(n: u128) -> u128 {
        if n == 0 {
            return 0;
        }
        let mut x = 1u128 << (128 - n.leading_zeros()).div_ceil(2);
        loop {
            let y = (x + n / x) >> 1;
            if y >= x {
                return x;
            }
            x = y;
        }
    }
    let mut z = 0x1234_5678_9abc_def0u64;
    let mut next = move || {
        z = z.wrapping_add(0x9e37_79b9_7f4a_7c15);
        let mut x = z;
        x = (x ^ (x >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        x = (x ^ (x >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        x ^ (x >> 31)
    };
    for k in 0..3_000_000u32 {
        let n = ((u128::from(next()) << 64) | u128::from(next())) >> (k % 128);
        assert_eq!(isqrt(n), newton(n), "{n}");
    }
    for r in (0..64).flat_map(|b| [1u128 << b, (1u128 << b) - 1, (1u128 << b) + 1]) {
        let sq = r * r;
        for n in [sq.saturating_sub(1), sq, sq + 1, sq + 2 * r, sq.saturating_add(2 * r + 1)] {
            assert_eq!(isqrt(n), newton(n), "{n}");
        }
    }
    assert_eq!(isqrt(u128::MAX), newton(u128::MAX));
    for _ in 0..3_000_000 {
        let v = [0; 3].map(|_: i64| (next() as i64) >> (next() % 63));
        let Some(u) = unit(v) else { continue };
        // The reference: the same scale and root, an i128 division.
        let most = v.iter().map(|c| c.unsigned_abs()).max().unwrap();
        let shift = 61 - (63 - most.leading_zeros() as i32);
        let c = v.map(|c| {
            let m = u128::from(c.unsigned_abs());
            let m = if shift >= 0 { m << shift } else { m >> -shift } as i64;
            if c < 0 { -m } else { m }
        });
        let sum: u128 = c.iter().map(|&x| u128::from(x.unsigned_abs()).pow(2)).sum();
        let root = newton(sum) as i128;
        let want = c.map(|x| ((i128::from(x) << 32) / root) as i64);
        assert_eq!(u, want, "{v:?}");
    }
}
