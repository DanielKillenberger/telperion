//! The executor that grows nothing (fn-214): every fine root its closed
//! form row, read at its base scale, through the four phases and, where
//! any PA keeps its tree as grown, the exchange before them (task 2).
use super::super::expansion::{Lived, Loaded, RootSeed, Roots};
use super::super::step::{ready, Step};
use crate::bounded::girth::committed_radius;
use crate::closed_form::boundary::{Row, Rows};
use crate::error::Result;
use crate::fixed::feedback::total::Total;
use crate::fixed::presence::{fade, q, real};
use crate::geometry::{bud_frame, dominance};
use crate::girth::{leaf, terms};
use crate::grow::stepped::Stood;
use crate::species::Species;
use crate::structure::{Origin, Structure};
use crate::transcendental::Transcendental;

/// A fine root's expectations as the phases read them. Where it stood
/// as an aggregate (task 2), the light it last read and its pipes grown
/// per unit base scale, unripened; light-neutral (whole light, none)
/// where it stood as its bud alone.
pub(super) struct Root {
    pub pa: usize,
    pub birth: u32,
    pub lineage: u64,
    pub origin: Origin,
    pub vigour: f64,
    pub living: Lived,
    pub scale: f64,
    pub radius: f64,
    pub centre: f64,
    pub count: f64,
    pub light: f64,
    pub pipes: f64,
}

/// Every root its row; the tree's leaf mean once the exchange gives it.
pub(super) struct Expected<'a> {
    pub species: &'a Species,
    pub rows: &'a Rows,
    pub age: u32,
    pub roots: Vec<Root>,
    pub mean: Option<f64>,
}

impl<'a> Expected<'a> {
    pub fn new(species: &'a Species, rows: &'a Rows, age: u32) -> Self {
        Self {
            species,
            rows,
            age,
            roots: Vec::new(),
            mean: None,
        }
    }

    /// The roots as they stood in the stepped executor: one shed or died
    /// out lived to that cycle and lives no more; one standing at the end
    /// lives as its row keeps it.
    pub fn stood(mut self, stood: &[Stood]) -> Self {
        self.roots = (stood.iter())
            .map(|s| {
                let a = &s.axis;
                let living = match s.died {
                    Some(d) => (q(f64::from(d.saturating_sub(a.birth))), 0),
                    None => self.living(a.pa, a.birth),
                };
                let mut root = self.root(a.pa, a.birth, a.lineage, a.origin, a.vigour);
                (root.living, root.light, root.pipes) = (living, s.light, s.agg.pipes);
                root
            })
            .collect();
        self
    }

    fn root(&self, pa: usize, birth: u32, lineage: u64, origin: Origin, vigour: f64) -> Root {
        Root {
            pa,
            birth,
            lineage,
            origin,
            vigour,
            living: self.living(pa, birth),
            scale: 0.0,
            radius: 0.0,
            centre: 0.0,
            count: self.rows.lateral(pa, birth).count,
            light: 1.0,
            pipes: 0.0,
        }
    }

    /// Each root's living, by root.
    pub fn lived(&self) -> Vec<Lived> {
        self.roots.iter().map(|r| r.living).collect()
    }

    /// A lateral's living as its bearer's fade reads it (`shed.rs`,
    /// `Living`): a root its boundary keeps with any chance stands to the
    /// tree's age, its presence that chance; one it never keeps has none.
    /// What it stands for is weighed by that chance (its boundary's
    /// partial moments, E[ · ; kept]), so it is never shed by a draw.
    fn living(&self, pa: usize, birth: u32) -> Lived {
        let kept = self.rows.lateral(pa, birth).kept;
        if kept > 0.0 {
            (q(f64::from(self.age - birth)), q(kept.min(1.0)))
        } else {
            (0, 0)
        }
    }

    /// Its base scale as grown, from its bearer's base scale and its
    /// node's drawn scale: its vigour as made and its dominance share
    /// (`geometry::scale`).
    fn grown(&self, root: &Root, (base, drawn): (f64, f64)) -> f64 {
        let state = &self.species.states[root.pa];
        let share = dominance(state.form.dominance, root.lineage);
        base * drawn * root.vigour * share
    }

    /// Its base scale: as grown, faded as its living fades it.
    fn scale(&self, root: &Root, scales: (f64, f64)) -> f64 {
        let state = &self.species.states[root.pa];
        let faded = if state.shedding.is_finite() {
            real(fade(
                q(state.shedding + 1.0),
                root.birth,
                root.living,
                self.age,
            ))
        } else {
            1.0
        };
        self.grown(root, scales) * faded
    }

    /// Its base radius at base scale `s` from its row's section moments
    /// (`moments` of `row`): E[r | kept] weighed by the chance it is kept,
    /// in its pipe's exponent, so its section is E[S; kept]; by its leaf
    /// factor under the tree's mean, as its own pipes would be.
    fn radius(&self, root: &Root, (row, moments): (&Row, [f64; 2]), s: f64) -> f64 {
        let row_at = Row {
            s1: moments[0],
            s2: moments[1],
            ..*row
        };
        let given = committed_radius(self.species, &row_at, s, root.pa);
        let chi = self.species.states[root.pa].leaf_girth;
        let lit = leaf(root.light, chi, self.mean);
        given * row.kept.clamp(0.0, 1.0).powf_fixed(1.0 / row.e) * lit
    }
}

impl Roots for Expected<'_> {
    fn grow<'a>(&'a mut self, seeds: &'a [RootSeed]) -> Step<'a, Result<(Vec<Lived>, u64)>> {
        self.roots = (seeds.iter())
            .map(|seed| {
                let a = &seed.handed.bud.axis;
                self.root(a.pa, a.birth, a.lineage, a.origin, seed.made.0)
            })
            .collect();
        ready(Ok((self.lived(), 0)))
    }

    /// Each root's leaf partials as grown: its pipes at its base scale as
    /// grown, by the light it last read. That scale is kept for its
    /// radius as grown, until phase 2 fades it.
    fn grown_partials(&mut self, stands: &[(i64, f64)]) -> Result<Vec<[Total; 2]>> {
        let mut out = Vec::with_capacity(self.roots.len());
        for (k, &(base, drawn)) in stands.iter().enumerate() {
            let base = base as f64 / crate::fixed::ONE32 as f64;
            let s = self.grown(&self.roots[k], (base, drawn));
            let root = &mut self.roots[k];
            root.scale = s;
            let chi = self.species.states[root.pa].leaf_girth;
            let (pipe, lit) = terms(s * root.pipes, root.light, chi);
            let (mut total, mut weighted) = (Total::default(), Total::default());
            total.add(pipe);
            weighted.add(lit);
            out.push([total, weighted]);
        }
        Ok(out)
    }

    /// Each root's base radius as grown (at the scale `grown_partials`
    /// kept), from its row's sections before its own fade, under the
    /// tree's mean; the mean kept for phase 2.
    /// Its shed pipes are its row's, never kept apart.
    fn grown_radii(&mut self, mean: Option<f64>, _above: &[i64]) -> Result<Vec<f64>> {
        self.mean = mean;
        let one = |root: &Root| {
            let row = self.rows.lateral(root.pa, root.birth);
            let s = root.scale;
            self.radius(root, (row, [row.grown1, row.grown2]), s)
        };
        Ok(self.roots.iter().map(one).collect())
    }

    fn girth<'a>(
        &'a mut self,
        live: &'a [usize],
        scales: &'a [(f64, f64)],
    ) -> Step<'a, Result<Vec<f64>>> {
        let radii = (live.iter().zip(scales))
            .map(|(&k, &scales)| {
                let root = &self.roots[k];
                let scale = self.scale(root, scales);
                let row = self.rows.lateral(root.pa, root.birth);
                let radius = self.radius(root, (row, [row.s1, row.s2]), scale);
                let root = &mut self.roots[k];
                (root.scale, root.radius) = (scale, radius);
                radius
            })
            .collect();
        ready(Ok(radii))
    }

    fn lay<'a>(
        &'a mut self,
        tree: &'a Structure,
        live: &'a [usize],
        bearers: &'a [usize],
    ) -> Step<'a, Result<Vec<Loaded>>> {
        let (species, rows) = (self.species, self.rows);
        let one = |(&k, &bearer): (&usize, &usize)| {
            let root = &mut self.roots[k];
            let Origin::Lateral {
                node,
                slot,
                whorl,
                woken,
                ..
            } = root.origin
            else {
                unreachable!("a fine root is a lateral bud")
            };
            let origin = Origin::Lateral {
                parent: bearer,
                node,
                slot,
                whorl,
                woken,
            };
            let bud = (origin, root.pa, root.lineage);
            let (base, heading) = bud_frame(&tree.axes, species, bud);
            let row = rows.lateral(root.pa, root.birth);
            let mass = row.mass * root.scale.powi(3);
            let along = row.moment * root.scale.powi(4);
            root.centre = if mass > 0.0 { along / mass } else { 0.0 };
            (mass, base * mass + heading * along)
        };
        ready(Ok(live.iter().zip(bearers).map(one).collect()))
    }

    fn bend<'a>(
        &'a mut self,
        _: &'a Structure,
        _: &'a [usize],
        _: &'a [usize],
    ) -> Step<'a, Result<()>> {
        ready(Ok(()))
    }
}
