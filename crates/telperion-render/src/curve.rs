//! The wood surfaced from its curve on the GPU each frame (fn-208, host
//! decisions 2, 4, 14 and 16 to 21): the curve's points, clusters and cells
//! uploaded once, and for each view - the camera's, and the sun's for its map
//! - output budgets sized from its pixels that compute passes fill and two
//! indirect draws read. The only wood path the renderer has. The CPU
//! reference is telperion_core's `Curve::tessellate`.
use crate::{device::Gpu, Camera};
use telperion_core::{
    math::Vec3,
    surface::{Curve, Viewer, CLUSTER_WORDS, POINT_WORDS, RIBBON, SECTION_FLOATS},
};

mod draw;
mod target;
pub(crate) use draw::CurveDraw;
use target::Target;
pub use target::{Budget, CurveReport};

/// Floats a vertex the passes write: position, normal, the bark's (along,
/// angle) and the radius.
pub const VERTEX_FLOATS: u64 = 9;
/// Words of one ring's record.
const RECORD_WORDS: u64 = 24;
/// The error the wood is surfaced at, in pixels (the spec's half pixel).
pub const ERROR: f64 = 0.5;
/// The most one storage binding holds: WebGPU's default
/// `maxStorageBufferBindingSize`, so the passes bind the same slices in a
/// browser as natively (host decision 21).
pub const BINDING: u64 = 128 << 20;

/// The passes, in order, and the bindings each reads.
const PASSES: [(&str, &[u32]); 9] = [
    ("measure", &[0, 1, 2, 3, 8, 9]),
    ("choose", &[0, 8]),
    ("count", &[0, 1, 2, 3, 5, 8, 9]),
    ("scan_local", &[0, 5, 6, 7]),
    ("scan_blocks", &[0, 7, 8, 15]),
    ("scan_add", &[0, 6, 7]),
    ("rings", &[0, 1, 2, 3, 5, 6, 8, 9]),
    ("emit", &[0, 4, 8, 9, 10, 11, 12, 13, 14]),
    ("draws", &[0, 8, 15]),
];

/// The curve on the device: its points (bound in two slices), clusters and
/// cells, the passes, and the two views' targets.
pub(crate) struct CurveGpu {
    pipelines: Vec<wgpu::ComputePipeline>,
    points: wgpu::Buffer,
    /// Words of points the first binding holds, and the points' bytes.
    point_slice: u64,
    point_bytes: u64,
    clusters: wgpu::Buffer,
    sections: wgpu::Buffer,
    count: u32,
    shape: [f32; 4],
    lobes: u32,
    pub(crate) camera: Option<Target>,
    pub(crate) sun: Option<Target>,
}

fn filled(gpu: &Gpu, label: &str, data: &[u8]) -> wgpu::Buffer {
    let b = target::buffer(gpu, label, data.len() as u64, wgpu::BufferUsages::STORAGE);
    gpu.queue.write_buffer(&b, 0, data);
    b
}

impl CurveGpu {
    /// Uploads `curve` and builds its passes; each view's budget is made at
    /// its first frame, from its pixels.
    pub(crate) fn new(gpu: &Gpu, curve: &Curve) -> Self {
        let points: Vec<[u32; POINT_WORDS]> = curve.packed();
        let clusters: Vec<[u32; CLUSTER_WORDS]> = curve.packed_clusters();
        let mut sections: Vec<[f32; SECTION_FLOATS]> = curve.packed_sections();
        if sections.is_empty() {
            sections.push([0.0; SECTION_FLOATS]);
        }
        let module = gpu
            .device
            .create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("curve"),
                source: wgpu::ShaderSource::Wgsl(
                    format!(
                        "{}\n{}\n{}",
                        include_str!("shaders/curve.wgsl"),
                        include_str!("shaders/curve_walk.wgsl"),
                        include_str!("shaders/curve_emit.wgsl")
                    )
                    .into(),
                ),
            });
        let pipelines = PASSES
            .iter()
            .map(|(entry, _)| {
                gpu.device
                    .create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
                        label: Some(entry),
                        layout: None,
                        module: &module,
                        entry_point: Some(entry),
                        compilation_options: Default::default(),
                        cache: None,
                    })
            })
            .collect();
        // A slice of whole points, on a 256-byte boundary.
        let per_slice = (BINDING / 256) * 256 / 4;
        Self {
            pipelines,
            points: filled(gpu, "curve points", bytemuck::cast_slice(&points)),
            point_slice: per_slice.min((points.len() * POINT_WORDS) as u64),
            point_bytes: (points.len() * POINT_WORDS * 4) as u64,
            clusters: filled(gpu, "curve clusters", bytemuck::cast_slice(&clusters)),
            sections: filled(gpu, "curve sections", bytemuck::cast_slice(&sections)),
            count: clusters.len() as u32,
            shape: [
                curve.lobe_depth as f32,
                curve.twist_rate as f32,
                curve.height as f32,
                0.0,
            ],
            lobes: curve.lobes,
            camera: None,
            sun: None,
        }
    }

    /// Bytes on the device: the curve and both views' budgets.
    pub(crate) fn bytes(&self) -> u64 {
        let views = [&self.camera, &self.sun]
            .into_iter()
            .flatten()
            .map(Target::bytes)
            .sum::<u64>();
        self.points.size() + self.clusters.size() + self.sections.size() + views
    }

    /// Surfaces the wood for the camera and for the sun into their budgets,
    /// in one compute pass the timer's fourth pair stands around.
    pub(crate) fn record(
        &mut self,
        gpu: &Gpu,
        encoder: &mut wgpu::CommandEncoder,
        camera: &Camera,
        viewport: (u32, u32),
        light: &crate::shadow::Light,
        timestamps: Option<wgpu::ComputePassTimestampWrites<'_>>,
    ) {
        let pixels = u64::from(viewport.0.max(1)) * u64::from(viewport.1.max(1));
        // The tree fills a fraction of the sun's fitted map: a quarter of its
        // texels is the view the sun's budget is sized for.
        let texels = u64::from(crate::shadow::RESOLUTION).pow(2) / 4;
        let budget = |p| Budget::for_pixels(p, &gpu.device.limits());
        if self
            .camera
            .as_ref()
            .is_none_or(|t| t.budget != budget(pixels))
        {
            self.camera = Some(Target::new(gpu, "curve camera", budget(pixels), self));
        }
        if self.sun.is_none() {
            self.sun = Some(Target::new(gpu, "curve sun", budget(texels), self));
        }
        let d = light.direction;
        let sun = Viewer {
            eye: Vec3::ZERO,
            forward: -Vec3::new(f64::from(d[0]), f64::from(d[1]), f64::from(d[2])),
            pixels_per_metre: 1.0 / light.texel_size,
            near: 1e-3,
            orthographic: true,
            planes: Some(wide(crate::select::frame::planes(&light.view_projection))),
        };
        let views = [
            (self.camera.as_ref().unwrap(), viewer(camera, viewport)),
            (self.sun.as_ref().unwrap(), sun),
        ];
        for (t, view) in &views {
            gpu.queue
                .write_buffer(&t.config, 0, bytemuck::cast_slice(&self.config(t, view)));
            encoder.clear_buffer(&t.totals, 0, None);
        }
        let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
            label: Some("curve"),
            timestamp_writes: timestamps,
        });
        let groups = self.count.div_ceil(256).max(1);
        let row = groups.min(gpu.device.limits().max_compute_workgroups_per_dimension);
        for (t, _) in &views {
            for (k, pipeline) in self.pipelines.iter().enumerate() {
                pass.set_pipeline(pipeline);
                pass.set_bind_group(0, &t.groups[k], &[]);
                match PASSES[k].0 {
                    "choose" | "draws" | "scan_blocks" => pass.dispatch_workgroups(1, 1, 1),
                    "emit" => pass.dispatch_workgroups_indirect(&t.args, 40),
                    _ => pass.dispatch_workgroups(row, groups.div_ceil(row), 1),
                }
            }
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
        c[48..50].copy_from_slice(&[t.vertex_slice as u32, t.index_slice as u32]);
        c
    }
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
