//! Exercise the production shared lookup against a known depth plane, including
//! the map boundary and a receiver whose normal offset crosses the plane.
use super::*;
use crate::pass::{attachment, pass, pipeline, Depth, Surface, Target};

#[test]
fn kernel_averages_comparisons_and_offsets_the_receiver() {
    let gpu = match pollster::block_on(Gpu::request(None)) {
        Ok(gpu) => gpu,
        Err(crate::RenderError::WebGpuUnavailable(reason)) => {
            eprintln!("skipped: {reason}");
            return;
        }
        Err(crate::RenderError::FallbackOnly { adapter }) => {
            eprintln!("skipped: {adapter}");
            return;
        }
        Err(error) => panic!("{error}"),
    };
    let shadow = Shadow::new(&gpu);
    let source = include_str!("../shaders/common.wgsl").replace(
        "@group(0) @binding(0) var<uniform> u: Uniforms;",
        "var<private> u: Uniforms;",
    );
    let stages = r#"
@vertex fn vertex(@builtin(vertex_index) i: u32) -> @builtin(position) vec4<f32> {
    let p = array<vec2<f32>, 3>(vec2(-1.0, -1.0), vec2(3.0, -1.0), vec2(-1.0, 3.0));
    return vec4(p[i], 0.0, 1.0);
}
@fragment fn fragment(@builtin(position) p: vec4<f32>) -> @location(0) vec4<f32> {
    u.light_view_projection = mat4x4<f32>(vec4(1.0,0.0,0.0,0.0),
        vec4(0.0,1.0,0.0,0.0), vec4(0.0,0.0,1.0,0.0), vec4(0.0,0.0,0.0,1.0));
    let uv = (p.xy - vec2(8.0)) / 1024.0;
    let world = vec3(2.0 * uv.x - 1.0, 1.0 - 2.0 * uv.y, 0.75);
    u.shadow_filter = vec4(0.0, 1.0 / 1024.0, 0.0, 0.0);
    u.shadow_offset = vec4(0.0);
    let raw = sunlight(world, vec3(0.0, 0.0, -1.0));
    u.shadow_filter.x = 1.0;
    let filtered = sunlight(world, vec3(0.0, 0.0, -1.0));
    u.shadow_offset.x = 0.5;
    let offset = sunlight(world, vec3(0.0, 0.0, -1.0));
    return vec4(raw, filtered, offset, 1.0);
}
"#;
    let module = gpu
        .device
        .create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("kernel probe"),
            source: wgpu::ShaderSource::Wgsl((source + stages).into()),
        });
    let surface = Surface {
        format: wgpu::TextureFormat::Rgba8Unorm,
        samples: 1,
    };
    let pipeline = pipeline(
        &gpu,
        &[None, None, Some(shadow.layout())],
        &module,
        surface,
        &[],
        Depth::Behind,
        "kernel probe",
    );
    let colour = attachment(&gpu, "probe", surface.format, (16, 16), 1);
    let depth = attachment(&gpu, "probe depth", DEPTH_FORMAT, (16, 16), 1);
    let colour_view = colour.create_view(&Default::default());
    let depth_view = depth.create_view(&Default::default());
    let readback = gpu.device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("probe readback"),
        size: 256 * 16,
        usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
        mapped_at_creation: false,
    });
    let mut encoder = gpu.device.create_command_encoder(&Default::default());
    {
        let _clear = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("known depth"),
            color_attachments: &[],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: &shadow.view,
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Clear(0.5),
                    store: wgpu::StoreOp::Store,
                }),
                stencil_ops: None,
            }),
            ..Default::default()
        });
    }
    {
        let mut draw = pass(
            &mut encoder,
            "probe",
            Target {
                colour: &colour_view,
                depth: &depth_view,
                resolve: None,
            },
            Some(wgpu::Color::BLACK),
            None,
        );
        draw.set_pipeline(&pipeline);
        shadow.bind(&mut draw);
        draw.draw(0..3, 0..1);
    }
    encoder.copy_texture_to_buffer(
        colour.as_image_copy(),
        wgpu::TexelCopyBufferInfo {
            buffer: &readback,
            layout: wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(256),
                rows_per_image: Some(16),
            },
        },
        wgpu::Extent3d {
            width: 16,
            height: 16,
            depth_or_array_layers: 1,
        },
    );
    gpu.queue.submit([encoder.finish()]);
    let (send, recv) = std::sync::mpsc::channel();
    readback
        .slice(..)
        .map_async(wgpu::MapMode::Read, move |r| send.send(r).unwrap());
    gpu.device
        .poll(wgpu::PollType::wait_indefinitely())
        .unwrap();
    recv.recv().unwrap().unwrap();
    let pixels = readback.slice(..).get_mapped_range().unwrap();
    let channel = |x: usize, y: usize, c: usize| f64::from(pixels[y * 256 + x * 4 + c]);
    let mut intermediate = false;
    for y in 1..15 {
        for x in 1..15 {
            let mut average = 0.0;
            for dy in y - 1..=y + 1 {
                for dx in x - 1..=x + 1 {
                    average += channel(dx, dy, 0) / 9.0;
                }
            }
            let filtered = channel(x, y, 1);
            assert!(filtered + 1.0 >= average, "filter darkened the block");
            assert!(
                (filtered - average).abs() <= 1.0,
                "filter is not the block mean"
            );
            assert_eq!(
                channel(x, y, 2),
                255.0,
                "normal offset did not clear the plane"
            );
            intermediate |= filtered > 0.0 && filtered < 255.0;
        }
    }
    assert!(intermediate, "the kernel did not soften the map boundary");
    assert_eq!(
        channel(12, 12, 0),
        0.0,
        "the plane did not shadow the receiver"
    );
}
