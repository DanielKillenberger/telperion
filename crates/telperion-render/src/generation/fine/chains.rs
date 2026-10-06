//! The chain grower's host side for its tests (fn-209 F1, task 2): the
//! species point's tables and the chains' buds as `chain.wgsl` reads them,
//! a dispatch of chains in regions, and their axes packed and read back
//! (`pack.wgsl`) as `telperion_space::chain_cases` gives the CPU's.
use super::super::io;
use super::units::pack;
use crate::Gpu;
use std::time::Instant;
use telperion_space::{ChainAxis, ChainBud, ChainTables, Woke};

const CHAIN: &str = include_str!("chain.wgsl");
const APEX: &str = include_str!("apex.wgsl");
const SLEEP: &str = include_str!("sleep.wgsl");
const PACK: &str = include_str!("pack.wgsl");
const AXW: usize = 28;
const PW: usize = 6;
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
}

impl Caps {
    pub(super) fn times(self, k: u32) -> Self {
        Self {
            axes: self.axes * k,
            phyts: self.phyts * k,
            live: self.live * k,
            sleepers: self.sleepers * k,
            woken: self.woken * k,
        }
    }
}

/// A sleeping bud's and a woken bud's words (`chain.wgsl`).
const SW: u32 = 10;
const QW: u32 = 8;

/// The unit tables, then each PA's row (flags, lived, successor, words at)
/// and its words (`chain.wgsl`); the rows' offset in the table.
pub(super) fn packed(t: &ChainTables) -> (Vec<u32>, Vec<u64>, u32) {
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
            | u32::from(r.lasting.is_some()) << 5;
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
        ]);
        woods.push((at as usize + 15, pa));
    }
    for (slot, pa) in woods {
        words[slot] = words.len() as u64;
        words.extend(t.log_woods[pa].iter().map(|&l| l as u64));
    }
    (table, words, rows)
}

fn bud_words(b: &ChainBud) -> [u64; 10] {
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
        pair(u32::from(b.woke.is_some()), u32::from(b.slot)),
        w.key,
        pair(w.pa as u32, w.stage as u32),
        w.spent,
        pair(w.sleep as u32, w.cycle),
    ]
}

pub(super) struct Grower {
    pass: io::Pass,
    pack: io::Pass,
}

/// One dispatch's chains as grown: each chain's axes, or none where it
/// outgrew its region.
pub(super) type Grown = Vec<Option<Vec<ChainAxis>>>;

impl Grower {
    pub(super) fn new(gpu: &Gpu) -> Self {
        let shader = format!("{}\n{CHAIN}\n{APEX}\n{SLEEP}", super::zones());
        let read = [true, true, true, false, false, false, false, false];
        let pack = PACK.to_string();
        Self {
            pass: io::Pass::new(gpu, shader, &read, &["grow"]),
            pack: io::Pass::new(
                gpu,
                pack,
                &[true, true, true, true, false, false],
                &["pack"],
            ),
        }
    }

    /// Grows `buds` in regions of `caps`: each chain's axes, or none where
    /// it outgrew its region; the milliseconds from upload to the device's
    /// finishing the growth, and those packing and reading it back.
    pub(super) fn run(
        &self,
        gpu: &Gpu,
        buds: &[ChainBud],
        t: &(Vec<u32>, Vec<u64>, u32),
        (span, age): (i64, u32),
        caps: Caps,
    ) -> (Grown, f64, f64) {
        let start = Instant::now();
        let storage = wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC;
        let n = buds.len() as u32;
        let groups = n.div_ceil(64);
        let row = groups.min(gpu.device.limits().max_compute_workgroups_per_dimension) * 64;
        let (lo, hi) = (span as u32, (span >> 32) as u32);
        let cfg: [u32; 12] = [
            n,
            row,
            caps.axes,
            caps.phyts,
            caps.live,
            t.2,
            age,
            lo,
            hi,
            caps.sleepers,
            caps.woken,
            0,
        ];
        let input: Vec<u64> = buds.iter().flat_map(bud_words).collect();
        let b =
            |label, bytes: u64, data: &[u8]| io::buffer(gpu, label, bytes, storage, data).unwrap();
        let uniform = |label, data: &[u32]| {
            let bytes = std::mem::size_of_val(data) as u64;
            let usage = wgpu::BufferUsages::UNIFORM;
            io::buffer(gpu, label, bytes, usage, bytemuck::cast_slice(data)).unwrap()
        };
        let config = uniform("chain cfg", &cfg);
        let input = b(
            "chain buds",
            input.len() as u64 * 8,
            bytemuck::cast_slice(&input),
        );
        let table = b(
            "chain table",
            t.0.len() as u64 * 4,
            bytemuck::cast_slice(&t.0),
        );
        let words = b(
            "chain words",
            t.1.len() as u64 * 8,
            bytemuck::cast_slice(&t.1),
        );
        let axes = b("chain axes", u64::from(n * caps.axes) * AXW as u64 * 4, &[]);
        let phyts = b(
            "chain phytomers",
            u64::from(n * caps.phyts) * PW as u64 * 4,
            &[],
        );
        let live = b("chain apexes", u64::from(n * caps.live) * 32, &[]);
        let status = b("chain status", u64::from(n) * 16, &[]);
        let sleep = u64::from(n * (caps.sleepers * SW + caps.woken * QW)) * 4;
        let sleepers = b("chain sleeping buds", sleep, &[]);
        let all = [
            &config, &input, &table, &words, &axes, &phyts, &live, &status, &sleepers,
        ];
        let bind = self.pass.bind(gpu, &all);
        let mut encoder = gpu.device.create_command_encoder(&Default::default());
        self.pass.run(&mut encoder, &bind, 0, groups);
        gpu.queue.submit([encoder.finish()]);
        gpu.device
            .poll(wgpu::PollType::wait_indefinitely())
            .unwrap();
        let grown_ms = start.elapsed().as_secs_f64() * 1e3;
        let start = Instant::now();
        let read = io::read(gpu, &status, u64::from(n) * 16).unwrap();
        let status_words: Vec<[u32; 4]> = bytemuck::cast_slice::<u8, u32>(&read)
            .as_chunks::<4>()
            .0
            .to_vec();
        // Each chain's place in the packed axes and phytomers.
        let mut offsets = Vec::with_capacity(buds.len());
        let (mut at_axes, mut at_phyts) = (0u32, 0u32);
        for &[count, grown, overflow, _] in &status_words {
            offsets.push([at_axes, at_phyts]);
            if overflow == 0 {
                (at_axes, at_phyts) = (at_axes + count, at_phyts + grown);
            }
        }
        let pack_cfg = uniform("pack cfg", &[n, row, caps.axes, caps.phyts]);
        let offsets_buf = b(
            "pack offsets",
            offsets.len() as u64 * 8,
            bytemuck::cast_slice(&offsets),
        );
        let axes_bytes = u64::from(at_axes.max(1)) * AXW as u64 * 4;
        let phyt_bytes = u64::from(at_phyts.max(1)) * PW as u64 * 4;
        let packed_axes = b("packed axes", axes_bytes, &[]);
        let packed_phyts = b("packed phytomers", phyt_bytes, &[]);
        let bind = self.pack.bind(
            gpu,
            &[
                &pack_cfg,
                &status,
                &offsets_buf,
                &axes,
                &phyts,
                &packed_axes,
                &packed_phyts,
            ],
        );
        let mut encoder = gpu.device.create_command_encoder(&Default::default());
        self.pack.run(&mut encoder, &bind, 0, groups);
        gpu.queue.submit([encoder.finish()]);
        let axes = io::read(gpu, &packed_axes, axes_bytes).unwrap();
        let axes: &[u32] = bytemuck::cast_slice(&axes);
        let phyts = io::read(gpu, &packed_phyts, phyt_bytes).unwrap();
        let phyts: &[u32] = bytemuck::cast_slice(&phyts);
        let grown = buds
            .iter()
            .enumerate()
            .map(|(c, bud)| {
                let [count, grown, overflow, _] = status_words[c];
                let [a, p] = offsets[c].map(|o| o as usize);
                (overflow == 0).then(|| {
                    let a = &axes[a * AXW..(a + count as usize) * AXW];
                    topology(bud, a, &phyts[p * PW..(p + grown as usize) * PW])
                })
            })
            .collect();
        (grown, grown_ms, start.elapsed().as_secs_f64() * 1e3)
    }
}

/// A chain's axes from its region, as `telperion_space::chain_cases` gives
/// the CPU's: the bearer's stub left out.
fn topology(bud: &ChainBud, axes: &[u32], phyts: &[u32]) -> Vec<ChainAxis> {
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
