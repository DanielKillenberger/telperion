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
//! A species whose buds read light (the oak, task 2, host option 1)
//! grows its structure above the cut in the stepped executor instead,
//! each fine root standing there as its aggregate through every cycle's
//! balance, allotment and light (`grow/stepped/stands.rs`); it settles on
//! its roots' rows as above, with the exchange where it keeps its tree as
//! grown: each root's leaf partials from its aggregate's pipes and last
//! light, and its base radius as grown from its row's sections before its
//! own fade.
//!
//! What it stands for is then a record of the root, where its bud stands
//! on its bearer as laid under sag: its heading, base radius, the centre
//! of its expected wood along its heading and its expected kept
//! phytomers. A coarse tree is not the tree: its fine roots are their
//! expectations, never their draws.
mod expected;

use super::expansion::{settle_grown, settle_with, Above, Settled, Stages};
use super::step::now;
use super::{Clock, Plan};
use crate::closed_form::boundary::Rows;
use crate::error::{refuse, Result};
use crate::geometry::bud_frame;
use crate::grow::alone::rules;
use crate::grow::{keeps, Request};
use crate::species::Species;
use crate::structure::{Structure, Vec3};
use expected::Expected;
use std::sync::Arc;

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
/// Refuses a species that keeps its tree as grown but whose buds read no
/// light: nothing crosses the cut each cycle for a root to stand in, and
/// its roots' leaf partials have no light to be read under.
pub fn grow_coarse(species: &Species, request: Request, cut: f64) -> Result<Coarse> {
    if !(cut.is_finite() && cut >= 0.0) {
        return refuse("cut", "a cut is a count of expected kept phytomers");
    }
    if keeps(species) && !species.reads_light() {
        return refuse(
            "cut",
            "a coarse tree that keeps its tree as grown needs buds that read light",
        );
    }
    request.light.validate()?;
    let rows = Rows::laterals(species, request.age, crate::cores::available());
    if species.reads_light() {
        return lit(species, request, Arc::new(rows), cut);
    }
    let rules = rules(species, request.age, request.budget)?;
    let predictor = rows.predictor();
    let mut expected = Expected::new(species, &rows, request.age);
    let plan = Plan {
        cut,
        ..Plan::default()
    };
    let context = (request, &predictor, &rules);
    let (settled, _) = now(settle_with(species, context, plan, &mut expected, false))?;
    Ok(coarse(species, settled, &expected))
}

/// A species whose buds read light grown with its roots standing in the
/// stepped executor, then settled on their rows.
fn lit(species: &Species, request: Request, rows: Arc<Rows>, cut: f64) -> Result<Coarse> {
    let standing = crate::grow::stepped::stand(species, request, rows.clone(), cut)?;
    let expected = Expected::new(species, &rows, request.age);
    let mut expected = expected.stood(&standing.roots);
    let threads = crate::cores::available();
    let plan = Plan {
        cut,
        threads,
        ..Plan::default()
    };
    let above = Above {
        grown: standing.grown,
        seats: standing.seats,
    };
    let settled = now(settle_grown(
        species,
        (request, &standing.rules, plan, threads),
        above,
        (expected.lived(), 0),
        &mut expected,
        None,
        (Clock::default(), Stages::default()),
    ))?;
    Ok(coarse(species, settled, &expected))
}

/// The settled structure and each live root as it stands.
fn coarse(species: &Species, settled: Settled, expected: &Expected) -> Coarse {
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
    Coarse {
        structure: tree,
        stands,
    }
}
