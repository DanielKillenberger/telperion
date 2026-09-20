//! The plaited wood surface: the core's own position, normal, coordinate and
//! index arrays uploaded as they lie in memory, drawn as one indexed mesh.
#[cfg(all(test, not(target_arch = "wasm32")))]
mod calibration;
mod radius;

use telperion_core::{
    material::MaterialParams,
    surface::{SurfaceMesh, SurfaceRun},
};

use crate::{
    buffer::{self, Held, Region},
    device::Gpu,
    FrameStats,
};

/// The core keeps positions, normals and surface coordinates in separate
/// arrays, so the pipeline takes three vertex buffers and no array is
/// interleaved on the way up.
const POSITION: [wgpu::VertexAttribute; 1] = wgpu::vertex_attr_array![0 => Float32x3];
const NORMAL: [wgpu::VertexAttribute; 1] = wgpu::vertex_attr_array![1 => Float32x3];
const COORD: [wgpu::VertexAttribute; 1] = wgpu::vertex_attr_array![2 => Float32x2];

/// The switch the wood stages are built on, as the shader states it.
const SMOOTH_ON: &str = "const SMOOTH_BARK: bool = true;";

/// Whether a material draws any of smooth bark's terms: lichen, lenticels or
/// a peeling strip. One that draws none is drawn by the pipeline without them.
pub fn smooth_bark(m: &MaterialParams) -> bool {
    (m.lichen_strength > 0.0 && m.lichen_scale > 0.0)
        || (m.lenticel_strength > 0.0 && m.lenticel_length > 0.0)
        || m.peel_curl > 0.0
}

/// The wood pipeline twice - with smooth bark's terms and without them - the
/// depth-only one the sun draws it through, and the buffers one tree's
/// surface lives in.
pub struct Wood {
    pipeline: wgpu::RenderPipeline,
    smooth: wgpu::RenderPipeline,
    smooth_on: bool,
    shadow: wgpu::RenderPipeline,
    positions: Option<Held>,
    normals: Option<Held>,
    coords: Option<Held>,
    indices: Option<Held>,
    radii: Option<Held>,
    radius_layout: wgpu::BindGroupLayout,
    radius_group: Option<wgpu::BindGroup>,
    index_count: u32,
    runs: Vec<SurfaceRun>,
    pub caster_index_count: u32,
}

impl Wood {
    pub(crate) fn allocated_bytes(&self) -> u64 {
        [
            &self.positions,
            &self.normals,
            &self.coords,
            &self.indices,
            &self.radii,
        ]
        .into_iter()
        .flatten()
        .map(|b| b.region().capacity())
        .sum::<u64>()
    }

    pub fn new(
        gpu: &Gpu,
        layout: &wgpu::BindGroupLayout,
        shadow: &crate::shadow::Shadow,
        surface: crate::pass::Surface,
    ) -> Self {
        let stages = format!(
            "{}\n{}\n{}\n{}",
            include_str!("shaders/bark.wgsl"),
            include_str!("shaders/plates.wgsl"),
            include_str!("shaders/smooth.wgsl"),
            include_str!("shaders/wood.wgsl")
        );
        assert!(
            stages.contains(SMOOTH_ON),
            "the wood stages lost their switch"
        );
        let plain = stages.replace(SMOOTH_ON, "const SMOOTH_BARK: bool = false;");
        let shader = crate::pass::lit_shader(gpu, "wood", &plain);
        let smooth = crate::pass::lit_shader(gpu, "smooth wood", &stages);
        let radius_layout = gpu
            .device
            .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("wood radii"),
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
            });
        let vertex = |attributes, floats: u64| {
            Some(wgpu::VertexBufferLayout {
                array_stride: floats * size_of::<f32>() as u64,
                step_mode: wgpu::VertexStepMode::Vertex,
                attributes,
            })
        };
        let lit = |module, label| {
            crate::pipeline(
                gpu,
                &[Some(layout), Some(&radius_layout), Some(shadow.layout())],
                module,
                surface,
                &[vertex(&POSITION, 3), vertex(&NORMAL, 3), vertex(&COORD, 2)],
                crate::pass::Depth::Surface,
                label,
            )
        };
        Self {
            pipeline: lit(&shader, "wood"),
            smooth: lit(&smooth, "smooth wood"),
            smooth_on: false,
            // The sun sees a position and nothing else, so the normals and the
            // coordinates are not bound for it at all.
            shadow: crate::depth_pipeline(
                gpu,
                &[Some(shadow.light_layout())],
                shadow.module(),
                "wood",
                &[vertex(&POSITION, 3)],
                "wood shadow",
            ),
            positions: None,
            normals: None,
            coords: None,
            indices: None,
            radii: None,
            radius_layout,
            radius_group: None,
            index_count: 0,
            runs: Vec::new(),
            caster_index_count: 0,
        }
    }

    /// Uploads the surface straight from the core's arrays. A second, smaller
    /// tree reuses the allocations it fits in.
    pub fn submit(&mut self, gpu: &Gpu, mesh: &SurfaceMesh) {
        self.index_count = mesh.indices.len() as u32;
        self.runs.clone_from(&mesh.run_table);
        self.caster_index_count = self.index_count;
        if self.index_count == 0 {
            return;
        }
        buffer::write(
            gpu,
            &mut self.radii,
            "wood radii",
            wgpu::BufferUsages::STORAGE,
            bytemuck::cast_slice(&radius::radii(mesh)),
        );
        let radii = self.radii.as_ref().expect("submitted radii");
        self.radius_group = Some(gpu.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("wood radii"),
            layout: &self.radius_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                    buffer: radii.buffer(),
                    offset: 0,
                    size: std::num::NonZeroU64::new(radii.region().used()),
                }),
            }],
        }));
        buffer::write(
            gpu,
            &mut self.positions,
            "wood positions",
            wgpu::BufferUsages::VERTEX,
            bytemuck::cast_slice(&mesh.positions),
        );
        buffer::write(
            gpu,
            &mut self.normals,
            "wood normals",
            wgpu::BufferUsages::VERTEX,
            bytemuck::cast_slice(&mesh.normals),
        );
        buffer::write(
            gpu,
            &mut self.coords,
            "wood coordinates",
            wgpu::BufferUsages::VERTEX,
            bytemuck::cast_slice(&buffer::attributes(
                &mesh.coords,
                mesh.positions.len() / 3 * 2,
            )),
        );
        buffer::write(
            gpu,
            &mut self.indices,
            "wood indices",
            wgpu::BufferUsages::INDEX,
            bytemuck::cast_slice(&mesh.indices),
        );
    }

    /// Which of the two lit pipelines the material draws through.
    pub fn set_material(&mut self, material: &MaterialParams) {
        self.smooth_on = smooth_bark(material);
    }

    /// Draws the surface, or nothing when no tree has been submitted.
    pub fn draw(&self, pass: &mut wgpu::RenderPass<'_>) -> FrameStats {
        let (Some(positions), Some(normals), Some(coords), Some(indices)) =
            (&self.positions, &self.normals, &self.coords, &self.indices)
        else {
            return FrameStats::default();
        };
        pass.set_pipeline(if self.smooth_on {
            &self.smooth
        } else {
            &self.pipeline
        });
        pass.set_bind_group(1, self.radius_group.as_ref().expect("submitted radii"), &[]);
        pass.set_vertex_buffer(0, positions.live());
        pass.set_vertex_buffer(1, normals.live());
        pass.set_vertex_buffer(2, coords.live());
        pass.set_index_buffer(indices.live(), wgpu::IndexFormat::Uint32);
        pass.draw_indexed(0..self.index_count, 0, 0..1);
        FrameStats {
            draw_calls: 1,
            triangles: self.index_count / 3,
            instances: 0,
        }
    }

    /// Whole runs at or above the fitted radius threshold form one prefix.
    /// Re-derived on submission or a row change, never from the camera pose.
    pub fn set_casters(&mut self, threshold: f64) {
        let kept = self
            .runs
            .partition_point(|run| run.largest_radius >= threshold);
        self.caster_index_count = kept.checked_sub(1).map_or(0, |last| {
            let run = self.runs[last];
            run.first_index + run.index_count
        });
    }

    /// Writes the radius-selected prefix using the frame's own index buffer.
    pub fn draw_shadow(&self, pass: &mut wgpu::RenderPass<'_>) {
        let (Some(positions), Some(indices)) = (&self.positions, &self.indices) else {
            return;
        };
        pass.set_pipeline(&self.shadow);
        pass.set_vertex_buffer(0, positions.live());
        pass.set_index_buffer(indices.live(), wgpu::IndexFormat::Uint32);
        pass.draw_indexed(0..self.caster_index_count, 0, 0..1);
    }

    /// The live ranges, for a caller that wants to see what was uploaded.
    pub fn regions(&self) -> Option<(Region, Region, Region)> {
        Some((
            self.positions.as_ref()?.region(),
            self.normals.as_ref()?.region(),
            self.indices.as_ref()?.region(),
        ))
    }
}
