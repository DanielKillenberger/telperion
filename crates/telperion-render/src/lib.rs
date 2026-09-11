//! One tree, one clay room, one GPU. The renderer takes the core's mesh and
//! draws it; it knows nothing about which tree it is or where the parameters
//! came from.
mod buffer;
mod camera;
mod device;
mod foliage;
#[cfg(not(target_arch = "wasm32"))]
mod headless;
mod pass;
mod scene;
mod select;
mod shadow;
mod submit;
mod timing;
mod view;
#[cfg(target_arch = "wasm32")]
mod web;
mod wood;

pub use buffer::Region;
pub use camera::{hero_pose, orbit_pose, walk_pose, Camera, FIELD_OF_VIEW, FRAME_MARGIN};
pub use device::{Gpu, RenderError, Result};
pub use scene::{SceneRow, DEPTH_FORMAT, GROUND_REACH};
pub use select::{Level, MAX_LEVELS};
pub use submit::{fits, Submitted};
pub use timing::{
    judge, Hardware, LevelCount, Report, Session, Verdict, CONDITIONING, CONTENTION_RATIO,
    MEASURED, WARMUP,
};
pub use view::View;

#[cfg(target_arch = "wasm32")]
pub use web::WebRenderer;

#[cfg(not(target_arch = "wasm32"))]
pub use headless::{attachment, render, write_png, Still, STILL_FORMAT};
#[cfg(not(target_arch = "wasm32"))]
pub use timing::{orbit as measure_orbit, run as measure};

use pass::{aspect_of, pass};
pub(crate) use pass::{depth_pipeline, pipeline};
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

/// The three timestamp pairs a timed frame writes: the vegetation render pass,
/// the selection compute pass that decides what it draws, and the sun's own
/// depth pass. Every pair is written on every timed frame, whatever the view is
/// showing, so a session never resolves a query no pass wrote.
pub struct Timed<'a> {
    pub vegetation: wgpu::RenderPassTimestampWrites<'a>,
    pub selection: wgpu::ComputePassTimestampWrites<'a>,
    pub shadow: wgpu::RenderPassTimestampWrites<'a>,
}

/// A device with the room built on it, holding at most one tree.
pub struct Renderer {
    gpu: Gpu,
    scene: scene::Scene,
    shadow: shadow::Shadow,
    wood: wood::Wood,
    foliage: foliage::Foliage,
    view: View,
    bounds: Option<Bounds>,
    /// The tolerance of each level of the crown's element, coarsest first, as
    /// the core built them. Kept here because a timing record names each
    /// level by the error it accepts, not by its position in the ladder.
    level_deviations: Vec<f64>,
    colour_format: wgpu::TextureFormat,
}

impl Renderer {
    /// Builds the room on an existing device. The colour format is the target's:
    /// an offscreen texture natively, the configured surface in a browser.
    pub fn new(gpu: Gpu, colour_format: wgpu::TextureFormat) -> Self {
        let shadow = shadow::Shadow::new(&gpu);
        let scene = scene::Scene::new(&gpu, colour_format, shadow.layout());
        let wood = wood::Wood::new(&gpu, scene.layout(), &shadow, colour_format);
        let foliage = foliage::Foliage::new(&gpu, scene.layout(), &shadow, colour_format);
        Self {
            gpu,
            scene,
            shadow,
            wood,
            foliage,
            view: View::default(),
            bounds: None,
            level_deviations: Vec::new(),
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
        self.submit_at(mesh, Level::default())
    }

    /// The same upload with the crown held at one level, which is how a
    /// measurement asks what the leaves cost. The level travels with the tree
    /// rather than sitting on the renderer, so no later frame inherits it.
    pub fn submit_at(&mut self, mesh: &TreeMesh, level: Level) -> Result<Submitted> {
        fits(&self.gpu.device.limits(), mesh)?;
        self.wood.submit(&self.gpu, &mesh.wood);
        self.foliage.submit(&self.gpu, &mesh.foliage, level);
        self.scene
            .place_figure(&self.gpu, mesh.bounds.max.y - mesh.bounds.min.y);
        self.bounds = Some(mesh.bounds);
        self.level_deviations = mesh
            .foliage
            .element
            .levels
            .iter()
            .map(|level| level.deviation)
            .collect();
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

    /// Puts the sun somewhere else and paints the sky and the ground with it.
    /// A scene is not a property of a tree, so it outlives every submission
    /// and no blend between two families touches it.
    pub fn set_scene(&mut self, row: SceneRow) {
        self.scene.set_row(row);
    }

    /// The sun, sky and ground the next frame is drawn under.
    pub fn scene(&self) -> &SceneRow {
        self.scene.row()
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

    /// What the last frame's selection counted: one entry per level, coarsest
    /// first, and last the leaves no level drew because the frame did not
    /// show them. Reading it stalls on the device, so it belongs to a check or
    /// a record and never to a frame.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn level_counts(&self) -> Option<Vec<u32>> {
        self.foliage.counted(&self.gpu)
    }

    /// What each level of the submitted crown accepts as error, in metres,
    /// coarsest first. One shorter than `level_counts`, which ends with the
    /// bucket of leaves no level drew and so has no tolerance of its own.
    pub fn level_deviations(&self) -> &[f64] {
        &self.level_deviations
    }

    /// The sun's depth map as the last frame left it, a depth per texel, row by
    /// row: one where nothing stood between that texel and the sun. Reading it
    /// waits on the device and brings a whole map back, so it belongs to a
    /// check or a record and never to a frame.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn shadow_depths(&self) -> Option<Vec<f32>> {
        self.shadow.depths(&self.gpu)
    }

    /// Draws one frame into the given colour and depth views, at the size in
    /// pixels those views were taken at: the crown's levels are chosen first,
    /// then the sun's depth map, then the room, then the vegetation in a pass
    /// of its own.
    pub fn draw(
        &mut self,
        camera: &Camera,
        viewport: (u32, u32),
        colour: &wgpu::TextureView,
        depth: &wgpu::TextureView,
    ) -> FrameStats {
        self.draw_with(camera, viewport, colour, depth, None)
    }

    /// The same frame with a pair around each of the three passes the tree
    /// costs: the selection pass that decides what is drawn, the shadow pass
    /// the sun writes, and the vegetation pass that draws the tree. What each
    /// measures is the tree and not the floor it stands on.
    pub fn draw_timed(
        &mut self,
        camera: &Camera,
        viewport: (u32, u32),
        colour: &wgpu::TextureView,
        depth: &wgpu::TextureView,
        timed: Timed<'_>,
    ) -> FrameStats {
        self.draw_with(camera, viewport, colour, depth, Some(timed))
    }

    /// The statistics are the picture's: the room and the vegetation. The
    /// sun's pass draws no pixels anybody looks at, and its cost is the third
    /// pair of a timing record rather than a line in the frame's own count.
    fn draw_with(
        &mut self,
        camera: &Camera,
        viewport: (u32, u32),
        colour: &wgpu::TextureView,
        depth: &wgpu::TextureView,
        timed: Option<Timed<'_>>,
    ) -> FrameStats {
        let light = shadow::light(self.scene.row(), self.bounds());
        self.scene
            .set_frame(&self.gpu, camera, aspect_of(viewport), &light);
        self.shadow.set_light(&self.gpu, &light);
        let (vegetation_writes, selection_writes, shadow_writes) = match timed {
            Some(timed) => (
                Some(timed.vegetation),
                Some(timed.selection),
                Some(timed.shadow),
            ),
            None => (None, None, None),
        };
        let mut encoder = self
            .gpu
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("frame"),
            });
        self.foliage.dispatch(
            &self.gpu,
            &mut encoder,
            camera,
            viewport,
            self.view,
            selection_writes,
        );
        {
            let mut pass = self.shadow.begin(&mut encoder, shadow_writes);
            match self.view {
                // One blade at the origin is judged on its own shape, with
                // nothing above it to cast and nothing below it to catch.
                View::Leaf => {}
                // Bare wood casts its own shadow and takes it; the crown that
                // is not drawn is not in the sun's view either.
                View::Bare => self.wood.draw_shadow(&mut pass),
                _ => {
                    self.wood.draw_shadow(&mut pass);
                    self.foliage.draw_shadow(&mut pass);
                }
            }
        }
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
            self.shadow.bind(&mut pass);
            match self.view {
                // A leaf is judged on its own: at 0.1 m the room around it is a
                // wall, and the scale figure is not a scale for a leaf.
                View::Leaf => FrameStats::default(),
                _ => self.scene.draw(&mut pass),
            }
        };
        let vegetation = {
            let mut pass = pass(
                &mut encoder,
                "vegetation",
                colour,
                depth,
                None,
                vegetation_writes,
            );
            self.scene.bind(&mut pass);
            self.shadow.bind(&mut pass);
            match self.view {
                View::Leaf => self.foliage.draw(&mut pass, self.view),
                view => self.wood.draw(&mut pass) + self.foliage.draw(&mut pass, view),
            }
        };
        self.gpu.queue.submit([encoder.finish()]);
        room + vegetation
    }
}
