//! The browser target: a page canvas, one device, one tree. Generation runs
//! here too, in this module's own memory, so nothing about a tree crosses the
//! JavaScript boundary except its parameters going in and its counts coming
//! out. Every failure arrives in JavaScript as the Rust error's own words.
use std::{cell::RefCell, rc::Rc};

use serde_json::json;
use telperion_core::{
    math::Vec3,
    mesh::{self, Detail},
    params,
    surface::Bounds,
};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;
use web_sys::HtmlCanvasElement;

use crate::{
    device::{Gpu, RenderError, Result},
    hero_pose, Camera, FrameStats, Renderer, SceneRow, Submitted, Timed, View, DEPTH_FORMAT,
    GROUND_REACH,
};

/// The timing protocol as a page runs it, kept beside this file rather than in
/// it so neither outgrows the project's line rule.
mod session;

/// What the camera looks at before a tree has been submitted: a subject one
/// metre across at the origin. A room cannot be fitted to a point, and the
/// clay room the panel came from used the same floor for the same reason.
const EMPTY: Bounds = Bounds {
    min: Vec3::new(-0.5, 0.0, -0.5),
    max: Vec3::new(0.5, 1.0, 0.5),
};

/// Everything one canvas owns. Held behind a cell so an awaited timing session
/// can put it down between frames, and taken out on disposal so the device
/// goes with the page's own lifecycle rather than with the garbage collector.
struct Live {
    surface: wgpu::Surface<'static>,
    config: wgpu::SurfaceConfiguration,
    depth: wgpu::TextureView,
    renderer: Renderer,
    camera: Camera,
    stats: FrameStats,
}

impl Live {
    async fn new(canvas: HtmlCanvasElement) -> Result<Self> {
        let (width, height) = (canvas.width().max(1), canvas.height().max(1));
        let gpu = Gpu::request(None).await?;
        let surface = gpu.create_surface(wgpu::SurfaceTarget::Canvas(canvas))?;
        let config = gpu.surface_config(&surface, width, height)?;
        surface.configure(&gpu.device, &config);
        let depth = depth_view(&gpu, width, height);
        let camera = hero_pose(EMPTY, aspect(&config), GROUND_REACH);
        let renderer = Renderer::new(gpu, config.format);
        Ok(Self {
            surface,
            config,
            depth,
            renderer,
            camera,
            stats: FrameStats::default(),
        })
    }

    fn resize(&mut self, width: u32, height: u32) {
        let limit = self.renderer.gpu().device.limits().max_texture_dimension_2d;
        let (width, height) = (width.clamp(1, limit), height.clamp(1, limit));
        if (width, height) == (self.config.width, self.config.height) {
            return;
        }
        (self.config.width, self.config.height) = (width, height);
        // Between frames by construction: a surface texture is acquired,
        // drawn and presented inside `draw`, and never held across a call.
        self.surface
            .configure(&self.renderer.gpu().device, &self.config);
        self.depth = depth_view(self.renderer.gpu(), width, height);
    }

    /// Draws one frame onto the canvas at the pose the camera is at. A timed
    /// frame carries every pair the session resolves, so no query it reads is
    /// left unwritten.
    fn draw(&mut self, timed: Option<Timed<'_>>) -> Result<()> {
        let frame = self.texture()?;
        let colour = frame.texture.create_view(&Default::default());
        let size = (self.config.width, self.config.height);
        self.stats = match timed {
            Some(timed) => {
                self.renderer
                    .draw_timed(&self.camera, size, &colour, &self.depth, timed)
            }
            None => self.renderer.draw(&self.camera, size, &colour, &self.depth),
        };
        self.renderer.gpu().queue.present(frame);
        Ok(())
    }

    /// The frame to draw into. A swap chain the page has outgrown is
    /// reconfigured once and asked again; a second refusal is the real one.
    fn texture(&mut self) -> Result<wgpu::SurfaceTexture> {
        use wgpu::CurrentSurfaceTexture as Acquired;
        match self.surface.get_current_texture() {
            Acquired::Outdated | Acquired::Lost => {
                self.surface
                    .configure(&self.renderer.gpu().device, &self.config);
                frame_of(self.surface.get_current_texture())
            }
            acquired => frame_of(acquired),
        }
    }
}

/// The acquired frame, or the condition that came instead of one. A suboptimal
/// frame is still a frame: the next resize reconfigures the surface anyway, and
/// a slightly stale picture beats a blank canvas and an alert.
fn frame_of(acquired: wgpu::CurrentSurfaceTexture) -> Result<wgpu::SurfaceTexture> {
    use wgpu::CurrentSurfaceTexture as Acquired;
    let refused = |detail: &str| RenderError::Surface(detail.into());
    match acquired {
        Acquired::Success(frame) | Acquired::Suboptimal(frame) => Ok(frame),
        Acquired::Timeout => Err(refused("the surface timed out")),
        Acquired::Occluded => Err(refused("the canvas is not visible")),
        Acquired::Outdated => Err(refused("the surface configuration is outdated")),
        Acquired::Lost => Err(refused("the surface was lost")),
        Acquired::Validation => Err(refused("the surface raised a validation error")),
    }
}

fn aspect(config: &wgpu::SurfaceConfiguration) -> f64 {
    f64::from(config.width) / f64::from(config.height)
}

fn depth_view(gpu: &Gpu, width: u32, height: u32) -> wgpu::TextureView {
    gpu.device
        .create_texture(&wgpu::TextureDescriptor {
            label: Some("canvas depth"),
            size: wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: DEPTH_FORMAT,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        })
        .create_view(&Default::default())
}

/// One canvas's renderer, as the page holds it. Every method that can fail
/// fails with the renderer's own message, so the panel's alert never has to
/// invent one.
#[wasm_bindgen]
pub struct WebRenderer {
    live: Rc<RefCell<Option<Live>>>,
}

#[wasm_bindgen]
impl WebRenderer {
    /// A renderer on this canvas, or a rejection naming the condition that
    /// stopped it: no WebGPU, a software-only adapter, a refused device.
    pub fn create(canvas: HtmlCanvasElement) -> js_sys::Promise {
        future_to_promise(async move {
            let live = Live::new(canvas).await.map_err(js_error)?;
            Ok(JsValue::from(Self {
                live: Rc::new(RefCell::new(Some(live))),
            }))
        })
    }

    /// Generates the tree these parameters describe and uploads it whole,
    /// returning what went up. A set the generator refuses leaves the tree
    /// already on the canvas exactly where it was.
    #[wasm_bindgen(js_name = setTree)]
    pub fn set_tree(&self, family: &str) -> std::result::Result<String, JsError> {
        let value = serde_json::from_str(family)
            .map_err(|error| JsError::new(&format!("the parameters are not JSON: {error}")))?;
        let family = params::parse(&value).map_err(|error| js_error(RenderError::from(error)))?;
        let mesh = mesh::build(&family, Detail::Full).map_err(|error| js_error(error.into()))?;
        let mut live = self.borrow()?;
        // The material rides with the tree: these parameters state both, and a
        // tree drawn in the last tree's colours would be nobody's family.
        live.renderer.set_material(family.material);
        let submitted = live.renderer.submit(&mesh).map_err(js_error)?;
        Ok(submitted_json(&submitted))
    }

    /// Selects what the next frame draws of the tree that is up.
    #[wasm_bindgen(js_name = setView)]
    pub fn set_view(&self, view: &str) -> std::result::Result<(), JsError> {
        let view = View::from_id(view).ok_or_else(|| {
            JsError::new(&format!(
                "unknown view \"{view}\"; one of {}",
                View::NAMES.join(", ")
            ))
        })?;
        self.borrow()?.renderer.set_view(view);
        Ok(())
    }

    /// The sun, sky and ground the next frame is drawn under, as the row's
    /// own JSON. The panel reads the default out of here rather than keeping a
    /// second copy of it.
    pub fn scene(&self) -> std::result::Result<String, JsError> {
        Ok(self.borrow()?.renderer.scene().to_json())
    }

    /// Stands the sun somewhere else and paints the sky and the ground with
    /// it. The text is one whole row: what it names it states, and what it
    /// leaves out takes the default rather than whatever was set before. A row
    /// the renderer will not have leaves the sky exactly where it was and
    /// arrives in JavaScript as the renderer's own words.
    #[wasm_bindgen(js_name = setScene)]
    pub fn set_scene(&self, scene: &str) -> std::result::Result<(), JsError> {
        let row = SceneRow::parse(scene).map_err(js_error)?;
        self.borrow()?.renderer.set_scene(row);
        Ok(())
    }

    /// Where the camera stands and what it looks at, as JSON, or `null` before
    /// a tree has been submitted. The pose is solved from the bounds alone, so
    /// a browser frame and a headless still of one tree are one picture.
    #[wasm_bindgen(js_name = heroCamera)]
    pub fn hero_camera(&self) -> std::result::Result<String, JsError> {
        let mut live = self.borrow()?;
        let Some(bounds) = live.renderer.bounds() else {
            return Ok("null".into());
        };
        let camera = hero_pose(bounds, aspect(&live.config), GROUND_REACH);
        live.camera = camera;
        Ok(json!({
            "position": [camera.position.x, camera.position.y, camera.position.z],
            "target": [camera.target.x, camera.target.y, camera.target.z],
        })
        .to_string())
    }

    /// Stands the camera somewhere else, looking at the same kind of point.
    /// The near and far planes stay the pose's: they are solved from the
    /// subject, and orbiting does not change the subject.
    #[wasm_bindgen(js_name = setCamera)]
    pub fn set_camera(
        &self,
        x: f64,
        y: f64,
        z: f64,
        target_x: f64,
        target_y: f64,
        target_z: f64,
    ) -> std::result::Result<(), JsError> {
        let mut live = self.borrow()?;
        live.camera.position = Vec3::new(x, y, z);
        live.camera.target = Vec3::new(target_x, target_y, target_z);
        Ok(())
    }

    /// Reconfigures the canvas for a new size, clamped to what the device can
    /// hold. A size that has not changed costs nothing.
    pub fn resize(&self, width: u32, height: u32) -> std::result::Result<(), JsError> {
        self.borrow()?.resize(width, height);
        Ok(())
    }

    /// Draws one frame.
    pub fn frame(&self) -> std::result::Result<(), JsError> {
        self.borrow()?.draw(None).map_err(js_error)
    }

    /// What the last frame cost, as JSON.
    pub fn stats(&self) -> std::result::Result<String, JsError> {
        let stats = self.borrow()?.stats;
        Ok(json!({
            "drawCalls": stats.draw_calls,
            "triangles": stats.triangles,
            "instances": stats.instances,
        })
        .to_string())
    }

    /// Runs the whole timing protocol on what is up and resolves with the
    /// record, verdict included. An adapter that cannot be timed resolves with
    /// an unavailable record rather than rejecting: a missing measurement is a
    /// result the panel still has to show.
    pub fn timing(&self) -> js_sys::Promise {
        let live = Rc::clone(&self.live);
        future_to_promise(async move {
            session::measure(&live)
                .await
                .map(JsValue::from)
                .map_err(JsValue::from)
        })
    }

    /// Runs the same protocol while the camera makes one full turn about the
    /// pose it stands at, and resolves with the record: the GPU numbers when
    /// the adapter can be timed, and either way the frame-to-frame wall clock
    /// of ten seconds of turning, which is the frame rate a viewer would see.
    pub fn orbit(&self) -> js_sys::Promise {
        let live = Rc::clone(&self.live);
        future_to_promise(async move {
            session::orbit(&live)
                .await
                .map(JsValue::from)
                .map_err(JsValue::from)
        })
    }

    /// Drops the device and the canvas surface with it. Every later call says
    /// the renderer is disposed rather than reaching a device that is gone.
    pub fn dispose(&self) {
        self.live.borrow_mut().take();
    }
}

impl WebRenderer {
    fn borrow(&self) -> std::result::Result<std::cell::RefMut<'_, Live>, JsError> {
        borrow(&self.live)
    }
}

/// The live canvas, or the reason there is none. Kept out of the exported impl
/// so the timing session can reach it with a clone of the cell alone.
fn borrow(
    live: &Rc<RefCell<Option<Live>>>,
) -> std::result::Result<std::cell::RefMut<'_, Live>, JsError> {
    let cell = live
        .try_borrow_mut()
        .map_err(|_| JsError::new("the renderer is already drawing"))?;
    std::cell::RefMut::filter_map(cell, Option::as_mut)
        .map_err(|_| JsError::new("the renderer is disposed"))
}

fn submitted_json(submitted: &Submitted) -> String {
    let (min, max) = (submitted.bounds.min, submitted.bounds.max);
    json!({
        "woodVertices": submitted.wood_vertices,
        "woodTriangles": submitted.wood_triangles,
        "foliageInstances": submitted.foliage_instances,
        "bounds": { "min": [min.x, min.y, min.z], "max": [max.x, max.y, max.z] },
    })
    .to_string()
}

/// Every renderer failure reaches JavaScript as its own `Display` text, which
/// is what the panel's alert shows the owner.
fn js_error(error: RenderError) -> JsError {
    JsError::new(&error.to_string())
}
