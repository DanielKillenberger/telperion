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
    foliage: Map<Map<Placement>>,
    placement_count: usize,
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
        self.foliage.iter().flat_map(Map::iter)
    }
    pub fn node(&self, id: NodeIdentity) -> Option<&PackedNode> {
        self.structural
            .get(id.birth_order())
            .or_else(|| self.local.get(id.birth_order()))
            .filter(|n| n.wood.identity == id)
    }
    pub fn placement(&self, id: PlacementIdentity) -> Option<&Placement> {
        self.foliage
            .get(id.shoot.birth_order())?
            .get(u64::from(id.station))
            .filter(|p| p.identity == id)
    }
    pub fn placement_count(&self) -> usize {
        self.placement_count
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
            out.put_placement(p);
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
            self.remove_placement(id);
        }
        for p in record
            .born_placements
            .iter()
            .chain(&record.moved_placements)
        {
            self.put_placement(p.clone());
        }
    }
    fn put_placement(&mut self, placement: Placement) {
        let birth = placement.identity.shoot.birth_order();
        let station = u64::from(placement.identity.station);
        if let Some(stations) = self.foliage.get_mut(birth) {
            self.placement_count += usize::from(stations.get(station).is_none());
            stations.set(station, Some(placement));
        } else {
            let mut stations = Map::default();
            stations.set(station, Some(placement));
            self.foliage.set(birth, Some(stations));
            self.placement_count += 1;
        }
    }
    fn remove_placement(&mut self, id: PlacementIdentity) {
        if self.placement(id).is_none() {
            return;
        }
        let birth = id.shoot.birth_order();
        let stations = self.foliage.get_mut(birth).unwrap();
        stations.set(u64::from(id.station), None);
        self.placement_count -= 1;
        if stations.len() == 0 {
            self.foliage.set(birth, None);
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
#[cfg(test)]
mod tests;
