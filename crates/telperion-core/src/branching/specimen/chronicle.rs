//! Identity topology is retained from birth, independently of packed indices.
use super::*;
use std::collections::BTreeSet;

#[derive(Clone)]
pub(super) struct Links {
    pub parent: Option<NodeIdentity>,
    pub run: NodeIdentity,
    pub(super) children: Vec<NodeIdentity>,
    pub(super) members: Vec<NodeIdentity>,
}
impl Specimen {
    pub(super) fn link_births(&mut self, born: &[usize]) {
        if self.timeline.is_none() {
            return;
        }
        for &i in born {
            let n = &self.tree.nodes[i];
            let parent = n.parent.map(|p| self.tree.nodes[p as usize].identity);
            self.links.insert(
                n.identity.key,
                Links {
                    parent,
                    run: self.tree.nodes[n.branch as usize].identity,
                    children: Vec::new(),
                    members: Vec::new(),
                },
            );
            self.births
                .record(n.shoot.birth_year as u64, n.identity.key);
            let run = self.links[n.identity.key].run;
            self.links[run.key].members.push(n.identity);
            if let Some(parent) = parent {
                self.links[parent.key].children.push(n.identity);
            }
        }
    }
    /// Stamp the complete subtree, including children born after the decision.
    /// The simulation caches forget dead work; the chronicle keeps every slot.
    pub(super) fn stamp_deaths(&mut self, roots: &[NodeIdentity], year: u64) {
        if roots.is_empty() {
            return;
        }
        self.read.take();
        let roots: BTreeSet<_> = roots.iter().copied().collect();
        let mut dead = Vec::new();
        let mut pending = roots;
        while let Some(id) = pending.pop_first() {
            let i = self.identities[id.key];
            let n = &mut self.tree.nodes[i];
            if i == 0 || n.shoot.death_year.is_some() {
                continue;
            }
            n.shoot.death_year = Some(year);
            self.keyframes.events.record(year, id.key);
            dead.push(i);
            pending.extend(&self.links[id.key].children);
        }
        // Freeze a canonical final width even if this advance has not yet
        // materialized its live outputs. Tick partitions must retain identical
        // dead records as well as identical living reads.
        let t = self.timeline.as_ref().unwrap();
        for &i in &dead {
            let [radius, start_radius, base_radius] = t.widths.sample(&self.tree, &t.pipes, i);
            let n = &mut self.tree.nodes[i];
            n.radius = radius;
            n.start_radius = start_radius;
            n.base_radius = base_radius;
        }
        let t = self.timeline.as_mut().unwrap();
        t.unpacked = true;
        t.pipes.remove_dead(&self.tree, &dead);
        self.local.remove_dead(&self.tree, &dead);
        self.scaffold.remove_dead(&self.tree);
        self.shed += dead.len();
    }

    #[cfg(test)]
    pub(super) fn retire(&mut self, roots: &[NodeIdentity]) {
        self.stamp_deaths(roots, self.timeline.as_ref().unwrap().age.slice);
    }
}
