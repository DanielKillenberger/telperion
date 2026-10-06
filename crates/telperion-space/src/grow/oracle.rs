//! fn-211.4's light oracle (host, 2026-10-06): today's tree grown under
//! today's exact light field, except that every bud below a cut reads a
//! summary of its fine root's light that cycle, so the effect on growth
//! and look of what a fine root's record could carry is measured before
//! the record is built. A fine root is a lateral whose bearer stands above
//! the cut and whose expected kept phytomers (the closed form's `count`,
//! light-neutral) are at most the cut; its descendants stand in it.
//!
//! - `Read::Exact`: every bud reads its own light (today's tree, to the
//!   bit), only the growth is tallied.
//! - `Read::Mean`: a fine root's buds read their mean (option C's best).
//! - `Read::Slopes`: they read that mean plus its least-squares slopes in
//!   their tips' positions, fitted that cycle (option B's best).
//!
//! An experiment behind its own entry point: `grow` never builds one.
use super::partition::Grouping;
use super::{run_beside, Apex, Beside, Grower, Options};
use crate::bounded::Expected;
use crate::closed_form::boundary::Rows;
use crate::error::Result;
use crate::grow::Request;
use crate::species::Species;
use crate::structure::{Axis, Origin, Structure, Vec3};
use std::collections::HashMap;
use std::sync::Arc;

/// What a bud below the cut reads.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Read {
    Exact,
    Mean,
    Slopes,
}

/// The oracle beside the grower: the cut, each axis's fine root, and per
/// cycle and PA the growing buds' presence and presence times size.
pub(crate) struct Oracle {
    rows: Arc<Rows>,
    cut: f64,
    read: Read,
    root_of: Vec<Option<usize>>,
    growth: Vec<Vec<[f64; 2]>>,
    /// Probe (a): how the whole-tree reductions are grouped (none: as
    /// today), the axis each of the cycle's leaves stands on, and each
    /// cycle's state hashed after each phase.
    pub grouping: Option<Grouping>,
    pub leaf_axes: Vec<usize>,
    phases: Vec<[u64; 4]>,
}

/// A tree grown under the oracle, and its growth per cycle and PA.
pub struct Oracled {
    pub structure: Structure,
    pub growth: Vec<Vec<[f64; 2]>>,
}

/// Grows `species` as `grow` does, its buds below `cut` reading `read`.
pub fn grow_oracle(
    species: &Species,
    request: Request,
    expected: &Expected,
    cut: f64,
    read: Read,
) -> Result<Oracled> {
    let oracle = Oracle::new(expected, cut, read, None);
    let beside = Beside {
        oracle: Some(oracle),
        ..Beside::default()
    };
    let quiet = &mut |_| {};
    let (grown, besides) = run_beside(species, request, (quiet, true), Options::default(), beside)?;
    Ok(Oracled {
        structure: grown.structure,
        growth: besides.oracle.map_or_else(Vec::new, |o| o.growth),
    })
}

/// A tree grown as probe (a) partitions it, and its state hashed after
/// each phase of each cycle.
pub struct Partitioned {
    pub structure: Structure,
    pub phases: Vec<[u64; 4]>,
}

/// Grows `species` as `grow` does, every whole-tree reduction grouped as
/// a partition at `cut` takes it (`grouping`; none, today's), hashing
/// each phase.
pub fn grow_partitioned(
    species: &Species,
    request: Request,
    expected: &Expected,
    cut: f64,
    grouping: Option<Grouping>,
) -> Result<Partitioned> {
    let oracle = Oracle::new(expected, cut, Read::Exact, grouping);
    let beside = Beside {
        oracle: Some(oracle),
        ..Beside::default()
    };
    let quiet = &mut |_| {};
    let (grown, besides) = run_beside(species, request, (quiet, true), Options::default(), beside)?;
    Ok(Partitioned {
        structure: grown.structure,
        phases: besides.oracle.map_or_else(Vec::new, |o| o.phases),
    })
}

impl Oracle {
    fn new(expected: &Expected, cut: f64, read: Read, grouping: Option<Grouping>) -> Self {
        Self {
            rows: expected.rows(),
            cut,
            read,
            root_of: Vec::new(),
            growth: Vec::new(),
            grouping,
            leaf_axes: Vec::new(),
            phases: Vec::new(),
        }
    }

    /// The fine root axis `i` stands in, the axes classed first.
    pub(super) fn root(&mut self, axes: &[Axis], i: usize) -> Option<usize> {
        self.classify(axes);
        self.root_of[i]
    }

    /// Records phase `k`'s hash in `cycle`.
    pub(super) fn mark(&mut self, cycle: u32, k: usize, h: u64) {
        let c = cycle as usize;
        if self.phases.len() <= c {
            self.phases.resize(c + 1, [0; 4]);
        }
        self.phases[c][k] = h;
    }

    /// Classes the axes made since the last call (parents first).
    fn classify(&mut self, axes: &[Axis]) {
        for axis in &axes[self.root_of.len()..] {
            let i = self.root_of.len();
            let inherited = axis.origin.parent().and_then(|p| self.root_of[p]);
            let own = matches!(axis.origin, Origin::Lateral { .. })
                && self.rows.lateral(axis.pa, axis.birth).count <= self.cut;
            self.root_of.push(inherited.or(own.then_some(i)));
        }
    }

    /// Rewrites what each living apex below the cut reads.
    pub(super) fn reread(&mut self, axes: &[Axis], live: &[Apex], tips: &[Vec3], read: &mut [f64]) {
        self.classify(axes);
        if self.read == Read::Exact {
            return;
        }
        let mut by_root: HashMap<usize, Vec<usize>> = HashMap::new();
        for (k, apex) in live.iter().enumerate() {
            if let Some(r) = self.root_of[apex.axis] {
                by_root.entry(r).or_default().push(k);
            }
        }
        for buds in by_root.values() {
            let seen: Vec<(f64, Vec3)> = buds.iter().map(|&k| (read[k], tips[k])).collect();
            let summary = match self.read {
                Read::Mean => vec![mean(&seen); seen.len()],
                _ => fitted(&seen),
            };
            for (&k, s) in buds.iter().zip(summary) {
                read[k] = s;
            }
        }
    }

    /// Adds `cycle`'s growing buds: each one's PA, presence and size.
    pub(super) fn tally(&mut self, cycle: u32, buds: impl Iterator<Item = (usize, f64, f64)>) {
        for (pa, p, size) in buds.filter(|b| b.1 > 0.0) {
            super::subtree::tally(&mut self.growth, cycle, pa, p, size);
        }
    }
}

fn mean(seen: &[(f64, Vec3)]) -> f64 {
    seen.iter().map(|s| s.0).sum::<f64>() / seen.len() as f64
}

/// Each bud's light as the least-squares plane through its root's buds
/// predicts it, within the light's range; a root of four buds or fewer
/// reads its own, as an exact fit would give.
fn fitted(seen: &[(f64, Vec3)]) -> Vec<f64> {
    if seen.len() <= 4 {
        return seen.iter().map(|s| s.0).collect();
    }
    let n = seen.len() as f64;
    let m = mean(seen);
    let c = seen.iter().fold(Vec3::default(), |a, s| a + s.1) * (1.0 / n);
    let (mut a, mut y) = ([[0.0f64; 3]; 3], [0.0f64; 3]);
    for s in seen {
        let d = s.1 - c;
        let x = [d.x, d.y, d.z];
        for i in 0..3 {
            y[i] += x[i] * (s.0 - m);
            for j in 0..3 {
                a[i][j] += x[i] * x[j];
            }
        }
    }
    // Damped, so a root whose buds stand in a line or a plane keeps a
    // finite slope across it.
    for (i, row) in a.iter_mut().enumerate() {
        row[i] += 1e-6 * n;
    }
    let w = solve(a, y);
    seen.iter()
        .map(|s| {
            let d = s.1 - c;
            (m + w[0] * d.x + w[1] * d.y + w[2] * d.z).clamp(0.0, 1.0)
        })
        .collect()
}

/// Gaussian elimination with partial pivoting on a 3 × 3 system.
fn solve(mut a: [[f64; 3]; 3], mut y: [f64; 3]) -> [f64; 3] {
    for c in 0..3 {
        let p = (c..3)
            .max_by(|&i, &j| a[i][c].abs().total_cmp(&a[j][c].abs()))
            .unwrap_or(c);
        a.swap(c, p);
        y.swap(c, p);
        for r in c + 1..3 {
            let f = a[r][c] / a[c][c];
            let pivot = a[c];
            for (x, q) in a[r][c..].iter_mut().zip(&pivot[c..]) {
                *x -= f * q;
            }
            y[r] -= f * y[c];
        }
    }
    let mut w = [0.0; 3];
    for c in (0..3).rev() {
        let rest: f64 = (c + 1..3).map(|j| a[c][j] * w[j]).sum();
        w[c] = (y[c] - rest) / a[c][c];
    }
    w
}

impl Grower<'_> {
    /// Each PA's Σp and Σp(v/p)^ψ, grouped as a partition at the oracle's
    /// cut takes them (probe (a)).
    pub(super) fn grouped_sums(
        &mut self,
        got: &[f64],
        (presence, psi, pa): (&[f64], &[f64], &[usize]),
        g: &Grouping,
    ) -> Vec<[f64; 2]> {
        let pas = self.species.states.len();
        let oracle = self.oracle.as_mut().expect("an oracle");
        let items: Vec<(Option<usize>, Vec<f64>)> = (0..got.len())
            .filter(|&i| presence[i] > 0.0)
            .map(|i| {
                let mut v = vec![0.0; 2 * pas];
                v[2 * pa[i]] = presence[i];
                v[2 * pa[i] + 1] = (got[i] / presence[i]).powf(psi[i]) * presence[i];
                (oracle.root(&self.axes, i), v)
            })
            .collect();
        let sums = super::partition::grouped(&items, 2 * pas, g);
        (0..pas).map(|p| [sums[2 * p], sums[2 * p + 1]]).collect()
    }

    /// The leaf-light girth's tree-wide mean for re-laid `tree` (its axes
    /// from the grower's `from`), grouped as a partition takes it; none
    /// where nothing groups it, and `thicken` takes it whole.
    pub(super) fn grouped_leaf_mean(&mut self, tree: &Structure, from: &[usize]) -> Option<f64> {
        let species = self.species;
        if species.states.iter().all(|s| s.leaf_girth == 0.0) {
            return None;
        }
        let oracle = self.oracle.as_mut()?;
        let g = oracle.grouping.filter(|g| g.girth)?;
        let mut items = Vec::new();
        for (axis, &i) in tree.axes.iter().zip(from) {
            let state = &species.states[axis.pa];
            let root = oracle.root(&self.axes, i);
            for p in &axis.phytomers {
                let pipe = crate::girth::own(&state.form, p, tree.age, p.scale, 1.0);
                items.push((root, vec![pipe, pipe * p.light.powf(state.leaf_girth)]));
            }
        }
        let sums = super::partition::grouped(&items, 2, &g);
        (sums[1] > 0.0).then(|| sums[0] / sums[1])
    }

    /// Under an oracle, hashes phase `k` of `cycle`: the living apexes'
    /// balance (kept, memory), sizes, the tree grown (axes, phytomers,
    /// living), or their light and tips.
    pub(super) fn mark(&mut self, cycle: u32, k: usize) {
        let (Some(oracle), Some(sketch)) = (self.oracle.as_mut(), self.sketch.as_ref()) else {
            return;
        };
        // Order-free: each living apex hashed with its lineage and summed,
        // so a run that numbers its axes otherwise (a stepped executor)
        // compares by the same digest.
        let live = self.live.iter().filter(|a| a.axis < sketch.pencils.len());
        let p = |a: &Apex| &sketch.pencils[a.axis];
        let keyed = |a: &Apex, v: &[f64]| {
            let lineage = f64::from_bits(self.axes[a.axis].lineage);
            super::partition::hash(std::iter::once(lineage).chain(v.iter().copied()))
        };
        let tip = |a: &Apex| {
            let axis = &self.axes[a.axis];
            axis.phytomers.last().map_or(axis.base, |q| q.tip)
        };
        let h = match k {
            0 => live
                .map(|a| keyed(a, &[p(a).kept, p(a).memory]))
                .fold(0, u64::wrapping_add),
            1 => live
                .map(|a| keyed(a, &[p(a).size]))
                .fold(0, u64::wrapping_add),
            2 => {
                let phytomers: usize = self.axes.iter().map(|a| a.phytomers.len()).sum();
                let n = [self.axes.len(), phytomers, self.live.len()];
                super::partition::hash(n.into_iter().map(|x| x as f64))
            }
            _ => live
                .map(|a| {
                    let t = tip(a);
                    keyed(a, &[p(a).light, t.x, t.y, t.z])
                })
                .fold(0, u64::wrapping_add),
        };
        oracle.mark(cycle, k, h);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_plane_is_read_back_exactly() {
        let seen: Vec<(f64, Vec3)> = (0..12)
            .map(|k| {
                let p = Vec3::new(f64::from(k % 3), f64::from(k / 3), f64::from(k % 5) * 0.5);
                (0.3 + 0.02 * p.x - 0.01 * p.y + 0.04 * p.z, p)
            })
            .collect();
        for (got, s) in fitted(&seen).iter().zip(&seen) {
            assert!((got - s.0).abs() < 1e-4, "{got} against {}", s.0);
        }
    }

    /// Probe (a): ungrouped, the partitioned run is today's tree; grouped,
    /// it keeps every phytomer and moves nothing visible.
    #[test]
    fn a_partition_regroups_only_the_tree_wide_sums() {
        use crate::grow::partition::{Grouping, Order};
        let species = crate::oak();
        let request = crate::grow::Request {
            age: 24,
            seed: 7,
            budget: 20_000_000,
            light: crate::light::Light {
                extinction: 0.5,
                sky: 0.5,
            },
        };
        let expected = Expected::new(&species, request.age, 0);
        let today = crate::grow::grow(&species, request).unwrap();
        let whole = grow_partitioned(&species, request, &expected, 1024.0, None).unwrap();
        assert_eq!(
            crate::fine::digest(&whole.structure),
            crate::fine::digest(&today)
        );
        let g = Grouping {
            order: Order::Reverse,
            chunk: 7,
            sizes: true,
            leaves: true,
            girth: true,
        };
        let part = grow_partitioned(&species, request, &expected, 1024.0, Some(g)).unwrap();
        let (a, b) = (&whole.structure, &part.structure);
        assert_eq!(a.axes.len(), b.axes.len());
        for (x, y) in a.axes.iter().zip(&b.axes) {
            assert_eq!(
                (x.lineage, x.phytomers.len()),
                (y.lineage, y.phytomers.len())
            );
            for (p, q) in x.phytomers.iter().zip(&y.phytomers) {
                assert!((p.tip - q.tip).length() < 1e-9);
            }
        }
    }
}
