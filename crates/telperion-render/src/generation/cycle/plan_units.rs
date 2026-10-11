//! A recorded cycle's units and wake (DESIGN.md sections 2.1 and 2.2):
//! the units counted and emitted in ranges of living apexes
//! (bounded work per dispatch), scanned between; the cycle's sleeping
//! buds compacted, their waking counted, scanned and emitted; then what
//! they made taken into the census and the new axes' and phytomers'
//! columns set (`glue.wgsl`).
use super::build::{Build, Kit, Tables};
use super::census::{self as c, Flag};
use super::device::{CYCLE, FLAG, GATHER, T_LIST, T_UNITS, T_UNITS_B, T_WAKE, T_WAKE_B};
use super::glue;
use super::plan::{Builder, PassId, MAXX};
use super::plan_scan::{fine, Fine};
use super::gate::{rec, Cond};
use super::vm::E;
use crate::Result;

/// The units' tally's word `k`, as the census holds it after the cycle.
fn made(k: usize) -> E {
    E::At(c::MADE + k)
}

impl Build {
    /// The units' uniform as the host knows it: the counts the device
    /// writes are none here (`units_dynamic`).
    pub(super) fn units_static(&self, t: &Tables, (emit, wake, base): (u32, u32, u32)) -> Vec<u32> {
        let s = self.state.caps;
        vec![
            0, MAXX * 64, 0, t.age, emit, wake, t.lights, 0, s.axes, s.units, s.phyts, s.sleepers, s.next, s.woken,
            s.sleepers, 0, self.b_at(), s.sleep_at(), s.next_at(), s.woken_at(), s.list_at(), 0, 0, 0, 0, 0, t.rows,
            t.whorls, 0, base, t.seed as u32, (t.seed >> 32) as u32,
        ]
    }

    /// A units slot's words the device writes: its count `n`, the cycle,
    /// the laid axes, the counts before the cycle and the living read.
    fn units_slot(&self, b: &mut Builder, t: &Tables, n: E, (emit, wake, base): (u32, u32, u32)) -> u32 {
        let slot = b.slot(&self.units_static(t, (emit, wake, base)));
        let words = [
            (0, n),
            (2, E::At(c::CYCLE)),
            (15, E::At(c::LAID0)),
            (21, E::At(c::AXES0)),
            (22, E::At(c::UNITS0)),
            (23, E::At(c::PHYTS0)),
            (24, E::At(c::SLEEPERS0)),
            (25, E::At(c::LIVE).mul(E::C(2))),
            (28, E::At(c::WOKEN0)),
        ];
        for (w, e) in words {
            b.set(slot, w, e);
        }
        slot
    }

    fn units_bind(&self, b: &Builder, kit: &Kit, t: &Tables, slot: u32) -> wgpu::BindGroup {
        let s = &self.state;
        let rest = [&*t.table, &t.words, &self.inputs, &s.axes, &s.units, &s.grown, &s.lists, &self.counts];
        b.bind(&kit.units.cycle, slot, &rest)
    }

    /// The cycle's units and wake, recorded.
    pub(super) fn plan_units(&self, b: &mut Builder, kit: &Kit, t: &Tables, (ranges, range): (u32, u32)) -> Result<Vec<Fine>> {
        b.point();
        let live = E::At(c::LIVE);
        b.flag(Flag::UnitRanges, live.clone().gt(E::At(rec::UNITS).mul(E::C(range))));
        b.point();
        let laid = E::At(c::LAID0);
        let w = glue::Words { n: 0, live_stride: self.sums.live_stride, ..glue::Words::default() };
        let inputs = [(0, live.clone().max(laid.clone())), (3, laid.clone()), (13, laid.gt(E::C(0)).sel(E::C(0), E::C(1))), (14, live.clone())];
        self.glue_step(b, kit, (glue::INPUTS, &self.inputs, &self.spare), w, &inputs);
        let mut binds = Vec::new();
        for emit in 0..2 {
            for r in 0..ranges {
                let slot = self.units_slot(b, t, live.clone(), (emit, 0, r * range));
                let left = live.clone().sub(E::C(r * range)).min(E::C(range));
                let size = b.groups(left.div(E::C(64)));
                binds.push((self.units_bind(b, kit, t, slot), size));
            }
        }
        let sleepers = E::At(c::SLEEPERS0);
        let mut wake = Vec::new();
        for emit in 0..2 {
            let slot = self.units_slot(b, t, sleepers.clone(), (emit, 1, 0));
            let size = b.groups(sleepers.clone().div(E::C(64)));
            wake.push((self.units_bind(b, kit, t, slot), size));
        }
        let unit = |k| (PassId::Units, k);
        let (count, emitted) = binds.split_at(ranges as usize);
        for (r, (bind, size)) in count.iter().enumerate() {
            b.enter(Cond::Units(r as u32));
            b.dispatch(unit(CYCLE), bind.clone(), *size);
            b.leave();
        }
        let lists = &self.state.lists;
        let (n_live, n_sleep) = (self.state.caps.next, self.state.caps.sleepers);
        let scan = (&kit.units.scan, PassId::UnitScan);
        let (a, b_at) = ((&self.counts, 0), (&self.counts, self.b_at()));
        let mut held = vec![
            fine(b, scan, a, (live.clone(), n_live, 4, 4), (lists, T_UNITS as u64))?,
            fine(b, scan, b_at, (live.clone(), n_live, 4, 4), (lists, T_UNITS_B as u64))?,
        ];
        for (r, (bind, size)) in emitted.iter().enumerate() {
            b.enter(Cond::Units(r as u32));
            b.dispatch(unit(CYCLE), bind.clone(), *size);
            b.leave();
        }
        b.dispatch(unit(FLAG), wake[0].0.clone(), wake[0].1);
        held.push(fine(b, scan, a, (sleepers.clone(), n_sleep, 4, 1), (lists, T_LIST as u64))?);
        b.dispatch(unit(GATHER), wake[0].0.clone(), wake[0].1);
        b.dispatch(unit(CYCLE), wake[0].0.clone(), wake[0].1);
        held.push(fine(b, scan, a, (sleepers.clone(), n_sleep, 4, 4), (lists, T_WAKE as u64))?);
        held.push(fine(b, scan, b_at, (sleepers, n_sleep, 4, 4), (lists, T_WAKE_B as u64))?);
        b.dispatch(unit(CYCLE), wake[1].0.clone(), wake[1].1);
        b.tally(lists, 0, c::MADE, 4 * 24);
        self.plan_made(b, kit, t);
        Ok(held)
    }

    /// What the units made taken into the census, its capacities and
    /// faults flagged, and the new axes' and phytomers' columns set.
    fn plan_made(&self, b: &mut Builder, kit: &Kit, t: &Tables) {
        b.point();
        let both = |k: usize| made(T_UNITS + k).add(made(T_WAKE + k));
        let s = self.state.caps;
        let held = [
            (c::AXES, E::At(c::AXES0).add(both(0)), s.axes, Flag::Axes),
            (c::PHYTS, E::At(c::PHYTS0).add(both(1)), s.phyts, Flag::Phyts),
            (c::UNITS, E::At(c::UNITS0).add(both(2)), s.units, Flag::Units),
            (c::SLEEPERS, E::At(c::SLEEPERS0).add(both(3)), s.sleepers, Flag::Sleepers),
            (c::NEXT, made(T_UNITS_B).add(made(T_WAKE_B)), s.next, Flag::Next),
            (c::WOKEN, E::At(c::WOKEN0).add(made(T_WAKE_B + 1)), s.woken, Flag::Woken),
        ];
        for (w, e, cap, flag) in held {
            b.count(w, e.clone());
            b.flag(flag, e.gt(E::C(cap)));
        }
        let grown = E::At(c::GROWN).add(both(1));
        b.count(c::GROWN, grown.clone());
        b.flag(Flag::Budget, grown.gt(E::C(t.budget)));
        b.flag(Flag::UnitFaults, made(T_UNITS_B + 2).max(made(T_WAKE_B + 2)));
        let w = glue::Words { live_stride: self.sums.live_stride, ..glue::Words::default() };
        let born = [(0, both(0)), (11, E::At(c::AXES0))];
        self.glue_step(b, kit, (glue::BORN, &self.spare, &self.flags), w, &born);
        let fresh = [(0, both(1)), (11, E::At(c::PHYTS0))];
        self.glue_step(b, kit, (glue::FRESH, &self.spare, &self.flags), w, &fresh);
    }
}
