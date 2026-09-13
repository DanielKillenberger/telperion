//! Canonical annual radii, keyed independently of output storage.
use super::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) struct Frame {
    pub year: u64,
    pub radii: [f64; 3],
}

#[derive(Clone, Default)]
pub(super) struct Keyframes {
    frames: slotmap::SecondaryMap<NodeKey, Vec<Frame>>,
    pending: std::collections::BTreeSet<NodeIdentity>,
}

impl Keyframes {
    pub(super) fn forget(&mut self, id: NodeIdentity) {
        self.frames.remove(id.key);
        self.pending.remove(&id);
    }

    pub(super) fn at(&self, id: NodeIdentity, year: u64) -> Option<[f64; 3]> {
        let frames = self.frames.get(id.key)?;
        if let Some(last) = frames.last().filter(|frame| frame.year <= year) {
            return Some(last.radii);
        }
        let end = frames.partition_point(|frame| frame.year <= year);
        end.checked_sub(1).map(|index| frames[index].radii)
    }

    pub(super) fn changed(&self, id: NodeIdentity, lo: u64, hi: u64) -> bool {
        self.frames.get(id.key).is_some_and(|frames| {
            let Some(last) = frames.last() else {
                return false;
            };
            if last.year <= lo {
                return false;
            }
            if last.year <= hi {
                return true;
            }
            let first = frames.partition_point(|frame| frame.year <= lo);
            frames.get(first).is_some_and(|frame| frame.year <= hi)
        })
    }

    pub(super) fn record(
        &mut self,
        id: NodeIdentity,
        year: u64,
        mut radii: [f64; 3],
        tolerance: f64,
    ) {
        let frames = self.frames.entry(id.key).unwrap().or_default();
        if let Some(last) = frames.last() {
            for (radius, old) in radii.iter_mut().zip(last.radii) {
                *radius = radius.max(old);
            }
            if !radii
                .iter()
                .zip(last.radii)
                .any(|(new, old)| new - old > tolerance)
            {
                return;
            }
            assert!(last.year < year, "radius keyframe year must increase");
        }
        frames.push(Frame { year, radii });
        self.pending.insert(id);
    }
}

impl Specimen {
    pub(super) fn record_widths(&mut self, year: u64, first_birth: usize) {
        let t = self.timeline.as_mut().unwrap();
        let changed = t.pipes.changed(&self.tree);
        let locals = t
            .widths
            .changed(&self.tree, &self.identities, &changed, &t.pipes);
        for i in changed {
            let (distal, proximal) = t.pipes.width(i);
            let n = &self.tree.nodes[i];
            self.keyframes.record(
                n.identity,
                year,
                [distal, proximal, n.base_radius],
                t.traits.resize_tolerance,
            );
        }
        for (i, radii) in locals {
            self.keyframes.record(
                self.tree.nodes[i].identity,
                year,
                radii,
                t.traits.resize_tolerance,
            );
        }
        // A shoot can be born and shed in the same slice. It still has a birth
        // frame, but never appears in a living read.
        for i in first_birth..self.tree.nodes.len() {
            let n = &self.tree.nodes[i];
            if !self.keyframes.frames.contains_key(n.identity.key) {
                self.keyframes.record(
                    n.identity,
                    year,
                    [n.radius, n.start_radius, n.base_radius],
                    t.traits.resize_tolerance,
                );
            }
        }
        #[cfg(test)]
        {
            self.cost.keyframe_pipes = t.pipes.visited;
            self.cost.keyframe_widths = t.widths.visited;
        }
    }

    pub(super) fn seed_widths(&mut self) {
        let n = &self.tree.nodes[0];
        self.keyframes.record(
            n.identity,
            0,
            [n.radius, n.start_radius, n.base_radius],
            0.0,
        );
    }

    pub(super) fn finish_widths(&mut self) -> Result<()> {
        #[cfg(test)]
        let clock = std::time::Instant::now();
        for id in std::mem::take(&mut self.keyframes.pending) {
            let i = self.identities[id.key];
            let n = &mut self.tree.nodes[i];
            if n.shoot.death_year.is_some() {
                continue;
            }
            let frame = self.keyframes.frames[id.key].last().unwrap();
            [n.radius, n.start_radius, n.base_radius] = frame.radii;
            self.tree.validate_range(i..i + 1, true)?;
            #[cfg(test)]
            {
                if self.tree.nodes[i].kind == NodeKind::Structural {
                    self.cost.pipes += 1;
                } else {
                    self.cost.widths += 1;
                }
            }
        }
        #[cfg(test)]
        {
            self.cost.finalizing = clock.elapsed();
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests;
