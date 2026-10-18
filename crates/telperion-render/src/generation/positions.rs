//! The position programme. Production makes its `emit` entry alone, which
//! the resident build's ring seating runs (`ring_vertices`); the tests
//! add the comparison route's `admit`, `radii` and `finish`, whose
//! admission completes before any foliage consumes candidate geometry.
use super::{io, Generator};
use crate::buffer::Owned;
use crate::Result;

/// The entries production makes, by index: `emit` alone (fn-287 R1).
pub(super) const ENTRIES: &[&str] = &["emit"];

/// The entries the position pass is made with: production's, then under
/// test the comparison route's, which index after them.
pub(super) fn entries() -> Vec<&'static str> {
    let entries = ENTRIES.to_vec();
    #[cfg(test)]
    let entries = [entries, vec!["admit", "radii", "finish"]].concat();
    entries
}

impl Generator {
    /// The vertices of `rings` compact ring descriptors already on the
    /// device (`[u32; 16]` words, as `compact::read` writes them, in a block
    /// of `vertices`) around `angular`'s samples: the positions pass's `emit`
    /// alone, what a seated leaf's contact reads (fn-212 task 6, the fine
    /// block), each ring's outline `lobes` lobes `depth` deep.
    pub(super) fn ring_vertices(
        &self,
        (descriptors, rings, vertices): (&wgpu::Buffer, u32, u32),
        (segments, lobes, depth): (u32, u32, f32),
        angular: &[[f32; 4]],
    ) -> Result<Owned> {
        let limits = self.gpu.device.limits();
        let config = [
            rings,
            0,
            segments,
            limits.max_compute_workgroups_per_dimension,
            0,
            0,
            lobes,
            depth.to_bits(),
        ];
        let storage = wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC;
        let make =
            |label, size, usage, bytes: &[u8]| io::buffer(&self.gpu, label, size, usage, bytes);
        let uniform = make(
            "ring config",
            32,
            wgpu::BufferUsages::UNIFORM,
            bytemuck::cast_slice(&config),
        )?;
        let bytes = std::mem::size_of_val(angular) as u64;
        let angular = make(
            "ring angular",
            bytes,
            storage,
            bytemuck::cast_slice(angular),
        )?;
        let positions = make(
            "ring positions",
            u64::from(vertices.max(1)) * 12,
            storage,
            &[],
        )?;
        // A placeholder each: WebGPU refuses one buffer at two bindings
        // where either is written.
        let none = || make("ring none", 32, storage, &[]);
        let (meta, partial, status) = (none()?, none()?, none()?);
        let pass = &self.positions;
        let all = [
            &uniform,
            descriptors,
            &angular,
            &meta,
            &positions,
            &partial,
            &status,
        ];
        let bind = pass.bind(&self.gpu, &all);
        let mut encoder = self.gpu.encoder(&Default::default());
        if rings > 0 {
            pass.run(&mut encoder, &bind, 0, rings);
        }
        self.gpu.submit([encoder.finish()]);
        Ok(positions)
    }
}
