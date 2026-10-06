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
use super::{run_beside, Apex, Beside, Options};
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
    let oracle = Oracle {
        rows: expected.rows(),
        cut,
        read,
        root_of: Vec::new(),
        growth: Vec::new(),
    };
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
}
