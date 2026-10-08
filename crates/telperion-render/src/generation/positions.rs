//! Admission completes before any foliage consumes candidate geometry.
use super::{
    io,
    wood::{self, UploadedWood},
    Clock, Generator, Metrics,
};
use crate::Result;
use telperion_core::surface::compact::CompactSurface;

pub(super) fn cpu_bytes(p: &CompactSurface) -> u64 {
    (p.rings.capacity() * 64
        + p.angular.capacity() * 16
        + p.runs.capacity() * size_of::<telperion_core::surface::prepared::Run>()
        + p.run_table.capacity() * size_of::<telperion_core::surface::SurfaceRun>()) as u64
}

struct PendingPositions {
    surface: CompactSurface,
    positions: wgpu::Buffer,
    metadata: wgpu::Buffer,
    _scratch: [wgpu::Buffer; 5],
    read: io::PendingRead,
}

impl Generator {
    pub(super) async fn emit_positions(
        &self,
        p: CompactSurface,
        metrics: &mut Metrics,
    ) -> Result<Option<UploadedWood>> {
        match self.begin_positions(p, metrics)? {
            Some(pending) => self.complete_positions(pending, metrics).await,
            None => Ok(None),
        }
    }

    fn begin_positions(
        &self,
        p: CompactSurface,
        metrics: &mut Metrics,
    ) -> Result<Option<PendingPositions>> {
        if !p.qualified() {
            metrics.position_fallback = Some("precision domain");
            return Ok(None);
        }
        let limits = self.gpu.device.limits();
        let words = p.runs.len() * 4 + p.rings.len() * 2 + p.angular.len();
        // What the pass binds: the positions and the metadata. The ring
        // mesh's normals, coordinates and indices are never built (nothing
        // draws them, fn-208), so their sizes no longer refuse a tree: the
        // spruce at 80's 2.55 GB of indices did (fn-212 R2).
        let sizes = [p.vertices as u64 * 12, words as u64 * 4];
        let scratch_sizes = [
            p.rings.len() as u64 * 64,
            p.angular.len() as u64 * 16,
            p.runs.len() as u64 * 32,
        ];
        if !wood::compute_fits(&limits, &sizes, p.runs.len() as u64)
            || !wood::compute_fits(&limits, &scratch_sizes, p.rings.len() as u64)
        {
            metrics.position_fallback = Some("compute limits");
            return Ok(None);
        }
        let start = Clock::now();
        let mut metadata = Vec::new();
        metadata
            .try_reserve_exact(words)
            .map_err(|_| telperion_core::Error::ResourceLimit("position metadata"))?;
        for r in &p.runs {
            metadata.extend([r.base, r.first_index, r.ring_start, r.rings]);
        }
        let ring_offset = metadata.len() as u32;
        metadata.resize(metadata.len() + p.rings.len() * 2, 0);
        let angle_offset = metadata.len() as u32;
        metadata.extend(p.angular.iter().map(|a| a[0].to_bits()));
        metrics.position_cpu_bytes = cpu_bytes(&p) + metadata.capacity() as u64 * 4;
        let config = [
            p.rings.len() as u32,
            p.runs.len() as u32,
            p.segments,
            limits.max_compute_workgroups_per_dimension,
            ring_offset,
            angle_offset,
            0,
            0,
        ];
        let storage = wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC;
        let make =
            |label, size, usage, bytes: &[u8]| io::buffer(&self.gpu, label, size, usage, bytes);
        let uniform = make(
            "position config",
            32,
            wgpu::BufferUsages::UNIFORM,
            bytemuck::cast_slice(&config),
        )?;
        let descriptors = make(
            "position rings",
            scratch_sizes[0],
            storage,
            bytemuck::cast_slice(&p.rings),
        )?;
        let angular = make(
            "position angular",
            scratch_sizes[1],
            storage,
            bytemuck::cast_slice(&p.angular),
        )?;
        let meta = make(
            "position wood metadata",
            sizes[1],
            storage,
            bytemuck::cast_slice(&metadata),
        )?;
        let positions = make(
            "resident wood positions",
            sizes[0],
            storage | wgpu::BufferUsages::VERTEX,
            &[],
        )?;
        let partial = make("position admission scratch", scratch_sizes[2], storage, &[])?;
        let status = make("position admission", 32, storage, &[])?;
        let pass = self
            .positions
            .as_ref()
            .expect("resident positions pipeline");
        let bind = pass.bind(
            &self.gpu,
            &[
                &uniform,
                &descriptors,
                &angular,
                &meta,
                &positions,
                &partial,
                &status,
            ],
        );
        let mut encoder = self.gpu.device.create_command_encoder(&Default::default());
        if !p.rings.is_empty() {
            pass.run(&mut encoder, &bind, 0, config[0]);
            pass.run(&mut encoder, &bind, 1, config[1]);
        }
        pass.run(&mut encoder, &bind, 3, 1);
        if !p.rings.is_empty() {
            pass.run(&mut encoder, &bind, 2, config[0]);
        }
        self.gpu.queue.submit([encoder.finish()]);
        metrics.position_gpu_peak_bytes = [
            &uniform,
            &descriptors,
            &angular,
            &meta,
            &positions,
            &partial,
            &status,
        ]
        .iter()
        .map(|b| b.size())
        .sum::<u64>()
            + 32;
        drop(metadata);
        let read = io::begin_read(&self.gpu, &status, 32)?;
        metrics.position_upload_ms = start.elapsed_ms();
        Ok(Some(PendingPositions {
            surface: p,
            positions,
            metadata: meta,
            _scratch: [uniform, descriptors, angular, partial, status],
            read,
        }))
    }

    async fn complete_positions(
        &self,
        pending: PendingPositions,
        metrics: &mut Metrics,
    ) -> Result<Option<UploadedWood>> {
        let PendingPositions {
            surface: p,
            positions,
            metadata: meta,
            _scratch,
            read,
        } = pending;
        let wait = Clock::now();
        let bytes = read.complete(&self.gpu).await?;
        metrics.position_wait_ms = wait.elapsed_ms();
        let status: Vec<u32> = bytes
            .chunks_exact(4)
            .map(|b| u32::from_le_bytes(b.try_into().unwrap()))
            .collect();
        if status[3] != 0 {
            metrics.position_fallback = Some("geometry or normal admission");
            return Ok(None);
        }
        metrics.gpu_positions = true;
        metrics.position_retained_metadata_bytes = meta.size();
        Ok(Some(UploadedWood {
            positions,
            metadata: Some(meta),
            runs: p.run_table,
        }))
    }
}

impl Generator {
    /// The vertices of `rings` compact ring descriptors already on the
    /// device (`[u32; 16]` words, as `compact::read` writes them, in a block
    /// of `vertices`) around `angular`'s samples: the positions pass's `emit`
    /// alone, what a seated leaf's contact reads (fn-212 task 6, the fine
    /// block).
    pub(super) fn ring_vertices(
        &self,
        (descriptors, rings, vertices): (&wgpu::Buffer, u32, u32),
        segments: u32,
        angular: &[[f32; 4]],
    ) -> Result<wgpu::Buffer> {
        let limits = self.gpu.device.limits();
        let config = [
            rings,
            0,
            segments,
            limits.max_compute_workgroups_per_dimension,
            0,
            0,
            0,
            0,
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
        let pass = self
            .positions
            .as_ref()
            .expect("resident positions pipeline");
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
        let mut encoder = self.gpu.device.create_command_encoder(&Default::default());
        if rings > 0 {
            pass.run(&mut encoder, &bind, 0, rings);
        }
        self.gpu.queue.submit([encoder.finish()]);
        Ok(positions)
    }
}
