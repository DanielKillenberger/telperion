//! Retain packed node payloads through advances. Structural insertion shifts the
//! local index suffix, but only changed nodes clone shoot histories and radii.
use super::*;
impl Specimen {
    pub(in crate::pipeline::branching::specimen) fn update_packed(&mut self, cached: Option<Read>) {
        self.read_active = false;
        let mut updates = std::mem::take(&mut self.read_updates);
        let Some(mut read) = cached else {
            return;
        };
        // Deaths change the dense index permutation. Rebuild on the next read;
        // ordinary sparse extension preserves all existing payloads below.
        if self.shed != read.shed {
            return;
        }
        #[cfg(test)]
        let clock = std::time::Instant::now();
        let first = read.indices.len();
        let old_crossover = read.tree.crossover;
        let structural: Vec<_> = self.tree.nodes[first..]
            .iter()
            .filter(|n| n.kind == NodeKind::Structural)
            .cloned()
            .collect();
        let added = structural.len();
        if added > 0 {
            // Packed parent/branch references are numeric offsets. Inserting
            // structure necessarily shifts every existing local reference.
            for n in &mut read.tree.nodes[old_crossover..] {
                n.parent = n.parent.map(|p| {
                    p + if p as usize >= old_crossover {
                        added as u32
                    } else {
                        0
                    }
                });
                if n.branch as usize >= old_crossover {
                    n.branch += added as u32;
                }
            }
            for index in &mut read.indices {
                if *index != usize::MAX && *index >= old_crossover {
                    *index += added;
                }
            }
            #[cfg(test)]
            {
                self.cost.remapped_nodes = read.tree.nodes.len() - old_crossover;
            }
            read.tree
                .nodes
                .splice(old_crossover..old_crossover, structural);
            read.tree.crossover += added;
        }
        let mut structural_index = old_crossover;
        let mut local_index = read.tree.nodes.len();
        for n in &self.tree.nodes[first..] {
            let index = if n.kind == NodeKind::Structural {
                let i = structural_index;
                structural_index += 1;
                i
            } else {
                let i = local_index;
                local_index += 1;
                read.tree.nodes.push(n.clone());
                i
            };
            read.indices.push(index);
        }
        updates.extend(first..self.tree.nodes.len());
        updates.sort_unstable();
        updates.dedup();
        for i in updates {
            let at = read.indices[i];
            if at == usize::MAX {
                continue;
            }
            let mut node = self.tree.nodes[i].clone();
            let links = &self.links[node.identity.key];
            node.parent = links
                .parent
                .map(|id| read.indices[self.identities[id.key]] as u32);
            node.branch = read.indices[self.identities[links.run.key]] as u32;
            read.tree.nodes[at] = node;
            #[cfg(test)]
            {
                self.cost.packed_nodes += 1;
            }
        }
        read.tree.diagnostics = self.tree.diagnostics;
        #[cfg(test)]
        {
            self.cost.packing = clock.elapsed();
        }
        let _ = self.read.set(read);
    }
}
