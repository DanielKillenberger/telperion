//! The feedback contract's conformance probe (fn-211 task 8; Astra's
//! review of CONTRACT-A.md, section 4): every function of
//! `telperion_space::fixed::feedback` on the GPU and the CPU, over random
//! cases at and about their domains' edges with Astra's counterexamples
//! first, gives the same words, error statuses included.
use super::super::io;
use crate::Gpu;
use telperion_space::fixed::feedback::{
    angle, edge, lattice, power, shed, status, total::Total, vector, wide,
};
use telperion_space::fixed::{ONE32, P_ONE};

/// Random cases a selector, after its counterexamples.
const CASES: usize = 1 << 14;
/// Results a case.
const SLOTS: usize = 24;
const PROBE: &str = include_str!("feedback_probe.wgsl");

const NAMES: [&str; 14] = [
    "mul_shr",
    "prod_shr",
    "ratio",
    "floor_shift, zero_shift",
    "radians, turns, between, toward, half",
    "sincos, atan2",
    "asin, acos, isqrt, length",
    "unit",
    "unit_w",
    "log2_w, exp2_w",
    "Total",
    "weights, deposits",
    "balance (D16)",
    "ripe, shade, settle, tangent_ratio",
];

/// The probe's WGSL: the decision contract, the feedback contract, the
/// probe.
pub(super) fn source() -> String {
    format!("{}\n{PROBE}", super::feedback())
}

/// SplitMix64, for cases.
struct Rng(u64);

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
    fn wide(&mut self) -> i128 {
        (i128::from(self.any()) << 64 | i128::from(self.next())) >> (self.next() % 128)
    }
}

fn split(x: i128) -> [u64; 2] {
    [x as u64, (x >> 64) as u64]
}

fn join(lo: u64, hi: u64) -> i128 {
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
fn cases(s: u32, rng: &mut Rng, terms: &mut Vec<u64>) -> Vec<[u64; 8]> {
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
                for k in 0..4 {
                    a[k] = rng.any() as u64;
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
                for k in 4..7 {
                    a[k] = rng.any() as u64;
                }
            }
            7 => {
                for k in 0..3 {
                    a[k] = rng.any() as u64;
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
                for k in 0..3 {
                    a[k] = rng.within(0, (1 << 31) - 1) as u64;
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

/// The CPU's results for one case.
fn expected(s: u32, a: &[u64; 8], terms: &[u64]) -> [u64; SLOTS] {
    let mut r = [0u64; SLOTS];
    let i = |k: usize| a[k] as i64;
    let x = |k: usize| join(a[k], a[k + 1]);
    let mut wide = |k: usize, v: Result<i128, _>| {
        if let Ok(v) = v {
            r[k..k + 2].copy_from_slice(&split(v));
        }
        r[k + 2] = status(&v);
    };
    match s {
        0 => wide(0, wide::mul_shr(x(0), x(2), a[4] as u32)),
        1 => {
            let v = wide::prod_shr([i(0), i(1), i(2), i(3)], a[4] as u32);
            r[0] = v.map_or(0, |v| v as u64);
            r[1] = status(&v);
        }
        2 => wide(0, wide::ratio(x(0), x(2), a[4] as u32)),
        3 => {
            let n = a[2] as u32;
            r[..2].copy_from_slice(&split(wide::floor_shift(x(0), n)));
            r[2..4].copy_from_slice(&split(wide::zero_shift(x(0), n)));
        }
        4 => {
            let v = angle::radians(i(0));
            r[0] = v.map_or(0, |v| v as u64);
            r[1] = status(&v);
            r[2] = angle::turns(i(1)) as u64;
            r[3] = angle::between(a[2] as u32, a[3] as u32) as u64;
            r[4] = u64::from(angle::toward(a[2] as u32, a[3] as u32, i(4)));
            r[5] = angle::half(i(0)) as u64;
        }
        5 => {
            let (c, sn) = angle::sincos(a[0] as u32);
            r[..3].copy_from_slice(&[c as u64, sn as u64, u64::from(angle::atan2(i(1), i(2)))]);
        }
        6 => {
            r[0] = u64::from(angle::asin(i(0)));
            r[1] = u64::from(angle::acos(i(1)));
            r[2] = vector::isqrt(x(2) as u128) as u64;
            r[3] = vector::length([i(4), i(5), i(6)]) as u64;
        }
        7 => {
            let u = vector::unit([i(0), i(1), i(2)]);
            for (k, c) in u.unwrap_or_default().into_iter().enumerate() {
                r[k] = c as u64;
            }
            r[3] = u64::from(u.is_some());
        }
        8 => {
            let (u, log) = vector::unit_w([x(0), x(2), x(4)]);
            for (k, c) in u.into_iter().enumerate() {
                r[k] = c as u64;
            }
            r[3] = log as u64;
        }
        9 => {
            r[0] = power::log2_w(x(0) as u128) as u64;
            let v = power::exp2_w(i(2));
            if let Ok(v) = v {
                r[1..3].copy_from_slice(&split(v as i128));
            }
            r[3] = status(&v);
        }
        10 => {
            let (start, len) = (a[0] as usize, a[1] as usize);
            let sum: Total = (start..start + len)
                .map(|k| join(terms[2 * k], terms[2 * k + 1]))
                .sum();
            wide(0, sum.get());
        }
        11 => {
            let w = lattice::weights([a[0], a[1], a[2]]);
            r[..8].copy_from_slice(&w);
            for (c, d) in lattice::deposits(i(3), &w).into_iter().enumerate() {
                r[8 + 2 * c..10 + 2 * c].copy_from_slice(&split(d as i128));
            }
        }
        12 => {
            let bud = shed::Bud {
                q: i(0),
                c: i(1),
                memory: i(2),
                hazard: i(3),
                tolerance: i(4),
                draw: a[5],
                window: i(6),
                keep: a[7],
            };
            let b = shed::balance(&bud);
            if let Ok(b) = b {
                r[..4].copy_from_slice(&[b.memory as u64, b.threshold, u64::from(b.shed), b.kept]);
            }
            r[4] = status(&b);
        }
        _ => {
            let (fall, high) = (a[5] as u32, (a[5] >> 32) as u32);
            r[0] = edge::ripe(i(0), i(1)) as u64;
            r[1] = edge::shade(i(2), i(3), i(4)) as u64;
            r[2] = u64::from(edge::settle(fall, i(6)));
            r[3] = edge::tangent_ratio(fall, high) as u64;
        }
    }
    r
}

/// Selector `s` over `cases` on the GPU.
fn run(gpu: &Gpu, pass: &io::Pass, s: u32, cases: &[[u64; 8]], terms: &[u64]) -> Vec<[u64; SLOTS]> {
    let storage = wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC;
    let n = cases.len() as u32;
    let groups = n.div_ceil(64);
    let row = groups.min(gpu.device.limits().max_compute_workgroups_per_dimension) * 64;
    let cfg = [n, s, row, 0];
    let buffer = |label, bytes: &[u8], usage| {
        io::buffer(gpu, label, bytes.len() as u64, usage, bytes).expect("a buffer")
    };
    let uniform = buffer(
        "feedback cfg",
        bytemuck::cast_slice(&cfg),
        wgpu::BufferUsages::UNIFORM,
    );
    let input = buffer("feedback cases", bytemuck::cast_slice(cases), storage);
    let terms = buffer("feedback terms", bytemuck::cast_slice(terms), storage);
    let bytes = cases.len() as u64 * SLOTS as u64 * 8;
    let output = io::buffer(gpu, "feedback results", bytes, storage, &[]).expect("a buffer");
    let bind = pass.bind(gpu, &[&uniform, &input, &terms, &output]);
    let mut encoder = gpu.device.create_command_encoder(&Default::default());
    pass.run(&mut encoder, &bind, 0, groups);
    gpu.queue.submit([encoder.finish()]);
    let out = io::read(gpu, &output, bytes).expect("the results");
    bytemuck::cast_slice::<u8, u64>(&out)
        .as_chunks::<SLOTS>()
        .0
        .to_vec()
}

#[test]
fn the_gpu_computes_every_feedback_word_the_cpu_does() {
    let Some(gpu) = crate::generation::test_gpu() else {
        return;
    };
    let pass = io::Pass::new(&gpu, source(), &[true, true, false], &["main"]);
    let mut failed = Vec::new();
    for s in 0..NAMES.len() as u32 {
        let mut terms = vec![0u64; 2];
        let cases = cases(s, &mut Rng(0xfeed + u64::from(s)), &mut terms);
        let got = run(&gpu, &pass, s, &cases, &terms);
        let mut wrong = 0;
        for (a, g) in cases.iter().zip(&got) {
            let want = expected(s, a, &terms);
            if want != *g {
                if wrong < 3 {
                    eprintln!(
                        "{}: {a:x?}\n  cpu {want:x?}\n  gpu {g:x?}",
                        NAMES[s as usize]
                    );
                }
                wrong += 1;
            }
        }
        println!(
            "{}: {} cases, {wrong} differ",
            NAMES[s as usize],
            cases.len()
        );
        if wrong > 0 {
            failed.push(NAMES[s as usize]);
        }
    }
    assert!(failed.is_empty(), "differ: {failed:?}");
    assert_eq!(P_ONE, 1 << 62);
}
