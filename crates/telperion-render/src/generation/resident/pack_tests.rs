//! The renderer's packed point as the sweep writes it (`pack.wgsl`).
use super::super::io;
use super::sweep::PACK;
use telperion_core::math::Vec3;
use telperion_core::surface::CurvePoint;

/// Each frame of `frames` (six floats) packed into its point's last three
/// words.
const FRAMES: &str = "
struct Cfg { n: u32, _a: u32, _b: u32, _c: u32 }
@group(0) @binding(0) var<uniform> cfg: Cfg;
@group(0) @binding(1) var<storage, read> frames: array<f32>;
@group(0) @binding(2) var<storage, read_write> packed: array<u32>;
@compute @workgroup_size(64)
fn frames_packed(@builtin(global_invocation_id) id: vec3<u32>) {
    let k = id.x;
    if k >= cfg.n {
        return;
    }
    let f = k * 6u;
    let n = vec3<f32>(frames[f], frames[f + 1u], frames[f + 2u]);
    let b = vec3<f32>(frames[f + 3u], frames[f + 4u], frames[f + 5u]);
    let w = frame_words(n, b);
    packed[k * 3u] = w.x;
    packed[k * 3u + 1u] = w.y;
    packed[k * 3u + 2u] = w.z;
}
";

/// fn-212 task 11, Astra's finding 5: the device packs a frame as the
/// CPU does, a zero component's sign as `signum` takes it. The frames
/// are exact in float32, so no rounding residual enters.
#[test]
fn the_device_packs_a_frame_as_the_cpu_does() {
    let Some(gpu) = crate::generation::test_gpu() else {
        return;
    };
    let frames = [
        (Vec3::new(0.0, 0.0, -1.0), Vec3::new(1.0, 0.0, 0.0)),
        (Vec3::new(-0.0, 0.0, -1.0), Vec3::new(0.0, -1.0, -0.0)),
        (Vec3::new(0.0, -0.5, -0.5), Vec3::new(-0.0, 0.5, -0.5)),
        (Vec3::new(0.5, 0.0, -0.5), Vec3::new(0.0, 1.0, 0.0)),
    ];
    let words: Vec<f32> = (frames.iter())
        .flat_map(|(n, b)| [n.x, n.y, n.z, b.x, b.y, b.z])
        .map(|v| v as f32)
        .collect();
    let n = frames.len() as u32;
    let usage = wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC;
    let cfg = [n, 0, 0, 0];
    let cfg = io::buffer(
        &gpu,
        "cfg",
        16,
        wgpu::BufferUsages::UNIFORM,
        bytemuck::cast_slice(&cfg),
    );
    let fine = io::buffer(
        &gpu,
        "frames",
        4 * words.len() as u64,
        usage,
        bytemuck::cast_slice(&words),
    );
    let packed = io::buffer(&gpu, "packed", 12 * u64::from(n), usage, &[]);
    let (cfg, fine, packed) = (cfg.unwrap(), fine.unwrap(), packed.unwrap());
    let pass = io::Pass::new(
        &gpu,
        format!("{PACK}\n{FRAMES}"),
        &[true, false],
        &["frames_packed"],
    );
    let bind = pass.bind(&gpu, &[&cfg, &fine, &packed]);
    let mut e = gpu.encoder(&Default::default());
    pass.run(&mut e, &bind, 0, 1);
    gpu.submit([e.finish()]);
    let read = io::read(&gpu, &packed, 12 * u64::from(n)).unwrap();
    let read: &[u32] = bytemuck::cast_slice(&read);
    for (k, &(normal, binormal)) in frames.iter().enumerate() {
        let point = CurvePoint {
            centre: Vec3::new(1.0, 2.0, 3.0),
            radius: 0.5,
            along: 4.0,
            normal,
            binormal,
        };
        assert_eq!(read[k * 3..k * 3 + 3], point.pack()[5..8], "frame {k}");
    }
}
