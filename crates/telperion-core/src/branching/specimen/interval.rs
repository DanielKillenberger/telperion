//! Interval selection is over births, deaths and canonical radius frames. Only
//! selected target runs and foliage transforms are materialized for the record.
use super::*;
use crate::growth::Age;
use std::collections::BTreeSet;

impl Specimen {
    /// Changes from either retained age to the other, including backward reads.
    pub fn changes_between(&self, from: f64, to: f64) -> Result<ChangeRecord> {
        self.interval(self.read_age(from)?, self.read_age(to)?)
    }

    pub(super) fn interval(&self, from: Age, to: Age) -> Result<ChangeRecord> {
        if from.slice == to.slice {
            return Ok(ChangeRecord::default());
        }
        let lo = from.slice.min(to.slice);
        let hi = from.slice.max(to.slice);
        let mut changed = BTreeSet::new();
        for key in self.keyframes.events.between(lo + 1, hi) {
            #[cfg(test)]
            self.cost.event_visits.set(self.cost.event_visits.get() + 1);
            if let Some(&i) = self.identities.get(key).filter(|&&i| i != usize::MAX) {
                changed.insert(self.tree.nodes[i].identity);
            }
        }
        let selected: BTreeSet<_> = changed.iter().map(|id| self.links[id.key].run).collect();
        let alive = |id: NodeIdentity, age: Age| {
            let n = &self.tree.nodes[self.identities[id.key]];
            n.shoot.birth_year <= age.slice as f64
                && n.shoot.death_year.is_none_or(|death| age.slice < death)
        };
        let mut record = ChangeRecord::default();
        for id in selected {
            if alive(id, to) {
                let nodes = self.links[id.key]
                    .members
                    .iter()
                    .filter(|&&member| alive(member, to))
                    .map(|&member| {
                        let n = &self.tree.nodes[self.identities[member.key]];
                        Ok(RunNode {
                            identity: member,
                            parent: self.links[member.key].parent,
                            position: n.position,
                            radii: self
                                .keyframes
                                .at(member, to.slice)
                                .ok_or(Error::InvalidInput("missing radius history"))?,
                            kind: n.kind,
                        })
                    })
                    .collect::<Result<Vec<_>>>()?;
                let run = Run {
                    identity: id,
                    nodes,
                };
                if alive(id, from) {
                    record.resized_runs.push(run);
                } else {
                    record.born_runs.push(run);
                }
            } else if alive(id, from) {
                record.shed_runs.push(id);
            }
        }
        let foliage = &self.timeline.as_ref().unwrap().foliage;
        let (before, old_envelope, after, envelope) =
            if let Some(lifetime) = foliage.sparse_interval() {
                let mut candidates = changed.clone();
                // Only shoots still filling at some point in the interval can gain
                // cohort stations. Births are indexed independently of width frames.
                for key in self.births.between(lo.saturating_sub(lifetime), hi) {
                    if let Some(&i) = self.identities.get(key).filter(|&&i| i != usize::MAX) {
                        candidates.insert(self.tree.nodes[i].identity);
                    }
                }
                if candidates.is_empty() {
                    return Ok(record);
                }
                (
                    self.selected_wood(from, &candidates)?,
                    self.envelope_at(from),
                    self.selected_wood(to, &candidates)?,
                    self.envelope_at(to),
                )
            } else {
                let (before, old_envelope) = self.wood_at(from, false)?;
                let (after, envelope) = self.wood_at(to, false)?;
                (before, old_envelope, after, envelope)
            };
        let dependencies = (foliage.sparse_interval().is_none()
            && changed.len() * 8 < self.tree.nodes.len())
        .then(|| {
            let mut ids = self.contact_candidates(&changed, from);
            ids.extend(self.contact_candidates(&changed, to));
            ids
        });
        self.timeline.as_ref().unwrap().foliage.changes(
            (&before, old_envelope, from),
            (&after, envelope, to),
            &changed,
            dependencies.as_ref(),
            &mut record,
        )?;
        Ok(record)
    }
}

#[cfg(test)]
pub(super) mod tests;

impl Specimen {
    /// Small endpoint geometry for independent shoot transforms. Include the
    /// ancestor closure so even zero-length attachments retain exact positions.
    fn selected_wood(&self, age: Age, selected: &BTreeSet<NodeIdentity>) -> Result<Tree> {
        let mut ids = BTreeSet::new();
        ids.insert(self.tree.nodes[0].identity);
        for &id in selected {
            let mut at = Some(id);
            while let Some(id) = at {
                if !ids.insert(id) {
                    break;
                }
                at = self.links[id.key].parent;
            }
        }
        let mut indices = std::collections::BTreeMap::new();
        let mut nodes = Vec::new();
        let mut crossover = 0;
        for structural in [true, false] {
            for &id in &ids {
                let n = &self.tree.nodes[self.identities[id.key]];
                if (n.kind == NodeKind::Structural) != structural
                    || n.shoot.birth_year > age.slice as f64
                    || n.shoot.death_year.is_some_and(|death| death <= age.slice)
                {
                    continue;
                }
                let radii = self
                    .keyframes
                    .at(id, age.slice)
                    .ok_or(Error::InvalidInput("missing radius history"))?;
                indices.insert(id, nodes.len() as u32);
                nodes.push(Node {
                    identity: id,
                    position: n.position,
                    kind: n.kind,
                    radius: radii[0],
                    start_radius: radii[1],
                    base_radius: radii[2],
                    shoot: crate::tree::ShootState {
                        birth_year: n.shoot.birth_year,
                        ..Default::default()
                    },
                    ..Node::root()
                });
            }
            if structural {
                crossover = nodes.len();
            }
        }
        for n in &mut nodes {
            n.parent = self.links[n.identity.key].parent.map(|id| indices[&id]);
            n.branch = *indices.get(&self.links[n.identity.key].run).unwrap_or(&0);
        }
        Ok(Tree {
            nodes,
            crossover,
            diagnostics: self.tree.diagnostics,
        })
    }
}
