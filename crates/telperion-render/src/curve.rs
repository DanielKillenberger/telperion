//! The wood surfaced from its curve on the GPU each frame (fn-208, host
//! decisions 2, 4 and 14): the curve's points, clusters and cells uploaded
//! once, and for each view - the camera's, and the sun's for its map - a
//! budget of vertices and indices that compute passes fill and two indirect
//! draws read. The CPU reference is telperion_core's `Curve::tessellate`.
use crate::{device::Gpu, Camera};
use telperion_core::{
    math::Vec3,
    surface::{Curve, Viewer, CLUSTER_WORDS, POINT_WORDS, RIBBON, SECTION_FLOATS},
};

mod draw;
pub(crate) use draw::CurveDraw;

/// Floats a vertex the passes write: position, normal, the bark's (along,
/// angle), the radius and the coverage.
pub const VERTEX_FLOATS: u64 = 10;
/// The error the wood is surfaced at, in pixels (the spec's half pixel).
pub const ERROR: f64 = 0.5;

/// The most one view writes: vertices, tube indices and ribbon indices.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Budget {
    pub vertices: u32,
    pub tube_indices: u32,
    pub ribbon_indices: u32,
}

/// The camera's budget, sized from the screen rather than the tree.
pub const CAMERA: Budget = Budget {
    vertices: 3_000_000,
    tube_indices: 12_000_000,
    ribbon_indices: 6_000_000,
};
/// The sun's: tubes alone, no ribbons.
pub const SUN: Budget = Budget {
    vertices: 1_000_000,
    tube_indices: 4_000_000,
    ribbon_indices: 0,
};

/// What one view's passes wrote, read back from the device.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct CurveReport {
    /// The error scale the budget let the view be drawn at.
    pub scale: f64,
    /// 1: no scale fitted the budget; 2: a cluster past it was left out.
    pub overrun: u32,
    pub vertices: u32,
    pub tube_triangles: u32,
    pub ribbon_triangles: u32,
}

/// One view's target buffers and the bind group each pass reads them by.
pub(crate) struct Target {
    pub(crate) budget: Budget,
    config: wgpu::Buffer,
    totals: wgpu::Buffer,
    pub(crate) vertices: wgpu::Buffer,
    pub(crate) indices: wgpu::Buffer,
    pub(crate) args: wgpu::Buffer,
    groups: Vec<wgpu::BindGroup>,
}

/// The passes, in order, and the bindings each reads.
const PASSES: [(&str, &[u32]); 8] = [
    ("measure", &[0, 1, 2, 3, 7, 8, 9]),
    ("choose", &[0, 7]),
    ("count", &[0, 1, 2, 3, 4, 7, 8, 9]),
    ("scan_local", &[0, 4, 5, 6]),
    ("scan_blocks", &[0, 6, 7]),
    ("scan_add", &[0, 5, 6]),
    ("emit", &[0, 1, 2, 3, 4, 5, 7, 8, 9]),
    ("draws", &[0, 7, 10]),
];

/// The curve on the device, its passes and its two views.
pub(crate) struct CurveGpu {
    pipelines: Vec<wgpu::ComputePipeline>,
    shared: [wgpu::Buffer; 3],
    clusters: u32,
    shape: [f32; 4],
    lobes: u32,
    pub(crate) camera: Target,
    pub(crate) sun: Target,
}

fn buffer(gpu: &Gpu, label: &str, bytes: u64, usage: wgpu::BufferUsages) -> wgpu::Buffer {
    gpu.device.create_buffer(&wgpu::BufferDescriptor {
        label: Some(label),
        size: bytes.max(16).div_ceil(4) * 4,
        usage: usage | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    })
}

fn filled(gpu: &Gpu, label: &str, data: &[u8]) -> wgpu::Buffer {
    let b = buffer(gpu, label, data.len() as u64, wgpu::BufferUsages::STORAGE);
    gpu.queue.write_buffer(&b, 0, data);
    b
}

impl CurveGpu {
    /// Uploads `curve` and builds its passes and both views' budgets.
    pub(crate) fn new(gpu: &Gpu, curve: &Curve) -> Self {
        let points: Vec<[u32; POINT_WORDS]> = curve.packed();
        let clusters: Vec<[u32; CLUSTER_WORDS]> = curve.packed_clusters();
        let mut sections: Vec<[f32; SECTION_FLOATS]> = curve.packed_sections();
        if sections.is_empty() {
            sections.push([0.0; SECTION_FLOATS]);
        }
        let shared = [
            filled(gpu, "curve points", bytemuck::cast_slice(&points)),
            filled(gpu, "curve clusters", bytemuck::cast_slice(&clusters)),
            filled(gpu, "curve sections", bytemuck::cast_slice(&sections)),
        ];
        let module = gpu
            .device
            .create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("curve"),
                source: wgpu::ShaderSource::Wgsl(
                    format!(
                        "{}\n{}",
                        include_str!("shaders/curve.wgsl"),
                        include_str!("shaders/curve_walk.wgsl")
                    )
                    .into(),
                ),
            });
        let pipelines: Vec<_> = PASSES
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
        let n = clusters.len() as u32;
        let target = |label, budget| Target::new(gpu, label, budget, n, &shared, &pipelines);
        Self {
            camera: target("curve camera", CAMERA),
            sun: target("curve sun", SUN),
            pipelines,
            shared,
            clusters: n,
            shape: [
                curve.lobe_depth as f32,
                curve.twist_rate as f32,
                curve.height as f32,
                0.0,
            ],
            lobes: curve.lobes,
        }
    }

    /// Bytes on the device: the curve and both views' budgets.
    pub(crate) fn bytes(&self) -> u64 {
        self.shared.iter().map(|b| b.size()).sum::<u64>() + self.camera.bytes() + self.sun.bytes()
    }

    /// Surfaces the wood for the camera and for the sun into their budgets.
    pub(crate) fn record(
        &self,
        gpu: &Gpu,
        encoder: &mut wgpu::CommandEncoder,
        camera: &Camera,
        viewport: (u32, u32),
        light: &crate::shadow::Light,
    ) {
        self.pass(gpu, encoder, &self.camera, &viewer(camera, viewport), true);
        let d = light.direction;
        let sun = Viewer {
            eye: Vec3::ZERO,
            forward: -Vec3::new(f64::from(d[0]), f64::from(d[1]), f64::from(d[2])),
            pixels_per_metre: 1.0 / light.texel_size,
            near: 1e-3,
            orthographic: true,
            planes: Some(wide(crate::select::frame::planes(&light.view_projection))),
        };
        self.pass(gpu, encoder, &self.sun, &sun, false);
    }

    fn pass(
        &self,
        gpu: &Gpu,
        encoder: &mut wgpu::CommandEncoder,
        t: &Target,
        view: &Viewer,
        ribbons: bool,
    ) {
        let groups = self.clusters.div_ceil(256).max(1);
        let maximum = gpu.device.limits().max_compute_workgroups_per_dimension;
        let row = groups.min(maximum);
        let config = self.config(t, view, ribbons, row, groups);
        gpu.queue
            .write_buffer(&t.config, 0, bytemuck::cast_slice(&config));
        encoder.clear_buffer(&t.totals, 0, None);
        let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
            label: Some("curve"),
            timestamp_writes: None,
        });
        for (k, pipeline) in self.pipelines.iter().enumerate() {
            pass.set_pipeline(pipeline);
            pass.set_bind_group(0, &t.groups[k], &[]);
            match PASSES[k].0 {
                "choose" | "draws" | "scan_blocks" => pass.dispatch_workgroups(1, 1, 1),
                _ => pass.dispatch_workgroups(row, groups.div_ceil(row), 1),
            }
        }
    }

    fn config(&self, t: &Target, v: &Viewer, ribbons: bool, row: u32, blocks: u32) -> [u32; 48] {
        let f = |x: f64| (x as f32).to_bits();
        let mut c = [0u32; 48];
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
        c[36..40].copy_from_slice(&[
            self.lobes,
            u32::from(v.orthographic),
            culling,
            self.clusters,
        ]);
        let b = t.budget;
        c[40..44].copy_from_slice(&[b.vertices, b.tube_indices, b.ribbon_indices, row]);
        c[44..48].copy_from_slice(&[u32::from(ribbons), blocks, f(RIBBON), 0]);
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

impl Target {
    fn new(
        gpu: &Gpu,
        label: &str,
        budget: Budget,
        clusters: u32,
        shared: &[wgpu::Buffer; 3],
        pipelines: &[wgpu::ComputePipeline],
    ) -> Self {
        use wgpu::BufferUsages as U;
        let per = u64::from(clusters.max(1)) * 16;
        let blocks = u64::from(clusters.div_ceil(256).max(1)) * 16;
        let indices = u64::from(budget.tube_indices + budget.ribbon_indices) * 4;
        let made = [
            buffer(gpu, label, 192, U::UNIFORM),
            buffer(gpu, label, per, U::STORAGE),
            buffer(gpu, label, per, U::STORAGE),
            buffer(gpu, label, blocks, U::STORAGE),
            buffer(gpu, label, 128, U::STORAGE | U::COPY_SRC),
            buffer(
                gpu,
                label,
                u64::from(budget.vertices) * VERTEX_FLOATS * 4,
                U::STORAGE | U::VERTEX | U::COPY_SRC,
            ),
            buffer(gpu, label, indices, U::STORAGE | U::INDEX | U::COPY_SRC),
            buffer(gpu, label, 40, U::STORAGE | U::INDIRECT | U::COPY_SRC),
        ];
        let [config, counts, offsets, block, totals, vertices, index, args] = made;
        let by = |binding: u32| -> &wgpu::Buffer {
            match binding {
                0 => &config,
                1..=3 => &shared[binding as usize - 1],
                4 => &counts,
                5 => &offsets,
                6 => &block,
                7 => &totals,
                8 => &vertices,
                9 => &index,
                _ => &args,
            }
        };
        let groups = PASSES
            .iter()
            .zip(pipelines)
            .map(|((entry, bindings), pipeline)| {
                let entries: Vec<_> = bindings
                    .iter()
                    .map(|&b| wgpu::BindGroupEntry {
                        binding: b,
                        resource: by(b).as_entire_binding(),
                    })
                    .collect();
                gpu.device.create_bind_group(&wgpu::BindGroupDescriptor {
                    label: Some(entry),
                    layout: &pipeline.get_bind_group_layout(0),
                    entries: &entries,
                })
            })
            .collect();
        let _keep = (counts, offsets, block);
        Self {
            budget,
            config,
            totals,
            vertices,
            indices: index,
            args,
            groups,
        }
    }

    /// Bytes this view holds on the device.
    pub(crate) fn bytes(&self) -> u64 {
        self.vertices.size() + self.indices.size() + self.args.size()
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl Target {
    /// What the last frame's passes wrote, read back.
    pub(crate) fn report(&self, gpu: &Gpu) -> CurveReport {
        let words: Vec<u32> = bytemuck::cast_slice(&read(gpu, &self.totals, 128)).to_vec();
        CurveReport {
            scale: f64::from(1u32 << words[16].min(3)),
            overrun: words[17],
            vertices: words[24],
            tube_triangles: words[25] / 3,
            ribbon_triangles: words[26] / 3,
        }
    }

    /// The last frame's vertices (ten floats each), tube and ribbon indices.
    pub(crate) fn mesh(&self, gpu: &Gpu) -> (Vec<f32>, Vec<u32>, Vec<u32>) {
        let r = self.report(gpu);
        let floats = u64::from(r.vertices) * VERTEX_FLOATS;
        let vertices = bytemuck::cast_slice(&read(gpu, &self.vertices, floats * 4)).to_vec();
        let all: Vec<u32> =
            bytemuck::cast_slice(&read(gpu, &self.indices, self.indices.size())).to_vec();
        let tube = all[..(r.tube_triangles * 3) as usize].to_vec();
        let base = self.budget.tube_indices as usize;
        let ribbon = all[base..base + (r.ribbon_triangles * 3) as usize].to_vec();
        (vertices, tube, ribbon)
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn read(gpu: &Gpu, source: &wgpu::Buffer, bytes: u64) -> Vec<u8> {
    let bytes = bytes.max(4).div_ceil(4) * 4;
    let staging = gpu.device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("curve readback"),
        size: bytes,
        usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
        mapped_at_creation: false,
    });
    let mut encoder = gpu.device.create_command_encoder(&Default::default());
    encoder.copy_buffer_to_buffer(source, 0, &staging, 0, bytes);
    gpu.queue.submit([encoder.finish()]);
    let slice = staging.slice(..);
    slice.map_async(wgpu::MapMode::Read, |_| {});
    let _ = gpu.device.poll(wgpu::PollType::wait_indefinitely());
    let data = slice.get_mapped_range().expect("mapped").to_vec();
    staging.unmap();
    data
}
