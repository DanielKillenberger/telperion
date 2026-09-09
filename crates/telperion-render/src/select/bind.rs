//! The layouts selection binds through, and the groups built to them. The
//! compute pass reads and writes every selection buffer; the foliage pass sees
//! only the placements and the list of the level it is drawing, read-only, at
//! that level's own offset.
use crate::{buffer::Held, device::Gpu};

/// Column-major, the same layout the core's instance matrices use.
const IDENTITY: [f32; 16] = [
    1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
];

pub fn compute_layout(gpu: &Gpu) -> wgpu::BindGroupLayout {
    let compute = wgpu::ShaderStages::COMPUTE;
    gpu.device
        .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("select"),
            entries: &[
                uniform(0),
                storage(1, compute, true, false),
                storage(2, compute, true, false),
                storage(3, compute, false, false),
                storage(4, compute, false, false),
                storage(5, compute, false, false),
            ],
        })
}

pub fn draw_layout(gpu: &Gpu) -> wgpu::BindGroupLayout {
    let vertex = wgpu::ShaderStages::VERTEX;
    gpu.device
        .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("foliage selection"),
            entries: &[
                storage(0, vertex, true, false),
                // The level being drawn arrives as a dynamic offset into the
                // one buffer every level's list lives in.
                storage(1, vertex, true, true),
            ],
        })
}

/// The group the pass fills the lists, the counters and the indirect blocks
/// through. Every buffer is bound whole: the pass addresses a level by stride.
pub fn compute_group(
    gpu: &Gpu,
    layout: &wgpu::BindGroupLayout,
    uniforms: &wgpu::Buffer,
    buffers: [&Held; 5],
) -> wgpu::BindGroup {
    let mut entries = vec![wgpu::BindGroupEntry {
        binding: 0,
        resource: uniforms.as_entire_binding(),
    }];
    entries.extend(
        buffers
            .iter()
            .enumerate()
            .map(|(n, held)| wgpu::BindGroupEntry {
                binding: n as u32 + 1,
                resource: held.buffer().as_entire_binding(),
            }),
    );
    gpu.device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("select"),
        layout,
        entries: &entries,
    })
}

/// The group the foliage pass draws a level through: the whole crown's
/// placements, and one level's worth of list, offset to the level at bind.
pub fn draw_group(
    gpu: &Gpu,
    layout: &wgpu::BindGroupLayout,
    placements: &Held,
    lists: &Held,
    stride: u64,
) -> wgpu::BindGroup {
    gpu.device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("foliage selection"),
        layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: placements.buffer().as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                    buffer: lists.buffer(),
                    offset: 0,
                    size: wgpu::BufferSize::new(stride),
                }),
            },
        ],
    })
}

/// The leaf view's own placements and list: one matrix at identity, and a list
/// naming it. It never changes, so it is built once with the device.
pub fn leaf_group(gpu: &Gpu, layout: &wgpu::BindGroupLayout) -> wgpu::BindGroup {
    let held = |label: &'static str, bytes: &[u8]| {
        let buffer = gpu.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some(label),
            size: bytes.len() as u64,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        gpu.queue.write_buffer(&buffer, 0, bytes);
        buffer
    };
    let placements = held("leaf placement", bytemuck::cast_slice(&IDENTITY));
    let list = held("leaf list", bytemuck::cast_slice(&[0u32]));
    gpu.device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("leaf selection"),
        layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: placements.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: list.as_entire_binding(),
            },
        ],
    })
}

fn uniform(binding: u32) -> wgpu::BindGroupLayoutEntry {
    wgpu::BindGroupLayoutEntry {
        binding,
        visibility: wgpu::ShaderStages::COMPUTE,
        ty: wgpu::BindingType::Buffer {
            ty: wgpu::BufferBindingType::Uniform,
            has_dynamic_offset: false,
            min_binding_size: None,
        },
        count: None,
    }
}

fn storage(
    binding: u32,
    visibility: wgpu::ShaderStages,
    read_only: bool,
    has_dynamic_offset: bool,
) -> wgpu::BindGroupLayoutEntry {
    wgpu::BindGroupLayoutEntry {
        binding,
        visibility,
        ty: wgpu::BindingType::Buffer {
            ty: wgpu::BufferBindingType::Storage { read_only },
            has_dynamic_offset,
            min_binding_size: None,
        },
        count: None,
    }
}
