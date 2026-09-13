//! Owned views of the chronicle; reading never advances a growth frontier.
use super::*;
use crate::{foliage::Placement, growth::Age, tree::Diagnostics};

#[derive(Debug, Clone, PartialEq)]
pub struct SpecimenRead {
    pub tree: Tree,
    pub envelope: Envelope,
    pub placements: Vec<Placement>,
    /// Identities whose death stamp has been reached, in birth order.
    pub shed: Vec<NodeIdentity>,
}

#[derive(Clone, Copy)]
pub(super) struct Year {
    pub year: u64,
    pub envelope: Envelope,
    pub diagnostics: Diagnostics,
}

impl Specimen {
    /// Read all native outputs at the frontier, without advancing the specimen.
    pub fn read(&self) -> Result<SpecimenRead> {
        let t = self
            .timeline
            .as_ref()
            .ok_or(Error::InvalidInput("specimen has no age"))?;
        self.read_at(t.age)
    }

    /// Filter births, deaths, radius frames and cohorts at or below the frontier.
    /// The returned buffers are owned; later reads and advances cannot alter them.
    pub fn read_at_age(&self, years: f64) -> Result<SpecimenRead> {
        self.read_at(self.read_age(years)?)
    }

    fn read_at(&self, age: Age) -> Result<SpecimenRead> {
        let t = self.timeline.as_ref().unwrap();
        let state = t.years.iter().rev().find(|state| state.year <= age.slice);
        let envelope = state.map_or(
            Envelope {
                height: 0.0,
                ..self.params.envelope
            },
            |state| state.envelope,
        );
        let diagnostics = if age == t.age {
            self.tree.diagnostics
        } else {
            state.map_or_else(Diagnostics::default, |state| state.diagnostics)
        };
        let tree = self.historical_tree(age, diagnostics)?;
        let placements = if age == t.age {
            t.foliage.read(&tree, envelope, age)?
        } else {
            t.foliage.read_uncached(&tree, envelope, age)?
        };
        let mut shed: Vec<_> = self
            .tree
            .nodes
            .iter()
            .filter(|n| n.shoot.death_year.is_some_and(|death| death <= age.slice))
            .map(|n| n.identity)
            .collect();
        shed.sort_unstable();
        Ok(SpecimenRead {
            tree,
            envelope,
            placements,
            shed,
        })
    }

    pub(super) fn read_age(&self, years: f64) -> Result<Age> {
        let age = Age::from_years(years)?;
        let t = self
            .timeline
            .as_ref()
            .ok_or(Error::InvalidInput("specimen has no age"))?;
        if age.ticks() > t.age.ticks() {
            return Err(Error::InvalidValue {
                field: "age beyond specimen frontier",
                value: format!("{years}; frontier {}", t.age.years()),
            });
        }
        Ok(age)
    }

    fn historical_tree(&self, age: Age, diagnostics: Diagnostics) -> Result<Tree> {
        let alive = |n: &&Node| {
            n.shoot.birth_year <= age.slice as f64
                && n.shoot.death_year.is_none_or(|death| age.slice < death)
        };
        let mut nodes = Vec::new();
        let mut indices = vec![usize::MAX; self.tree.nodes.len()];
        let mut crossover = 0;
        for structural in [true, false] {
            for n in self
                .tree
                .nodes
                .iter()
                .filter(alive)
                .filter(|n| (n.kind == NodeKind::Structural) == structural)
            {
                indices[self.identities[n.identity.key]] = nodes.len();
                let mut node = n.clone();
                node.shoot.death_year = None;
                node.shoot
                    .vigour_events
                    .retain(|event| event.year <= age.slice);
                [node.radius, node.start_radius, node.base_radius] = self
                    .keyframes
                    .at(n.identity, age.slice)
                    .ok_or(Error::InvalidInput("missing radius history"))?;
                nodes.push(node);
            }
            if structural {
                crossover = nodes.len();
            }
        }
        for n in &mut nodes {
            let links = &self.links[n.identity.key];
            n.parent = links.parent.map(|p| indices[self.identities[p.key]] as u32);
            n.branch = indices[self.identities[links.run.key]] as u32;
        }
        Ok(Tree {
            nodes,
            crossover,
            diagnostics,
        })
    }
}

#[cfg(test)]
mod tests;
