//! Resolving one chain (fn-209, option 4): its girth, its lay before it
//! bends, its levers from its own wood and its borne buds' committed mass
//! and moment, and its lay under them. The chain is worked on a slice of
//! its own: its bearer's axis as laid, then its own axes, renumbered, so
//! chains of one order are worked at once and nothing outside the chain
//! is written until it is done.
use super::girth::bud;
use super::Bounded;
use crate::error::{Error, Result};
use crate::geometry::{bud_frame, lay_one, reaches};
use crate::sag::{any, gather, Borne};
use crate::structure::{Axis, Origin};

/// A resolved chain: each of its axes' index, as laid before bending and
/// as laid.
pub(crate) struct Done {
    axes: Vec<(usize, Axis, Axis)>,
}

impl Bounded<'_> {
    pub(super) fn work(&self, c: usize) -> Result<Done> {
        let chain = &self.chains[c];
        if let Some(p) = chain.parent {
            assert!(self.resolved[p], "a chain is resolved after its bearer");
        }
        // The bearer's axis, where the chain is a lateral's, then its own.
        let lead = match self.bent.axes[chain.axes[0]].origin {
            Origin::Lateral { parent, .. } => Some(parent),
            _ => None,
        };
        let global: Vec<usize> = lead.into_iter().chain(chain.axes.iter().copied()).collect();
        let first = usize::from(lead.is_some());
        let slice = |tree: &[Axis]| -> Vec<Axis> {
            global
                .iter()
                .enumerate()
                .map(|(j, &g)| {
                    let mut axis = tree[g].clone();
                    axis.origin = renumbered(axis.origin, &global, j < first);
                    axis
                })
                .collect()
        };
        let (mut unbent, mut bent) = (slice(&self.unbent.axes), slice(&self.bent.axes));
        let radii = self.radii(c, &unbent, first, &global);
        for tree in [&mut unbent, &mut bent] {
            for (axis, radii) in tree.iter_mut().zip(&radii).skip(first) {
                for (p, &r) in axis.phytomers.iter_mut().zip(radii) {
                    p.radius = r;
                }
            }
        }
        let reaches = reaches(&unbent);
        let (age, trunk) = (self.bent.age, chain.record.is_none());
        let at = |j: usize| (j, age, trunk);
        let named = |e: Error| match e {
            Error::BelowGround {
                axis,
                pa,
                birth,
                base,
                height,
            } => Error::BelowGround {
                axis: global[axis],
                pa,
                birth,
                base,
                height,
            },
            e => e,
        };
        let mut layers = vec![None; global.len()];
        for j in first..global.len() {
            let layer = lay_one(&mut unbent, self.species, at(j), &layers, &reaches, None);
            layers[j] = Some(layer.map_err(named)?);
        }
        if any(self.species) {
            let levers = self.levers(c, &unbent, first, &global);
            let mut layers = vec![None; global.len()];
            for j in first..global.len() {
                let load = Some(&levers[j - first][..]);
                let layer = lay_one(&mut bent, self.species, at(j), &layers, &reaches, load);
                layers[j] = Some(layer.map_err(named)?);
            }
        } else {
            bent.clone_from(&unbent);
        }
        let axes = global
            .iter()
            .zip(unbent.into_iter().zip(bent))
            .skip(first)
            .map(|(&g, (u, b))| (g, u, b))
            .collect();
        Ok(Done { axes })
    }

    /// The chain's levers on its unbent slice: its own wood, and each bud
    /// it bore by its committed mass at its bud's place and its committed
    /// moment along its bud's heading.
    fn levers(
        &self,
        c: usize,
        unbent: &[Axis],
        first: usize,
        global: &[usize],
    ) -> Vec<Vec<crate::sag::Lever>> {
        let borne: Vec<Borne> = self.chains[c]
            .borne
            .iter()
            .map(|&r| {
                let record = &self.records[r];
                let (origin, pa, lineage) = bud(record);
                let origin = renumbered(origin, global, false);
                let (base, heading) = bud_frame(unbent, self.species, (origin, pa, lineage));
                let row = self.rows.lateral(record.pa, record.birth);
                let s = self.base_scale(record);
                let mass = row.mass * s.powi(3);
                let along = row.moment * s.powi(4);
                let Origin::Lateral { parent, node, .. } = origin else {
                    unreachable!("a record is a lateral")
                };
                Borne {
                    axis: parent,
                    node,
                    mass,
                    moment: base * mass + heading * along,
                }
            })
            .collect();
        let members: Vec<usize> = (first..unbent.len()).collect();
        let trunk = vec![self.chains[c].record.is_none(); unbent.len()];
        gather(
            unbent,
            self.species,
            self.bent.age,
            (&members, &trunk),
            &borne,
        )
    }

    /// Writes a resolved chain into the tree.
    pub(super) fn apply(&mut self, c: usize, done: Done) {
        for (g, unbent, bent) in done.axes {
            for (tree, axis) in [(&mut self.unbent, unbent), (&mut self.bent, bent)] {
                let at = &mut tree.axes[g];
                at.phytomers = axis.phytomers;
                (at.base, at.heading, at.side) = (axis.base, axis.heading, axis.side);
            }
        }
        self.resolved[c] = true;
    }
}

/// An origin renumbered into a chain's slice; the bearer's own, which
/// the slice does not hold, as the seed's.
fn renumbered(origin: Origin, global: &[usize], lead: bool) -> Origin {
    let at = |p: usize| global.binary_search(&p).expect("in the slice");
    match origin {
        _ if lead => Origin::Seed,
        Origin::Seed => Origin::Seed,
        Origin::Lateral {
            parent,
            node,
            slot,
            whorl,
            woken,
        } => Origin::Lateral {
            parent: at(parent),
            node,
            slot,
            whorl,
            woken,
        },
        Origin::Continuation { parent } => Origin::Continuation { parent: at(parent) },
        Origin::Relay { parent, node } => Origin::Relay {
            parent: at(parent),
            node,
        },
    }
}
