//! The GPU executor's waves (fn-209 F1, tasks 2, 5 and 6): a wave's
//! chains grown from their buds in runs of a batch, recorded and packed.
//! The overflow rule is the chain: a chain that outgrows its region is
//! queued and grown again in one eight times larger. Per run only the
//! scan's totals cross to the host, to size the next buffers (host
//! decision 6); nothing per chain or per node does.
use super::chains::{packed, upload, Caps, Grower, Packed, Uploaded, AXW, PW};
use super::laid::{self, Resolver};
use super::pool::Pool;
use super::shedding::Fades;
use super::steps::{buffer, Steps, BW2, IW, RW};
use crate::buffer::Owned;
use crate::{Gpu, Result};
use std::sync::Arc;
use telperion_space::{ChainTables, GeometryTables};

/// A chain's first region: most chains are a few axes, so a small region
/// and a large batch; one that outgrows it is grown again in a region
/// eight times larger, in batches an eighth the size.
const CAPS: Caps = Caps {
    axes: 32,
    phyts: 64,
    live: 8,
    sleepers: 16,
    woken: 8,
    units: 64,
};

/// What expanding a tree's roots took: waves, chains grown (again ones
/// included) and runs, milliseconds growing and recording (to each run's
/// totals) and laying, the chains grown again, and the round trips: each
/// a few totals read back.
#[derive(Debug, Default, Clone, Copy)]
pub(super) struct Cost {
    pub waves: usize,
    pub chains: usize,
    pub runs: usize,
    pub grow: f64,
    pub lay: f64,
    /// Under realized mechanics: phase 1's fades and its report.
    pub fade: f64,
    pub again: usize,
    pub trips: usize,
    /// Under realized mechanics (task 9): phase 1's milliseconds growing
    /// the groups sown while the structure above the cut grew, and in
    /// `grow` itself, the rest's.
    #[cfg_attr(target_arch = "wasm32", allow(dead_code))]
    pub sown: f64,
    pub last: f64,
}

impl Cost {
    /// `other`'s counts added to these.
    pub(super) fn add(&mut self, other: &Cost) {
        self.waves += other.waves;
        self.chains += other.chains;
        self.runs += other.runs;
        self.again += other.again;
        self.trips += other.trips;
    }
}

/// A species point's tables, packed and on the device.
pub(super) struct Tables {
    pub chain: Arc<ChainTables>,
    pub packed: Packed,
    pub uploaded: Uploaded,
    pub table: Owned,
    pub sags: bool,
    /// Each PA's divergence, which a bearer's phase reads (`bearer_frame`).
    pub divergence: Vec<f64>,
}

impl Tables {
    pub(super) fn new(gpu: &Gpu, chain: Arc<ChainTables>, geo: &GeometryTables) -> Result<Self> {
        let (table, sags) = laid::table(geo);
        let usage = wgpu::BufferUsages::STORAGE;
        let bytes = table.len() as u64 * 4;
        let data = bytemuck::cast_slice(&table);
        let table = super::super::io::buffer(gpu, "laid table", bytes, usage, data)?;
        let packed = packed(&chain, geo);
        Ok(Self {
            chain,
            uploaded: upload(gpu, &packed)?,
            packed,
            table,
            sags,
            divergence: geo.pas.iter().map(|p| p.divergence).collect(),
        })
    }
}

/// The executor's kernels.
pub(super) struct Kernels {
    pub grower: Grower,
    pub steps: Steps,
    pub resolver: Resolver,
    pub fades: Fades,
    pub pool: Pool,
    /// Held while the kernels live: their pool keeps a settled tree's
    /// buffers, so whole-tree tests take the device one at a time
    /// (`whole_tree`).
    #[cfg(test)]
    _one: std::sync::MutexGuard<'static, ()>,
}

pub(super) fn kernels(gpu: &Gpu) -> Kernels {
    Kernels {
        grower: Grower::new(gpu),
        steps: Steps::new(gpu),
        resolver: Resolver::new(gpu),
        fades: Fades::new(gpu),
        pool: Pool::default(),
        #[cfg(test)]
        _one: crate::generation::whole_tree(),
    }
}

/// One run as the device keeps it: its chains, their records (places in
/// its packed regions and scratch, counts, wave indices), the regions
/// packed, the geometry scratch and items.
pub(super) struct Run {
    pub n: u32,
    pub runoff: Owned,
    pub axes: Owned,
    pub phyts: Owned,
    pub scratch: Owned,
    pub items: Owned,
}

/// One wave as the device keeps it: its buds' roots and bearers, its
/// chains' places in its laid buffer, and its runs.
pub(super) struct Grown {
    pub meta: Owned,
    pub places: Owned,
    pub runs: Vec<Run>,
}

/// The four totals a run reads back: its packed axes and phytomers, its
/// scratch words, and its chains queued for a larger region.
pub(super) async fn totals(
    gpu: &Gpu,
    buffer: &wgpu::Buffer,
    words: u64,
    cost: &mut Cost,
) -> Result<Vec<u32>> {
    cost.trips += 1;
    let bytes = super::super::io::read_async(gpu, buffer, words * 4).await?;
    Ok(bytemuck::cast_slice(&bytes).to_vec())
}

/// Grows a wave's `n` buds in runs of `batch`, each chain that outgrew its
/// region again in one eight times larger, each run recorded and packed;
/// each chain's counts into `places`.
pub(super) async fn grow_wave(
    gpu: &Gpu,
    k: &Kernels,
    (buds, n, batch): (&wgpu::Buffer, u32, usize),
    (places, tables): (&wgpu::Buffer, &Tables),
    cost: &mut Cost,
) -> Result<Vec<Run>> {
    let mut runs = Vec::new();
    // A level's work: its buds (none: the wave's own), the wave indices
    // they stand for, and their count.
    let mut work: Vec<(Option<Owned>, Owned, u32)> = vec![(None, buffer(gpu, "no list", 1)?, n)];
    let (mut caps, mut size) = first_region(n, batch as u32);
    let mut level = 0u32;
    loop {
        let mut queues = Vec::new();
        for (gathered, list, count) in &work {
            let input = gathered.as_deref().unwrap_or(buds);
            for first in (0..*count).step_by(size as usize) {
                let m = size.min(count - first);
                let at = (input, &**list, first, m, level);
                let (run, queued) = grow_run(gpu, k, at, (places, tables), caps, cost).await?;
                cost.runs += 1;
                cost.chains += m as usize;
                runs.push(run);
                queues.extend(queued);
            }
        }
        if queues.is_empty() {
            return Ok(runs);
        }
        let mut encoder = gpu.encoder(&Default::default());
        work = Vec::with_capacity(queues.len());
        for (queue, count) in queues {
            cost.again += count as usize;
            let gathered = buffer(gpu, "gathered buds", u64::from(count * BW2))?;
            k.steps
                .gather(gpu, &mut encoder, count, (&queue, buds, &gathered))?;
            work.push((Some(gathered), queue, count));
        }
        gpu.submit([encoder.finish()]);
        (caps, size, level) = (caps.times(8), (size / 8).max(64), level + 1);
        if caps.axes >= 1 << 16 {
            return Err(telperion_core::Error::ResourceLimit("a chain no region holds").into());
        }
    }
}

/// A wave of `n` chains' first regions and run size: a wave a fraction
/// of a batch takes regions as many times larger as fit in the batch's
/// room, so its long chains are not grown twice, once to overflow and
/// once in a larger region (task 8: the beech's level-1 runs, a few
/// chains each, took 2 to 5 ms a wave). A chain grows the same in any
/// region that holds it.
fn first_region(n: u32, batch: u32) -> (Caps, u32) {
    let (mut caps, mut size) = (CAPS, batch);
    while n <= size / 8 && size / 8 >= 64 {
        (caps, size) = (caps.times(8), size / 8);
    }
    (caps, size)
}

/// One run: `m` chains from `first` of `input` grown in regions of
/// `caps`, recorded, scanned and packed; the chains it queued for a larger
/// region, if any. One round trip: its four totals.
async fn grow_run(
    gpu: &Gpu,
    k: &Kernels,
    (input, list, first, m, level): (&wgpu::Buffer, &wgpu::Buffer, u32, u32, u32),
    (places, tables): (&wgpu::Buffer, &Tables),
    caps: Caps,
    cost: &mut Cost,
) -> Result<(Run, Option<(Owned, u32)>)> {
    let t = tables;
    let mut encoder = gpu.encoder(&Default::default());
    let at = (&t.uploaded, &t.packed, t.chain.span, t.chain.age);
    let region = k
        .grower
        .region(gpu, &mut encoder, (input, first, m), at, caps)?;
    let runoff = buffer(gpu, "run records", u64::from(m * RW))?;
    let queue = buffer(gpu, "run queue", u64::from(m))?;
    let queued = buffer(gpu, "run queued", 1)?;
    let sums = buffer(gpu, "run totals", 4)?;
    k.steps.record(
        gpu,
        &mut encoder,
        (m, first, level, caps.axes),
        (&region.status, &region.axes, list),
        (places, &queue, &queued, &runoff),
    )?;
    k.steps
        .scan(gpu, &mut encoder, (&runoff, m, RW, 3), &sums)?;
    encoder.copy_buffer_to_buffer(&queued, 0, &sums, 12, 4);
    gpu.submit([encoder.finish()]);
    let t = totals(gpu, &sums, 4, cost).await?;
    let axes = k
        .pool
        .take(gpu, "packed axes", u64::from(t[0]) * AXW as u64)?;
    let phyts = k
        .pool
        .take(gpu, "packed phytomers", u64::from(t[1]) * PW as u64)?;
    let scratch = k.pool.take(gpu, "laid scratch", u64::from(t[2]))?;
    let items = buffer(gpu, "laid items", u64::from(m * IW))?;
    let mut encoder = gpu.encoder(&Default::default());
    let packed = (&*axes, &*phyts);
    k.grower.pack(
        gpu,
        &mut encoder,
        (&region, m, caps),
        (&*runoff, RW),
        packed,
    )?;
    gpu.submit([encoder.finish()]);
    let run = Run {
        n: m,
        runoff,
        axes,
        phyts,
        scratch,
        items,
    };
    Ok((run, (t[3] > 0).then_some((queue, t[3]))))
}
