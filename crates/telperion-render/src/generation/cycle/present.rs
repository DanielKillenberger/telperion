//! fn-270 R5 and R5a on the device (`present.wgsl`, the CPU's
//! `telperion_space::unfold`): the settled tree drawn a fraction into its
//! last year from last year's drawing. Two submissions: the axes of both
//! years matched by key, last year's gone ones and their phytomers
//! counted and scanned, read back at each depth; then the presented node
//! table laid out depth by depth (this year's axes, then last year's
//! gone), every organ moved, this year's shoots unfolded and each
//! phytomer's leaf share written, a depth after the one it stands on.
use super::super::io;
use super::hier_scan::{sums_words, uniform, widest, words, zeroed, Scan};
use super::settle_device::Table;
use super::settle_run::View;
use crate::buffer::Owned;
use crate::{Gpu, Result};

const ENTRIES: [&str; 9] = ["keys_main", "insert_main", "match_main", "gone_main", "axes_main", "present_main", "gather_main", "count_main", "place_main"];
const KEYS: usize = 0;
const INSERT: usize = 1;
const MATCH: usize = 2;
const GONE: usize = 3;
const AXES: usize = 4;
const PRESENT: usize = 5;
const GATHER: usize = 6;
const COUNT: usize = 7;
const PLACE: usize = 8;
/// The node table's phytomer columns (`settle_device::NP_COLS`).
const NP_COLS: u64 = 22;

pub(in crate::generation) struct Kernels {
    pass: io::Pass,
}

impl Kernels {
    pub(super) fn new(gpu: &Gpu) -> Self {
        let source = format!("{}\n{}", super::contract(), include_str!("present.wgsl"));
        let read = [true, true, false, false, false, false, true];
        Self { pass: io::Pass::new(gpu, source, &read, &ENTRIES) }
    }
}

/// The presented tree: its node table, its layout, each depth's first
/// axis, and each phytomer's leaf share.
pub(super) struct Presented {
    pub nt: Owned,
    pub table: Table,
    pub starts: Vec<u32>,
    pub leaf: Owned,
}

impl Presented {
    pub(super) fn view(&self) -> View<'_> {
        View { nt: &self.nt, table: &self.table, starts: &self.starts }
    }
}

/// The uniform's words (`present.wgsl`'s `Cfg`).
mod w {
    pub const BASE: usize = 1;
    pub const COUNT: usize = 2;
    pub const OUT: usize = 12;
    pub const MASK: usize = 15;
    pub const COUNTS: usize = 27;
    pub const START_HERE: usize = 31;
    pub const START_UP: usize = 32;
    pub const TAU: usize = 33;
    pub const WHICH: usize = 35;
    pub const DEPTH: usize = 36;
    pub const WORDS: usize = 38;
}

/// A buffer the presentation writes, copies into and reads back.
fn storage(gpu: &Gpu, label: &'static str, words: u64) -> Result<Owned> {
    let usage = wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC | wgpu::BufferUsages::COPY_DST;
    io::buffer(gpu, label, words.max(4) * 4, usage, &[])
}

/// The work's columns, each from a 64-word bound: last year's keys, the
/// matches either way, the gone flags and phytomers, last year's depths'
/// starts, what is gathered at them, each presented axis's shift, each
/// axis's presented place either year, and a depth's scanned children.
struct Work {
    keys: u64,
    old: u64,
    now_of: u64,
    gone: u64,
    gphyt: u64,
    starts: u64,
    gather: u64,
    shift: u64,
    pos_now: u64,
    pos_gone: u64,
    scan: u64,
    words: u64,
}

impl Work {
    fn new(kept: u64, pkept: u64, depths: u64) -> Self {
        let mut at = 64;
        let mut take = |n: u64| {
            let here = at;
            at = (at + n.max(1)).next_multiple_of(64);
            here
        };
        let (keys, old, now_of) = (take(4 * pkept), take(kept), take(pkept));
        let (gone, gphyt, starts) = (take(pkept + 1), take(pkept + 1), take(depths + 1));
        let (gather, shift) = (take(2 * (depths + 1)), take(3 * (kept + pkept)));
        let (pos_now, pos_gone, scan) = (take(kept), take(pkept), take(kept + pkept + 1));
        Self { keys, old, now_of, gone, gphyt, starts, gather, shift, pos_now, pos_gone, scan, words: at }
    }
}

/// One presentation's recording state.
struct Rec<'a> {
    gpu: &'a Gpu,
    k: &'a Kernels,
    buffers: [&'a wgpu::Buffer; 7],
    cfg: [u32; w::WORDS],
    held: Vec<Owned>,
}

impl Rec<'_> {
    fn run(&mut self, e: &mut wgpu::CommandEncoder, entry: usize, words: &[(usize, u32)], (base, count): (u32, u32)) -> Result<()> {
        if count == 0 {
            return Ok(());
        }
        let mut cfg = self.cfg;
        for &(k, v) in words.iter().chain(&[(w::BASE, base), (w::COUNT, count)]) {
            cfg[k] = v;
        }
        let buf = uniform(self.gpu, &cfg)?;
        let [a, b, c, d, f, g, h] = self.buffers;
        let bind = self.k.pass.bind(self.gpu, &[&*buf, a, b, c, d, f, g, h]);
        self.k.pass.run(e, &bind, entry, count.div_ceil(64));
        self.held.push(buf);
        Ok(())
    }
}

/// Each depth's range of a table's axes.
fn ranges(starts: &[u32], depths: usize) -> Vec<(u32, u32)> {
    let last = *starts.last().unwrap_or(&0);
    let at = |d: usize| starts.get(d).copied().unwrap_or(last);
    (0..depths).map(|d| (at(d), at(d + 1) - at(d))).collect()
}

/// `now`, settled, drawn `fraction` into its year from `before`, last
/// year's drawing (none before the first year), each PA's `unfold` row
/// (elongation, expansion, internode spread).
pub(super) async fn present(gpu: &Gpu, (k, scan): (&Kernels, &io::Pass), (now, before): (View<'_>, Option<View<'_>>), unfold: &[f32], fraction: f32) -> Result<Presented> {
    let (t, prev) = (now.table, before.unwrap_or(now));
    let pkept = if before.is_some() { prev.table.kept } else { 0 };
    let now_depths = now.starts.len().saturating_sub(1);
    let prev_depths = if before.is_some() { prev.starts.len().saturating_sub(1) } else { 0 };
    let depths = now_depths.max(prev_depths);
    let work_at = Work::new(u64::from(t.kept), u64::from(pkept), depths as u64);
    let work = storage(gpu, "present work", work_at.words)?;
    let mask = (2 * pkept).max(64).next_power_of_two() - 1;
    let hash = zeroed(gpu, "present hash", u64::from(mask) + 1)?;
    let unfold = io::buffer(gpu, "present rows", (unfold.len() as u64).max(4) * 4, wgpu::BufferUsages::STORAGE, bytemuck::cast_slice(unfold))?;
    let prev_starts: Vec<u32> = (0..=depths).map(|d| prev.starts.get(d).copied().unwrap_or(pkept).min(pkept)).collect();
    let prev_starts = if before.is_some() { prev_starts } else { vec![0; depths + 1] };
    gpu.queue.write_buffer(&work, work_at.starts * 4, bytemuck::cast_slice(&prev_starts));
    let p = prev.table;
    let mut cfg = [0u32; w::WORDS];
    let fixed = [widest(gpu).0, 0, 0, t.sk, t.skp, t.at_np, t.kept, t.phyts, p.sk, p.skp, p.at_np, pkept];
    cfg[..fixed.len()].copy_from_slice(&fixed);
    let col = [u64::from(mask), work_at.keys, work_at.old, work_at.now_of, work_at.gone, work_at.gphyt, work_at.starts, work_at.gather, work_at.shift, work_at.pos_now, work_at.pos_gone, work_at.scan];
    for (i, v) in col.iter().enumerate() {
        cfg[w::MASK + i] = *v as u32;
    }
    let tau = fraction * telperion_space::unfold::YEAR as f32;
    (cfg[w::TAU], cfg[w::TAU + 1]) = (tau.to_bits(), fraction.to_bits());
    let small = zeroed(gpu, "present unbound", 4)?;
    let small2 = zeroed(gpu, "present unbound leaf", 4)?;
    // The keys, the matches, the gone counts and their scans.
    let (now_ranges, prev_ranges) = (ranges(now.starts, depths), ranges(&prev_starts, depths));
    let gathered = {
        let mut r = Rec { gpu, k, buffers: [now.nt, prev.nt, &small, &work, &hash, &small2, &unfold].map(|b| &**b), cfg, held: Vec::new() };
        let mut e = gpu.encoder(&Default::default());
        for (d, &range) in prev_ranges.iter().enumerate() {
            r.run(&mut e, KEYS, &[(w::DEPTH, d as u32)], range)?;
        }
        r.run(&mut e, INSERT, &[], (0, pkept))?;
        for (d, &range) in now_ranges.iter().enumerate() {
            r.run(&mut e, MATCH, &[(w::DEPTH, d as u32)], range)?;
        }
        r.run(&mut e, GONE, &[], (0, pkept + 1))?;
        let sums = zeroed(gpu, "present scan sums", sums_words(pkept + 1, 1))?;
        let tally = zeroed(gpu, "present scan tally", 64)?;
        let mut scans = Vec::new();
        for at in [work_at.gone, work_at.gphyt] {
            let s = Scan::new(gpu, scan, (&work, at, pkept + 1), &sums, (&tally, 0))?;
            s.record(scan, &mut e);
            scans.push(s);
        }
        r.run(&mut e, GATHER, &[], (0, depths as u32 + 1))?;
        gpu.submit([e.finish()]);
        let fault = words(gpu, &work, 0, 1).await?;
        if fault[0] != 0 {
            return Err(io::error("two axes of one year share a key: the years cannot be told apart".into()));
        }
        words(gpu, &work, work_at.gather, 2 * (depths as u64 + 1)).await?
    };
    // The gone axes and phytomers before each depth.
    let before_depth = |d: usize| gathered[2 * d.min(depths)];
    let (gone, gone_phyts) = (gathered[2 * depths], gathered[2 * depths + 1]);
    let table = Table::new(t.kept + gone, 0, t.phyts + gone_phyts);
    let nt = storage(gpu, "presented node table", table.words)?;
    let leaf = zeroed(gpu, "presented leaves", u64::from(table.phyts))?;
    let now_at = |d: usize| now.starts.get(d).copied().unwrap_or(t.kept);
    let starts: Vec<u32> = (0..=depths).map(|d| now_at(d) + before_depth(d)).collect();
    let o = &table;
    (cfg[w::OUT], cfg[w::OUT + 1], cfg[w::OUT + 2]) = (o.sk, o.skp, o.at_np);
    // The children counts: every child, this year's, this year's first,
    // each of last year's axes' first.
    let okept = u64::from(o.kept);
    let at = [0, okept, 2 * okept, 3 * okept];
    for (i, a) in at.iter().enumerate() {
        cfg[w::COUNTS + i] = *a as u32;
    }
    let counts = storage(gpu, "present children", 3 * okept + u64::from(pkept))?;
    let mut r = Rec { gpu, k, buffers: [now.nt, prev.nt, &nt, &work, &counts, &leaf, &unfold].map(|b| &**b), cfg, held: Vec::new() };
    let mut e = gpu.encoder(&Default::default());
    for c in 0..NP_COLS {
        let (from, to) = (u64::from(t.at_np) + c * u64::from(t.skp), u64::from(o.at_np) + c * u64::from(o.skp));
        e.copy_buffer_to_buffer(now.nt, from * 4, &nt, to * 4, u64::from(t.phyts) * 4);
    }
    // Each depth placed below the one above: its children counted onto
    // their bearers, the counts scanned, each axis placed.
    let sums = zeroed(gpu, "present place sums", sums_words(o.kept + 1, 1))?;
    let tally = zeroed(gpu, "present place tally", 64)?;
    let mut scans = Vec::new();
    for d in 1..depths {
        let (up, here) = (starts[d - 1], starts[d]);
        let words = [(w::START_HERE, here), (w::START_UP, up), (w::DEPTH, d as u32)];
        r.run(&mut e, COUNT, &[&words[..], &[(w::WHICH, 0)]].concat(), now_ranges[d])?;
        r.run(&mut e, COUNT, &[&words[..], &[(w::WHICH, 1)]].concat(), prev_ranges[d])?;
        e.copy_buffer_to_buffer(&counts, u64::from(up) * 4, &work, work_at.scan * 4, u64::from(here - up).max(1) * 4);
        let s = Scan::new(gpu, scan, (&work, work_at.scan, here - up), &sums, (&tally, 0))?;
        s.record(scan, &mut e);
        scans.push(s);
        r.run(&mut e, PLACE, &[&words[..], &[(w::WHICH, 0)]].concat(), now_ranges[d])?;
        r.run(&mut e, PLACE, &[&words[..], &[(w::WHICH, 1)]].concat(), prev_ranges[d])?;
    }
    for entry in [AXES, PRESENT] {
        for d in 0..depths {
            let words = [(w::DEPTH, d as u32)];
            r.run(&mut e, entry, &[&words[..], &[(w::WHICH, 0)]].concat(), now_ranges[d])?;
            r.run(&mut e, entry, &[&words[..], &[(w::WHICH, 1)]].concat(), prev_ranges[d])?;
        }
    }
    gpu.submit([e.finish()]);
    Ok(Presented { nt, table, starts, leaf })
}
