//! The GPU executor's waves (fn-209 F1, tasks 2, 5 and 6): a wave's
//! chains grown from their buds in runs of a batch, recorded and packed.
//! The overflow rule is the chain: a chain that outgrows its region is
//! queued and grown again in one eight times larger. Per run only the
//! scan's totals cross to the host, to size the next buffers (host
//! decision 6); nothing per chain or per node does.
use super::chains::{packed, upload, Caps, Grower, Packed, Uploaded, AXW, PW};
use super::laid::{self, Resolver};
use super::shedding::Fades;
use super::steps::{buffer, Steps, BW2, IW, RW};
use crate::Gpu;
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
}

/// A species point's tables, packed and on the device.
pub(super) struct Tables<'a> {
    pub chain: &'a ChainTables,
    pub packed: Packed,
    pub uploaded: Uploaded,
    pub table: wgpu::Buffer,
    pub sags: bool,
}

impl<'a> Tables<'a> {
    pub(super) fn new(gpu: &Gpu, chain: &'a ChainTables, geo: &GeometryTables) -> Self {
        let (table, sags) = laid::table(geo);
        let usage = wgpu::BufferUsages::STORAGE;
        let bytes = table.len() as u64 * 4;
        let data = bytemuck::cast_slice(&table);
        let table = super::super::io::buffer(gpu, "laid table", bytes, usage, data).unwrap();
        let packed = packed(chain, geo);
        Self {
            chain,
            uploaded: upload(gpu, &packed),
            packed,
            table,
            sags,
        }
    }
}

/// The executor's kernels.
pub(super) struct Kernels {
    pub grower: Grower,
    pub steps: Steps,
    pub resolver: Resolver,
    pub fades: Fades,
}

pub(super) fn kernels(gpu: &Gpu) -> Kernels {
    Kernels {
        grower: Grower::new(gpu),
        steps: Steps::new(gpu),
        resolver: Resolver::new(gpu),
        fades: Fades::new(gpu),
    }
}

/// One run as the device keeps it: its chains, their records (places in
/// its packed regions and scratch, counts, wave indices), the regions
/// packed, the geometry scratch and items.
pub(super) struct Run {
    pub n: u32,
    pub runoff: wgpu::Buffer,
    pub axes: wgpu::Buffer,
    pub phyts: wgpu::Buffer,
    pub scratch: wgpu::Buffer,
    pub items: wgpu::Buffer,
}

/// One wave as the device keeps it: its buds' roots and bearers, its
/// chains' places in its laid buffer, and its runs.
pub(super) struct Grown {
    pub meta: wgpu::Buffer,
    pub places: wgpu::Buffer,
    pub runs: Vec<Run>,
}

/// The four totals a run reads back: its packed axes and phytomers, its
/// scratch words, and its chains queued for a larger region.
pub(super) fn totals(gpu: &Gpu, buffer: &wgpu::Buffer, words: u64, cost: &mut Cost) -> Vec<u32> {
    cost.trips += 1;
    let bytes = super::super::io::read(gpu, buffer, words * 4).unwrap();
    bytemuck::cast_slice(&bytes).to_vec()
}

/// Grows a wave's `n` buds in runs of `batch`, each chain that outgrew its
/// region again in one eight times larger, each run recorded and packed;
/// each chain's counts into `places`.
pub(super) fn grow_wave(
    gpu: &Gpu,
    k: &Kernels,
    (buds, n, batch): (&wgpu::Buffer, u32, usize),
    (places, tables): (&wgpu::Buffer, &Tables),
    cost: &mut Cost,
) -> Vec<Run> {
    let mut runs = Vec::new();
    // A level's work: its buds (none: the wave's own), the wave indices
    // they stand for, and their count.
    let mut work: Vec<(Option<wgpu::Buffer>, wgpu::Buffer, u32)> =
        vec![(None, buffer(gpu, "no list", 1), n)];
    let (mut caps, mut size) = first_region(n, batch as u32);
    let mut level = 0u32;
    loop {
        let mut queues = Vec::new();
        for (gathered, list, count) in &work {
            let input = gathered.as_ref().unwrap_or(buds);
            for first in (0..*count).step_by(size as usize) {
                let m = size.min(count - first);
                let at = (input, list, first, m, level);
                let (run, queued) = grow_run(gpu, k, at, (places, tables), caps, cost);
                cost.runs += 1;
                cost.chains += m as usize;
                runs.push(run);
                queues.extend(queued);
            }
        }
        if queues.is_empty() {
            return runs;
        }
        let mut encoder = gpu.device.create_command_encoder(&Default::default());
        work = queues
            .into_iter()
            .map(|(queue, count)| {
                cost.again += count as usize;
                let gathered = buffer(gpu, "gathered buds", u64::from(count * BW2));
                k.steps
                    .gather(gpu, &mut encoder, count, (&queue, buds, &gathered));
                (Some(gathered), queue, count)
            })
            .collect();
        gpu.queue.submit([encoder.finish()]);
        (caps, size, level) = (caps.times(8), (size / 8).max(64), level + 1);
        assert!(caps.axes < 1 << 16, "a chain no region holds");
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
fn grow_run(
    gpu: &Gpu,
    k: &Kernels,
    (input, list, first, m, level): (&wgpu::Buffer, &wgpu::Buffer, u32, u32, u32),
    (places, tables): (&wgpu::Buffer, &Tables),
    caps: Caps,
    cost: &mut Cost,
) -> (Run, Option<(wgpu::Buffer, u32)>) {
    let t = tables;
    let mut encoder = gpu.device.create_command_encoder(&Default::default());
    let at = (&t.uploaded, &t.packed, t.chain.span, t.chain.age);
    let region = k
        .grower
        .region(gpu, &mut encoder, (input, first, m), at, caps);
    let runoff = buffer(gpu, "run records", u64::from(m * RW));
    let queue = buffer(gpu, "run queue", u64::from(m));
    let queued = buffer(gpu, "run queued", 1);
    let sums = buffer(gpu, "run totals", 4);
    k.steps.record(
        gpu,
        &mut encoder,
        (m, first, level, caps.axes),
        (&region.status, &region.axes, list),
        (places, &queue, &queued, &runoff),
    );
    k.steps.scan(gpu, &mut encoder, (&runoff, m, RW, 3), &sums);
    encoder.copy_buffer_to_buffer(&queued, 0, &sums, 12, 4);
    gpu.queue.submit([encoder.finish()]);
    let t = totals(gpu, &sums, 4, cost);
    let axes = buffer(gpu, "packed axes", u64::from(t[0]) * AXW as u64);
    let phyts = buffer(gpu, "packed phytomers", u64::from(t[1]) * PW as u64);
    let scratch = buffer(gpu, "laid scratch", u64::from(t[2]));
    let items = buffer(gpu, "laid items", u64::from(m * IW));
    let mut encoder = gpu.device.create_command_encoder(&Default::default());
    let packed = (&axes, &phyts);
    k.grower
        .pack(gpu, &mut encoder, (&region, m, caps), (&runoff, RW), packed);
    gpu.queue.submit([encoder.finish()]);
    let run = Run {
        n: m,
        runoff,
        axes,
        phyts,
        scratch,
        items,
    };
    (run, (t[3] > 0).then_some((queue, t[3])))
}
