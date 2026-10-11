//! The layout's sine, cosine and arctangent (`q.wgsl`, `sincos.wgsl`)
//! held to libm's, which the CPU's layout calls, over a sweep of the
//! layout's angles and of every exponent the reduction takes.
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
    let sc = sincosf(xs[2u * i]);
    out[3u * i] = sc.x;
    out[3u * i + 1u] = sc.y;
    out[3u * i + 2u] = atan2f(xs[2u * i], xs[2u * i + 1u]);
}
";

/// The sweep's pairs: half the layout's own angles, half any exponent
/// at or below pi; each beside a second argument of any exponent.
fn inputs(n: usize) -> Vec<f32> {
    let mut xs = Vec::with_capacity(2 * n);
    let mut state = 0x9e37_79b9_7f4a_7c15u64;
    for k in 0..n {
        state = state.wrapping_mul(6_364_136_223_846_793_005).wrapping_add(1_442_695_040_888_963_407);
        let u = (state >> 40) as f32 / (1u64 << 24) as f32;
        let bits = (state >> 32) as u32;
        let wide = f32::from_bits((bits & 0x807f_ffff) | ((100 + (bits >> 23) % 29) << 23));
        let x = match k % 2 {
            0 => (u - 0.5) * std::f32::consts::TAU,
            _ if wide.abs() > std::f32::consts::PI => wide * 0.5,
            _ => wide,
        };
        let y = f32::from_bits((state as u32 & 0x3fff_ffff) | (((state >> 63) as u32) << 31));
        xs.extend([x, y]);
    }
    xs
}

/// The device's sine, cosine and arctangent of each pair, against libm's:
/// the rows of each that differ, the first few printed.
fn differing(gpu: &Gpu) -> [usize; 3] {
    let n = 1 << 20;
    let xs = inputs(n);
    let shader = format!(
        "{}\n{}\n{}\n{}\n{MATH}\n",
        super::super::fine::contract_with(super::ARITH),
        super::ROOT,
        super::Q,
        super::SINCOS
    );
    let pass = io::Pass::new(gpu, shader, &[true, false], &["math_main"]);
    let usage = wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC;
    let input = io::buffer(gpu, "math in", 8 * n as u64, usage, bytemuck::cast_slice(&xs)).unwrap();
    let output = io::buffer(gpu, "math out", 12 * n as u64, usage, &[]).unwrap();
    let groups = (n as u32).div_ceil(64);
    let most = gpu.device.limits().max_compute_workgroups_per_dimension;
    let cfg = [n as u32, groups.min(most) * 64, 0, 0];
    let uniform = io::buffer(gpu, "math cfg", 16, wgpu::BufferUsages::UNIFORM, bytemuck::cast_slice(&cfg)).unwrap();
    let bind = pass.bind(gpu, &[&uniform, &input, &output]);
    let mut e = gpu.encoder(&Default::default());
    pass.run(&mut e, &bind, 0, groups);
    gpu.submit([e.finish()]);
    let raw = io::read(gpu, &output, 12 * n as u64).unwrap();
    let got: &[f32] = bytemuck::cast_slice(&raw);
    let mut count = [0; 3];
    let mut shown = 0;
    for (k, xy) in xs.as_chunks::<2>().0.iter().enumerate() {
        let (s, c) = xy[0].sin_cos_fixed();
        let cpu = [s, c, xy[0].atan2_fixed(xy[1])];
        let dev = &got[3 * k..3 * k + 3];
        // The sine and cosine word for word; the arctangent within the
        // ruled tolerance of a half turn (`ruled.rs`).
        let far = f64::from((dev[2] - cpu[2]).abs()) > super::ruled::FLOAT_TOLERANCE * std::f64::consts::PI;
        let differ = [dev[0].to_bits() != cpu[0].to_bits(), dev[1].to_bits() != cpu[1].to_bits(), far];
        for (c, d) in count.iter_mut().zip(&differ) {
            *c += usize::from(*d);
        }
        if differ.contains(&true) && shown < 8 {
            shown += 1;
            println!("x {:e} y {:e}: device {dev:?}, libm {cpu:?}", xy[0], xy[1]);
        }
    }
    println!("of {n}: sin {}, cos {}, atan2 {} differ", count[0], count[1], count[2]);
    count
}

/// The sine and cosine are summed in integers (`sincos.wgsl`), so they
/// are libm's on any device.
#[test]
fn the_layouts_sine_and_cosine_are_libms() {
    let _held = super::super::whole_tree();
    let Some(gpu) = super::super::test_gpu() else {
        return;
    };
    let [sin, cos, _] = differing(&gpu);
    assert_eq!((sin, cos), (0, 0), "the layout's sine and cosine are libm's");
}

/// Within the ruled tolerance: the driver's division and reordering move
/// it in its last places (LAYOUT.md).
#[test]
fn the_layouts_arctangent_is_libms_within_tolerance() {
    let _held = super::super::whole_tree();
    let Some(gpu) = super::super::test_gpu() else {
        return;
    };
    let atan2 = differing(&gpu)[2];
    assert_eq!(atan2, 0, "the layout's arctangent is within the tolerance of libm's");
}
