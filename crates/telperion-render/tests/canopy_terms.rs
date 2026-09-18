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

#[test]
fn the_crown_over_a_leaf_takes_its_share_of_the_sky_per_radius() {
    let Some(gpu) = common::gpu() else { return };
    // A crown of radii 2, 4 and 2 about (0, 5, 0), read straight up.
    let rows = evaluate(
        &gpu,
        2,
        "let c = vec3<f32>(0.0, 5.0, 0.0);\n\
         let r = vec3<f32>(2.0, 4.0, 2.0);\n\
         let up = vec3<f32>(0.0, 1.0, 0.0);\n\
         result[0] = vec4(crown_chord(vec3<f32>(0.0, 1.2, 0.0), up, c, r), \
             crown_chord(vec3<f32>(0.0, 5.0, 0.0), up, c, r), \
             crown_chord(vec3<f32>(0.0, 8.8, 0.0), up, c, r), \
             crown_chord(vec3<f32>(3.0, 5.0, 0.0), up, c, r));\n\
         result[1] = vec4(through_crown(1.9, 0.0), through_crown(1.0, 0.3), \
             through_crown(1.9, 0.3), through_crown(1.9, 1.0));",
    );
    let [under, centre, top, outside] = rows[0];
    // Under the crown the whole height stands over the leaf; at its centre
    // half of it; at its top almost none; beside it none.
    assert!(close(under, 1.95), "under {under}");
    assert!(close(centre, 1.0), "centre {centre}");
    assert!(close(top, 0.05), "top {top}");
    assert_eq!(outside, 0.0);
    let [none, half, most, all] = rows[1];
    assert_eq!(none, 1.0, "no shade takes nothing");
    assert!(close(half, 0.7) && close(most, 1.0 - 0.57), "{:?}", rows[1]);
    assert_eq!(
        all, 0.0,
        "a full shade over most of a diameter takes it all"
    );
}

#[test]
fn the_highlight_returns_at_most_its_fresnel_share_of_the_sun() {
    let Some(gpu) = common::gpu() else { return };
    // The lobe integrated over every direction the eye can take, weighted
    // by the eye's cosine and divided by pi: the share of the sun a surface
    // returns as its highlight, by reciprocity. A face at +z under a sun 27
    // degrees off it, seen square, oblique and at grazing, chalk and smooth.
    let rows = evaluate(
        &gpu,
        3,
        "let n = vec3<f32>(0.0, 0.0, 1.0);\n\
         let to_sun = normalize(vec3<f32>(0.0, 0.5, 1.0));\n\
         var views = array<vec3<f32>, 3>(n, normalize(vec3<f32>(0.0, 1.0, 1.0)), normalize(vec3<f32>(0.0, 5.0, 1.0)));\n\
         for (var r = 0; r < 2; r++) {\n\
             let roughness = select(0.95, 0.4, r == 1);\n\
             var row = vec4<f32>(0.0);\n\
             for (var i = 0; i < 90; i++) {\n\
                 let theta = (f32(i) + 0.5) * 0.017453292;\n\
                 for (var j = 0; j < 180; j++) {\n\
                     let phi = (f32(j) + 0.5) * 0.034906585;\n\
                     let to_eye = vec3<f32>(sin(theta) * cos(phi), sin(theta) * sin(phi), cos(theta));\n\
                     let h = highlight(n, to_sun, to_eye, 0.04, roughness);\n\
                     row.x += h.x * h.y * cos(theta) * sin(theta) * 0.017453292 * 0.034906585 / 3.14159265;\n\
                 }\n\
             }\n\
             for (var v = 0; v < 3; v++) {\n\
                 let h = highlight(n, to_sun, views[v], 0.04, roughness);\n\
                 row[v + 1] = h.x * h.y;\n\
             }\n\
             result[r] = row;\n\
         }\n\
         let mirror = normalize(vec3<f32>(0.0, -0.5, 1.0));\n\
         let none = highlight(n, to_sun, mirror, 0.0, 0.4);\n\
         result[2] = vec4(none.x, none.y, highlight(n, to_sun, mirror, 0.04, 0.4).y, highlight(n, to_sun, mirror, 0.04, 0.95).y);",
    );
    for (name, row) in [("chalk", rows[0]), ("smooth", rows[1])] {
        // The physical bound: over the hemisphere a dielectric returns a few
        // per cent of the sun and never more than its Fresnel share, where
        // the old lobes returned the gloss row's fraction of it outright.
        assert!(
            row[0] > 0.01 && row[0] <= 0.08,
            "{name} returns {} of the sun over the hemisphere",
            row[0]
        );
        // Toward the eye the lobe times its share is never more than the sun.
        assert!(
            row[1..].iter().all(|v| (0.0..=1.0).contains(v)),
            "{name}: {row:?}"
        );
    }
    let [none_x, none_y, smooth, chalk] = rows[2];
    assert_eq!(
        (none_x, none_y),
        (0.0, 0.0),
        "no reflectance mirrors nothing"
    );
    assert!(
        smooth > chalk,
        "a smooth lobe peaks above a chalk one: {smooth} {chalk}"
    );
}
