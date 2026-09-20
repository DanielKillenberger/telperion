//! Analytic intersections exercise the actual production coordinate walk.
mod common;

#[test]
fn parallax_descends_away_from_the_eye_and_refines_the_intersection() {
    let wood = include_str!("../src/shaders/wood.wgsl");
    let start = wood.find("fn bark_parallax(").unwrap();
    let end = start + wood[start..].find("\n}").unwrap() + 2;
    let source = format!(
        r#"
struct Uniforms {{ eye: vec4<f32>, bark_structure: vec4<f32> }};
var<private> u: Uniforms;
var<private> slope: f32;
fn bark_groove(circle: vec2<f32>, along: f32, radius: f32, footprint: vec2<f32>) -> f32 {{
    return 0.0;
}}
fn bark_height(circle: vec2<f32>, along: f32, radius: f32, footprint: vec2<f32>,
    groove: f32) -> f32 {{ return slope * along; }}
{}
@group(0) @binding(0) var<storage, read_write> result: array<vec4<f32>>;
fn walk() -> vec3<f32> {{
    return bark_parallax(vec3(0.0, 1.0, 0.0), vec3(1.0, 0.0, 0.0),
        vec3(0.0, 0.0, 1.0), vec3(0.0, 0.0, 1.0),
        vec3(1.0, 0.0, 0.0), vec3(0.0, 1.0, 0.0), vec3(0.0),
        0.0, vec2(0.0, 0.01), 0.4, vec2(0.001));
}}
@compute @workgroup_size(1) fn main() {{
    u.eye = vec4(1.0, 0.0, 1.0, 0.0);
    u.bark_structure = vec4(0.0, 0.0, 1.0, 0.0);
    slope = 0.0;
    result[0] = vec4(walk(), 0.0);
    u.bark_structure.z = 0.0;
    result[1] = vec4(walk(), 0.0);
    u.bark_structure.z = 1.0;
    u.eye = vec4(0.0, 0.0, 1.0, 0.0);
    result[2] = vec4(walk(), 0.0);
    u.eye = vec4(1.0, 0.0, 1.0, 0.0);
    slope = 0.5;
    result[3] = vec4(walk(), 0.0);
}}
"#,
        &wood[start..end]
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
            label: Some("production parallax analytic contract"),
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
        size: 64,
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
        mapped_at_creation: false,
    });
    let readback = gpu.device.create_buffer(&wgpu::BufferDescriptor {
        label: None,
        size: 64,
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
    encoder.copy_buffer_to_buffer(&output, 0, &readback, 0, 64);
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
    eprintln!("flat, zero strength, head-on, ramp: {values:?}");
    assert!(
        (values[0][0] + 0.01).abs() < 1e-6,
        "flat floor must move away from eye"
    );
    assert_eq!(values[1], [0.0, 1.0, 0.0, 0.0], "zero strength");
    assert_eq!(values[2], [0.0, 1.0, 0.0, 0.0], "head-on view");
    let residual = |x: f32| (x + 0.01 - 0.5 * x).abs();
    let first = residual(-0.01);
    let refined = residual(values[3][0]);
    eprintln!("ramp residual: first {first}, refined {refined}");
    assert!(
        refined < first * 0.6,
        "two corrections must improve ramp intersection"
    );
}
