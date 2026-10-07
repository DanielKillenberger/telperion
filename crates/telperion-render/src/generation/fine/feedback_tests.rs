//! The feedback contract's conformance probe (fn-211 task 8; Astra's
//! review of CONTRACT-A.md, section 4): every function of
//! `telperion_space::fixed::feedback` on the GPU and the CPU, over random
//! cases at and about their domains' edges with Astra's counterexamples
//! first, gives the same words, error statuses included.
use super::super::io;
use super::feedback_cases::{cases, join, split, Rng};
use crate::Gpu;
use telperion_space::fixed::feedback::wide::I256;
use telperion_space::fixed::feedback::{
    angle, edge, lattice, power, shed, status, total::Total, vector, wide,
};
use telperion_space::fixed::P_ONE;

/// Results a case.
const SLOTS: usize = 24;
const PROBE: &str = include_str!("feedback_probe.wgsl");

const NAMES: [&str; 17] = [
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
    "turned",
    "carry, dot, cross",
    "quotient",
];

/// The probe's WGSL: the decision contract, the feedback contract, the
/// probe.
pub(super) fn source() -> String {
    format!("{}\n{PROBE}", super::feedback())
}

/// The CPU's results for one case.
pub(super) fn expected(s: u32, a: &[u64; 8], terms: &[u64]) -> [u64; SLOTS] {
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
        14 => {
            let t = vector::Turn {
                pivot: [i(3), i(4), i(5)],
                cos: i(6),
                sin: i(7),
            };
            for (k, c) in vector::turned([i(0), i(1), i(2)], &t)
                .into_iter()
                .enumerate()
            {
                r[k] = c as u64;
            }
        }
        15 => {
            let (from, to) = ([i(0), i(1), i(2)], [i(3), i(4), i(5)]);
            if let Some(t) = vector::carry(from, to) {
                for k in 0..3 {
                    r[k] = t.pivot[k] as u64;
                }
                r[3..6].copy_from_slice(&[t.cos as u64, t.sin as u64, 1]);
            }
            r[6] = vector::dot(from, to) as u64;
            for (k, c) in vector::cross(from, to).into_iter().enumerate() {
                r[7 + k] = c as u64;
            }
        }
        16 => {
            let n = I256 {
                hi: join(a[2], a[3]),
                lo: join(a[0], a[1]) as u128,
            };
            let v = n.quotient(join(a[4], a[5]));
            if let Ok(v) = v {
                r[..2].copy_from_slice(&split(v.lo as i128));
                r[2..4].copy_from_slice(&split(v.hi));
            }
            r[4] = status(&v);
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
pub(super) fn run(
    gpu: &Gpu,
    pass: &io::Pass,
    s: u32,
    cases: &[[u64; 8]],
    terms: &[u64],
) -> Vec<[u64; SLOTS]> {
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
