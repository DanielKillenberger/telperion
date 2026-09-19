//! Measure actual filtered height in metres, independent of colour and lighting.
mod common;
use telperion_core::{material::MaterialParams, presets::Preset};

const SIDE: usize = 128;
const COUNT: usize = SIDE * SIDE;

fn sample_function(name: &str, m: MaterialParams) -> String {
    format!(
        "fn {name}(circle: vec2<f32>, along: f32) -> f32 {{\n\
         return bark_field_filtered(circle, along, 0.4, {:.9}, {:.9}, vec2(0.001), {:.9},\n\
         vec4({:.9}, {:.9}, {:.9}, {:.9}), vec3({:.9}, {:.9}, {:.9}));\n}}\n",
        m.ridge_scale,
        m.plate_scale,
        m.furrow_strength,
        m.plate_cell_scale,
        m.plate_elongation,
        m.plate_dome,
        m.plate_edge_lift,
        m.plate_identity,
        m.plate_furrow_width,
        m.peel_curl,
    )
}

#[test]
fn oak_and_spruce_profiles_increase_resolved_height_range() {
    let oak = Preset::OregonWhiteOak.parameters().material;
    let spruce = Preset::NorwaySpruce.parameters().material;
    let prior_oak = MaterialParams {
        plate_dome: 0.41875,
        plate_edge_lift: 0.27,
        plate_furrow_width: 0.0,
        ..oak
    };
    let prior_spruce = MaterialParams {
        plate_dome: 0.39375,
        plate_edge_lift: 0.5,
        plate_furrow_width: 0.0,
        ..spruce
    };
    let mut source = include_str!("../src/shaders/common.wgsl").replace(
        "@group(0) @binding(0) var<uniform> u: Uniforms;",
        "var<private> u: Uniforms;",
    );
    source.push_str(include_str!("../src/shaders/bark.wgsl"));
    source.push_str(include_str!("../src/shaders/plates.wgsl"));
    for (name, material) in [
        ("prior_oak", prior_oak),
        ("candidate_oak", oak),
        ("prior_spruce", prior_spruce),
        ("candidate_spruce", spruce),
    ] {
        source.push_str(&sample_function(name, material));
    }
    source.push_str(
        r#"
@group(0) @binding(0) var<storage, read_write> heights: array<vec4<f32>>;
@compute @workgroup_size(64) fn main(@builtin(global_invocation_id) id: vec3<u32>) {
    let x = -0.2 + (f32(id.x % 128u) + 0.5) * (0.4 / 128.0);
    let along = 1.8 + (f32(id.x / 128u) + 0.5) * (0.4 / 128.0);
    let circle = vec2(cos(x / 0.4), sin(x / 0.4));
    heights[id.x] = vec4(prior_oak(circle, along), candidate_oak(circle, along),
        prior_spruce(circle, along), candidate_spruce(circle, along));
}
"#,
    );
    let module = wgpu::naga::front::wgsl::parse_str(&source)
        .unwrap_or_else(|e| panic!("{}", e.emit_to_string(&source)));
    wgpu::naga::valid::Validator::new(
        wgpu::naga::valid::ValidationFlags::all(),
        wgpu::naga::valid::Capabilities::all(),
    )
    .validate(&module)
    .unwrap();
    let Some(gpu) = common::gpu() else { return };
    let shader = gpu
        .device
        .create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("filtered bark physical height ranges"),
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
    let bytes = (COUNT * 4 * size_of::<f32>()) as u64;
    let heights = gpu.device.create_buffer(&wgpu::BufferDescriptor {
        label: None,
        size: bytes,
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
        mapped_at_creation: false,
    });
    let readback = gpu.device.create_buffer(&wgpu::BufferDescriptor {
        label: None,
        size: bytes,
        usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });
    let bindings = gpu.device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: None,
        layout: &pipeline.get_bind_group_layout(0),
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: heights.as_entire_binding(),
        }],
    });
    let mut encoder = gpu.device.create_command_encoder(&Default::default());
    {
        let mut pass = encoder.begin_compute_pass(&Default::default());
        pass.set_pipeline(&pipeline);
        pass.set_bind_group(0, &bindings, &[]);
        pass.dispatch_workgroups((COUNT / 64) as u32, 1, 1);
    }
    encoder.copy_buffer_to_buffer(&heights, 0, &readback, 0, bytes);
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
    let values: &[[f32; 4]] = bytemuck::cast_slice(&mapped);
    assert_eq!(values.len(), COUNT);
    assert!(values.iter().flatten().all(|v| v.is_finite()));
    let ranges: [f32; 4] = std::array::from_fn(|channel| {
        let mut sorted: Vec<f32> = values.iter().map(|v| v[channel]).collect();
        sorted.sort_by(f32::total_cmp);
        sorted[(COUNT - 1) * 95 / 100] - sorted[(COUNT - 1) * 5 / 100]
    });
    for (species, before, after) in [
        ("oak", ranges[0], ranges[1]),
        ("spruce", ranges[2], ranges[3]),
    ] {
        eprintln!("{species} physical height p95-p5: before {before:.9} m, after {after:.9} m, ratio {:.6}; radius .4m, square .4m, footprint .001m, 128x128 samples", after / before);
        assert!(before > 0.0);
        assert!(
            after > before * 1.15,
            "{species} needs >15% greater resolved relief: {before} -> {after}"
        );
    }
}
