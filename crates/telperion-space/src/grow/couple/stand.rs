//! Where a fine root stands in the coupling pass (fn-211.4): its stand-in
//! leaves spread over its subtree's expected extent, the point it reads
//! its light at. Each reads the closed form's row of a subtree of the root's
//! age then (its PA, born that many cycles before the tree's age), at the
//! root's base scale: light-neutral expectations, so the subtree's wood
//! and extent grow in with it.
use super::super::Grower;
use crate::closed_form::boundary::Row;
use crate::structure::Vec3;

/// The pass's last field before the tree's last cycle (the light tool,
/// fn-211.4): its leaves, marked where a stand-in cast them, and its
/// reads, each its axis, point, light and whether a fine root read it.
#[derive(Debug, Clone, Default)]
pub(crate) struct Probe {
    pub leaves: Vec<(Vec3, f64, bool)>,
    pub reads: Vec<(usize, Vec3, f64, bool)>,
}

/// A fine root's stand-in: its axis, the point it reads its light at,
/// and its leaves.
pub(crate) type StandIn = (usize, Vec3, Vec<(Vec3, f64)>);

/// The stencil a stand-in's leaves are spread over: its centroid and six
/// points on its frame's axes, each a seventh, at the distance that
/// gives the stencil its extent's spread (variance σ² a direction at
/// ±σ√(7/2)).
const STENCIL: f64 = 1.0 / 7.0;

impl Grower<'_> {
    /// A fine root's row at `cycle`: a subtree of its age then.
    fn row_at(&self, k: usize, cycle: u32) -> &Row {
        let pass = self.couple.as_ref().expect("coupling");
        let f = &pass.fine[k];
        let age = cycle.saturating_sub(f.birth).max(1);
        pass.rows.lateral(f.pa, self.rules.age.saturating_sub(age))
    }

    /// Each living fine root's stand-in leaves of `cycle`, spread over its
    /// expected extent, and the point it reads its light at (its expected
    /// centroid): `(axis, point, leaves)`.
    pub(crate) fn fine_leaves(&self, cycle: u32) -> Vec<StandIn> {
        let Some(pass) = self.couple.as_ref() else {
            return Vec::new();
        };
        let mut out = Vec::new();
        for (k, f) in pass.fine.iter().enumerate().filter(|(_, f)| !f.dead) {
            let row = self.row_at(k, cycle);
            let (base, heading, side) = self.fine_frame(f.axis);
            let at = base + heading * (row.along * f.scale);
            let area = f.scale * f.agg.leaves;
            if area <= 0.0 {
                out.push((f.axis, at, Vec::new()));
                continue;
            }
            // The spread about the centroid, from the spread about the base.
            let about = (row.spread.powi(2) - row.along.powi(2)).max(0.0).sqrt();
            let reach = about * f.scale / 3f64.sqrt() * (3.5f64).sqrt();
            let up = heading.cross(side);
            let mut leaves = vec![(at, area * STENCIL)];
            for d in [heading, side, up] {
                leaves.push((at + d * reach, area * STENCIL));
                leaves.push((at - d * reach, area * STENCIL));
            }
            out.push((f.axis, at, leaves));
        }
        out
    }

    /// Keeps the pass's last field: its leaves (those from `own` on are
    /// stand-ins') and every read, the living apexes' then the fine
    /// roots', at `tips` with `light`.
    pub(crate) fn keep_field(&mut self, own: usize, tips: &[Vec3], light: &[f64]) {
        let leaves = self.leaves.iter().enumerate();
        let leaves: Vec<_> = leaves.map(|(k, &(at, a))| (at, a, k >= own)).collect();
        let n = self.live.len();
        let pass = self.couple.as_ref().expect("coupling");
        let fine = pass.living().map(|f| f.axis);
        let axes = self.live.iter().map(|a| a.axis).chain(fine);
        let reads = axes
            .enumerate()
            .map(|(k, i)| (i, tips[k], light[k], k >= n))
            .collect();
        let pass = self.couple.as_mut().expect("coupling");
        pass.probe = Probe { leaves, reads };
    }
}
