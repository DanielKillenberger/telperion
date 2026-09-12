//! Environment and bud decisions use only the tree present at slice start.
use super::*;
use crate::tree::BudFate;
use std::collections::BTreeSet;

/// Limit independent subtree cuts, in identity order; descendants count once.
const MAX_SHEDS: usize = 32;
impl Specimen {
    pub(super) fn environment(&mut self, month: u64) -> Vec<NodeIdentity> {
        let t = self.timeline.as_ref().unwrap();
        let envelope = t.envelope;
        let profile = envelope.profile();
        let width = envelope.max_radius().max(1e-9);
        let year = (month - 1) as f64 / 12.0;
        let threshold = self.params.habit.shedding_threshold;
        let tolerance = (t.traits.shedding_tolerance * 12.0).ceil().max(1.0) as u64;
        let mut order: Vec<_> = (0..self.tree.nodes.len()).collect();
        order.sort_unstable_by_key(|&i| self.tree.nodes[i].identity);
        for &i in &order {
            let n = &mut self.tree.nodes[i];
            let radial = n.position.x.hypot_fixed(n.position.z);
            // The old shell's radial OR profile-distance test becomes continuous
            // exposure. Young wood has more vigour than equally shaded old wood.
            let depth = (envelope.radius_at(n.position.y) - radial)
                .min(distance_to_profile(&profile, radial, n.position.y))
                .max(0.0);
            let exposure = (1.0 - depth / width).clamp(0.0, 1.0);
            let age = (year - n.shoot.birth_year).max(0.0);
            n.shoot.vigour = exposure / (1.0 + t.traits.rate * age);
        }
        // An illuminated descendant supports its path to the root. The maximum
        // avoids making a large old branch dark merely because it stopped extending.
        for &i in order.iter().rev() {
            if let Some(parent) = self.tree.nodes[i].parent {
                let v = self.tree.nodes[i].shoot.vigour;
                let p = &mut self.tree.nodes[parent as usize].shoot;
                p.vigour = p.vigour.max(v);
            }
        }
        let mut roots = Vec::new();
        let mut removed = vec![false; self.tree.nodes.len()];
        for i in order {
            let n = &mut self.tree.nodes[i];
            // Equality survives; only strictly below the threshold accumulates.
            n.shoot.low_months = if n.shoot.vigour < threshold {
                n.shoot.low_months + 1
            } else {
                0
            };
            let covered = n.parent.is_some_and(|p| removed[p as usize]);
            let origin = if i < self.tree.crossover {
                n.shoot.bud_fate == BudFate::Lateral
            } else {
                n.branch as usize == i
            };
            let cut = i != 0
                && origin
                && !covered
                && roots.len() < MAX_SHEDS
                && n.shoot.low_months >= tolerance;
            removed[i] = covered || cut;
            if cut {
                roots.push(n.identity);
            }
        }
        roots
    }

    pub(super) fn retire(&mut self, roots: &[NodeIdentity]) {
        if roots.is_empty() {
            return;
        }
        let roots: BTreeSet<_> = roots.iter().copied().collect();
        let mut map = vec![None; self.tree.nodes.len()];
        let mut next = 0;
        for (i, n) in self.tree.nodes.iter().enumerate() {
            if !roots.contains(&n.identity) && n.parent.is_none_or(|p| map[p as usize].is_some()) {
                map[i] = Some(next);
                next += 1;
            }
        }
        let t = self.timeline.as_mut().unwrap();
        t.pipes.remap(&self.tree, &map);
        t.widths.retire(&self.tree, &map);
        self.scaffold.remap(&map);
        self.local.remap(&map);
        self.tree.crossover = map[..self.tree.crossover].iter().flatten().count();
        let mut i = 0;
        self.tree.nodes.retain_mut(|n| {
            let keep = map[i].is_some();
            i += 1;
            if keep {
                n.parent = n.parent.map(|p| map[p as usize].unwrap());
                n.branch = map[n.branch as usize].unwrap();
                self.identities[n.identity.key] = map[i - 1].unwrap() as usize;
            } else {
                self.identities.remove(n.identity.key);
            }
            keep
        });
        self.shed += map.len() - next as usize;
    }
}
