//! Smooth bark's far path returns what its near path averages to. Lichen,
//! lenticels and a curling strip each fade to a mean once a pixel cannot
//! resolve them; a mean that is not the field's own average is a step in
//! colour as a trunk recedes, which no distance tolerance would explain.
mod common;

/// Every probe samples the surface of a cylinder a few cells across, one ring
/// per row at a radius of its own, so the rings cross the lattice at every
/// offset rather than the one a single radius would. Each is read at zero
/// footprint and at one no pixel could resolve, so the far value is the mean
/// the shader returns rather than a number copied into this test.
const PROBE: &str = r#"
@group(0) @binding(0) var<storage, read_write> result: array<vec4<f32>>;
fn read(id: u32, footprint: vec2<f32>) -> vec4<f32> {
    let i = id % 256u;
    let j = f32(id / 256u);
    let circle = vec2(cos(f32(i) * 0.02454369), sin(f32(i) * 0.02454369));
    let share = SHARE;
    let row = vec4(30.0, 0.05, 1.0, -1.0);
    let lichen = lichen_layer(circle * (3.1 + 0.0917 * j), 0.173 * j, footprint, 1.0, share, 0.0);
    let dash = lenticel_dash(circle, 0.0087 * j, 0.12 + 0.0031 * j, footprint, row);
    let plate = vec4(0.032, 0.0, 0.0, 0.0);
    let strip = vec3(0.0, 0.0, share);
    let peel = bark_plate_field(circle * (5.0 + 0.1 * j), 0.0157 * j, 0.032, 1.0, footprint,
        vec2(0.5), plate, strip).relief;
    return vec4(lichen, dash.x, peel, dash.y);
}
@compute @workgroup_size(64) fn main(@builtin(global_invocation_id) id: vec3<u32>) {
    result[id.x] = read(id.x, vec2(0.0));
    if (id.x == 0u) { result[65536u] = read(0u, vec2(1.0e3)); }
}
"#;

fn sample(share: f32) -> Option<(Vec<[f32; 4]>, [f32; 4])> {
    let source = include_str!("../src/shaders/common.wgsl").replace(
        "@group(0) @binding(0) var<uniform> u: Uniforms;",
        "var<private> u: Uniforms;",
    ) + include_str!("../src/shaders/bark.wgsl")
        + include_str!("../src/shaders/plates.wgsl")
        + include_str!("../src/shaders/smooth.wgsl")
        + &PROBE.replace("SHARE", &format!("{share:?}"));
    let module = wgpu::naga::front::wgsl::parse_str(&source)
        .unwrap_or_else(|e| panic!("{}", e.emit_to_string(&source)));
    wgpu::naga::valid::Validator::new(
        wgpu::naga::valid::ValidationFlags::all(),
        wgpu::naga::valid::Capabilities::all(),
    )
    .validate(&module)
    .unwrap_or_else(|e| panic!("{}", e.emit_to_string(&source)));
    let gpu = common::gpu()?;
    let shader = gpu
        .device
        .create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("smooth bark means"),
            source: wgpu::ShaderSource::Wgsl(source.into()),
        });
    let pipeline = gpu
        .device
        .create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: None,
            layout: None,
            module: &shader,
            entry_point: Some("main"),
            compilation_options: Default::default(),
            cache: None,
        });
    let bytes = (65536 + 1) * 16;
    let usage = wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC;
    let output = gpu.device.create_buffer(&wgpu::BufferDescriptor {
        label: None,
        size: bytes,
        usage,
        mapped_at_creation: false,
    });
    let read = gpu.device.create_buffer(&wgpu::BufferDescriptor {
        label: None,
        size: bytes,
        usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });
    let group = gpu.device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: None,
        layout: &pipeline.get_bind_group_layout(0),
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: output.as_entire_binding(),
        }],
    });
    let mut encoder = gpu.device.create_command_encoder(&Default::default());
    {
        let mut pass = encoder.begin_compute_pass(&Default::default());
        pass.set_pipeline(&pipeline);
        pass.set_bind_group(0, &group, &[]);
        pass.dispatch_workgroups(1024, 1, 1);
    }
    encoder.copy_buffer_to_buffer(&output, 0, &read, 0, bytes);
    gpu.queue.submit([encoder.finish()]);
    let (send, recv) = std::sync::mpsc::channel();
    read.slice(..)
        .map_async(wgpu::MapMode::Read, move |r| send.send(r).unwrap());
    gpu.device
        .poll(wgpu::PollType::wait_indefinitely())
        .unwrap();
    recv.recv().unwrap().unwrap();
    let mapped = read.slice(..).get_mapped_range().unwrap();
    let rows: &[[f32; 4]] = bytemuck::cast_slice(&mapped);
    Some((rows[..65536].to_vec(), rows[65536]))
}

#[test]
fn every_smooth_layer_fades_to_the_mean_its_field_averages_to() {
    let mut worst = Vec::new();
    for share in [0.25_f32, 0.5, 0.75, 1.0] {
        let Some((values, far)) = sample(share) else {
            return;
        };
        let mean =
            |c: usize| values.iter().map(|v| f64::from(v[c])).sum::<f64>() / values.len() as f64;
        let (lichen, lenticel, peel, groove) = (mean(0), mean(1), mean(2), mean(3));
        eprintln!(
            "share {share}: lichen {lichen:.4} far {:.4}, lenticel {lenticel:.4} far {:.4}, \
             its groove {groove:.4} far {:.4}, a strip curled {share} {peel:.4} far {:.4}",
            far[0], far[1], far[3], far[2]
        );
        for (name, near, far, tolerance) in [
            ("lichen", lichen, far[0], 0.015),
            ("lenticel", lenticel, far[1], 0.004),
            ("a lenticel's groove", groove, far[3], 0.004),
            ("a curled strip", peel, far[2], 0.02),
        ] {
            if (near - f64::from(far)).abs() > tolerance {
                worst.push(format!("{name} at {share}: near {near:.4}, far {far:.4}"));
            }
        }
    }
    assert!(worst.is_empty(), "{}", worst.join("; "));
}
