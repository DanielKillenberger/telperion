//! Put the production canopy terms to a synthetic leaf facing the sun and
//! facing away from it, on the GPU, with every direction and value known.
mod common;

/// Runs `body` as the only statement block of a one-thread compute stage over
/// the canopy file, and reads back the `vec4` rows it wrote.
fn evaluate(gpu: &telperion_render::Gpu, rows: u64, body: &str) -> Vec<[f32; 4]> {
    let source = format!(
        "{}\n@group(0) @binding(0) var<storage, read_write> result: array<vec4<f32>>;\n\
         @compute @workgroup_size(1) fn main() {{\n{body}\n}}\n",
        include_str!("../src/shaders/canopy.wgsl")
    );
    let shader = gpu
        .device
        .create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("canopy contract"),
            source: wgpu::ShaderSource::Wgsl(source.into()),
        });
    let pipeline = gpu
        .device
        .create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some("canopy contract"),
            layout: None,
            module: &shader,
            entry_point: Some("main"),
            compilation_options: Default::default(),
            cache: None,
        });
    let size = rows * 16;
    let buffer = |usage, label| {
        gpu.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some(label),
            size,
            usage,
            mapped_at_creation: false,
        })
    };
    let output = buffer(
        wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
        "canopy values",
    );
    let readback = buffer(
        wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
        "canopy readback",
    );
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
    encoder.copy_buffer_to_buffer(&output, 0, &readback, 0, size);
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
    bytemuck::cast_slice::<u8, [f32; 4]>(&mapped).to_vec()
}

fn close(a: f32, b: f32) -> bool {
    (a - b).abs() < 1e-5
}

#[test]
fn the_wrap_lights_past_the_terminator_and_leaves_a_square_face_alone() {
    let Some(gpu) = common::gpu() else { return };
    let rows = evaluate(
        &gpu,
        2,
        "result[0] = vec4(wrapped(1.0, 0.0), wrapped(0.0, 0.0), wrapped(-0.3, 0.0), wrapped(0.6, 0.0));\n\
         result[1] = vec4(wrapped(1.0, 0.5), wrapped(0.0, 0.5), wrapped(-0.3, 0.5), wrapped(-0.6, 0.5));",
    );
    // No wrap is the plain cosine, to the bit.
    assert_eq!(rows[0], [1.0, 0.0, 0.0, 0.6]);
    // Facing the sun: all of it. Edge-on: w / (1 + w). Turned away by less
    // than the wrap: some. Turned away by more: none.
    let [square, edge, behind, beyond] = rows[1];
    assert!(close(square, 1.0), "{square}");
    assert!(close(edge, 0.5 / 1.5), "{edge}");
    assert!(close(behind, 0.2 / 1.5), "{behind}");
    assert_eq!(beyond, 0.0);
}

#[test]
fn the_canopy_normal_turns_a_leaf_facing_away_toward_a_sunward_shell() {
    let Some(gpu) = common::gpu() else { return };
    // The face turned away from a sun at +x, on the crown's sunward shell.
    let rows = evaluate(
        &gpu,
        5,
        "let face = normalize(vec3<f32>(-0.8, 0.0, 0.6));\n\
         let outward = crown_outward(vec3<f32>(3.0, 5.0, 0.0), vec3<f32>(0.0, 5.0, 0.0), vec3<f32>(2.0, 4.0, 2.0));\n\
         result[0] = vec4(outward, 0.0);\n\
         result[1] = vec4(canopy_normal(face, outward, 0.0), 0.0);\n\
         result[2] = vec4(canopy_normal(face, outward, 0.5), 0.0);\n\
         result[3] = vec4(canopy_normal(face, outward, 1.0), 0.0);\n\
         result[4] = vec4(canopy_normal(face, vec3<f32>(0.0), 1.0), \
             length(crown_outward(vec3<f32>(0.0, 5.0, 0.0), vec3<f32>(0.0, 5.0, 0.0), vec3<f32>(2.0))));",
    );
    let near = |row: [f32; 4], v: [f32; 3]| (0..3).all(|i| close(row[i], v[i]));
    assert!(
        near(rows[0], [1.0, 0.0, 0.0]),
        "the shell at +x faces +x: {:?}",
        rows[0]
    );
    assert_eq!(rows[1][..3], [-0.8, 0.0, 0.6], "no bend is the face");
    let sun = |row: [f32; 4]| row[0];
    assert!(
        sun(rows[2]) > 0.0,
        "half a bend turns it sunward: {:?}",
        rows[2]
    );
    assert!(close(
        rows[2].iter().take(3).map(|v| v * v).sum::<f32>(),
        1.0
    ));
    assert!(
        near(rows[3], [1.0, 0.0, 0.0]),
        "a full bend is the shell: {:?}",
        rows[3]
    );
    assert_eq!(rows[4][..3], [-0.8, 0.0, 0.6], "no outward leaves the face");
    assert_eq!(rows[4][3], 0.0, "the centre has no outward");
}

#[test]
fn diffuse_transmission_reaches_a_leaf_lit_from_behind_at_any_angle() {
    let Some(gpu) = common::gpu() else { return };
    let rows = evaluate(
        &gpu,
        4,
        "let to_sun = vec3<f32>(1.0, 0.0, 0.0);\n\
         let sun = vec3<f32>(3.0);\n\
         let sky = vec3<f32>(0.5);\n\
         let leaf = vec3<f32>(0.1, 0.2, 0.05);\n\
         let away = normalize(vec3<f32>(-0.6, 0.0, 0.8));\n\
         result[0] = vec4(diffuse_through(away, to_sun, sun, 1.0, sky, leaf), 0.0);\n\
         result[1] = vec4(diffuse_through(-away, to_sun, sun, 1.0, sky, leaf), 0.0);\n\
         result[2] = vec4(diffuse_through(away, to_sun, sun, 0.0, sky, leaf), 0.0);\n\
         result[3] = vec4(diffuse_through(away, to_sun, sun, 1.0, vec3<f32>(0.0), leaf), 0.0);",
    );
    for (channel, t) in [0.1_f32, 0.2, 0.05].into_iter().enumerate() {
        // Facing away from the sun: the sun through the far face at its
        // cosine, and the sky behind, whatever the eye's angle.
        assert!(
            close(rows[0][channel], t * (3.0 * 0.6 + 0.5)),
            "away {channel}"
        );
        // Facing the sun: nothing of it passes; the sky behind still does.
        assert!(close(rows[1][channel], t * 0.5), "facing {channel}");
        // The sun's map gates the sun and not the sky.
        assert!(close(rows[2][channel], t * 0.5), "shadowed {channel}");
        assert!(close(rows[3][channel], t * 1.8), "no sky {channel}");
    }
}

#[test]
fn the_sheen_rises_from_its_reflectance_to_all_of_the_sky_at_grazing() {
    let Some(gpu) = common::gpu() else { return };
    let rows = evaluate(
        &gpu,
        2,
        "let n = vec3<f32>(0.0, 0.0, 1.0);\n\
         let square = vec3<f32>(0.0, 0.0, 1.0);\n\
         let oblique = normalize(vec3<f32>(0.0, 1.0, 1.0));\n\
         let grazing = vec3<f32>(0.0, 1.0, 0.0);\n\
         result[0] = vec4(sheen(n, square, 0.0), sheen(n, oblique, 0.0), sheen(n, grazing, 0.0), 0.0);\n\
         result[1] = vec4(sheen(n, square, 0.06), sheen(n, oblique, 0.06), sheen(n, grazing, 0.06), sheen(-n, square, 0.06));",
    );
    assert_eq!(rows[0], [0.0; 4], "no reflectance reflects nothing");
    let [square, oblique, grazing, behind] = rows[1];
    assert!(close(square, 0.06), "{square}");
    assert!(square < oblique && oblique < grazing, "{:?}", rows[1]);
    assert!(close(grazing, 1.0), "{grazing}");
    assert!(close(behind, 1.0), "a face turned away is grazing at most");
}
