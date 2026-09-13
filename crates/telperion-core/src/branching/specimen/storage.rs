//! Internal slices append in birth order; consumers receive a cached packed view
//! on demand. No local node or frontier reference moves on structural birth.
use super::*;
#[derive(Clone)]
pub(super) struct Read {
    pub tree: Tree,
    pub indices: Vec<usize>,
}
impl Specimen {
    pub(super) fn packed_read(&self) -> Option<&Read> {
        if !self
            .timeline
            .as_ref()
            .is_some_and(|t| t.unpacked || self.shed > 0)
        {
            return None;
        }
        Some(self.read.get_or_init(|| {
            let mut structural = 0;
            let mut local = self
                .tree
                .nodes
                .iter()
                .filter(|n| n.kind == NodeKind::Structural && n.shoot.death_year.is_none())
                .count();
            let indices: Vec<_> = self
                .tree
                .nodes
                .iter()
                .map(|n| {
                    if n.shoot.death_year.is_some() {
                        return usize::MAX;
                    }
                    let next = if n.kind == NodeKind::Structural {
                        &mut structural
                    } else {
                        &mut local
                    };
                    let index = *next;
                    *next += 1;
                    index
                })
                .collect();
            let mut tree = Tree {
                nodes: Vec::with_capacity(self.tree.nodes.len()),
                crossover: structural,
                diagnostics: self.tree.diagnostics,
            };
            for structural in [true, false] {
                tree.nodes.extend(
                    self.tree
                        .nodes
                        .iter()
                        .filter(|n| {
                            n.shoot.death_year.is_none()
                                && (n.kind == NodeKind::Structural) == structural
                        })
                        .map(|n| {
                            let mut node = n.clone();
                            let links = &self.links[n.identity.key];
                            node.parent =
                                links.parent.map(|p| indices[self.identities[p.key]] as u32);
                            node.branch = indices[self.identities[links.run.key]] as u32;
                            node
                        }),
                );
            }
            Read { tree, indices }
        }))
    }

    pub(super) fn insert_structural(&mut self, first: usize, previous_len: usize) {
        let added = self.tree.nodes.len() - previous_len;
        self.tree.crossover = first + added;
        if added == 0 {
            return;
        }
        self.timeline.as_mut().unwrap().unpacked |= first < previous_len;
        self.identify_range(previous_len..self.tree.nodes.len());
    }

    #[cfg(test)]
    pub(super) fn pack_storage(&mut self) {
        self.read.take();
        let t = self.timeline.as_mut().unwrap();
        if !std::mem::take(&mut t.unpacked) {
            return;
        }
        #[cfg(test)]
        let clock = std::time::Instant::now();
        let mut structural = 0;
        let mut local = self.tree.crossover;
        let map: Vec<_> = self
            .tree
            .nodes
            .iter()
            .map(|n| {
                let next = if n.kind == NodeKind::Structural {
                    &mut structural
                } else {
                    &mut local
                };
                let i = *next;
                *next += 1;
                Some(i as u32)
            })
            .collect();
        t.pipes.remap(&self.tree, &map);
        self.scaffold.remap(&map);
        self.local.remap(&map);
        for n in &mut self.tree.nodes {
            n.parent = n.parent.map(|p| map[p as usize].unwrap());
            n.branch = map[n.branch as usize].unwrap();
        }
        // Stability preserves birth order within each kind, exactly matching
        // the map above. Keep the allocation and its append headroom intact.
        self.tree
            .nodes
            .sort_by_key(|n| n.kind != NodeKind::Structural);
        for (i, n) in self.tree.nodes.iter().enumerate() {
            self.identities[n.identity.key] = i;
        }
        #[cfg(test)]
        {
            self.cost.packing = clock.elapsed();
            self.cost.packed_nodes = self.tree.nodes.len();
        }
    }
}
