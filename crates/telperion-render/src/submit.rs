//! What a tree costs before it is uploaded, and what it turned into after. The
//! fit check is a pure function so a caller can ask before it owns a device.
use telperion_core::{mesh::TreeMesh, surface::Bounds};

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
