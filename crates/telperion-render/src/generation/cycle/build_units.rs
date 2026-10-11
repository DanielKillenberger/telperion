//! A build cycle's units and wake (DESIGN.md sections 2.1 and 2.2) over
//! the state: the living apexes' units counted, scanned and emitted, the
//! cycle's sleeping buds compacted, their waking counted, scanned and
//! emitted, each append at its scanned place (`units.wgsl`).
use super::build::{Build, Kit, Tables};
use super::device::{record_scans, row, scan, CYCLE, FLAG, GATHER, T_LIST, T_UNITS, T_UNITS_B, T_WAKE, T_WAKE_B};
use super::hier_scan::{read_words, zeroed};
use super::state::Held;
use crate::{Gpu, Result};

/// What the units appended in a cycle, by the scans' totals.
#[derive(Debug, Clone, Copy, Default)]
pub(super) struct Made {
    pub axes: u32,
    pub phyts: u32,
    pub units: u32,
    pub sleepers: u32,
    pub next: u32,
    pub woken: u32,
    pub faults: u32,
}

impl Build {
    /// The units' uniform (`arena.wgsl`'s `Cfg`) over `n` invocations,
    /// emitting or counting, in the units or the wake, in `cycle`, with
    /// `live` living apexes read.
    pub(super) fn units_words(&self, (gpu, t): (&Gpu, &Tables), (cycle, live): (u32, u32), (n, emit, wake): (u32, u32, u32)) -> [u32; 32] {
        let (c, h) = (self.state.caps, self.held);
        [
            n,
            row(gpu, n).1,
            cycle,
            t.age,
            emit,
            wake,
            t.lights,
            0,
            c.axes,
            c.units,
            c.phyts,
            c.sleepers,
            c.next,
            c.woken,
            c.sleepers,
            self.laid,
            c.phyt_at(),
            c.sleep_at(),
            c.next_at(),
            c.woken_at(),
            c.list_at(),
            h.axes,
            h.units,
            h.phyts,
            h.sleepers,
            2 * live,
            t.rows,
            t.whorls,
            h.woken,
            0,
            t.seed as u32,
            (t.seed >> 32) as u32,
        ]
    }

    /// The units' bind under `words`, the counts sized for the state.
    pub(super) fn units_bind(&self, gpu: &Gpu, kit: &Kit, t: &Tables, words: [u32; 32]) -> Result<(wgpu::BindGroup, [crate::buffer::Owned; 3])> {
        let s = &self.state;
        let n = u64::from(s.caps.next.max(s.caps.sleepers).max(1));
        let usage = wgpu::BufferUsages::UNIFORM;
        let c = super::super::io::buffer(gpu, "cycle cfg", 128, usage, bytemuck::cast_slice(&words))?;
        let a = zeroed(gpu, "cycle counts", 4 * n)?;
        let b = zeroed(gpu, "cycle counts b", 4 * n)?;
        let all = [&*c, &t.table, &t.words, &self.inputs, &s.axes, &s.arena, &s.lists, &a, &b];
        Ok((kit.units.cycle.bind(gpu, &all), [c, a, b]))
    }

    /// The cycle's units and wake over `live` living apexes: what they
    /// made, read back.
    pub(super) fn units(&mut self, gpu: &Gpu, kit: &Kit, t: &Tables, (cycle, live): (u32, u32)) -> Result<Made> {
        let sleepers = self.held.sleepers;
        let words = |n, emit, wake| self.units_words((gpu, t), (cycle, live), (n, emit, wake));
        let (count, held) = self.units_bind(gpu, kit, t, words(live, 0, 0))?;
        let [_, a, b] = &held;
        let mut binds = Vec::new();
        for (n, emit, wake) in [(live, 1, 0), (sleepers, 0, 1), (sleepers, 1, 1)] {
            let words = words(n, emit, wake);
            let usage = wgpu::BufferUsages::UNIFORM;
            let c = super::super::io::buffer(gpu, "cycle cfg", 128, usage, bytemuck::cast_slice(&words))?;
            let s = &self.state;
            let all = [&*c, &t.table, &t.words, &self.inputs, &s.axes, &s.arena, &s.lists, a, b];
            binds.push((kit.units.cycle.bind(gpu, &all), c));
        }
        let pass = &kit.units.scan;
        let scans = [
            scan(gpu, pass, (a, live, 4), T_UNITS)?,
            scan(gpu, pass, (b, live, 4), T_UNITS_B)?,
            scan(gpu, pass, (a, sleepers, 1), T_LIST)?,
            scan(gpu, pass, (a, sleepers, 4), T_WAKE)?,
            scan(gpu, pass, (b, sleepers, 4), T_WAKE_B)?,
        ];
        let lists = &self.state.lists;
        let mut e = gpu.encoder(&Default::default());
        let k = &kit.units.cycle;
        k.run(&mut e, &count, CYCLE, row(gpu, live).0);
        record_scans(pass, &mut e, &scans[0..2], lists);
        k.run(&mut e, &binds[0].0, CYCLE, row(gpu, live).0);
        k.run(&mut e, &binds[1].0, FLAG, row(gpu, sleepers).0);
        record_scans(pass, &mut e, &scans[2..3], lists);
        k.run(&mut e, &binds[1].0, GATHER, row(gpu, sleepers).0);
        k.run(&mut e, &binds[1].0, CYCLE, row(gpu, sleepers).0);
        record_scans(pass, &mut e, &scans[3..5], lists);
        k.run(&mut e, &binds[2].0, CYCLE, row(gpu, sleepers).0);
        gpu.submit([e.finish()]);
        let t = read_words(gpu, lists, 0, 20)?;
        let made = Made {
            axes: t[T_UNITS] + t[T_WAKE],
            phyts: t[T_UNITS + 1] + t[T_WAKE + 1],
            units: t[T_UNITS + 2] + t[T_WAKE + 2],
            sleepers: t[T_UNITS + 3] + t[T_WAKE + 3],
            next: t[T_UNITS_B] + t[T_WAKE_B],
            woken: t[T_WAKE_B + 1],
            faults: t[T_UNITS_B + 2] | t[T_WAKE_B + 2],
        };
        self.held = Held {
            axes: self.held.axes + made.axes,
            units: self.held.units + made.units,
            phyts: self.held.phyts + made.phyts,
            sleepers: self.held.sleepers + made.sleepers,
            next: made.next,
            woken: self.held.woken + made.woken,
        };
        self.grown += u64::from(made.phyts);
        Ok(made)
    }
}
