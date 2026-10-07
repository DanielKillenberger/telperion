//! The structure above the cut with each fine root standing as its
//! expected occupancy (fn-214's probe): no fine root is grown. The
//! structure above the cut grows and settles through the four phases of
//! realized mechanics (`expansion.rs`) exactly as `grow_fine` settles it,
//! but each root reports up its closed form's expectations, read from its
//! committed boundary (`closed_form::boundary`, fn-209 option 4) at its
//! base scale, in place of what it would realize:
//!
//! 1. its living, from its extinction curve: the expected cycles it lives
//!    past its birth, and the chance it lives at the tree's age;
//! 2. its base radius, its boundary's E[r | kept] (`bounded/girth.rs`);
//! 3. its load, its boundary's mass at its base and moment along its
//!    heading (`bounded/work.rs`);
//! 4. nothing: it holds no wood to bend.
//!
//! What it stands for is then a record of the root, where its bud stands
//! on its bearer as laid under sag: its heading, base radius, the centre
//! of its expected wood along its heading and its expected kept
//! phytomers. A coarse tree is not the tree: its fine roots are their
//! expectations, never their draws.
use super::expansion::{settle_with, Lived, Loaded, RootSeed, Roots};
use super::Plan;
use crate::bounded::girth::committed_radius;
use crate::closed_form::boundary::Rows;
use crate::error::{refuse, Result};
use crate::fixed::presence::{fade, q, real};
use crate::geometry::{bud_frame, dominance};
use crate::grow::alone::rules;
use crate::grow::{keeps, Request};
use crate::species::Species;
use crate::structure::{Origin, Structure, Vec3};

/// A fine root as a coarse tree holds it: its stand-in's axis in the
/// structure, where its bud stands and heads, its base radius, how far
/// along its heading its expected wood's centre stands (metres) and its
/// expected kept phytomers.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Stand {
    pub axis: usize,
    pub base: Vec3,
    pub heading: Vec3,
    pub radius: f64,
    pub centre: f64,
    pub phytomers: f64,
}

/// The structure above a cut, its fine roots' stand-ins unexpanded, and
/// the live roots as their expectations.
pub struct Coarse {
    pub structure: Structure,
    pub stands: Vec<Stand>,
}

/// Grows `species` above the cut `cut` (expected kept phytomers) and
/// settles it on its fine roots' expectations, growing none of them.
/// Refuses where `grow_fine` does: a species any bud of which reads light,
/// or whose girth reads the tree as grown, has no structure above a cut
/// apart from what it bears (fn-211).
pub fn grow_coarse(species: &Species, request: Request, cut: f64) -> Result<Coarse> {
    if keeps(species) {
        return refuse(
            "cut",
            "a coarse tree needs a structure above the cut that grows apart from its fine roots",
        );
    }
    if !(cut.is_finite() && cut >= 0.0) {
        return refuse("cut", "a cut is a count of expected kept phytomers");
    }
    let rules = rules(species, request.age, request.budget)?;
    request.light.validate()?;
    let rows = Rows::laterals(species, request.age, crate::cores::available());
    let predictor = rows.predictor();
    let mut expected = Expected {
        species,
        rows: &rows,
        age: request.age,
        roots: Vec::new(),
    };
    let plan = Plan {
        cut,
        ..Plan::default()
    };
    let context = (request, &predictor, &rules);
    let (settled, _) = settle_with(species, context, plan, &mut expected, false)?;
    let tree = settled.structure;
    let stands = settled
        .live
        .iter()
        .map(|&k| {
            let axis = settled.at[k].expect("live");
            let a = &tree.axes[axis];
            let (base, heading) = bud_frame(&tree.axes, species, (a.origin, a.pa, a.lineage));
            let root = &expected.roots[k];
            Stand {
                axis,
                base,
                heading,
                radius: root.radius,
                centre: root.centre,
                phytomers: root.count,
            }
        })
        .filter(|s: &Stand| s.phytomers > 0.0)
        .collect();
    Ok(Coarse {
        structure: tree,
        stands,
    })
}

/// A fine root's expectations as the phases read them.
struct Root {
    pa: usize,
    birth: u32,
    lineage: u64,
    origin: Origin,
    vigour: f64,
    living: Lived,
    scale: f64,
    radius: f64,
    centre: f64,
    count: f64,
}

/// The executor that grows nothing: every root its boundary.
struct Expected<'a> {
    species: &'a Species,
    rows: &'a Rows,
    age: u32,
    roots: Vec<Root>,
}

impl Expected<'_> {
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

    /// Its base scale from its bearer's base scale and its node's drawn
    /// scale: its vigour as made, faded as its living fades it, and its
    /// dominance share (`geometry::scale`).
    fn scale(&self, root: &Root, (base, drawn): (f64, f64)) -> f64 {
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
        let share = dominance(state.form.dominance, root.lineage);
        base * drawn * root.vigour * faded * share
    }
}

impl Roots for Expected<'_> {
    fn grow(&mut self, seeds: &[RootSeed]) -> Result<(Vec<Lived>, u64)> {
        self.roots = seeds
            .iter()
            .map(|seed| {
                let axis = &seed.handed.bud.axis;
                Root {
                    pa: axis.pa,
                    birth: axis.birth,
                    lineage: axis.lineage,
                    origin: axis.origin,
                    vigour: seed.made.0,
                    living: self.living(axis.pa, axis.birth),
                    scale: 0.0,
                    radius: 0.0,
                    centre: 0.0,
                    count: self.rows.lateral(axis.pa, axis.birth).count,
                }
            })
            .collect();
        Ok((self.roots.iter().map(|r| r.living).collect(), 0))
    }

    fn girth(&mut self, live: &[usize], scales: &[(f64, f64)]) -> Vec<f64> {
        live.iter()
            .zip(scales)
            .map(|(&k, &scales)| {
                let scale = self.scale(&self.roots[k], scales);
                let root = &self.roots[k];
                let row = self.rows.lateral(root.pa, root.birth);
                // E[r | kept] weighed by the chance it is kept, in its
                // pipe's exponent, so its section is E[S; kept].
                let given = committed_radius(self.species, row, scale, root.pa);
                let radius = given * row.kept.clamp(0.0, 1.0).powf(1.0 / row.e);
                let root = &mut self.roots[k];
                (root.scale, root.radius) = (scale, radius);
                radius
            })
            .collect()
    }

    fn lay(&mut self, tree: &Structure, live: &[usize], bearers: &[usize]) -> Result<Vec<Loaded>> {
        Ok(live
            .iter()
            .zip(bearers)
            .map(|(&k, &bearer)| {
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
                let (base, heading) = bud_frame(&tree.axes, self.species, bud);
                let row = self.rows.lateral(root.pa, root.birth);
                let mass = row.mass * root.scale.powi(3);
                let along = row.moment * root.scale.powi(4);
                root.centre = if mass > 0.0 { along / mass } else { 0.0 };
                (mass, base * mass + heading * along)
            })
            .collect())
    }

    fn bend(&mut self, _: &Structure, _: &[usize], _: &[usize]) -> Result<()> {
        Ok(())
    }
}
