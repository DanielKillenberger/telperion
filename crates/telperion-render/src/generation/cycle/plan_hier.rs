//! A recorded cycle's balance and allotment over the hierarchy
//! (DESIGN.md sections 2 to 4): the tree's view gathered, the
//! hierarchy's step taking in the axes laid since the last (its
//! generations and placing passes recorded up to their bounds), the sums
//! by block level (up to the levels' bound), and their words into the
//! pencils. A count past a bound raises its flag.
use super::build::{Build, Kit, Tables};
use super::census::{self as c, Flag};
use super::glue;
use super::hier_build::{R_LVL_START, X_FAULT, X_GENS, X_LEVELS, X_UNPLACED};
use super::hier_device::{self as hd, BUD_BLOCK, CLEARED, FROM_TREE, V_FLAG};
use super::hier_plan as hp;
use super::plan::{Builder, PassId, Size, Step, MAXX};
use super::plan_scan::levelled;
use super::vm::E;

/// How far the hierarchy's loops are recorded: generations a step
/// places, placing passes a generation takes, block levels.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct HierBounds {
    pub gens: u32,
    pub passes: u32,
    pub levels: u32,
}

impl Build {
    fn hier_bind(&self, b: &Builder, kit: &Kit, slot: u32, parity: usize) -> wgpu::BindGroup {
        let h = &self.hier;
        let rest = [&*self.tree, &h.shape, &h.counts, &h.lists[parity], &h.lists[1 - parity], &h.bucket, &h.tally];
        b.bind(&kit.hier.build, slot, &rest)
    }

    /// The first `rows` rows of the counts' column `col`, none.
    fn clear_rows(&self, b: &mut Builder, kit: &Kit, col: u64, rows: E) {
        let s = self.state.caps.axes;
        let w = glue::Words { first: col as u32 * s, live_stride: self.sums.live_stride, ..glue::Words::default() };
        self.glue_step(b, kit, (glue::CLEAR, &self.hier.counts, &self.spare), w, &[(0, rows)]);
    }

    /// The hierarchy's step over lists `parity` (its next the other).
    pub(super) fn plan_step(&self, b: &mut Builder, kit: &Kit, bound: HierBounds, parity: usize) {
        let h = &self.hier;
        b.tally(&h.tally, 0, c::BLOCKS0, 8);
        b.point();
        let (old, n) = (E::At(c::BUILT), E::At(c::LAID0));
        let new = n.clone().sub(old.clone());
        let s = self.state.caps.axes;
        let words = |round: u32| [0, 0, h.cap, round, s, MAXX * 64, 0, 0];
        let head = b.slot(&words(0));
        b.set(head, 0, old.clone());
        b.set(head, 1, n.clone());
        let head_bind = self.hier_bind(b, kit, head, parity);
        let over_new = b.groups(new.clone().div(E::C(64)));
        let w = glue::Words { live_stride: self.sums.live_stride, ..glue::Words::default() };
        self.glue_step(b, kit, (glue::TREE, &self.tree, &self.spare), w, &[(0, n.clone()), (3, n.clone())]);
        for col in [hp::C_KIDS, hp::C_KID_FILL] {
            self.clear_rows(b, kit, col, n.clone());
        }
        b.step(Step::Clear { buffer: (*h.tally).clone(), at: 4 * X_GENS, bytes: Some(4) });
        let build = |k| (PassId::HierBuild, k);
        b.dispatch(build(hp::BEGIN), head_bind.clone(), Size::Direct(1));
        b.dispatch(build(hp::DEPTH), head_bind, over_new);
        b.tally(&h.tally, 4 * X_GENS, c::GENS, 4);
        b.point();
        b.flag(Flag::HierGens, E::At(c::GENS).gt(E::C(bound.gens)));
        let blocks = E::At(c::BLOCKS0).add(new.clone());
        let levels = E::At(c::LEVELS0).add(E::At(c::GENS));
        // A round past the step's generations places nothing: its
        // dispatches are empty.
        let mut rounds = Vec::new();
        for r in 0..bound.gens {
            let slot = b.slot(&words(r));
            for (w, e) in [(0, old.clone()), (1, n.clone()), (6, blocks.clone()), (7, levels.clone())] {
                b.set(slot, w, e);
            }
            let active = E::At(c::GENS).gt(E::C(r));
            let over = b.groups(active.clone().sel(new.clone().div(E::C(64)), E::C(0)));
            rounds.push((self.hier_bind(b, kit, slot, parity), over, active));
        }
        let groups = |b: &mut Builder, e: E| b.groups(e.div(E::C(64)));
        let (g_new, g_n, g_old, g_blocks) = (groups(b, new.clone()), groups(b, n.clone()), groups(b, old), groups(b, blocks.clone()));
        let g_levels = b.groups(levels.clone());
        let scan = |b: &mut Builder, col: u64, rows: E, to: u64| {
            let at = col * u64::from(s);
            levelled(b, &kit.hier.scan, (&h.counts, at, s), &h.sums, (&h.tally, to), rows);
        };
        scan(b, hp::C_KIDS, n.clone(), hp::X_KIDS);
        let base = rounds[0].0.clone();
        b.dispatch(build(hp::KID_PLACE), base.clone(), g_new);
        b.dispatch(build(hp::KID_MERGE), base.clone(), g_n);
        for (r, (bind, over, active)) in rounds.iter().enumerate() {
            let (rows, fresh) = (active.clone().sel(blocks.clone(), E::C(0)), active.clone().sel(new.clone(), E::C(0)));
            for _ in 0..bound.passes {
                for col in [hp::C_JOIN, hp::C_JOIN_FILL] {
                    self.clear_rows(b, kit, col, rows.clone());
                }
                b.step(Step::Clear { buffer: (*h.tally).clone(), at: 4 * X_UNPLACED, bytes: Some(4) });
                b.dispatch(build(hp::TARGET), bind.clone(), *over);
                scan(b, hp::C_JOIN, rows.clone(), hp::X_JOIN);
                b.dispatch(build(hp::JOIN_PLACE), bind.clone(), *over);
                b.dispatch(build(hp::DECIDE), bind.clone(), *over);
                scan(b, hp::C_OPEN, fresh.clone(), hp::X_OPEN);
                b.dispatch(build(hp::ASSIGN), bind.clone(), *over);
                b.dispatch(build(hp::FOLLOW), bind.clone(), *over);
                b.dispatch(build(hp::ROUND_END), bind.clone(), Size::Direct(1));
            }
            b.tally(&h.tally, 4 * X_UNPLACED, c::UNPLACED + r, 4);
        }
        self.clear_rows(b, kit, hp::C_MEMBERS, blocks.clone());
        self.clear_rows(b, kit, hp::C_MEMBER_FILL, blocks.clone());
        self.clear_rows(b, kit, hp::C_LEVEL, levels.clone());
        b.dispatch(build(hp::MEMBER_COUNT), base.clone(), g_new);
        scan(b, hp::C_MEMBERS, blocks, hp::X_MEMBERS);
        scan(b, hp::C_LEVEL, levels, hp::X_LEVEL);
        for (entry, size) in [(hp::MEMBER_PLACE, g_new), (hp::MEMBER_RANK, g_new), (hp::MEMBER_OLD, g_old), (hp::MEMBER_NEW, g_new), (hp::MEMBER_START, g_blocks), (hp::LEVEL_MERGE, g_levels)] {
            b.dispatch(build(entry), base.clone(), size);
        }
        let lists = &h.lists[1 - parity];
        let at = 4 * R_LVL_START * (u64::from(s) + 64);
        b.tally(lists, at, c::LVL, 4 * (u64::from(bound.levels) + 1));
        b.tally(&h.tally, 4 * X_LEVELS as u64, c::LEVELS, 4);
        b.tally(&h.tally, 4 * X_FAULT as u64, c::HIER_FAULT, 4);
        b.point();
        let unplaced = (0..bound.gens as usize).fold(E::C(0), |a, r| a.max(E::At(c::UNPLACED + r)));
        b.flag(Flag::HierPasses, unplaced);
        b.flag(Flag::HierLevels, E::At(c::LEVELS).gt(E::C(bound.levels)));
        b.flag(Flag::HierFault, E::At(c::HIER_FAULT));
        b.count(c::BUILT, E::At(c::LAID0));
    }

    fn sums_bind(&self, b: &Builder, kit: &Kit, slot: u32, parity: usize) -> wgpu::BindGroup {
        let (h, m) = (&self.hier, &self.sums);
        let rest = [&*self.tree, &m.rows, &h.lists[parity], &h.shape, &m.sums, &m.live, &m.norm, &m.tally];
        b.bind(&kit.hier.sums, slot, &rest)
    }

    /// The balance and allotment over the hierarchy as its step left it
    /// in lists `parity`, and their words into the pencils.
    pub(super) fn plan_sums(&self, b: &mut Builder, kit: &Kit, t: &Tables, bound: HierBounds, parity: usize) {
        let m = &self.sums;
        let s = u64::from(self.state.caps.axes);
        let col = 4 * s;
        for (from, to) in FROM_TREE {
            b.step(Step::Copy { from: (*self.tree).clone(), at: from * col, to: (*m.sums).clone(), to_at: to * col, bytes: col });
        }
        for k in CLEARED {
            b.step(Step::Clear { buffer: (*m.sums).clone(), at: k * col, bytes: Some(col) });
        }
        b.step(Step::Clear { buffer: (*m.tally).clone(), at: 0, bytes: None });
        let (n, live) = (E::At(c::LAID0), E::At(c::LIVE0));
        let words = |level: u32| [0, s as u32, 0, m.live_stride, 0, t.pas, level, MAXX * 64, MAXX, 0, 0, 0];
        let mut levels = Vec::new();
        for l in 0..bound.levels {
            let slot = b.slot(&words(l));
            for (w, e) in [(0, n.clone()), (2, live.clone()), (4, E::At(c::CYCLE))] {
                b.set(slot, w, e);
            }
            let width = E::At(c::LVL + l as usize + 1).sub(E::At(c::LVL + l as usize));
            let size = b.groups(E::At(c::LEVELS).gt(E::C(l)).sel(width, E::C(0)));
            levels.push((self.sums_bind(b, kit, slot, parity), size));
        }
        let apexes = b.groups(live.clone().div(E::C(64)));
        let partials = b.groups(live.clone().div(E::C(BUD_BLOCK)).max(E::C(1)));
        let (base, sums) = (levels[0].0.clone(), |k| (PassId::HierSums, k));
        b.dispatch(sums(hd::MARK), base.clone(), apexes);
        for (bind, size) in levels.iter().rev() {
            b.dispatch(sums(hd::TOTALS), bind.clone(), *size);
        }
        for (bind, size) in &levels {
            b.dispatch(sums(hd::WALK), bind.clone(), *size);
        }
        b.dispatch(sums(hd::SHED), base.clone(), apexes);
        let cap = m.live_stride;
        levelled(b, &kit.hier.scan, (&m.live, V_FLAG * u64::from(cap), cap), &m.scan_sums, (&m.tally, 0), live);
        b.dispatch(sums(hd::COMPACT), base.clone(), apexes);
        b.dispatch(sums(hd::PRESENCE), base.clone(), apexes);
        for (bind, size) in levels.iter().rev() {
            b.dispatch(sums(hd::SUB), bind.clone(), *size);
        }
        for (bind, size) in &levels {
            b.dispatch(sums(hd::SPLIT), bind.clone(), *size);
        }
        b.dispatch(sums(hd::PARTIALS), base.clone(), partials);
        b.dispatch(sums(hd::COMBINE), base.clone(), Size::Direct(1));
        b.dispatch(sums(hd::SIZES), base, apexes);
        b.tally(&m.tally, 0, c::LIVE, 4);
        b.point();
        let w = glue::Words { live_stride: m.live_stride, ..glue::Words::default() };
        let scatter = [(0, n.clone()), (3, n), (2, E::At(c::CYCLE))];
        self.glue_step(b, kit, (glue::SCATTER, &m.sums, &self.spare), w, &scatter);
    }
}
