//! Sample the production bark field in surface metres, independently of a camera.
mod common;

#[test]
fn surface_lengths_young_wood_wrap_and_plate_drift() {
    // The field now takes its noise from the same prelude as lit stages.
    let source = include_str!("../src/shaders/common.wgsl").replace(
        "@group(0) @binding(0) var<uniform> u: Uniforms;",
        "var<private> u: Uniforms;",
    ) + include_str!("../src/shaders/bark.wgsl")
        + r#"
@group(0) @binding(0) var<storage, read_write> result: array<vec4<f32>>;
@compute @workgroup_size(64) fn main(@builtin(global_invocation_id) id: vec3<u32>) {
    let angle = f32(id.x) * 6.2831853 / 4096.0;
    let circle = vec2(cos(angle), sin(angle));
    result[id.x] = vec4(
        bark_field(circle, 2.0, 0.4, 0.04, 0.18),
        bark_field(circle, 2.0, 0.8, 0.04, 0.18),
        bark_field(circle, 2.0, 0.01, 0.04, 0.18),
        bark_field(circle, 2.19, 0.4, 0.04, 0.18));
    if (id.x == 0u) {
        result[4096] = vec4(
            bark_field(vec2(-1.0, 0.000001), 2.0, 0.4, 0.04, 0.18),
            bark_field(vec2(-1.0, -0.000001), 2.0, 0.4, 0.04, 0.18),
            bark_field(circle, 2.0, 0.4, 0.0, 0.18),
            bark_field(circle, 2.0, 0.4, 0.0, 0.0));
    }
}
"#;
    let module = wgpu::naga::front::wgsl::parse_str(&source)
        .unwrap_or_else(|e| panic!("{}", e.emit_to_string(&source)));
    wgpu::naga::valid::Validator::new(
        wgpu::naga::valid::ValidationFlags::all(),
        wgpu::naga::valid::Capabilities::all(),
    )
    .validate(&module)
    .unwrap_or_else(|e| panic!("{}", e.emit_to_string(&source)));
    let Some(gpu) = common::gpu() else { return };
    let shader = gpu
        .device
        .create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("bark field contract"),
            source: wgpu::ShaderSource::Wgsl(source.into()),
        });
    let pipeline = gpu
        .device
        .create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some("bark field contract"),
            layout: None,
            module: &shader,
            entry_point: Some("main"),
            compilation_options: Default::default(),
            cache: None,
        });
    let output = gpu.device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("bark field values"),
        size: 4097 * 16,
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
        mapped_at_creation: false,
    });
    let readback = gpu.device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("bark field readback"),
        size: 4097 * 16,
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
        pass.dispatch_workgroups(64, 1, 1);
    }
    encoder.copy_buffer_to_buffer(&output, 0, &readback, 0, 4097 * 16);
    gpu.queue.submit([encoder.finish()]);
    let (send, recv) = std::sync::mpsc::channel();
    readback
        .slice(..)
        .map_async(wgpu::MapMode::Read, move |r| send.send(r).unwrap());
    gpu.device
        .poll(wgpu::PollType::wait_indefinitely())
        .unwrap();
    recv.recv().unwrap().unwrap();
    let mapped = readback.slice(..).get_mapped_range().unwrap();
    let rows: &[[f32; 4]] = bytemuck::cast_slice(&mapped);
    let field = &rows[..4096];
    assert!(
        field.iter().all(|r| r[2] == 0.0),
        "young wood still has relief"
    );
    eprintln!("wrap samples: {:?}", rows[4096]);
    assert!((rows[4096][0] - rows[4096][1]).abs() < 1e-6, "angular wrap");
    assert_eq!(&rows[4096][2..], &[0.0, 0.0], "disabled ridge field");
    let peaks = |channel: usize| {
        (0..4096)
            .filter(|&i| {
                field[i][channel] > field[(i + 4095) % 4096][channel]
                    && field[i][channel] > field[(i + 1) % 4096][channel]
            })
            .count()
    };
    let (narrow, wide) = (peaks(0), peaks(1));
    println!("ridge peaks around 0.4/0.8 m radii: {narrow}/{wide}");
    assert!(narrow > 20, "mature wood needs resolved ridges");
    assert!(
        wide as f32 / narrow as f32 > 1.6 && (wide as f32 / narrow as f32) < 2.4,
        "doubling circumference must double the ridge count: {narrow}/{wide}"
    );
    let drift: f32 = field.iter().map(|r| (r[0] - r[3]).abs()).sum();
    assert!(drift > 0.1, "plates must change along the run: {drift}");
}
