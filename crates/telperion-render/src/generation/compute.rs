use super::{data, io, Generator, Metrics, Resident};
use crate::{buffer::Held, Gpu, Result};
use std::time::Instant;
use telperion_core::{
    foliage::{prepared::PreparedStations, Element, Reference, TwigPlacement},
    math::Vec3,
    surface::Bounds,
    Family,
};
const STORAGE: wgpu::BufferUsages = wgpu::BufferUsages::STORAGE.union(wgpu::BufferUsages::COPY_SRC);
fn upload<T: bytemuck::Pod>(
    gpu: &Gpu,
    label: &'static str,
    values: &[T],
    usage: wgpu::BufferUsages,
) -> Result<wgpu::Buffer> {
    io::buffer(
        gpu,
        label,
        std::mem::size_of_val(values) as u64,
        usage,
        bytemuck::cast_slice(values),
    )
}
fn decoded(bits: u32) -> f64 {
    f32::from_bits(if bits & 0x80000000 != 0 {
        bits ^ 0x80000000
    } else {
        !bits
    }) as f64
}
fn bounds(summary: &[u32], offset: usize) -> Bounds {
    Bounds {
        min: Vec3::new(
            decoded(summary[offset]),
            decoded(summary[offset + 1]),
            decoded(summary[offset + 2]),
        ),
        max: Vec3::new(
            decoded(summary[offset + 3]),
            decoded(summary[offset + 4]),
            decoded(summary[offset + 5]),
        ),
    }
}
impl Generator {
    pub(super) fn compute(
        &self,
        p: PreparedStations,
        f: &Family,
        t: TwigPlacement,
        e: &Element,
        r: Reference,
        m: &mut Metrics,
    ) -> Result<Resident> {
        let started = Instant::now();
        let gpu = &self.gpu;
        let count = p.count;
        let groups = count.div_ceil(256);
        if u64::from(groups)
            > u64::from(gpu.device.limits().max_compute_workgroups_per_dimension).pow(2)
        {
            return Err(telperion_core::Error::ResourceLimit("GPU foliage dispatch").into());
        }
        let config = data::config(f, t, &p, e, r);
        let config = upload(
            gpu,
            "generation config",
            &[config],
            wgpu::BufferUsages::UNIFORM,
        )?;
        let segments = data::segments(&p);
        let rings: Vec<_> = p.rings.iter().map(|&v| data::vector(v, 0.0)).collect();
        m.descriptor_cpu_bytes = (p.segments.capacity()
            * size_of::<telperion_core::foliage::prepared::StationSegment>()
            + p.rings.capacity() * size_of::<Vec3>()
            + segments.capacity() * size_of::<data::Segment>()
            + rings.capacity() * 16) as u64;
        let segment_buffer = upload(gpu, "generation segments", &segments, STORAGE)?;
        let ring_buffer = upload(gpu, "generation contacts", &rings, STORAGE)?;
        drop((p, segments, rings));
        let mut geometry: Vec<_> = e.positions.iter().map(|&v| data::vector(v, 0.0)).collect();
        geometry.extend(
            f.skeleton
                .envelope
                .profile()
                .iter()
                .map(|v| [v[0] as f32, v[1] as f32, 0.0, 0.0]),
        );
        let geometry = upload(gpu, "generation element and shell", &geometry, STORAGE)?;
        let raw = io::buffer(gpu, "generation raw", u64::from(count) * 12, STORAGE, &[])?;
        let ranks = io::buffer(gpu, "generation ranks", u64::from(count) * 4, STORAGE, &[])?;
        let mut initial = vec![0u32; 16 + groups as usize];
        for i in 0..12 {
            initial[2 + i] = if (i / 3) % 2 == 0 { u32::MAX } else { 0 };
        }
        let summary = upload(gpu, "generation summary", &initial, STORAGE)?;
        let live = [
            &config,
            &segment_buffer,
            &ring_buffer,
            &geometry,
            &raw,
            &ranks,
            &summary,
        ]
        .iter()
        .map(|b| b.size())
        .sum::<u64>();
        let bind = self.place.bind(
            gpu,
            &[
                &config,
                &segment_buffer,
                &ring_buffer,
                &geometry,
                &raw,
                &ranks,
                &summary,
            ],
        );
        let mut encoder = gpu.device.create_command_encoder(&Default::default());
        self.place.run(&mut encoder, &bind, 0, groups);
        self.place.run(&mut encoder, &bind, 1, 1);
        gpu.queue.submit([encoder.finish()]);
        m.upload_dispatch_ms = started.elapsed().as_secs_f64() * 1000.0;
        let wait = Instant::now();
        let bytes = io::read(gpu, &summary, 64)?;
        m.placement_wait_ms = wait.elapsed().as_secs_f64() * 1000.0;
        let values: Vec<u32> = bytes
            .chunks_exact(4)
            .map(|b| u32::from_ne_bytes(b.try_into().unwrap()))
            .collect();
        if values[1] != 0 {
            return Err(telperion_core::Error::InvalidInput(
                "GPU foliage transform/contact/reference",
            )
            .into());
        }
        let survivors = values[0];
        m.input_instances = count;
        m.instances = survivors;
        let crown = (survivors > 0).then(|| bounds(&values, 2));
        let full = (survivors > 0).then(|| bounds(&values, 8));
        let start = Instant::now();
        let leaves = io::buffer(
            gpu,
            "generated foliage",
            u64::from(survivors) * 12,
            STORAGE,
            &[],
        )?;
        let compact_config = upload(
            gpu,
            "generation compact config",
            &[[count, groups, 0, 0]],
            wgpu::BufferUsages::UNIFORM,
        )?;
        let compact_bind = self
            .compact
            .bind(gpu, &[&compact_config, &raw, &ranks, &summary, &leaves]);
        let mut encoder = gpu.device.create_command_encoder(&Default::default());
        self.compact.run(&mut encoder, &compact_bind, 0, groups);
        gpu.queue.submit([encoder.finish()]);
        gpu.device
            .poll(wgpu::PollType::wait_indefinitely())
            .map_err(|e| io::error(e.to_string()))?;
        m.compact_ms = start.elapsed().as_secs_f64() * 1000.0;
        m.gpu_compute_peak_bytes = live + leaves.size() + compact_config.size() + 64;
        drop((
            bind,
            compact_bind,
            config,
            segment_buffer,
            ring_buffer,
            geometry,
            raw,
            ranks,
            summary,
            compact_config,
        ));
        let start = Instant::now();
        let masses = self.mass(&leaves, survivors, crown, r, m)?;
        m.mass_ms = start.elapsed().as_secs_f64() * 1000.0;
        Ok(Resident {
            leaves: Held::resident(leaves, u64::from(survivors) * 12, "generated foliage"),
            masses: Held::resident(masses.0, masses.1, "generated foliage masses"),
            count: survivors,
            crown,
            full,
        })
    }
    fn mass(
        &self,
        leaves: &wgpu::Buffer,
        count: u32,
        crown: Option<Bounds>,
        reference: Reference,
        m: &mut Metrics,
    ) -> Result<(wgpu::Buffer, u64)> {
        let gpu = &self.gpu;
        let Some(crown) = crown else {
            return Ok((
                upload(gpu, "empty generated mass", &[0f32; 8], STORAGE)?,
                32,
            ));
        };
        let size = crown.max - crown.min;
        let cells = ((count as f64 / 8.0).cbrt().clamp(1.0, 64.0)).floor();
        let edge = size.x.max(size.y).max(size.z) / cells;
        if edge <= 0.0 || !edge.is_finite() {
            return Ok((
                upload(gpu, "empty generated mass", &[0f32; 8], STORAGE)?,
                32,
            ));
        }
        let dims = [size.x, size.y, size.z].map(|v| ((v / edge).floor() as u32 + 1).min(64));
        let total = dims.iter().product::<u32>();
        let row = data::Mass {
            count,
            reach: (cells * 0.125).round().max(1.0) as u32,
            pad: [0; 2],
            minimum: data::vector(reference.min, 0.0),
            extent: data::vector(reference.extent, 0.0),
            grid_min: data::vector(crown.min, edge),
            dims: [dims[0], dims[1], dims[2], 0],
        };
        let config = upload(
            gpu,
            "generation mass config",
            &[row],
            wgpu::BufferUsages::UNIFORM,
        )?;
        let counts = io::buffer(
            gpu,
            "generation occupancy",
            u64::from(total + 1) * 4,
            STORAGE,
            &[],
        )?;
        let masses = io::buffer(
            gpu,
            "generated mass",
            u64::from(total + 8) * 4,
            STORAGE,
            &[],
        )?;
        let bind = self.mass.bind(gpu, &[&config, leaves, &counts, &masses]);
        let mut encoder = gpu.device.create_command_encoder(&Default::default());
        self.mass.run(&mut encoder, &bind, 0, count.div_ceil(256));
        self.mass.run(&mut encoder, &bind, 1, total.div_ceil(256));
        gpu.queue.submit([encoder.finish()]);
        gpu.device
            .poll(wgpu::PollType::wait_indefinitely())
            .map_err(|e| io::error(e.to_string()))?;
        m.gpu_compute_peak_bytes = m
            .gpu_compute_peak_bytes
            .max(leaves.size() + config.size() + counts.size() + masses.size());
        Ok((masses, u64::from(total + 8) * 4))
    }
}
