//! The wood surfaced from its curve on the GPU each frame (fn-208, host
//! decisions 2, 4, 14 and 16 to 21): the curve's points, clusters and cells
//! uploaded once, and for each view - the camera's, and the sun's for its map
//! - output budgets sized from its pixels that compute passes fill and two
//! indirect draws read. The only wood path the renderer has. The CPU
//! reference is telperion_core's `Curve::tessellate`.
use crate::buffer::Owned;
use crate::programs::Program;
use crate::{device::Gpu, Camera};
use telperion_core::{
    math::Vec3,
    surface::{Curve, Demand, Viewer, CLUSTER_WORDS, POINT_WORDS, RIBBON, SECTION_FLOATS},
};

mod draw;
mod feedback;
mod target;
pub(crate) use draw::CurveDraw;
use feedback::Feedback;
pub use target::{Budget, CurveReport};
use target::{Shared, Target};

/// Floats a vertex the passes write: position, normal, the bark's (along,
/// angle) and the radius.
pub const VERTEX_FLOATS: u64 = 9;
/// Words of one ring's record.
const RECORD_WORDS: u64 = 22;
/// The error the wood is surfaced at, in pixels (the spec's half pixel).
pub const ERROR: f64 = 0.5;
/// The most one storage binding holds: WebGPU's default
/// `maxStorageBufferBindingSize`, so the passes bind the same slices in a
/// browser as natively (host decision 21).
pub const BINDING: u64 = 128 << 20;

/// The passes, in order, and the bindings each reads.
const PASSES: [(&str, &[u32]); 11] = [
    ("measure", &[0, 1, 2, 3, 5, 8, 9, 12]),
    ("choose", &[0, 8, 15]),
    ("measure_coarse", &[0, 1, 2, 3, 8, 9, 12]),
    ("choose_coarse", &[0, 8]),
    ("count", &[0, 1, 2, 3, 5, 8, 9, 12]),
    ("scan_local", &[0, 5, 6, 7]),
    ("scan_blocks", &[0, 7, 8, 15]),
    ("scan_add", &[0, 6, 7]),
    ("rings", &[0, 1, 2, 3, 5, 6, 8, 9, 12]),
    ("emit", &[0, 4, 8, 9, 10, 11, 12, 13, 14]),
    ("draws", &[0, 8, 15]),
];

/// The passes' programs: made once a device, on its first curve or when
/// the page prepares its programs, and shared by every tree's curve
/// (fn-243). In a browser they compile in the background.
pub(crate) fn passes(gpu: &Gpu) -> &[Program] {
    gpu.curve.get_or_init(|| {
        let module = crate::shaders::module(gpu, "curve", crate::shaders::curve());
        PASSES
            .iter()
            .map(|(entry, _)| {
                Program::make(
                    gpu,
                    &wgpu::ComputePipelineDescriptor {
                        label: Some(entry),
                        layout: None,
                        module: &module,
                        entry_point: Some(entry),
                        compilation_options: Default::default(),
                        cache: None,
                    },
                )
            })
            .collect()
    })
}

pub(crate) struct CurveGpu {
    points: Owned,
    /// Words of points the first binding holds, and the points' bytes.
    point_slice: u64,
    point_bytes: u64,
    clusters: Owned,
    sections: Owned,
    count: u32,
    /// What the most any view could ask of this tree rests on.
    demand: Demand,
    shape: [f32; 4],
    lobes: u32,
    /// The buffers both views write in turn, and each view's own.
    shared: Option<Shared>,
    pub(crate) camera: Option<Target>,
    pub(crate) sun: Option<Target>,
    /// What each view is surfaced from this frame, camera then sun.
    views: Option<[Viewer; 2]>,
    /// What each view asked, camera then sun, and the capacity its
    /// budget is held to.
    feedback: [Feedback; 2],
}

/// The view a compute pass surfaces the wood for.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Side {
    Camera,
    Sun,
}

fn filled(gpu: &Gpu, label: &str, data: &[u8]) -> Owned {
    // Copied out as the device's own blocks are (the resident digest).
    let usage = wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC;
    let b = target::buffer(gpu, label, data.len() as u64, usage);
    gpu.queue.write_buffer(&b, 0, data);
    b
}

/// `curve`'s sections packed on the device, one of none where it has none.
pub(crate) fn sections(gpu: &Gpu, curve: &Curve) -> Owned {
    let mut sections: Vec<[f32; SECTION_FLOATS]> = curve.packed_sections();
    if sections.is_empty() {
        sections.push([0.0; SECTION_FLOATS]);
    }
    filled(gpu, "curve sections", bytemuck::cast_slice(&sections))
}

impl CurveGpu {
    /// Uploads `curve` and builds its passes; each view's budget is made at
    /// its first frame, from its pixels.
    pub(crate) fn new(gpu: &Gpu, curve: &Curve) -> Self {
        let points: Vec<[u32; POINT_WORDS]> = curve.packed();
        let clusters: Vec<[u32; CLUSTER_WORDS]> = curve.packed_clusters();
        let blocks = (
            filled(gpu, "curve points", bytemuck::cast_slice(&points)),
            filled(gpu, "curve clusters", bytemuck::cast_slice(&clusters)),
        );
        let blocks = (blocks.0, blocks.1, sections(gpu, curve));
        let counts = (points.len() as u64, clusters.len() as u32);
        Self::on_device(gpu, curve, blocks, counts, curve.demand())
    }

    /// A curve already on the device in its packed form (`points`,
    /// `clusters` and `sections` words, `counts` of the first two) with
    /// the rows and demand `rows`' and `demand` give: the device's block
    /// packed after the CPU's (fn-212 task 7, host decision 2).
    pub(crate) fn on_device(
        gpu: &Gpu,
        rows: &Curve,
        (points, clusters, sections): (Owned, Owned, Owned),
        (point_count, count): (u64, u32),
        demand: Demand,
    ) -> Self {
        // A slice of whole points, on a 256-byte boundary.
        let per_slice = (BINDING / 256) * 256 / 4;
        let words = point_count * POINT_WORDS as u64;
        Self {
            points,
            point_slice: per_slice.min(words),
            point_bytes: words * 4,
            clusters,
            sections,
            count,
            demand,
            shape: [
                rows.lobe_depth as f32,
                rows.twist_rate as f32,
                rows.height as f32,
                0.0,
            ],
            lobes: rows.lobes,
            shared: None,
            camera: None,
            sun: None,
            views: None,
            feedback: [Feedback::new(gpu), Feedback::new(gpu)],
        }
    }

    /// The section's rows this curve draws by: lobe depth, twist, height,
    /// and the lobes.
    #[cfg(test)]
    pub(crate) fn rows(&self) -> ([f32; 4], u32) {
        (self.shape, self.lobes)
    }

    /// The packed points and clusters, how many of each, and the demand
    /// (fn-212 task 7's parity probe reads them back).
    #[cfg(test)]
    pub(crate) fn blocks(&self) -> (&wgpu::Buffer, &wgpu::Buffer, u64, u32, Demand) {
        let points = self.point_bytes / (POINT_WORDS as u64 * 4);
        (
            &self.points,
            &self.clusters,
            points,
            self.count,
            self.demand,
        )
    }

    /// The packed sections (the resident digest reads them back).
    #[cfg(test)]
    pub(crate) fn sections(&self) -> &wgpu::Buffer {
        &self.sections
    }

    /// Bytes on the device: the curve, the views' shared buffers and
    /// each view's own.
    pub(crate) fn bytes(&self) -> u64 {
        let views = [&self.camera, &self.sun]
            .into_iter()
            .flatten()
            .map(Target::bytes)
            .sum::<u64>();
        let shared = self.shared.as_ref().map_or(0, Shared::bytes);
        self.points.size() + self.clusters.size() + self.sections.size() + shared + views
    }

    /// The views' buffers let go, to be made again at the next frame.
    pub(crate) fn forget(&mut self) {
        (self.shared, self.camera, self.sun, self.views) = (None, None, None, None);
    }

    /// Fits both views to this frame: the camera's budget from its screen
    /// and the sun's from its map, the buffers they share as large as the
    /// larger asks. `None` until the passes can run: until the browser has
    /// compiled them the wood is not surfaced, and a frame draws none
    /// (fn-243). Otherwise whether the sun's view was made again, and so
    /// holds nothing until it is surfaced again with its map.
    pub(crate) fn fit(
        &mut self,
        gpu: &Gpu,
        camera: &Camera,
        viewport: (u32, u32),
        light: &crate::shadow::Light,
    ) -> Option<bool> {
        let Some(pipelines) = compiled(gpu) else {
            self.views = None;
            return None;
        };
        let pixels = u64::from(viewport.0.max(1)) * u64::from(viewport.1.max(1));
        // The sun's map is fitted to the tree, so the tree covers most of its
        // texels: the spruce asks 2.23 rings, 6.71 vertices and 18.94 ribbon
        // indices a texel of the whole map (STEP5.md).
        let texels = u64::from(crate::shadow::RESOLUTION).pow(2);
        // Each view's budget is the lesser of its screen's and the most the
        // tree could ask of a view that sees none of it nearer than its own
        // nearest pixels a metre (host decision 23).
        let camera_view = viewer(camera, viewport);
        let nearest = [
            camera_view.pixels_per_metre / camera_view.near.max(1e-6),
            1.0 / light.texel_size,
        ];
        let (demand, limits) = (self.demand, gpu.device.limits());
        let budget =
            |p: u64, near: f64| Budget::for_pixels(p, &limits).within(demand.at(near, ERROR));
        // Each held to the capacity its own demand grew to, once read.
        for f in &mut self.feedback {
            f.read();
        }
        let held = |b: Budget, f: &Feedback| b.within(f.capacity());
        let wanted = [
            held(budget(pixels, nearest[0]), &self.feedback[0]),
            held(budget(texels, nearest[1]), &self.feedback[1]),
        ];
        if self.shared.as_ref().is_none_or(|s| !s.fits(wanted)) {
            self.forget();
            self.shared = Some(Shared::new(gpu, self, wanted));
        }
        let shared = self.shared.as_ref().expect("made above");
        let made = |t: &Option<Target>, label, budget| {
            let current = t.as_ref().is_some_and(|t| t.budget == budget);
            (!current).then(|| Target::new(gpu, label, budget, (self, shared), &pipelines))
        };
        let camera_target = made(&self.camera, "curve camera", wanted[0]);
        let sun_target = made(&self.sun, "curve sun", wanted[1]);
        let remade = sun_target.is_some();
        self.camera = camera_target.or(self.camera.take());
        self.sun = sun_target.or(self.sun.take());
        let d = light.direction;
        let sun_view = Viewer {
            eye: Vec3::ZERO,
            forward: -Vec3::new(f64::from(d[0]), f64::from(d[1]), f64::from(d[2])),
            pixels_per_metre: 1.0 / light.texel_size,
            near: 1e-3,
            orthographic: true,
            planes: Some(wide(crate::select::frame::planes(&light.view_projection))),
        };
        self.views = Some([camera_view, sun_view]);
        Some(remade)
    }

    /// Surfaces the wood for `side` in one compute pass, which `timestamps`
    /// stand around, as the frame's last `fit` sized it. A frame surfaces
    /// the sun's view, draws its map, and only then the camera's: the two
    /// write the same buffers.
    pub(crate) fn surface(
        &mut self,
        gpu: &Gpu,
        encoder: &mut wgpu::CommandEncoder,
        side: Side,
        timestamps: Option<wgpu::ComputePassTimestampWrites<'_>>,
    ) {
        let (Some(pipelines), Some(views)) = (compiled(gpu), &self.views) else {
            return;
        };
        let (target, view, k) = match side {
            Side::Camera => (&self.camera, &views[0], 0),
            Side::Sun => (&self.sun, &views[1], 1),
        };
        let t = target.as_ref().expect("fitted with its views");
        gpu.queue
            .write_buffer(&t.config, 0, bytemuck::cast_slice(&self.config(t, view)));
        encoder.clear_buffer(&t.totals, 0, None);
        let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
            label: Some("curve"),
            timestamp_writes: timestamps,
        });
        let groups = self.count.div_ceil(256).max(1);
        let row = groups.min(gpu.device.limits().max_compute_workgroups_per_dimension);
        for (k, pipeline) in pipelines.iter().enumerate() {
            pass.set_pipeline(pipeline);
            pass.set_bind_group(0, &t.groups[k], &[]);
            match PASSES[k].0 {
                "choose" | "choose_coarse" | "draws" | "scan_blocks" => {
                    pass.dispatch_workgroups(1, 1, 1)
                }
                "emit" => pass.dispatch_workgroups_indirect(&t.args, 40),
                "measure_coarse" => pass.dispatch_workgroups_indirect(&t.args, 52),
                _ => pass.dispatch_workgroups(row, groups.div_ceil(row), 1),
            }
        }
        drop(pass);
        let totals = (*t.totals).clone();
        self.feedback[k].copy(encoder, &totals);
    }

    /// Maps the demand each view's passes copied, once the frame that
    /// copied it is submitted.
    pub(crate) fn submitted(&mut self) {
        for f in &mut self.feedback {
            f.map();
        }
    }

    fn config(&self, t: &Target, v: &Viewer) -> [u32; 52] {
        let f = |x: f64| (x as f32).to_bits();
        let mut c = [0u32; 52];
        c[..4].copy_from_slice(&[f(v.eye.x), f(v.eye.y), f(v.eye.z), f(v.near)]);
        let d = v.forward;
        c[4..8].copy_from_slice(&[f(d.x), f(d.y), f(d.z), f(v.pixels_per_metre)]);
        if let Some(planes) = &v.planes {
            for (k, p) in planes.iter().enumerate() {
                c[8 + 4 * k..12 + 4 * k].copy_from_slice(&p.map(f));
            }
        }
        let s = self.shape;
        c[32..36].copy_from_slice(&[f(ERROR), s[0].to_bits(), s[1].to_bits(), s[2].to_bits()]);
        let culling = u32::from(v.planes.is_some());
        c[36..40].copy_from_slice(&[self.lobes, u32::from(v.orthographic), culling, self.count]);
        let b = t.budget;
        c[40..44].copy_from_slice(&[b.vertices, b.tube_indices, b.ribbon_indices, b.rings]);
        let groups = self.count.div_ceil(256).max(1);
        let row = groups.min(65535);
        c[44..48].copy_from_slice(&[row, groups, f(RIBBON), self.point_slice as u32]);
        c[48..51].copy_from_slice(&[
            t.vertex_slice as u32,
            t.index_slice as u32,
            t.record_slice as u32,
        ]);
        c
    }
}

/// The passes' pipelines, where every one has compiled.
fn compiled(gpu: &Gpu) -> Option<Vec<&wgpu::ComputePipeline>> {
    passes(gpu).iter().map(Program::compiled).collect()
}

/// The camera's view as the passes and the CPU reference both read it: its
/// eye and direction, pixels a metre at one metre for this viewport's
/// height, its near plane and its frustum's planes.
pub fn viewer(camera: &Camera, viewport: (u32, u32)) -> Viewer {
    let aspect = f64::from(viewport.0.max(1)) / f64::from(viewport.1.max(1));
    let pixels =
        crate::select::frame::pixels_per_metre(f64::from(viewport.1), camera.field_of_view);
    Viewer {
        eye: camera.position,
        forward: (camera.target - camera.position).normalized(),
        pixels_per_metre: pixels,
        near: camera.near,
        orthographic: false,
        planes: Some(wide(crate::select::frame::planes(
            &camera.view_projection(aspect),
        ))),
    }
}

fn wide(planes: [[f32; 4]; 6]) -> [[f64; 4]; 6] {
    planes.map(|p| p.map(f64::from))
}
