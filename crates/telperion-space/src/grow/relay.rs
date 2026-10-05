//! The full lay grown with the tree (fn-197, host decision 8): every
//! `RELAY_EVERY` cycles the tree as shedding would leave it then, living
//! wood and dead wood within its PA's delay, each sized by its fade, is
//! sized, thickened and laid with sag as the final lay lays it, at the
//! tree's age then. Its positions replace the rough layout's, and each
//! axis's rough walk carries on from where the full lay ended it, so
//! light reads sagged boughs where they hang. Wood shedding has dropped
//! is left where the rough layout put it.
use super::Grower;
use crate::error::Result;
use crate::geometry::{place, scale};
use crate::girth::{thicken, Girth};
use crate::presence::assign;
use crate::sag;
use crate::shed::{shed, standing};
use crate::structure::{Origin, Structure};

/// The cycles between full lays: an engine constant, never a setting, so
/// no walk crosses one (fn-197 step 3b measures its cost and its gap).
pub(crate) const RELAY_EVERY: u32 = 10;

impl Grower<'_> {
    /// Lays the living tree in full at `cycle`.
    pub(super) fn relay(&mut self, cycle: u32) -> Result<()> {
        // The tree as shedding would leave it at `cycle`: living wood and
        // dead wood still within its PA's delay, each sized by its fade, so
        // a branch that dies changes the lay by degree.
        let standing = standing(&self.axes, self.species, cycle);
        let mut index = vec![usize::MAX; self.axes.len()];
        let (mut axes, mut draws, mut from) = (Vec::new(), Vec::new(), Vec::new());
        for i in (0..self.axes.len()).filter(|&i| standing[i]) {
            let mut axis = self.axes[i].clone();
            match &mut axis.origin {
                Origin::Seed => {}
                Origin::Continuation { parent }
                | Origin::Lateral { parent, .. }
                | Origin::Relay { parent, .. } => *parent = index[*parent],
            }
            index[i] = axes.len();
            axes.push(axis);
            draws.push(self.draws[i].clone());
            from.push(i);
        }
        // The apexes living now, as the grown tree marks its own at the end.
        for apex in &self.live {
            if index[apex.axis] != usize::MAX {
                let draws = &mut draws[index[apex.axis]];
                (draws.alive, draws.living) = (true, 1.0);
            }
        }
        assign(&mut axes, &draws);
        let shed = shed(axes, self.species, cycle);
        let from: Vec<usize> = (0..from.len())
            .filter(|&j| shed.index[j] != usize::MAX)
            .map(|j| from[j])
            .collect();
        let mut tree = Structure {
            age: cycle,
            pas: self.species.states.len(),
            axes: shed.axes,
        };
        scale(&mut tree, self.species);
        thicken(&mut tree, self.species, &Girth::default());
        let mut layers = place(&mut tree, self.species, None)?;
        if sag::any(self.species) {
            let levers = sag::levers(&tree, self.species);
            layers = place(&mut tree, self.species, Some(&levers))?;
        }
        let sketch = self.sketch.as_mut().expect("sketching");
        for ((laid, layer), &i) in tree.axes.iter().zip(layers).zip(&from) {
            let axis = &mut self.axes[i];
            (axis.base, axis.heading, axis.side) = (laid.base, laid.heading, laid.side);
            for (p, q) in axis.phytomers.iter_mut().zip(&laid.phytomers) {
                (p.tip, p.heading, p.side) = (q.tip, q.heading, q.side);
            }
            sketch.pencils[i].layer = layer;
        }
        // The cycle's leaves, where the full lay put them.
        self.leaves.clear();
        for axis in &self.axes {
            let area = self.species.states[axis.pa].leaf_area;
            for p in axis.phytomers.iter().rev().take_while(|p| p.cycle == cycle) {
                self.leaves.push((p.tip, p.scale * p.size * area));
            }
        }
        Ok(())
    }
}
