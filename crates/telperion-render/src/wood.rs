//! The plaited wood surface: the core's own position, normal and index arrays
//! uploaded as they lie in memory, drawn as one indexed mesh.
use telperion_core::surface::SurfaceMesh;

use crate::device::Gpu;

/// A quarter again what the tree needs. The headroom is the append seam: a
/// later streamed refinement writes above `used` without a re-layout.
const HEADROOM: f64 = 1.25;
/// Buffer sizes stay a multiple of this, which every copy and binding wants.
const ALIGNMENT: u64 = wgpu::COPY_BUFFER_ALIGNMENT;
/// The core keeps positions and normals in separate arrays, so the pipeline
/// takes two vertex buffers and neither array is interleaved on the way up.
const POSITION: [wgpu::VertexAttribute; 1] = wgpu::vertex_attr_array![0 => Float32x3];
const NORMAL: [wgpu::VertexAttribute; 1] = wgpu::vertex_attr_array![1 => Float32x3];

/// An allocation with headroom and the live range inside it. Reuse is decided
/// here and nowhere else, so the decision is testable without a device.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Region {
    capacity: u64,
    used: u64,
}

impl Region {
    /// What to allocate so `bytes` fit with room to grow.
    pub fn capacity_for(bytes: u64) -> u64 {
        let wanted = (bytes as f64 * HEADROOM).ceil() as u64;
        wanted.max(bytes).div_ceil(ALIGNMENT) * ALIGNMENT
    }

    pub fn new(bytes: u64) -> Self {
        Self {
            capacity: Self::capacity_for(bytes),
            used: bytes,
        }
    }

    /// Takes `bytes` as the live range. Reports whether the existing allocation
    /// held them; a `false` means the caller must allocate `capacity` again.
    pub fn fit(&mut self, bytes: u64) -> bool {
        let reused = bytes <= self.capacity;
        if !reused {
            self.capacity = Self::capacity_for(bytes);
        }
        self.used = bytes;
        reused
    }

    pub fn capacity(&self) -> u64 {
        self.capacity
    }

    pub fn used(&self) -> u64 {
        self.used
    }
}

struct Held {
    buffer: wgpu::Buffer,
    region: Region,
    usage: wgpu::BufferUsages,
    label: &'static str,
}

impl Held {
    fn new(gpu: &Gpu, label: &'static str, usage: wgpu::BufferUsages, bytes: &[u8]) -> Self {
        let region = Region::new(bytes.len() as u64);
        Self {
            buffer: allocate(gpu, label, usage, region.capacity(), bytes),
            region,
            usage,
            label,
        }
    }

    /// Writes the new contents, reusing the allocation whenever they fit.
    fn write(&mut self, gpu: &Gpu, bytes: &[u8]) {
        if self.region.fit(bytes.len() as u64) {
            gpu.queue.write_buffer(&self.buffer, 0, bytes);
        } else {
            self.buffer = allocate(gpu, self.label, self.usage, self.region.capacity(), bytes);
        }
    }

    fn live(&self) -> wgpu::BufferSlice<'_> {
        self.buffer.slice(0..self.region.used())
    }
}

fn allocate(
    gpu: &Gpu,
    label: &'static str,
    usage: wgpu::BufferUsages,
    capacity: u64,
    bytes: &[u8],
) -> wgpu::Buffer {
    let buffer = gpu.device.create_buffer(&wgpu::BufferDescriptor {
        label: Some(label),
        size: capacity,
        usage: usage | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });
    gpu.queue.write_buffer(&buffer, 0, bytes);
    buffer
}

/// The wood pipeline and the buffers one tree's surface lives in.
pub struct Wood {
    pipeline: wgpu::RenderPipeline,
    positions: Option<Held>,
    normals: Option<Held>,
    indices: Option<Held>,
    index_count: u32,
}

impl Wood {
    pub fn new(
        gpu: &Gpu,
        layout: &wgpu::BindGroupLayout,
        colour_format: wgpu::TextureFormat,
    ) -> Self {
        let shader = gpu
            .device
            .create_shader_module(wgpu::include_wgsl!("shaders/wood.wgsl"));
        let vertex = |attributes| {
            Some(wgpu::VertexBufferLayout {
                array_stride: 3 * size_of::<f32>() as u64,
                step_mode: wgpu::VertexStepMode::Vertex,
                attributes,
            })
        };
        Self {
            pipeline: crate::pipeline(
                gpu,
                layout,
                &shader,
                colour_format,
                &[vertex(&POSITION), vertex(&NORMAL)],
                "wood",
            ),
            positions: None,
            normals: None,
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
        write(
            gpu,
            &mut self.positions,
            "wood positions",
            wgpu::BufferUsages::VERTEX,
            bytemuck::cast_slice(&mesh.positions),
        );
        write(
            gpu,
            &mut self.normals,
            "wood normals",
            wgpu::BufferUsages::VERTEX,
            bytemuck::cast_slice(&mesh.normals),
        );
        write(
            gpu,
            &mut self.indices,
            "wood indices",
            wgpu::BufferUsages::INDEX,
            bytemuck::cast_slice(&mesh.indices),
        );
    }

    /// Triangles drawn, or none when no tree has been submitted.
    pub fn draw(&self, pass: &mut wgpu::RenderPass<'_>) -> u32 {
        let (Some(positions), Some(normals), Some(indices)) =
            (&self.positions, &self.normals, &self.indices)
        else {
            return 0;
        };
        pass.set_pipeline(&self.pipeline);
        pass.set_vertex_buffer(0, positions.live());
        pass.set_vertex_buffer(1, normals.live());
        pass.set_index_buffer(indices.live(), wgpu::IndexFormat::Uint32);
        pass.draw_indexed(0..self.index_count, 0, 0..1);
        self.index_count / 3
    }

    /// The live ranges, for a caller that wants to see what was uploaded.
    pub fn regions(&self) -> Option<(Region, Region, Region)> {
        Some((
            self.positions.as_ref()?.region,
            self.normals.as_ref()?.region,
            self.indices.as_ref()?.region,
        ))
    }
}

fn write(
    gpu: &Gpu,
    slot: &mut Option<Held>,
    label: &'static str,
    usage: wgpu::BufferUsages,
    bytes: &[u8],
) {
    match slot {
        Some(held) => held.write(gpu, bytes),
        None => *slot = Some(Held::new(gpu, label, usage, bytes)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_allocation_carries_headroom_above_what_it_holds() {
        let region = Region::new(1_000);
        assert!(region.capacity() >= 1_250, "{}", region.capacity());
        assert_eq!(region.used(), 1_000);
        assert_eq!(region.capacity() % ALIGNMENT, 0);
    }

    #[test]
    fn a_second_smaller_submit_reuses_the_allocation() {
        let mut region = Region::new(1_000);
        let capacity = region.capacity();
        assert!(region.fit(400), "a smaller tree forced a re-layout");
        assert_eq!(region.capacity(), capacity, "the allocation moved");
        assert_eq!(region.used(), 400, "the live range did not follow the tree");
    }

    #[test]
    fn a_submit_that_outgrows_the_headroom_allocates_again() {
        let mut region = Region::new(1_000);
        assert!(region.fit(1_200), "the headroom went unused");
        assert!(
            !region.fit(4_000),
            "a tree beyond the headroom was let through"
        );
        assert!(region.capacity() >= 5_000);
        assert_eq!(region.used(), 4_000);
    }

    #[test]
    fn an_empty_tree_asks_for_nothing() {
        let region = Region::new(0);
        assert_eq!(region.capacity(), 0);
        assert_eq!(region.used(), 0);
    }
}
