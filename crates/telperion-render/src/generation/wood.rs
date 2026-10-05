//! The wood's ring positions on the device, which the leaves' stations seat
//! on. The wood itself is drawn from its curve (fn-208, host decision 20).
use super::{io, Clock, Generator, Metrics};
use crate::Result;
use telperion_core::surface::{prepared::PreparedSurface, SurfaceRun};

pub(super) struct UploadedWood {
    pub positions: wgpu::Buffer,
    /// The positions kernel's run and ring metadata, where it made them.
    pub metadata: Option<wgpu::Buffer>,
    pub(super) runs: Vec<SurfaceRun>,
}
impl UploadedWood {
    pub fn gpu_metadata_bytes(&self) -> u64 {
        self.metadata.as_ref().map_or(0, wgpu::Buffer::size)
    }

    pub fn metadata_bytes(&self) -> u64 {
        (self.runs.capacity() * size_of::<SurfaceRun>()) as u64
    }
}
impl Generator {
    /// The CPU's ring positions on the device, for the stations to read.
    pub(super) fn upload_wood(
        &self,
        p: PreparedSurface,
        metrics: &mut Metrics,
    ) -> Result<Option<UploadedWood>> {
        metrics.wood_prepared_cpu_bytes = prepared_cpu_bytes(&p);
        let limits = self.gpu.device.limits();
        let bytes = p.positions.len() as u64 * 4;
        if !compute_fits(&limits, &[bytes], p.runs.len() as u64) {
            metrics.wood_fallback = Some("compute limits");
            return Ok(None);
        }
        let start = Clock::now();
        let positions = io::buffer(
            &self.gpu,
            "wood ring positions",
            bytes,
            wgpu::BufferUsages::STORAGE
                | wgpu::BufferUsages::COPY_SRC
                | wgpu::BufferUsages::COPY_DST,
            bytemuck::cast_slice(&p.positions),
        )?;
        metrics.wood_upload_dispatch_ms = start.elapsed_ms();
        Ok(Some(UploadedWood {
            positions,
            metadata: None,
            runs: p.run_table,
        }))
    }
}

pub(super) fn compute_fits(limits: &wgpu::Limits, sizes: &[u64], runs: u64) -> bool {
    let stored = limits
        .max_buffer_size
        .min(limits.max_storage_buffer_binding_size);
    sizes.iter().all(|&n| n.max(16) <= stored)
        && runs <= u64::from(limits.max_compute_workgroups_per_dimension).pow(2)
}

pub(super) fn cpu_bytes(wood: &telperion_core::surface::SurfaceMesh) -> u64 {
    ((wood.positions.capacity()
        + wood.normals.capacity()
        + wood.coords.capacity()
        + wood.indices.capacity())
        * 4
        + wood.run_table.capacity() * size_of::<SurfaceRun>()) as u64
}

pub(super) fn prepared_cpu_bytes(p: &PreparedSurface) -> u64 {
    ((p.positions.capacity() + p.angles.capacity()) * 4
        + p.rings.capacity() * 8
        + p.runs.capacity() * size_of::<telperion_core::surface::prepared::Run>()
        + p.run_table.capacity() * size_of::<SurfaceRun>()) as u64
}
