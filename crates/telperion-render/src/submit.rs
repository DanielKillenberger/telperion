//! What a tree costs before it is uploaded, and what it turned into after. The
//! fit check is a pure function so a caller can ask before it owns a device.
use telperion_core::{mesh::TreeMesh, surface::Bounds};

use crate::{
    buffer::Region,
    device::{RenderError, Result},
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

/// Whether this mesh's buffers fit the limit the device granted. Nothing is
/// ever truncated to make it fit: a tree too large for the hardware is a named
/// refusal, not a tree with its crown quietly missing.
pub fn fits(limits: &wgpu::Limits, mesh: &TreeMesh) -> Result<()> {
    let bytes = |count: usize, width: usize| (count * width) as u64;
    let payloads = [
        (
            "wood positions",
            bytes(mesh.wood.positions.len(), size_of::<f32>()),
        ),
        (
            "wood normals",
            bytes(mesh.wood.normals.len(), size_of::<f32>()),
        ),
        (
            "wood indices",
            bytes(mesh.wood.indices.len(), size_of::<u32>()),
        ),
        (
            "foliage instances",
            bytes(
                mesh.foliage.instances.matrices.len(),
                size_of::<[f32; 16]>(),
            ),
        ),
    ];
    for (buffer, payload) in payloads {
        // The allocation is judged, not the payload: buffers are taken with
        // headroom, and the headroom is what the device has to grant.
        let wanted = Region::capacity_for(payload);
        if wanted > limits.max_buffer_size {
            return Err(RenderError::Oversize {
                buffer,
                bytes: wanted,
                limit: limits.max_buffer_size,
            });
        }
    }
    Ok(())
}
