//! The crown: one leaf element, drawn once per instance at the level selection
//! chose for it. The placements go up as the core wrote them, as a storage
//! buffer the vertex shader reads through the level's own index list; only the
//! element's f64 positions are narrowed, once per tree, because a vertex
//! buffer holds f32.
use telperion_core::{foliage::Element, math::Vec3, mesh, surface::Bounds};

use crate::{
    buffer::{self, Held, Region},
    device::Gpu,
    select::{Level, Select},
    view::View,
    Camera, FrameStats,
};

/// Positions and normals step per vertex. Nothing steps per instance any more:
/// the placement is looked up, not fed in.
const POSITION: [wgpu::VertexAttribute; 1] = wgpu::vertex_attr_array![0 => Float32x3];
const NORMAL: [wgpu::VertexAttribute; 1] = wgpu::vertex_attr_array![1 => Float32x3];

/// The element's own vertex normals, area-weighted over the triangles that
/// share each vertex. The core ships positions and indices only, and a leaf
/// with no normal has no cup and no curl to see. The whole element is summed,
/// whichever level ends up drawn: a coarse level's vertices are the fine
/// one's, so the leaf keeps the shading it had.
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

/// The foliage pipeline, the buffers one tree's crown lives in, and the pass
/// that decides which level each of its leaves is drawn at.
pub struct Foliage {
    pipeline: wgpu::RenderPipeline,
    select: Select,
    positions: Option<Held>,
    normals: Option<Held>,
    /// Every level's triangles in one index buffer, as the core packed them.
    indices: Option<Held>,
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
        let select = Select::new(gpu);
        Self {
            pipeline: crate::pipeline(
                gpu,
                &[Some(layout), Some(select.draw_layout())],
                &shader,
                colour_format,
                &[vertex(&POSITION), vertex(&NORMAL)],
                "foliage",
            ),
            select,
            positions: None,
            normals: None,
            indices: None,
            bounds: None,
        }
    }

    /// Uploads the element once with every level's triangles, and the
    /// placements selection reads. The level asked for changes what each
    /// instance draws and nothing about how many there are.
    pub fn submit(&mut self, gpu: &Gpu, foliage: &mesh::Foliage, level: Level) {
        let element = &foliage.element;
        self.bounds = element_bounds(element);
        self.select.submit(gpu, foliage, level);
        if element.level_indices.is_empty() {
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
            "foliage level indices",
            wgpu::BufferUsages::INDEX,
            bytemuck::cast_slice(&element.level_indices),
        );
    }

    /// Chooses a level for every leaf of the crown, before the frame draws
    /// any of it. Only the whole view selects: the bare view has no crown to
    /// select from, and the leaf view is one instance at one known level.
    pub fn dispatch(
        &self,
        gpu: &Gpu,
        encoder: &mut wgpu::CommandEncoder,
        camera: &Camera,
        viewport: (u32, u32),
        view: View,
    ) {
        if view == View::Whole {
            self.select.dispatch(gpu, encoder, camera, viewport);
        }
    }

    /// Draws the crown this view asks for: every placement at the level
    /// selection gave it, one leaf at the origin whole, or none at all.
    ///
    /// No count comes back from the device, so the statistics are what was
    /// issued: one indirect draw per level, and the triangles they can reach
    /// between them, which is the whole crown at its finest.
    pub fn draw(&self, pass: &mut wgpu::RenderPass<'_>, view: View) -> FrameStats {
        if view == View::Bare {
            return FrameStats::default();
        }
        let (Some(positions), Some(normals), Some(indices)) =
            (&self.positions, &self.normals, &self.indices)
        else {
            return FrameStats::default();
        };
        let Some(finest) = self.select.levels().last().cloned() else {
            return FrameStats::default();
        };
        pass.set_pipeline(&self.pipeline);
        pass.set_vertex_buffer(0, positions.live());
        pass.set_vertex_buffer(1, normals.live());
        pass.set_index_buffer(indices.live(), wgpu::IndexFormat::Uint32);
        if view == View::Leaf {
            self.select.bind_leaf(pass);
            pass.draw_indexed(finest.clone(), 0, 0..1);
            return FrameStats {
                draw_calls: 1,
                triangles: finest.len() as u32 / 3,
                instances: 1,
            };
        }
        let mut draws = 0;
        for level in 0..self.select.levels().len() {
            let Some((arguments, offset)) = self.select.arguments(level) else {
                break;
            };
            if !self.select.bind_level(pass, level) {
                break;
            }
            pass.draw_indexed_indirect(arguments, offset);
            draws += 1;
        }
        let instances = self.select.instances();
        FrameStats {
            draw_calls: draws,
            triangles: (finest.len() as u32 / 3).saturating_mul(instances),
            instances,
        }
    }

    /// The element's bounds at the origin, which frames the leaf view.
    pub fn bounds(&self) -> Option<Bounds> {
        self.bounds
    }

    /// The live placement range, for a caller that wants to see what went up.
    pub fn region(&self) -> Option<Region> {
        self.select.region()
    }

    /// What the last frame's selection counted per level, the unseen bucket
    /// last. A stall on the device: a check and a record, not a frame's work.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn counted(&self, gpu: &Gpu) -> Option<Vec<u32>> {
        self.select.counted(gpu)
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
        // A leaf faces +Z, so every normal leans that way rather than along
        // the leaf's own axis; a normal that did not would flat-shade the cup.
        let facing = normals
            .as_chunks::<3>()
            .0
            .iter()
            .filter(|n| n[2] > 0.5)
            .count();
        assert!(facing > 0, "no normal faces the leaf's own front");
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
