//! fn-246: the wood's shaded pass reads a pixel as only as many shading
//! cells as its footprint needs. Every cell stays inside the widest footprint
//! the dense grid already reads one at (half a ridge width, a quarter under
//! chipped edges), and a pixel a ridge width wide or more keeps the grid.
mod common;

/// `bark_cells` from the production shader, run on the device for each
/// pixel width and edge, one row a case.
fn cells(gpu: &telperion_render::Gpu, cases: &[([f32; 2], bool)]) -> Vec<[i32; 2]> {
    let wood = include_str!("../src/shaders/wood.wgsl");
    let start = wood.find("fn bark_cells(").expect("the shader names its cells");
    let end = start + wood[start..].find("\n}").unwrap() + 2;
    let body: String = cases
        .iter()
        .enumerate()
        .map(|(i, ([x, y], chipped))| {
            format!("result[{i}] = bark_cells(vec2({x:?}, {y:?}), {chipped});\n")
        })
        .collect();
    let source = format!(
        "{}\n@group(0) @binding(0) var<storage, read_write> result: array<vec2<i32>>;\n\
         @compute @workgroup_size(1) fn main() {{\n{body}}}\n",
        &wood[start..end]
    );
    let module = gpu
        .device
        .create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("bark cells"),
            source: wgpu::ShaderSource::Wgsl(source.into()),
        });
    let pipeline = gpu
        .device
        .create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some("bark cells"),
            layout: None,
            module: &module,
            entry_point: Some("main"),
            compilation_options: Default::default(),
            cache: None,
        });
    let size = 8 * cases.len() as u64;
    let buffer = |usage| {
        gpu.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("bark cells"),
            size,
            usage,
            mapped_at_creation: false,
        })
    };
    let output = buffer(wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC);
    let readback = buffer(wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST);
    let group = gpu.device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: None,
        layout: &pipeline.get_bind_group_layout(0),
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: output.as_entire_binding(),
        }],
    });
    let mut encoder = gpu.encoder(&Default::default());
    {
        let mut pass = encoder.begin_compute_pass(&Default::default());
        pass.set_pipeline(&pipeline);
        pass.set_bind_group(0, &group, &[]);
        pass.dispatch_workgroups(1, 1, 1);
    }
    encoder.copy_buffer_to_buffer(&output, 0, &readback, 0, size);
    gpu.submit([encoder.finish()]);
    readback.slice(..).map_async(wgpu::MapMode::Read, |r| r.unwrap());
    gpu.device
        .poll(wgpu::PollType::wait_indefinitely())
        .unwrap();
    let mapped = readback.slice(..).get_mapped_range().unwrap();
    bytemuck::cast_slice::<u8, [i32; 2]>(&mapped).to_vec()
}

#[test]
fn a_pixel_is_read_as_the_cells_its_footprint_needs() {
    let Some(gpu) = common::gpu() else { return };
    // (pixel in ridge widths per axis, chipped edges) and the cells a side.
    let table: [(([f32; 2], bool), [i32; 2]); 8] = [
        (([0.1, 0.1], false), [1, 1]),
        (([0.5, 0.5], false), [1, 1]),
        (([0.4, 0.7], false), [1, 2]),
        (([0.9, 0.99], false), [2, 2]),
        (([1.5, 0.2], false), [3, 1]),
        (([6.0, 6.0], false), [3, 3]),
        (([0.2, 0.3], true), [1, 2]),
        (([0.6, 2.0], true), [3, 4]),
    ];
    let cases: Vec<_> = table.iter().map(|(case, _)| *case).collect();
    let got = cells(&gpu, &cases);
    for ((case, want), got) in table.iter().zip(got) {
        assert_eq!(got, *want, "cells for a pixel {case:?}");
    }
}
