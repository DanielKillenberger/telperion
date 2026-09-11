//! What a tree costs before it is uploaded, and what it turned into after. The
//! fit check is a pure function so a caller can ask before it owns a device.
use telperion_core::{
    math::Vec3,
    mesh::{Foliage, TreeMesh},
    surface::Bounds,
};

use crate::{
    buffer::Region,
    device::{RenderError, Result},
    select::{self, MAX_LEVELS},
};

/// What a submitted tree turned into on the GPU. Every number is the mesh's
/// own, so a caller can hold the renderer to what the core reported.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Submitted {
    pub wood_vertices: usize,
    pub wood_triangles: usize,
    pub foliage_instances: usize,
    pub bounds: Bounds,
}

/// The ellipsoid the crown's placements fill, which is what a leaf's depth
/// into the crown is measured against. Taken from where the leaves were put
/// rather than from every vertex of every blade: the term reads a placement's
/// own position, and a blade's centimetre of reach either side of it is not
/// worth a pass over the whole crown to learn.
pub fn crown_of(foliage: &Foliage) -> Option<Bounds> {
    let mut bounds: Option<Bounds> = None;
    for placement in &foliage.instances.matrices {
        let at = Vec3::new(
            f64::from(placement[12]),
            f64::from(placement[13]),
            f64::from(placement[14]),
        );
        bounds = Some(match bounds {
            Some(b) => Bounds {
                min: Vec3::new(b.min.x.min(at.x), b.min.y.min(at.y), b.min.z.min(at.z)),
                max: Vec3::new(b.max.x.max(at.x), b.max.y.max(at.y), b.max.z.max(at.z)),
            },
            None => Bounds { min: at, max: at },
        });
    }
    bounds
}

/// Whether this mesh's buffers fit the limits the device granted. Nothing is
/// ever truncated to make it fit: a tree too large for the hardware is a named
/// refusal, not a tree with its crown quietly missing.
pub fn fits(limits: &wgpu::Limits, mesh: &TreeMesh) -> Result<()> {
    let bytes = |count: usize, width: usize| (count * width) as u64;
    let element = &mesh.foliage.element;
    let instances = mesh.foliage.instances.matrices.len();
    // Selection reads the placements and writes the lists through storage
    // bindings, which the device caps on their own beside the buffer size.
    let stored = limits
        .max_storage_buffer_binding_size
        .min(limits.max_buffer_size);
    let selection = select::sizes(
        instances,
        element.levels.len(),
        u64::from(limits.min_storage_buffer_offset_alignment),
    );
    let payloads = [
        (
            "wood positions",
            bytes(mesh.wood.positions.len(), size_of::<f32>()),
            limits.max_buffer_size,
        ),
        (
            "wood normals",
            bytes(mesh.wood.normals.len(), size_of::<f32>()),
            limits.max_buffer_size,
        ),
        (
            "wood coordinates",
            bytes(mesh.wood.coords.len(), size_of::<f32>()),
            limits.max_buffer_size,
        ),
        (
            "wood indices",
            bytes(mesh.wood.indices.len(), size_of::<u32>()),
            limits.max_buffer_size,
        ),
        (
            "foliage level indices",
            bytes(element.level_indices.len(), size_of::<u32>()),
            limits.max_buffer_size,
        ),
        (
            "foliage instances",
            bytes(instances, size_of::<[f32; 16]>()),
            stored,
        ),
        ("foliage level lists", selection.lists, stored),
        ("foliage level counters", selection.counts, stored),
        ("foliage indirect arguments", selection.arguments, stored),
    ];
    for (buffer, payload, limit) in payloads {
        // The allocation is judged, not the payload: buffers are taken with
        // headroom, and the headroom is what the device has to grant.
        let wanted = Region::capacity_for(payload);
        if wanted > limit {
            return Err(RenderError::Oversize {
                buffer,
                bytes: wanted,
                limit,
            });
        }
    }
    // The per-workgroup tally selection reserves its ranges through is sized
    // when the shader is compiled, so a ladder longer than that is refused
    // rather than silently cut short.
    if element.levels.len() > MAX_LEVELS {
        return Err(RenderError::TooManyLevels {
            levels: element.levels.len(),
            limit: MAX_LEVELS,
        });
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use telperion_core::foliage::{Element, Instances};

    fn placement(at: [f32; 3]) -> [f32; 16] {
        let mut m = [0.0; 16];
        (m[0], m[5], m[10], m[15]) = (1.0, 1.0, 1.0, 1.0);
        [m[12], m[13], m[14]] = at;
        m
    }

    #[test]
    fn the_crown_is_the_box_the_placements_fill_and_nothing_when_there_are_none() {
        let foliage = Foliage {
            element: Element::default(),
            instances: Instances {
                matrices: vec![
                    placement([1.0, 2.0, -3.0]),
                    placement([-1.0, 6.0, 3.0]),
                    placement([0.0, 4.0, 0.0]),
                ],
            },
        };
        let crown = crown_of(&foliage).expect("three leaves stand somewhere");
        assert_eq!((crown.min.x, crown.min.y, crown.min.z), (-1.0, 2.0, -3.0));
        assert_eq!((crown.max.x, crown.max.y, crown.max.z), (1.0, 6.0, 3.0));
        // A crown of no leaves is no interior at all, and the leaf view says so
        // by having nothing to be deep inside.
        assert!(crown_of(&Foliage {
            element: Element::default(),
            instances: Instances::default(),
        })
        .is_none());
    }
}
