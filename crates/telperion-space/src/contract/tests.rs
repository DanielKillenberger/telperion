//! The contract's discrete promises (fn-254 R4, check 2): a draw's test
//! is the real comparison to the bit, its endpoints are taken by name,
//! and the f32 arithmetic is its f64 form's to its last places.
use super::decide::{lived_fine, lived_ticks, relay_node_f};
use super::*;

/// SplitMix64, for cases.
fn cases(mut z: u64) -> impl FnMut() -> u64 {
    move || {
        z = z.wrapping_add(0x9e37_79b9_7f4a_7c15);
        let mut x = z;
        x = (x ^ (x >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        x = (x ^ (x >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        x ^ (x >> 31)
    }
}

#[test]
fn a_draw_falls_under_an_f32_chance_exactly_as_its_uniform_does() {
    let mut next = cases(1);
    for _ in 0..1_000_000 {
        let p = f32::from_bits((next() as u32) & 0x3f7f_ffff);
        let d = next() >> 11;
        let real = (d as f64 / ONE as f64) < f64::from(p);
        assert_eq!(d < threshold(p), real, "p {p:e}, d {d}");
        assert_eq!(floor53(p), (f64::from(p) * ONE as f64).floor() as u64);
    }
    for (p, t) in [(0.0, 0), (-1.0, 0), (f32::NAN, 0), (1.0, ONE), (2.0, ONE)] {
        assert_eq!(threshold(p), t, "{p}");
    }
    assert_eq!(
        threshold(f32::from_bits(1)),
        1,
        "the least chance passes draw 0"
    );
}

#[test]
fn the_endpoints_are_taken_by_name() {
    assert_eq!(
        (neg_log2(0.0), neg_log2(1.0), neg_log2(f32::NAN)),
        (INF, 0.0, INF)
    );
    assert_eq!((prob(INF), prob(0.0), prob(-1.0)), (0.0, 1.0, 1.0));
    assert_eq!((fails(0.0), fails(INF)), (0.0, 1.0));
    assert_eq!(scaled(INF, 0), INF, "unbounded at any share");
    assert_eq!(
        (presence(INF, 0.1), presence(1.0, 0.0), presence(-1.0, 0.1)),
        (1.0, 1.0, 0.0)
    );
    assert_eq!(
        (logit(0.0), logit(1.0), logit53(0), logit53(ONE)),
        (-INF, INF, -INF, INF)
    );
    assert_eq!(decide::and(INF, 3.0), INF);
    // A unit with no presence stands its relay by count, rounded up.
    assert_eq!(relay_node_f(&[0.0; 4], 2, 6, 0.5), 3);
}

#[test]
fn the_contracts_log_and_power_are_their_reals() {
    let mut next = cases(2);
    let (mut log, mut power, mut fail) = (0f64, 0f64, 0f64);
    for _ in 0..1_000_000 {
        let x = f32::from_bits((next() as u32) & 0x7f7f_ffff).max(f32::MIN_POSITIVE);
        let want = f64::from(x).log2();
        log = log.max((f64::from(log2(x)) - want).abs() / want.abs().max(1.0));
        let y = (next() >> 40) as f32 / (1u64 << 24) as f32 * 250.0 - 125.0;
        let want = f64::from(y).exp2();
        power = power.max((f64::from(exp2(y)) - want).abs() / want);
        let l = y.abs() / 1000.0;
        let want = -(-f64::from(l) * std::f64::consts::LN_2).exp_m1();
        fail = fail.max((f64::from(fails(l)) - want).abs() / want.max(f64::MIN_POSITIVE));
    }
    // Relative, a few units in f32's last place (2^-24 is 6e-8).
    assert!(
        log < 4e-7 && power < 4e-7 && fail < 4e-7,
        "{log:e} {power:e} {fail:e}"
    );
}

/// A small chance of surviving keeps its bits: 2^-52 a cycle over half a
/// cycle is 2^-26, whose threshold is 2^27 (fn-254 review); and the two
/// forms meet at a half.
#[test]
fn a_small_survival_keeps_its_threshold() {
    use super::decide::survival_fine;
    assert_eq!(survival_fine(52.0, 1 << 31), 1 << 27);
    assert_eq!(survival_fine(1.0, 1 << 32), 1 << 52);
    assert_eq!(survival_fine(2.0, 1 << 31), 1 << 52);
}

#[test]
fn a_lived_time_rounds_up_to_a_tick_past_its_fine_part() {
    let mut next = cases(3);
    for _ in 0..100_000 {
        let l = (next() >> 40) as f32 / (1u64 << 24) as f32 * 8.0;
        let (share, d) = (next() % (1 << 20) + 1, next() >> 11);
        let (whole, fine) = (lived_ticks(l, share, d), lived_fine(l, share, d));
        assert!(whole << 12 >= fine && (whole << 12) - fine < 1 << 12 || whole == share);
        assert!(whole <= share);
    }
}

/// Chances near certainty keep their distance from it (fn-254 review): a
/// draw a hair under one lives half its cycle, two near-certain chances
/// conditioned on each other shed half the time, and a small uniform
/// waits its own long time.
#[test]
fn near_certain_chances_keep_their_distance() {
    use super::decide::{kept, lived_ticks, wake_ticks};
    let cycle = 1u64 << 20;
    let l = neg_log2_of(0.999_999_99);
    let lived = lived_ticks(l, cycle, 9_007_199_209_704_996);
    assert!(
        (cycle * 2 / 5..=cycle * 3 / 5).contains(&lived),
        "{lived} of {cycle}"
    );
    let (dead, context) = (neg_log2_of(1.0 - 1e-8), Some(neg_log2_of(1.0 - 1e-8)));
    assert!(
        !kept(dead, context, ONE / 4),
        "half shed: under a quarter is shed"
    );
    assert!(
        kept(dead, context, ONE * 3 / 4),
        "half shed: over three quarters is kept"
    );
    let wait = wake_ticks(0, 5e8, 45_035_996).unwrap() as f64 / cycle as f64;
    assert!((wait - 2.5).abs() < 0.01, "{wait} years");
    // The other end: a draw a place under one waits its finite 36.74 nats.
    let wait = wake_ticks(0, 1.0, ONE - 1).unwrap() as f64 / cycle as f64;
    assert!((wait - 36.7368).abs() < 0.01, "{wait} years");
    // Unconditioned, a near-certain death keeps its chance of living.
    let dead = neg_log2_of(1.0 - 1e-8);
    assert!(
        kept(dead, None, ONE - 45_035_996),
        "a draw past the death's chance keeps"
    );
    assert!(!kept(dead, None, ONE - 135_107_988), "one under it sheds");
}
