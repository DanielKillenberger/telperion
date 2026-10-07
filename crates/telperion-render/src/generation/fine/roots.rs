//! Fine roots expanded on the GPU, wave by wave, on the device (fn-209
//! F1, tasks 2, 4 and 5): each wave's chains grown from their buds in runs
//! of a batch, recorded, packed and laid, and the buds their kept axes
//! bear written as the next wave's, until none is left. The overflow rule
//! is the chain: a chain that outgrows its region is queued and grown
//! again in one eight times larger. Per run and per wave only the scans'
//! totals cross to the host, to size the next buffers (host decision 6);
//! nothing per chain or per node does.
use super::chains::{bud_words, packed, upload, Caps, Grower, Packed, Uploaded, AXW, PW};
use super::laid::{self, Resolver, Wave};
use super::shedding::Fades;
use super::steps::{buffer, Steps, BW2, CW, DESCRIBE, HAND, IW, RW};
use crate::Gpu;
use std::time::Instant;
use telperion_space::{ChainBud, ChainTables, GeometryTables, RootGeometry};

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
    /// Realized mechanics: every axis kept by the chain grower.
    pub keep: bool,
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
            keep: false,
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

/// The roots expanded on the device: the laid buffers (the first the
/// roots' attachments) and, for the tests, every wave as kept.
pub(super) struct Expanded {
    pub waves: Vec<Wave>,
    pub grown: Vec<Grown>,
}

/// The four totals a run reads back: its packed axes and phytomers, its
/// scratch words, and its chains queued for a larger region.
pub(super) fn totals(gpu: &Gpu, buffer: &wgpu::Buffer, words: u64, cost: &mut Cost) -> Vec<u32> {
    cost.trips += 1;
    let bytes = super::super::io::read(gpu, buffer, words * 4).unwrap();
    bytemuck::cast_slice(&bytes).to_vec()
}

/// Expands every root of `buds` (none that can reach the ground, host
/// decision 3), taken in `order`, `batch` chains a run.
pub(super) fn expand(
    (gpu, k): (&Gpu, &Kernels),
    (buds, attached, order, batch): (&[ChainBud], &[RootGeometry], &[usize], usize),
    tables: &Tables,
) -> (Expanded, Cost) {
    let mut cost = Cost::default();
    let (stubs, origins) = laid::stubs(attached);
    let usage = wgpu::BufferUsages::STORAGE;
    let data = bytemuck::cast_slice(&origins);
    let bytes = origins.len().max(4) as u64 * 4;
    let origins = super::super::io::buffer(gpu, "laid origins", bytes, usage, data).unwrap();
    let roots = buds.len() as u32;
    let mut waves = vec![Wave::new(gpu, (roots, roots), &stubs)];
    // The first wave's buds are the roots' records, uploaded.
    let first: Vec<usize> = order
        .iter()
        .copied()
        .filter(|&r| !attached[r].grounded)
        .collect();
    let words: Vec<u64> = first
        .iter()
        .flat_map(|&r| {
            bud_words(
                &buds[r],
                [attached[r].vigour as f32, attached[r].whorl as f32],
                [1 << 32; 2],
            )
        })
        .collect();
    let meta: Vec<[u32; 2]> = first.iter().map(|&r| [r as u32, r as u32]).collect();
    let storage = wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC;
    let up = |label, data: &[u8]| {
        let bytes = data.len().max(16) as u64;
        super::super::io::buffer(gpu, label, bytes, storage, data).unwrap()
    };
    let mut wave_buds = up("wave buds", bytemuck::cast_slice(&words));
    let mut wave_meta = up("wave meta", bytemuck::cast_slice(&meta));
    let mut n = first.len() as u32;
    let mut grown = Vec::new();
    while n > 0 {
        cost.waves += 1;
        let start = Instant::now();
        let places = buffer(gpu, "wave places", u64::from(n * CW));
        let runs = grow_wave(gpu, k, (&wave_buds, n, batch), (&places, tables), &mut cost);
        // The wave's places, and its laid buffer and next buds sized.
        let mut encoder = gpu.device.create_command_encoder(&Default::default());
        let sums = buffer(gpu, "wave totals", 4);
        k.steps.scan(gpu, &mut encoder, (&places, n, CW, CW), &sums);
        gpu.queue.submit([encoder.finish()]);
        let t = totals(gpu, &sums, 3, &mut cost);
        cost.grow += start.elapsed().as_secs_f64() * 1e3;
        let start = Instant::now();
        let out = Wave::new(gpu, (t[0], t[1]), &[]);
        let next_buds = buffer(gpu, "wave buds", u64::from(t[2] * BW2));
        let next_meta = buffer(gpu, "wave meta", u64::from(t[2] * 2));
        let stub = waves.len() == 1;
        let mut encoder = gpu.device.create_command_encoder(&Default::default());
        for run in &runs {
            let lay = (&run.runoff, &places, &wave_meta, &run.items);
            let hand = (&run.axes, &run.scratch, &next_buds, &next_meta);
            k.steps
                .hand(gpu, &mut encoder, (run.n, stub, DESCRIBE), lay, hand);
            k.resolver.dispatch(
                gpu,
                &mut encoder,
                (&run.items, run.n, (&run.axes, &run.phyts), &run.scratch),
                (&tables.table, &origins),
                (waves.last().expect("a wave"), &out),
                (tables.chain.age, tables.sags),
            );
            k.steps
                .hand(gpu, &mut encoder, (run.n, stub, HAND), lay, hand);
        }
        gpu.queue.submit([encoder.finish()]);
        gpu.device
            .poll(wgpu::PollType::wait_indefinitely())
            .unwrap();
        cost.lay += start.elapsed().as_secs_f64() * 1e3;
        waves.push(out);
        grown.push(Grown {
            meta: wave_meta,
            places,
            runs,
        });
        (wave_buds, wave_meta, n) = (next_buds, next_meta, t[2]);
    }
    (Expanded { waves, grown }, cost)
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
    let (mut caps, mut size, mut level) = (CAPS, batch as u32, 0u32);
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
    let at = (&t.uploaded, &t.packed, t.chain.span, t.chain.age, t.keep);
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
