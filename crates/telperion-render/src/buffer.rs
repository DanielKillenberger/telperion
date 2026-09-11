//! Allocations with headroom and the live range inside them. Every tree buffer
//! is taken through here, so growth is decided in one place and the decision is
//! testable without a device.
use crate::device::Gpu;

/// A quarter again what the tree needs. The headroom is the append seam: a
/// later streamed refinement writes above `used` without a re-layout.
const HEADROOM: f64 = 1.25;
/// Buffer sizes stay a multiple of this, which every copy and binding wants.
const ALIGNMENT: u64 = wgpu::COPY_BUFFER_ALIGNMENT;

/// An allocation with headroom and the live range inside it. Reuse is decided
/// here and nowhere else, so the decision is testable without a device.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Region {
    capacity: u64,
    used: u64,
}

impl Region {
    /// What to allocate so `bytes` fit with room to grow.
    pub fn capacity_for(bytes: u64) -> u64 {
        let wanted = (bytes as f64 * HEADROOM).ceil() as u64;
        wanted.max(bytes).div_ceil(ALIGNMENT) * ALIGNMENT
    }

    pub fn new(bytes: u64) -> Self {
        Self {
            capacity: Self::capacity_for(bytes),
            used: bytes,
        }
    }

    /// Takes `bytes` as the live range. Reports whether the existing allocation
    /// held them; a `false` means the caller must allocate `capacity` again.
    pub fn fit(&mut self, bytes: u64) -> bool {
        let reused = bytes <= self.capacity;
        if !reused {
            self.capacity = Self::capacity_for(bytes);
        }
        self.used = bytes;
        reused
    }

    pub fn capacity(&self) -> u64 {
        self.capacity
    }

    pub fn used(&self) -> u64 {
        self.used
    }
}

/// A vertex attribute array as long as the vertices it belongs to. A mesh
/// built by hand carries none, and a vertex buffer shorter than the draw would
/// read past its own end on the device, so what is missing goes up as zeros.
pub fn attributes(values: &[f32], floats: usize) -> std::borrow::Cow<'_, [f32]> {
    if values.len() == floats {
        std::borrow::Cow::Borrowed(values)
    } else {
        std::borrow::Cow::Owned(vec![0.0; floats])
    }
}

/// One buffer and the region that says how much of it is live.
pub struct Held {
    buffer: wgpu::Buffer,
    region: Region,
    usage: wgpu::BufferUsages,
    label: &'static str,
}

impl Held {
    fn new(gpu: &Gpu, label: &'static str, usage: wgpu::BufferUsages, bytes: &[u8]) -> Self {
        let region = Region::new(bytes.len() as u64);
        Self {
            buffer: allocate(gpu, label, usage, region.capacity(), bytes),
            region,
            usage,
            label,
        }
    }

    fn new_reserved(gpu: &Gpu, label: &'static str, usage: wgpu::BufferUsages, bytes: u64) -> Self {
        let region = Region::new(bytes);
        Self {
            buffer: allocate(gpu, label, usage, region.capacity(), &[]),
            region,
            usage,
            label,
        }
    }

    /// Writes the new contents, reusing the allocation whenever they fit.
    fn write(&mut self, gpu: &Gpu, bytes: &[u8]) {
        if self.region.fit(bytes.len() as u64) {
            gpu.queue.write_buffer(&self.buffer, 0, bytes);
        } else {
            self.buffer = allocate(gpu, self.label, self.usage, self.region.capacity(), bytes);
        }
    }

    /// Takes the same live range with nothing in it, for a buffer the GPU
    /// fills and the host only reads. What was there before is left there:
    /// a caller that reserves is a caller that writes before it reads.
    fn reserve(&mut self, gpu: &Gpu, bytes: u64) {
        if !self.region.fit(bytes) {
            self.buffer = allocate(gpu, self.label, self.usage, self.region.capacity(), &[]);
        }
    }

    /// The whole allocation, headroom and all: what a binding, a clear or an
    /// indirect draw addresses by offset rather than by slice.
    pub fn buffer(&self) -> &wgpu::Buffer {
        &self.buffer
    }

    /// Just the live range, never the headroom above it.
    pub fn live(&self) -> wgpu::BufferSlice<'_> {
        self.buffer.slice(0..self.region.used())
    }

    pub fn region(&self) -> Region {
        self.region
    }
}

fn allocate(
    gpu: &Gpu,
    label: &'static str,
    usage: wgpu::BufferUsages,
    capacity: u64,
    bytes: &[u8],
) -> wgpu::Buffer {
    let buffer = gpu.device.create_buffer(&wgpu::BufferDescriptor {
        label: Some(label),
        size: capacity,
        usage: usage | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });
    if !bytes.is_empty() {
        gpu.queue.write_buffer(&buffer, 0, bytes);
    }
    buffer
}

/// Fills a slot, allocating it the first time and reusing it after.
pub fn write(
    gpu: &Gpu,
    slot: &mut Option<Held>,
    label: &'static str,
    usage: wgpu::BufferUsages,
    bytes: &[u8],
) {
    match slot {
        Some(held) => held.write(gpu, bytes),
        None => *slot = Some(Held::new(gpu, label, usage, bytes)),
    }
}

/// Takes a slot of this size with nothing in it, for a buffer the GPU is the
/// one that fills. Zero bytes is no buffer at all: a device refuses an empty
/// allocation, and a caller with nothing to hold has nothing to bind either.
pub fn reserve(
    gpu: &Gpu,
    slot: &mut Option<Held>,
    label: &'static str,
    usage: wgpu::BufferUsages,
    bytes: u64,
) {
    if bytes == 0 {
        *slot = None;
        return;
    }
    match slot {
        Some(held) => held.reserve(gpu, bytes),
        None => *slot = Some(Held::new_reserved(gpu, label, usage, bytes)),
    }
}

/// Brings a buffer's live range back to the host, copy and all. The device is
/// waited on, so this belongs to a check or a record and never to a frame; a
/// device that will not give the bytes up reports nothing rather than half of
/// them. The buffer must have been taken with `COPY_SRC`.
#[cfg(not(target_arch = "wasm32"))]
pub fn read(gpu: &Gpu, held: &Held) -> Option<Vec<u8>> {
    let size = held.region().used();
    let readback = gpu.device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("readback"),
        size,
        usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
        mapped_at_creation: false,
    });
    let mut encoder = gpu
        .device
        .create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("readback"),
        });
    encoder.copy_buffer_to_buffer(held.buffer(), 0, &readback, 0, size);
    gpu.queue.submit([encoder.finish()]);

    let (sender, receiver) = std::sync::mpsc::channel();
    readback
        .slice(..)
        .map_async(wgpu::MapMode::Read, move |result| {
            let _ = sender.send(result);
        });
    gpu.device.poll(wgpu::PollType::wait_indefinitely()).ok()?;
    receiver.recv().ok()?.ok()?;
    let view = readback.slice(..).get_mapped_range().ok()?;
    let bytes = view.to_vec();
    drop(view);
    readback.unmap();
    Some(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_allocation_carries_headroom_above_what_it_holds() {
        let region = Region::new(1_000);
        assert!(region.capacity() >= 1_250, "{}", region.capacity());
        assert_eq!(region.used(), 1_000);
        assert_eq!(region.capacity() % ALIGNMENT, 0);
    }

    #[test]
    fn a_second_smaller_submit_reuses_the_allocation() {
        let mut region = Region::new(1_000);
        let capacity = region.capacity();
        assert!(region.fit(400), "a smaller tree forced a re-layout");
        assert_eq!(region.capacity(), capacity, "the allocation moved");
        assert_eq!(region.used(), 400, "the live range did not follow the tree");
    }

    #[test]
    fn a_submit_that_outgrows_the_headroom_allocates_again() {
        let mut region = Region::new(1_000);
        assert!(region.fit(1_200), "the headroom went unused");
        assert!(
            !region.fit(4_000),
            "a tree beyond the headroom was let through"
        );
        assert!(region.capacity() >= 5_000);
        assert_eq!(region.used(), 4_000);
    }

    #[test]
    fn an_empty_tree_asks_for_nothing() {
        let region = Region::new(0);
        assert_eq!(region.capacity(), 0);
        assert_eq!(region.used(), 0);
    }
}
