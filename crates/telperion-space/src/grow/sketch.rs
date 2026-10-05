//! The rough layout grown with the tree (fn-197, host decisions
//! 2026-10-05). At the end of each cycle every growth unit grown in it is
//! laid on from where its axis's walk stood (`geometry::Layer`), each new
//! axis framed from its parent as the final lay frames it, every phytomer
//! at the scale its draws give it so far. Sag is left out, erection and
//! the base's straightening are taken at the axis's age when the unit
//! grew, and the fade of a straightening over the axis's reach at its
//! reach so far: what the final lay knows only once the tree has grown.
//! The leaves grown in the cycle then shade the sky for the buds that grow
//! in the next. What it writes into the phytomers and axes (their tip,
//! frame, scale and rank) is all written again once the tree has grown.
use super::Grower;
use crate::allocation::{sizes, vigours, Link, TIP};
use crate::error::{Error, Result};
use crate::geometry::{bend, dominance, frame, Layer};
use crate::light::{Field, Light};
use crate::structure::{Origin, Vec3};

/// The rough layout and the light it casts.
pub(super) struct Sketch {
    light: Light,
    pub(super) pencils: Vec<Pencil>,
}

/// Where an axis's rough layout stands.
#[derive(Debug, Clone, Copy)]
pub(super) struct Pencil {
    pub(super) layer: Layer,
    /// Its scale at its base, and its draws' presence over the growth
    /// units laid.
    pub(super) base_scale: f64,
    pub(super) running: f64,
    /// Its nodes counted by their presence, the units and phytomers laid.
    rank: f64,
    units: usize,
    laid: usize,
    trunk: bool,
    /// The light at its tip, from the last cycle's leaves, and its
    /// growth unit's size among its siblings this cycle.
    pub(super) light: f64,
    size: f64,
    /// Its own present wood in metres, its remembered carbon balance and
    /// what its subtrees' survival of shedding makes of its unit this
    /// cycle (`balance.rs`).
    pub(super) wood: f64,
    pub(super) memory: f64,
    pub(super) kept: f64,
}

impl Sketch {
    pub fn new(light: Light) -> Self {
        Self {
            light,
            pencils: Vec::new(),
        }
    }
}

impl Grower<'_> {
    /// Lays the units the `advanced` axes grew in `cycle`, then frames and
    /// lays the axes made in it: parents before children, so every frame
    /// reads wood already laid.
    pub(super) fn sketch(&mut self, cycle: u32, advanced: &[usize]) -> Result<()> {
        let mut sketch = self.sketch.take().expect("sketching");
        let mut leaves = std::mem::take(&mut self.leaves);
        leaves.clear();
        let first_new = sketch.pencils.len();
        for &i in advanced.iter().filter(|&&i| i < first_new) {
            self.draw(&mut sketch, i, cycle, &mut leaves)?;
        }
        for i in first_new..self.axes.len() {
            let pencil = self.pencil(&sketch, i);
            sketch.pencils.push(pencil);
            self.draw(&mut sketch, i, cycle, &mut leaves)?;
        }
        self.leaves = leaves;
        self.sketch = Some(sketch);
        Ok(())
    }

    /// Sweeps the cycle's leaves from the sky and reads the light at every
    /// living apex's tip.
    pub(super) fn shade(&mut self) {
        let mut sketch = self.sketch.take().expect("sketching");
        let tips: Vec<Vec3> = self.live.iter().map(|a| self.tip(a.axis)).collect();
        let field = Field::new(&sketch.light, &self.leaves, &tips);
        for (apex, &tip) in self.live.iter().zip(&tips) {
            sketch.pencils[apex.axis].light = field.at(tip);
        }
        self.sketch = Some(sketch);
    }

    /// The light axis `i`'s apex grows in: from the leaves of the cycle
    /// before, or whole where nothing shades.
    pub(super) fn light(&self, i: usize) -> f64 {
        self.sketch
            .as_ref()
            .and_then(|s| s.pencils.get(i))
            .map_or(1.0, |p| p.light)
    }

    /// Axis `i`'s growth unit's size this cycle (`allocation.rs`); whole
    /// where nothing shades.
    pub(super) fn size(&self, i: usize) -> f64 {
        self.sketch
            .as_ref()
            .and_then(|s| s.pencils.get(i))
            .map_or(1.0, |p| p.size)
    }

    /// Gives each living apex its unit's size this cycle from the vigour
    /// the tree's light allots it per presence, against the mean of the
    /// growing buds of its PA (`allocation.rs`): before any apex grows.
    pub(super) fn allot(&mut self) {
        let species = self.species;
        let Some(sketch) = self.sketch.as_mut() else {
            return;
        };
        if species.states.iter().all(|s| s.shade_size == 0.0) {
            return;
        }
        let n = sketch.pencils.len();
        let links: Vec<Link> = self.axes[..n]
            .iter()
            .map(|a| match a.origin {
                Origin::Seed => Link {
                    parent: None,
                    node: 0,
                },
                Origin::Lateral { parent, node, .. } => Link {
                    parent: Some(parent),
                    node,
                },
                Origin::Continuation { parent } | Origin::Relay { parent, .. } => Link {
                    parent: Some(parent),
                    node: TIP,
                },
            })
            .collect();
        let lambda: Vec<f64> = self.axes[..n]
            .iter()
            .map(|a| species.states[a.pa].apical_control)
            .collect();
        let (mut lit, mut presence, mut psi) = (vec![0.0; n], vec![0.0; n], vec![0.0; n]);
        for apex in self.live.iter().filter(|a| a.axis < n) {
            let p = &sketch.pencils[apex.axis];
            // Its presence this cycle includes its subtrees' survival of
            // shedding (`balance.rs`), so a bud about to be shed weighs
            // nothing as it goes.
            presence[apex.axis] = p.base_scale * p.running * p.kept;
            lit[apex.axis] = presence[apex.axis] * p.light;
            psi[apex.axis] = species.states[self.axes[apex.axis].pa].shade_size;
        }
        let got = vigours(&links, &lit, &lambda);
        let order: Vec<usize> = self.axes[..n].iter().map(|a| a.pa).collect();
        let sized = sizes(&got, &presence, &psi, &order);
        for apex in self.live.iter().filter(|a| a.axis < n) {
            sketch.pencils[apex.axis].size = sized[apex.axis];
        }
    }

    fn tip(&self, i: usize) -> Vec3 {
        let axis = &self.axes[i];
        axis.phytomers.last().map_or(axis.base, |p| p.tip)
    }

    /// A new axis's pencil, at its frame, sized as `geometry::scale` sizes
    /// it from what its draws have made of it so far.
    fn pencil(&mut self, sketch: &Sketch, i: usize) -> Pencil {
        let framed = frame(&self.axes, self.species, i);
        let axis = &self.axes[i];
        let made = self.draws[i].birth[0] * self.draws[i].birth[1];
        let curve = match axis.origin {
            Origin::Continuation { parent } | Origin::Relay { parent, .. } => {
                sketch.pencils[parent].layer.curve()
            }
            _ => Vec3::default(),
        };
        let (inherited, vigour, trunk) = match axis.origin {
            Origin::Seed => (1.0, made, true),
            Origin::Lateral { parent, node, .. } => {
                (self.axes[parent].phytomers[node].scale, made, false)
            }
            Origin::Continuation { parent } | Origin::Relay { parent, .. } => {
                let p = &sketch.pencils[parent];
                (p.base_scale, p.running * made, p.trunk)
            }
        };
        let share = match axis.origin {
            Origin::Lateral { .. } => {
                dominance(self.species.states[axis.pa].form.dominance, axis.lineage)
            }
            _ => 1.0,
        };
        let axis = &mut self.axes[i];
        (axis.base, axis.heading, axis.side) = framed;
        Pencil {
            layer: Layer::bent(framed, curve),
            base_scale: inherited * vigour * share,
            running: 1.0,
            rank: 0.0,
            units: 0,
            laid: 0,
            trunk,
            light: 1.0,
            size: 1.0,
            wood: 0.0,
            memory: 1.0,
            kept: 1.0,
        }
    }

    /// Lays axis `i`'s units not yet laid, as `presence::assign` will size
    /// them, and gathers their leaves.
    fn draw(
        &mut self,
        sketch: &mut Sketch,
        i: usize,
        cycle: u32,
        leaves: &mut Vec<(Vec3, f64)>,
    ) -> Result<()> {
        let mut pencil = sketch.pencils[i];
        let draws = &self.draws[i];
        let axis = &mut self.axes[i];
        let state = &self.species.states[axis.pa];
        let years = f64::from(cycle.saturating_sub(axis.birth)) - draws.sleep;
        let bent = bend(axis, state, years);
        while pencil.units < draws.units.len() {
            let k = pencil.units;
            let [survive, persist] = draws.units[k];
            let grown = if k == 0 { 1.0 - draws.sleep } else { 1.0 };
            let present = pencil.running * survive;
            let size = draws.sizes.get(k).copied().unwrap_or(1.0);
            let unit = |p: &crate::structure::Phytomer| (p.cycle - axis.birth - 1) as usize == k;
            let end = pencil.laid
                + axis.phytomers[pencil.laid..]
                    .iter()
                    .take_while(|p| unit(p))
                    .count();
            for j in pencil.laid..end {
                let p = &mut axis.phytomers[j];
                // Unsized: what it bears inherits its scale without its size.
                p.scale = pencil.base_scale * present * draws.nodes[j] * grown;
                p.size = size;
                p.rank = pencil.rank;
                pencil.rank += draws.nodes[j] * grown;
            }
            let total = pencil.layer.run()
                + axis.phytomers[pencil.laid..end]
                    .iter()
                    .map(|p| p.scale * p.size)
                    .sum::<f64>();
            for j in pencil.laid..end {
                let p = &mut axis.phytomers[j];
                // Laid at its size, then left unsized for what it bears.
                let bare = p.scale;
                p.scale = bare * p.size;
                pencil
                    .layer
                    .step(p, state, (bent, total), None, pencil.trunk)
                    .map_err(|height| Error::BelowGround {
                        axis: i,
                        pa: axis.pa,
                        birth: axis.birth,
                        base: axis.base.z,
                        height,
                    })?;
                leaves.push((p.tip, p.scale * state.leaf_area));
                pencil.wood += state.internode * p.scale;
                p.scale = bare;
            }
            pencil.laid = end;
            pencil.running = present * persist;
            pencil.units += 1;
        }
        axis.rank = pencil.rank;
        sketch.pencils[i] = pencil;
        Ok(())
    }
}
