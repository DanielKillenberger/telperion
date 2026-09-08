//! One tree, one clay room, one GPU. The renderer takes the core's mesh and
//! draws it; it knows nothing about which tree it is or where the parameters
//! came from.
mod buffer;
mod camera;
mod device;
mod foliage;
#[cfg(not(target_arch = "wasm32"))]
mod headless;
mod scene;
mod submit;
mod timing;
mod view;
mod wood;

pub use buffer::Region;
pub use camera::{hero_pose, Camera, FIELD_OF_VIEW, FRAME_MARGIN};
pub use device::{Gpu, RenderError, Result};
pub use scene::{DEPTH_FORMAT, GROUND_REACH};
pub use submit::{fits, Submitted};
pub use timing::{
    judge, Hardware, Report, Session, Verdict, CONDITIONING, CONTENTION_RATIO, MEASURED, WARMUP,
};
pub use view::View;

#[cfg(not(target_arch = "wasm32"))]
pub use headless::{attachment, render, write_png, Still, STILL_FORMAT};
#[cfg(not(target_arch = "wasm32"))]
pub use timing::run as measure;

use telperion_core::{mesh::TreeMesh, surface::Bounds};

/// What one frame cost, in the terms a reader of a timing record needs.
/// `instances` counts the foliage placements drawn, which is the number the
/// view changes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct FrameStats {
    pub draw_calls: u32,
    pub triangles: u32,
    pub instances: u32,
}

impl std::ops::Add for FrameStats {
    type Output = Self;

    /// A frame's cost is what each pass in it cost, so the passes add up.
    fn add(self, other: Self) -> Self {
        Self {
            draw_calls: self.draw_calls + other.draw_calls,
            triangles: self.triangles.saturating_add(other.triangles),
            instances: self.instances.saturating_add(other.instances),
        }
    }
}

/// A device with the room built on it, holding at most one tree.
pub struct Renderer {
    gpu: Gpu,
    scene: scene::Scene,
    wood: wood::Wood,
    foliage: foliage::Foliage,
    view: View,
    bounds: Option<Bounds>,
    colour_format: wgpu::TextureFormat,
}

impl Renderer {
    /// Builds the room on an existing device. The colour format is the target's:
    /// an offscreen texture natively, the configured surface in a browser.
    pub fn new(gpu: Gpu, colour_format: wgpu::TextureFormat) -> Self {
        let scene = scene::Scene::new(&gpu, colour_format);
        let wood = wood::Wood::new(&gpu, scene.layout(), colour_format);
        let foliage = foliage::Foliage::new(&gpu, scene.layout(), colour_format);
        Self {
            gpu,
            scene,
            wood,
            foliage,
            view: View::default(),
            bounds: None,
            colour_format,
        }
    }

    pub fn gpu(&self) -> &Gpu {
        &self.gpu
    }

    pub fn colour_format(&self) -> wgpu::TextureFormat {
        self.colour_format
    }

    /// Uploads one tree in place from the core's arrays and stands the scale
    /// figure beside it. A tree the device cannot hold is refused whole,
    /// before anything of it goes up.
    pub fn submit(&mut self, mesh: &TreeMesh) -> Result<Submitted> {
        fits(&self.gpu.device.limits(), mesh)?;
        self.wood.submit(&self.gpu, &mesh.wood);
        self.foliage.submit(&self.gpu, &mesh.foliage);
        self.scene
            .place_figure(&self.gpu, mesh.bounds.max.y - mesh.bounds.min.y);
        self.bounds = Some(mesh.bounds);
        Ok(Submitted {
            wood_vertices: mesh.wood_vertices(),
            wood_triangles: mesh.wood_triangles(),
            foliage_instances: mesh.foliage_instances(),
            bounds: mesh.bounds,
        })
    }

    /// Selects what the next frame draws of the submitted tree.
    pub fn set_view(&mut self, view: View) {
        self.view = view;
    }

    pub fn view(&self) -> View {
        self.view
    }

    /// The bounds of what the current view draws, which is what a caller frames
    /// the camera on. The leaf view frames the element, not the tree.
    pub fn bounds(&self) -> Option<Bounds> {
        match self.view {
            View::Leaf => self.foliage.bounds(),
            _ => self.bounds,
        }
    }

    /// The live wood ranges: what is used of what was allocated.
    pub fn wood_regions(&self) -> Option<(Region, Region, Region)> {
        self.wood.regions()
    }

    /// The live foliage instance range.
    pub fn foliage_region(&self) -> Option<Region> {
        self.foliage.region()
    }

    /// Draws one frame into the given colour and depth views: the room first,
    /// then the vegetation in a pass of its own. The timestamp pair, when one
    /// is given, goes around that second pass, so what is measured is the tree
    /// and not the floor it stands on.
    pub fn draw(
        &mut self,
        camera: &Camera,
        aspect: f64,
        colour: &wgpu::TextureView,
        depth: &wgpu::TextureView,
        timestamps: Option<wgpu::RenderPassTimestampWrites<'_>>,
    ) -> FrameStats {
        self.scene.set_camera(&self.gpu, camera, aspect);
        let mut encoder = self
            .gpu
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("frame"),
            });
        let room = {
            let mut pass = pass(
                &mut encoder,
                "room",
                colour,
                depth,
                Some(self.scene.background()),
                None,
            );
            self.scene.bind(&mut pass);
            match self.view {
                // A leaf is judged on its own: at 0.1 m the room around it is a
                // wall, and the scale figure is not a scale for a leaf.
                View::Leaf => FrameStats::default(),
                _ => self.scene.draw(&mut pass),
            }
        };
        let vegetation = {
            let mut pass = pass(&mut encoder, "vegetation", colour, depth, None, timestamps);
            self.scene.bind(&mut pass);
            match self.view {
                View::Leaf => self.foliage.draw(&mut pass, self.view),
                view => self.wood.draw(&mut pass) + self.foliage.draw(&mut pass, view),
            }
        };
        self.gpu.queue.submit([encoder.finish()]);
        room + vegetation
    }
}

/// One pass of a frame. The first pass of a frame clears the targets and every
/// later one loads them, which is the whole difference between them.
fn pass<'encoder>(
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
