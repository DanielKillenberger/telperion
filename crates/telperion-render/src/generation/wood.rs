use super::{io, Clock, Generator, Metrics};
use crate::{buffer::Held, Result};
use telperion_core::surface::{prepared::PreparedSurface, Bounds, SurfaceRun};

pub(super) struct ResidentWood {
    pub positions: Held,
    pub normals: Held,
    pub coords: Held,
    pub indices: Held,
    pub radii: Held,
    pub vertices: usize,
    pub index_count: u32,
    pub runs: Vec<SurfaceRun>,
    pub bounds: Option<Bounds>,
}
impl ResidentWood {
    pub fn bytes(&self) -> u64 {
        [
            &self.positions,
            &self.normals,
            &self.coords,
            &self.indices,
            &self.radii,
        ]
        .iter()
        .map(|b| b.region().capacity())
        .sum()
    }
}
pub(super) struct UploadedWood {
    pub positions: wgpu::Buffer,
    pub metadata: Vec<u32>,
    config: [u32; 8],
    sizes: [u64; 6],
    vertices: usize,
    index_count: u32,
    runs: Vec<SurfaceRun>,
    bounds: Option<Bounds>,
}
impl UploadedWood {
    pub fn metadata_bytes(&self) -> u64 {
        (self.metadata.capacity() * 4 + self.runs.capacity() * size_of::<SurfaceRun>()) as u64
    }
}
impl Generator {
    pub(super) async fn expand_wood(
        &self,
        p: PreparedSurface,
        metrics: &mut Metrics,
    ) -> Result<Option<ResidentWood>> {
        let Some(uploaded) = self.upload_wood(p, metrics)? else {
            return Ok(None);
        };
        self.expand_uploaded_wood(uploaded, metrics).await
    }
    pub(super) fn upload_wood(
        &self,
        p: PreparedSurface,
        metrics: &mut Metrics,
    ) -> Result<Option<UploadedWood>> {
        metrics.wood_prepared_cpu_bytes = prepared_cpu_bytes(&p);
        let limits = self.gpu.device.limits();
        let vertices = p.positions.len() / 3;
        let metadata_words = p.runs.len() * 4 + p.rings.len() * 2 + p.angles.len();
        let sizes = [
            p.positions.len() as u64 * 4,
            vertices as u64 * 12,
            vertices as u64 * 8,
            p.index_count as u64 * 4,
            vertices as u64 * 4,
            metadata_words as u64 * 4,
        ];
        if !compute_fits(&limits, &sizes, p.runs.len() as u64) {
            metrics.wood_fallback = Some("compute limits");
            return Ok(None);
        }
        let start = Clock::now();
        let mut metadata = Vec::<u32>::new();
        metadata
            .try_reserve_exact(metadata_words)
            .map_err(|_| telperion_core::Error::ResourceLimit("surface GPU metadata"))?;
        for r in &p.runs {
            metadata.extend([r.base, r.first_index, r.ring_start, r.rings]);
        }
        let ring_offset = metadata.len() as u32;
        metadata.extend(p.rings.iter().flatten().map(|v| v.to_bits()));
        let angle_offset = metadata.len() as u32;
        metadata.extend(p.angles.iter().map(|v| v.to_bits()));
        let cfg = [
            p.runs.len() as u32,
            p.segments,
            limits.max_compute_workgroups_per_dimension,
            ring_offset,
            angle_offset,
            0,
            0,
            0,
        ];
        let positions = io::buffer(
            &self.gpu,
            "resident wood positions",
            sizes[0],
            wgpu::BufferUsages::STORAGE
                | wgpu::BufferUsages::COPY_SRC
                | wgpu::BufferUsages::COPY_DST
                | wgpu::BufferUsages::VERTEX,
            bytemuck::cast_slice(&p.positions),
        )?;
        metrics.wood_metadata_cpu_bytes = metadata.capacity() as u64 * 4;
        metrics.wood_upload_dispatch_ms = start.elapsed_ms();
        Ok(Some(UploadedWood {
            positions,
            metadata,
            config: cfg,
            sizes,
            vertices,
            index_count: p.index_count,
            runs: p.run_table,
            bounds: p.bounds,
        }))
    }
    pub(super) async fn expand_uploaded_wood(
        &self,
        p: UploadedWood,
        metrics: &mut Metrics,
    ) -> Result<Option<ResidentWood>> {
        let start = Clock::now();
        let UploadedWood {
            positions,
            metadata,
            config,
            sizes,
            vertices,
            index_count,
            runs,
            bounds,
        } = p;
        let storage = wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC;
        let make =
            |label, size, usage, bytes: &[u8]| io::buffer(&self.gpu, label, size, usage, bytes);
        let uniform = make(
            "wood expansion config",
            32,
            wgpu::BufferUsages::UNIFORM,
            bytemuck::cast_slice(&config),
        )?;
        let meta = make(
            "wood expansion metadata",
            sizes[5],
            storage,
            bytemuck::cast_slice(&metadata),
        )?;
        let normals = make(
            "resident wood normals",
            sizes[1],
            storage | wgpu::BufferUsages::VERTEX,
            &[],
        )?;
        let coords = make(
            "resident wood coords",
            sizes[2],
            storage | wgpu::BufferUsages::VERTEX,
            &[],
        )?;
        let indices = make(
            "resident wood indices",
            sizes[3],
            storage | wgpu::BufferUsages::INDEX,
            &[],
        )?;
        let radii = make("resident wood radii", sizes[4], storage, &[])?;
        let status = make("wood expansion status", 16, storage, &[0; 16])?;
        let bind = self.wood.as_ref().expect("resident pipeline").bind(
            &self.gpu,
            &[
                &uniform, &positions, &meta, &normals, &indices, &coords, &radii, &status,
            ],
        );
        let mut encoder = self.gpu.device.create_command_encoder(&Default::default());
        if config[0] != 0 {
            self.wood
                .as_ref()
                .expect("resident pipeline")
                .run(&mut encoder, &bind, 0, config[0]);
        }
        self.gpu.queue.submit([encoder.finish()]);
        metrics.wood_metadata_cpu_bytes = metadata.capacity() as u64 * 4;
        metrics.wood_gpu_peak_bytes = sizes.iter().map(|&n| n.max(16)).sum::<u64>() + 64;
        metrics.wood_upload_dispatch_ms += start.elapsed_ms();
        drop(metadata);
        let wait = Clock::now();
        let result = io::read_async(&self.gpu, &status, 4).await?;
        metrics.wood_wait_ms = wait.elapsed_ms();
        if result != [0, 0, 0, 0] {
            metrics.wood_fallback = Some("GPU normal unusable");
            return Ok(None);
        }
        Ok(Some(ResidentWood {
            positions: Held::resident(positions, sizes[0], "resident wood positions"),
            normals: Held::resident(normals, sizes[1], "resident wood normals"),
            coords: Held::resident(coords, sizes[2], "resident wood coords"),
            indices: Held::resident(indices, sizes[3], "resident wood indices"),
            radii: Held::resident(radii, sizes[4], "resident wood radii"),
            vertices,
            index_count,
            runs,
            bounds,
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
