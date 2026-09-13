//! Cache the pipe reductions. Insertion invalidates only its ancestor paths.
//! A changed trunk scale still writes each affected radius; it never re-sums
//! unchanged forks. The taper reference is the authored height in metres.
use super::*;
use crate::tree::NodeKind;
use std::{cmp::Ordering, collections::BTreeSet};

#[derive(Clone, Copy, Debug)]
struct Scale(f64);
impl PartialEq for Scale {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}
impl Eq for Scale {}
impl PartialOrd for Scale {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Scale {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.total_cmp(&other.0)
    }
}

#[derive(Clone, Default)]
pub(crate) struct Pipes {
    #[cfg(test)]
    pub(crate) visited: usize,
    children: Vec<Vec<usize>>,
    distal: Vec<f64>,
    proximal: Vec<f64>,
    shed: Vec<f64>,
    history: super::history::History,
    epochs: Vec<usize>,
    floors: Vec<(f64, f64)>,
    pending: BTreeSet<usize>,
    finalized: usize,
    waiting: BTreeSet<(Scale, usize)>,
    thresholds: Vec<Scale>,
    dirty: BTreeSet<usize>,
}
impl Pipes {
    #[cfg(test)]
    pub fn update(
        &mut self,
        tree: &mut Tree,
        height: f64,
        reference_height: f64,
        params: RadiusParams,
    ) -> Result<Vec<usize>> {
        self.record(tree, height, reference_height, params)?;
        self.finish(tree)
    }
    /// Update only changed structural paths; output radii are not inputs.
    pub fn record(
        &mut self,
        tree: &Tree,
        height: f64,
        reference_height: f64,
        params: RadiusParams,
    ) -> Result<()> {
        let p = params.resolved()?;
        let first = self.children.len();
        let count = tree.nodes.len();
        self.children.resize_with(count, Vec::new);
        self.distal.resize(count, 1.0);
        self.proximal.resize(count, 1.0);
        self.shed.resize(count, 0.0);
        self.epochs.resize(count, self.history.len());
        self.floors.resize(count, (0.0, 0.0));
        self.thresholds.resize(count, Scale(0.0));
        let mut dirty = std::mem::take(&mut self.dirty);
        for i in first..count {
            if tree.nodes[i].kind != NodeKind::Structural
                || tree.nodes[i].shoot.death_year.is_some()
            {
                continue;
            }
            self.floors[i] = (tree.nodes[i].radius, tree.nodes[i].start_radius);
            dirty.insert(i);
            if let Some(parent) = tree.nodes[i].parent {
                let parent = parent as usize;
                self.children[parent].push(i);
                self.shed[i] = (self.shed[parent]
                    + p.length_taper
                        * tree.nodes[parent].position.distance(tree.nodes[i].position)
                        / reference_height.max(1e-6))
                .min(12.0);
                let mut at = Some(parent);
                while let Some(j) = at {
                    if !dirty.insert(j) {
                        break;
                    }
                    at = tree.nodes[j].parent.map(|n| n as usize);
                }
            }
        }
        for &i in &dirty {
            self.floors[i] = self.width(i);
            self.epochs[i] = self.history.len();
        }
        self.pending.extend(&dirty);
        // Structural children are birth ordered even when local storage lies
        // between them. Optional packing preserves that order.
        for &i in dirty.iter().rev() {
            let carried: f64 = self.children[i]
                .iter()
                .map(|&j| self.proximal[j].powf_fixed(p.fork_exponent))
                .sum();
            self.distal[i] = if carried > 0.0 {
                carried.powf_fixed(1.0 / p.fork_exponent)
            } else {
                1.0
            };
            self.proximal[i] = self.distal[i]
                * tree.nodes[i].parent.map_or(1.0, |parent| {
                    (self.shed[i] - self.shed[parent as usize]).exp_fixed()
                });
        }
        if count == 0 {
            return Ok(());
        }
        let scale = p.trunk_radius * height.max(1e-6) / self.distal[0];
        self.history.push(scale);
        Ok(())
    }
    pub fn contains(&self, i: usize) -> bool {
        i < self.floors.len()
    }
    pub fn width(&self, i: usize) -> (f64, f64) {
        let scale = self.history.maximum(self.epochs[i]);
        let distal = (self.distal[i] * scale).max(self.floors[i].0);
        let proximal = (self.proximal[i] * scale).max(self.floors[i].1).max(distal);
        (distal, proximal)
    }
    /// Select only wood whose recorded maximum can have changed. Reading the
    /// annual solve must not materialize the advance's output radii.
    pub fn changed(&mut self, tree: &Tree) -> Vec<usize> {
        #[cfg(test)]
        {
            self.visited = 0;
        }
        let mut changed = Vec::new();
        let mut write = std::mem::take(&mut self.pending);
        let maximum = self.history.maximum(self.finalized);
        while let Some(&(threshold, i)) = self.waiting.first() {
            if threshold.0 > maximum {
                break;
            }
            self.waiting.pop_first();
            write.insert(i);
        }
        self.finalized = self.history.len();
        for i in write {
            if tree.nodes[i].shoot.death_year.is_some() {
                continue;
            }
            self.waiting.remove(&(self.thresholds[i], i));
            #[cfg(test)]
            {
                self.visited += 1;
            }
            let (distal, proximal) = self.width(i);
            changed.push(i);
            self.thresholds[i] = Scale((distal / self.distal[i]).min(proximal / self.proximal[i]));
            self.waiting.insert((self.thresholds[i], i));
        }
        changed
    }
    #[cfg(test)]
    pub fn finish(&mut self, tree: &mut Tree) -> Result<Vec<usize>> {
        let mut changed = Vec::new();
        for i in self.changed(tree) {
            let (distal, proximal) = self.width(i);
            let n = &mut tree.nodes[i];
            if (distal, proximal) != (n.radius, n.start_radius) {
                n.radius = distal;
                n.start_radius = proximal;
                changed.push(i);
            }
            tree.validate_range(i..i + 1, true)?;
        }
        Ok(changed)
    }
    /// Invalidate only surviving ancestor paths; node indices never change.
    pub fn remove_dead(&mut self, tree: &Tree, dead: &[usize]) {
        for &i in dead {
            if tree.nodes[i].kind != NodeKind::Structural || i >= self.children.len() {
                continue;
            }
            self.pending.remove(&i);
            self.dirty.remove(&i);
            self.waiting.remove(&(self.thresholds[i], i));
            if let Some(parent) = tree.nodes[i].parent {
                self.children[parent as usize].retain(|&child| child != i);
            }
            let mut at = tree.nodes[i].parent;
            while let Some(parent) = at {
                let n = &tree.nodes[parent as usize];
                if n.shoot.death_year.is_none() && !self.dirty.insert(parent as usize) {
                    break;
                }
                at = n.parent;
            }
        }
    }

    /// Preserve unchanged fork reductions through compaction. Only ancestors of
    /// a removed structural child are invalidated; no full pipe solve on a cut.
    #[cfg(test)]
    pub fn remap(&mut self, tree: &Tree, map: &[Option<u32>]) {
        self.pending = self
            .pending
            .iter()
            .filter_map(|&i| map[i].map(|i| i as usize))
            .collect();
        self.dirty = self
            .dirty
            .iter()
            .filter_map(|&i| map[i].map(|i| i as usize))
            .collect();
        for (i, n) in tree.nodes.iter().enumerate() {
            if n.kind == NodeKind::Structural && map[i].is_none() {
                let mut at = n.parent.map(|p| p as usize);
                while let Some(j) = at {
                    if let Some(new) = map[j] {
                        self.dirty.insert(new as usize);
                    }
                    at = tree.nodes[j].parent.map(|p| p as usize);
                }
            }
        }
        let count = map.iter().flatten().count();
        let mut children = Vec::with_capacity(self.children.capacity().max(count));
        children.resize_with(count, Vec::new);
        for (i, old) in std::mem::take(&mut self.children).into_iter().enumerate() {
            if let Some(new) = map[i] {
                children[new as usize] = old
                    .into_iter()
                    .filter_map(|j| map[j].map(|v| v as usize))
                    .collect();
            }
        }
        self.children = children;
        self.waiting = self
            .waiting
            .iter()
            .filter_map(|&(v, i)| map[i].map(|i| (v, i as usize)))
            .collect();
        remap_values(&mut self.thresholds, map, count, Scale(0.0));
        remap_values(&mut self.epochs, map, count, self.history.len());
        remap_values(&mut self.floors, map, count, (0.0, 0.0));
        for values in [&mut self.distal, &mut self.proximal, &mut self.shed] {
            remap_values(values, map, count, 0.0);
        }
    }
}

#[cfg(test)]
fn remap_values<T: Copy>(values: &mut Vec<T>, map: &[Option<u32>], count: usize, default: T) {
    let mut remapped = Vec::with_capacity(values.capacity().max(count));
    remapped.resize(count, default);
    for (i, value) in values.iter().enumerate() {
        if let Some(new) = map[i] {
            remapped[new as usize] = *value;
        }
    }
    *values = remapped;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{math::Vec3, tree::Node};
    #[test]
    fn rising_scale_does_not_visit_wood_below_its_previous_width() {
        let mut tree = Tree::default();
        for i in 0..1000 {
            tree.nodes.push(Node {
                parent: (i > 0).then_some(0),
                branch: i,
                position: Vec3::new(i as f64, 1.0, 0.0),
                radius: 1000.0,
                start_radius: 1000.0,
                ..Node::root()
            });
        }
        tree.crossover = tree.nodes.len();
        let mut pipes = Pipes::default();
        pipes
            .update(&mut tree, 1.0, 24.0, RadiusParams::default())
            .unwrap();
        let old = tree.clone();
        pipes
            .update(&mut tree, 2.0, 24.0, RadiusParams::default())
            .unwrap();
        assert_eq!(tree, old);
        assert_eq!(pipes.visited, 0, "unchanged structural wood was scanned");
        let mut eager = tree.clone();
        Pipes::default()
            .update(&mut eager, 1e9, 24.0, RadiusParams::default())
            .unwrap();
        pipes
            .update(&mut tree, 1e9, 24.0, RadiusParams::default())
            .unwrap();
        assert_eq!(
            tree, eager,
            "the index must wake wood when its width is exceeded"
        );
    }
}
