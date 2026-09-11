//! The shapes every frame shares: how a lit shader is built, how a pass is
//! opened over the targets, and the two pipelines a subject may be drawn
//! through - the lit one every colour pass uses, and the depth-only one the sun
//! writes its map with.
use crate::{device::Gpu, scene::DEPTH_FORMAT};

/// The block, the map and the terms every lit shader is drawn under. WGSL has
/// no include of its own, so one prelude is put in front of each shader's own
/// stages here rather than copied into each of them.
const PRELUDE: &str = include_str!("shaders/common.wgsl");

/// Samples per pixel a frame's colour and depth are drawn at where the device
/// offers it. Four is what a WebGPU implementation that multisamples at all
/// offers, and the count the edges were judged at.
pub const MULTISAMPLE: u32 = 4;

/// How many samples a frame is actually drawn at: the multisample count where
/// the colour target and the depth beside it both take it, and one sample where
/// either does not. Both have to agree, because a pass draws into the pair.
pub fn samples(colour: bool, depth: bool) -> u32 {
    if colour && depth {
        MULTISAMPLE
    } else {
        1
    }
}

/// What a pipeline draws into: the format of the colour target and how many
/// samples a pixel of it carries. The two travel together because a pipeline
/// that disagrees with its target on either one will not validate.
#[derive(Clone, Copy)]
pub struct Surface {
    pub format: wgpu::TextureFormat,
    pub samples: u32,
}

/// Where a frame is drawn: the colour and depth every pass writes, and, when
/// those carry several samples a pixel, the single-sample texture the last pass
/// resolves them into. The resolve is the picture anybody looks at; at one
/// sample there is none and the colour attachment is itself the picture.
#[derive(Clone, Copy)]
pub struct Target<'a> {
    pub colour: &'a wgpu::TextureView,
    pub depth: &'a wgpu::TextureView,
    pub resolve: Option<&'a wgpu::TextureView>,
}

impl<'a> Target<'a> {
    /// The same targets with nothing resolved out of them, which is every pass
    /// but the last: the samples are left where the pass after this one adds to
    /// them.
    pub fn kept(self) -> Self {
        Self {
            resolve: None,
            ..self
        }
    }
}

/// A render target of this format, size and sample count. Only a single-sample
/// texture is ever copied out of - a multisampled attachment is resolved, never
/// read - so only it asks to be a copy source.
pub fn attachment(
    gpu: &Gpu,
    label: &str,
    format: wgpu::TextureFormat,
    size: (u32, u32),
    samples: u32,
) -> wgpu::Texture {
    let copied = if samples == 1 {
        wgpu::TextureUsages::COPY_SRC
    } else {
        wgpu::TextureUsages::empty()
    };
    gpu.device.create_texture(&wgpu::TextureDescriptor {
        label: Some(label),
        size: wgpu::Extent3d {
            width: size.0,
            height: size.1,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: samples,
        dimension: wgpu::TextureDimension::D2,
        format,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT | copied,
        view_formats: &[],
    })
}

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
    target: Target<'_>,
    clear: Option<wgpu::Color>,
    timestamps: Option<wgpu::RenderPassTimestampWrites<'_>>,
) -> wgpu::RenderPass<'encoder> {
    encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        label: Some(label),
        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
            view: target.colour,
            depth_slice: None,
            resolve_target: target.resolve,
            ops: wgpu::Operations {
                load: clear.map_or(wgpu::LoadOp::Load, wgpu::LoadOp::Clear),
                // A pass that resolves is the last one over these targets and
                // what is kept is the resolve, so the samples behind it are
                // dropped rather than written out a second time.
                store: match target.resolve {
                    Some(_) => wgpu::StoreOp::Discard,
                    None => wgpu::StoreOp::Store,
                },
            },
        })],
        depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
            view: target.depth,
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
    surface: Surface,
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
                targets: &[Some(surface.format.into())],
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
            multisample: wgpu::MultisampleState {
                count: surface.samples,
                ..Default::default()
            },
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
            // The sun's map is one sample a texel: nobody looks at it, and a
            // shadow test reads a depth rather than an edge.
            multisample: Default::default(),
            multiview_mask: None,
            cache: None,
        })
}

#[cfg(test)]
mod tests {
    use super::{samples, MULTISAMPLE};

    #[test]
    fn a_target_that_will_not_multisample_takes_the_whole_frame_down_to_one_sample() {
        assert_eq!(samples(true, true), MULTISAMPLE);
        assert_eq!(samples(false, true), 1);
        assert_eq!(samples(true, false), 1);
        assert_eq!(samples(false, false), 1);
    }
}
