//! The allotment's f64 products (`hier_real.wgsl`) held to the CPU's
//! multiply, bit for bit, over presences and lights of every exponent
//! they take, and the f32 each rounds to.
use super::super::io;
use crate::Gpu;

const MUL: &str = "
@group(0) @binding(0) var<uniform> n: vec4<u32>;
@group(0) @binding(1) var<storage, read> xs: array<u32>;
@group(0) @binding(2) var<storage, read_write> out: array<u32>;
@compute @workgroup_size(64)
fn mul_main(@builtin(global_invocation_id) id: vec3<u32>) {
    let i = id.x + id.y * n.y;
    if i >= n.x {
        return;
    }
    let a = vec2<u32>(xs[4u * i], xs[4u * i + 1u]);
    let b = vec2<u32>(xs[4u * i + 2u], xs[4u * i + 3u]);
    let p = mul_real(a, b);
    out[3u * i] = p.x;
    out[3u * i + 1u] = p.y;
    out[3u * i + 2u] = bitcast<u32>(single(p));
}
";

/// The pairs: a product of two f32s times a third (a presence), and an
/// f64 of any significand in a presence's range times one in a light's;
/// a few with none.
fn inputs(n: usize) -> Vec<(f64, f64)> {
    let mut state = 0x2545_f491_4f6c_dd1du64;
    let mut next = || {
        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;
        state
    };
    let mut f32_in = |lo: i32, hi: i32| {
        let bits = next();
        let e = lo + (bits % (hi - lo) as u64) as i32;
        f64::from(f32::from_bits(
            ((e + 127) as u32) << 23 | (bits >> 40) as u32 & 0x7f_ffff,
        ))
    };
    let mut pairs = Vec::with_capacity(n);
    for k in 0..n {
        let pair = match k % 3 {
            0 => (f32_in(-30, 1) * f32_in(-30, 1), f32_in(-60, 1)),
            1 => (
                f32_in(-40, 1) * f32_in(-20, 1) * f32_in(-20, 1),
                f32_in(-20, 2) / 3.0,
            ),
            _ => (f32_in(-40, 1), f32_in(-10, 2) * 0.1),
        };
        pairs.push(if k % 997 == 0 { (0.0, pair.1) } else { pair });
    }
    pairs
}

/// The products and their f32s that differ from the CPU's.
fn differing(gpu: &Gpu) -> [usize; 2] {
    let pairs = inputs(1 << 20);
    let n = pairs.len();
    let words: Vec<u32> = (pairs.iter())
        .flat_map(|(a, b)| [a.to_bits(), b.to_bits()])
        .flat_map(|x| [x as u32, (x >> 32) as u32])
        .collect();
    let shader = format!(
        "{}\n{}\n{}\n{MUL}",
        super::contract(),
        super::REAL,
        super::HIER_REAL
    );
    let pass = io::Pass::new(gpu, shader, &[true, false], &["mul_main"]);
    let usage = wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC;
    let input = io::buffer(
        gpu,
        "mul in",
        16 * n as u64,
        usage,
        bytemuck::cast_slice(&words),
    )
    .unwrap();
    let output = io::buffer(gpu, "mul out", 12 * n as u64, usage, &[]).unwrap();
    let groups = (n as u32).div_ceil(64);
    let most = gpu.device.limits().max_compute_workgroups_per_dimension;
    let cfg = [n as u32, groups.min(most) * 64, 0, 0];
    let usage = wgpu::BufferUsages::UNIFORM;
    let uniform = io::buffer(gpu, "mul cfg", 16, usage, bytemuck::cast_slice(&cfg)).unwrap();
    let bind = pass.bind(gpu, &[&uniform, &input, &output]);
    let mut e = gpu.encoder(&Default::default());
    pass.run(&mut e, &bind, 0, groups);
    gpu.submit([e.finish()]);
    let raw = io::read(gpu, &output, 12 * n as u64).unwrap();
    let got: &[u32] = bytemuck::cast_slice(&raw);
    let mut count = [0; 2];
    for (k, (a, b)) in pairs.iter().enumerate() {
        let p = a * b;
        let dev = u64::from(got[3 * k]) | u64::from(got[3 * k + 1]) << 32;
        let wrong = [dev != p.to_bits(), got[3 * k + 2] != (p as f32).to_bits()];
        if wrong.contains(&true) && count.iter().sum::<usize>() < 8 {
            println!("{a:e} × {b:e}: device {:e}, cpu {p:e}", f64::from_bits(dev));
        }
        count[0] += usize::from(wrong[0]);
        count[1] += usize::from(wrong[1]);
    }
    println!(
        "of {n}: products {}, their f32s {} differ",
        count[0], count[1]
    );
    count
}

#[test]
fn the_allotments_products_are_the_cpus() {
    let _held = super::super::whole_tree();
    let Some(gpu) = super::super::test_gpu() else {
        return;
    };
    assert_eq!(
        differing(&gpu),
        [0, 0],
        "the device's f64 products are the CPU's"
    );
}
