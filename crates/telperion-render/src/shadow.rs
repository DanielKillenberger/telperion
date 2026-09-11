//! The sun's own view of the subject: one depth map, written once per frame
//! from where the sun stands, that every lit pass compares against.
//!
//! The map is depth and nothing else - no colour target, no fragment stage -
//! and it is fitted to what is on stage plus the shadow the sun's elevation
//! throws from it, never to the 400 m floor the subject stands on. The crown is
//! drawn at the element's coarsest level over the whole placement buffer: the
//! sun's view is not the camera's, so nothing the camera selected applies here
//! and no second selection is run for it.
use crate::{device::Gpu, scene::DEPTH_FORMAT};

/// Where the sun stands and what its map covers, kept beside this file
/// rather than in it so neither outgrows the project's line rule.
mod fit;

pub use fit::{light, Light};

/// The map's edge in texels. One 4 MB depth texture, sized once with the
/// device: an oak and its ground shadow come to some 35 m across, which is
/// three centimetres a texel.
///
/// Measured at 2,048 first, as the task asked: the pass cost 2.30 ms p50 on
/// the RTX 3080 against the 1.5 ms the owner allowed, so the map was halved.
/// The pass draws 8.2 M wood triangles and 869 k placements, so what is left
/// is the geometry and not the raster; the number is recorded, not gated, and
/// what to do about it is the spec's to decide.
pub const RESOLUTION: u32 = 1_024;

/// The map, the sampler every lit shader compares through, and the light the
/// depth-only pipelines are drawn under.
pub struct Shadow {
    view: wgpu::TextureView,
    /// The map itself, which only a host-side readback has any use for: the
    /// view keeps the texture alive for a frame on its own.
    #[cfg(not(target_arch = "wasm32"))]
    map: wgpu::Texture,
    sampled: wgpu::BindGroup,
    sampled_layout: wgpu::BindGroupLayout,
    light: wgpu::Buffer,
    light_group: wgpu::BindGroup,
    light_layout: wgpu::BindGroupLayout,
    module: wgpu::ShaderModule,
}

impl Shadow {
    pub fn new(gpu: &Gpu) -> Self {
        let map = gpu.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("shadow map"),
            size: wgpu::Extent3d {
                width: RESOLUTION,
                height: RESOLUTION,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            // The map stays single-sample whatever the frame is drawn at: a
            // depth comparison averages nothing.
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: DEPTH_FORMAT,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT
                | wgpu::TextureUsages::TEXTURE_BINDING
                | wgpu::TextureUsages::COPY_SRC,
            view_formats: &[],
        });
        let view = map.create_view(&Default::default());
        // Linear filtering on a comparison sampler is the hardware's own four
        // taps: one call, four texels, a soft edge rather than a staircase.
        let sampler = gpu.device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("shadow"),
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            compare: Some(wgpu::CompareFunction::LessEqual),
            ..Default::default()
        });
        let sampled_layout =
            gpu.device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    label: Some("shadow"),
                    entries: &[
                        wgpu::BindGroupLayoutEntry {
                            binding: 0,
                            visibility: wgpu::ShaderStages::FRAGMENT,
                            ty: wgpu::BindingType::Texture {
                                sample_type: wgpu::TextureSampleType::Depth,
                                view_dimension: wgpu::TextureViewDimension::D2,
                                multisampled: false,
                            },
                            count: None,
                        },
                        wgpu::BindGroupLayoutEntry {
                            binding: 1,
                            visibility: wgpu::ShaderStages::FRAGMENT,
                            ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Comparison),
                            count: None,
                        },
                    ],
                });
        let sampled = gpu.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("shadow"),
            layout: &sampled_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&sampler),
                },
            ],
        });

        let light = gpu.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("light"),
            size: size_of::<[f32; 16]>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let light_layout = gpu
            .device
            .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("light"),
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
            });
        let light_group = gpu.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("light"),
            layout: &light_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: light.as_entire_binding(),
            }],
        });

        Self {
            view,
            #[cfg(not(target_arch = "wasm32"))]
            map,
            sampled,
            sampled_layout,
            light,
            light_group,
            light_layout,
            module: gpu
                .device
                .create_shader_module(wgpu::include_wgsl!("shaders/shadow.wgsl")),
        }
    }

    /// The layout a lit pipeline samples the map through, and the one a
    /// depth-only pipeline is drawn under.
    pub fn layout(&self) -> &wgpu::BindGroupLayout {
        &self.sampled_layout
    }

    pub fn light_layout(&self) -> &wgpu::BindGroupLayout {
        &self.light_layout
    }

    /// The one shadow shader both subjects build their depth-only pipeline from.
    pub fn module(&self) -> &wgpu::ShaderModule {
        &self.module
    }

    /// Stands the sun where the scene row puts it, for the frame about to be
    /// drawn.
    pub fn set_light(&self, gpu: &Gpu, light: &Light) {
        gpu.queue
            .write_buffer(&self.light, 0, bytemuck::cast_slice(&light.view_projection));
    }

    /// Opens the depth-only pass over the whole map, cleared. It is opened on
    /// every frame, whatever the view draws into it, so a timed frame always
    /// writes its pair and a view that casts nothing reads as open ground
    /// rather than as whatever the last frame left behind.
    pub fn begin<'encoder>(
        &self,
        encoder: &'encoder mut wgpu::CommandEncoder,
        timestamps: Option<wgpu::RenderPassTimestampWrites<'_>>,
    ) -> wgpu::RenderPass<'encoder> {
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("shadow"),
            color_attachments: &[],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: &self.view,
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Clear(1.0),
                    store: wgpu::StoreOp::Store,
                }),
                stencil_ops: None,
            }),
            timestamp_writes: timestamps,
            occlusion_query_set: None,
            multiview_mask: None,
        });
        pass.set_bind_group(0, &self.light_group, &[]);
        pass
    }

    /// Binds the map for a pass that is lit by it.
    pub fn bind(&self, pass: &mut wgpu::RenderPass<'_>) {
        pass.set_bind_group(2, &self.sampled, &[]);
    }

    /// The map as the device holds it, a depth per texel, row by row. The
    /// device is waited on and 16 MB come back, so this belongs to a check or a
    /// record and never to a frame.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn depths(&self, gpu: &Gpu) -> Option<Vec<f32>> {
        let tight = RESOLUTION * size_of::<f32>() as u32;
        let padded =
            tight.div_ceil(wgpu::COPY_BYTES_PER_ROW_ALIGNMENT) * wgpu::COPY_BYTES_PER_ROW_ALIGNMENT;
        let readback = gpu.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("shadow readback"),
            size: u64::from(padded) * u64::from(RESOLUTION),
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            mapped_at_creation: false,
        });
        let mut encoder = gpu
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("shadow readback"),
            });
        encoder.copy_texture_to_buffer(
            self.map.as_image_copy(),
            wgpu::TexelCopyBufferInfo {
                buffer: &readback,
                layout: wgpu::TexelCopyBufferLayout {
                    offset: 0,
                    bytes_per_row: Some(padded),
                    rows_per_image: Some(RESOLUTION),
                },
            },
            wgpu::Extent3d {
                width: RESOLUTION,
                height: RESOLUTION,
                depth_or_array_layers: 1,
            },
        );
        gpu.queue.submit([encoder.finish()]);

        let (sender, receiver) = std::sync::mpsc::channel();
        readback
            .slice(..)
            .map_async(wgpu::MapMode::Read, move |result| {
                let _ = sender.send(result);
            });
        gpu.device.poll(wgpu::PollType::wait_indefinitely()).ok()?;
        receiver.recv().ok()?.ok()?;
        let view = readback.slice(..).get_mapped_range().ok()?;
        let mut depths = Vec::with_capacity((RESOLUTION * RESOLUTION) as usize);
        for row in 0..RESOLUTION as usize {
            let start = row * padded as usize;
            depths.extend(
                view[start..start + tight as usize]
                    .as_chunks::<4>()
                    .0
                    .iter()
                    .map(|word| f32::from_ne_bytes(*word)),
            );
        }
        drop(view);
        readback.unmap();
        Some(depths)
    }
}
