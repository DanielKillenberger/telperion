//! The cycle's units and wake recorded on the device (DESIGN.md sections
//! 2.1 and 2.2): units counted, scanned and emitted; the cycle's sleeping
//! buds compacted; their waking counted, scanned and emitted. Each
//! dispatch is its own compute pass (`io::Pass::run`), so a profile
//! stamps each; only the scans' totals move, by copies into the tally.
use super::super::io;
use super::columns::{Caps, Columns, SLEEPERS, TALLY, UNITS, WOKEN};
use crate::buffer::Owned;
use crate::{Gpu, Result};

const SCAN: &str = include_str!("../fine/scan.wgsl");

const CYCLE: usize = 0;
const FLAG: usize = 1;
const GATHER: usize = 2;

/// The tally's places (`arena.wgsl`'s `T_` words).
pub(super) const T_UNITS: usize = 0;
pub(super) const T_UNITS_B: usize = 4;
pub(super) const T_LIST: usize = 8;
pub(super) const T_WAKE: usize = 12;
pub(super) const T_WAKE_B: usize = 16;

/// The phases a run's stamped passes fall in, in order, with their passes.
pub(super) const PHASES: [(&str, usize); 7] = [
    ("units count", 1),
    ("units scan", 6),
    ("units emit", 1),
    ("compaction", 5),
    ("wake count", 1),
    ("wake scan", 6),
    ("wake emit", 1),
];

pub(super) struct Kernels {
    cycle: io::Pass,
    scan: io::Pass,
}

impl Kernels {
    pub(super) fn new(gpu: &Gpu) -> Self {
        let mut reads = vec![true; 3];
        reads.extend([false; 5]);
        let entries = ["cycle_main", "flag", "gather"];
        Self {
            cycle: io::Pass::new(gpu, super::source(), &reads, &entries),
            scan: io::Pass::new(
                gpu,
                SCAN.into(),
                &[false; 3],
                &["blocks", "spine", "spread"],
            ),
        }
    }
}

fn storage(gpu: &Gpu, label: &'static str, words: &[u32]) -> Result<Owned> {
    let usage = wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC;
    let bytes = (words.len() as u64 * 4).max(16);
    io::buffer(gpu, label, bytes, usage, bytemuck::cast_slice(words))
}

fn zeroed(gpu: &Gpu, label: &'static str, words: u64) -> Result<Owned> {
    let usage = wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC;
    io::buffer(gpu, label, words.max(4) * 4, usage, &[])
}

/// Workgroups over `n` invocations, and the row a dispatch spreads them in.
fn row(gpu: &Gpu, n: u32) -> (u32, u32) {
    let groups = n.div_ceil(64);
    let most = gpu.device.limits().max_compute_workgroups_per_dimension;
    (groups, groups.min(most).max(1) * 64)
}

/// One scan of `n` records' first `fields` words, its totals copied to
/// the tally's word `to`.
struct Scan {
    bind: wgpu::BindGroup,
    totals: Owned,
    blocks: u32,
    groups: u32,
    to: usize,
    _cfg: Owned,
    _sums: Owned,
}

/// A captured cycle on the device at `caps`: the tables, the state it
/// starts from, kept to start each run again, and the working buffers.
pub(super) struct Run {
    pub caps: Caps,
    pub axes: Owned,
    pub arena: Owned,
    pub lists: Owned,
    start: [Owned; 3],
    /// The units counted and emitted, then the wake: each over its rows.
    binds: Vec<(u32, wgpu::BindGroup)>,
    scans: Vec<Scan>,
    /// The bytes the cycle holds resident (its state, tables and inputs),
    /// and its scratch (the counts and the scans' sums), the captured
    /// state the runs restart from aside.
    pub resident: u64,
    pub scratch: u64,
    _held: Vec<Owned>,
}

impl Run {
    pub(super) fn new(gpu: &Gpu, k: &Kernels, cols: &Columns, caps: Caps) -> Result<Self> {
        let start = [
            storage(gpu, "cycle axes start", &cols.axes_at(caps.axes))?,
            storage(gpu, "cycle arena start", &cols.arena_at(caps))?,
            storage(gpu, "cycle lists start", &cols.lists_at(caps))?,
        ];
        let work = |b: &Owned, label| zeroed(gpu, label, b.size() / 4);
        let (axes, arena, lists) = (
            work(&start[0], "cycle axes")?,
            work(&start[1], "cycle arena")?,
            work(&start[2], "cycle lists")?,
        );
        let table = storage(gpu, "cycle table", &cols.table)?;
        let words = storage(gpu, "cycle words", bytemuck::cast_slice(&cols.words))?;
        let inputs = storage(gpu, "cycle inputs", &cols.inputs)?;
        let n = cols.live.max(cols.sleepers).max(1);
        let counts_a = zeroed(gpu, "cycle counts", u64::from(n) * 4)?;
        let counts_b = zeroed(gpu, "cycle counts b", u64::from(n) * 4)?;
        let mut held = vec![table, words, inputs];
        let mut binds = Vec::new();
        let jobs = [
            (cols.live, 0, 0),
            (cols.live, 1, 0),
            (cols.sleepers, 0, 1),
            (cols.sleepers, 1, 1),
        ];
        for (n, emit, wake) in jobs {
            let words = config(gpu, cols, caps, (n, emit, wake));
            let usage = wgpu::BufferUsages::UNIFORM;
            let c = io::buffer(gpu, "cycle cfg", 128, usage, bytemuck::cast_slice(&words))?;
            let all = [
                &*c, &held[0], &held[1], &held[2], &axes, &arena, &lists, &counts_a, &counts_b,
            ];
            binds.push((n, k.cycle.bind(gpu, &all)));
            held.push(c);
        }
        let mut scans = Vec::new();
        for (data, n, fields, to) in [
            (&counts_a, cols.live, 4, T_UNITS),
            (&counts_b, cols.live, 4, T_UNITS_B),
            (&counts_a, cols.sleepers, 1, T_LIST),
            (&counts_a, cols.sleepers, 4, T_WAKE),
            (&counts_b, cols.sleepers, 4, T_WAKE_B),
        ] {
            scans.push(scan(gpu, k, (data, n, fields), to)?);
        }
        let size = |b: &[&Owned]| b.iter().map(|b| b.size()).sum::<u64>();
        let resident = size(&[&axes, &arena, &lists, &held[0], &held[1], &held[2]]);
        let sums: Vec<&Owned> = scans.iter().flat_map(|s| [&s._sums, &s.totals]).collect();
        let scratch = size(&[&counts_a, &counts_b]) + size(&sums);
        held.extend([counts_a, counts_b]);
        Ok(Self {
            caps,
            axes,
            arena,
            lists,
            start,
            binds,
            scans,
            resident,
            scratch,
            _held: held,
        })
    }

    /// The cycle recorded into `encoder`, from the captured state.
    pub(super) fn record(&self, gpu: &Gpu, k: &Kernels, encoder: &mut wgpu::CommandEncoder) {
        for (from, to) in self
            .start
            .iter()
            .zip([&self.axes, &self.arena, &self.lists])
        {
            encoder.copy_buffer_to_buffer(from, 0, to, 0, from.size());
        }
        let [units_count, units_emit, wake_count, wake_emit] = [0, 1, 2, 3].map(|j| &self.binds[j]);
        let run =
            |e: &mut wgpu::CommandEncoder, entry: usize, (n, bind): &(u32, wgpu::BindGroup)| {
                k.cycle.run(e, bind, entry, row(gpu, *n).0);
            };
        run(encoder, CYCLE, units_count);
        self.scan(k, encoder, &self.scans[0..2]);
        run(encoder, CYCLE, units_emit);
        run(encoder, FLAG, wake_count);
        self.scan(k, encoder, &self.scans[2..3]);
        run(encoder, GATHER, wake_count);
        run(encoder, CYCLE, wake_count);
        self.scan(k, encoder, &self.scans[3..5]);
        run(encoder, CYCLE, wake_emit);
    }

    fn scan(&self, k: &Kernels, encoder: &mut wgpu::CommandEncoder, scans: &[Scan]) {
        for s in scans {
            k.scan.run(encoder, &s.bind, 0, s.blocks.div_ceil(64));
            k.scan.run(encoder, &s.bind, 1, 1);
            k.scan.run(encoder, &s.bind, 2, s.groups);
        }
        for s in scans {
            encoder.copy_buffer_to_buffer(&s.totals, 0, &self.lists, s.to as u64 * 4, 16);
        }
    }
}

/// A scan's pass over `n` records 4 words apart, its first `fields`.
fn scan(gpu: &Gpu, k: &Kernels, (data, n, fields): (&Owned, u32, u32), to: usize) -> Result<Scan> {
    let blocks = n.div_ceil(256);
    let (groups, row) = row(gpu, n);
    let words = [n, 4, fields, blocks, row, 0, 0, 0];
    let cfg = io::buffer(
        gpu,
        "scan cfg",
        32,
        wgpu::BufferUsages::UNIFORM,
        bytemuck::cast_slice(&words),
    )?;
    let sums = zeroed(gpu, "scan sums", u64::from(blocks.max(1)) * 4)?;
    let totals = zeroed(gpu, "scan totals", 4)?;
    let bind = k.scan.bind(gpu, &[&cfg, data, &sums, &totals]);
    Ok(Scan {
        bind,
        totals,
        blocks,
        groups,
        to,
        _cfg: cfg,
        _sums: sums,
    })
}

/// The kernels' uniform (`arena.wgsl`'s `Cfg`) over `n` invocations,
/// emitting or counting, in the units or the wake.
fn config(gpu: &Gpu, cols: &Columns, caps: Caps, (n, emit, wake): (u32, u32, u32)) -> [u32; 32] {
    let phyt_at = UNITS as u32 * caps.units;
    let sleep_at = TALLY as u32;
    let next_at = sleep_at + SLEEPERS as u32 * caps.sleepers;
    let woken_at = next_at + 2 * caps.next;
    let list_at = woken_at + WOKEN as u32 * caps.woken;
    [
        n,
        row(gpu, n).1,
        cols.cycle,
        cols.age,
        emit,
        wake,
        cols.lights,
        0,
        caps.axes,
        caps.units,
        caps.phyts,
        caps.sleepers,
        caps.next,
        caps.woken,
        cols.sleepers.max(1),
        cols.pencils,
        phyt_at,
        sleep_at,
        next_at,
        woken_at,
        list_at,
        cols.axes,
        cols.units,
        0,
        cols.sleepers,
        2 * cols.live,
        cols.rows,
        cols.whorls,
        0,
        0,
        0,
        0,
    ]
}
