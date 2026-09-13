//! Consumer retention changes storage, never simulation or identity allocation.
use super::*;
use crate::growth::Age;

pub(super) const DEFAULT_HISTORY_CAP: f64 = 10_000.0;

#[derive(Clone)]
pub(super) struct Retention {
    pub cap: Age,
    floor: Age,
    // Compact death index preserves the public cumulative shed set. Geometry,
    // shoot histories, frames and placements of these identities are discarded.
    // Their generational slots remain reserved so future births never reuse them.
    pub dead: Vec<NodeIdentity>,
}
impl Default for Retention {
    fn default() -> Self {
        Self {
            cap: Age::from_years(DEFAULT_HISTORY_CAP).unwrap(),
            floor: Age::default(),
            dead: Vec::new(),
        }
    }
}

pub(super) fn checked_cap(years: f64) -> Result<Age> {
    Age::from_years(years).map_err(|_| Error::InvalidValue {
        field: "history cap",
        value: years.to_string(),
    })
}

impl Specimen {
    pub fn history_cap(&self) -> f64 {
        self.retention.cap.years()
    }

    /// Expanding retention cannot restore discarded history; rebuilding can.
    pub fn set_history_cap(&mut self, years: f64) -> Result<()> {
        let cap = checked_cap(years)?;
        if self.timeline.is_none() {
            return Err(Error::InvalidInput("specimen has no age"));
        }
        self.retention.cap = cap;
        self.compact_history();
        Ok(())
    }

    pub(super) fn check_retained(&self, age: Age) -> Result<()> {
        if age.ticks() < self.retention.floor.ticks() {
            return Err(Error::InvalidValue {
                field: "age older than history cap",
                value: format!(
                    "{}; cap {}; earliest retained age {}",
                    age.years(),
                    self.history_cap(),
                    self.retention.floor.years()
                ),
            });
        }
        Ok(())
    }

    pub(super) fn compact_history(&mut self) {
        let frontier = self.timeline.as_ref().unwrap().age;
        let floor = frontier
            .ticks()
            .saturating_sub(self.retention.cap.ticks())
            .max(self.retention.floor.ticks());
        self.retention.floor = Age::from_ticks(floor);
        if floor == 0 || self.shed == self.retention.dead.len() {
            return;
        }
        let mut kept = 0;
        let map: Vec<_> = self
            .tree
            .nodes
            .iter()
            .map(|n| {
                if n.shoot.death_year.is_some_and(|death| {
                    Age {
                        slice: death,
                        remainder: 0,
                    }
                    .ticks()
                        < floor
                }) {
                    None
                } else {
                    let index = kept;
                    kept += 1;
                    Some(index as u32)
                }
            })
            .collect();
        if kept == self.tree.nodes.len() {
            return;
        }
        self.read.take();
        let t = self.timeline.as_mut().unwrap();
        t.pipes.remap(&self.tree, &map);
        self.scaffold.remap(&map);
        self.local.remap(&map);
        let mut removed = Vec::new();
        for (i, n) in self.tree.nodes.iter_mut().enumerate() {
            let id = n.identity;
            self.identities[id.key] = map[i].map_or(usize::MAX, |i| i as usize);
            if map[i].is_none() {
                removed.push(id);
                self.links.remove(id.key);
                self.keyframes.forget(id);
            } else {
                n.parent = n.parent.map(|p| map[p as usize].unwrap());
                n.branch = map[n.branch as usize].unwrap();
            }
        }
        for (_, links) in self.links.iter_mut() {
            links
                .members
                .retain(|id| self.identities[id.key] != usize::MAX);
            links
                .children
                .retain(|id| self.identities[id.key] != usize::MAX);
        }
        self.keyframes.events.forget(&self.identities);
        self.births.forget(&self.identities);
        t.widths.forget(&removed, &self.identities);
        t.foliage.forget(&removed);
        self.config.max_nodes -= removed.len();
        self.retention.dead.extend(removed);
        self.tree
            .nodes
            .retain(|n| self.identities[n.identity.key] != usize::MAX);
        self.tree.nodes.shrink_to_fit();
        self.tree.crossover = self
            .tree
            .nodes
            .iter()
            .filter(|n| n.kind == NodeKind::Structural)
            .count();
        t.unpacked = true;
    }
}

#[cfg(test)]
mod tests;
