//! Local widths propagate only from changed parents and newly born runs.
use super::*;
use crate::tree::LocalWidth;
use slotmap::SecondaryMap;
use std::collections::BTreeSet;

#[derive(Clone, Default)]
pub(super) struct Widths {
    children: SecondaryMap<NodeKey, Vec<NodeIdentity>>,
    pending: BTreeSet<NodeIdentity>,
}
impl Widths {
    pub fn born(&mut self, tree: &mut Tree, i: usize) {
        let parent = tree.nodes[i].parent.unwrap() as usize;
        let support = tree.nodes[parent].radius;
        let n = &mut tree.nodes[i];
        // child_radius is homogeneous in parent radius. Cache its evaluated
        // birth allocation, including twig minimum and seeded variation, rather
        // than redraw a ratio after compaction. Taper and twig length stay in metres.
        n.shoot.width = Some(LocalWidth {
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
    pub fn update(
        &mut self,
        tree: &mut Tree,
        ids: &DenseSlotMap<NodeKey, usize>,
        changed: &[usize],
    ) {
        let mut pending = std::mem::take(&mut self.pending);
        for &i in changed {
            if let Some(children) = self.children.get(tree.nodes[i].identity.key) {
                pending.extend(children);
            }
        }
        while let Some(id) = pending.pop_first() {
            let i = ids[id.key];
            let parent = tree.nodes[i].parent.unwrap() as usize;
            let support = tree.nodes[parent].radius;
            let n = &mut tree.nodes[i];
            let w = n.shoot.width.unwrap();
            let base = child_radius(support, w.ratio, w.power).max(n.base_radius);
            let distal = (base * w.distal).max(n.radius);
            let proximal = (base * w.proximal).max(n.start_radius).max(distal);
            let changed = distal != n.radius;
            n.base_radius = base;
            n.radius = distal;
            n.start_radius = proximal;
            if changed {
                if let Some(children) = self.children.get(id.key) {
                    pending.extend(children);
                }
            }
        }
    }
    pub fn retire(&mut self, tree: &Tree, map: &[Option<u32>]) {
        for (i, n) in tree.nodes.iter().enumerate() {
            if map[i].is_none() {
                self.children.remove(n.identity.key);
                self.pending.remove(&n.identity);
                if let Some(p) = n.parent {
                    if let Some(children) =
                        self.children.get_mut(tree.nodes[p as usize].identity.key)
                    {
                        children.retain(|id| *id != n.identity);
                    }
                }
            }
        }
    }
}
