//! Owned geometry reads share unchanged payloads. Packing is an iteration order,
//! not relocation of nodes or rewriting of numeric parent offsets.
use super::*;
use crate::foliage::{Placement, PlacementIdentity};
mod map;
use map::Map;

#[derive(Clone, Debug, PartialEq)]
pub struct PackedNode {
    pub run: NodeIdentity,
    pub wood: RunNode,
}
/// Immutable owned geometry view. Cloning copies only the radix roots; nodes
/// iterate structural-first, then local, each in birth order. Parent and run
/// references remain identities. Materializing contiguous numeric offsets is an
/// explicit full-output operation through the existing `read`/`tree` APIs.
#[derive(Clone, Default)]
pub struct PackedRead {
    structural: Map<PackedNode>,
    local: Map<PackedNode>,
    runs: Map<Run>,
    foliage: Map<Placement>,
    pub envelope: Envelope,
    pub surface_height: f64,
}
impl PackedRead {
    pub fn nodes(&self) -> impl Iterator<Item = &PackedNode> {
        self.structural.iter().chain(self.local.iter())
    }
    pub fn node_count(&self) -> usize {
        self.structural.len() + self.local.len()
    }
    pub fn crossover(&self) -> usize {
        self.structural.len()
    }
    pub fn placements(&self) -> impl Iterator<Item = &Placement> {
        self.foliage.iter()
    }
    pub fn node(&self, id: NodeIdentity) -> Option<&PackedNode> {
        self.structural
            .get(id.birth_order())
            .or_else(|| self.local.get(id.birth_order()))
            .filter(|n| n.wood.identity == id)
    }
    pub fn placement(&self, id: PlacementIdentity) -> Option<&Placement> {
        self.foliage
            .get(placement_key(id))
            .filter(|p| p.identity == id)
    }
    pub fn placement_count(&self) -> usize {
        self.foliage.len()
    }

    fn from_read(read: SpecimenRead) -> Self {
        let mut out = Self {
            envelope: read.envelope,
            surface_height: read.surface_height,
            ..Default::default()
        };
        for run in changes::runs(&read.tree, |_| true).into_values() {
            out.put_run(run);
        }
        for p in read.placements {
            out.foliage.set(placement_key(p.identity), Some(p));
        }
        out
    }
    fn apply(&mut self, record: &ChangeRecord) {
        for id in record
            .shed_runs
            .iter()
            .chain(record.resized_runs.iter().map(|r| &r.identity))
        {
            if let Some(run) = self.runs.get(id.birth_order()).cloned() {
                for n in run.nodes {
                    let map = if n.kind == NodeKind::Structural {
                        &mut self.structural
                    } else {
                        &mut self.local
                    };
                    map.set(n.identity.birth_order(), None);
                }
            }
            self.runs.set(id.birth_order(), None);
        }
        for run in record.born_runs.iter().chain(&record.resized_runs) {
            self.put_run(run.clone());
        }
        for &id in &record.shed_placements {
            self.foliage.set(placement_key(id), None);
        }
        for p in record
            .born_placements
            .iter()
            .chain(&record.moved_placements)
        {
            self.foliage.set(placement_key(p.identity), Some(p.clone()));
        }
    }
    fn put_run(&mut self, run: Run) {
        for wood in &run.nodes {
            let map = if wood.kind == NodeKind::Structural {
                &mut self.structural
            } else {
                &mut self.local
            };
            map.set(
                wood.identity.birth_order(),
                Some(PackedNode {
                    run: run.identity,
                    wood: wood.clone(),
                }),
            );
        }
        self.runs.set(run.identity.birth_order(), Some(run));
    }
}
impl Specimen {
    /// Shared owned frontier geometry. Earlier-age reads remain a chronicle
    /// filter; they do not replace the cached frontier view.
    pub fn read_packed(&self) -> Result<PackedRead> {
        self.read_packed_at_age(self.age())
    }
    pub fn read_packed_at_age(&self, years: f64) -> Result<PackedRead> {
        let age = self.read_age(years)?;
        if age == self.timeline.as_ref().unwrap().age {
            if let Some((cached_age, read)) = self.shared.borrow().as_ref() {
                if *cached_age == age {
                    return Ok(read.clone());
                }
            }
        }
        let (tree, envelope) = self.wood_at(age, false)?;
        let placements = self
            .timeline
            .as_ref()
            .unwrap()
            .foliage
            .read_uncached(&tree, envelope, age)?;
        let read = PackedRead::from_read(SpecimenRead {
            tree,
            envelope,
            surface_height: self.surface_height(),
            placements,
            reference: self.timeline.as_ref().unwrap().foliage.reference(),
            shed: Vec::new(),
        });
        if age == self.timeline.as_ref().unwrap().age {
            *self.shared.borrow_mut() = Some((age, read.clone()));
        }
        Ok(read)
    }
    pub(super) fn update_shared(&self, from: crate::growth::Age, record: &ChangeRecord) {
        let mut cached = self.shared.borrow_mut();
        if let Some((age, read)) = cached.as_mut() {
            if *age != from {
                *cached = None;
                return;
            }
            #[cfg(test)]
            let clock = std::time::Instant::now();
            read.apply(record);
            read.envelope = self.envelope();
            *age = self.timeline.as_ref().unwrap().age;
            #[cfg(test)]
            self.cost.shared_update.set(clock.elapsed());
        }
    }
}
fn placement_key(id: PlacementIdentity) -> u128 {
    (u128::from(id.shoot.birth_order()) << u32::BITS) | u128::from(id.station)
}

#[cfg(test)]
mod tests;
