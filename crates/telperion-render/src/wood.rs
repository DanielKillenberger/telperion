//! The wood: the tree's curve surfaced on the device at the screen's error
//! (fn-208) and drawn with its bark.
use telperion_core::{material::MaterialParams, surface::Curve};

use crate::{device::Gpu, FrameStats};

/// Whether a material draws any of smooth bark's terms: lichen, lenticels or
/// a peeling strip. One that draws none is drawn by the pipeline without them.
pub fn smooth_bark(m: &MaterialParams) -> bool {
    (m.lichen_strength > 0.0 && m.lichen_scale > 0.0)
        || (m.lenticel_strength > 0.0 && m.lenticel_length > 0.0)
        || m.peel_curl > 0.0
}

/// The wood's one path (fn-208, host decision 20): the tree's curve on the
/// device, surfaced each frame for the camera and the sun, and the pipelines
/// it is drawn through, with smooth bark's terms and without them.
pub struct Wood {
    smooth_on: bool,
    /// A measurement's override: the viewport the wood is surfaced for, in
    /// place of the frame's, so two frames share one tessellation.
    pinned: Option<(u32, u32)>,
    curve: Option<crate::curve::CurveGpu>,
    draw: crate::curve::CurveDraw,
}

impl Wood {
    pub(crate) fn allocated_bytes(&self) -> u64 {
        self.curve.as_ref().map_or(0, |c| c.bytes())
    }

    /// Lets the curve's buffers go: nothing is drawn until the next submit.
    pub(crate) fn release(&mut self) {
        self.curve = None;
    }

    pub fn new(
        gpu: &Gpu,
        layout: &wgpu::BindGroupLayout,
        shadow: &crate::shadow::Shadow,
        surface: crate::pass::Surface,
    ) -> Self {
        let shader = crate::shaders::module(gpu, "wood", crate::shaders::wood(false));
        let smooth = crate::shaders::module(gpu, "smooth wood", crate::shaders::wood(true));
        Self {
            smooth_on: false,
            pinned: None,
            curve: None,
            draw: crate::curve::CurveDraw::new(gpu, layout, shadow, surface, [&shader, &smooth]),
        }
    }

    /// Uploads a tree's wood as its curve; no wood leaves nothing drawn.
    pub fn submit(&mut self, gpu: &Gpu, curve: &Curve) {
        self.curve = (!curve.clusters.is_empty()).then(|| crate::curve::CurveGpu::new(gpu, curve));
    }

    /// Takes a curve already on the device (fn-212 task 7).
    pub(crate) fn submit_on_device(&mut self, curve: crate::curve::CurveGpu) {
        self.curve = Some(curve);
    }

    /// Surfaces the wood for `viewport` whatever the frame's size, or for the
    /// frame again with `None`: a measurement override, never a production
    /// path.
    pub fn pin_viewport(&mut self, viewport: Option<(u32, u32)>) {
        self.pinned = viewport;
    }

    /// Which of the two lit pipelines the material draws through.
    pub fn set_material(&mut self, material: &MaterialParams) {
        self.smooth_on = smooth_bark(material);
    }

    /// Records the frame's passes over the curve, where there is one: the
    /// camera's always, the sun's where `sun` asks. Whether the wood is
    /// surfaced, which it is not while its programs compile.
    pub fn tessellate(
        &mut self,
        gpu: &Gpu,
        encoder: &mut wgpu::CommandEncoder,
        camera: &crate::Camera,
        viewport: (u32, u32),
        light: &crate::shadow::Light,
        sun: bool,
        timestamps: Option<wgpu::ComputePassTimestampWrites<'_>>,
    ) -> bool {
        let viewport = self.pinned.unwrap_or(viewport);
        match &mut self.curve {
            Some(curve) => curve.record(gpu, encoder, camera, viewport, light, sun, timestamps),
            // A timed frame writes its pair whether or not there is wood.
            None => {
                if timestamps.is_some() {
                    drop(encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                        label: Some("curve"),
                        timestamp_writes: timestamps,
                    }));
                }
                true
            }
        }
    }

    /// The curve's views, where there is a curve.
    pub(crate) fn curve(&self) -> Option<&crate::curve::CurveGpu> {
        self.curve.as_ref()
    }

    /// The curve's view targets let go, to be made again at the next frame.
    pub(crate) fn forget_targets(&mut self) {
        if let Some(curve) = &mut self.curve {
            (curve.camera, curve.sun) = (None, None);
        }
    }

    /// Draws the surface, or nothing when no tree has been submitted.
    pub fn draw(&self, pass: &mut wgpu::RenderPass<'_>) -> FrameStats {
        match &self.curve {
            Some(curve) => self.draw.draw(pass, curve, self.smooth_on),
            None => FrameStats::default(),
        }
    }

    /// The wood into the sun's map.
    pub fn draw_shadow(&self, pass: &mut wgpu::RenderPass<'_>) {
        if let Some(curve) = &self.curve {
            self.draw.draw_shadow(pass, curve);
        }
    }
}
