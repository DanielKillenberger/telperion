//! Repair only storage that actually moved after structural insertion.
use super::*;
impl Specimen {
    pub(super) fn insert_structural(&mut self, first: usize, previous_len: usize) {
        let added = self.tree.nodes.len() - previous_len;
        self.tree.crossover = first + added;
        if added == 0 {
            return;
        }
        if first < previous_len {
            #[cfg(test)]
            {
                self.cost.storage = previous_len - first;
                self.cost.identities += previous_len - first;
            }
            // Repair old locals in one pass. Their identities already exist;
            // only their dense-map location and local storage references shift.
            for (i, n) in self.tree.nodes[first..previous_len].iter_mut().enumerate() {
                if let Some(parent) = &mut n.parent {
                    if *parent as usize >= first {
                        *parent += added as u32;
                    }
                }
                n.branch += added as u32;
                self.identities[n.identity.key] = first + i + added;
            }
            for (i, n) in self.tree.nodes[previous_len..].iter_mut().enumerate() {
                n.parent = n.parent.map(|p| {
                    if p as usize >= previous_len {
                        (first + p as usize - previous_len) as u32
                    } else {
                        p
                    }
                });
                n.branch = (first + i) as u32;
            }
            self.tree.nodes[first..].rotate_right(added);
            self.scaffold.reindex_appended(first, previous_len);
            self.local.shift(first, added);
        }
        self.identify_range(first..first + added);
    }
}
