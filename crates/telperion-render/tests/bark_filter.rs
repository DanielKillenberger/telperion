//! The production field must filter either metre coordinate independently.
mod common;

#[test]
fn either_unresolved_axis_leaves_little_relief_and_furrows_can_close() {
    // The field now takes its noise from the same prelude as lit stages.
    let source = include_str!("../src/shaders/common.wgsl").replace(
        "@group(0) @binding(0) var<uniform> u: Uniforms;",
        "var<private> u: Uniforms;",
    ) + include_str!("../src/shaders/bark.wgsl")
        + include_str!("../src/shaders/plates.wgsl")
        + r#"
@group(0) @binding(0) var<storage, read_write> result: array<vec4<f32>>;
@compute @workgroup_size(64) fn main(@builtin(global_invocation_id) id: vec3<u32>) {
    let angle = f32(id.x % 64u) * 0.013;
    let along = 1.0 + f32(id.x / 64u) * 0.0027;
    let circle = vec2(cos(angle), sin(angle));
    result[id.x] = vec4(
        bark_field_filtered(circle, along, 0.4, 0.014, 0.022, vec2(0.0), 1.0, vec4(0.0), vec3(0.0)),
        bark_field_filtered(circle, along, 0.4, 0.014, 0.022, vec2(0.5, 0.0), 1.0, vec4(0.0), vec3(0.0)),
        bark_field_filtered(circle, along, 0.4, 0.014, 0.022, vec2(0.0, 0.5), 1.0, vec4(0.0), vec3(0.0)),
        bark_field_filtered(circle, along, 0.4, 0.014, 0.022, vec2(0.0), 0.0, vec4(0.0), vec3(0.0)));
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
            label: Some("bark filtering contract"),
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
    let output = gpu.device.create_buffer(&wgpu::BufferDescriptor {
        label: None,
        size: 4096 * 16,
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
        mapped_at_creation: false,
    });
    let readback = gpu.device.create_buffer(&wgpu::BufferDescriptor {
        label: None,
        size: output.size(),
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
    encoder.copy_buffer_to_buffer(&output, 0, &readback, 0, output.size());
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
    let deviations = std::array::from_fn::<_, 4, _>(|c| {
        let mean = rows.iter().map(|r| f64::from(r[c])).sum::<f64>() / rows.len() as f64;
        (rows
            .iter()
            .map(|r| (f64::from(r[c]) - mean).powi(2))
            .sum::<f64>()
            / rows.len() as f64)
            .sqrt()
    });
    eprintln!("height deviations: resolved / unresolved arc / unresolved axial / closed furrows {deviations:?}");
    assert!(rows.iter().flatten().all(|v| v.is_finite()));
    assert!(deviations[0] > 0.0001, "the resolved field lost its relief");
    // The field leaves by its edge integrals alone (fn-71): its caller reads
    // it under a ridge width a cell, and a box of thirty-six widths on either
    // axis, which no caller reads, keeps under a tenth of the resolved relief.
    assert!(
        deviations[1] < deviations[0] * 0.1 && deviations[2] < deviations[0] * 0.1,
        "an unresolved coordinate still modulates height: {deviations:?}"
    );
    assert!(
        deviations[3] > deviations[0] * 0.05 && deviations[3] < deviations[0] * 0.6,
        "closing furrows must retain low scales independently: {deviations:?}"
    );
}
