//! The full lay grown with the tree (fn-197, host decision 8): every
//! `RELAY_EVERY` cycles the living part of the tree, every axis with a
//! living apex and all that bears it, is sized, thickened and laid with
//! sag as the final lay lays it, at the tree's age then. Its positions
//! replace the rough layout's, and each axis's rough walk carries on from
//! where the full lay ended it, so light reads sagged boughs where they
//! hang. Wood with nothing living in it casts no leaves and is left where
//! the rough layout put it; it lends no load or girth to the lay.
use super::Grower;
use crate::error::Result;
use crate::geometry::{place, scale};
use crate::girth::thicken;
use crate::presence::assign;
use crate::sag;
use crate::structure::{Origin, Structure};

/// The cycles between full lays: an engine constant, never a setting, so
/// no walk crosses one (fn-197 step 3b measures its cost and its gap).
pub(crate) const RELAY_EVERY: u32 = 10;

impl Grower<'_> {
    /// Lays the living tree in full at `cycle`.
    pub(super) fn relay(&mut self, cycle: u32) -> Result<()> {
        let count = self.axes.len();
        let mut kept = vec![false; count];
        for apex in &self.live {
            kept[apex.axis] = true;
        }
        // Parents precede children: one backward pass keeps every bearer.
        for i in (1..count).rev() {
            if kept[i] {
                if let Some(parent) = self.axes[i].origin.parent() {
                    kept[parent] = true;
                }
            }
        }
        let mut index = vec![usize::MAX; count];
        let (mut axes, mut draws, mut from) = (Vec::new(), Vec::new(), Vec::new());
        for i in (0..count).filter(|&i| kept[i]) {
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
        assign(&mut axes, &draws);
        let mut tree = Structure {
            age: cycle,
            pas: self.species.states.len(),
            axes,
        };
        scale(&mut tree, self.species);
        thicken(&mut tree, self.species);
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
