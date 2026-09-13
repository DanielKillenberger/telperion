//! Evaluate the production blade term on the GPU with known directions and visibility.
mod common;

#[test]
fn shadow_blocks_transmission_and_a_lit_backface_obeys_the_row() {
    let Some(gpu) = common::gpu() else { return };
    let source = include_str!("../src/shaders/transmission.wgsl").to_owned()
        + r#"
@group(0) @binding(0) var<storage, read_write> result: array<vec4<f32>>;
@compute @workgroup_size(1) fn main() {
    let n = vec3<f32>(0.0, 0.0, -1.0);
    let eye = n;
    let sun = -n;
    let tint = vec3<f32>(0.2, 0.6, 0.1);
    result[0] = vec4(transmitted(n, eye, sun, tint, 0.8, 0.5, 0.0), 0.0);
    result[1] = vec4(transmitted(n, eye, sun, tint, 0.8, 0.5, 1.0), 0.0);
    result[2] = vec4(transmitted(n, eye, sun, tint, 0.4, 0.5, 1.0), 0.0);
    result[3] = vec4(transmitted(n, eye, sun, tint, 0.8, 1.0, 1.0), 0.0);
    result[4] = vec4(transmitted(n, eye, -sun, tint, 0.8, 0.5, 1.0), 0.0);
    result[5] = vec4(transmitted(n, eye, sun, tint, 0.8, 0.5, 0.5), 0.0);
}
"#;
    let shader = gpu
        .device
        .create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("transmission contract"),
            source: wgpu::ShaderSource::Wgsl(source.into()),
        });
    let pipeline = gpu
        .device
        .create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some("transmission contract"),
            layout: None,
            module: &shader,
            entry_point: Some("main"),
            compilation_options: Default::default(),
            cache: None,
        });
    let output = gpu.device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("transmission values"),
        size: 96,
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
        mapped_at_creation: false,
    });
    let readback = gpu.device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("transmission readback"),
        size: 96,
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
        pass.dispatch_workgroups(1, 1, 1);
    }
    encoder.copy_buffer_to_buffer(&output, 0, &readback, 0, 96);
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
    assert_eq!(rows[0], [0.0; 4], "shadowed fragment");
    assert_eq!(rows[4], [0.0; 4], "sun on the visible face");
    for (channel, tint) in [0.2, 0.6, 0.1].into_iter().enumerate() {
        let expected = tint * 0.8 * (-0.5_f32).exp();
        assert!(
            (rows[1][channel] - expected).abs() < 1e-6,
            "lit backface tint {channel}"
        );
        assert!((rows[2][channel] - expected * 0.5).abs() < 1e-6, "strength");
        assert!(
            (rows[3][channel] - expected * (-0.5_f32).exp()).abs() < 1e-6,
            "thickness"
        );
        assert!(
            (rows[5][channel] - expected * 0.5).abs() < 1e-6,
            "filtered shadow"
        );
    }
}
