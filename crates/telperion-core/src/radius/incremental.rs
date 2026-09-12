//! Cache the pipe reductions. Insertion invalidates only its ancestor paths.
//! A changed trunk scale still writes each affected radius; it never re-sums
//! unchanged forks. The taper reference is the authored height in metres.
use super::*;
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
    scale: f64,
    waiting: BTreeSet<(Scale, usize)>,
    thresholds: Vec<Scale>,
    dirty: BTreeSet<usize>,
}
impl Pipes {
    pub fn update(
        &mut self,
        tree: &mut Tree,
        height: f64,
        reference_height: f64,
        params: RadiusParams,
    ) -> Result<Vec<usize>> {
        #[cfg(test)]
        {
            self.visited = 0;
        }
        let p = params.resolved()?;
        let first = self.children.len();
        let count = tree.crossover;
        self.children.resize_with(count, Vec::new);
        self.distal.resize(count, 1.0);
        self.proximal.resize(count, 1.0);
        self.shed.resize(count, 0.0);
        self.thresholds.resize(count, Scale(0.0));
        let mut dirty = std::mem::take(&mut self.dirty);
        for i in first..count {
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
        // Structural insertion is birth ordered. Local storage indices are never
        // part of a reduction, including after structural/local re-indexing.
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
            return Ok(Vec::new());
        }
        let scale = p.trunk_radius * height.max(1e-6) / self.distal[0];
        let mut changed = Vec::new();
        // A scale increase need only visit nodes whose historical maximum can
        // be exceeded. Frozen, formerly wider branches stay in the ordered index.
        let mut write = dirty;
        if scale != self.scale {
            while let Some(&(threshold, i)) = self.waiting.first() {
                if threshold.0 > scale {
                    break;
                }
                self.waiting.pop_first();
                write.insert(i);
            }
        }
        for i in write {
            self.waiting.remove(&(self.thresholds[i], i));
            #[cfg(test)]
            {
                self.visited += 1;
            }
            let n = &mut tree.nodes[i];
            let distal = (self.distal[i] * scale).max(n.radius);
            let proximal = (self.proximal[i] * scale).max(n.start_radius).max(distal);
            if (distal, proximal) != (n.radius, n.start_radius) {
                n.radius = distal;
                n.start_radius = proximal;
                changed.push(i);
            }
            self.thresholds[i] =
                Scale((n.radius / self.distal[i]).min(n.start_radius / self.proximal[i]));
            self.waiting.insert((self.thresholds[i], i));
            tree.validate_range(i..i + 1, true)?;
        }
        self.scale = scale;
        Ok(changed)
    }
    /// Preserve unchanged fork reductions through compaction. Only ancestors of
    /// a removed structural child are invalidated; no full pipe solve on a cut.
    pub fn remap(&mut self, tree: &Tree, map: &[Option<u32>]) {
        let count = self.children.len();
        for i in 0..count {
            if map[i].is_none() {
                let mut at = tree.nodes[i].parent.map(|p| p as usize);
                while let Some(j) = at {
                    if let Some(new) = map[j] {
                        self.dirty.insert(new as usize);
                    }
                    at = tree.nodes[j].parent.map(|p| p as usize);
                }
            }
        }
        let mut children = Vec::new();
        for (i, old) in std::mem::take(&mut self.children).into_iter().enumerate() {
            if map[i].is_some() {
                children.push(
                    old.into_iter()
                        .filter_map(|j| map[j].map(|v| v as usize))
                        .collect(),
                );
            }
        }
        self.children = children;
        self.waiting = self
            .waiting
            .iter()
            .filter_map(|&(v, i)| map[i].map(|i| (v, i as usize)))
            .collect();
        let mut i = 0;
        self.thresholds.retain(|_| {
            let keep = map[i].is_some();
            i += 1;
            keep
        });
        for values in [&mut self.distal, &mut self.proximal, &mut self.shed] {
            let mut i = 0;
            values.retain(|_| {
                let keep = map[i].is_some();
                i += 1;
                keep
            });
        }
    }
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
