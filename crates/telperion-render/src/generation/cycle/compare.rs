//! The device's cycle read back and held to the CPU's: every axis's
//! columns by index, each axis's new units and phytomers in order, the
//! sleeping buds by the cycle they wake in, the living apexes and the
//! woken buds in order, and the phytomers grown. Every word is compared
//! exactly; a mismatch is counted by field and its first row named.
use super::columns::{Cap, Caps, Columns, SLEEPERS, TALLY, WOKEN};
use super::device::{Run, T_LIST, T_UNITS, T_UNITS_B, T_WAKE, T_WAKE_B};
use crate::{Gpu, Result};

/// A run's buffers as the device left them.
pub(super) struct Readback {
    pub caps: Caps,
    pub axes: Vec<u32>,
    pub units: Vec<u32>,
    pub phyts: Vec<u32>,
    pub lists: Vec<u32>,
}

fn words(gpu: &Gpu, b: &wgpu::Buffer) -> Result<Vec<u32>> {
    let raw = super::super::io::read(gpu, b, b.size())?;
    Ok(bytemuck::cast_slice(&raw).to_vec())
}

impl Run {
    pub(super) fn read(&self, gpu: &Gpu) -> Result<Readback> {
        Ok(Readback {
            caps: self.caps,
            axes: words(gpu, &self.axes)?,
            units: words(gpu, &self.units)?,
            phyts: words(gpu, &self.phyts)?,
            lists: words(gpu, &self.lists)?,
        })
    }
}

/// What the cycle appended, by the scans' totals: axes, phytomers,
/// units, sleeping buds, living apexes and woken buds, then its faults
/// and the buds that woke.
#[derive(Debug, Clone, Copy, Default)]
pub(super) struct Made {
    pub axes: u32,
    pub phyts: u32,
    pub units: u32,
    pub sleepers: u32,
    pub next: u32,
    pub woken: u32,
    pub faults: u32,
    pub waking: u32,
}

impl Readback {
    pub(super) fn made(&self) -> Made {
        let t = |k: usize| self.lists[k];
        Made {
            axes: t(T_UNITS) + t(T_WAKE),
            phyts: t(T_UNITS + 1) + t(T_WAKE + 1),
            units: t(T_UNITS + 2) + t(T_WAKE + 2),
            sleepers: t(T_UNITS + 3) + t(T_WAKE + 3),
            next: t(T_UNITS_B) + t(T_WAKE_B),
            woken: t(T_WAKE_B + 1),
            faults: t(T_UNITS_B + 2) | t(T_WAKE_B + 2),
            waking: t(T_LIST),
        }
    }

    fn col(&self, from: &[u32], cap: u32, k: usize, rows: std::ops::Range<u32>) -> Vec<u32> {
        let at = k * cap as usize;
        from[at + rows.start as usize..at + rows.end as usize].to_vec()
    }
}

/// The rows each arena needs to hold what the cycle made.
pub(super) fn needed(cols: &Columns, m: Made) -> Caps {
    Caps {
        axes: cols.axes + m.axes,
        units: cols.units + m.units,
        phyts: m.phyts,
        sleepers: cols.sleepers + m.sleepers,
        next: m.next,
        woken: m.woken,
    }
}

/// Each field compared: its rows, those that differ, and the first.
#[derive(Default)]
pub(super) struct Agreement {
    pub fields: Vec<(String, usize, usize, String)>,
}

impl Agreement {
    pub(super) fn eq<T: PartialEq + std::fmt::Debug>(&mut self, name: &str, dev: &[T], cpu: &[T]) {
        let differ = dev.iter().zip(cpu).filter(|(a, b)| a != b).count();
        let length = dev.len() != cpu.len();
        let rows: Vec<String> = (dev.iter().zip(cpu).enumerate())
            .filter(|(_, (a, b))| a != b)
            .take(4)
            .map(|(k, (a, b))| format!("row {k}: device {a:?}, cpu {b:?}"))
            .collect();
        let mut first = rows.join("; ");
        if length {
            first += &format!(" (device {} rows, cpu {})", dev.len(), cpu.len());
        }
        self.fields.push((
            name.to_string(),
            cpu.len(),
            differ + usize::from(length),
            first,
        ));
    }

    pub fn exact(&self) -> bool {
        self.fields.iter().all(|f| f.2 == 0)
    }

    pub fn report(&self) -> String {
        let mut out = String::new();
        for (name, rows, differ, first) in &self.fields {
            out += &format!("{name:>24} {rows:>10} rows {differ:>8} differ {first}\n");
        }
        out
    }
}

/// Indices of `keys` in a stable order by key: a list kept per key.
fn by_key(keys: &[u32]) -> Vec<usize> {
    let mut order: Vec<usize> = (0..keys.len()).collect();
    order.sort_by_key(|&k| keys[k]);
    order
}

fn pick(v: &[u32], order: &[usize]) -> Vec<u32> {
    order.iter().map(|&k| v[k]).collect()
}

fn lo_hi(v: &[u64]) -> Vec<u32> {
    v.iter()
        .flat_map(|&x| [x as u32, (x >> 32) as u32])
        .collect()
}

/// The device's cycle against the CPU's.
pub(super) fn compare(c: &Cap, cols: &Columns, out: &Readback) -> Agreement {
    let mut a = Agreement::default();
    let m = out.made();
    a.eq("faults", &[m.faults], &[0]);
    let n = cols.axes + m.axes;
    let cpu_n = c.u32s("out_pa").len() as u32;
    a.eq("axes", &[n], &[cpu_n]);
    let grown = c.u64s("in_grown")[0] + u64::from(m.phyts);
    a.eq("grown", &[grown], &c.u64s("out_grown"));
    let n = n.min(cpu_n).min(out.caps.axes);
    axes(&mut a, c, out, n);
    units(&mut a, c, cols, out, m);
    phytomers(&mut a, c, out, m);
    sleepers(&mut a, c, cols, out, m);
    lists(&mut a, c, out, m);
    a
}

fn axes(a: &mut Agreement, c: &Cap, out: &Readback, n: u32) {
    let col = |k: usize| out.col(&out.axes, out.caps.axes, k, 0..n);
    let pair = |k: usize| -> Vec<u32> {
        let (lo, hi) = (col(k), col(k + 1));
        lo.iter().zip(&hi).flat_map(|(&l, &h)| [l, h]).collect()
    };
    let cpu = |name: &str| c.u32s(&format!("out_{name}"))[..n as usize].to_vec();
    let cpu64 = |name: &str| lo_hi(&c.u64s(&format!("out_{name}"))[..n as usize]);
    for (k, name) in [
        (1, "pa"),
        (2, "birth"),
        (3, "origin"),
        (4, "parent"),
        (5, "node"),
    ] {
        a.eq(name, &col(k), &cpu(name));
    }
    for (k, name) in [
        (6, "lineage"),
        (8, "root"),
        (10, "horizon"),
        (12, "whorl"),
        (14, "blend"),
    ] {
        a.eq(name, &pair(k), &cpu64(name));
    }
    for (k, name) in [(16, "apex_end"), (17, "phyt_n"), (18, "spent")] {
        a.eq(name, &col(k), &cpu(name));
    }
    a.eq("b0", &pair(19), &cpu64("b0"));
    a.eq("b1", &pair(21), &cpu64("b1"));
    let scalars = [
        (23, "flags"),
        (24, "below"),
        (25, "outlived"),
        (26, "ended_relays"),
        (27, "aged"),
        (28, "sleep"),
        (29, "units_n"),
        (30, "grown_ticks"),
        (31, "succ0"),
        (32, "succ1"),
    ];
    for (k, name) in scalars {
        a.eq(name, &col(k), &cpu(name));
    }
}

fn units(a: &mut Agreement, c: &Cap, cols: &Columns, out: &Readback, m: Made) {
    let rows = cols.units..(cols.units + m.units).min(out.caps.units);
    let col = |k: usize| out.col(&out.units, out.caps.units, k, rows.clone());
    let order = by_key(&col(0));
    let dev = |k: usize| pick(&col(k), &order);
    let cpu = |name: &str| c.u32s(&format!("out_unit_{name}"));
    a.eq("unit axis", &dev(0), &cpu("axis"));
    a.eq("unit survive", &dev(1), &cpu("survive"));
    a.eq("unit persist", &dev(2), &cpu("persist"));
    a.eq("unit share", &dev(3), &cpu("share"));
    a.eq("unit size", &dev(4), &cpu("size"));
    let shade: Vec<Vec<u32>> = (5..9).map(dev).collect();
    let shade: Vec<u32> = (0..order.len())
        .flat_map(|j| shade.iter().map(move |s| s[j]))
        .collect();
    a.eq("unit shade", &shade, &lo_hi(&c.u64s("out_unit_shade")));
}

fn phytomers(a: &mut Agreement, c: &Cap, out: &Readback, m: Made) {
    let rows = 0..m.phyts.min(out.caps.phyts);
    let col = |k: usize| out.col(&out.phyts, out.caps.phyts, k, rows.clone());
    let order = by_key(&col(2));
    let dev = |k: usize| pick(&col(k), &order);
    let key: Vec<u32> = dev(0)
        .iter()
        .zip(&dev(1))
        .flat_map(|(&l, &h)| [l, h])
        .collect();
    a.eq("phytomer axis", &dev(2), &c.u32s("out_phyt_axis"));
    a.eq("phytomer key", &key, &lo_hi(&c.u64s("out_phyt_key")));
    a.eq("phytomer cycle", &dev(3), &c.u32s("out_phyt_cycle"));
    a.eq("phytomer node", &dev(4), &c.u32s("out_phyt_node"));
}

fn sleepers(a: &mut Agreement, c: &Cap, cols: &Columns, out: &Readback, m: Made) {
    let lists = &out.lists[TALLY..];
    let rows = cols.sleepers..(cols.sleepers + m.sleepers).min(out.caps.sleepers);
    let col = |k: usize| out.col(lists, out.caps.sleepers, k, rows.clone());
    let order = by_key(&col(0));
    let dev = |k: usize| pick(&col(k), &order);
    let names = [
        "wake", "parent", "node", "unit", "slot", "pa", "own", "sleep", "slept",
    ];
    for (k, name) in names.iter().enumerate() {
        a.eq(
            &format!("sleeper {name}"),
            &dev(k),
            &c.u32s(&format!("out_sleep_{name}")),
        );
    }
    for (k, name) in [(9, "whorl"), (11, "key")] {
        let both: Vec<u32> = dev(k)
            .iter()
            .zip(&dev(k + 1))
            .flat_map(|(&l, &h)| [l, h])
            .collect();
        a.eq(
            &format!("sleeper {name}"),
            &both,
            &lo_hi(&c.u64s(&format!("out_sleep_{name}"))),
        );
    }
}

fn lists(a: &mut Agreement, c: &Cap, out: &Readback, m: Made) {
    let caps = out.caps;
    let next_at = TALLY + SLEEPERS * caps.sleepers as usize;
    let n = m.next.min(caps.next) as usize;
    let next = &out.lists[next_at..];
    a.eq("live axis", &next[..n], &c.u32s("out_live_axis"));
    let spent = &next[caps.next as usize..caps.next as usize + n];
    a.eq("live spent", spent, &c.u32s("out_live_spent"));
    let woken_at = next_at + 2 * caps.next as usize;
    let woken = &out.lists[woken_at..];
    let w = m.woken.min(caps.woken) as usize;
    let names = ["axis", "bearer", "next", "cycle", "share"];
    for (k, name) in names.iter().enumerate().take(WOKEN) {
        let at = k * caps.woken as usize;
        a.eq(
            &format!("woken {name}"),
            &woken[at..at + w],
            &c.u32s(&format!("out_woken_{name}")),
        );
    }
}
