use super::*;
use crate::tree::NodeKey;
use slotmap::SecondaryMap;

impl Foliage {
    // Contact and radius-relative shoot eligibility have additional global
    // dependencies. Finite budgets also require a complete count validation.
    pub(crate) fn sparse_interval(&self) -> Option<u64> {
        (self.canopy.surface_contact == 0.0
            && self.canopy.shoot_radius == 0.0
            && self.canopy.max_instances == usize::MAX)
            .then_some(self.lifetime.slice + u64::from(self.lifetime.remainder > 0))
    }

    /// Clock-only cohort births. Offsets are annual, so fractional ticks never
    /// touch wood or derive a contact surface. A saturated shoot adds no work.
    pub(crate) fn filled(
        &self,
        tree: &Tree,
        envelope: Envelope,
        before: Age,
        after: Age,
    ) -> Result<Vec<Placement>> {
        if before.slice == after.slice {
            return Ok(Vec::new());
        }
        let mut born = BTreeMap::new();
        let mut total = 0;
        for i in self.living(tree, after)? {
            let n = &tree.nodes[i];
            let birth = Age::from_years(n.shoot.birth_year)?;
            let length = n
                .position
                .distance(tree.nodes[n.parent.unwrap() as usize].position);
            let count = if length == 0.0 {
                0
            } else {
                (length / self.twig.internode_length - 1e-9).ceil().max(1.0) as usize
                    * self.twig.stations_per_internode as usize
            };
            let start = self.visible(birth, before, count);
            let end = self.visible(birth, after, count);
            total += end;
            if total > self.canopy.max_instances {
                return Err(Error::ResourceLimit("foliage instance budget"));
            }
            if end > start {
                born.insert(n.identity, (i, start..end));
            }
        }
        if born.is_empty() {
            return Ok(Vec::new());
        }
        let cache = self.cache.borrow();
        let geometry_matches = self.canopy.surface_contact == 0.0
            || cache.geometry.as_ref() == Some(&contact_geometry(tree, envelope.height));
        let reusable = |id, i: usize| {
            cache.shoots.get(&id).filter(|entry| {
                let n = &tree.nodes[i];
                geometry_matches
                    && entry.wood.from == tree.nodes[n.parent.unwrap() as usize].position
                    && entry.wood.to == n.position
                    && entry.wood.radii == [n.start_radius, n.radius]
            })
        };
        let missing: BTreeSet<_> = born
            .iter()
            .filter(|(id, (i, _))| reusable(**id, *i).is_none())
            .map(|(&id, _)| id)
            .collect();
        let contacts = if self.canopy.surface_contact > 0.0 && !missing.is_empty() {
            Some(AttachmentSurface::selected(
                tree,
                envelope.height.max(1e-6),
                &self.surface,
                Some(&missing),
            )?)
        } else {
            None
        };
        let mut out = Vec::new();
        for (id, (i, range)) in born {
            if let Some(entry) = reusable(id, i) {
                out.extend_from_slice(&entry.placements[range]);
            } else {
                let placements = self.place_shoot(tree, i, envelope, contacts.as_ref())?;
                out.extend_from_slice(&placements[range]);
            }
        }
        Ok(out)
    }
}

impl Foliage {
    fn counts(&self, tree: &Tree, age: Age) -> Result<SecondaryMap<NodeKey, (usize, usize)>> {
        let mut counts = SecondaryMap::new();
        let mut total = 0;
        for i in self.living(tree, age)? {
            let n = &tree.nodes[i];
            let length = n
                .position
                .distance(tree.nodes[n.parent.unwrap() as usize].position);
            let stations = if length == 0.0 {
                0
            } else {
                (length / self.twig.internode_length - 1e-9).ceil().max(1.0) as usize
                    * self.twig.stations_per_internode as usize
            };
            if stations > 512 {
                return Err(Error::ResourceLimit("twig station budget"));
            }
            let count = self.visible(Age::from_years(n.shoot.birth_year)?, age, stations);
            total += count;
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
        let (old_tree, old_envelope, from) = before;
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
            // Height enters flare, burial and twist. The stamped envelope is a
            // dependency too, even when a large radius tolerance suppressed wood.
            if old_envelope.height != envelope.height
                && (self.surface.flare_radius != 1.0
                    || self.surface.flare_depth > 0.0
                    || (self.surface.lobes > 0 && self.surface.twist_rate != 0.0))
            {
                candidates.extend(new.values().map(|&(i, _)| tree.nodes[i].identity));
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
        let contacts = |tree, height, ids: &BTreeSet<_>| {
            if self.canopy.surface_contact > 0.0 && !ids.is_empty() {
                AttachmentSurface::selected(tree, f64::max(height, 1e-6), &self.surface, Some(ids))
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
        let old_contacts = contacts(old_tree, old_envelope.height, &common)?;
        let new_contacts = contacts(tree, envelope.height, &selected)?;
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
