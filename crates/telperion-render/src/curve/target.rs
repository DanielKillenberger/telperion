//! One view's output budget and the buffers the passes fill (fn-208, host
//! decisions 19 and 21): sized from the view's pixels, each buffer larger
//! than a binding bound in slices that do not overlap.
use super::{CurveGpu, BINDING, PASSES, RECORD_WORDS, VERTEX_FLOATS};
use crate::device::Gpu;

/// What a view may write per pixel it covers: rings, vertices, tube indices
/// and ribbon indices (host decision 19). The most the four engine species
/// asked for at the finest scale, at their hero view, limb, twig and the shot
/// along a twig at 960 x 720, was 3.13, 9.40, 8.17 and 25.75 (the spruce's
/// hero and the oak's twig, STEP5.md); each bound stands about a quarter
/// above it.
const PER_PIXEL: [f64; 4] = [4.0, 12.0, 10.0, 32.0];

/// The most one view writes: rings, vertices, tube indices, ribbon indices.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Budget {
    pub rings: u32,
    pub vertices: u32,
    pub tube_indices: u32,
    pub ribbon_indices: u32,
}

impl Budget {
    /// The budget of a view of `pixels`, within what the device's bindings
    /// and buffers hold: the records, the vertices and the indices each in
    /// two bindings, a slice's alignment spare, and every buffer under
    /// `max_buffer_size`. A view past them is drawn coarser, and says so.
    pub fn for_pixels(pixels: u64, limits: &wgpu::Limits) -> Self {
        let largest = limits.max_buffer_size;
        let wanted = |k: usize| (pixels as f64 * PER_PIXEL[k]).ceil() as u64;
        let held = |bytes: u64| (2 * (BINDING - 4096)).min(largest) / bytes;
        let records = held(RECORD_WORDS * 4);
        let vertices = held(VERTEX_FLOATS * 4);
        let indices = held(4);
        let tube = wanted(2).min(indices / 3);
        let clamp = |v: u64| v.min(u64::from(u32::MAX)) as u32;
        Self {
            rings: clamp(wanted(0).min(records)),
            vertices: clamp(wanted(1).min(vertices)),
            tube_indices: clamp(tube),
            ribbon_indices: clamp(wanted(3).min(indices - tube)),
        }
    }
}

impl Budget {
    /// This budget, no larger than what the tree could ask of the view
    /// (`Demand::at`; host decision 23): a palm holds a palm's.
    pub fn within(self, finest: [u64; 4]) -> Self {
        let cap = |b: u32, k: usize| b.min(finest[k].min(u64::from(u32::MAX)) as u32);
        Self {
            rings: cap(self.rings, 0),
            vertices: cap(self.vertices, 1),
            tube_indices: cap(self.tube_indices, 2),
            ribbon_indices: cap(self.ribbon_indices, 3),
        }
    }
}

/// What one view's passes wrote, read back from the device.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct CurveReport {
    /// The error scale the budget let the view be drawn at.
    pub scale: f64,
    /// 1: no scale fitted the budget; 2: a cluster past it was left out.
    pub overrun: u32,
    pub vertices: u32,
    pub tube_triangles: u32,
    pub ribbon_triangles: u32,
    /// What the view asked for at the finest scale: rings, vertices, tube
    /// and ribbon indices.
    pub demand: [u32; 4],
    pub budget: [u32; 4],
}

/// One view's buffers, its slices and the bind group each pass reads.
pub(crate) struct Target {
    pub(crate) budget: Budget,
    pub(super) config: wgpu::Buffer,
    pub(super) totals: wgpu::Buffer,
    pub(crate) vertices: wgpu::Buffer,
    pub(crate) indices: wgpu::Buffer,
    pub(crate) args: wgpu::Buffer,
    pub(super) vertex_slice: u64,
    pub(super) index_slice: u64,
    pub(super) record_slice: u64,
    pub(super) groups: Vec<wgpu::BindGroup>,
    held: Vec<wgpu::Buffer>,
}

pub(super) fn buffer(
    gpu: &Gpu,
    label: &str,
    bytes: u64,
    usage: wgpu::BufferUsages,
) -> wgpu::Buffer {
    gpu.device.create_buffer(&wgpu::BufferDescriptor {
        label: Some(label),
        size: bytes.max(256).div_ceil(256) * 256,
        usage: usage | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    })
}

/// `elements` words cut into `slices` equal slices on 256-byte boundaries:
/// the words a slice holds.
fn slice(elements: u64, slices: u64) -> u64 {
    elements.div_ceil(slices).max(1).div_ceil(64) * 64
}

fn whole(b: &wgpu::Buffer) -> wgpu::BindingResource<'_> {
    b.as_entire_binding()
}

fn part(b: &wgpu::Buffer, offset: u64, size: u64) -> wgpu::BindingResource<'_> {
    wgpu::BindingResource::Buffer(wgpu::BufferBinding {
        buffer: b,
        offset,
        size: std::num::NonZeroU64::new(size.max(4)),
    })
}

impl Target {
    pub(super) fn new(gpu: &Gpu, label: &str, budget: Budget, curve: &CurveGpu) -> Self {
        use wgpu::BufferUsages as U;
        let per = u64::from(curve.count.max(1)) * 16;
        let blocks = u64::from(curve.count.div_ceil(256).max(1)) * 16;
        // Floats a vertex slice holds and indices an index slice holds, so
        // the slices tile the buffer without overlapping.
        let vertex_slice = slice(u64::from(budget.vertices) * VERTEX_FLOATS, 2);
        // Whole records a slice, on a 256-byte boundary: 32 of 88 bytes.
        let record_slice = u64::from(budget.rings).div_ceil(2).div_ceil(32).max(1) * 32;
        let indices = u64::from(budget.tube_indices) + u64::from(budget.ribbon_indices);
        let index_slice = slice(indices, 2);
        let made = [
            buffer(gpu, label, 208, U::UNIFORM),
            buffer(gpu, label, per, U::STORAGE),
            buffer(gpu, label, per, U::STORAGE),
            buffer(gpu, label, blocks, U::STORAGE),
            buffer(gpu, label, 192, U::STORAGE | U::COPY_SRC),
            buffer(gpu, label, 2 * record_slice * RECORD_WORDS * 4, U::STORAGE),
            buffer(
                gpu,
                label,
                2 * vertex_slice * 4,
                U::STORAGE | U::VERTEX | U::COPY_SRC,
            ),
            buffer(
                gpu,
                label,
                2 * index_slice * 4,
                U::STORAGE | U::INDEX | U::COPY_SRC,
            ),
            buffer(gpu, label, 64, U::STORAGE | U::INDIRECT | U::COPY_SRC),
        ];
        let [config, counts, offsets, block, totals, records, vertices, index, args] = made;
        // The points' second slice, or where they fit one, a read-only
        // repeat of the first's start.
        let first = curve.point_slice * 4;
        let rest = curve.point_bytes.saturating_sub(first);
        let (rest_offset, rest_size) = if rest > 0 { (first, rest) } else { (0, 256) };
        let resource = |binding: u32| -> wgpu::BindingResource<'_> {
            match binding {
                0 => whole(&config),
                1 => part(&curve.points, 0, first.max(4)),
                2 => part(&curve.points, rest_offset, rest_size),
                3 => whole(&curve.clusters),
                4 => whole(&curve.sections),
                5 => whole(&counts),
                6 => whole(&offsets),
                7 => whole(&block),
                8 => whole(&totals),
                9 | 12 => {
                    let bytes = record_slice * RECORD_WORDS * 4;
                    part(&records, u64::from(binding == 12) * bytes, bytes)
                }
                10 | 11 => part(
                    &vertices,
                    u64::from(binding - 10) * vertex_slice * 4,
                    vertex_slice * 4,
                ),
                13 | 14 => part(
                    &index,
                    u64::from(binding - 13) * index_slice * 4,
                    index_slice * 4,
                ),
                _ => whole(&args),
            }
        };
        let groups = PASSES
            .iter()
            .zip(&curve.pipelines)
            .map(|((entry, bindings), pipeline)| {
                let entries: Vec<_> = bindings
                    .iter()
                    .map(|&b| wgpu::BindGroupEntry {
                        binding: b,
                        resource: resource(b),
                    })
                    .collect();
                gpu.device.create_bind_group(&wgpu::BindGroupDescriptor {
                    label: Some(entry),
                    layout: &pipeline.get_bind_group_layout(0),
                    entries: &entries,
                })
            })
            .collect();
        Self {
            budget,
            config,
            totals,
            vertices,
            indices: index,
            args,
            vertex_slice,
            index_slice,
            record_slice,
            groups,
            held: vec![counts, offsets, block, records],
        }
    }

    /// Bytes this view holds on the device.
    pub(crate) fn bytes(&self) -> u64 {
        let own = [
            &self.vertices,
            &self.indices,
            &self.args,
            &self.totals,
            &self.config,
        ];
        own.iter()
            .chain(self.held.iter().collect::<Vec<_>>().iter())
            .map(|b| b.size())
            .sum()
    }
}

/// The report a view's totals words make under its budget.
fn reported(b: Budget, words: &[u32]) -> CurveReport {
    CurveReport {
        scale: f64::from(1u32 << words[16].min(3)),
        overrun: words[17],
        vertices: words[24],
        tube_triangles: words[25] / 3,
        ribbon_triangles: words[26] / 3,
        demand: [words[0], words[1], words[2], words[3]],
        budget: [b.rings, b.vertices, b.tube_indices, b.ribbon_indices],
    }
}

impl Target {
    /// What the last frame's passes wrote, awaited: the browser's readback.
    /// The copy is queued now and the future owns everything it reads, so
    /// no borrow of the renderer is held across the wait.
    #[cfg(target_arch = "wasm32")]
    pub(crate) fn report_async(
        &self,
        gpu: &Gpu,
    ) -> impl std::future::Future<Output = Option<CurveReport>> + 'static {
        let staging = gpu.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("curve readback"),
            size: 192,
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            mapped_at_creation: false,
        });
        let mut encoder = gpu.device.create_command_encoder(&Default::default());
        encoder.copy_buffer_to_buffer(&self.totals, 0, &staging, 0, 192);
        gpu.queue.submit([encoder.finish()]);
        let (sender, receiver) = futures_channel::oneshot::channel();
        staging.slice(..).map_async(wgpu::MapMode::Read, move |r| {
            let _ = sender.send(r);
        });
        let budget = self.budget;
        async move {
            receiver.await.ok()?.ok()?;
            let words: Vec<u32> =
                bytemuck::cast_slice(&staging.slice(..).get_mapped_range().ok()?).to_vec();
            Some(reported(budget, &words))
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl Target {
    /// What the last frame's passes wrote, read back.
    pub(crate) fn report(&self, gpu: &Gpu) -> CurveReport {
        let words: Vec<u32> = bytemuck::cast_slice(&read(gpu, &self.totals, 192)).to_vec();
        reported(self.budget, &words)
    }

    /// The last frame's vertices (nine floats each), tube and ribbon indices.
    pub(crate) fn mesh(&self, gpu: &Gpu) -> (Vec<f32>, Vec<u32>, Vec<u32>) {
        let r = self.report(gpu);
        let floats = u64::from(r.vertices) * VERTEX_FLOATS;
        let vertices = bytemuck::cast_slice(&read(gpu, &self.vertices, floats * 4)).to_vec();
        let all: Vec<u32> =
            bytemuck::cast_slice(&read(gpu, &self.indices, self.indices.size())).to_vec();
        let tube = all[..(r.tube_triangles * 3) as usize].to_vec();
        let base = self.budget.tube_indices as usize;
        let ribbon = all[base..base + (r.ribbon_triangles * 3) as usize].to_vec();
        (vertices, tube, ribbon)
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn read(gpu: &Gpu, source: &wgpu::Buffer, bytes: u64) -> Vec<u8> {
    let bytes = bytes.max(4).div_ceil(4) * 4;
    let staging = gpu.device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("curve readback"),
        size: bytes,
        usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
        mapped_at_creation: false,
    });
    let mut encoder = gpu.device.create_command_encoder(&Default::default());
    encoder.copy_buffer_to_buffer(source, 0, &staging, 0, bytes);
    gpu.queue.submit([encoder.finish()]);
    let slice = staging.slice(..);
    slice.map_async(wgpu::MapMode::Read, |_| {});
    let _ = gpu.device.poll(wgpu::PollType::wait_indefinitely());
    let data = slice.get_mapped_range().expect("mapped").to_vec();
    staging.unmap();
    data
}
