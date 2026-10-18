//! fn-256 task 2 (SETTLE-PLAN.md): the settle recorded and run on the
//! device over a finished build, test-only. Three submissions, each
//! sized by what the one before read back: the grown tree's shape and
//! depths; the mark, the assignment and the grown tree's sweeps a depth
//! at a time; then the lineage order a depth at a time and the
//! compaction into the node table. The dispatches between two scans share
//! a compute pass, each its own stamped pass while a profile is on
//! (`io::Pass::run_all`).
use super::build::{Build, Kit, Tables};
use super::hier_scan::{sums_words, uniform, widest, words as read, Scan};
use super::settle_device::{self as d, Kernels, Layout, Table};
use crate::buffer::Owned;
use crate::{Gpu, Result};
use telperion_space::SettleTo;

/// The uniform's words (`settle_cols.wgsl`'s `Cfg`).
mod w {
    pub const BASE: usize = 2;
    pub const COUNT: usize = 3;
    pub const SK: usize = 11;
    pub const SKP: usize = 16;
    pub const AT_NP: usize = 18;
    pub const AT_NU: usize = 19;
    pub const FLIP: usize = 20;
    pub const DEPTH: usize = 21;
    pub const NEXT: usize = 22;
    pub const KEPT: usize = 29;
}

/// The species' flags (`settle_cols.wgsl`'s `KEEPS` to `SHEDS`), from
/// each PA's (`grow_capture_settle_tables`).
pub(super) fn flags(pa: &[u32]) -> u32 {
    let any = |bit: u32| pa.chunks(16).any(|r| r[7] & bit != 0);
    let (sheds, retains, leaves) = (any(1), any(2), any(4));
    u32::from(retains || leaves)
        | u32::from(leaves) << 1
        | u32::from(retains) << 2
        | u32::from(sheds) << 3
}

/// The phytomers a block of the leaf mean's reduction holds
/// (`girth::LEAF_BLOCK`): each block summed in order, then the blocks.
const LEAF_BLOCK: u32 = 1024;
/// The blocks a leaf-mean dispatch takes: a count, never a timing.
const LEAF_BLOCKS: u32 = 512;
/// The axes a walking pass's dispatch takes (`Rec::ranged`): a count,
/// never a timing.
const RANGE: u32 = 1 << 17;
/// The kept axes a compaction dispatch copies: a count, never a timing.
const COMPACT_RANGE: u32 = 1 << 13;
/// The grown axes of a depth an upward sweep's dispatch takes, each
/// walking its phytomers and children: a count, never a timing.
const UP_RANGE: u32 = 1 << 14;
/// The depth's jumps: depths below 2^12.
const JUMPS: u32 = 12;

/// The settle's buffers, and what each submission read back.
pub(super) struct Settled {
    pub layout: Layout,
    pub table: Table,
    pub work: Owned,
    pub atom: Owned,
    pub nt: Owned,
    pub pw: Owned,
    /// The kept depths' first final indices, and the error a settle met
    /// (`s_error`'s code and grown axis; none: 0).
    pub kept_starts: Vec<u32>,
    /// Each kept depth's longest axis, in phytomers.
    pub kept_long: Vec<u32>,
    pub error: [u32; 2],
    /// The growth's bytes freed before the settle, and the settle's own
    /// at its largest.
    #[cfg(test)]
    pub freed: u64,
    pub flags: u32,
    pub stamped: Vec<&'static str>,
}

/// A laid node table as the chains and the dress read it: its words, its
/// layout and each depth's first axis (one past the last at the end).
#[derive(Clone, Copy)]
pub(super) struct View<'a> {
    pub nt: &'a Owned,
    pub table: &'a Table,
    pub starts: &'a [u32],
}

impl Settled {
    pub(super) fn view(&self) -> View<'_> {
        View {
            nt: &self.nt,
            table: &self.table,
            starts: &self.kept_starts,
        }
    }

    #[cfg(test)]
    /// The bytes the settle holds: its work, counts, node table and the
    /// place's columns.
    pub(super) fn bytes(&self) -> u64 {
        [&self.work, &self.atom, &self.nt, &self.pw]
            .iter()
            .map(|b| b.size())
            .sum()
    }
}

/// One settle's recording state.
struct Rec<'a> {
    gpu: &'a Gpu,
    pass: &'a super::super::io::Pass,
    scan: &'a super::super::io::Pass,
    b: &'a Build,
    cfg: [u32; 32],
    work: Owned,
    atom: Owned,
    nt: Owned,
    sums: Owned,
    tally: Owned,
    held: Vec<Owned>,
    scans: Vec<Scan>,
    stamped: Vec<&'static str>,
    /// The dispatches recorded since the last flush.
    pending: Vec<(wgpu::BindGroup, usize, u32)>,
}

impl Rec<'_> {
    /// `entry` over `items` invocations, the uniform's `words` set.
    fn run(
        &mut self,
        (entry, phase): (usize, &'static str),
        words: &[(usize, u32)],
        items: u32,
    ) -> Result<()> {
        let mut cfg = self.cfg;
        for &(k, v) in words {
            cfg[k] = v;
        }
        let buf = uniform(self.gpu, &cfg)?;
        let st = &self.b.state;
        let r = &self.b.retire;
        let bind = self.pass.bind(
            self.gpu,
            &[
                &buf, &st.axes, &st.units, &st.grown, &self.work, &self.atom, &self.nt, &r.ret,
            ],
        );
        let groups = match entry {
            d::LEAF => items,
            d::LEAF_SUM => 1,
            _ => items.div_ceil(64).max(1),
        };
        self.pending.push((bind, entry, groups));
        self.held.push(buf);
        self.stamped.push(phase);
        Ok(())
    }

    /// `entry` over `items` invocations in ranges of `RANGE`, each
    /// dispatch's first item its `base`: the passes whose invocations walk
    /// an axis's units and phytomers, cut so no dispatch runs long.
    fn ranged(
        &mut self,
        at: (usize, &'static str),
        words: &[(usize, u32)],
        (items, range): (u32, u32),
    ) -> Result<()> {
        for base in (0..items.max(1)).step_by(range as usize) {
            let mut all = words.to_vec();
            all.push((w::BASE, base));
            self.run(at, &all, range.min(items - base).max(1))?;
        }
        Ok(())
    }

    /// An exclusive scan of `rows` words of `data` from word `at`.
    fn scan(
        &mut self,
        e: &mut wgpu::CommandEncoder,
        data: Data,
        at: u64,
        rows: u32,
        phase: &'static str,
    ) -> Result<()> {
        self.flush(e);
        let buffer = match data {
            Data::Work => &self.work,
            Data::Atom => &self.atom,
        };
        let s = Scan::new(
            self.gpu,
            self.scan,
            (buffer, at, rows),
            &self.sums,
            (&self.tally, 0),
        )?;
        s.record(self.scan, e);
        self.stamped
            .extend(std::iter::repeat_n(phase, s.dispatches()));
        self.scans.push(s);
        Ok(())
    }

    /// The dispatches recorded since the last flush, into `e` in one
    /// compute pass.
    fn flush(&mut self, e: &mut wgpu::CommandEncoder) {
        self.pass.run_all(e, &std::mem::take(&mut self.pending));
    }

    /// `e` submitted, the dispatches recorded into it flushed.
    fn submit(&mut self, mut e: crate::encoder::Encoder) {
        self.flush(&mut e);
        self.gpu.submit([e.finish()]);
    }
}

/// A buffer of `words` the settle writes, copies into and reads back.
pub(super) fn zeroed(gpu: &Gpu, label: &'static str, words: u64) -> Result<Owned> {
    let usage =
        wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC | wgpu::BufferUsages::COPY_DST;
    super::super::io::buffer(gpu, label, words.max(4) * 4, usage, &[])
}

#[derive(Clone, Copy)]
enum Data {
    Work,
    Atom,
}

/// The settle of the build `b` grew, as far as `to`, each PA's constants
/// `pa` (`grow_capture_settle_tables`).
pub(super) async fn settle(
    gpu: &Gpu,
    (kit, k): (&Kit, &Kernels),
    (b, t): (&Build, &Tables),
    (pa, to): (&[u32], SettleTo),
) -> Result<Settled> {
    let _freed = b.shed_growth();
    let h = b.held;
    // The grown phytomers the arena holds, and a place for each retired
    // root's frozen terms (fn-270 R4): no more than the axes.
    let phyts = h.phyts + h.axes;
    let layout = Layout::new(h.axes, phyts, h.units, h.next.max(h.woken), t.pas);
    let flags = flags(pa);
    let mut cfg = [0u32; 32];
    let caps = b.state.caps;
    let l = &layout;
    let words = [
        h.axes,
        widest(gpu).0,
        0,
        0,
        caps.axes,
        caps.units,
        caps.phyts,
        b.age(),
        l.sa,
        l.sp,
        l.su,
        0,
        l.at_a,
        l.at_p,
        l.at_u,
        l.at_f,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        l.at_l,
        h.next,
        h.woken,
        flags,
        0,
        l.sl,
        0,
        l.kfill(),
        0,
    ];
    cfg.copy_from_slice(&words);
    let mut r = Rec {
        gpu,
        pass: &k.pass,
        scan: &kit.hier.scan,
        b,
        cfg,
        work: zeroed(gpu, "settle work", layout.words)?,
        atom: zeroed(gpu, "settle atom", layout.atom_words())?,
        nt: zeroed(gpu, "settle nodes", 4)?,
        sums: zeroed(
            gpu,
            "settle sums",
            sums_words(phyts.max(h.axes).max(d::DMAX) + 1, 1),
        )?,
        tally: zeroed(gpu, "settle tally", 64)?,
        held: Vec::new(),
        scans: Vec::new(),
        stamped: Vec::new(),
        pending: Vec::new(),
    };
    gpu.queue
        .write_buffer(&r.work, 4 * u64::from(d::K_AT), bytemuck::cast_slice(pa));
    let depths = shape(&mut r, &layout).await?;
    let deepest = depths.len() - 1;
    let (tally, kept, kept_long) = sweeps(&mut r, &layout, (&depths, flags)).await?;
    let table = Table::new(
        tally[d::T_KEPT],
        tally[d::T_KEPT_UNITS],
        tally[d::T_KEPT_PHYTS],
    );
    let collapsed = table.phyts == 0;
    let mut kept_starts = vec![0u32];
    for c in kept.iter().take(deepest + 1) {
        kept_starts.push(kept_starts.last().expect("a start") + c);
    }
    if !collapsed {
        order(&mut r, &layout, &table, &kept_starts)?;
    }
    let pw = zeroed(gpu, "settle place", 4)?;
    let error = if collapsed {
        [1, u32::MAX]
    } else {
        [0, u32::MAX]
    };
    let mut s = Settled {
        layout,
        table,
        work: r.work,
        atom: r.atom,
        nt: r.nt,
        pw,
        kept_starts,
        kept_long,
        error,
        #[cfg(test)]
        freed: _freed,
        flags,
        stamped: r.stamped,
    };
    if collapsed || to == SettleTo::Shed {
        return Ok(s);
    }
    let cfg = super::settle_place::thicken(gpu, k, b, &mut s).await?;
    if matches!(to, SettleTo::Laid | SettleTo::Placed) {
        let sags = pa.chunks(16).any(|r| r[7] & 8 != 0);
        super::settle_place::lay(gpu, k, b, (cfg, sags, to), &mut s).await?;
    }
    Ok(s)
}

/// The first submission: the lists and the shape, each axis's depth,
/// and the scans; each depth's first and count in its list back.
async fn shape(r: &mut Rec<'_>, l: &Layout) -> Result<Vec<(u32, u32)>> {
    let mut e = r.gpu.encoder(&Default::default());
    let (caps, h) = (r.b.state.caps, r.b.held);
    let lists = &r.b.state.lists;
    let to = |c: u32| 4 * (u64::from(l.at_l) + u64::from(c) * u64::from(l.sl));
    e.copy_buffer_to_buffer(
        lists,
        4 * u64::from(caps.next_at()),
        &r.work,
        to(0),
        4 * u64::from(h.next),
    );
    e.copy_buffer_to_buffer(
        lists,
        4 * u64::from(caps.next_at() + caps.next),
        &r.work,
        to(1),
        4 * u64::from(h.next),
    );
    for k in 0..5 {
        let from = 4 * (u64::from(caps.woken_at()) + u64::from(k * caps.woken));
        e.copy_buffer_to_buffer(lists, from, &r.work, to(2 + k), 4 * u64::from(h.woken));
    }
    let n = l.n;
    let g = "settle shape";
    r.run((d::SEED, g), &[], n + 1)?;
    for j in 0..JUMPS {
        r.run((d::DEPTH, g), &[(w::FLIP, j % 2)], n)?;
    }
    r.ranged((d::COUNT, g), &[(w::FLIP, JUMPS % 2)], (n, RANGE))?;
    r.scan(&mut e, Data::Work, l.a(d::W_PSTART, 0), n + 1, g)?;
    r.scan(&mut e, Data::Work, l.a(d::W_USTART, 0), n + 1, g)?;
    r.scan(&mut e, Data::Atom, u64::from(d::T_KCOUNT), n + 1, g)?;
    r.scan(&mut e, Data::Atom, u64::from(d::T_DCOUNT), d::DMAX + 1, g)?;
    r.submit(e);
    let read = read(r.gpu, &r.atom, 0, u64::from(d::T_DCOUNT + d::DMAX + 1)).await?;
    refuse(read[d::T_FAULT])?;
    let deepest = read[d::T_DEEPEST] as usize;
    let start = |k: usize| read[d::T_DCOUNT as usize + k];
    Ok((0..=deepest)
        .map(|k| (start(k), start(k + 1) - start(k)))
        .collect())
}

/// A fault the settle met, by name.
fn refuse(fault: u32) -> Result<()> {
    if fault == 0 {
        return Ok(());
    }
    let names = [
        "an axis's units or phytomers not as counted",
        "a phytomer no unit assigned",
        "a depth past the bound",
        "two kept siblings of one key",
    ];
    let named: Vec<&str> = (0..4)
        .filter(|k| fault & 1 << k != 0)
        .map(|k| names[k])
        .collect();
    Err(super::super::io::error(format!(
        "the settle: {}",
        named.join(", ")
    )))
}

/// The second submission: the lists filled, the mark, the assignment,
/// the grown tree's sweeps; the tallies and the kept per depth back.
async fn sweeps(
    r: &mut Rec<'_>,
    l: &Layout,
    (depths, flags): (&[(u32, u32)], u32),
) -> Result<(Vec<u32>, Vec<u32>, Vec<u32>)> {
    let e = r.gpu.encoder(&Default::default());
    let (n, h) = (l.n, r.b.held);
    let g = "settle mark and assign";
    r.ranged((d::FILL, g), &[(w::FLIP, JUMPS % 2)], (n, RANGE))?;
    r.run((d::SORT, g), &[], n)?;
    r.run((d::LIVING, g), &[], h.next)?;
    r.run((d::WOKEN, g), &[], h.woken)?;
    r.ranged((d::ASSIGN_E, g), &[], (n, RANGE))?;
    r.run((d::VIGOUR, g), &[], n)?;
    let at = |k: usize, (base, count): (u32, u32)| {
        [(w::BASE, base), (w::COUNT, count), (w::DEPTH, k as u32)]
    };
    for (k, &dc) in depths.iter().enumerate() {
        r.run((d::DOWN, "settle sweeps"), &at(k, dc), dc.1)?;
    }
    if flags & 3 == 3 {
        // A workgroup a block, `LEAF_BLOCKS` blocks a dispatch.
        let blocks = l.phyts.div_ceil(LEAF_BLOCK);
        for first in (0..blocks).step_by(LEAF_BLOCKS as usize) {
            let words = [(w::BASE, l.phyts), (w::COUNT, blocks), (w::FLIP, first)];
            r.run(
                (d::LEAF, "settle leaf mean"),
                &words,
                LEAF_BLOCKS.min(blocks - first),
            )?;
        }
        r.run((d::LEAF_SUM, "settle leaf mean"), &[(w::COUNT, blocks)], 1)?;
    }
    for (k, &(base, count)) in depths.iter().enumerate().rev() {
        for off in (0..count).step_by(UP_RANGE as usize) {
            let dc = (base + off, UP_RANGE.min(count - off));
            r.run((d::UP, "settle sweeps"), &at(k, dc), dc.1)?;
        }
    }
    for (k, &dc) in depths.iter().enumerate() {
        r.run((d::KEEP, "settle sweeps"), &at(k, dc), dc.1)?;
    }
    r.submit(e);
    // The tally, the kept depths' counts and their longest, in one read.
    let n = depths.len();
    let words = read(r.gpu, &r.atom, 0, u64::from(d::T_KDLONG) + n as u64).await?;
    refuse(words[d::T_FAULT])?;
    let at = |k: u32| words[k as usize..k as usize + n].to_vec();
    Ok((words[..64].to_vec(), at(d::T_KDCOUNT), at(d::T_KDLONG)))
}

/// The third submission: the lineage order a depth at a time, then the
/// compaction into the node table.
fn order(r: &mut Rec, l: &Layout, t: &Table, starts: &[u32]) -> Result<()> {
    r.nt = zeroed(r.gpu, "settle nodes", t.words)?;
    for (k, v) in [
        (w::SK, t.sk),
        (w::SKP, t.skp),
        (w::AT_NP, t.at_np),
        (w::AT_NU, t.at_nu),
        (w::KEPT, t.kept),
    ] {
        r.cfg[k] = v;
    }
    let mut e = r.gpu.encoder(&Default::default());
    let g = "settle order";
    r.run((d::ROOT, g), &[], 1)?;
    for depth in 1..starts.len() - 1 {
        let (base, count) = (starts[depth - 1], starts[depth] - starts[depth - 1]);
        if count == 0 || starts[depth + 1] == starts[depth] {
            break;
        }
        let words = [(w::BASE, base), (w::COUNT, count), (w::NEXT, starts[depth])];
        r.run((d::COUNT_KIDS, g), &words, count)?;
        r.scan(&mut e, Data::Work, l.f(d::WF_SCAN, 0), count + 1, g)?;
        r.run((d::PLACE, g), &words, count)?;
        let kids = starts[depth + 1] - starts[depth];
        r.run(
            (d::RANK, g),
            &[words[0], words[1], words[2], (w::FLIP, kids)],
            kids,
        )?;
    }
    let c = "settle compact";
    r.run((d::SIZES, c), &[], t.kept + 1)?;
    r.scan(&mut e, Data::Work, l.f(d::WF_PSTART, 0), t.kept + 1, c)?;
    r.scan(&mut e, Data::Work, l.f(d::WF_USTART, 0), t.kept + 1, c)?;
    r.ranged((d::COMPACT_E, c), &[], (t.kept, COMPACT_RANGE))?;
    r.submit(e);
    Ok(())
}

/// `settle`, blocked on: the tests' settles.
#[cfg(test)]
pub(super) fn settle_now(
    gpu: &Gpu,
    kits: (&Kit, &Kernels),
    at: (&Build, &Tables),
    to: (&[u32], SettleTo),
) -> Result<Settled> {
    pollster::block_on(settle(gpu, kits, at, to))
}
