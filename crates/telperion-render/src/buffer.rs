//! Allocations with headroom and the live range inside them. Every tree buffer
//! is taken through here, so growth is decided in one place and the decision is
//! testable without a device.
use crate::device::Gpu;

/// A quarter again what the tree needs. The headroom is the append seam: a
/// later streamed refinement writes above `used` without a re-layout.
const HEADROOM: f64 = 1.25;
/// Buffer sizes stay a multiple of this, which every copy and binding wants.
const ALIGNMENT: u64 = wgpu::COPY_BUFFER_ALIGNMENT;

/// A buffer destroyed when its owner lets it go. A browser frees a buffer
/// only when it is destroyed or its wrapper is collected (wgpu's web backend
/// drops nothing), so every buffer the renderer allocates is owned through
/// this and freed where its owner ends (fn-212 task 10, host decision 2).
/// A pass may bind a buffer its owner lets go before the encoder is
/// submitted, so the destroy waits for every encoder open when it was let
/// go (`Ledger::close`; fn-212 task 11).
#[derive(Debug)]
pub(crate) struct Owned {
    buffer: wgpu::Buffer,
    ledger: std::sync::Arc<Ledger>,
    label: &'static str,
    /// Whether it was let go ahead of its owner (`destroy`).
    gone: std::sync::atomic::AtomicBool,
}

/// The device's owned buffers: those let go and not yet destroyed, and the
/// bytes charged now and at most since the peak was last read (fn-212 task
/// 10: a browser build is checked against a stated budget). A buffer's
/// bytes are charged before it is made (`reserve`) and stay charged until
/// the device has finished every submission made before its destroy, when
/// the device frees it (fn-256 task 5: a dropped build's tables held the
/// card while the next build's were made).
#[derive(Debug, Default)]
pub(crate) struct Ledger {
    waiting: std::sync::Mutex<Waiting>,
    counts: std::sync::Arc<Counts>,
    /// The caps of the builds running now, each its id and bytes
    /// (`capped`): the least holds every buffer made (fn-256 task 5
    /// review: two builds on one device each keep theirs).
    caps: std::sync::Mutex<(u64, Vec<(u64, u64)>)>,
    /// The queue whose completion frees a destroyed buffer's bytes; none
    /// releases them at the destroy (a ledger with no device).
    queue: Option<wgpu::Queue>,
    /// The submissions made on the device (`Gpu::submit`).
    #[cfg(test)]
    submits: std::sync::atomic::AtomicU64,
}

/// The bytes charged, their peak, and (in tests) by label.
#[derive(Debug, Default)]
struct Counts {
    live: std::sync::atomic::AtomicU64,
    peak: std::sync::atomic::AtomicU64,
    /// The most charged since a watch began (`Ledger::watch`).
    watched: std::sync::atomic::AtomicU64,
    #[cfg(test)]
    labels: std::sync::Mutex<Labels>,
}

/// The bytes charged by each label now, and as they stood at the peak.
#[cfg(test)]
#[derive(Debug, Default)]
struct Labels {
    now: std::collections::BTreeMap<&'static str, u64>,
    at_peak: std::collections::BTreeMap<&'static str, u64>,
}

impl Counts {
    /// `bytes` charged to `label`, refused where they would pass `cap`
    /// (none at zero): the bytes charged after, and the cap.
    fn charge(&self, bytes: u64, label: &'static str, cap: u64) -> std::result::Result<(), (u64, u64)> {
        use std::sync::atomic::Ordering::Relaxed;
        let mut now = self.live.load(Relaxed);
        loop {
            let after = now.saturating_add(bytes);
            if cap != 0 && after > cap {
                return Err((after, cap));
            }
            match self.live.compare_exchange_weak(now, after, Relaxed, Relaxed) {
                Ok(_) => {
                    let peaked = self.peak.fetch_max(after, Relaxed) < after;
                    self.watched.fetch_max(after, Relaxed);
                    self.label(label, bytes as i64, peaked);
                    return Ok(());
                }
                Err(seen) => now = seen,
            }
        }
    }

    fn release(&self, bytes: u64, label: &'static str) {
        self.live.fetch_sub(bytes, std::sync::atomic::Ordering::Relaxed);
        self.label(label, -(bytes as i64), false);
    }

    #[cfg(test)]
    fn label(&self, label: &'static str, bytes: i64, peaked: bool) {
        let mut l = self.labels.lock().unwrap_or_else(|e| e.into_inner());
        let held = l.now.entry(label).or_default();
        *held = held.saturating_add_signed(bytes);
        if peaked {
            l.at_peak = l.now.clone();
        }
    }

    #[cfg(not(test))]
    fn label(&self, _: &'static str, _: i64, _: bool) {}
}

/// A build's cap on the ledger, lifted when it is dropped; any other
/// build's stays.
pub(crate) struct Capped(std::sync::Arc<Ledger>, u64);

impl Drop for Capped {
    fn drop(&mut self) {
        self.0.caps().1.retain(|c| c.0 != self.1);
    }
}

/// The encoders open now, and each buffer let go with those it waits on.
#[derive(Debug, Default)]
struct Waiting {
    next: u64,
    open: Vec<u64>,
    let_go: Vec<(wgpu::Buffer, &'static str, Vec<u64>)>,
}

impl Ledger {
    /// The ledger of the device `queue` submits to.
    pub(crate) fn of(queue: &wgpu::Queue) -> Self {
        Self { queue: Some(queue.clone()), ..Self::default() }
    }

    fn waiting(&self) -> std::sync::MutexGuard<'_, Waiting> {
        self.waiting.lock().unwrap_or_else(|e| e.into_inner())
    }

    /// Opens an encoder, named by what it returns.
    pub(crate) fn open(&self) -> u64 {
        let mut waiting = self.waiting();
        let id = waiting.next;
        waiting.next += 1;
        waiting.open.push(id);
        id
    }

    /// Closes encoder `id`, submitted or dropped, and destroys each
    /// buffer that waited on no other.
    pub(crate) fn close(&self, id: u64) {
        let mut done = Vec::new();
        let mut waiting = self.waiting();
        waiting.open.retain(|&o| o != id);
        // A buffer some open encoder may still bind stays; one none does
        // is destroyed, its record let go.
        waiting.let_go.retain_mut(|(buffer, label, on)| {
            on.retain(|&o| o != id);
            !on.is_empty() || {
                done.push((buffer.clone(), *label));
                false
            }
        });
        drop(waiting);
        done.iter().for_each(|(b, label)| self.destroy(b, label));
    }

    /// `buffer` destroyed, its bytes charged until the device has done
    /// every submission made before now: the device frees it only then,
    /// so a cap (`reserve`) counts a move's old tables beside its new, and
    /// a dropped build's tables beside the next build's until the device
    /// is polled free (`io::complete`).
    fn destroy(&self, buffer: &wgpu::Buffer, label: &'static str) {
        buffer.destroy();
        let bytes = buffer.size();
        match &self.queue {
            Some(queue) => {
                let counts = self.counts.clone();
                queue.on_submitted_work_done(move || counts.release(bytes, label));
            }
            None => self.counts.release(bytes, label),
        }
    }

    /// Destroys `buffer` now, or once every encoder open now has closed:
    /// any of them may bind it.
    fn let_go(&self, buffer: &wgpu::Buffer, label: &'static str) {
        let mut waiting = self.waiting();
        if waiting.open.is_empty() {
            drop(waiting);
            self.destroy(buffer, label);
        } else {
            let on = waiting.open.clone();
            waiting.let_go.push((buffer.clone(), label, on));
        }
    }

    /// The ledger capped at `bytes` until the guard is dropped: a buffer
    /// that would pass it is refused before it is made (`reserve`).
    pub(crate) fn capped(self: &std::sync::Arc<Self>, bytes: u64) -> Capped {
        let mut caps = self.caps();
        let id = caps.0;
        caps.0 += 1;
        caps.1.push((id, bytes.max(1)));
        drop(caps);
        Capped(self.clone(), id)
    }

    fn caps(&self) -> std::sync::MutexGuard<'_, (u64, Vec<(u64, u64)>)> {
        self.caps.lock().unwrap_or_else(|e| e.into_inner())
    }

    /// `bytes` for `label` charged before the buffer is made, refused
    /// where they would pass the cap: the bytes charged after, and the
    /// cap. Taken in one step, so two makers never both fit in the room
    /// one of them had.
    pub(crate) fn reserve(&self, bytes: u64, label: &'static str) -> std::result::Result<(), (u64, u64)> {
        let cap = self.caps().1.iter().map(|c| c.1).min().unwrap_or(0);
        self.counts.charge(bytes, label, cap)
    }

    /// Whether `bytes` more would fit the cap, charging nothing (a build's
    /// room for its first move, fn-276 task 2): the bytes they would make,
    /// and the cap, where they would not.
    pub(crate) fn fits(&self, bytes: u64) -> std::result::Result<(), (u64, u64)> {
        let cap = self.caps().1.iter().map(|c| c.1).min().unwrap_or(0);
        let after = self.counts.live.load(std::sync::atomic::Ordering::Relaxed).saturating_add(bytes);
        if cap != 0 && after > cap {
            return Err((after, cap));
        }
        Ok(())
    }

    /// A reservation given back: its buffer was never made.
    pub(crate) fn unreserve(&self, bytes: u64, label: &'static str) {
        self.counts.release(bytes, label);
    }

    /// The bytes charged now.
    #[cfg_attr(not(any(test, target_arch = "wasm32")), allow(dead_code))]
    pub(crate) fn live(&self) -> u64 {
        self.counts.live.load(std::sync::atomic::Ordering::Relaxed)
    }

    /// Starts a watch of the most charged from now on; one at a time, as
    /// a tree's draws are made one after another.
    pub(crate) fn watch(&self) {
        self.counts.watched.store(self.live(), std::sync::atomic::Ordering::Relaxed);
    }

    /// The most charged since the watch began.
    pub(crate) fn watched(&self) -> u64 {
        self.counts.watched.load(std::sync::atomic::Ordering::Relaxed)
    }

    /// A submission made.
    pub(crate) fn submitted(&self) {
        #[cfg(test)]
        self.submits.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    }

    /// The submissions made on the device.
    #[cfg(test)]
    pub(crate) fn submits(&self) -> u64 {
        self.submits.load(std::sync::atomic::Ordering::Relaxed)
    }

    /// The most they held since the last call, which starts the next span
    /// at what they hold now.
    #[cfg(test)]
    pub(crate) fn take_peak(&self) -> u64 {
        let live = self.live();
        self.counts
            .peak
            .swap(live, std::sync::atomic::Ordering::Relaxed)
            .max(live)
    }

    /// The bytes each label held at the peak, the largest first, and the
    /// span restarted as `take_peak` restarts it.
    #[cfg(test)]
    pub(crate) fn take_peak_census(&self) -> (u64, Vec<(&'static str, u64)>) {
        let mut l = self.counts.labels.lock().unwrap_or_else(|e| e.into_inner());
        let mut at: Vec<_> = l.at_peak.iter().map(|(k, v)| (*k, *v)).filter(|e| e.1 > 0).collect();
        at.sort_by_key(|e| std::cmp::Reverse(e.1));
        l.at_peak = l.now.clone();
        drop(l);
        (self.take_peak(), at)
    }

    /// The bytes each label holds now, the largest first.
    #[cfg(test)]
    pub(crate) fn census_now(&self) -> Vec<(&'static str, u64)> {
        let l = self.counts.labels.lock().unwrap_or_else(|e| e.into_inner());
        let mut now: Vec<_> = l.now.iter().map(|(k, v)| (*k, *v)).filter(|e| e.1 > 0).collect();
        now.sort_by_key(|e| std::cmp::Reverse(e.1));
        now
    }
}

impl Owned {
    /// `buffer` owned, its bytes charged now: a buffer made outside the
    /// budget (`io::buffer` reserves before it makes).
    pub(crate) fn new(gpu: &Gpu, buffer: wgpu::Buffer) -> Self {
        let size = buffer.size();
        gpu.ledger.counts.charge(size, "unbudgeted", 0).expect("no cap");
        Self { buffer, ledger: gpu.ledger.clone(), label: "unbudgeted", gone: Default::default() }
    }

    pub(crate) fn create(gpu: &Gpu, descriptor: &wgpu::BufferDescriptor<'_>) -> Self {
        Self::new(gpu, gpu.device.create_buffer(descriptor))
    }

    /// `buffer` owned under `label`, its bytes reserved before it was made.
    pub(crate) fn reserved(gpu: &Gpu, buffer: wgpu::Buffer, label: &'static str) -> Self {
        Self { buffer, ledger: gpu.ledger.clone(), label, gone: Default::default() }
    }
}

/// A buffer a pass binds, its own or one another owner lends it.
pub(crate) enum Lent<'a> {
    Own(Owned),
    Borrowed(&'a wgpu::Buffer),
}

impl std::ops::Deref for Lent<'_> {
    type Target = wgpu::Buffer;
    fn deref(&self) -> &wgpu::Buffer {
        match self {
            Lent::Own(b) => b,
            Lent::Borrowed(b) => b,
        }
    }
}

impl std::ops::Deref for Owned {
    type Target = wgpu::Buffer;
    fn deref(&self) -> &wgpu::Buffer {
        &self.buffer
    }
}

impl Owned {
    /// The buffer let go ahead of its owner, once, as a drop lets it go:
    /// destroyed once no open encoder may bind it, its bytes charged until
    /// the device is done with it. It shadows `wgpu::Buffer::destroy`,
    /// which freed the memory and left the ledger charging it until the
    /// owner dropped (fn-270 task 6, `grow_in_peak`).
    pub(crate) fn destroy(&self) {
        if !self.gone.swap(true, std::sync::atomic::Ordering::Relaxed) {
            self.ledger.let_go(&self.buffer, self.label);
        }
    }
}

impl Drop for Owned {
    fn drop(&mut self) {
        self.destroy();
    }
}

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
    buffer: Owned,
    region: Region,
    usage: wgpu::BufferUsages,
    label: &'static str,
}

impl Held {
    pub(crate) fn resident(buffer: Owned, used: u64, label: &'static str) -> Self {
        let capacity = buffer.size();
        let usage = buffer.usage();
        Self {
            buffer,
            region: Region { capacity, used },
            usage,
            label,
        }
    }

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
) -> Owned {
    let buffer = Owned::create(
        gpu,
        &wgpu::BufferDescriptor {
            label: Some(label),
            size: capacity,
            usage: usage | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        },
    );
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
    let readback = Owned::create(
        gpu,
        &wgpu::BufferDescriptor {
            label: Some("readback"),
            size,
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            mapped_at_creation: false,
        },
    );
    let mut encoder = gpu.encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("readback"),
    });
    encoder.copy_buffer_to_buffer(held.buffer(), 0, &readback, 0, size);
    gpu.submit([encoder.finish()]);

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
