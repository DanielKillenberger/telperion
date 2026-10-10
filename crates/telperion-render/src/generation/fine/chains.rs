//! The chain grower's host side for its tests (fn-209 F1, task 2): the
//! species point's tables and the chains' buds as `chain.wgsl` reads them,
//! a dispatch of chains in regions, and their axes packed and read back
//! (`pack.wgsl`) as `telperion_space::chain_cases` gives the CPU's.
use super::super::io;
use super::units::{f, pack};
use crate::buffer::Owned;
use crate::{Gpu, Result};
use std::cell::RefCell;
#[cfg(test)]
use telperion_space::ChainAxis;
use telperion_space::{ChainBud, ChainTables, GeometryTables, Woke};

const CHAIN: &str = include_str!("chain.wgsl");
const APEX: &str = include_str!("apex.wgsl");
const SLEEP: &str = include_str!("sleep.wgsl");
const SHED: &str = include_str!("shed.wgsl");
const PACK: &str = include_str!("pack.wgsl");
const PRESENCE: &str = include_str!("presence.wgsl");
const ASSIGN: &str = include_str!("assign.wgsl");
const CARRY: &str = include_str!("carry.wgsl");
pub(super) const AXW: usize = 80;
pub(super) const PW: usize = 11;
const NONE: u32 = u32::MAX;
/// Threads a chain's asks are shared among (`assign.wgsl`'s `LANES`).
const ASK_LANES: u32 = 8;

/// A region's bounds per chain: axes, phytomers, living apexes, sleeping
/// buds and the buds that woke.
#[derive(Debug, Clone, Copy)]
pub(super) struct Caps {
    pub axes: u32,
    pub phyts: u32,
    pub live: u32,
    pub sleepers: u32,
    pub woken: u32,
    pub units: u32,
}

impl Caps {
    pub(super) fn times(self, k: u32) -> Self {
        Self {
            axes: self.axes * k,
            phyts: self.phyts * k,
            live: self.live * k,
            sleepers: self.sleepers * k,
            woken: self.woken * k,
            units: self.units * k,
        }
    }
}

/// A sleeping bud's and a woken bud's words (`chain.wgsl`).
const SW: u32 = 15;
const QW: u32 = 21;
const UW: u32 = 28;

/// The unit tables, then each PA's row (flags, lived, successor, words at)
/// and its words (`chain.wgsl`); the rows' offset in the table.
pub(super) fn packed(t: &ChainTables, g: &GeometryTables) -> Packed {
    let (mut table, mut words) = pack(&t.units);
    let rows = table.len() as u32;
    let (mut woods, mut reaches) = (Vec::new(), Vec::new());
    for (pa, r) in t.pas.iter().enumerate() {
        let at = words.len() as u32;
        let flags = u32::from(r.lives)
            | u32::from(r.abortion.is_some()) << 1
            | u32::from(r.abortion.is_some_and(|a| a.2.is_some())) << 2
            | u32::from(r.successor.is_some()) << 3
            | u32::from(r.delay.is_some()) << 4
            | u32::from(r.lasting.is_some()) << 5;
        let (a_thr, a_log, rise) = r.abortion.unwrap_or((0, 0.0, None));
        let (next, go, chance) = r
            .successor
            .map_or((NONE, 0, 0.0), |(n, g, q)| (n as u32, g, q));
        table.extend([flags, r.lived.map_or(NONE, |l| l as u32), next, at]);
        // Thresholds and ticks whole, every other value an f32 (`fr`).
        words.extend([
            r.life,
            r.viability.0,
            f(r.viability.1),
            a_thr,
            f(a_log),
            f(rise.unwrap_or(0.0)),
            r.relay[0],
            r.relay[1],
            r.relay[2],
            go,
            f(chance),
            f(r.moved.0),
            f(r.moved.1),
            f(r.delay.unwrap_or(0.0)),
            f(r.lasting.unwrap_or(0.0)),
            0, // the log woods' offset, set below
            f(r.relay_at),
            0,
        ]);
        // Host decision 8's presences (`assign.wgsl`), from word 18.
        for (rate, floor) in r.stakes {
            words.extend([f(rate), f(floor)]);
        }
        words.extend([f(r.living.0), f(r.living.1), f(r.fade_delay), 0]);
        woods.push((at as usize + 15, pa));
        reaches.push((at as usize + 27, pa));
    }
    for (slot, pa) in reaches {
        words[slot] = words.len() as u64;
        words.extend(t.pas[pa].reaches.iter().map(|&r| f(r)));
    }
    for (slot, pa) in woods {
        words[slot] = words.len() as u64;
        words.extend(t.log_woods[pa].iter().map(|&l| f(l)));
    }
    let geo = words.len() as u32;
    words.extend(super::laid::presences(g));
    (table, words, [rows, geo])
}

/// The tables as words, and where the PA rows and the presences' values
/// start.
pub(super) type Packed = (Vec<u32>, Vec<u64>, [u32; 2]);

/// A bud's words: its record, its vigour as made and its node's buds a
/// whorl, and its making draws' presences (host decision 8), each f32 in
/// a word's low half.
pub(super) fn bud_words(b: &ChainBud, made: [f32; 2], birth: [f32; 2]) -> [u64; 14] {
    let pair = |lo: u32, hi: u32| u64::from(lo) | u64::from(hi) << 32;
    let w = b.woke.unwrap_or(Woke {
        key: 0,
        pa: 0,
        sleep: 0,
        stage: 0,
        spent: 0,
        cycle: 0,
    });
    [
        b.lineage,
        pair(b.pa as u32, b.birth),
        pair(u32::from(b.seed), b.node as u32),
        b.root,
        f(b.horizon),
        pair(
            u32::from(b.woke.is_some()) | u32::from(b.context.is_some()) << 1,
            u32::from(b.slot),
        ),
        w.key,
        pair(w.pa as u32, w.stage as u32),
        w.spent,
        pair(w.sleep as u32, w.cycle),
        f(b.context.unwrap_or(0.0)),
        pair(made[0].to_bits(), made[1].to_bits()),
        f(birth[0]),
        f(birth[1]),
    ]
}

/// The chain grower's WGSL.
pub(super) fn source() -> String {
    format!(
        "{}\n{CHAIN}\n{PRESENCE}\n{ASSIGN}\n{CARRY}\n{APEX}\n{SLEEP}\n{SHED}",
        super::zones()
    )
}

/// The packing pass's WGSL.
pub(super) fn pack_source() -> String {
    PACK.to_string()
}

/// The tables on the device.
pub(super) struct Uploaded {
    pub table: Owned,
    pub words: Owned,
}

pub(super) fn upload(gpu: &Gpu, t: &Packed) -> Result<Uploaded> {
    let usage = wgpu::BufferUsages::STORAGE;
    let b = |label, bytes: u64, data: &[u8]| io::buffer(gpu, label, bytes, usage, data);
    Ok(Uploaded {
        table: b(
            "chain table",
            t.0.len() as u64 * 4,
            bytemuck::cast_slice(&t.0),
        )?,
        words: b(
            "chain words",
            t.1.len() as u64 * 8,
            bytemuck::cast_slice(&t.1),
        )?,
    })
}

/// A run's regions on the device, as the grower left them: handles on
/// the grower's pool, which owns them.
pub(super) struct Region {
    pub axes: wgpu::Buffer,
    pub phyts: wgpu::Buffer,
    pub status: wgpu::Buffer,
    #[cfg_attr(not(test), allow(dead_code))]
    pub row: u32,
    #[cfg_attr(not(test), allow(dead_code))]
    pub groups: u32,
}

pub(super) struct Grower {
    pub(super) pass: io::Pass,
    pub(super) pack: io::Pass,
    /// The regions' buffers, kept from run to run: every shader writes a
    /// record before it reads it, and the queue orders a run's growth
    /// after the last run's packing, so a run takes the buffers the last
    /// one left (task 8: a fresh 1.4 GB a run cost 30 ms of allocation and
    /// clearing, three times the run's kernels).
    pool: RefCell<Vec<Owned>>,
}

/// The regions' labels.
const LABELS: [&str; 5] = [
    "chain axes",
    "chain phytomers",
    "chain apexes",
    "chain status",
    "chain sleeping buds",
];

impl Grower {
    pub(super) fn new(gpu: &Gpu) -> Self {
        let read = [true, true, true, false, false, false, false, false];
        Self {
            pass: io::Pass::new(gpu, source(), &read, &["grow", "asks", "assign"]),
            pack: io::Pass::new(
                gpu,
                pack_source(),
                &[true, true, true, true, false, false],
                &["pack"],
            ),
            pool: RefCell::new(Vec::new()),
        }
    }

    /// The regions let go once a build is done: growth scratch no frame
    /// reads (fn-258 R2). The next build makes them again.
    pub(super) fn release(&self) {
        self.pool.borrow_mut().clear();
    }

    /// The region's five buffers of at least `bytes` each, reused where
    /// the pool's are large enough.
    fn buffers(&self, gpu: &Gpu, bytes: [u64; 5]) -> Result<[wgpu::Buffer; 5]> {
        // Each buffer only grows: a run's regions are as many chains of
        // larger caps or more of smaller ones, and its status a word a
        // chain whatever the caps, so replacing all five at once lets two
        // waves trade them back and forth on every build.
        let storage = wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC;
        let mut pool = self.pool.borrow_mut();
        for (k, (&label, n)) in LABELS.iter().zip(bytes).enumerate() {
            if pool.get(k).is_none_or(|b| b.size() < n) {
                let grown = io::buffer(gpu, label, n, storage, &[])?;
                match pool.get_mut(k) {
                    Some(b) => *b = grown,
                    None => pool.push(grown),
                }
            }
        }
        Ok(std::array::from_fn(|k| (*pool[k]).clone()))
    }

    /// Grows the `n` buds from `first` of `input` in regions of `caps`,
    /// recorded into `encoder`, every axis kept (realized mechanics).
    pub(super) fn region(
        &self,
        gpu: &Gpu,
        encoder: &mut wgpu::CommandEncoder,
        (input, first, n): (&wgpu::Buffer, u32, u32),
        (tables, t, span, age): (&Uploaded, &Packed, f32, u32),
        caps: Caps,
    ) -> Result<Region> {
        let groups = n.div_ceil(64);
        let row = groups.min(gpu.device.limits().max_compute_workgroups_per_dimension) * 64;
        // The asks' pass: `assign.wgsl`'s `LANES` threads a chain.
        let ask_groups = (n * ASK_LANES).div_ceil(64);
        let ask_row = ask_groups.min(gpu.device.limits().max_compute_workgroups_per_dimension) * 64;
        let cfg: [u32; 16] = [
            n,
            row,
            caps.axes,
            caps.phyts,
            caps.live,
            t.2[0],
            age,
            span.to_bits(),
            0,
            caps.sleepers,
            caps.woken,
            t.2[1],
            caps.units,
            first,
            ask_row,
            0,
        ];
        let usage = wgpu::BufferUsages::UNIFORM;
        let config = io::buffer(gpu, "chain cfg", 64, usage, bytemuck::cast_slice(&cfg))?;
        let sleep = u64::from(n * (caps.sleepers * SW + caps.woken * QW + caps.units * UW)) * 4;
        let bytes = [
            u64::from(n * caps.axes) * AXW as u64 * 4,
            u64::from(n * caps.phyts) * PW as u64 * 4,
            u64::from(n * caps.live) * 32,
            u64::from(n) * 16,
            sleep,
        ];
        let [axes, phyts, live, status, sleepers] = self.buffers(gpu, bytes)?;
        let all = [
            &config,
            input,
            &tables.table,
            &tables.words,
            &axes,
            &phyts,
            &live,
            &status,
            &sleepers,
        ];
        let bind = self.pass.bind(gpu, &all);
        self.pass.run(encoder, &bind, 0, groups);
        self.pass.run(encoder, &bind, 1, ask_groups);
        self.pass.run(encoder, &bind, 2, groups);
        Ok(Region {
            axes,
            phyts,
            status,
            row,
            groups,
        })
    }

    /// Packs a run's regions to `offsets` (`stride` words a chain) into
    /// `packed`, recorded into `encoder`.
    #[allow(clippy::too_many_arguments)]
    pub(super) fn pack(
        &self,
        gpu: &Gpu,
        encoder: &mut wgpu::CommandEncoder,
        (region, n, caps): (&Region, u32, Caps),
        (offsets, stride): (&wgpu::Buffer, u32),
        (axes, phyts): (&wgpu::Buffer, &wgpu::Buffer),
    ) -> Result<()> {
        // A workgroup a chain, in rows of the most a dimension takes.
        let row = n.min(gpu.device.limits().max_compute_workgroups_per_dimension);
        let cfg = [n, row, caps.axes, caps.phyts, stride, 0, 0, 0];
        let usage = wgpu::BufferUsages::UNIFORM;
        let cfg = io::buffer(gpu, "pack cfg", 32, usage, bytemuck::cast_slice(&cfg))?;
        let all = [
            &cfg,
            &region.status,
            offsets,
            &region.axes,
            &region.phyts,
            axes,
            phyts,
        ];
        let bind = self.pack.bind(gpu, &all);
        self.pack.run(encoder, &bind, 0, n);
        Ok(())
    }
}

/// A chain's axes from its region, as `telperion_space::chain_cases` gives
/// the CPU's: the bearer's stub left out.
#[cfg(test)]
pub(super) fn topology(bud: &ChainBud, axes: &[u32], phyts: &[u32]) -> Vec<ChainAxis> {
    let root = usize::from(!bud.seed);
    let word = |i: usize, f: usize| axes[i * AXW + f];
    let wide = |i: usize, f: usize| u64::from(word(i, f)) | u64::from(word(i, f + 1)) << 32;
    let count = axes.len() / AXW;
    let mut out: Vec<ChainAxis> = (root..count)
        .map(|i| {
            let kind = word(i, 4) & 0xff;
            let parent = word(i, 5);
            let lateral = kind == 1;
            ChainAxis {
                woken: word(i, 4) & 0x80000 != 0,
                parent: (parent != NONE && parent as usize >= root).then(|| parent as usize - root),
                kind: kind as u8,
                node: if lateral || kind == 3 {
                    word(i, 6) as usize
                } else {
                    0
                },
                slot: if lateral {
                    (word(i, 4) >> 8 & 0xff) as u8
                } else {
                    0
                },
                lineage: wide(i, 0),
                pa: word(i, 2) as usize,
                birth: word(i, 3),
                apex_end: word(i, 7).checked_sub(1),
                phytomers: Vec::new(),
                kept: word(i, 28) & 1 != 0,
                context: (word(i, 28) & 2 != 0).then(|| f32::from_bits(word(i, 29))),
            }
        })
        .collect();
    for p in phyts.as_chunks::<PW>().0 {
        let axis = p[3] as usize;
        if axis >= root {
            let key = u64::from(p[0]) | u64::from(p[1]) << 32;
            out[axis - root].phytomers.push((p[2], key));
        }
    }
    out
}
