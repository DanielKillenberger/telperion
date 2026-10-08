//! The contract's quotient on the GPU (fn-211 task 11): `xquotient` by
//! Knuth's algorithm D in 16-bit digits, against the CPU's bit loop
//! (`I256::quotient`) over a large randomised set and the boundaries. A
//! model of the WGSL's steps counts the paths the cases take, so the set
//! is shown to reach each one, the rare add-back among them.
use super::super::io;
use super::feedback_cases::{split, Rng};
use super::feedback_tests::{expected, run, source};
use telperion_space::fixed::feedback::wide::I256;

/// The probe's selector for the quotient.
pub(super) const QUOTIENT: u32 = 16;

/// A case: the numerator's four words from the lowest, the divisor's two.
fn case(n: [u64; 4], d: i128) -> [u64; 8] {
    let d = split(d);
    [n[0], n[1], n[2], n[3], d[0], d[1], 0, 0]
}

/// The numerator as words of an i256 of value `x`.
fn of(x: i128) -> [u64; 4] {
    let (lo, hi) = (split(x), split(x >> 127));
    [lo[0], lo[1], hi[0], hi[1]]
}

/// The boundaries: none, one, the extremes of both, a numerator equal to
/// or one off its divisor, and the divisor's sign flipped against each.
pub(super) fn edges() -> Vec<[u64; 8]> {
    let min256 = [0, 0, 0, 1 << 63];
    let max256 = [u64::MAX, u64::MAX, u64::MAX, u64::MAX >> 1];
    let all = [u64::MAX; 4];
    let ds = [
        0,
        1,
        -1,
        2,
        i128::MAX,
        i128::MIN,
        i128::MIN + 1,
        1 << 64,
        (1 << 64) - 1,
        0xffff,
        0x1_0000,
        0x8000,
        0x7fff_ffff_ffff_ffff_ffff,
    ];
    let mut out = Vec::new();
    for d in ds {
        for n in [
            min256,
            max256,
            all,
            [0; 4],
            of(1),
            of(-1),
            of(d),
            of(d.wrapping_sub(1)),
        ] {
            out.push(case(n, d));
            out.push(case(n, d.wrapping_neg()));
        }
    }
    out
}

/// A 16-bit digit, the edges of a digit often.
fn digit(rng: &mut Rng) -> u64 {
    match rng.next() % 8 {
        0 => 0,
        1 => 0xffff,
        2 => 0x8000,
        3 => 0x7fff,
        4 => 1,
        _ => rng.next() & 0xffff,
    }
}

/// A random magnitude of up to `max` digits, its length drawn, negated
/// half the time; as 16 digits from the lowest.
fn digits(rng: &mut Rng, max: u64) -> ([u64; 16], bool) {
    let len = 1 + rng.next() % max;
    let mut d = [0u64; 16];
    for v in d.iter_mut().take(len as usize) {
        *v = digit(rng);
    }
    (d, rng.next() % 2 == 0)
}

/// A random case: a numerator of up to 16 digits over a divisor of up to
/// 8; a sixth of them a divisor times a quotient plus a remainder near it.
pub(super) fn random(rng: &mut Rng) -> [u64; 8] {
    let (vd, vneg) = digits(rng, 8);
    let mut v: i128 = 0;
    for k in (0..8).rev() {
        v = (v << 16) | vd[k] as i128;
    }
    let v = if vneg { v.wrapping_neg() } else { v };
    let (ud, uneg) = digits(rng, 16);
    let mut n = [0u64; 4];
    for (k, d) in ud.iter().enumerate() {
        n[k / 4] |= d << (16 * (k % 4));
    }
    if rng.next() % 6 == 0 {
        // A product of the divisor's magnitude and a small quotient, less
        // one: the remainder one short of the divisor.
        let q = (rng.next() >> 40) as i128;
        let p = I256::mul(v.unsigned_abs() as i128 & i128::MAX, q);
        let p = p.add(I256::of(-1));
        n = [
            p.lo as u64,
            (p.lo >> 64) as u64,
            p.hi as u64,
            (p.hi >> 64) as u64,
        ];
    }
    if uneg {
        let m = negate(words(n));
        n = [
            m.lo as u64,
            (m.lo >> 64) as u64,
            m.hi as u64,
            (m.hi >> 64) as u64,
        ];
    }
    case(n, v)
}

/// Four words from the lowest as an i256.
fn words(n: [u64; 4]) -> I256 {
    I256 {
        hi: (n[2] as u128 | (n[3] as u128) << 64) as i128,
        lo: n[0] as u128 | (n[1] as u128) << 64,
    }
}

fn negate(x: I256) -> I256 {
    let lo = (!x.lo).wrapping_add(1);
    I256 {
        hi: (!x.hi).wrapping_add(i128::from(lo == 0)),
        lo,
    }
}

/// The paths a set of cases takes through the WGSL's steps.
#[derive(Debug, Default)]
struct Paths {
    by_none: u64,
    under: u64,
    short: u64,
    long: u64,
    unshifted: u64,
    corrected: [u64; 3],
    added_back: u64,
}

/// The WGSL's `xquotient`, step for step, counting its paths; its
/// quotient's magnitude as 16 digits.
fn model(c: &[u64; 8], paths: &mut Paths) -> [u64; 16] {
    let mut q = [0u64; 16];
    let d = (c[4] as u128 | (c[5] as u128) << 64) as i128;
    if d == 0 {
        paths.by_none += 1;
        return q;
    }
    let x = words([c[0], c[1], c[2], c[3]]);
    let m = if x.negative() { negate(x) } else { x };
    let mw = [
        m.lo as u64,
        (m.lo >> 64) as u64,
        m.hi as u64,
        (m.hi >> 64) as u64,
    ];
    let den = d.unsigned_abs();
    let mut u = [0i64; 17];
    let mut v = [0i64; 8];
    for k in 0..16 {
        u[k] = ((mw[k / 4] >> (16 * (k % 4))) & 0xffff) as i64;
    }
    for (k, x) in v.iter_mut().enumerate() {
        *x = ((den >> (16 * k)) & 0xffff) as i64;
    }
    let n = 8 - v.iter().rev().take_while(|&&x| x == 0).count();
    let len = 16 - u[..16].iter().rev().take_while(|&&x| x == 0).count();
    if len < n {
        paths.under += 1;
        return q;
    }
    if n == 1 {
        paths.short += 1;
        let mut r = 0;
        for j in (0..len).rev() {
            let top = (r << 16) | u[j];
            q[j] = (top / v[0]) as u64;
            r = top - q[j] as i64 * v[0];
        }
        return q;
    }
    paths.long += 1;
    let s = (v[n - 1] as u16).leading_zeros();
    paths.unshifted += u64::from(s == 0);
    for i in (1..n).rev() {
        v[i] = ((v[i] << s) | (v[i - 1] >> (16 - s))) & 0xffff;
    }
    v[0] = (v[0] << s) & 0xffff;
    u[len] = u[len - 1] >> (16 - s);
    for i in (1..len).rev() {
        u[i] = ((u[i] << s) | (u[i - 1] >> (16 - s))) & 0xffff;
    }
    u[0] = (u[0] << s) & 0xffff;
    let (top, next) = (v[n - 1], v[n - 2]);
    for j in (0..=len - n).rev() {
        let head = (u[j + n] << 16) | u[j + n - 1];
        let (mut qhat, mut rhat) = (head / top, head % top);
        let mut steps = 0;
        while qhat >= 0x10000 || qhat * next > ((rhat << 16) | u[j + n - 2]) {
            qhat -= 1;
            rhat += top;
            steps += 1;
            if rhat >= 0x10000 {
                break;
            }
        }
        paths.corrected[steps.min(2)] += 1;
        let mut k = 0;
        for i in 0..n {
            let p = qhat * v[i];
            let t = u[i + j] - k - (p & 0xffff);
            u[i + j] = t & 0xffff;
            k = (p >> 16) - (t >> 16);
        }
        let t = u[j + n] - k;
        u[j + n] = t & 0xffff;
        q[j] = qhat as u64;
        if t < 0 {
            paths.added_back += 1;
            q[j] -= 1;
            let mut c = 0;
            for i in 0..n {
                let sum = u[i + j] + v[i] + c;
                u[i + j] = sum & 0xffff;
                c = sum >> 16;
            }
            u[j + n] = (u[j + n] + c) & 0xffff;
        }
    }
    q
}

/// The model's quotient is the CPU's: its magnitude, before the sign.
fn agrees(c: &[u64; 8], q: &[u64; 16]) -> bool {
    let want = expected(QUOTIENT, c, &[]);
    if want[4] != 0 {
        return true;
    }
    let mut got = [0u64; 4];
    for (k, d) in q.iter().enumerate() {
        got[k / 4] |= d << (16 * (k % 4));
    }
    let x = words([want[0], want[1], want[2], want[3]]);
    let m = if x.negative() { negate(x) } else { x };
    got == [
        m.lo as u64,
        (m.lo >> 64) as u64,
        m.hi as u64,
        (m.hi >> 64) as u64,
    ]
}

/// Cases a GPU run, so the results buffer stays under 64 MB.
const BATCH: usize = 1 << 18;

#[test]
fn the_gpus_quotient_is_the_cpus_over_four_million_cases() {
    let mut rng = Rng(0xd1_7155);
    let mut cases = edges();
    cases.extend((0..(1 << 22)).map(|_| random(&mut rng)));
    let mut paths = Paths::default();
    for c in &cases {
        let q = model(c, &mut paths);
        assert!(agrees(c, &q), "the model's steps: {c:x?}");
    }
    println!("{} cases: {paths:?}", cases.len());
    assert!(paths.by_none > 0 && paths.under > 0 && paths.short > 0 && paths.long > 0);
    assert!(paths.unshifted > 0 && paths.corrected.iter().all(|&c| c > 0));
    assert!(paths.added_back >= 100, "the add-back is reached");
    let Some(gpu) = crate::generation::test_gpu() else {
        return;
    };
    let pass = io::Pass::new(&gpu, source(), &[true, true, false], &["main"]);
    let mut wrong = 0;
    for chunk in cases.chunks(BATCH) {
        let got = run(&gpu, &pass, QUOTIENT, chunk, &[0, 0]);
        for (c, g) in chunk.iter().zip(&got) {
            if expected(QUOTIENT, c, &[]) != *g {
                if wrong < 3 {
                    eprintln!("{c:x?}\n  gpu {g:x?}");
                }
                wrong += 1;
            }
        }
    }
    assert_eq!(wrong, 0, "quotients differ");
}
