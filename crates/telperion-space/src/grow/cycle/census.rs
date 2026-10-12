//! COUNCIL.md's early tests 1 and 2 on the one cycle: what each cycle
//! holds (active apexes against the axes and phytomers kept as history),
//! the lattice it deposits (its box and the nodes a leaf reached), every
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
    /// The subtrees retired as the tree grew.
    pub retired: usize,
    depth: Vec<u32>,
    lattice: Option<Cycle>,
}

/// One cycle's records and lattice.
#[derive(Debug, Clone, Copy, Default)]
pub struct Cycle {
    pub cycle: u32,
    pub active: usize,
    pub axes: usize,
    pub phytomers: usize,
    /// The phytomers grown by the cycle's end, the retired ones included:
    /// `phytomers` against it is the live set's share (fn-256 task 5).
    pub grown: u64,
    pub leaves: usize,
    pub size: [u32; 3],
    pub nodes: usize,
    pub occupied: usize,
}

impl Census {
    /// The cycle's lattice and its leaves.
    pub(in crate::grow) fn lattice(&mut self, field: &Field, leaves: usize) {
        self.lattice = Some(Cycle {
            leaves,
            size: field.size(),
            nodes: field.nodes(),
            occupied: field.occupied(),
            ..Cycle::default()
        });
    }

    /// The records at the end of `cycle`.
    pub(in crate::grow) fn cycle(&mut self, (cycle, grown): (u32, u64), axes: &[Axis], live: &[Apex]) {
        self.extend(axes);
        let mut c = self.lattice.take().unwrap_or_default();
        (c.cycle, c.grown) = (cycle, grown);
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
