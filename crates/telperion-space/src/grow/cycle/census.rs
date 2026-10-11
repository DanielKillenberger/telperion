//! COUNCIL.md's early tests 1 and 2 on the one cycle: what each cycle
//! holds (active apexes against the axes and phytomers kept as history),
//! the lattice it deposits (its box and the nodes a leaf reached), how far
//! the other deposit's light lies from the one growth reads, every
//! stage's seconds, and the shape of the grown hierarchy (each axis's
//! ancestors and attachments).
use super::super::Apex;
use crate::light::Field;
use crate::structure::Axis;
use std::collections::BTreeMap;

/// The census of one run.
#[derive(Default)]
pub struct Census {
    pub cycles: Vec<Cycle>,
    /// Seconds by the stage that ended them, the settle's included.
    pub phases: BTreeMap<String, f64>,
    /// Ancestors of every grown axis at the end, and of the living apexes
    /// in the busiest cycle, as p50, p99 and most; and that cycle.
    pub depth_all: [u32; 3],
    pub depth_busy: [u32; 3],
    pub busy: u32,
    /// Axes born on each grown axis (laterals, continuations and relays),
    /// over the axes that bear any: p50, p99 and most; and the grown axes.
    pub attachments: [u32; 3],
    pub axes: usize,
    depth: Vec<u32>,
    lattice: Option<Cycle>,
}

/// One cycle's records, lattice and the deposits' difference.
#[derive(Debug, Clone, Copy, Default)]
pub struct Cycle {
    pub cycle: u32,
    pub active: usize,
    pub axes: usize,
    pub phytomers: usize,
    pub leaves: usize,
    pub size: [u32; 3],
    pub nodes: usize,
    pub occupied: usize,
    /// Buds read, the largest and summed size of the other deposit's
    /// light less this one's, and the buds where it differs at all.
    pub buds: usize,
    pub most: f64,
    pub sum: f64,
    pub differ: usize,
}

impl Census {
    /// The cycle's lattice, its leaves, and its light at the buds beside
    /// the other deposit's.
    pub(in crate::grow) fn lattice(&mut self, field: &Field, leaves: usize, read: &[f64], other: &[f64]) {
        let mut c = Cycle {
            leaves,
            size: field.size(),
            nodes: field.nodes(),
            occupied: field.occupied(),
            buds: read.len(),
            ..Cycle::default()
        };
        for (a, b) in read.iter().zip(other) {
            let d = (b - a).abs();
            (c.most, c.sum) = (c.most.max(d), c.sum + d);
            c.differ += usize::from(d > 0.0);
        }
        self.lattice = Some(c);
    }

    /// The records at the end of `cycle`.
    pub(in crate::grow) fn cycle(&mut self, cycle: u32, axes: &[Axis], live: &[Apex]) {
        self.extend(axes);
        let mut c = self.lattice.take().unwrap_or_default();
        c.cycle = cycle;
        (c.active, c.axes) = (live.len(), axes.len());
        c.phytomers = axes.iter().map(|a| a.phytomers.len()).sum();
        let busiest = self.cycles.iter().map(|c| c.active).max().unwrap_or(0);
        if live.len() > busiest {
            let mut d: Vec<u32> = live.iter().map(|a| self.depth[a.axis]).collect();
            (self.depth_busy, self.busy) = (quantiles(&mut d), cycle);
        }
        self.cycles.push(c);
    }

    /// The grown hierarchy's shape at the end of growth.
    pub(in crate::grow) fn ended(&mut self, axes: &[Axis]) {
        self.extend(axes);
        self.axes = axes.len();
        self.depth_all = quantiles(&mut self.depth.clone());
        let mut born = vec![0u32; axes.len()];
        for a in axes {
            if let Some(p) = a.origin.parent() {
                born[p] += 1;
            }
        }
        let mut bearing: Vec<u32> = born.into_iter().filter(|&b| b > 0).collect();
        self.attachments = quantiles(&mut bearing);
    }

    fn extend(&mut self, axes: &[Axis]) {
        for a in &axes[self.depth.len()..] {
            let d = a.origin.parent().map_or(0, |p| self.depth[p] + 1);
            self.depth.push(d);
        }
    }
}

/// p50, p99 and the most of `values`.
fn quantiles(values: &mut [u32]) -> [u32; 3] {
    if values.is_empty() {
        return [0; 3];
    }
    values.sort_unstable();
    let at = |q: f64| values[((values.len() - 1) as f64 * q).round() as usize];
    [at(0.5), at(0.99), values[values.len() - 1]]
}
