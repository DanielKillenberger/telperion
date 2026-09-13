//! Local widths propagate only from changed parents and newly born runs.
use super::*;
use crate::tree::LocalWidth;
use slotmap::SecondaryMap;
use std::collections::BTreeSet;

#[derive(Clone, Default)]
pub(super) struct Widths {
    #[cfg(test)]
    pub visited: usize,
    children: SecondaryMap<NodeKey, Vec<NodeIdentity>>,
    pending: BTreeSet<NodeIdentity>,
    queued: SecondaryMap<NodeKey, bool>,
    recorded: SecondaryMap<NodeKey, [f64; 3]>,
    // Invalidation must not clear a slot array proportional to the whole tree.
    generation: u64,
    cache: std::cell::RefCell<SecondaryMap<NodeKey, (u64, [f64; 3])>>,
}
impl Widths {
    pub fn invalidate(&mut self) {
        self.generation += 1;
    }
    /// Birth geometry queries the slice record, never a previous finalization.
    pub fn sample(&self, tree: &Tree, pipes: &radius::Pipes, i: usize) -> [f64; 3] {
        let n = &tree.nodes[i];
        if n.kind == NodeKind::Structural && pipes.contains(i) {
            let (distal, proximal) = pipes.width(i);
            return [distal, proximal, n.base_radius];
        }
        let Some(w) = n.shoot.width else {
            return [n.radius, n.start_radius, n.base_radius];
        };
        if let Some(&(generation, value)) = self.cache.borrow().get(n.identity.key) {
            if generation == self.generation {
                return value;
            }
        }
        let support = self.sample(tree, pipes, n.parent.unwrap() as usize)[0];
        let base = child_radius(support, w.ratio, w.power).max(w.birth[2]);
        let distal = (base * w.distal).max(w.birth[0]);
        let proximal = (base * w.proximal).max(w.birth[1]).max(distal);
        let value = [distal, proximal, base];
        self.cache
            .borrow_mut()
            .insert(n.identity.key, (self.generation, value));
        value
    }
    pub fn born(&mut self, tree: &mut Tree, pipes: &radius::Pipes, i: usize) {
        let parent = tree.nodes[i].parent.unwrap() as usize;
        let support = self.sample(tree, pipes, parent)[0];
        let n = &mut tree.nodes[i];
        // child_radius is homogeneous in parent radius. Cache its evaluated
        // birth allocation, including twig minimum and seeded variation, rather
        // than redraw a ratio after compaction. Taper and twig length stay in metres.
        n.shoot.width = Some(LocalWidth {
            birth: [n.radius, n.start_radius, n.base_radius],
            ratio: n.base_radius / support.max(1e-15),
            power: 1.0,
            distal: n.radius / n.base_radius.max(1e-15),
            proximal: n.start_radius / n.base_radius.max(1e-15),
        });
        let id = n.identity;
        self.children
            .entry(tree.nodes[parent].identity.key)
            .unwrap()
            .or_default()
            .push(id);
        self.pending.insert(id);
    }
    pub fn changed(
        &mut self,
        tree: &Tree,
        ids: &DenseSlotMap<NodeKey, usize>,
        changed: &[usize],
        pipes: &radius::Pipes,
    ) -> Vec<(usize, [f64; 3])> {
        #[cfg(test)]
        {
            self.visited = 0;
        }
        let mut pending = Pending::default();
        let mut changes = Vec::new();
        let enqueue =
            |id: NodeIdentity, pending: &mut Pending, queued: &mut SecondaryMap<_, bool>| {
                if !queued.get(id.key).copied().unwrap_or(false) {
                    queued.insert(id.key, true);
                    pending.push(id);
                }
            };
        for id in std::mem::take(&mut self.pending) {
            enqueue(id, &mut pending, &mut self.queued);
        }
        for &i in changed {
            if let Some(children) = self.children.get(tree.nodes[i].identity.key) {
                for &id in children {
                    enqueue(id, &mut pending, &mut self.queued);
                }
            }
        }
        while let Some(id) = pending.pop() {
            self.queued[id.key] = false;
            #[cfg(test)]
            {
                self.visited += 1;
            }
            let i = ids[id.key];
            if tree.nodes[i].shoot.death_year.is_some() {
                continue;
            }
            let radii = self.sample(tree, pipes, i);
            if self.recorded.get(id.key) != Some(&radii) {
                self.recorded.insert(id.key, radii);
                changes.push((i, radii));
                if let Some(children) = self.children.get(id.key) {
                    for &id in children {
                        enqueue(id, &mut pending, &mut self.queued);
                    }
                }
            }
        }
        changes
    }
}

// Children are always born after their parents. A radix queue preserves exact
// birth order without a comparison-tree allocation for each changed internode.
struct Pending {
    buckets: [Vec<NodeIdentity>; 65],
    last: u64,
}
impl Default for Pending {
    fn default() -> Self {
        Self {
            buckets: std::array::from_fn(|_| Vec::new()),
            last: 0,
        }
    }
}
impl Pending {
    fn push(&mut self, id: NodeIdentity) {
        debug_assert!(id.birth_order() >= self.last);
        let bucket = (64 - (id.birth_order() ^ self.last).leading_zeros()) as usize;
        self.buckets[bucket].push(id);
    }
    fn pop(&mut self) -> Option<NodeIdentity> {
        if self.buckets[0].is_empty() {
            let i = (1..65).find(|&i| !self.buckets[i].is_empty())?;
            self.last = self.buckets[i]
                .iter()
                .map(|id| id.birth_order())
                .min()
                .unwrap();
            while let Some(id) = self.buckets[i].pop() {
                self.push(id);
            }
        }
        self.buckets[0].pop()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn width_queue_keeps_birth_order_across_bucket_boundaries_and_new_children() {
        let id = |birth| NodeIdentity {
            birth,
            ..NodeIdentity::default()
        };
        let mut pending = Pending::default();
        for birth in [500, 3, 260, 128, 4] {
            pending.push(id(birth));
        }
        assert_eq!(pending.pop(), Some(id(3)));
        pending.push(id(7));
        for birth in [4, 7, 128, 260, 500] {
            assert_eq!(pending.pop(), Some(id(birth)));
        }
        assert_eq!(pending.pop(), None);
    }
}
