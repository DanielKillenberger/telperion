//! The shapes every frame shares: how a lit shader is built, how a pass is
//! opened over the targets, and the two pipelines a subject may be drawn
//! through - the lit one every colour pass uses, and the depth-only one the sun
//! writes its map with.
use crate::{device::Gpu, scene::DEPTH_FORMAT};

/// The block, the map and the terms every lit shader is drawn under. WGSL has
/// no include of its own, so one prelude is put in front of each shader's own
/// stages here rather than copied into each of them.
const PRELUDE: &str = include_str!("shaders/common.wgsl");

/// A lit shader: the prelude, then this module's own stages.
pub fn lit_shader(gpu: &Gpu, label: &str, stages: &str) -> wgpu::ShaderModule {
    gpu.device
        .create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some(label),
            source: wgpu::ShaderSource::Wgsl(format!("{PRELUDE}\n{stages}").into()),
        })
}

/// Whether a pipeline takes its own place in the depth buffer, or stands
/// behind everything that does. Only the sky stands behind: it is not a
/// surface, it is what is left where no surface is.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Depth {
    Surface,
    Behind,
}

/// The frame's aspect, from the pixels it is drawn into. A viewport with no
/// height is a frame nobody sees; it still has to divide.
pub fn aspect_of((width, height): (u32, u32)) -> f64 {
    f64::from(width.max(1)) / f64::from(height.max(1))
}

/// One pass of a frame. The first pass of a frame clears the targets and every
/// later one loads them, which is the whole difference between them.
pub fn pass<'encoder>(
    encoder: &'encoder mut wgpu::CommandEncoder,
    label: &'static str,
    colour: &wgpu::TextureView,
    depth: &wgpu::TextureView,
    clear: Option<wgpu::Color>,
    timestamps: Option<wgpu::RenderPassTimestampWrites<'_>>,
) -> wgpu::RenderPass<'encoder> {
    encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        label: Some(label),
        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
            view: colour,
            depth_slice: None,
            resolve_target: None,
            ops: wgpu::Operations {
                load: clear.map_or(wgpu::LoadOp::Load, wgpu::LoadOp::Clear),
                store: wgpu::StoreOp::Store,
            },
        })],
        depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
            view: depth,
            depth_ops: Some(wgpu::Operations {
                load: clear.map_or(wgpu::LoadOp::Load, |_| wgpu::LoadOp::Clear(1.0)),
                store: wgpu::StoreOp::Store,
            }),
            stencil_ops: None,
        }),
        timestamp_writes: timestamps,
        occlusion_query_set: None,
        multiview_mask: None,
    })
}

/// The one pipeline shape every pass shares: clay under a hemisphere, depth
/// tested, and no culling because a swept surface may plait either way round.
pub fn pipeline(
    gpu: &Gpu,
    bind_group_layouts: &[Option<&wgpu::BindGroupLayout>],
    shader: &wgpu::ShaderModule,
    colour_format: wgpu::TextureFormat,
    buffers: &[Option<wgpu::VertexBufferLayout<'_>>],
    depth: Depth,
    label: &'static str,
) -> wgpu::RenderPipeline {
    let layout = gpu
        .device
        .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some(label),
            bind_group_layouts,
            immediate_size: 0,
        });
    gpu.device
        .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some(label),
            layout: Some(&layout),
            vertex: wgpu::VertexState {
                module: shader,
                entry_point: Some("vertex"),
                compilation_options: Default::default(),
                buffers,
            },
            fragment: Some(wgpu::FragmentState {
                module: shader,
                entry_point: Some("fragment"),
                compilation_options: Default::default(),
                targets: &[Some(colour_format.into())],
            }),
            primitive: wgpu::PrimitiveState {
                cull_mode: None,
                ..Default::default()
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: DEPTH_FORMAT,
                depth_write_enabled: Some(depth == Depth::Surface),
                depth_compare: Some(match depth {
                    Depth::Surface => wgpu::CompareFunction::Less,
                    Depth::Behind => wgpu::CompareFunction::Always,
                }),
                stencil: Default::default(),
                bias: Default::default(),
            }),
            multisample: Default::default(),
            multiview_mask: None,
            cache: None,
        })
}

/// Depth written at a grazing angle lands on the wrong side of the surface that
/// wrote it and the surface shadows itself. The slope term carries most of it;
/// the constant covers a face the light hits square on.
const DEPTH_BIAS: wgpu::DepthBiasState = wgpu::DepthBiasState {
    constant: 2,
    slope_scale: 3.0,
    clamp: 0.0,
};

/// A depth-only pipeline: one matrix, one vertex buffer of positions, and no
/// fragment stage at all. Apart from the lit shape above because it shares
/// almost none of it - no colour target, no shading rule - and because it
/// carries a bias nothing that is looked at wants.
pub fn depth_pipeline(
    gpu: &Gpu,
    bind_group_layouts: &[Option<&wgpu::BindGroupLayout>],
    shader: &wgpu::ShaderModule,
    entry_point: &str,
    buffers: &[Option<wgpu::VertexBufferLayout<'_>>],
    label: &'static str,
) -> wgpu::RenderPipeline {
    let layout = gpu
        .device
        .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some(label),
            bind_group_layouts,
            immediate_size: 0,
        });
    gpu.device
        .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some(label),
            layout: Some(&layout),
            vertex: wgpu::VertexState {
                module: shader,
                entry_point: Some(entry_point),
                compilation_options: Default::default(),
                buffers,
            },
            fragment: None,
            primitive: wgpu::PrimitiveState {
                // A swept surface may plait either way round and a blade has no
                // back, so neither subject may be culled here either.
                cull_mode: None,
                ..Default::default()
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: DEPTH_FORMAT,
                depth_write_enabled: Some(true),
                depth_compare: Some(wgpu::CompareFunction::Less),
                stencil: Default::default(),
                bias: DEPTH_BIAS,
            }),
            multisample: Default::default(),
            multiview_mask: None,
            cache: None,
        })
}
