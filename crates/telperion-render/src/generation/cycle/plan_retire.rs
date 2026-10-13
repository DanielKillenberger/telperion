//! A recorded cycle's retirement (DESIGN.md section 6, as decided for the
//! build), at its start before the balance, over the hierarchy its step
//! just left: the open axes marked, the closed flagged and assigned, the
//! woken buds sized, the living carried up the block levels from the
//! deepest (up to the levels' bound) and each candidate judged. Each
//! dispatch is sized from the axes, the living and the buds the cycle
//! began with, a count only the device knows, as the balance's are.
//!
//! The sweep marking every axis below a retired root inside it, from the
//! root's level down, is recorded apart (`Inside`): only a compaction and
//! the settle read the marks, so the build takes them before each
//! (fn-256 task 12). Nothing is laid under a retired root, and no axis
//! inside one is judged again (its bearer is closed, so it stands beside
//! no open axis), so the marks taken then are the marks a sweep every
//! cycle leaves.
use super::build::{Build, Kit, Tables};
use super::census as c;
use super::gate::{rec, Cond};
use super::plan::{Builder, PassId, Size, Step, MAXX};
use super::plan_hier::{swept, HierBounds};
use super::retire_device as rd;
use super::vm::E;

/// Retirement's inside sweep over the block levels its cycle's
/// retirement sized and bound: recorded on its own, replayed before a
/// compaction and before the settle (`Build::inside`).
pub(super) struct Inside {
    levels: Vec<(wgpu::BindGroup, Size, Cond)>,
    folds: [wgpu::BindGroup; 2],
}

impl Inside {
    /// The sweep, from the top level down.
    pub(super) fn plan(&self, b: &mut Builder) {
        let ret = |k| (PassId::Retire, k);
        swept(b, (ret(rd::INSIDE), ret(rd::FOLDS[1])), (&self.levels, &self.folds), false);
    }
}

impl Build {
    /// Retirement over the hierarchy's lists `parity`; its inside sweep,
    /// recorded apart.
    pub(super) fn plan_retire(&self, b: &mut Builder, kit: &Kit, t: &Tables, bound: HierBounds, parity: usize) -> Inside {
        let (s, h, r) = (self.state.caps, &self.hier, &self.retire);
        b.point();
        let words = |level: u32| {
            [
                0, s.axes, 0, level, MAXX * 64, MAXX, 0, 0, 0, 0, 0, t.age, s.axes, s.units, s.phyts, s.sleep_at(), s.sleepers,
                s.next_at(), s.woken_at(), s.woken, r.head() as u32, r.log_cap, 0, 0,
            ]
        };
        let dynamic = [(0, E::At(c::LAID0)), (2, E::At(c::CYCLE)), (6, E::At(c::LEVELS)), (8, E::At(c::LIVE0)), (9, E::At(c::SLEEPERS0)), (10, E::At(c::WOKEN0))];
        let st = &self.state;
        let bound_slot = |b: &mut Builder, level: u32| {
            let slot = b.slot(&words(level));
            for (w, e) in &dynamic {
                b.set(slot, *w, e.clone());
            }
            let rest = [&*st.axes, &st.units, &st.grown, &st.lists, &h.lists[parity], &h.shape, &r.ret, &r.verdicts];
            b.bind(&kit.retire.pass, slot, &rest)
        };
        let base = bound_slot(b, 0);
        let per = |b: &mut Builder, n: E| b.groups(n.div(E::C(64)));
        let ret = |k| (PassId::Retire, k);
        let sources = per(b, E::At(c::LIVE0).add(E::At(c::SLEEPERS0)));
        let laid = per(b, E::At(c::LAID0));
        let woken = per(b, E::At(c::WOKEN0));
        b.dispatch(ret(rd::OPEN), base.clone(), sources);
        b.dispatch(ret(rd::CLOSE), base.clone(), laid);
        b.dispatch(ret(rd::WOKEN), base.clone(), woken);
        b.dispatch(ret(rd::TERMS), base.clone(), laid);
        let mut levels = Vec::new();
        for l in 0..bound.levels {
            let bind = bound_slot(b, l);
            let width = E::At(c::LVL + l as usize + 1).sub(E::At(c::LVL + l as usize));
            let size = b.groups(E::At(c::LEVELS).gt(E::C(l)).sel(width, E::C(0)));
            levels.push((bind, size, Cond::Level(l)));
        }
        // The folded levels' workgroups: the top's, and the deepest's.
        let folds = [(E::C(0), E::At(rec::TOP)), (E::At(rec::TAIL), E::At(c::LEVELS))].map(|(lo, hi)| {
            let slot = b.slot(&words(0));
            for (w, e) in dynamic.iter().cloned().chain([(22, lo), (23, hi)]) {
                b.set(slot, w, e);
            }
            let rest = [&*st.axes, &st.units, &st.grown, &st.lists, &h.lists[parity], &h.shape, &r.ret, &r.verdicts];
            b.bind(&kit.retire.pass, slot, &rest)
        });
        swept(b, (ret(rd::LIVING), ret(rd::FOLDS[0])), (&levels, &folds), true);
        if t.counted {
            let (at, bytes) = (4 * rd::L_REUSE, 4 * rd::REUSE_WORDS);
            b.step(Step::Clear { buffer: (*r.verdicts).clone(), at, bytes: Some(bytes) });
            b.dispatch(ret(rd::REUSED), base.clone(), laid);
            b.tally(&r.verdicts, at, c::REUSE, bytes);
        }
        let laid = per(b, E::At(c::LAID0));
        b.dispatch(ret(rd::VERDICT), base.clone(), laid);
        b.dispatch(ret(rd::END), base, Size::Direct(1));
        Inside { levels, folds }
    }
}
