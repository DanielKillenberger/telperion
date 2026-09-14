//! Environment and bud decisions use only the tree present at slice start.
use super::*;
use crate::tree::BudFate;

/// Limit independent subtree cuts, in identity order; descendants count once.
const MAX_SHEDS: usize = 32;
impl Specimen {
    pub(super) fn environment(&mut self, slice: u64) -> Vec<NodeIdentity> {
        let t = self.timeline.as_mut().unwrap();
        let envelope = t.envelope;
        t.crown.prepare(envelope);
        let year = (slice - 1) as f64;
        let threshold = self.params.habit.shedding_threshold;
        let tolerance = t.traits.shedding_tolerance.ceil().max(1.0) as u64;
        if threshold == 0.0 {
            return Vec::new();
        }
        let mut order: Vec<_> = (0..self.tree.nodes.len())
            .filter(|&i| self.tree.nodes[i].shoot.death_year.is_none())
            .collect();
        order.sort_unstable_by_key(|&i| self.tree.nodes[i].identity);
        order.dedup();
        let mut vigour = vec![0.0_f64; self.tree.nodes.len()];
        for &i in &order {
            let n = &self.tree.nodes[i];
            let exposure = t.crown.exposure(n);
            let age = (year - n.shoot.birth_year).max(0.0);
            vigour[i] = t.traits.vigour(exposure, age);
        }
        // An illuminated descendant supports its path to the root. The maximum
        // avoids making a large old branch dark merely because it stopped extending.
        for &i in order.iter().rev() {
            if let Some(parent) = self.tree.nodes[i].parent {
                vigour[parent as usize] = vigour[parent as usize].max(vigour[i]);
            }
        }
        let mut roots = Vec::new();
        let mut removed = vec![false; self.tree.nodes.len()];
        for i in order {
            let n = &mut self.tree.nodes[i];
            // Equality survives; only strictly below the threshold accumulates.
            let low_slices = if vigour[i] < threshold {
                n.shoot.low_slices() + 1
            } else {
                0
            };
            n.shoot.record_vigour(slice, vigour[i], low_slices);
            if self.read_active {
                self.read_updates.push(i);
            }
            let covered = n.parent.is_some_and(|p| removed[p as usize]);
            let origin = if n.kind == NodeKind::Structural {
                n.shoot.bud_fate == BudFate::Lateral
            } else {
                n.branch as usize == i
            };
            let cut =
                i != 0 && origin && !covered && roots.len() < MAX_SHEDS && low_slices >= tolerance;
            removed[i] = covered || cut;
            if cut {
                roots.push(n.identity);
            }
        }
        roots
    }

    /// With survival disabled, vigour cannot gate a bud. Sample only shoots
    /// actually visited, against the immutable slice-start crown prepared above.
    pub(super) fn sample_frontier(&mut self, slice: u64) {
        if self.params.habit.shedding_threshold > 0.0 {
            return;
        }
        let mut visited: Vec<_> = self
            .scaffold
            .visited()
            .chain(self.local.visited())
            .collect();
        visited.sort_unstable_by_key(|&i| self.tree.nodes[i].identity);
        visited.dedup();
        let t = self.timeline.as_mut().unwrap();
        let year = (slice - 1) as f64;
        for i in visited {
            let n = &mut self.tree.nodes[i];
            let exposure = t.crown.exposure(n);
            let vigour = t
                .traits
                .vigour(exposure, (year - n.shoot.birth_year).max(0.0));
            n.shoot.record_vigour(slice, vigour, 0);
            if self.read_active {
                self.read_updates.push(i);
            }
        }
    }
}
