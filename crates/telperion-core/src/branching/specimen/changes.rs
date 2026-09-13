//! Consumer wood and foliage diffs, computed at advance boundaries, never births.
use super::*;
use crate::foliage::{Placement, PlacementIdentity};
use std::collections::BTreeMap;

/// Metres. Only the consumer run buffer rounds radii to this fixed precision;
/// simulation radii and the wood mesh retain their original precision. Rounding
/// both fresh reads and diffs avoids cumulative drift from pairwise epsilon tests.
pub const RADIUS_TOLERANCE: f64 = 1e-9;

#[derive(Debug, Clone)]
pub struct RunNode {
    pub identity: NodeIdentity,
    pub parent: Option<NodeIdentity>,
    pub position: Vec3,
    /// Distal, proximal and allocation radii, in metres.
    pub radii: [f64; 3],
    pub kind: NodeKind,
}

impl PartialEq for RunNode {
    fn eq(&self, other: &Self) -> bool {
        let position = |p: Vec3| [p.x, p.y, p.z].map(f64::to_bits);
        self.identity == other.identity
            && self.parent == other.parent
            && self.kind == other.kind
            && position(self.position) == position(other.position)
            && self.radii.map(f64::to_bits) == other.radii.map(f64::to_bits)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Run {
    pub identity: NodeIdentity,
    pub nodes: Vec<RunNode>,
}

/// Identity ordered consumer buffers. Storage compaction cannot reorder these.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct SpecimenBuffers {
    pub runs: BTreeMap<NodeIdentity, Run>,
    pub placements: BTreeMap<PlacementIdentity, Placement>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ChangeRecord {
    pub radius_tolerance: f64,
    pub born_runs: Vec<Run>,
    /// Full replacement runs, including extension and changed node positions.
    pub resized_runs: Vec<Run>,
    pub shed_runs: Vec<NodeIdentity>,
    pub born_placements: Vec<Placement>,
    pub moved_placements: Vec<Placement>,
    pub shed_placements: Vec<PlacementIdentity>,
}
impl Default for ChangeRecord {
    fn default() -> Self {
        Self {
            radius_tolerance: RADIUS_TOLERANCE,
            born_runs: Vec::new(),
            resized_runs: Vec::new(),
            shed_runs: Vec::new(),
            born_placements: Vec::new(),
            moved_placements: Vec::new(),
            shed_placements: Vec::new(),
        }
    }
}
impl Specimen {
    pub fn buffers(&self) -> Result<SpecimenBuffers> {
        let tree = self.tree();
        let mut out = SpecimenBuffers::default();
        for n in &tree.nodes {
            let id = tree.nodes[n.branch as usize].identity;
            let run = out.runs.entry(id).or_insert_with(|| Run {
                identity: id,
                nodes: Vec::new(),
            });
            run.nodes.push(RunNode {
                identity: n.identity,
                parent: n.parent.map(|p| tree.nodes[p as usize].identity),
                position: n.position,
                radii: [n.radius, n.start_radius, n.base_radius]
                    .map(|r| (r / RADIUS_TOLERANCE).round() * RADIUS_TOLERANCE),
                kind: n.kind,
            });
        }
        for run in out.runs.values_mut() {
            run.nodes.sort_by_key(|n| n.identity);
        }
        out.placements = self
            .placements()?
            .into_iter()
            .map(|p| (p.identity, p))
            .collect();
        Ok(out)
    }
}
impl ChangeRecord {
    pub(super) fn between(before: &SpecimenBuffers, after: &SpecimenBuffers) -> Self {
        let mut out = Self::default();
        for (&id, run) in &after.runs {
            match before.runs.get(&id) {
                None => out.born_runs.push(run.clone()),
                Some(old) if old != run => out.resized_runs.push(run.clone()),
                _ => {}
            }
        }
        out.shed_runs = before
            .runs
            .keys()
            .filter(|id| !after.runs.contains_key(id))
            .copied()
            .collect();
        for (&id, placement) in &after.placements {
            match before.placements.get(&id) {
                None => out.born_placements.push(placement.clone()),
                Some(old) if old != placement => out.moved_placements.push(placement.clone()),
                _ => {}
            }
        }
        out.shed_placements = before
            .placements
            .keys()
            .filter(|id| !after.placements.contains_key(id))
            .copied()
            .collect();
        out
    }

    /// Apply without a fresh read. Check all category memberships first, so an
    /// invalid record leaves the caller's buffers intact. Placements are exact
    /// f32 matrices; run radii use the same fixed tolerance as `buffers()`.
    pub fn apply(&self, previous: &mut SpecimenBuffers) -> Result<()> {
        if self.radius_tolerance != RADIUS_TOLERANCE {
            return Err(Error::InvalidValue {
                field: "change record radius tolerance",
                value: self.radius_tolerance.to_string(),
            });
        }
        check_keys(
            &previous.runs,
            self.born_runs.iter().map(|r| r.identity),
            self.resized_runs.iter().map(|r| r.identity),
            self.shed_runs.iter().copied(),
            run_error,
        )?;
        check_keys(
            &previous.placements,
            self.born_placements.iter().map(|p| p.identity),
            self.moved_placements.iter().map(|p| p.identity),
            self.shed_placements.iter().copied(),
            |id| placement_error(id, previous),
        )?;
        for id in &self.shed_runs {
            previous.runs.remove(id);
        }
        for r in self.born_runs.iter().chain(&self.resized_runs) {
            previous.runs.insert(r.identity, r.clone());
        }
        for id in &self.shed_placements {
            previous.placements.remove(id);
        }
        for p in self.born_placements.iter().chain(&self.moved_placements) {
            previous.placements.insert(p.identity, p.clone());
        }
        Ok(())
    }

    /// Verify exact reconciliation with a fresh structure/placement read. Missing
    /// or incorrect wood names the run's birth identity, never a storage index.
    pub fn validate(&self, previous: &SpecimenBuffers, fresh: &SpecimenBuffers) -> Result<()> {
        let mut applied = previous.clone();
        self.apply(&mut applied)?;
        for &id in applied.runs.keys().chain(fresh.runs.keys()) {
            if applied.runs.get(&id) != fresh.runs.get(&id) {
                return Err(run_error(id));
            }
        }
        for &id in applied.placements.keys().chain(fresh.placements.keys()) {
            if applied.placements.get(&id) != fresh.placements.get(&id) {
                return Err(placement_error(
                    id,
                    if fresh.placements.contains_key(&id) {
                        fresh
                    } else {
                        previous
                    },
                ));
            }
        }
        Ok(())
    }
}

fn run_error(id: NodeIdentity) -> Error {
    Error::InvalidValue {
        field: "change record run",
        value: id.birth_order().to_string(),
    }
}
fn placement_error(id: PlacementIdentity, buffers: &SpecimenBuffers) -> Error {
    let run = buffers
        .runs
        .values()
        .find(|r| r.nodes.iter().any(|n| n.identity == id.shoot))
        .map_or(id.shoot, |r| r.identity);
    Error::InvalidValue {
        field: "change record placement",
        value: format!(
            "run {}, shoot {}, station {}",
            run.birth_order(),
            id.shoot.birth_order(),
            id.station
        ),
    }
}
fn check_keys<K: Ord + Copy, V>(
    old: &BTreeMap<K, V>,
    born: impl Iterator<Item = K>,
    changed: impl Iterator<Item = K>,
    shed: impl Iterator<Item = K>,
    error: impl Fn(K) -> Error,
) -> Result<()> {
    let mut seen = std::collections::BTreeSet::new();
    for (id, exists) in born
        .map(|id| (id, false))
        .chain(changed.chain(shed).map(|id| (id, true)))
    {
        if old.contains_key(&id) != exists || !seen.insert(id) {
            return Err(error(id));
        }
    }
    Ok(())
}
