//! A recorded cycle's units and wake (DESIGN.md sections 2.1, 2.2 and
//! 10): the units counted and emitted in ranges of living apexes, a job
//! each, each range's jobs run in rounds of small phases, scanned
//! between; the cycle's sleeping buds compacted, their waking counted,
//! scanned and emitted the same way; then what they made taken into the
//! census and the new axes' and phytomers' columns set (`glue.wgsl`).
use super::build::{Build, Kit, Tables};
use super::census::{self as c, Flag};
use super::device::{self, AFTER, CLOSE, DECIDE, FLAG, GATHER, INIT, NODES, RANGE, T_LIST, T_ROUNDS, T_UNITS, T_UNITS_B, T_WAKE, T_WAKE_B, WOKE, ZONES};
use super::glue;
use super::plan::{Builder, PassId, Step, MAXX};
use super::plan_scan::{levelled, levels_needed, sums_words, Shape};
use super::gate::{levels_of, rec, Cond, Scan};
use super::hier_scan::zeroed;
use crate::buffer::Owned;
use super::vm::E;
use crate::Result;

/// The units' tally's word `k`, as the census holds it after the cycle.
fn made(k: usize) -> E {
    E::At(c::MADE + k)
}

impl Build {
    /// The units' uniform as the host knows it: the counts the device
    /// writes are none here (`units_dynamic`).
    pub(super) fn units_static(&self, t: &Tables, (emit, wake, base, write): (u32, u32, u32, u32)) -> Vec<u32> {
        let s = self.state.caps;
        let at = device::counts(s, t.zones);
        vec![
            0, MAXX * 64, 0, t.age, emit, wake, t.lights, 0, s.axes, s.units, s.phyts, s.sleepers, s.next, s.woken,
            s.sleepers, 0, self.b_at(), s.sleep_at(), s.next_at(), s.woken_at(), s.list_at(), 0, 0, 0, 0, 0, t.rows,
            t.whorls, 0, base, t.seed as u32, (t.seed >> 32) as u32, at.job_at, at.jobs, at.zone_at, t.zones.max(1), write,
        ]
    }

    /// A units slot's words the device writes: its count `n`, the cycle,
    /// the laid axes, the counts before the cycle and the living read.
    fn units_slot(&self, b: &mut Builder, t: &Tables, n: E, (emit, wake, base, write): (u32, u32, u32, u32)) -> u32 {
        let slot = b.slot(&self.units_static(t, (emit, wake, base, write)));
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
    pub(super) fn plan_units(&self, b: &mut Builder, kit: &Kit, t: &Tables, (ranges, range, rounds): (u32, u32, u32)) -> Result<Vec<Owned>> {
        let (n_live, n_sleep) = (self.state.caps.next, self.state.caps.sleepers);
        b.point();
        let live = E::At(c::LIVE);
        let sleepers = E::At(c::SLEEPERS0);
        b.flag(Flag::UnitRanges, live.clone().gt(E::At(rec::UNITS).mul(E::C(range))));
        let most = |n: u32| levels_of(n) as usize;
        b.flag(Flag::UnitScan, levels_needed(&live, most(n_live)).gt(E::At(rec::scan(Scan::Units))));
        b.flag(Flag::SleepScan, levels_needed(&sleepers, most(n_sleep)).gt(E::At(rec::scan(Scan::Sleep))));
        b.point();
        let laid = E::At(c::LAID0);
        let w = glue::Words { n: 0, live_stride: self.sums.live_stride, ..glue::Words::default() };
        let inputs = [(0, live.clone().max(laid.clone())), (3, laid.clone()), (13, laid.gt(E::C(0)).sel(E::C(0), E::C(1))), (14, live.clone())];
        self.glue_step(b, kit, (glue::INPUTS, &self.inputs, &self.spare), w, &inputs);
        let lists = &self.state.lists;
        b.step(Step::Clear { buffer: (**lists).clone(), at: 4 * T_ROUNDS as u64, bytes: Some(4) });
        let zones = t.zones.max(1);
        let words = sums_words(n_live, 4).max(sums_words(n_sleep, 4));
        let sums = zeroed(b.gpu, "units scan sums", words)?;
        let (a, b_at) = (0, u64::from(self.b_at()));
        // The first `fields` of 4-word counts from word `at`, `n` of
        // them at most, totalled to the lists' word `to`.
        let scan = |b: &mut Builder, (at, fields, to): (u64, u32, usize), (rows, kind): (&E, Scan)| {
            let n = if kind == Scan::Units { n_live } else { n_sleep };
            let shape = Shape { stride: 4, fields };
            levelled(b, (&kit.hier.scan, kind), (&self.counts, at, n, shape), &sums, (lists, to as u64), rows.clone());
        };
        // A pass of a range's jobs: begun, `rounds` rounds of phases, closed.
        let pass = |b: &mut Builder, (n, emit, wake, base): (&E, u32, u32, u32), left: &E| {
            let [counted, written] = [0, 1].map(|write| {
                let slot = self.units_slot(b, t, n.clone(), (emit, wake, base, write));
                self.units_bind(b, kit, t, slot)
            });
            let (jobs, items) = (b.groups(left.clone().div(E::C(64))), b.groups(left.clone().mul(E::C(zones)).div(E::C(64))));
            let unit = |k| (PassId::Units, k);
            b.dispatch(unit(INIT), counted.clone(), jobs);
            for _ in 0..rounds {
                if wake != 0 {
                    b.dispatch(unit(WOKE), counted.clone(), jobs);
                }
                b.dispatch(unit(DECIDE), counted.clone(), jobs);
                b.dispatch(unit(ZONES), counted.clone(), items);
                b.dispatch(unit(NODES), counted.clone(), items);
                if emit != 0 {
                    b.dispatch(unit(NODES), written.clone(), items);
                }
                b.dispatch(unit(AFTER), counted.clone(), jobs);
            }
            b.dispatch(unit(CLOSE), counted, jobs);
        };
        assert!(range <= RANGE, "a range the jobs' state holds");
        for emit in 0..2 {
            for r in 0..ranges {
                let left = live.clone().sub(E::C(r * range)).min(E::C(range));
                b.enter(Cond::Units(r));
                pass(b, (&live, emit, 0, r * range), &left);
                b.leave();
            }
            if emit == 0 {
                scan(b, (a, 4, T_UNITS), (&live, Scan::Units));
                scan(b, (b_at, 4, T_UNITS_B), (&live, Scan::Units));
            }
        }
        let flags = self.units_slot(b, t, sleepers.clone(), (0, 1, 0, 0));
        let flags = self.units_bind(b, kit, t, flags);
        let over = b.groups(sleepers.clone().div(E::C(64)));
        b.dispatch((PassId::Units, FLAG), flags.clone(), over);
        scan(b, (a, 1, T_LIST), (&sleepers, Scan::Sleep));
        b.dispatch((PassId::Units, GATHER), flags, over);
        pass(b, (&sleepers, 0, 1, 0), &sleepers);
        scan(b, (a, 4, T_WAKE), (&sleepers, Scan::Sleep));
        scan(b, (b_at, 4, T_WAKE_B), (&sleepers, Scan::Sleep));
        pass(b, (&sleepers, 1, 1, 0), &sleepers);
        b.tally(lists, 0, c::MADE, 4 * 24);
        self.plan_made(b, kit, t);
        Ok(vec![sums])
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
        b.flag(Flag::UnitRounds, made(T_ROUNDS));
        let w = glue::Words { live_stride: self.sums.live_stride, ..glue::Words::default() };
        let born = [(0, both(0)), (11, E::At(c::AXES0))];
        self.glue_step(b, kit, (glue::BORN, &self.spare, &self.flags), w, &born);
        let fresh = [(0, both(1)), (11, E::At(c::PHYTS0))];
        self.glue_step(b, kit, (glue::FRESH, &self.spare, &self.flags), w, &fresh);
    }
}
