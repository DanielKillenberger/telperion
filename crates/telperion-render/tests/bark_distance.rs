//! Resolved centimetre scales must survive the distance-series footprints.
mod common;
#[path = "common/resolution.rs"]
mod resolution;
use telperion_core::{math::Vec3, mesh, presets::Preset};
use telperion_render::{render, Camera, Renderer, View, STILL_FORMAT};

#[test]
fn resolved_scales_survive_until_the_two_pixel_boundary() {
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
        bark_field_filtered(circle, along, 0.4, 0.032, 0.055, vec2(0.001414), 1.0, vec4(0.0), vec3(0.0)),
        bark_field_filtered(circle, along, 0.4, 0.032, 0.055, vec2(0.006514), 1.0, vec4(0.0), vec3(0.0)),
        bark_pass(0.5), bark_pass(1.0));
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
    eprintln!(
        "distance-series height deviation near/far: {}/{}",
        deviations[0], deviations[1]
    );
    assert!(rows.iter().flatten().all(|v| v.is_finite()));
    // At 4x, 32 mm columns still span 4.9 pixels at the facing surface.
    // Preserve at least a quarter of their resolved height variation.
    assert!(
        deviations[1] >= deviations[0] * 0.25,
        "resolved far scales vanished: {deviations:?}"
    );
    assert!(
        rows.iter().all(|r| r[2] == 1.0 && r[3] == 0.0),
        "whole bands must not fade before two pixels, and must vanish at one"
    );

    drop(mapped);
    readback.unmap();
    let mut renderer = Renderer::new(gpu, STILL_FORMAT);
    let mut rows = Vec::new();
    // The oak alone: the spruce's bare crown throws twig shadows over its
    // trunk at every height the series can frame, and a strip through them
    // measures shadow edges, not bark (fn-71: plain wood reads 6.2 at 2x).
    for (preset, height) in [(Preset::OregonWhiteOak, 2.0)] {
        let mut family = preset.parameters();
        family.skeleton.seed = 7;
        let tree = mesh::build(&family).unwrap();
        renderer.submit(&tree).unwrap();
        renderer.set_material(family.material);
        for distance in [2, 4] {
            let target = Vec3::new(0.0, height, 0.0);
            let camera = Camera {
                target,
                position: target
                    + Vec3::new(1.7307636095778745, 0.2967023330704928, -1.7307636095778745)
                        * distance as f64,
                field_of_view: 38.0,
                near: 0.01,
                far: 1000.0,
            };
            // A fixed physical trunk strip about the target, on the wood the clay
            // room paints there and clear of silhouette and ground, so a
            // thinner trunk than the oak's is measured on its own pixels.
            renderer.set_view(View::Clay);
            let clay = render(&mut renderer, &camera, 800, 500).unwrap();
            let strip = 400 - 100 / distance..400 + 100 / distance;
            let mask = resolution::wood_mask(&clay)
                .into_iter()
                .filter(|&(x, y)| (200..300).contains(&y) && strip.contains(&x))
                .collect::<Vec<_>>();
            assert!(
                mask.len() > 1000,
                "{preset:?} {distance}x: too little trunk: {}",
                mask.len()
            );
            renderer.set_view(View::Bare);
            let high = render(&mut renderer, &camera, 1600, 1000).unwrap();
            let low = render(&mut renderer, &camera, 800, 500).unwrap();
            let (mean, p95) = resolution::masked_agreement(&high, &low, &mask);
            eprintln!("{preset:?} {distance}x resolution: {} pixels, mean {mean:.6}/255, p95 {p95:.2}/255", mask.len());
            rows.push((format!("{preset:?} {distance}x"), mean, p95, 3.0));
        }
    }
    resolution::record("bark_distance", &rows);
    // Three code values mean at every distance again. Between fn-55 and
    // fn-71 the 4x bound stood at 3.25 (owner, 2026-09-18): the old sheen had
    // put 15% of the sun on the lit trunk and sat the frame on the tone
    // curve's shoulder, where the relief's cross-resolution error compressed
    // into fewer code values; base read 2.904, fn-55's physical highlight
    // 3.134, no highlight at all 3.167. fn-71 found the error a bias, the far
    // draw lighter than the near one reduced, and gave the far draw the shade
    // of the slopes its footprint lost; the oak reads 2.53 at 4x since.
    for (fixture, mean, p95, bound) in &rows {
        assert!(
            mean <= bound && *p95 <= 12.0,
            "distance series aliases on the {fixture}: mean {mean:.4}, p95 {p95:.2}"
        );
    }
}
