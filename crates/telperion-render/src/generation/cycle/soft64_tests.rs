//! The software f64 (`soft64.wgsl`, the settle's) held to Rust's f64 on
//! the device, word for word: over a million inputs an operation, drawn to
//! reach every path (any exponent, subnormals, zeros of both signs,
//! cancellation, rounding ties, overflow).
use super::super::io;
use crate::Gpu;

const PROBE: &str = "
struct Probe {
    n: u32,
    row: u32,
    op: u32,
    pad: u32,
}
@group(0) @binding(0) var<uniform> P: Probe;
@group(0) @binding(1) var<storage, read> xs: array<vec4<u32>>;
@group(0) @binding(2) var<storage, read_write> out: array<vec2<u32>>;

@compute @workgroup_size(64)
fn arith_main(@builtin(global_invocation_id) id: vec3<u32>) {
    let i = id.x + id.y * P.row;
    if i >= P.n {
        return;
    }
    let a = xs[i].xy;
    let b = xs[i].zw;
    var r = vec2<u32>(0u);
    switch P.op {
        case 0u: { r = f64_add(a, b); }
        case 1u: { r = f64_add(a, f64_neg(b)); }
        case 2u: { r = f64_mul(a, b); }
        case 3u: { r = f64_div(a, b); }
        case 4u: { r = vec2<u32>(bitcast<u32>(f64_to_f32(a)), 0u); }
        case 5u: { r = f64_from_f32(bitcast<f32>(a.x)); }
        case 6u: { r = f64_floor(a); }
        case 7u: { r = vec2<u32>(u32(f64_to_i32(a)), 0u); }
        default: { r = f64_from_i32(i32(a.x)); }
    }
    out[i] = r;
}
";

const N: usize = 1 << 20;

/// An operation by name and Rust's own.
type Op = (&'static str, fn(f64, f64) -> f64);
/// A conversion: its name, its probe's operation, its inputs and Rust's words.
type Case = (&'static str, u32, Vec<(f64, f64)>, Vec<u64>);

/// splitmix64.
struct Draw(u64);

impl Draw {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9e37_79b9_7f4a_7c15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        z ^ (z >> 31)
    }

    /// A finite f64 of kind `k`: any, near one, subnormal or tiny, a
    /// zero, a short significand (ties), or near the largest.
    fn real(&mut self, k: u64) -> f64 {
        let r = self.next();
        let sign = r & (1 << 63);
        let frac = r & ((1 << 52) - 1);
        let exp = |e: u64| f64::from_bits(sign | e << 52 | frac);
        match k % 6 {
            0 => exp((r >> 52) % 2047),
            1 => exp(1023 - 40 + (r >> 52) % 80),
            2 => exp((r >> 52) % 4),
            3 => f64::from_bits(sign),
            4 => f64::from_bits(
                sign | (1023 - 30 + (r >> 52) % 60) << 52 | (frac >> (r % 50)) << (r % 50),
            ),
            _ => exp(2040 + (r >> 52) % 7),
        }
    }

    /// A pair: two reals of any kinds, or the second a few units in the
    /// last place from the first, of either sign (cancellation).
    fn pair(&mut self) -> (f64, f64) {
        let (k, j) = (self.next(), self.next());
        let a = self.real(k);
        if j % 4 == 0 {
            let b = f64::from_bits(a.to_bits().wrapping_add(j >> 60).wrapping_sub(8));
            let b = if b.is_finite() { b } else { a };
            return (a, if j & 2 == 0 { -b } else { b });
        }
        (a, self.real(j))
    }
}

fn words(x: f64) -> [u32; 2] {
    let b = x.to_bits();
    [b as u32, (b >> 32) as u32]
}

/// Each input row's words on the device under `entry` and `op`.
fn run(gpu: &Gpu, entry: &str, op: u32, rows: &[[u32; 4]]) -> Vec<u64> {
    let shader = format!("{}\n{PROBE}", super::SOFT64);
    let pass = io::Pass::new(gpu, shader, &[true, false], &[entry]);
    let usage = wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC;
    let n = rows.len();
    let input = io::buffer(
        gpu,
        "soft64 in",
        16 * n as u64,
        usage,
        bytemuck::cast_slice(rows),
    )
    .unwrap();
    let output = io::buffer(gpu, "soft64 out", 8 * n as u64, usage, &[]).unwrap();
    let groups = (n as u32).div_ceil(64);
    let most = gpu.device.limits().max_compute_workgroups_per_dimension;
    let cfg = [n as u32, groups.min(most) * 64, op, 0];
    let uniform = io::buffer(
        gpu,
        "soft64 cfg",
        16,
        wgpu::BufferUsages::UNIFORM,
        bytemuck::cast_slice(&cfg),
    )
    .unwrap();
    let bind = pass.bind(gpu, &[&uniform, &input, &output]);
    let mut e = gpu.encoder(&Default::default());
    pass.run(&mut e, &bind, 0, groups);
    gpu.submit([e.finish()]);
    let raw = io::read(gpu, &output, 8 * n as u64).unwrap();
    bytemuck::cast_slice::<u8, [u32; 2]>(&raw)
        .iter()
        .map(|w| u64::from(w[0]) | u64::from(w[1]) << 32)
        .collect()
}

/// The rows the device's results differ from `want` in, the first few
/// printed.
fn differ(name: &str, inputs: &[(f64, f64)], got: &[u64], want: &[u64]) -> usize {
    let mut count = 0;
    for ((x, g), w) in inputs.iter().zip(got).zip(want) {
        if g != w {
            count += 1;
            if count <= 6 {
                println!(
                    "{name} {:e} {:e} ({:#x}, {:#x}): device {g:#018x}, Rust {w:#018x}",
                    x.0,
                    x.1,
                    x.0.to_bits(),
                    x.1.to_bits()
                );
            }
        }
    }
    println!("{name}: {count} of {} differ", inputs.len());
    count
}

/// How many results are subnormal, zero and infinite: the paths reached.
fn reach(name: &str, want: &[u64]) {
    let class = |c: std::num::FpCategory| {
        want.iter()
            .filter(|w| f64::from_bits(**w).classify() == c)
            .count()
    };
    use std::num::FpCategory::{Infinite, Subnormal, Zero};
    println!(
        "{name}: {} subnormal, {} zero, {} infinite results",
        class(Subnormal),
        class(Zero),
        class(Infinite)
    );
}

fn rows(inputs: &[(f64, f64)]) -> Vec<[u32; 4]> {
    inputs
        .iter()
        .map(|(a, b)| {
            let (a, b) = (words(*a), words(*b));
            [a[0], a[1], b[0], b[1]]
        })
        .collect()
}

#[test]
fn the_software_f64_is_ieee() {
    let _held = super::super::whole_tree();
    let Some(gpu) = super::super::test_gpu() else {
        return;
    };
    let mut d = Draw(0x05ee_df64);
    let pairs: Vec<(f64, f64)> = (0..N).map(|_| d.pair()).collect();
    let ops: [Op; 4] = [
        ("add", |a, b| a + b),
        ("sub", |a, b| a - b),
        ("mul", |a, b| a * b),
        ("div", |a, b| a / b),
    ];
    let mut wrong = Vec::new();
    for (op, (name, f)) in ops.iter().enumerate() {
        let mut inputs: Vec<(f64, f64)> = pairs
            .iter()
            .copied()
            .filter(|p| !(op == 3 && p.1 == 0.0))
            .collect();
        while inputs.len() < N {
            inputs.extend(Some(d.pair()).filter(|p| p.1 != 0.0));
        }
        let want: Vec<u64> = inputs.iter().map(|&(a, b)| f(a, b).to_bits()).collect();
        reach(name, &want);
        if differ(
            name,
            &inputs,
            &run(&gpu, "arith_main", op as u32, &rows(&inputs)),
            &want,
        ) > 0
        {
            wrong.push(*name);
        }
    }
    assert!(
        wrong.is_empty(),
        "the software f64 is not IEEE's: {wrong:?}"
    );
}

#[test]
fn the_software_conversions_and_floor_are_rusts() {
    let _held = super::super::whole_tree();
    let Some(gpu) = super::super::test_gpu() else {
        return;
    };
    let mut d = Draw(0xc0_47e7);
    // Reals about f32's range and its subnormals, any, f32s, and ties
    // halfway between two f32s, normal and subnormal.
    let reals: Vec<f64> = (0..N)
        .map(|k| {
            let r = d.next();
            match k % 5 {
                0 => f64::from_bits(
                    r & ((1 << 63) | ((1 << 52) - 1)) | (1023 - 160 + (r >> 52) % 300) << 52,
                ),
                1 => d.real(r % 3),
                2 => f64::from(f32::from_bits(
                    r as u32 & 0x7f7f_ffff | (r >> 32) as u32 & 0x8000_0000,
                )),
                3 => f64::from_bits(
                    f64::from(f32::from_bits(r as u32 & 0x7f7f_ffff)).to_bits() | 1 << 28,
                ),
                _ => (2.0 * (r % (1 << 23)) as f64 + 1.0) * 2f64.powi(-150),
            }
        })
        .collect();
    let small: Vec<f64> = (0..N)
        .map(|k| ((d.next() as i64) >> (k % 62)) as f64 * 0.37)
        .collect();
    let floats: Vec<f32> = (0..N)
        .map(|_| f32::from_bits(d.next() as u32))
        .filter(|x| x.is_finite())
        .collect();
    let ints: Vec<i32> = (0..N).map(|k| (d.next() as i32) >> (k % 31)).collect();
    let single = |v: &[f64]| -> Vec<(f64, f64)> { v.iter().map(|&x| (x, 0.0)).collect() };
    let cases: [Case; 5] = [
        (
            "to f32",
            4,
            single(&reals),
            reals
                .iter()
                .map(|&x| u64::from((x as f32).to_bits()))
                .collect(),
        ),
        (
            "from f32",
            5,
            floats
                .iter()
                .map(|&x| (f64::from_bits(u64::from(x.to_bits())), 0.0))
                .collect(),
            floats.iter().map(|&x| f64::from(x).to_bits()).collect(),
        ),
        (
            "floor",
            6,
            single(&reals),
            reals.iter().map(|x| x.floor().to_bits()).collect(),
        ),
        (
            "to i32",
            7,
            single(&small),
            small.iter().map(|&x| u64::from(x as i32 as u32)).collect(),
        ),
        (
            "from i32",
            8,
            ints.iter()
                .map(|&n| (f64::from_bits(u64::from(n as u32)), 0.0))
                .collect(),
            ints.iter().map(|&n| f64::from(n).to_bits()).collect(),
        ),
    ];
    let mut wrong = Vec::new();
    for (name, op, inputs, want) in &cases {
        if differ(
            name,
            inputs,
            &run(&gpu, "arith_main", *op, &rows(inputs)),
            want,
        ) > 0
        {
            wrong.push(*name);
        }
    }
    assert!(
        wrong.is_empty(),
        "the software conversions are not Rust's: {wrong:?}"
    );
}
