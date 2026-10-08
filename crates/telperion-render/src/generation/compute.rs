use super::clock::Clock;
use super::{data, io, Generator, Metrics, Resident};
use crate::buffer::{Lent, Owned};
use crate::{buffer::Held, Gpu, Result};
use telperion_core::{
    foliage::{prepared::PreparedStations, Element, Reference, TwigPlacement},
    math::Vec3,
    pipeline::executor::LeafInput,
    surface::Bounds,
};
const STORAGE: wgpu::BufferUsages = wgpu::BufferUsages::STORAGE.union(wgpu::BufferUsages::COPY_SRC);
fn upload<T: bytemuck::Pod>(
    gpu: &Gpu,
    label: &'static str,
    values: &[T],
    usage: wgpu::BufferUsages,
) -> Result<Owned> {
    io::buffer(
        gpu,
        label,
        std::mem::size_of_val(values) as u64,
        usage,
        bytemuck::cast_slice(values),
    )
}
pub(super) fn decoded(bits: u32) -> f64 {
    f32::from_bits(if bits & 0x80000000 != 0 {
        bits ^ 0x80000000
    } else {
        !bits
    }) as f64
}
pub(super) fn bounds(summary: &[u32], offset: usize) -> Bounds {
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
/// Leaves placed on the device and their placement summary.
pub(super) struct Placed {
    pub leaves: Owned,
    pub summary: Owned,
    pub live: u64,
}

impl Generator {
    pub(super) async fn compute_async(
        &self,
        p: PreparedStations,
        l: &LeafInput,
        t: TwigPlacement,
        e: &Element,
        r: Reference,
        m: &mut Metrics,
    ) -> Result<Resident> {
        let started = Clock::now();
        let rings: Vec<_> = p
            .rings
            .iter()
            .map(|v| [v.x as f32, v.y as f32, v.z as f32])
            .collect();
        m.descriptor_cpu_bytes = (p.segments.capacity()
            * size_of::<telperion_core::foliage::prepared::StationSegment>()
            + p.rings.capacity() * size_of::<Vec3>()
            + rings.capacity() * 12) as u64;
        let ring_buffer = upload(&self.gpu, "generation contacts", &rings, STORAGE)?;
        drop((p.rings, rings));
        let stations = PreparedStations {
            segments: p.segments,
            count: p.count,
            ring_size: p.ring_size,
            rings: Lent::Own(ring_buffer),
        };
        let upload_ms = started.elapsed_ms();
        let result = self.compute_buffer_async(stations, l, t, e, r, m).await;
        m.upload_dispatch_ms += upload_ms;
        result
    }

    pub(super) async fn compute_buffer_async(
        &self,
        p: PreparedStations<Lent<'_>>,
        l: &LeafInput,
        t: TwigPlacement,
        e: &Element,
        r: Reference,
        m: &mut Metrics,
    ) -> Result<Resident> {
        let started = Clock::now();
        let gpu = &self.gpu;
        let count = p.count;
        let groups = count.div_ceil(256);
        if u64::from(groups)
            > u64::from(gpu.device.limits().max_compute_workgroups_per_dimension).pow(2)
        {
            return Err(telperion_core::Error::ResourceLimit("GPU foliage dispatch").into());
        }
        let config = data::config(l, t, (count, p.segments.len() as u32, p.ring_size), e, r);
        let segments = data::segments(&p);
        m.descriptor_cpu_bytes = m.descriptor_cpu_bytes.max(
            (p.segments.capacity() * size_of::<telperion_core::foliage::prepared::StationSegment>()
                + segments.capacity() * size_of::<data::Segment>()) as u64,
        );
        let segment_buffer = upload(gpu, "generation segments", &segments, STORAGE)?;
        let ring_buffer = p.rings;
        let retained_contacts = if matches!(ring_buffer, Lent::Borrowed(_)) {
            ring_buffer.size()
        } else {
            0
        };
        drop((p.segments, segments));
        let placed = self.place_leaves(config, (&segment_buffer, &ring_buffer), e, count)?;
        let (leaves, summary, live) = (placed.leaves, placed.summary, placed.live);
        m.upload_dispatch_ms = started.elapsed_ms();
        let wait = Clock::now();
        let bytes = io::read_async(gpu, &summary, 64).await?;
        m.placement_wait_ms = wait.elapsed_ms();
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
        m.instances = count;
        let crown = Some(bounds(&values, 2));
        let full = Some(bounds(&values, 8));
        m.gpu_compute_peak_bytes = live;
        drop((segment_buffer, ring_buffer, summary));
        let start = Clock::now();
        let masses = self
            .mass(&leaves, count, crown, r, retained_contacts, m)
            .await?;
        m.mass_ms = start.elapsed_ms();
        Ok(Resident {
            leaves: Held::resident(leaves, u64::from(count) * 12, "generated foliage"),
            masses: Held::resident(masses.0, masses.1, "generated foliage masses"),
            count,
            crown,
            full,
        })
    }
    /// The leaves `count` stations place from `segments` (`data::Segment`s
    /// on the device), seated on `rings`, under `config`: submitted, with
    /// the placement summary and the bytes the pass held.
    pub(super) fn place_leaves(
        &self,
        config: data::Config,
        (segments, rings): (&wgpu::Buffer, &wgpu::Buffer),
        e: &Element,
        count: u32,
    ) -> Result<Placed> {
        let leaves = self.leaf_buffer(count)?;
        let (summary, live) = self.place_into(config, (segments, rings), e, (count, &leaves))?;
        Ok(Placed {
            leaves,
            summary,
            live,
        })
    }
    /// A buffer of `count` leaves.
    pub(super) fn leaf_buffer(&self, count: u32) -> Result<Owned> {
        // Every placed leaf is drawn, written where it was placed (fn-212 R2).
        io::buffer(
            &self.gpu,
            "generated foliage",
            u64::from(count) * 12,
            STORAGE,
            &[],
        )
    }
    /// `place_leaves` into `leaves`, from the leaf `config` starts at: the
    /// placement summary and the bytes the pass held.
    pub(super) fn place_into(
        &self,
        config: data::Config,
        (segments, rings): (&wgpu::Buffer, &wgpu::Buffer),
        e: &Element,
        (count, leaves): (u32, &wgpu::Buffer),
    ) -> Result<(Owned, u64)> {
        let mut encoder = self.gpu.encoder(&Default::default());
        let placed = self.record_place(
            &mut encoder,
            Placement::Leaves,
            config,
            (segments, rings),
            e,
            (count, leaves),
        )?;
        self.gpu.submit([encoder.finish()]);
        Ok(placed)
    }
    /// `place_into`'s pass recorded into `encoder` and not submitted, so a
    /// block's leaves and its florets go in one submission (fn-229): the
    /// leaves from their segments, or the florets from their blooms'
    /// records, each packed as a leaf is.
    pub(super) fn record_place(
        &self,
        encoder: &mut wgpu::CommandEncoder,
        placement: Placement,
        config: data::Config,
        (segments, rings): (&wgpu::Buffer, &wgpu::Buffer),
        e: &Element,
        (count, leaves): (u32, &wgpu::Buffer),
    ) -> Result<(Owned, u64)> {
        let gpu = &self.gpu;
        let groups = count.div_ceil(256);
        let config = upload(
            gpu,
            "generation config",
            &[config],
            wgpu::BufferUsages::UNIFORM,
        )?;
        let geometry: Vec<_> = e.positions.iter().map(|&v| data::vector(v, 0.0)).collect();
        let geometry = upload(gpu, "generation element", &geometry, STORAGE)?;
        let mut initial = [0u32; 16];
        initial[0] = count;
        for i in 0..12 {
            initial[2 + i] = if (i / 3) % 2 == 0 { u32::MAX } else { 0 };
        }
        let summary = upload(gpu, "generation summary", &initial, STORAGE)?;
        let buffers = [&config, segments, rings, &geometry, leaves, &summary];
        let live = buffers.iter().map(|b| b.size()).sum::<u64>();
        let bind = self.place.bind(gpu, &buffers);
        self.place.run(encoder, &bind, placement as usize, groups);
        Ok((summary, live))
    }
    pub(super) async fn mass(
        &self,
        leaves: &wgpu::Buffer,
        count: u32,
        crown: Option<Bounds>,
        reference: Reference,
        retained_contacts: u64,
        m: &mut Metrics,
    ) -> Result<(Owned, u64)> {
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
        let mut encoder = gpu.encoder(&Default::default());
        self.mass.run(&mut encoder, &bind, 0, count.div_ceil(256));
        self.mass.run(&mut encoder, &bind, 1, total.div_ceil(256));
        gpu.submit([encoder.finish()]);
        io::complete(gpu).await?;
        m.gpu_compute_peak_bytes = m
            .gpu_compute_peak_bytes
            .max(retained_contacts + leaves.size() + config.size() + counts.size() + masses.size());
        Ok((masses, u64::from(total + 8) * 4))
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
impl Generator {
    pub(super) fn compute(
        &self,
        p: PreparedStations,
        l: &LeafInput,
        t: TwigPlacement,
        e: &Element,
        r: Reference,
        m: &mut Metrics,
    ) -> Result<Resident> {
        pollster::block_on(self.compute_async(p, l, t, e, r, m))
    }
}

/// What `place.wgsl` places: leaves at their stations, or the device's
/// florets at their blooms (fn-229).
#[derive(Clone, Copy)]
pub(super) enum Placement {
    Leaves = 0,
    Florets = 1,
}
