//! One tree, one clay room, one GPU. The renderer takes the core's mesh and
//! draws it; it knows nothing about which tree it is or where the parameters
//! came from.
mod camera;
mod device;
#[cfg(not(target_arch = "wasm32"))]
mod headless;
mod scene;
mod wood;

pub use camera::{hero_pose, Camera, FIELD_OF_VIEW, FRAME_MARGIN};
pub use device::{Gpu, RenderError, Result};
pub use scene::{DEPTH_FORMAT, GROUND_REACH};
pub use wood::Region;

#[cfg(not(target_arch = "wasm32"))]
pub use headless::{render, write_png, Still, STILL_FORMAT};

use telperion_core::mesh::TreeMesh;

/// What one frame cost, in the terms a reader of a timing record needs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct FrameStats {
    pub draw_calls: u32,
    pub triangles: u32,
}

/// What a submitted tree turned into on the GPU. The counts are the mesh's own,
/// so a caller can check them against what the core reported.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Submitted {
    pub wood_vertices: usize,
    pub wood_triangles: usize,
}

/// A device with the room built on it, holding at most one tree.
pub struct Renderer {
    gpu: Gpu,
    scene: scene::Scene,
    wood: wood::Wood,
    colour_format: wgpu::TextureFormat,
}

impl Renderer {
    /// Builds the room on an existing device. The colour format is the target's:
    /// an offscreen texture natively, the configured surface in a browser.
    pub fn new(gpu: Gpu, colour_format: wgpu::TextureFormat) -> Self {
        let scene = scene::Scene::new(&gpu, colour_format);
        let wood = wood::Wood::new(&gpu, scene.layout(), colour_format);
        Self {
            gpu,
            scene,
            wood,
            colour_format,
        }
    }

    pub fn gpu(&self) -> &Gpu {
        &self.gpu
    }

    pub fn colour_format(&self) -> wgpu::TextureFormat {
        self.colour_format
    }

    /// Uploads one tree's wood in place from the core's arrays and stands the
    /// scale figure beside it.
    pub fn submit(&mut self, mesh: &TreeMesh) -> Submitted {
        self.wood.submit(&self.gpu, &mesh.wood);
        self.scene
            .place_figure(&self.gpu, mesh.bounds.max.y - mesh.bounds.min.y);
        Submitted {
            wood_vertices: mesh.wood_vertices(),
            wood_triangles: mesh.wood_triangles(),
        }
    }

    /// The live wood ranges: what is used of what was allocated.
    pub fn wood_regions(&self) -> Option<(Region, Region, Region)> {
        self.wood.regions()
    }

    /// Draws one frame into the given colour and depth views.
    pub fn draw(
        &mut self,
        camera: &Camera,
        aspect: f64,
        colour: &wgpu::TextureView,
        depth: &wgpu::TextureView,
    ) -> FrameStats {
        self.scene.set_camera(&self.gpu, camera, aspect);
        let mut encoder = self
            .gpu
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("frame"),
            });
        let triangles = {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("frame"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: colour,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(self.scene.background()),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: depth,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                timestamp_writes: None,
                occlusion_query_set: None,
                multiview_mask: None,
            });
            self.scene.draw(&mut pass) + self.wood.draw(&mut pass)
        };
        self.gpu.queue.submit([encoder.finish()]);
        FrameStats {
            draw_calls: 2,
            triangles,
        }
    }
}

/// The one pipeline shape every pass shares: clay under a hemisphere, depth
/// tested, and no culling because a swept surface may plait either way round.
fn pipeline(
    gpu: &Gpu,
    bind_group_layout: &wgpu::BindGroupLayout,
    shader: &wgpu::ShaderModule,
    colour_format: wgpu::TextureFormat,
    buffers: &[Option<wgpu::VertexBufferLayout<'_>>],
    label: &'static str,
) -> wgpu::RenderPipeline {
    let layout = gpu
        .device
        .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some(label),
            bind_group_layouts: &[Some(bind_group_layout)],
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
                depth_write_enabled: Some(true),
                depth_compare: Some(wgpu::CompareFunction::Less),
                stencil: Default::default(),
                bias: Default::default(),
            }),
            multisample: Default::default(),
            multiview_mask: None,
            cache: None,
        })
}
