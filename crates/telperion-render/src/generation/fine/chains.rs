//! The chain grower's host side for its tests (fn-209 F1, task 2): the
//! species point's tables and the chains' buds as `chain.wgsl` reads them,
//! a dispatch of chains in regions, and their axes packed and read back
//! (`pack.wgsl`) as `telperion_space::chain_cases` gives the CPU's.
use super::super::io;
use super::units::pack;
use crate::Gpu;
use telperion_space::{ChainAxis, ChainBud, ChainTables, GeometryTables, Woke};

const CHAIN: &str = include_str!("chain.wgsl");
const APEX: &str = include_str!("apex.wgsl");
const SLEEP: &str = include_str!("sleep.wgsl");
const SHED: &str = include_str!("shed.wgsl");
const PACK: &str = include_str!("pack.wgsl");
const PRESENCE: &str = include_str!("presence.wgsl");
pub(super) const AXW: usize = 40;
pub(super) const PW: usize = 9;
const NONE: u32 = u32::MAX;

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
const SW: u32 = 13;
const QW: u32 = 17;
const UW: u32 = 4;

/// The unit tables, then each PA's row (flags, lived, successor, words at)
/// and its words (`chain.wgsl`); the rows' offset in the table.
pub(super) fn packed(t: &ChainTables, g: &GeometryTables) -> Packed {
    let (mut table, mut words) = pack(&t.units);
    let rows = table.len() as u32;
    let mut woods = Vec::new();
    for (pa, r) in t.pas.iter().enumerate() {
        let at = words.len() as u32;
        let flags = u32::from(r.lives)
            | u32::from(r.abortion.is_some()) << 1
            | u32::from(r.abortion.is_some_and(|a| a.2.is_some())) << 2
            | u32::from(r.successor.is_some()) << 3
            | u32::from(r.delay.is_some()) << 4
            | u32::from(r.lasting.is_some()) << 5
            | u32::from(r.shed_at.is_some()) << 6;
        let (a_thr, a_log, rise) = r.abortion.unwrap_or((0, 0, None));
        let (next, go, go62) = r
            .successor
            .map_or((NONE, 0, 0), |(n, g, q)| (n as u32, g, q));
        table.extend([flags, r.lived.map_or(NONE, |l| l as u32), next, at]);
        words.extend([
            r.life,
            r.viability.0,
            r.viability.1 as u64,
            a_thr,
            a_log as u64,
            rise.unwrap_or(0) as u64,
            r.relay[0],
            r.relay[1],
            r.relay[2],
            go,
            go62,
            r.moved.0 as u64,
            r.moved.1 as u64,
            r.delay.unwrap_or(0) as u64,
            r.lasting.unwrap_or(0) as u64,
            0, // the log woods' offset, set below
            r.relay_at,
            r.shed_at.unwrap_or(0) as u64,
        ]);
        woods.push((at as usize + 15, pa));
    }
    for (slot, pa) in woods {
        words[slot] = words.len() as u64;
        words.extend(t.log_woods[pa].iter().map(|&l| l as u64));
    }
    // The extinction curves: each one's start in the words and length.
    let curves = table.len() as u32;
    let dead = words.len() as u32;
    words.extend(t.dead.iter().map(|&d| d as u64));
    for &(at, len) in &t.curves {
        table.extend([dead + at, len]);
    }
    let geo = words.len() as u32;
    words.extend(super::laid::presences(g));
    (table, words, [rows, curves, geo])
}

/// The tables as words, and where the PA rows, the curves and the
/// presences' values start.
pub(super) type Packed = (Vec<u32>, Vec<u64>, [u32; 3]);

pub(super) fn bud_words(b: &ChainBud, made: [f32; 2]) -> [u64; 12] {
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
        b.horizon as u64,
        pair(
            u32::from(b.woke.is_some()) | u32::from(b.context.is_some()) << 1,
            u32::from(b.slot),
        ),
        w.key,
        pair(w.pa as u32, w.stage as u32),
        w.spent,
        pair(w.sleep as u32, w.cycle),
        b.context.unwrap_or(0) as u64,
        pair(made[0].to_bits(), made[1].to_bits()),
    ]
}

/// The chain grower's WGSL.
pub(super) fn source() -> String {
    format!(
        "{}\n{CHAIN}\n{PRESENCE}\n{APEX}\n{SLEEP}\n{SHED}",
        super::zones()
    )
}

/// The packing pass's WGSL.
pub(super) fn pack_source() -> String {
    PACK.to_string()
}

/// The tables on the device.
pub(super) struct Uploaded {
    pub table: wgpu::Buffer,
    pub words: wgpu::Buffer,
}

pub(super) fn upload(gpu: &Gpu, t: &Packed) -> Uploaded {
    let usage = wgpu::BufferUsages::STORAGE;
    let b = |label, bytes: u64, data: &[u8]| io::buffer(gpu, label, bytes, usage, data).unwrap();
    Uploaded {
        table: b(
            "chain table",
            t.0.len() as u64 * 4,
            bytemuck::cast_slice(&t.0),
        ),
        words: b(
            "chain words",
            t.1.len() as u64 * 8,
            bytemuck::cast_slice(&t.1),
        ),
    }
}

/// A run's regions on the device, as the grower left them.
pub(super) struct Region {
    pub axes: wgpu::Buffer,
    pub phyts: wgpu::Buffer,
    pub status: wgpu::Buffer,
    pub row: u32,
    pub groups: u32,
}

pub(super) struct Grower {
    pub(super) pass: io::Pass,
    pub(super) pack: io::Pass,
}

impl Grower {
    pub(super) fn new(gpu: &Gpu) -> Self {
        let read = [true, true, true, false, false, false, false, false];
        Self {
            pass: io::Pass::new(gpu, source(), &read, &["grow"]),
            pack: io::Pass::new(
                gpu,
                pack_source(),
                &[true, true, true, true, false, false],
                &["pack"],
            ),
        }
    }

    /// Grows the `n` buds from `first` of `input` in regions of `caps`,
    /// recorded into `encoder`.
    pub(super) fn region(
        &self,
        gpu: &Gpu,
        encoder: &mut wgpu::CommandEncoder,
        (input, first, n): (&wgpu::Buffer, u32, u32),
        (tables, t, span, age): (&Uploaded, &Packed, i64, u32),
        caps: Caps,
    ) -> Region {
        let storage = wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC;
        let groups = n.div_ceil(64);
        let row = groups.min(gpu.device.limits().max_compute_workgroups_per_dimension) * 64;
        let (lo, hi) = (span as u32, (span >> 32) as u32);
        let cfg: [u32; 16] = [
            n,
            row,
            caps.axes,
            caps.phyts,
            caps.live,
            t.2[0],
            age,
            lo,
            hi,
            caps.sleepers,
            caps.woken,
            t.2[1],
            t.2[2],
            caps.units,
            first,
            0,
        ];
        let b = |label, bytes: u64| io::buffer(gpu, label, bytes, storage, &[]).unwrap();
        let usage = wgpu::BufferUsages::UNIFORM;
        let config = io::buffer(gpu, "chain cfg", 64, usage, bytemuck::cast_slice(&cfg)).unwrap();
        let axes = b("chain axes", u64::from(n * caps.axes) * AXW as u64 * 4);
        let phyts = b("chain phytomers", u64::from(n * caps.phyts) * PW as u64 * 4);
        let live = b("chain apexes", u64::from(n * caps.live) * 32);
        let status = b("chain status", u64::from(n) * 16);
        let sleep = u64::from(n * (caps.sleepers * SW + caps.woken * QW + caps.units * UW)) * 4;
        let sleepers = b("chain sleeping buds", sleep);
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
        Region {
            axes,
            phyts,
            status,
            row,
            groups,
        }
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
    ) {
        let cfg = [n, region.row, caps.axes, caps.phyts, stride, 0, 0, 0];
        let usage = wgpu::BufferUsages::UNIFORM;
        let cfg = io::buffer(gpu, "pack cfg", 32, usage, bytemuck::cast_slice(&cfg)).unwrap();
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
        self.pack.run(encoder, &bind, 0, region.groups);
    }
}

/// A chain's axes from its region, as `telperion_space::chain_cases` gives
/// the CPU's: the bearer's stub left out.
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
                context: (word(i, 28) & 2 != 0).then(|| wide(i, 29) as i64),
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
