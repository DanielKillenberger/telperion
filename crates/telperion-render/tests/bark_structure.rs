//! Physical field regression: girth strengthens relief and grain stays axial.
mod common;

#[test]
fn mature_girth_strengthens_an_axial_field() {
    let Some(gpu) = common::gpu() else { return };
    let source = include_str!("../src/shaders/bark.wgsl").to_owned()
        + r#"
@group(0) @binding(0) var<storage, read_write> result: array<vec4<f32>>;
fn sample(x: f32, y: f32, radius: f32) -> f32 {
    let angle = x / radius;
    return bark_field(vec2(cos(angle), sin(angle)), y, radius, 0.032, 0.055);
}
@compute @workgroup_size(64) fn main(@builtin(global_invocation_id) id: vec3<u32>) {
    let x = f32(id.x % 128u) * 0.01;
    let y = 1.0 + f32(id.x / 128u) * 0.01;
    result[id.x] = vec4(sample(x, y, 0.12), sample(x, y, 0.6),
        sample(x + 0.0005, y, 0.6), sample(x, y + 0.0005, 0.6));
}
"#;
    let shader = gpu
        .device
        .create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("bark structure"),
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
        size: 16384 * 16,
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
        pass.dispatch_workgroups(256, 1, 1);
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
    let deviation = |channel: usize| {
        let mean = rows.iter().map(|r| f64::from(r[channel])).sum::<f64>() / rows.len() as f64;
        (rows
            .iter()
            .map(|r| (f64::from(r[channel]) - mean).powi(2))
            .sum::<f64>()
            / rows.len() as f64)
            .sqrt()
    };
    let across: f64 = rows.iter().map(|r| f64::from((r[2] - r[1]).abs())).sum();
    let along: f64 = rows.iter().map(|r| f64::from((r[3] - r[1]).abs())).sum();
    let girth = deviation(1) / deviation(0);
    eprintln!(
        "mature trunk/branch height deviation {girth}; across/along slope {}",
        across / along
    );
    assert!(
        girth > 1.3,
        "mature trunk relief must exceed mature branch relief: {girth}"
    );
    assert!(
        across > along * 1.3,
        "grain must be longer along the run: {across}/{along}"
    );
    assert!(rows.iter().flatten().all(|h| h.is_finite()));
}
