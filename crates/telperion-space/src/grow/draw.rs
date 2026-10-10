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

/// A cycle's rough layout in rounds (fn-211 task 10): the axes that grew,
/// then the new ones a generation at a time, each round framed here and
/// drawn by whichever executor lays it (`draw_round`, or a device).
pub(super) struct Sketching {
    sketch: super::sketch::Sketch,
    cycle: u32,
    pub(super) first_new: usize,
    grew: Vec<usize>,
    from: Vec<usize>,
    generations: std::vec::IntoIter<Vec<usize>>,
    fresh: Vec<Option<Pencil>>,
    failed: Option<(usize, Error)>,
    round: usize,
}

impl Sketching {
    /// The axes the first round draws: those that grew, by index.
    pub(super) fn grew(&self) -> Vec<usize> {
        let mut grew = self.grew.clone();
        grew.sort_unstable();
        grew
    }

    /// Each axis `todo` names, with its pencil, in `todo`'s order
    /// (increasing).
    pub(super) fn pencils<'s>(&'s mut self, todo: &[usize]) -> Vec<&'s mut Pencil> {
        let first_new = self.first_new;
        match todo.first() {
            Some(&i) if i >= first_new => {
                let at = todo.iter().map(|i| i - first_new);
                picked(&mut self.fresh, at)
                    .into_iter()
                    .map(|p| p.as_mut().expect("framed"))
                    .collect()
            }
            _ => picked(&mut self.sketch.pencils, todo.iter().copied()),
        }
    }

    /// A round's failures (axis, error) taken: the first round's, in the
    /// order the axes grew, ends the cycle; a later one's first, in index
    /// order, stops every axis after it.
    pub(super) fn failed(&mut self, mut failures: Vec<(usize, Error)>) -> Result<()> {
        if self.round == 0 {
            let rank = |i: usize| self.grew.iter().position(|&g| g == i);
            if let Some(k) = (0..failures.len()).min_by_key(|&k| rank(failures[k].0)) {
                return Err(failures.swap_remove(k).1);
            }
            return Ok(());
        }
        if let Some(first) = failures.into_iter().min_by_key(|f| f.0) {
            self.failed = Some(first);
        }
        Ok(())
    }
}

impl Grower<'_> {
    /// Lays the units the `advanced` axes grew in `cycle`, then frames and
    /// lays the axes made in it: parents before children, so every frame
    /// reads wood already laid.
    pub(super) fn sketch(&mut self, cycle: u32, advanced: &[usize]) -> Result<()> {
        let Some(mut s) = self.sketch_begin(cycle, advanced) else {
            return Ok(());
        };
        let mut todo = s.grew();
        loop {
            let failures = self.draw_round(&mut s, &todo);
            s.failed(failures)?;
            match self.sketch_round(&mut s) {
                Some(next) => todo = next,
                None => return self.sketch_end(s),
            }
        }
    }

    /// The cycle's rough layout begun, its sketch held aside: none in a
    /// fine subtree grown alone, whose units are tallied, never laid.
    pub(super) fn sketch_begin(&mut self, cycle: u32, advanced: &[usize]) -> Option<Sketching> {
        let mut sketch = self.sketch.take().expect("sketching");
        if self.fine.is_some() {
            self.tally_units(&mut sketch, advanced);
            self.sketch = Some(sketch);
            return None;
        }
        let first_new = sketch.pencils.len();
        let grew: Vec<usize> = advanced
            .iter()
            .copied()
            .filter(|&i| i < first_new)
            .collect();
        let from: Vec<usize> = grew.iter().map(|&i| sketch.pencils[i].laid).collect();
        Some(Sketching {
            sketch,
            cycle,
            first_new,
            grew,
            from,
            generations: Vec::new().into_iter(),
            fresh: Vec::new(),
            failed: None,
            round: 0,
        })
    }

    /// The CPU's round: each axis in `todo` drawn from its own walk, on
    /// every core; the axes that could not be.
    pub(super) fn draw_round(&mut self, s: &mut Sketching, todo: &[usize]) -> Vec<(usize, Error)> {
        let (species, threads, cycle) = (self.species, self.threads, s.cycle);
        let pencils = s.pencils(todo);
        let jobs: Vec<_> = todo
            .iter()
            .zip(
                picked(&mut self.axes, todo.iter().copied())
                    .into_iter()
                    .zip(pencils),
            )
            .collect();
        let draws = &self.draws;
        let failed = map(jobs, PIECE, threads, &|_, (&i, (axis, pencil))| {
            draw((axis, pencil), &draws[i], species, (i, cycle))
                .err()
                .map(|e| (i, e))
        });
        failed.into_iter().flatten().collect()
    }

    /// The next round's axes framed: after the first, the generations of
    /// the axes made in the cycle are found and each relay set at its
    /// node; then the next generation, as far as no axis before it failed,
    /// each framed from its parent alone. None once every one is drawn.
    pub(super) fn sketch_round(&mut self, s: &mut Sketching) -> Option<Vec<usize>> {
        let (species, threads, first_new) = (self.species, self.threads, s.first_new);
        if s.round == 0 {
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
            // A relay stands at its node, which its parent's last unit,
            // grown whole by now, decides (`presence::assign`): read off the
            // parent's phytomers and draws, which laying leaves alone.
            for i in first_new..self.axes.len() {
                if let Origin::Relay { parent, .. } = self.axes[i].origin {
                    let p = &self.axes[parent];
                    let share = species.states[p.pa].relay_at;
                    let node = relay_node(p, &self.draws[parent], share);
                    self.axes[i].origin = Origin::Relay { parent, node };
                }
            }
            s.fresh = vec![None; generation.len()];
            s.generations = generations.into_iter();
        }
        s.round += 1;
        let todo = s.generations.next()?;
        let todo: Vec<usize> = todo
            .into_iter()
            .filter(|&i| s.failed.as_ref().is_none_or(|f| i < f.0))
            .collect();
        let (axes, draws, pencils, fresh) = (&self.axes, &self.draws, &s.sketch.pencils, &s.fresh);
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
            s.fresh[i - first_new] = Some(pencil);
        }
        // A lateral the coupling pass takes as a fine root is not laid.
        let cycle = s.cycle;
        let todo = todo
            .into_iter()
            .filter(|&i| {
                let scale = s.fresh[i - first_new]
                    .as_ref()
                    .map_or(0.0, |p| p.base_scale);
                !self.classify(i, scale, cycle)
            })
            .collect();
        Some(todo)
    }

    /// The cycle's rough layout ended: its first failure, or its leaves
    /// gathered in the order laying in turn gave and its pencils kept.
    pub(super) fn sketch_end(&mut self, s: Sketching) -> Result<()> {
        let Sketching {
            mut sketch,
            first_new,
            grew,
            from,
            fresh,
            failed,
            ..
        } = s;
        if let Some((_, error)) = failed {
            return Err(error);
        }
        let species = self.species;
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
                leaves.push((p.tip, crate::light::leaf_area(p.scale, p.size, area)));
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
    let handed = match axis.carried_on() {
        Some((parent, b)) => {
            let p = &axes[parent];
            let state = &species.states[p.pa];
            // The parent's years when the relay was made: each axis's time
            // from its birth less the share of that cycle it slept, as
            // everywhere else, so a relay made at the end of a cycle and
            // one made at the start of the next meet (fn-199).
            let made = f64::from(axis.birth) + draws[i].sleep;
            let years = (made - f64::from(p.birth) - draws[parent].sleep).max(0.0);
            let layer = &of(parent).layer;
            layer.handed((bend(p, state, years), layer.run()), b)
        }
        None => Vec::new(),
    };
    // Every factor an f32, in this order.
    let made = draws[i].birth[0] as f32 * draws[i].birth[1] as f32;
    let curve = match axis.origin {
        Origin::Continuation { parent } | Origin::Relay { parent, .. } => of(parent).layer.curve(),
        _ => Vec3::default(),
    };
    let (inherited, vigour, trunk) = match axis.origin {
        Origin::Seed => (1.0, made, true),
        Origin::Lateral { parent, node, .. } => {
            (axes[parent].phytomers[node].scale as f32, made, false)
        }
        Origin::Continuation { parent } | Origin::Relay { parent, .. } => {
            let p = of(parent);
            (p.base_scale as f32, p.running as f32 * made, p.trunk)
        }
    };
    let share = match axis.origin {
        Origin::Lateral { .. } => dominance(species.states[axis.pa].form.dominance, axis.lineage),
        _ => 1.0,
    };
    let spread = species.states[axis.pa].form.tropism_spread;
    let turning = crate::lineage::tropism(axis.lineage, spread);
    let pencil = Pencil {
        layer: Layer::bent(framed, curve, turning).inheriting(handed),
        base_scale: f64::from(inherited * vigour * share as f32),
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
    let bent = bent(axis, draws, species, cycle);
    draw_bent((axis, pencil), draws, species, (i, bent))
}

/// An axis's pull towards the vertical at its age in `cycle` (`bend`).
pub(super) fn bent(axis: &Axis, draws: &Draws, species: &Species, cycle: u32) -> f64 {
    let years = f64::from(cycle.saturating_sub(axis.birth)) - draws.sleep;
    bend(axis, &species.states[axis.pa], years)
}

/// `draw` with its pull towards the vertical `bent` given.
pub(super) fn draw_bent(
    (axis, pencil): (&mut Axis, &mut Pencil),
    draws: &Draws,
    species: &Species,
    (i, bent): (usize, f64),
) -> Result<()> {
    let state = &species.states[axis.pa];
    while pencil.units < draws.units.len() {
        let k = pencil.units;
        let drawn = draws.units[k];
        // The share of a node's length its nodes grew at (`Unit::size`).
        let grown = drawn.size as f32;
        let present = pencil.running as f32 * drawn.survive as f32;
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
            let node = draws.nodes[j] as f32;
            p.scale = f64::from(pencil.base_scale as f32 * present * node * grown);
            // How much of a node it is (`stand`), as its scale.
            p.grown = f64::from(present * node * grown);
            p.size = size;
            p.rank = pencil.rank;
            pencil.rank = f64::from(pencil.rank as f32 + node * grown);
        }
        let total = pencil.layer.run() as f32
            + axis.phytomers[pencil.laid..end]
                .iter()
                .map(|p| p.scale as f32 * p.size as f32)
                .sum::<f32>();
        let total = f64::from(total);
        for j in pencil.laid..end {
            let p = &mut axis.phytomers[j];
            // Laid at its size, then left unsized for what it bears.
            let bare = p.scale;
            p.scale = f64::from(bare as f32 * p.size as f32);
            pencil
                .layer
                .step(p, state, (bent, total), None, pencil.trunk)
                .map_err(|s| s.error(i, axis.pa, axis.birth, axis.base.z))?;
            pencil.wood = f64::from(pencil.wood as f32 + state.internode as f32 * p.scale as f32);
            p.scale = bare;
        }
        pencil.laid = end;
        pencil.running = f64::from(present * drawn.persist as f32);
        pencil.units += 1;
    }
    axis.rank = pencil.rank;
    Ok(())
}
