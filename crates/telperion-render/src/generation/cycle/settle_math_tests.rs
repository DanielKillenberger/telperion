//! Sag's arcsine and hypotenuse (`settle_libm.wgsl`) held to libm's,
//! which the CPU's sag calls: `asinf` over (0, 1] and below, its f64
//! root in integers, and `hypotf` over pairs of every exponent a lever's
//! unit takes.
use super::super::io;
use crate::Gpu;
use telperion_space::Transcendental;

const MATH: &str = "
@group(0) @binding(0) var<uniform> n: vec4<u32>;
@group(0) @binding(1) var<storage, read> xs: array<f32>;
@group(0) @binding(2) var<storage, read_write> out: array<f32>;
@compute @workgroup_size(64)
fn math_main(@builtin(global_invocation_id) id: vec3<u32>) {
    let i = id.x + id.y * n.y;
    if i >= n.x {
        return;
    }
    out[2u * i] = asinf(xs[2u * i]);
    out[2u * i + 1u] = hypotf(xs[2u * i], xs[2u * i + 1u]);
}
";

/// The sweep's pairs: an arcsine's argument in [0, 1] (half of them at
/// least a half, where its root is taken in f64), beside a second of any
/// exponent at or below one.
fn inputs(n: usize) -> Vec<f32> {
    let mut xs = Vec::with_capacity(2 * n);
    let mut state = 0x6a09_e667_f3bc_c909u64;
    for k in 0..n {
        state = state.wrapping_mul(6_364_136_223_846_793_005).wrapping_add(1_442_695_040_888_963_407);
        let u = (state >> 40) as f32 / (1u64 << 24) as f32;
        let x = match k % 4 {
            0 => 0.5 + u * 0.5,
            1 => u,
            2 => 1.0 - u * 1e-3,
            _ => f32::from_bits(0x3f00_0000 + (state >> 41) as u32 % 0x80_0000),
        };
        let bits = (state >> 32) as u32;
        let y = f32::from_bits((bits & 0x807f_ffff) | ((60 + (bits >> 23) % 68) << 23));
        xs.extend([x, y]);
    }
    xs
}

/// Whether `dev` is beyond the ruled tolerance of `cpu`'s own magnitude
/// (`ruled.rs`): these values span every exponent.
fn far(dev: f32, cpu: f32) -> bool {
    f64::from((dev - cpu).abs()) > super::ruled::FLOAT_TOLERANCE * f64::from(cpu.abs()) || dev.is_nan() != cpu.is_nan()
}

/// The device's arcsine and hypotenuse of each pair against libm's: the
/// rows of each beyond the ruled tolerance, the first few printed.
fn differing(gpu: &Gpu) -> [usize; 2] {
    let n = 1 << 20;
    let xs = inputs(n);
    let shader = format!(
        "{}\n{}\n{}\n{}\n{}\n{}\n{MATH}\n",
        super::super::fine::contract_with(super::ARITH),
        super::ROOT,
        super::settle_device::soft64(),
        super::Q,
        super::SINCOS,
        include_str!("settle_libm.wgsl")
    );
    let pass = io::Pass::new(gpu, shader, &[true, false], &["math_main"]);
    let usage = wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC;
    let input = io::buffer(gpu, "math in", 8 * n as u64, usage, bytemuck::cast_slice(&xs)).unwrap();
    let output = io::buffer(gpu, "math out", 8 * n as u64, usage, &[]).unwrap();
    let groups = (n as u32).div_ceil(64);
    let most = gpu.device.limits().max_compute_workgroups_per_dimension;
    let cfg = [n as u32, groups.min(most) * 64, 0, 0];
    let uniform = io::buffer(gpu, "math cfg", 16, wgpu::BufferUsages::UNIFORM, bytemuck::cast_slice(&cfg)).unwrap();
    let bind = pass.bind(gpu, &[&uniform, &input, &output]);
    let mut e = gpu.encoder(&Default::default());
    pass.run(&mut e, &bind, 0, groups);
    gpu.submit([e.finish()]);
    let raw = io::read(gpu, &output, 8 * n as u64).unwrap();
    let got: &[f32] = bytemuck::cast_slice(&raw);
    let mut count = [0; 2];
    let mut shown = 0;
    for (k, xy) in xs.as_chunks::<2>().0.iter().enumerate() {
        let cpu = [xy[0].asin_fixed(), xy[0].hypot_fixed(xy[1])];
        let dev = &got[2 * k..2 * k + 2];
        let differ: Vec<bool> = dev.iter().zip(cpu).map(|(d, h)| far(*d, h)).collect();
        for (c, d) in count.iter_mut().zip(&differ) {
            *c += usize::from(*d);
        }
        if differ.contains(&true) && shown < 8 {
            shown += 1;
            println!("x {:e} y {:e}: device {dev:?}, libm {cpu:?}", xy[0], xy[1]);
        }
    }
    println!("of {n}: asin {}, hypot {} differ", count[0], count[1]);
    count
}

/// Within the ruled tolerance of libm's (`ruled.rs`).
#[test]
fn sags_arcsine_and_hypotenuse_are_libms() {
    let _held = super::super::whole_tree();
    let Some(gpu) = super::super::test_gpu() else {
        return;
    };
    assert_eq!(differing(&gpu), [0, 0], "sag's arcsine and hypotenuse are within the tolerance of libm's");
}

const TURN: &str = "
@group(0) @binding(0) var<uniform> n: vec4<u32>;
@group(0) @binding(1) var<storage, read> xs: array<f32>;
@group(0) @binding(2) var<storage, read_write> out: array<f32>;
@compute @workgroup_size(64)
fn turn_main(@builtin(global_invocation_id) id: vec3<u32>) {
    let i = id.x + id.y * n.y;
    if i >= n.x {
        return;
    }
    let x = 7u * i;
    let lever = vec3<f32>(xs[x], xs[x + 1u], xs[x + 2u]);
    out[4u * i] = sag_turn(unit_w(lever), xs[x + 3u], xs[x + 4u], xs[x + 5u], xs[x + 6u]);
    let uw = unit_w(lever);
    out[4u * i + 1u] = qatan2(hypotf(uw.u.x, uw.u.y), -uw.u.z);
    out[4u * i + 2u] = qbetween(0.0, qdiv(asinf(clamp(exp2c(min(xs[x + 3u] - uw.l, 0.0)), -1.0, 1.0)), QTAU));
    out[4u * i + 3u] = uw.l;
}
";

/// Sag's turn (`settle_sag.wgsl`'s `sag_turn`) against the CPU's over
/// levers, caps, sags, radii and lengths a young tree meets.
#[test]
#[ignore = "a probe"]
fn sags_turn_is_the_cpus() {
    let _held = super::super::whole_tree();
    let Some(gpu) = super::super::test_gpu() else {
        return;
    };
    let n = 1 << 18;
    let mut state = 0x243f_6a88_85a3_08d3u64;
    let mut next = || {
        state = state.wrapping_mul(6_364_136_223_846_793_005).wrapping_add(1_442_695_040_888_963_407);
        (state >> 40) as f32 / (1u64 << 24) as f32
    };
    let sags = [1e-4, 9e-4, 5e-4, 2e-5, 3e-4];
    let mut xs = Vec::new();
    let mut cases = Vec::new();
    for k in 0..n {
        let scale = (2.0f32).powf(next() * 24.0 - 16.0);
        let lever = [(next() - 0.5) * scale, (next() - 0.5) * scale, (next() - 0.5) * scale];
        let reach = (lever.iter().map(|v| v * v).sum::<f32>()).sqrt().log2();
        let cap = reach + (next() - 0.7) * 6.0;
        let sag: f64 = sags[k % sags.len()];
        let radius = f64::from((2.0f32).powf(next() * 8.0 - 12.0));
        let length = f64::from(next() * 0.5);
        xs.extend(lever);
        xs.extend([cap, sag.log2_fixed() as f32, radius as f32, length as f32]);
        cases.push((lever, cap, sag, radius, length));
    }
    let shader = format!(
        "{}\n{}\n{}\n{}\n{}\n{}\n{}\n{TURN}\n",
        super::super::fine::contract_with(super::ARITH),
        super::ROOT,
        super::settle_device::soft64(),
        super::Q,
        super::SINCOS,
        include_str!("settle_libm.wgsl"),
        super::settle_device::sag_math()
    );
    let pass = io::Pass::new(&gpu, shader, &[true, false], &["turn_main"]);
    let usage = wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC;
    let input = io::buffer(&gpu, "turn in", 4 * xs.len() as u64, usage, bytemuck::cast_slice(&xs)).unwrap();
    let output = io::buffer(&gpu, "turn out", 16 * n as u64, usage, &[]).unwrap();
    let groups = (n as u32).div_ceil(64);
    let most = gpu.device.limits().max_compute_workgroups_per_dimension;
    let cfg = [n as u32, groups.min(most) * 64, 0, 0];
    let uniform = io::buffer(&gpu, "turn cfg", 16, wgpu::BufferUsages::UNIFORM, bytemuck::cast_slice(&cfg)).unwrap();
    let bind = pass.bind(&gpu, &[&uniform, &input, &output]);
    let mut e = gpu.encoder(&Default::default());
    pass.run(&mut e, &bind, 0, groups);
    gpu.submit([e.finish()]);
    let raw = io::read(&gpu, &output, 16 * n as u64).unwrap();
    let got: &[f32] = bytemuck::cast_slice(&raw);
    let mut differ = 0;
    for (k, &(lever, cap, sag, radius, length)) in cases.iter().enumerate() {
        let cpu = telperion_space::settle_sag_turn(lever, cap, sag, (radius, length));
        if far(got[4 * k], cpu) {
            differ += 1;
            if differ <= 8 {
                println!("lever {lever:?} cap {cap} sag {sag} r {radius} len {length}: device {:?}, cpu {cpu}", &got[4 * k..4 * k + 4]);
            }
        }
    }
    println!("of {n}: {differ} turns differ");
    assert_eq!(differ, 0, "sag's turn is within the tolerance of the CPU's");
}

/// Whether the device keeps f32 subnormals: a product below the least
/// normal, and a sum of two subnormals.
#[test]
#[ignore = "a probe"]
fn the_device_keeps_subnormals() {
    let _held = super::super::whole_tree();
    let Some(gpu) = super::super::test_gpu() else {
        return;
    };
    let shader = "
@group(0) @binding(0) var<uniform> n: vec4<u32>;
@group(0) @binding(1) var<storage, read> xs: array<f32>;
@group(0) @binding(2) var<storage, read_write> out: array<f32>;
fn kept(x: f32) -> f32 { return bitcast<f32>(bitcast<u32>(x) ^ n.z); }
@compute @workgroup_size(1)
fn probe_main() {
    out[0] = kept(xs[0] * xs[1]);
    out[1] = kept(xs[2] + xs[3]);
    out[2] = kept(xs[2] * 2.0);
    out[3] = kept(xs[4] - xs[5]);
}
";
    let xs = [1.1e-20f32, 1.1e-20, 1e-40, 2e-40, 1.5e-38, 1.4e-38];
    let pass = io::Pass::new(&gpu, shader.into(), &[true, false], &["probe_main"]);
    let usage = wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC;
    let input = io::buffer(&gpu, "in", 24, usage, bytemuck::cast_slice(&xs)).unwrap();
    let output = io::buffer(&gpu, "out", 16, usage, &[]).unwrap();
    let uniform = io::buffer(&gpu, "cfg", 16, wgpu::BufferUsages::UNIFORM, bytemuck::cast_slice(&[0u32; 4])).unwrap();
    let bind = pass.bind(&gpu, &[&uniform, &input, &output]);
    let mut e = gpu.encoder(&Default::default());
    pass.run(&mut e, &bind, 0, 1);
    gpu.submit([e.finish()]);
    let raw = io::read(&gpu, &output, 16).unwrap();
    let got: &[f32] = bytemuck::cast_slice(&raw);
    let cpu = [xs[0] * xs[1], xs[2] + xs[3], xs[2] * 2.0, xs[4] - xs[5]];
    println!("device {got:?}, cpu {cpu:?}");
}
