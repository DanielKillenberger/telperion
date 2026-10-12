//! The arena compacted in place (COUNCIL.md round 4 step 3; the host's
//! rulings of 2026-10-12, fn-256 task 5): between two cycles, the rows a
//! retired subtree let go leave every table, a column at a time through
//! one column of scratch (`compact.wgsl`).
//!
//! - **Phytomers** of a retired subtree go, recorded first where a PA
//!   keeps shed pipes or leaf girth (DESIGN.md section 6).
//! - **Axes** inside a retired subtree go, with their units, where no PA
//!   keeps records (the settle reads a kept record through its axis's
//!   row). The root stays and carries its subtree's upkeep, frozen
//!   (`freeze_main`), as the CPU's balance charges it.
//! - Every column that holds an axis's, a unit's or a phytomer's index is
//!   remapped first: the axes' bearers, successors and units, the units'
//!   axis and links, the phytomers' axis, the sleeping, living and woken
//!   buds' axes, the hierarchy's lists and their starts, and the counts.
//!
//! Each scan keeps its rows' order, so slots stay in canonical order and
//! an axis is known by its lineage key, never its slot (DESIGN.md 13).
use super::build::{Build, Kit};
use super::census as c;
use super::hier_scan::{sums_words, uniform, widest, words as read, Scan};
use super::retire_device::zeroed_dst;
use super::state::{AXES, GROWN, LAID, PENCILS, UNITS};
use crate::buffer::Owned;
use crate::{Gpu, Result};

pub(super) const ENTRIES: [&str; 5] = ["flag_main", "remap_main", "move_main", "count_main", "freeze_main"];
const FLAG: usize = 0;
const REMAP: usize = 1;
const MOVE: usize = 2;
const COUNT: usize = 3;
const FREEZE: usize = 4;

/// What a flag reads: the row's index, its column's value, whether that
/// value is an index, or (the phytomers' records) a value's fate at or
/// past `below`.
const BY_INDEX: u32 = 0;
const BY_VALUE: u32 = 1;
const BY_SET: u32 = 2;
const BY_RECORDED: u32 = 3;
const ID: u32 = 0;
const POS: u32 = 1;

/// The columns read by index (`arena.wgsl`, `hier_cols.wgsl`,
/// `retire_cols.wgsl`, `hier_cycle.wgsl`).
const A_PARENT: u32 = 4;
const A_SUCC: [u32; 2] = [31, 32];
const A_TAIL: u32 = 33;
const A_HEAD: u32 = 34;
pub(super) const A_FROZEN: u32 = 51;
const U_AXIS: u32 = 0;
const U_LINKS: [u32; 2] = [9, 10];
const P_AXIS: u32 = 2;
const S_PARENT: u32 = 1;
const S_REC: u32 = 13;
const W_AXES: [u32; 2] = [0, 1];
const R_KID_START: u32 = 0;
const R_KIDS: u32 = 1;
const R_MEM_START: u32 = 2;
const R_MEMBERS: u32 = 3;
const H_PER_AXIS: [u32; 3] = [0, 1, 3];
const R_FATE: u32 = 1;
const RETIRED: u32 = 2;
const INSIDE: u32 = 3;
const S_COST: u32 = 2;

/// The compaction's buffers, made once a build: the column of scratch,
/// each table's scan (the phytomers' two: kept, recorded), the scans'
/// sums and tally, and two spare bindings.
pub(super) struct Compactor {
    scratch: Owned,
    axes: Owned,
    units: Owned,
    phyts: Owned,
    kids: Owned,
    members: Owned,
    sums: Owned,
    tally: Owned,
    spare: [Owned; 2],
}

/// A pass's words (`compact.wgsl`'s `Cfg`).
#[derive(Debug, Clone, Copy, Default)]
struct Cfg {
    n: u32,
    stride: u32,
    col: u32,
    at: u32,
    row_at: u32,
    val_at: u32,
    val_n: u32,
    mode: u32,
    dst_at: u32,
    fate_at: u32,
    below: u32,
    by: u32,
    flag: u32,
}

impl Cfg {
    fn words(&self, row: u32) -> [u32; 16] {
        let c = self;
        [c.n, c.stride, c.col, c.at, c.row_at, c.val_at, c.val_n, c.mode, c.dst_at, c.fate_at, c.below, c.by, row, c.flag, 0, 0]
    }

    /// Column `col` of a table `stride` rows a column from word `at`.
    fn column(n: u32, stride: u32, (at, col): (u32, u32)) -> Self {
        Self { n, stride, col, at, ..Self::default() }
    }
}

impl Compactor {
    /// The buffers for tables of `caps` and a hierarchy of `stride` rows.
    pub(super) fn new(gpu: &Gpu, caps: super::state::Caps, stride: u32) -> Result<Self> {
        let lists = stride + 64;
        let most = caps.axes.max(caps.units).max(caps.phyts).max(lists);
        let scan = |label, rows: u32| zeroed_dst(gpu, label, u64::from(rows + 1));
        Ok(Self {
            scratch: zeroed_dst(gpu, "compaction column", u64::from(most))?,
            axes: scan("compaction axes", caps.axes)?,
            units: scan("compaction units", caps.units)?,
            phyts: zeroed_dst(gpu, "compaction phytomers", 2 * u64::from(Self::half(caps.phyts)))?,
            kids: scan("compaction children", lists)?,
            members: scan("compaction members", lists)?,
            sums: zeroed_dst(gpu, "compaction sums", sums_words(most + 1))?,
            tally: zeroed_dst(gpu, "compaction tally", 64)?,
            spare: [zeroed_dst(gpu, "compaction spare", 4)?, zeroed_dst(gpu, "compaction spare", 4)?],
        })
    }

    /// The phytomers' recorded scan's first word: past the kept scan.
    fn half(phyts: u32) -> u32 {
        (phyts + 1).next_multiple_of(64)
    }
}

/// A compaction's commands, recorded into one encoder as it goes.
struct Rec<'a> {
    gpu: &'a Gpu,
    kit: &'a Kit,
    e: crate::encoder::Encoder,
    held: Vec<Owned>,
    row: u32,
}

impl Rec<'_> {
    fn run(&mut self, entry: usize, cfg: Cfg, [src, rows, vals, dst]: [&wgpu::Buffer; 4], groups: u32) -> Result<()> {
        let u = uniform(self.gpu, &cfg.words(self.row))?;
        let pass = &self.kit.retire.compact;
        let bind = pass.bind(self.gpu, &[&u, src, rows, vals, dst]);
        pass.run(&mut self.e, &bind, entry, groups);
        self.held.push(u);
        Ok(())
    }

    fn scan(&mut self, k: &Compactor, (data, at, rows): (&Owned, u64, u32)) -> Result<()> {
        let s = Scan::new(self.gpu, &self.kit.hier.scan, (data, at, rows + 1), &k.sums, (&k.tally, 0))?;
        s.record(&self.kit.hier.scan, &mut self.e);
        Ok(())
    }

    /// Rows of `table` (`cfg` its column) flagged into `scan` from `at`.
    fn flag(&mut self, k: &Compactor, cfg: Cfg, (table, vals): (&wgpu::Buffer, &wgpu::Buffer), (scan, at): (&Owned, u32)) -> Result<()> {
        let cfg = Cfg { row_at: at, ..cfg };
        self.run(FLAG, cfg, [table, scan, vals, &k.spare[0]], (cfg.n + 1).div_ceil(64))?;
        self.scan(k, (scan, u64::from(at), cfg.n))
    }

    /// A column of indices into a table `scan` compacts, remapped.
    fn remap(&mut self, k: &Compactor, (table, cfg): (&wgpu::Buffer, Cfg), (scan, n, mode): (&Owned, u32, u32)) -> Result<()> {
        let cfg = Cfg { val_n: n, mode, ..cfg };
        self.run(REMAP, cfg, [table, &k.spare[0], scan, &k.spare[1]], cfg.n.div_ceil(64))
    }

    /// A column's kept rows (by `scan` from `at`) at the column's head,
    /// through the scratch, the rest of it none.
    fn compact(&mut self, k: &Compactor, (table, cfg): (&wgpu::Buffer, Cfg), (scan, at): (&Owned, u32)) -> Result<()> {
        let cfg = Cfg { row_at: at, ..cfg };
        let bytes = 4 * u64::from(cfg.n);
        self.e.clear_buffer(&k.scratch, 0, Some(bytes));
        self.run(MOVE, cfg, [table, scan, &k.spare[0], &k.scratch], cfg.n.div_ceil(64))?;
        let to = 4 * (u64::from(cfg.at) + u64::from(cfg.col) * u64::from(cfg.stride));
        self.e.copy_buffer_to_buffer(&k.scratch, 0, table, to, bytes);
        Ok(())
    }

    /// The word `at` of `table`, a count of rows, as the rows before it
    /// that `scan` keeps; where `table` is the census, its count past `n`
    /// raises `Flag::Compaction`.
    fn count(&mut self, k: &Compactor, (table, at): (&wgpu::Buffer, u32), (scan, from, n): (&Owned, u32, u32), census: bool) -> Result<()> {
        let flag = if census { c::Flag::Compaction.word() as u32 } else { 0 };
        let cfg = Cfg { n, at, row_at: from, flag, ..Cfg::default() };
        self.run(COUNT, cfg, [table, scan, &k.spare[0], &k.spare[1]], 1)
    }
}

impl Build {
    /// The arena compacted in place after cycle `cycle`, `census` the
    /// plan's, each table's passes over the rows it may hold by then
    /// (`rows`: axes, units, phytomers; one that holds more is flagged):
    /// the hierarchy's current lists are those the next cycle's step reads
    /// (`Phases::cycle`).
    pub(super) async fn compact_in_place(&mut self, gpu: &Gpu, kit: &Kit, (census, cycle): (&Owned, u32), rows: [u32; 3]) -> Result<()> {
        let keeps = self.retire.keeps();
        let k = &self.compactor;
        let (held, rs) = (self.state.caps, self.retire.stride);
        let caps = super::state::Caps { axes: rows[0].min(held.axes), units: rows[1].min(held.units), phyts: rows[2].min(held.phyts), ..held };
        let fate_at = R_FATE * rs;
        let half = Compactor::half(held.phyts);
        let mut r = Rec { gpu, kit, e: gpu.encoder(&Default::default()), held: Vec::new(), row: widest(gpu).0 };
        let (st, ret) = (&self.state, &*self.retire.ret);
        // The phytomers: kept where their axis is not retired, recorded
        // where it is and a PA keeps them.
        let by_axis = Cfg { fate_at, val_n: rs, below: RETIRED, ..Cfg::column(caps.phyts, held.phyts, (0, P_AXIS)) };
        r.flag(k, Cfg { by: BY_VALUE, ..by_axis }, (&st.grown, ret), (&k.phyts, 0))?;
        if keeps {
            r.flag(k, Cfg { by: BY_RECORDED, ..by_axis }, (&st.grown, ret), (&k.phyts, half))?;
        }
        gpu.submit([std::mem::replace(&mut r.e, gpu.encoder(&Default::default())).finish()]);
        let base = self.retire.rec_n;
        if keeps {
            let recorded = read(gpu, &k.phyts, u64::from(half + caps.phyts), 1).await?[0];
            self.grow_records(gpu, base + recorded)?;
        }
        let (k, st, ret) = (&self.compactor, &self.state, &*self.retire.ret);
        let units_words = [held.units, held.axes, rs, half, base, u32::from(keeps), c::UNITS as u32, r.row];
        let cfg = uniform(gpu, &units_words)?;
        let bind = kit.retire.units.bind(gpu, &[&cfg, census, ret, &k.phyts, &st.units, &st.axes]);
        kit.retire.units.run(&mut r.e, &bind, 0, held.units.div_ceil(64).max(1));
        r.held.push(cfg);
        if keeps {
            let rec = &self.retire;
            for col in 0..GROWN as u32 {
                let cfg = Cfg { dst_at: col * rec.rec_cap + base, ..Cfg::column(caps.phyts, held.phyts, (0, col)) };
                r.run(MOVE, Cfg { row_at: half, ..cfg }, [&st.grown, &k.phyts, &k.spare[0], &rec.recs], caps.phyts.div_ceil(64))?;
            }
        } else {
            self.compact_axes(&mut r, (census, cycle), caps)?;
        }
        let (k, st) = (&self.compactor, &self.state);
        for (table, cols) in [(&st.grown, GROWN), (&st.laid, LAID)] {
            for col in 0..cols as u32 {
                r.compact(k, (table, Cfg::column(caps.phyts, held.phyts, (0, col))), (&k.phyts, 0))?;
            }
        }
        r.count(k, (census, c::PHYTS as u32), (&k.phyts, 0, caps.phyts), true)?;
        gpu.submit([r.e.finish()]);
        Ok(())
    }

    /// The axes inside retired subtrees, and their units, compacted out:
    /// each index into them remapped first, each retired root's upkeep
    /// frozen before its subtree goes.
    fn compact_axes(&self, r: &mut Rec, (census, cycle): (&Owned, u32), caps: super::state::Caps) -> Result<()> {
        let (k, st, h) = (&self.compactor, &self.state, &self.hier);
        let (held, s) = (st.caps, h.stride);
        let (ret, fate_at) = (&*self.retire.ret, R_FATE * self.retire.stride);
        let lists = &h.lists[(cycle as usize + 1) % 2];
        // A list's entries name the axes: no more than they are.
        let rows = (s + 64).min(caps.axes + 64);
        let axes = |col| Cfg::column(caps.axes, held.axes, (0, col));
        // Each retired root's upkeep, from the sums the cycle left.
        let freeze = Cfg { fate_at, row_at: S_COST * s, ..axes(A_FROZEN) };
        r.run(FREEZE, freeze, [&st.axes, &self.sums.sums, ret, &k.spare[0]], caps.axes.div_ceil(64))?;
        // The flags: an axis kept unless inside a retired subtree, a unit
        // where its axis is.
        let inside = Cfg { fate_at, val_n: self.retire.stride, below: INSIDE, ..Cfg::default() };
        r.flag(k, Cfg { by: BY_INDEX, ..Cfg { n: caps.axes, ..inside } }, (&st.axes, ret), (&k.axes, 0))?;
        let units = |col| Cfg::column(caps.units, held.units, (0, col));
        r.flag(k, Cfg { by: BY_VALUE, ..Cfg { n: caps.units, stride: held.units, col: U_AXIS, ..inside } }, (&st.units, ret), (&k.units, 0))?;
        // Every index into the axes and the units, remapped in place.
        let (by_a, by_u) = ((&k.axes, caps.axes, ID), (&k.units, caps.units, ID));
        for col in [A_PARENT, A_SUCC[0], A_SUCC[1]] {
            r.remap(k, (&st.axes, axes(col)), by_a)?;
        }
        for col in [A_HEAD, A_TAIL] {
            r.remap(k, (&st.axes, axes(col)), by_u)?;
        }
        r.remap(k, (&st.units, units(U_AXIS)), by_a)?;
        for col in U_LINKS {
            r.remap(k, (&st.units, units(col)), by_u)?;
        }
        r.remap(k, (&st.grown, Cfg::column(caps.phyts, held.phyts, (0, P_AXIS))), by_a)?;
        let sleepers = |col| Cfg::column(held.sleepers, held.sleepers, (held.sleep_at(), col));
        r.remap(k, (&st.lists, sleepers(S_PARENT)), by_a)?;
        r.remap(k, (&st.lists, sleepers(S_REC)), by_u)?;
        r.remap(k, (&st.lists, Cfg::column(held.next, held.next, (held.next_at(), 0))), by_a)?;
        for col in W_AXES {
            r.remap(k, (&st.lists, Cfg::column(held.woken, held.woken, (held.woken_at(), col))), by_a)?;
        }
        let list = |col| Cfg::column(rows, s + 64, (0, col));
        for col in [R_KIDS, R_MEMBERS] {
            r.remap(k, (lists, list(col)), by_a)?;
        }
        // The hierarchy's lists: entries kept where they still name an
        // axis, each run's start through them.
        r.flag(k, Cfg { by: BY_SET, ..list(R_KIDS) }, (lists, &k.spare[1]), (&k.kids, 0))?;
        r.flag(k, Cfg { by: BY_SET, ..list(R_MEMBERS) }, (lists, &k.spare[1]), (&k.members, 0))?;
        r.remap(k, (lists, list(R_KID_START)), (&k.kids, rows, POS))?;
        r.remap(k, (lists, list(R_MEM_START)), (&k.members, rows, POS))?;
        // The rows themselves.
        for (table, cols) in [(&*st.axes, AXES), (&*st.pencils, PENCILS)] {
            for col in 0..cols as u32 {
                r.compact(k, (table, axes(col)), (&k.axes, 0))?;
            }
        }
        for col in 0..super::retire_device::R_COLS as u32 {
            r.compact(k, (ret, Cfg::column(caps.axes, self.retire.stride, (0, col))), (&k.axes, 0))?;
        }
        for col in H_PER_AXIS {
            r.compact(k, (&h.shape, Cfg::column(caps.axes, s, (0, col))), (&k.axes, 0))?;
        }
        r.compact(k, (lists, Cfg { n: caps.axes, ..list(R_KID_START) }), (&k.axes, 0))?;
        r.compact(k, (lists, list(R_KIDS)), (&k.kids, 0))?;
        r.compact(k, (lists, list(R_MEMBERS)), (&k.members, 0))?;
        for col in 0..UNITS as u32 {
            r.compact(k, (&st.units, units(col)), (&k.units, 0))?;
        }
        // The counts: the axes, those laid and those the hierarchy holds,
        // the units, and those retirement saw laid.
        for w in [c::AXES, c::LAID, c::BUILT] {
            r.count(k, (census, w as u32), (&k.axes, 0, caps.axes), true)?;
        }
        r.count(k, (census, c::UNITS as u32), (&k.units, 0, caps.units), true)?;
        r.count(k, (ret, self.retire.head() as u32), (&k.axes, 0, caps.axes), false)?;
        Ok(())
    }
}
