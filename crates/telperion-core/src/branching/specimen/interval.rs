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
        let inside = |year: u64| lo < year && year <= hi;
        let mut changed = BTreeSet::new();
        let mut selected = BTreeSet::new();
        for n in &self.tree.nodes {
            if inside(n.shoot.birth_year as u64)
                || n.shoot.death_year.is_some_and(inside)
                || self.keyframes.changed(n.identity, lo, hi)
            {
                changed.insert(n.identity);
                selected.insert(self.links[n.identity.key].run);
            }
        }
        let (before, old_envelope) = self.wood_at(from, false)?;
        let (after, envelope) = self.wood_at(to, false)?;
        let alive = |id: NodeIdentity, age: Age| {
            let n = &self.tree.nodes[self.identities[id.key]];
            n.shoot.birth_year <= age.slice as f64
                && n.shoot.death_year.is_none_or(|death| age.slice < death)
        };
        let mut mask = slotmap::SecondaryMap::new();
        for id in &selected {
            mask.insert(id.key, ());
        }
        let mut record = ChangeRecord::default();
        for (id, run) in changes::runs(&after, |id| mask.contains_key(id.key)) {
            if alive(id, from) {
                record.resized_runs.push(run);
            } else {
                record.born_runs.push(run);
            }
        }
        record.shed_runs = selected
            .iter()
            .filter(|&&id| alive(id, from) && !alive(id, to))
            .copied()
            .collect();
        self.timeline.as_ref().unwrap().foliage.changes(
            (&before, old_envelope, from),
            (&after, envelope, to),
            &changed,
            &mut record,
        )?;
        Ok(record)
    }
}

#[cfg(test)]
pub(super) mod tests;
