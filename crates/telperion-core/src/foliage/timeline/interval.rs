use super::*;
use crate::tree::NodeKey;
use slotmap::SecondaryMap;

impl Foliage {
    fn counts(&self, tree: &Tree, age: Age) -> Result<SecondaryMap<NodeKey, (usize, usize)>> {
        let mut counts = SecondaryMap::new();
        let mut total = 0usize;
        for i in self.living(tree, age)? {
            let n = &tree.nodes[i];
            let length = n
                .position
                .distance(tree.nodes[n.parent.unwrap() as usize].position);
            let stations = if length == 0.0 {
                0.0
            } else {
                (length / self.twig.internode_length - 1e-9).ceil().max(1.0)
                    * self.twig.stations_per_internode as f64
            };
            if !stations.is_finite() || stations > u32::MAX as f64 {
                return Err(Error::ResourceLimit("foliage station identity overflow"));
            }
            let stations = stations as usize;
            let count = self.visible(Age::from_years(n.shoot.birth_year)?, age, stations);
            total = total
                .checked_add(count)
                .ok_or(Error::ResourceLimit("foliage count overflow"))?;
            if total > self.canopy.max_instances {
                return Err(Error::ResourceLimit("foliage instance budget"));
            }
            if count > 0 {
                counts.insert(n.identity.key, (i, count));
            }
        }
        Ok(counts)
    }

    /// Select station ranges from cohort offsets and living-shoot stamps. Contact
    /// candidates come from stamped sweep dependencies at both endpoints; compare
    /// their exact cache keys, never whole placement states or transform buffers.
    pub(crate) fn changes(
        &self,
        before: (&Tree, Envelope, Age),
        after: (&Tree, Envelope, Age),
        changed: &BTreeSet<NodeIdentity>,
        dependencies: Option<&BTreeSet<NodeIdentity>>,
        out: &mut crate::branching::ChangeRecord,
    ) -> Result<()> {
        let (old_tree, _old_envelope, from) = before;
        let (tree, envelope, to) = after;
        let old = self.counts(old_tree, from)?;
        let new = self.counts(tree, to)?;
        let mut candidates = changed.clone();
        if self.canopy.surface_contact > 0.0 {
            if let Some(dependencies) = dependencies {
                candidates.extend(dependencies);
            } else {
                candidates.extend(crate::surface::affected_contacts(old_tree, changed)?);
                candidates.extend(crate::surface::affected_contacts(tree, changed)?);
            }
        }
        let mut mask = SecondaryMap::new();
        for id in candidates {
            mask.insert(id.key, ());
        }
        let mut selected = BTreeSet::new();
        for (key, &(i, count)) in &new {
            let id = tree.nodes[i].identity;
            let previous = old.get(key).map_or(0, |&(_, count)| count);
            if count > previous || mask.contains_key(key) {
                selected.insert(id);
            }
        }
        let contacts = |tree, ids: &BTreeSet<_>| {
            if self.canopy.surface_contact > 0.0 && !ids.is_empty() {
                AttachmentSurface::selected(tree, self.contact_height, &self.surface, Some(ids))
                    .map(Some)
            } else {
                Ok(None)
            }
        };
        let common: BTreeSet<_> = selected
            .iter()
            .filter(|id| old.contains_key(id.key))
            .copied()
            .collect();
        let old_contacts = contacts(old_tree, &common)?;
        let new_contacts = contacts(tree, &selected)?;
        for (key, &(i, count)) in &old {
            let id = old_tree.nodes[i].identity;
            let remaining = new.get(key).map_or(0, |&(_, count)| count);
            out.shed_placements
                .extend((remaining..count).map(|station| PlacementIdentity {
                    shoot: id,
                    station: station as u32,
                }));
        }
        out.shed_placements.sort_unstable();
        for id in selected {
            let (i, count) = new[id.key];
            let previous = old.get(id.key).map_or(0, |&(_, count)| count);
            let moved = previous > 0
                && (changed.contains(&id) || {
                    let old_i = old[id.key].0;
                    old_contacts.as_ref().map(|s| s.signature(old_i))
                        != new_contacts.as_ref().map(|s| s.signature(i))
                });
            if count <= previous && !moved {
                continue;
            }
            let placements = self.place_shoot(tree, i, envelope, new_contacts.as_ref())?;
            if moved {
                out.moved_placements
                    .extend_from_slice(&placements[..previous.min(count)]);
            }
            if count > previous {
                out.born_placements
                    .extend_from_slice(&placements[previous..count]);
            }
        }
        Ok(())
    }
}
