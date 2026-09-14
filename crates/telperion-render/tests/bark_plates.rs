//! The plate network's own contracts: the mean the far path returns is the
//! mean the near path averages to, and one plate keeps one identity.
mod common;

/// The two shipped plate rows, as `(name, cell scale, elongation)`. The mean
/// the far path returns is one constant for every row, so both are measured
/// against it: a long oak plate and a nearly round spruce scale.
const ROWS: [(&str, f32, f32); 2] = [("oak", 0.09, 1.8), ("spruce", 0.022, 0.2)];

/// A wide sweep of the field at zero footprint, three profiles at a time:
/// the bare face, the face with its dome, and the face with its rim lifted.
/// `w` carries the plate's identity so the same pass can be read for it.
const PROBE: &str = r#"
@group(0) @binding(0) var<storage, read_write> result: array<vec4<f32>>;
@compute @workgroup_size(64) fn main(@builtin(global_invocation_id) id: vec3<u32>) {
    let across = f32(id.x % 128u) * (SIZE * 0.31);
    let along = f32(id.x / 128u) * (SIZE * (1.0 + LONG) * 0.30);
    let arc = vec2(across, 0.37) / 0.032;
    let bare = bark_plate_field(arc, along, 0.032, 1.0, vec2(0.0), vec2(0.5),
        vec4(SIZE, LONG, 0.0, 0.0), 0.0);
    let domed = bark_plate_field(arc, along, 0.032, 1.0, vec2(0.0), vec2(0.5),
        vec4(SIZE, LONG, 1.0, 0.0), 0.0);
    let lifted = bark_plate_field(arc, along, 0.032, 1.0, vec2(0.0), vec2(0.5),
        vec4(SIZE, LONG, 0.0, 1.0), 0.0);
    result[id.x] = vec4(bare.x, domed.x, lifted.x, bare.y);
}
"#;

fn sample(size: f32, long: f32) -> Vec<[f32; 4]> {
    let probe = PROBE
        .replace("SIZE", &format!("{size:?}"))
        .replace("LONG", &format!("{long:?}"));
    let source = include_str!("../src/shaders/common.wgsl").replace(
        "@group(0) @binding(0) var<uniform> u: Uniforms;",
        "var<private> u: Uniforms;",
    ) + include_str!("../src/shaders/bark.wgsl")
        + &probe;
    let module = wgpu::naga::front::wgsl::parse_str(&source)
        .unwrap_or_else(|e| panic!("{}", e.emit_to_string(&source)));
    wgpu::naga::valid::Validator::new(
        wgpu::naga::valid::ValidationFlags::all(),
        wgpu::naga::valid::Capabilities::all(),
    )
    .validate(&module)
    .unwrap_or_else(|e| panic!("{}", e.emit_to_string(&source)));
    let Some(gpu) = common::gpu() else {
        return Vec::new();
    };
    let shader = gpu
        .device
        .create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("bark plate network"),
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
    let count = 128 * 128u64;
    let bytes = count * 16;
    let output = gpu.device.create_buffer(&wgpu::BufferDescriptor {
        label: None,
        size: bytes,
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
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
    let mut encoder = gpu
        .device
        .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
    {
        let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
            label: None,
            timestamp_writes: None,
        });
        pass.set_pipeline(&pipeline);
        pass.set_bind_group(0, &group, &[]);
        pass.dispatch_workgroups(count as u32 / 64, 1, 1);
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
    rows.to_vec()
}

#[test]
fn the_pinned_plate_mean_is_the_mean_the_field_averages_to() {
    // The far path returns bark_plate_mean() outright and the near path
    // averages to it. A drift between them is a step in brightness as a trunk
    // recedes, which no tolerance in the distance tests would explain.
    let source = include_str!("../src/shaders/bark.wgsl");
    let pinned = |name: &str| {
        source
            .split_once(&format!("const {name} = "))
            .and_then(|(_, rest)| rest.split_once(';'))
            .map(|(value, _)| value.trim().parse::<f64>().unwrap())
            .unwrap_or_else(|| panic!("{name} is not pinned in bark.wgsl"))
    };
    for (row, size, long) in ROWS {
        let values = sample(size, long);
        if values.is_empty() {
            return;
        }
        let mean = |channel: usize| {
            values.iter().map(|v| f64::from(v[channel])).sum::<f64>() / values.len() as f64
        };
        let (face, domed, lifted) = (mean(0), mean(1), mean(2));
        eprintln!(
            "{row}: BARK_PLATE_FACE {face:.4}, BARK_PLATE_DOME {:.4}, BARK_PLATE_RIM {:.4}",
            domed - face,
            lifted - face
        );
        for (name, measured) in [
            ("BARK_PLATE_FACE", face),
            ("BARK_PLATE_DOME", domed - face),
            ("BARK_PLATE_RIM", lifted - face),
        ] {
            let pin = pinned(name);
            assert!(
                (pin - measured).abs() <= 0.02,
                "{name} is pinned at {pin} and the {row} field averages {measured}"
            );
        }
    }
}

#[test]
fn every_plate_carries_an_identity_of_its_own() {
    let (_, size, long) = ROWS[0];
    let values = sample(size, long);
    if values.is_empty() {
        return;
    }
    // One plate is one value. Neighbouring samples a millimetre apart are
    // almost always the same plate; samples a whole plate apart are not.
    let identity = |i: usize| f64::from(values[i][3]);
    let width = 128;
    let near = (0..values.len() - 1)
        .filter(|i| (i + 1) % width != 0)
        .filter(|&i| (identity(i) - identity(i + 1)).abs() < 1e-6)
        .count();
    let apart = (0..values.len() - 4)
        .filter(|i| (i + 4) % width > 3)
        .filter(|&i| (identity(i) - identity(i + 4)).abs() < 1e-6)
        .count();
    let spread = {
        let mean = values.iter().map(|v| f64::from(v[3])).sum::<f64>() / values.len() as f64;
        values
            .iter()
            .map(|v| (f64::from(v[3]) - mean).powi(2))
            .sum::<f64>()
            / values.len() as f64
    };
    eprintln!("identity: {near} adjacent pairs share a plate, {apart} at a plate's width, variance {spread:.4}");
    // The sweep steps a third of a plate at a time, so a neighbouring pair
    // usually sits on one plate and a pair four steps apart almost never
    // does. A hash of the position would share nothing at either distance.
    assert!(
        near > values.len() / 2,
        "a plate does not hold one identity across its own face: {near} of {}",
        values.len()
    );
    assert!(
        (apart as f64) < 0.5 * near as f64,
        "neighbouring plates carry the same identity: {apart} against {near}"
    );
    // A field of one value would pass both counts above and say nothing.
    assert!(
        spread > 0.05,
        "the identity does not vary: variance {spread}"
    );
}
