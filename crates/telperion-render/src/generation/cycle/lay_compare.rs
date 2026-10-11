//! The device's layout read back and held to the CPU's: every word of
//! every axis's, phytomer's and pencil's columns, each new axis's
//! generation, and the leaves as a set (the integer deposit takes them in
//! any order). A mismatch is counted by column and its first rows named.
use super::compare::Agreement;
use super::columns::Cap;
use super::lay_columns::{pencil_name, Laid, AXIS, AXIS_NAMES, PENCIL, PHYT, PHYT_NAMES, TALLY};
use super::lay_device::Run;
use crate::{Gpu, Result};

/// A run's buffers as the device left them.
pub(super) struct Readback {
    pub axes: Vec<u32>,
    pub phyts: Vec<u32>,
    pub pencils: Vec<u32>,
    pub lists: Vec<u32>,
}

fn words(gpu: &Gpu, b: &wgpu::Buffer) -> Result<Vec<u32>> {
    let raw = super::super::io::read(gpu, b, b.size())?;
    Ok(bytemuck::cast_slice(&raw).to_vec())
}

impl Run {
    pub(super) fn read(&self, gpu: &Gpu) -> Result<Readback> {
        Ok(Readback {
            axes: words(gpu, &self.axes)?,
            phyts: words(gpu, &self.phyts)?,
            pencils: words(gpu, &self.pencils)?,
            lists: words(gpu, &self.lists)?,
        })
    }
}

/// What the layout did, by the CPU's capture: axes drawn and framed,
/// phytomers laid, and generations.
#[derive(Debug, Clone, Copy)]
pub(super) struct Laying {
    pub drawn: u32,
    pub framed: u32,
    pub laid: u32,
    pub generations: u32,
}

pub(super) fn laying(c: &Cap, laid: &Laid) -> Laying {
    Laying {
        drawn: laid.grown() + laid.new_axes(),
        framed: laid.new_axes(),
        laid: (c.u32s("out_leaves").len() / 4) as u32,
        generations: laid.generations.len() as u32,
    }
}

/// Each column of a table of `rows` rows compared, named by `name`.
fn table(a: &mut Agreement, (dev, cpu): (&[u32], &[u32]), rows: usize, cols: usize, name: &dyn Fn(usize) -> String) {
    for k in 0..cols {
        let at = k * rows..(k + 1) * rows;
        let d = dev.get(at.clone()).unwrap_or_default();
        a.eq(&name(k), d, &cpu[at]);
    }
}

/// The device's layout against the CPU's.
pub(super) fn compare(c: &Cap, laid: &Laid, out: &Readback) -> Agreement {
    let mut a = Agreement::default();
    let (n, rows) = (laid.axes as usize, laid.phyts as usize);
    let faults = (out.axes[(AXIS - 1) * n..AXIS * n].iter()).fold(0, |f, &x| f | x);
    a.eq("faults", &[faults], &[0]);
    let at = TALLY + laid.grew.len();
    let gens = &out.lists[at..at + laid.new_axes() as usize];
    let mut sizes = vec![0u32; gens.iter().max().map_or(0, |&g| g as usize + 1)];
    for &g in gens {
        sizes[g as usize] += 1;
    }
    a.eq("generations", &sizes, &laid.generations);
    let axis = |k: usize| format!("axis {}", AXIS_NAMES[k]);
    table(&mut a, (&out.axes, &c.u32s("out_axes")), n, AXIS, &axis);
    let phyt = |k: usize| format!("phytomer {}", PHYT_NAMES[k]);
    table(&mut a, (&out.phyts, &c.u32s("out_phyts")), rows, PHYT, &phyt);
    let pencil = |k: usize| format!("pencil {}", pencil_name(k));
    table(&mut a, (&out.pencils, &c.u32s("out_pencils")), n, PENCIL, &pencil);
    a.eq("leaves", &leaves(out, rows), &sorted(c.u32s("out_leaves")));
    a
}

/// The leaves the device laid, each its tip and area, as a sorted set.
fn leaves(out: &Readback, rows: usize) -> Vec<u32> {
    let col = |k: usize| &out.phyts[k * rows..(k + 1) * rows];
    let laid = col(PHYT - 1);
    let mut leaves = Vec::new();
    for r in (0..rows).filter(|&r| laid[r] != 0) {
        leaves.extend([col(4)[r], col(5)[r], col(6)[r], col(PHYT - 2)[r]]);
    }
    sorted(leaves)
}

/// Leaves of four words each, sorted by their words.
fn sorted(words: Vec<u32>) -> Vec<u32> {
    let mut leaves: Vec<[u32; 4]> = words.as_chunks::<4>().0.to_vec();
    leaves.sort_unstable();
    leaves.concat()
}
