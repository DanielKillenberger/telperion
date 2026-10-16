//! Phases 2 to 4 of the settle (fn-209 F1 task 6), the executor's steps
//! awaited (fn-212 task 10, host decision 1).
use super::expansion::{Roots, Stages};
use super::Clock;
use crate::error::Result;
use crate::geometry::{place, place_bearers};
use crate::girth::{thicken_given, Girth};
use crate::sag::{any, levers_given, Load};
use crate::species::Species;
use crate::structure::Structure;

/// Phases 2 to 4: the live roots scaled and thickened, the structure
/// above the cut thickened and laid on them, the roots laid on its
/// frames, the structure loaded and bent by them, and the roots laid
/// again on it. The structure thickens by `girth`, what the exchange
/// made of its tree as grown (none where no PA keeps it).
#[allow(clippy::type_complexity)]
pub(super) async fn lay(
    (species, tree, roots, girth): (&Species, &mut Structure, &mut dyn Roots, &Girth),
    ((live, bearers), scales, by_axis): ((&[usize], &[usize]), &[(f64, f64)], &[Option<usize>]),
    (clock, stages): (&mut Clock, &mut Stages),
    threads: usize,
) -> Result<Vec<f64>> {
    let radii = clock.timed(roots.girth(live, scales)).await?;
    stages.mark("phase 2 (executor)");
    let radius = |i: usize| by_axis[i].map(|j: usize| radii[j]);
    thicken_given(tree, species, girth, &radius, threads);
    let bends = any(species);
    if bends {
        place_bearers(tree, species, threads)?;
    } else {
        place(tree, species, None, threads)?;
    }
    stages.mark("thicken and place");
    // Lay on each bearer's frame, and load the tree by each root's wood.
    let loads = clock.timed(roots.lay(tree, live, bearers)).await?;
    stages.mark("phase 3 (executor)");
    if bends {
        let load = |i: usize| by_axis[i].map(|j: usize| Load::of(loads[j].0, loads[j].1));
        let (levers, _) = levers_given(tree, species, &load, threads);
        place(tree, species, Some(&levers), threads)?;
        stages.mark("levers and place");
        clock.timed(roots.bend(tree, live, bearers)).await?;
        stages.mark("phase 4 (executor)");
    }
    Ok(radii)
}
