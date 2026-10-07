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
use super::partition::Run;
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
    /// Each cut's class of every axis classed so far: its fine root.
    root_of: Vec<(f64, Vec<Option<usize>>)>,
    growth: Vec<Vec<[f64; 2]>>,
    /// Probe (a): the partitioned runs compared beside the tree, each
    /// one's first difference (cycle, phase), the axis each of the
    /// cycle's leaves stands on, and each cycle's state hashed after each
    /// phase.
    pub runs: Vec<Run>,
    pub first: Vec<Option<(u32, usize)>>,
    pub leaf_axes: Vec<usize>,
    pub phases: Vec<[u64; 4]>,
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
    let oracle = Oracle::new(expected, cut, read);
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

impl Oracle {
    pub(super) fn new(expected: &Expected, cut: f64, read: Read) -> Self {
        Self {
            rows: expected.rows(),
            cut,
            read,
            root_of: Vec::new(),
            growth: Vec::new(),
            runs: Vec::new(),
            first: Vec::new(),
            leaf_axes: Vec::new(),
            phases: Vec::new(),
        }
    }

    /// The fine root axis `i` stands in at the oracle's cut.
    pub(super) fn root(&mut self, axes: &[Axis], i: usize) -> Option<usize> {
        self.root_at(axes, i, self.cut)
    }

    /// The fine root axis `i` stands in at `cut`, the axes classed first.
    pub(super) fn root_at(&mut self, axes: &[Axis], i: usize, cut: f64) -> Option<usize> {
        let k = match self.root_of.iter().position(|c| c.0 == cut) {
            Some(k) => k,
            None => {
                self.root_of.push((cut, Vec::new()));
                self.root_of.len() - 1
            }
        };
        let (cut, classes) = &mut self.root_of[k];
        classify(&self.rows, *cut, classes, axes);
        classes[i]
    }

    /// Records phase `k`'s hash in `cycle`.
    pub(super) fn mark(&mut self, cycle: u32, k: usize, h: u64) {
        let c = cycle as usize;
        if self.phases.len() <= c {
            self.phases.resize(c + 1, [0; 4]);
        }
        self.phases[c][k] = h;
    }

    /// Rewrites what each living apex below the cut reads.
    pub(super) fn reread(&mut self, axes: &[Axis], live: &[Apex], tips: &[Vec3], read: &mut [f64]) {
        if self.read == Read::Exact {
            return;
        }
        let mut by_root: HashMap<usize, Vec<usize>> = HashMap::new();
        for (k, apex) in live.iter().enumerate() {
            if let Some(r) = self.root(axes, apex.axis) {
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

/// Classes the axes made since the last call (parents first): a lateral
/// whose expected kept phytomers are at most `cut` roots a fine subtree,
/// and every axis on one stands in it.
pub(super) fn classify(rows: &Rows, cut: f64, classes: &mut Vec<Option<usize>>, axes: &[Axis]) {
    for axis in &axes[classes.len()..] {
        let i = classes.len();
        let inherited = axis.origin.parent().and_then(|p| classes[p]);
        let own = matches!(axis.origin, Origin::Lateral { .. })
            && rows.lateral(axis.pa, axis.birth).count <= cut;
        classes.push(inherited.or(own.then_some(i)));
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
    /// Under an oracle, hashes phase `k` of `cycle` (`partition::Digest`).
    pub(super) fn hash_phase(&mut self, cycle: u32, k: usize) {
        if self.oracle.is_none() {
            return;
        }
        let h = self.digest(k).finish(k);
        self.oracle.as_mut().expect("an oracle").mark(cycle, k, h);
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

    /// Probe (a): with no run beside it the partitioned run is today's
    /// tree; every run's grouping of the three sums is the tree's own.
    #[test]
    fn a_partition_regroups_only_the_tree_wide_sums() {
        use crate::grow::partition::{grow_partitioned, Grouping, Order, Run};
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
        let grouping = Grouping {
            order: Order::Reverse,
            chunk: 7,
            sizes: true,
            leaves: true,
            girth: true,
        };
        let runs = [256.0, 1024.0].map(|cut| Run { cut, grouping });
        let probed = grow_partitioned(&species, request, &expected, &runs).unwrap();
        assert_eq!(
            crate::fine::digest(&probed.structure),
            crate::fine::digest(&today)
        );
        assert_eq!(probed.first, vec![None, None]);
        assert_eq!(probed.phases.len(), 25);
    }
}
