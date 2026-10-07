//! The rough layout's cycle on every core (fn-211). An axis that grew is
//! laid on from where its own walk stood, and a new axis is framed from
//! its parent alone: so the axes that grew are laid at once, then the
//! new ones a generation at a time, parents first. Their leaves are
//! gathered after, in the order laying them in turn gave, and where axes
//! cannot be laid the error is the one laying them in turn meets first.
use super::sketch::Pencil;
use super::Grower;
use crate::cores::{map, picked, PIECE};
use crate::error::{Error, Result};
use crate::geometry::{bend, dominance, frame, Layer};
use crate::presence::{relay_node, Draws};
use crate::species::Species;
use crate::structure::{Axis, Origin, Vec3};

/// A new axis's frame: base, heading and side.
type Framed = (Vec3, Vec3, Vec3);

impl Grower<'_> {
    /// Lays the units the `advanced` axes grew in `cycle`, then frames and
    /// lays the axes made in it: parents before children, so every frame
    /// reads wood already laid.
    pub(super) fn sketch(&mut self, cycle: u32, advanced: &[usize]) -> Result<()> {
        let mut sketch = self.sketch.take().expect("sketching");
        if self.fine.is_some() {
            // A fine subtree grown alone: its units tallied, never laid.
            self.tally_units(&mut sketch, advanced);
            self.sketch = Some(sketch);
            return Ok(());
        }
        let (species, threads) = (self.species, self.threads);
        let first_new = sketch.pencils.len();
        let grew: Vec<usize> = advanced
            .iter()
            .copied()
            .filter(|&i| i < first_new)
            .collect();
        let from: Vec<usize> = grew.iter().map(|&i| sketch.pencils[i].laid).collect();
        // The axes that grew, each from its own walk: the first error in
        // their order.
        let mut order: Vec<(usize, usize)> =
            grew.iter().enumerate().map(|(k, &i)| (i, k)).collect();
        order.sort_unstable();
        let at = || order.iter().map(|o| o.0);
        let jobs: Vec<_> = order
            .iter()
            .zip(
                picked(&mut self.axes, at())
                    .into_iter()
                    .zip(picked(&mut sketch.pencils, at())),
            )
            .collect();
        let draws = &self.draws;
        let failed = map(jobs, PIECE, threads, &|_, (&(i, k), (axis, pencil))| {
            draw((axis, pencil), &draws[i], species, (i, cycle))
                .err()
                .map(|e| (k, e))
        });
        if let Some((_, error)) = failed.into_iter().flatten().min_by_key(|f| f.0) {
            return Err(error);
        }
        // The new axes, a generation at a time: the first error in index
        // order, as no axis before it depends on what follows it.
        let mut generation = vec![0usize; self.axes.len() - first_new];
        let mut generations: Vec<Vec<usize>> = Vec::new();
        for i in first_new..self.axes.len() {
            let g = match self.axes[i].origin.parent() {
                Some(p) if p >= first_new => generation[p - first_new] + 1,
                _ => 0,
            };
            generation[i - first_new] = g;
            if generations.len() <= g {
                generations.push(Vec::new());
            }
            generations[g].push(i);
        }
        // A relay stands at its node, which its parent's last unit, grown
        // whole by now, decides (`presence::assign`): read off the parent's
        // phytomers and draws, which laying leaves alone.
        for i in first_new..self.axes.len() {
            if let Origin::Relay { parent, .. } = self.axes[i].origin {
                let p = &self.axes[parent];
                let share = species.states[p.pa].relay_at;
                let node = relay_node(p, &self.draws[parent], share);
                self.axes[i].origin = Origin::Relay { parent, node };
            }
        }
        let mut fresh: Vec<Option<Pencil>> = vec![None; generation.len()];
        let mut failed: Option<(usize, Error)> = None;
        for todo in generations {
            let todo: Vec<usize> = todo
                .into_iter()
                .filter(|&i| failed.as_ref().is_none_or(|f| i < f.0))
                .collect();
            let (axes, draws, pencils) = (&self.axes, &self.draws, &sketch.pencils);
            let of = |p: usize| {
                if p < first_new {
                    &pencils[p]
                } else {
                    fresh[p - first_new].as_ref().expect("framed")
                }
            };
            let made = map(todo.clone(), PIECE, threads, &|_, i| {
                pencil((axes, draws), species, &of, i)
            });
            for (&i, (framed, pencil)) in todo.iter().zip(made) {
                let axis = &mut self.axes[i];
                (axis.base, axis.heading, axis.side) = framed;
                fresh[i - first_new] = Some(pencil);
            }
            // A lateral the coupling pass takes as a fine root is not laid.
            let todo: Vec<usize> = todo
                .into_iter()
                .filter(|&i| {
                    let scale = fresh[i - first_new].as_ref().map_or(0.0, |p| p.base_scale);
                    !self.classify(i, scale, cycle)
                })
                .collect();
            let jobs: Vec<_> = todo
                .iter()
                .zip(
                    picked(&mut self.axes, todo.iter().copied())
                        .into_iter()
                        .zip(picked(&mut fresh, todo.iter().map(|i| i - first_new))),
                )
                .collect();
            let draws = &self.draws;
            let drawn = map(jobs, PIECE, threads, &|_, (&i, (axis, pencil))| {
                let pencil = pencil.as_mut().expect("framed");
                draw((axis, pencil), &draws[i], species, (i, cycle))
                    .err()
                    .map(|e| (i, e))
            });
            if let Some(first) = drawn.into_iter().flatten().next() {
                failed = Some(first);
            }
        }
        if let Some((_, error)) = failed {
            return Err(error);
        }
        // The cycle's leaves, in the order laying in turn gathered them.
        let mut leaves = std::mem::take(&mut self.leaves);
        leaves.clear();
        let laid = grew
            .iter()
            .zip(from)
            .map(|(&i, from)| (i, from, sketch.pencils[i].laid));
        let made = fresh
            .iter()
            .enumerate()
            .map(|(k, p)| (first_new + k, 0, p.as_ref().expect("drawn").laid));
        let mut owners = self
            .oracle
            .as_mut()
            .map(|o| std::mem::take(&mut o.leaf_axes));
        if let Some(o) = owners.as_mut() {
            o.clear();
        }
        for (i, from, to) in laid.chain(made) {
            let axis = &self.axes[i];
            let area = species.states[axis.pa].leaf_area;
            for p in &axis.phytomers[from..to] {
                leaves.push((p.tip, p.scale * p.size * area));
            }
            if let Some(o) = owners.as_mut() {
                o.extend(std::iter::repeat_n(i, to - from));
            }
        }
        if let (Some(oracle), Some(o)) = (self.oracle.as_mut(), owners) {
            oracle.leaf_axes = o;
        }
        sketch
            .pencils
            .extend(fresh.into_iter().map(|p| p.expect("drawn")));
        self.leaves = leaves;
        self.sketch = Some(sketch);
        Ok(())
    }
}

/// A new axis's frame and pencil, sized as `geometry::scale` sizes it
/// from what its draws have made of it so far, from its parent's pencil
/// (`of`) and wood alone.
fn pencil<'p>(
    (axes, draws): (&[Axis], &[Draws]),
    species: &Species,
    of: &(dyn Fn(usize) -> &'p Pencil + Sync),
    i: usize,
) -> (Framed, Pencil) {
    let ends = |p: usize| of(p).layer.ends();
    let framed = frame(axes, species, i, &ends);
    let axis = &axes[i];
    // A relay carries on the pulls of the axis it replaces, as the final
    // lay has it, at that axis's age and reach so far (fn-206).
    let handed = match axis.origin {
        Origin::Relay { parent, .. } => {
            let p = &axes[parent];
            let state = &species.states[p.pa];
            let years = f64::from(axis.birth.saturating_sub(p.birth)) - draws[parent].sleep;
            let layer = &of(parent).layer;
            layer.handed((bend(p, state, years), layer.run()), axis.blend)
        }
        _ => Vec::new(),
    };
    let made = draws[i].birth[0] * draws[i].birth[1];
    let curve = match axis.origin {
        Origin::Continuation { parent } | Origin::Relay { parent, .. } => of(parent).layer.curve(),
        _ => Vec3::default(),
    };
    let (inherited, vigour, trunk) = match axis.origin {
        Origin::Seed => (1.0, made, true),
        Origin::Lateral { parent, node, .. } => (axes[parent].phytomers[node].scale, made, false),
        Origin::Continuation { parent } | Origin::Relay { parent, .. } => {
            let p = of(parent);
            (p.base_scale, p.running * made, p.trunk)
        }
    };
    let share = match axis.origin {
        Origin::Lateral { .. } => dominance(species.states[axis.pa].form.dominance, axis.lineage),
        _ => 1.0,
    };
    let pencil = Pencil {
        layer: Layer::bent(framed, curve).inheriting(handed),
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
    };
    (framed, pencil)
}

/// Lays axis `i`'s units not yet laid, as `presence::assign` will size
/// them, at `cycle`.
fn draw(
    (axis, pencil): (&mut Axis, &mut Pencil),
    draws: &Draws,
    species: &Species,
    (i, cycle): (usize, u32),
) -> Result<()> {
    let state = &species.states[axis.pa];
    let years = f64::from(cycle.saturating_sub(axis.birth)) - draws.sleep;
    let bent = bend(axis, state, years);
    while pencil.units < draws.units.len() {
        let k = pencil.units;
        let drawn = draws.units[k];
        // The share of its cycle the unit grew (`schedule.rs`).
        let grown = drawn.share;
        let present = pencil.running * drawn.survive;
        let [size, _] = draws.shade(k);
        let birth = axis.birth;
        let unit = |p: &crate::structure::Phytomer| (p.cycle - birth - 1) as usize == k;
        let end = pencil.laid
            + axis.phytomers[pencil.laid..]
                .iter()
                .take_while(|p| unit(p))
                .count();
        for j in pencil.laid..end {
            let p = &mut axis.phytomers[j];
            // Unsized: what it bears inherits its scale without its size.
            p.scale = pencil.base_scale * present * draws.nodes[j] * grown;
            if j == 0 {
                axis.first = present * draws.nodes[j];
            }
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
            pencil.wood += state.internode * p.scale;
            p.scale = bare;
        }
        pencil.laid = end;
        pencil.running = present * drawn.persist;
        pencil.units += 1;
    }
    axis.rank = pencil.rank;
    Ok(())
}
