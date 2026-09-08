//! The crown: one leaf element drawn once per instance matrix. The matrices go
//! up as the core wrote them; only the element's f64 positions are narrowed,
//! once per tree, because a vertex buffer holds f32.
use telperion_core::{foliage::Element, math::Vec3, mesh, surface::Bounds};

use crate::{
    buffer::{self, Held, Region},
    device::Gpu,
    view::View,
    FrameStats,
};

/// Positions and normals step per vertex; the placement steps per instance, one
/// mat4 as the four columns the core already packed.
const POSITION: [wgpu::VertexAttribute; 1] = wgpu::vertex_attr_array![0 => Float32x3];
const NORMAL: [wgpu::VertexAttribute; 1] = wgpu::vertex_attr_array![1 => Float32x3];
const PLACEMENT: [wgpu::VertexAttribute; 4] =
    wgpu::vertex_attr_array![2 => Float32x4, 3 => Float32x4, 4 => Float32x4, 5 => Float32x4];
/// Column-major, the same layout the core's instance matrices use.
const IDENTITY: [f32; 16] = [
    1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
];

/// The element's own vertex normals, area-weighted over the triangles that
/// share each vertex. The core ships positions and indices only, and a leaf
/// with no normal has no cup and no curl to see.
fn normals(element: &Element) -> Vec<f32> {
    let mut summed = vec![Vec3::ZERO; element.positions.len()];
    for triangle in element.indices.as_chunks::<3>().0 {
        let [a, b, c] = triangle.map(|i| element.positions[i as usize]);
        // The cross product's length is twice the triangle's area, so summing
        // it weights each face by how much of the vertex it accounts for.
        let face = (b - a).cross(c - a);
        for index in triangle {
            summed[*index as usize] += face;
        }
    }
    summed
        .iter()
        .flat_map(|n| {
            let n = if n.length_squared() > 0.0 {
                n.normalized()
            } else {
                Vec3::Y
            };
            [n.x as f32, n.y as f32, n.z as f32]
        })
        .collect()
}

fn positions(element: &Element) -> Vec<f32> {
    element
        .positions
        .iter()
        .flat_map(|p| [p.x as f32, p.y as f32, p.z as f32])
        .collect()
}

/// The element as it stands at the origin, which is what the leaf view frames.
fn element_bounds(element: &Element) -> Option<Bounds> {
    element
        .positions
        .iter()
        .fold(None, |bounds, &p| match bounds {
            None => Some(Bounds { min: p, max: p }),
            Some(b) => Some(Bounds {
                min: Vec3::new(b.min.x.min(p.x), b.min.y.min(p.y), b.min.z.min(p.z)),
                max: Vec3::new(b.max.x.max(p.x), b.max.y.max(p.y), b.max.z.max(p.z)),
            }),
        })
}

/// The foliage pipeline and the buffers one tree's crown lives in.
pub struct Foliage {
    pipeline: wgpu::RenderPipeline,
    /// The leaf view's placement: one matrix, never rewritten.
    identity: wgpu::Buffer,
    positions: Option<Held>,
    normals: Option<Held>,
    indices: Option<Held>,
    instances: Option<Held>,
    index_count: u32,
    instance_count: u32,
    bounds: Option<Bounds>,
}

impl Foliage {
    pub fn new(
        gpu: &Gpu,
        layout: &wgpu::BindGroupLayout,
        colour_format: wgpu::TextureFormat,
    ) -> Self {
        let shader = gpu
            .device
            .create_shader_module(wgpu::include_wgsl!("shaders/foliage.wgsl"));
        let vertex = |attributes| {
            Some(wgpu::VertexBufferLayout {
                array_stride: 3 * size_of::<f32>() as u64,
                step_mode: wgpu::VertexStepMode::Vertex,
                attributes,
            })
        };
        let identity = gpu.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("foliage identity placement"),
            size: size_of::<[f32; 16]>() as u64,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        gpu.queue
            .write_buffer(&identity, 0, bytemuck::cast_slice(&IDENTITY));
        Self {
            pipeline: crate::pipeline(
                gpu,
                layout,
                &shader,
                colour_format,
                &[
                    vertex(&POSITION),
                    vertex(&NORMAL),
                    Some(wgpu::VertexBufferLayout {
                        array_stride: size_of::<[f32; 16]>() as u64,
                        step_mode: wgpu::VertexStepMode::Instance,
                        attributes: &PLACEMENT,
                    }),
                ],
                "foliage",
            ),
            identity,
            positions: None,
            normals: None,
            indices: None,
            instances: None,
            index_count: 0,
            instance_count: 0,
            bounds: None,
        }
    }

    /// Uploads the element once and the placements as the core packed them.
    pub fn submit(&mut self, gpu: &Gpu, foliage: &mesh::Foliage) {
        let element = &foliage.element;
        self.index_count = element.indices.len() as u32;
        self.instance_count = foliage.instances.matrices.len() as u32;
        self.bounds = element_bounds(element);
        if self.index_count == 0 {
            return;
        }
        buffer::write(
            gpu,
            &mut self.positions,
            "foliage positions",
            wgpu::BufferUsages::VERTEX,
            bytemuck::cast_slice(&positions(element)),
        );
        buffer::write(
            gpu,
            &mut self.normals,
            "foliage normals",
            wgpu::BufferUsages::VERTEX,
            bytemuck::cast_slice(&normals(element)),
        );
        buffer::write(
            gpu,
            &mut self.indices,
            "foliage indices",
            wgpu::BufferUsages::INDEX,
            bytemuck::cast_slice(&element.indices),
        );
        buffer::write(
            gpu,
            &mut self.instances,
            "foliage instances",
            wgpu::BufferUsages::VERTEX,
            bytemuck::cast_slice(&foliage.instances.matrices),
        );
    }

    /// Draws the crown this view asks for: every placement, one at the origin,
    /// or none at all.
    pub fn draw(&self, pass: &mut wgpu::RenderPass<'_>, view: View) -> FrameStats {
        let (Some(positions), Some(normals), Some(indices)) =
            (&self.positions, &self.normals, &self.indices)
        else {
            return FrameStats::default();
        };
        let (placements, instances) = match view {
            View::Bare => return FrameStats::default(),
            View::Leaf => (self.identity.slice(..), 1),
            View::Whole => match &self.instances {
                Some(held) if self.instance_count > 0 => (held.live(), self.instance_count),
                _ => return FrameStats::default(),
            },
        };
        pass.set_pipeline(&self.pipeline);
        pass.set_vertex_buffer(0, positions.live());
        pass.set_vertex_buffer(1, normals.live());
        pass.set_vertex_buffer(2, placements);
        pass.set_index_buffer(indices.live(), wgpu::IndexFormat::Uint32);
        pass.draw_indexed(0..self.index_count, 0, 0..instances);
        FrameStats {
            draw_calls: 1,
            triangles: (self.index_count / 3).saturating_mul(instances),
            instances,
        }
    }

    /// The element's bounds at the origin, which frames the leaf view.
    pub fn bounds(&self) -> Option<Bounds> {
        self.bounds
    }

    /// The live instance range, for a caller that wants to see what went up.
    pub fn region(&self) -> Option<Region> {
        Some(self.instances.as_ref()?.region())
    }
}

#[cfg(test)]
mod tests {
    use telperion_core::foliage::{build_element, ElementParams};

    use super::*;

    #[test]
    fn a_leaf_carries_a_normal_off_its_own_face() {
        let element = build_element(ElementParams::default()).expect("the core built a leaf");
        let normals = normals(&element);
        assert_eq!(normals.len(), element.positions.len() * 3);
        for normal in normals.as_chunks::<3>().0 {
            let length = normal.iter().map(|v| v * v).sum::<f32>().sqrt();
            assert!(
                (length - 1.0).abs() < 1e-4,
                "{normal:?} is not a unit normal"
            );
        }
        // A blade faces +Z, so every normal leans that way rather than along
        // the leaf's own axis; a normal that did not would flat-shade the cup.
        let facing = normals
            .as_chunks::<3>()
            .0
            .iter()
            .filter(|n| n[2] > 0.5)
            .count();
        assert!(facing > 0, "no normal faces the blade's own front");
    }

    #[test]
    fn an_element_at_the_origin_bounds_itself() {
        let element = build_element(ElementParams::default()).expect("the core built a leaf");
        let bounds = element_bounds(&element).expect("a leaf has vertices");
        assert!(bounds.max.y > bounds.min.y, "the leaf has no length");
        assert!(bounds.max.x > bounds.min.x, "the leaf has no width");
        assert!(element_bounds(&Element::default()).is_none());
    }
}
