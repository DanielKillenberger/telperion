//! The plaited wood surface: the core's own position, normal, coordinate and
//! index arrays uploaded as they lie in memory, drawn as one indexed mesh.
use telperion_core::surface::SurfaceMesh;

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

/// The wood pipeline, the depth-only one the sun draws it through, and the
/// buffers one tree's surface lives in.
pub struct Wood {
    pipeline: wgpu::RenderPipeline,
    shadow: wgpu::RenderPipeline,
    positions: Option<Held>,
    normals: Option<Held>,
    coords: Option<Held>,
    indices: Option<Held>,
    index_count: u32,
}

impl Wood {
    pub fn new(
        gpu: &Gpu,
        layout: &wgpu::BindGroupLayout,
        shadow: &crate::shadow::Shadow,
        surface: crate::pass::Surface,
    ) -> Self {
        let shader = crate::pass::lit_shader(gpu, "wood", include_str!("shaders/wood.wgsl"));
        let vertex = |attributes, floats: u64| {
            Some(wgpu::VertexBufferLayout {
                array_stride: floats * size_of::<f32>() as u64,
                step_mode: wgpu::VertexStepMode::Vertex,
                attributes,
            })
        };
        Self {
            pipeline: crate::pipeline(
                gpu,
                &[Some(layout), None, Some(shadow.layout())],
                &shader,
                surface,
                &[vertex(&POSITION, 3), vertex(&NORMAL, 3), vertex(&COORD, 2)],
                crate::pass::Depth::Surface,
                "wood",
            ),
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
            index_count: 0,
        }
    }

    /// Uploads the surface straight from the core's arrays. A second, smaller
    /// tree reuses the allocations it fits in.
    pub fn submit(&mut self, gpu: &Gpu, mesh: &SurfaceMesh) {
        self.index_count = mesh.indices.len() as u32;
        if self.index_count == 0 {
            return;
        }
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

    /// Draws the surface, or nothing when no tree has been submitted.
    pub fn draw(&self, pass: &mut wgpu::RenderPass<'_>) -> FrameStats {
        let (Some(positions), Some(normals), Some(coords), Some(indices)) =
            (&self.positions, &self.normals, &self.coords, &self.indices)
        else {
            return FrameStats::default();
        };
        pass.set_pipeline(&self.pipeline);
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

    /// Writes the surface into the sun's depth map. The same triangles the
    /// frame draws, with nothing but their positions bound: what casts a
    /// shadow is where the wood is, not what it looks like.
    pub fn draw_shadow(&self, pass: &mut wgpu::RenderPass<'_>) {
        let (Some(positions), Some(indices)) = (&self.positions, &self.indices) else {
            return;
        };
        pass.set_pipeline(&self.shadow);
        pass.set_vertex_buffer(0, positions.live());
        pass.set_index_buffer(indices.live(), wgpu::IndexFormat::Uint32);
        pass.draw_indexed(0..self.index_count, 0, 0..1);
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
