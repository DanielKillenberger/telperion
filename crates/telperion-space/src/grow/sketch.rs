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
use crate::allocation::{sizes, Bud};
use crate::error::{Error, Result};
use crate::geometry::{bend, dominance, frame, Layer};
use crate::light::{Field, Light};
use crate::structure::{Origin, Vec3};
use std::collections::HashMap;

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
    base_scale: f64,
    running: f64,
    /// Its nodes counted by their presence, the units and phytomers laid.
    rank: f64,
    units: usize,
    laid: usize,
    trunk: bool,
    /// The light at its tip, from the last cycle's leaves, and its
    /// growth unit's size among its siblings this cycle.
    light: f64,
    size: f64,
    /// The axis that bears it and its siblings: a lateral's parent, and
    /// for an axis carried on, its first link's (none for the seed).
    bearer: usize,
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

    /// Shares this cycle's growth among the living apexes on each bearer
    /// by their light, before any of them grows.
    pub(super) fn allot(&mut self) {
        let Some(sketch) = self.sketch.as_mut() else {
            return;
        };
        let mut groups: HashMap<usize, Vec<(usize, Bud)>> = HashMap::new();
        for apex in &self.live {
            let Some(p) = sketch.pencils.get(apex.axis) else {
                continue;
            };
            let bud = Bud {
                weight: p.base_scale * p.running,
                light: p.light,
                psi: self.species.states[self.axes[apex.axis].pa].shade_size,
            };
            groups.entry(p.bearer).or_default().push((apex.axis, bud));
        }
        for members in groups.values() {
            let buds: Vec<Bud> = members.iter().map(|m| m.1).collect();
            for (&(axis, _), size) in members.iter().zip(sizes(&buds)) {
                sketch.pencils[axis].size = size;
            }
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
        let (inherited, vigour, trunk, bearer) = match axis.origin {
            Origin::Seed => (1.0, made, true, usize::MAX),
            Origin::Lateral { parent, node, .. } => {
                (self.axes[parent].phytomers[node].scale, made, false, parent)
            }
            Origin::Continuation { parent } | Origin::Relay { parent, .. } => {
                let p = &sketch.pencils[parent];
                (p.base_scale, p.running * made, p.trunk, p.bearer)
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
            layer: Layer::new(framed),
            base_scale: inherited * vigour * share,
            running: 1.0,
            rank: 0.0,
            units: 0,
            laid: 0,
            trunk,
            light: 1.0,
            size: 1.0,
            bearer,
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
                p.scale = pencil.base_scale * present * draws.nodes[j] * grown * size;
                p.rank = pencil.rank;
                pencil.rank += draws.nodes[j] * grown;
            }
            let total = pencil.layer.run()
                + axis.phytomers[pencil.laid..end]
                    .iter()
                    .map(|p| p.scale)
                    .sum::<f64>();
            for j in pencil.laid..end {
                let p = &mut axis.phytomers[j];
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
