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
    order: Vec<u32>,
    /// The axes born on each held axis; and the axes let go, by their
    /// depth and by the axes born on them (fn-333).
    born: Vec<u32>,
    gone: usize,
    gone_depth: BTreeMap<u32, u64>,
    gone_born: BTreeMap<u32, u64>,
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
    /// The most units one living apex's job and one waking bud's grew in
    /// the cycle (fn-276).
    pub jobs: [u32; 2],
    pub occupied: usize,
    /// The living apexes by order, the last row all the deeper (fn-323).
    pub by_order: [usize; 8],
    /// Of those, the apexes on axes under 3 phytomers, counted apart in a
    /// report and never in its cost (fn-323's manifest).
    pub short_by_order: [usize; 8],
    /// Each living tip's presence times the share its window reads at the
    /// next cycle's start (fn-356): its 1st, 10th, 25th, 50th and 90th
    /// percentiles, and the tips under the grain, which end then.
    pub grain: [f64; 5],
    pub under_grain: usize,
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
    pub(in crate::grow) fn cycle(
        &mut self,
        (cycle, grown): (u32, u64),
        axes: &[Axis],
        (live, reading): (&[Apex], &[f64]),
    ) {
        self.extend(axes);
        let mut c = self.lattice.take().unwrap_or_default();
        (c.cycle, c.grown) = (cycle, grown);
        (c.active, c.axes) = (live.len(), self.gone + axes.len());
        c.phytomers = axes.iter().map(|a| a.phytomers.len()).sum();
        for a in live {
            let row = (self.order[a.axis] as usize).min(7);
            c.by_order[row] += 1;
            c.short_by_order[row] += usize::from(axes[a.axis].phytomers.len() < 3);
        }
        let mut read = reading.to_vec();
        read.sort_by(f64::total_cmp);
        let at = |q: f64| read.get(((read.len() as f64 - 1.0) * q).round() as usize);
        c.grain = [0.01, 0.1, 0.25, 0.5, 0.9].map(|q| at(q).copied().unwrap_or(f64::NAN));
        c.under_grain = read.partition_point(|&v| v < crate::presence::GRAIN);
        let busiest = self.cycles.iter().map(|c| c.active).max().unwrap_or(0);
        if live.len() > busiest {
            let d: Vec<u32> = live.iter().map(|a| self.depth[a.axis]).collect();
            (self.depth_busy, self.busy) = (quantiles(&counted(&d)), cycle);
        }
        self.cycles.push(c);
    }

    /// The grown hierarchy's shape at the end of growth: the axes held
    /// and those let go (`compact.rs`) alike.
    pub(in crate::grow) fn ended(&mut self, axes: &[Axis]) {
        self.extend(axes);
        self.axes = self.gone + axes.len();
        let mut depth = self.gone_depth.clone();
        for &d in &self.depth {
            *depth.entry(d).or_default() += 1;
        }
        self.depth_all = quantiles(&depth);
        let mut bearing = self.gone_born.clone();
        let born = self.born.iter().filter(|&&b| b > 0);
        for &b in born {
            *bearing.entry(b).or_default() += 1;
        }
        self.attachments = quantiles(&bearing);
    }

    /// Each held axis's records moved to its new index, `held(i)`; those
    /// let go counted by their depth and the axes born on them, which
    /// are final, for nothing is born on a closed subtree.
    pub(super) fn compact(&mut self, held: &dyn Fn(usize) -> bool) {
        for (i, (&d, &b)) in self.depth.iter().zip(&self.born).enumerate() {
            if !held(i) {
                self.gone += 1;
                *self.gone_depth.entry(d).or_default() += 1;
                if b > 0 {
                    *self.gone_born.entry(b).or_default() += 1;
                }
            }
        }
        for v in [&mut self.depth, &mut self.order, &mut self.born] {
            let mut i = 0;
            v.retain(|_| (held(i), i += 1).0);
        }
    }

    /// What it holds (fn-333 R1).
    pub(super) fn footprint(&self, f: &mut super::footprint::Footprint) {
        f.vec("census.depth", &self.depth);
        f.vec("census.order", &self.order);
        f.vec("census.born", &self.born);
        f.vec("census.cycles", &self.cycles);
    }

    fn extend(&mut self, axes: &[Axis]) {
        for a in &axes[self.depth.len()..] {
            let d = a.origin.parent().map_or(0, |p| self.depth[p] + 1);
            self.depth.push(d);
            let lateral = matches!(a.origin, crate::structure::Origin::Lateral { .. });
            let order = a
                .origin
                .parent()
                .map_or(0, |p| self.order[p] + u32::from(lateral));
            self.order.push(order);
            self.born.push(0);
            if let Some(p) = a.origin.parent() {
                self.born[p] += 1;
            }
        }
    }
}

/// p50, p99 and the most of the values counted in `counts`, as sorting
/// them all would read them.
fn quantiles(counts: &BTreeMap<u32, u64>) -> [u32; 3] {
    let n: u64 = counts.values().sum();
    if n == 0 {
        return [0; 3];
    }
    let at = |k: u64| {
        let mut seen = 0;
        for (&v, &c) in counts {
            seen += c;
            if seen > k {
                return v;
            }
        }
        unreachable!("k below the count")
    };
    let q = |q: f64| at(((n - 1) as f64 * q).round() as u64);
    [q(0.5), q(0.99), at(n - 1)]
}

/// The values of `values`, counted.
fn counted(values: &[u32]) -> BTreeMap<u32, u64> {
    let mut counts = BTreeMap::new();
    for &v in values {
        *counts.entry(v).or_default() += 1;
    }
    counts
}
