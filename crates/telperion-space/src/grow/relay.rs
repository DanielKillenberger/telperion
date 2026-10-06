//! The full lay grown with the tree (fn-197, host decision 8): every
//! `RELAY_EVERY` cycles the tree as shedding would leave it then, living
//! wood and dead wood within its PA's delay, each sized by its fade, is
//! sized, thickened and laid with sag as the final lay lays it, at the
//! tree's age then. Its positions replace the rough layout's, and each
//! axis's rough walk carries on from where the full lay ended it, so
//! light reads sagged boughs where they hang. Wood shedding has dropped
//! is left where the rough layout put it.
use super::{Grower, Pass, Stage};
use crate::cores::{map, picked, PIECE};
use crate::error::Result;
use crate::geometry::{place, scale};
use crate::girth::{thicken, Girth};
use crate::presence::{assign, Draws};
use crate::sag;
use crate::shed::{shed, standing};
use crate::structure::{Axis, Origin, Structure};

/// The cycles between full lays: an engine constant, never a setting, so
/// no walk crosses one (fn-197 step 3b measures its cost and its gap).
pub(crate) const RELAY_EVERY: u32 = 10;

impl Grower<'_> {
    /// Lays the living tree in full at `cycle`.
    pub(super) fn relay(&mut self, cycle: u32, report: &mut dyn FnMut(Stage)) -> Result<()> {
        // The tree as shedding would leave it at `cycle`: living wood and
        // dead wood still within its PA's delay, each sized by its fade, so
        // a branch that dies changes the lay by degree.
        let standing = standing(&self.axes, self.species, cycle);
        let mut index = vec![usize::MAX; self.axes.len()];
        let from: Vec<usize> = (0..self.axes.len()).filter(|&i| standing[i]).collect();
        for (j, &i) in from.iter().enumerate() {
            index[i] = j;
        }
        // Copied on every core, each renamed to the standing tree's indices.
        let (grown, made) = (&self.axes, &self.draws);
        let copied = map(
            from.chunks(PIECE).collect(),
            1,
            self.threads,
            &|_, piece| {
                let axes: Vec<Axis> = piece
                    .iter()
                    .map(|&i| {
                        let mut axis = grown[i].clone();
                        match &mut axis.origin {
                            Origin::Seed => {}
                            Origin::Continuation { parent }
                            | Origin::Lateral { parent, .. }
                            | Origin::Relay { parent, .. } => *parent = index[*parent],
                        }
                        axis
                    })
                    .collect();
                let draws: Vec<Draws> = piece.iter().map(|&i| made[i].clone()).collect();
                (axes, draws)
            },
        );
        let (mut axes, mut draws) = (
            Vec::with_capacity(from.len()),
            Vec::with_capacity(from.len()),
        );
        for (a, d) in copied {
            axes.extend(a);
            draws.extend(d);
        }
        // The apexes living now, as the grown tree marks its own at the end.
        for apex in &self.live {
            if index[apex.axis] != usize::MAX {
                let draws = &mut draws[index[apex.axis]];
                (draws.alive, draws.living) = (true, 1.0);
            }
        }
        assign(&mut axes, &draws, self.threads);
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
        scale(&mut tree, self.species, self.threads);
        thicken(&mut tree, self.species, &Girth::default(), self.threads);
        report(Stage::Pass(Pass::RelaySettle));
        let mut layers = place(&mut tree, self.species, None, self.threads)?;
        report(Stage::Pass(Pass::RelayPlace));
        if sag::any(self.species) {
            let levers = sag::levers(&tree, self.species, self.threads);
            report(Stage::Pass(Pass::RelayLevers));
            layers = place(&mut tree, self.species, Some(&levers), self.threads)?;
            report(Stage::Pass(Pass::RelaySag));
        }
        let sketch = self.sketch.as_mut().expect("sketching");
        // Written back on every core, each axis its own.
        let jobs: Vec<_> = tree
            .axes
            .iter()
            .zip(layers)
            .zip(picked(&mut self.axes, from.iter().copied()))
            .zip(picked(&mut sketch.pencils, from.iter().copied()))
            .collect();
        map(jobs, PIECE, self.threads, &|_,
                                         (
            ((laid, layer), axis),
            pencil,
        )| {
            (axis.base, axis.heading, axis.side) = (laid.base, laid.heading, laid.side);
            for (p, q) in axis.phytomers.iter_mut().zip(&laid.phytomers) {
                (p.tip, p.heading, p.side) = (q.tip, q.heading, q.side);
            }
            pencil.layer = layer;
        });
        drop(tree);
        // The cycle's leaves, where the full lay put them, gathered on
        // every core in the tree's order.
        let (axes, species) = (&self.axes, self.species);
        let pieces: Vec<&[Axis]> = axes.chunks(PIECE).collect();
        let leaves = map(pieces, 1, self.threads, &|_, piece| {
            let mut out = Vec::new();
            for axis in piece {
                let area = species.states[axis.pa].leaf_area;
                for p in axis.phytomers.iter().rev().take_while(|p| p.cycle == cycle) {
                    out.push((p.tip, p.scale * p.size * area));
                }
            }
            out
        });
        self.leaves.clear();
        for piece in leaves {
            self.leaves.extend(piece);
        }
        report(Stage::Pass(Pass::RelayWrite));
        Ok(())
    }
}
