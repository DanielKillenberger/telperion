//! Cache the pipe reductions. Insertion invalidates only its ancestor paths.
//! A changed trunk scale still writes each affected radius; it never re-sums
//! unchanged forks. The taper reference is the authored height in metres.
use super::*;
use std::collections::BTreeSet;

#[derive(Clone, Default)]
pub(crate) struct Pipes {
    children: Vec<Vec<usize>>,
    distal: Vec<f64>,
    proximal: Vec<f64>,
    shed: Vec<f64>,
    scale: f64,
}
impl Pipes {
    pub fn update(
        &mut self,
        tree: &mut Tree,
        height: f64,
        reference_height: f64,
        params: RadiusParams,
    ) -> Result<()> {
        let p = params.resolved()?;
        let first = self.children.len();
        let count = tree.crossover;
        self.children.resize_with(count, Vec::new);
        self.distal.resize(count, 1.0);
        self.proximal.resize(count, 1.0);
        self.shed.resize(count, 0.0);
        let mut dirty = BTreeSet::new();
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
            return Ok(());
        }
        let scale = p.trunk_radius * height.max(1e-6) / self.distal[0];
        let write = |i: usize, tree: &mut Tree| {
            tree.nodes[i].radius = (self.distal[i] * scale).max(tree.nodes[i].radius);
            tree.nodes[i].start_radius = (self.proximal[i] * scale)
                .max(tree.nodes[i].start_radius)
                .max(tree.nodes[i].radius);
        };
        if scale != self.scale {
            for i in 0..count {
                write(i, tree);
            }
        } else {
            for i in dirty {
                write(i, tree);
            }
        }
        self.scale = scale;
        Ok(())
    }
}
