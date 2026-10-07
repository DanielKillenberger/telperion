//! The feedback probe's cases (fn-211 task 8): random arguments at and
//! about each selector's domain, Astra's counterexamples first.
use telperion_space::fixed::feedback::angle;
use telperion_space::fixed::ONE32;

/// Random cases a selector, after its counterexamples.
const CASES: usize = 1 << 14;

/// SplitMix64, for cases.
pub(super) struct Rng(pub(super) u64);

impl Rng {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9e37_79b9_7f4a_7c15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        z ^ (z >> 31)
    }
    /// Any i64, its magnitude's bits drawn too, with the extremes often.
    fn any(&mut self) -> i64 {
        match self.next() % 16 {
            0 => 0,
            1 => i64::MAX,
            2 => i64::MIN,
            3 => -1,
            _ => (self.next() as i64) >> (self.next() % 64),
        }
    }
    fn within(&mut self, lo: i64, hi: i64) -> i64 {
        let span = (hi as i128 - lo as i128 + 1) as u128;
        match self.next() % 8 {
            0 => lo,
            1 => hi,
            _ => (lo as i128 + (u128::from(self.next()) % span) as i128) as i64,
        }
    }
    /// A random Q32 unit vector (`vector::unit` of a random one).
    fn unit_vector(&mut self) -> [i64; 3] {
        loop {
            let v = [0; 3].map(|_: i64| (self.next() as i64) >> 20);
            if let Some(u) = telperion_space::fixed::feedback::vector::unit(v) {
                return u;
            }
        }
    }
    fn wide(&mut self) -> i128 {
        (i128::from(self.any()) << 64 | i128::from(self.next())) >> (self.next() % 128)
    }
}

pub(super) fn split(x: i128) -> [u64; 2] {
    [x as u64, (x >> 64) as u64]
}

pub(super) fn join(lo: u64, hi: u64) -> i128 {
    (i128::from(hi as i64) << 64) | i128::from(lo)
}

/// Astra's counterexamples and the contract's edges, a selector's first
/// cases.
fn edges(s: u32) -> Vec<[u64; 8]> {
    let q = ONE32 as u64;
    let w = |x: i128| split(x);
    let i128s = |a: i128, b: i128, n: u64| {
        let (a, b) = (w(a), w(b));
        [a[0], a[1], b[0], b[1], n, 0, 0, 0]
    };
    match s {
        0 => vec![
            i128s(1 << 100, 1 << 100, 128),
            i128s(1 << 100, 1 << 100, 64),
        ],
        1 => vec![[q, q, q, q, 96, 0, 0, 0], [u64::MAX, 1, 1, 1, 1, 0, 0, 0]],
        2 => vec![
            i128s(1 << 100, 1 << 99, 32),
            i128s(1, 0, 32),
            i128s(1 << 120, 1, 32),
        ],
        3 => vec![[u64::MAX, u64::MAX, 32, 0, 0, 0, 0, 0]],
        4 => vec![
            [q, 1, 0, 1 << 31, q / 2, 0, 0, 0],
            [1, 1 << 60, 7, 3, 1, 0, 0, 0],
        ],
        5 => vec![
            [0, 0, 0, 0, 0, 0, 0, 0],
            [1 << 30, 0, (-5i64) as u64, 0, 0, 0, 0, 0],
        ],
        6 => vec![[q, (-(ONE32)) as u64, u64::MAX, u64::MAX, 3, 4, 0, 0]],
        7 => vec![[1, 1, 0, 0, 0, 0, 0, 0], [0; 8]],
        8 => vec![[1, 0, 0, 0, u64::MAX, u64::MAX, 0, 0], [0; 8]],
        9 => vec![
            [0, 0, (-63i64 << 32) as u64, 0, 0, 0, 0, 0],
            [1, 0, 61 << 32, 0, 0, 0, 0, 0],
        ],
        11 => vec![[(1 << 31) / 3, (1 << 31) / 3, (1 << 31) / 3, 1, 0, 0, 0, 0]],
        12 => vec![[q / 4, q, 0, 2 * q, q / 2, 1 << 52, q, q]],
        13 => vec![
            [
                3 << 32,
                0,
                0,
                0,
                q,
                u64::from(angle::HALF) | u64::from(angle::QUARTER) << 32,
                q,
                0,
            ],
            [3 << 32, (-1i64) as u64, q / 8, 0, 0, 0, 0, 0],
        ],
        _ => vec![],
    }
}

/// Selector `s`'s cases, and the terms the sums read.
pub(super) fn cases(s: u32, rng: &mut Rng, terms: &mut Vec<u64>) -> Vec<[u64; 8]> {
    let q = ONE32;
    let mut out = edges(s);
    if s == 10 {
        // Eight terms of 2^125, which wrap to none.
        out.push([terms.len() as u64 / 2, 8, 3, 0, 0, 0, 0, 0]);
        for _ in 0..8 {
            terms.extend(split(1 << 125));
        }
    }
    for _ in 0..CASES {
        let mut a = [0u64; 8];
        let put = |a: &mut [u64; 8], k: usize, v: i128| a[k..k + 2].copy_from_slice(&split(v));
        match s {
            0 | 2 => {
                put(&mut a, 0, rng.wide());
                put(&mut a, 2, rng.wide());
                a[4] = rng.next() % if s == 0 { 256 } else { 127 };
            }
            1 => {
                for v in &mut a[0..4] {
                    *v = rng.any() as u64;
                }
                a[4] = rng.next() % 256;
            }
            3 => {
                put(&mut a, 0, rng.wide());
                a[2] = rng.next() % 128;
            }
            4 => {
                a[0] = rng.any() as u64;
                a[1] = rng.any() as u64;
                a[2] = rng.next() as u32 as u64;
                a[3] = rng.next() as u32 as u64;
                a[4] = rng.any() as u64;
            }
            5 => {
                a[0] = rng.next() as u32 as u64;
                a[1] = rng.any() as u64;
                a[2] = rng.any() as u64;
            }
            6 => {
                a[0] = rng.within(-2 * q, 2 * q) as u64;
                a[1] = rng.within(-2 * q, 2 * q) as u64;
                put(&mut a, 2, rng.wide());
                for v in &mut a[4..7] {
                    *v = rng.any() as u64;
                }
            }
            7 => {
                for v in &mut a[0..3] {
                    *v = rng.any() as u64;
                }
            }
            8 => {
                for k in [0, 2, 4] {
                    put(&mut a, k, rng.wide());
                }
            }
            9 => {
                put(&mut a, 0, rng.wide());
                a[2] = rng.within(-100 * q, 70 * q) as u64;
            }
            10 => {
                let len = 1 + rng.next() % 40;
                a[0] = terms.len() as u64 / 2;
                a[1] = len;
                a[2] = 1 + rng.next() % 9;
                let top = 64 + rng.next() % 64;
                for _ in 0..len {
                    terms.extend(split(rng.wide() >> (127 - top.min(127))));
                }
            }
            11 => {
                for v in &mut a[0..3] {
                    *v = rng.within(0, (1 << 31) - 1) as u64;
                }
                a[3] = rng.within(0, (1 << 48) - 1) as u64;
            }
            12 => {
                a[0] = rng.within(-2, 1 << 40) as u64;
                a[1] = rng.within(-2, 1 << 40) as u64;
                a[2] = rng.within(-q, q) as u64;
                a[3] = rng.within(-q, 8 * q) as u64;
                a[4] = rng.within(-q, q) as u64;
                a[5] = rng.within(0, (1 << 53) - 1) as u64;
                a[6] = rng.within(-(1 << 10), 1 << 40) as u64;
                a[7] = rng.within(0, q) as u64;
            }
            14 => {
                let v = rng.unit_vector();
                let scale = rng.within(1, 1 << 12);
                for k in 0..3 {
                    a[k] = v[k].wrapping_mul(scale) as u64;
                }
                let p = rng.unit_vector();
                a[3..6].copy_from_slice(&p.map(|c| c as u64));
                let (c, sn) = angle::sincos(rng.next() as u32);
                a[6] = c as u64;
                a[7] = sn as u64;
            }
            15 => {
                let from = rng.unit_vector();
                let to = match rng.next() % 6 {
                    0 => from,
                    1 => from.map(|c| -c),
                    2 => {
                        let mut t = from;
                        t[(rng.next() % 3) as usize] += (rng.next() % 9) as i64 - 4;
                        t
                    }
                    _ => rng.unit_vector(),
                };
                a[..3].copy_from_slice(&from.map(|c| c as u64));
                a[3..6].copy_from_slice(&to.map(|c| c as u64));
            }
            _ => {
                a[0] = rng.within(0, 1 << 40) as u64;
                a[1] = rng.within(-(1 << 34), 1 << 40) as u64;
                a[2] = match rng.next() % 6 {
                    0 => telperion_space::fixed::INF,
                    _ => rng.within(-q, 60 * q),
                } as u64;
                a[3] = rng.within(-2, 1 << 33) as u64;
                a[4] = rng.within(-(1 << 30), 4 * q) as u64;
                let t = |r: &mut Rng| r.within(0, 1 << 31) as u64;
                a[5] = t(rng) | t(rng) << 32;
                a[6] = rng.within(-q, 70 * q) as u64;
            }
        }
        out.push(a);
    }
    out
}
