//! Phases 2 to 4 of the settle (fn-209 F1 task 6), the executor's steps
//! awaited (fn-212 task 10, host decision 1): natively the driver waits
//! on a step where its own cores work beside it, in a browser it awaits.
use super::expansion::{RootSeed, Roots, Stages};
use super::landing::Landing;
#[cfg(not(target_family = "wasm"))]
use super::likely::before_laid;
use super::Clock;
use crate::closed_form::boundary::Rows;
use crate::error::Result;
use crate::geometry::place;
use crate::girth::{thicken_given, Girth};
use crate::sag::{any, levers_given, Load};
use crate::species::Species;
use crate::structure::Structure;

/// Phases 2 to 4: the live roots scaled and thickened, the structure
/// above the cut thickened and laid on them, the roots laid on its
/// frames, the structure loaded and bent by them, and the roots laid
/// again on it; each root that can reach the ground laid on the CPU. The
/// structure thickens by `girth`, what the exchange made of its tree as
/// grown (none where no PA keeps it).
#[allow(clippy::type_complexity)]
pub(super) async fn lay(
    (species, tree, roots, girth): (&Species, &mut Structure, &mut dyn Roots, &Girth),
    ((seeds, live, bearers), scales, by_axis): (
        (&[RootSeed], &[usize], &[usize]),
        &[(f64, f64)],
        &[Option<usize>],
    ),
    (landing, clock, stages): (&mut Landing<'_>, &mut Clock, &mut Stages),
    (threads, rows): (usize, Option<&Rows>),
) -> Result<Vec<f64>> {
    // The order the roots are grown ahead in, read while the executor
    // scales and thickens its roots.
    #[cfg(not(target_family = "wasm"))]
    let radii = std::thread::scope(|scope| {
        if let (Some(spare), Some(rows), true) = (landing.spare, rows, any(species)) {
            let tree = &*tree;
            scope.spawn(move || {
                let order = before_laid((species, tree, rows), (seeds, live, bearers, scales));
                spare.plan(order);
            });
        }
        clock.time(|| super::step::wait(roots.girth(live, scales)))
    })?;
    // No spare cores in a browser, so no order to read.
    #[cfg(target_family = "wasm")]
    let radii = {
        let _ = rows;
        clock.timed(roots.girth(live, scales)).await?
    };
    stages.mark("phase 2 (executor)");
    // The roots grown ahead wait while the structure's passes run.
    let spare = landing.spare;
    let hold = |held: bool| spare.inspect(|s| s.hold(held));
    hold(true);
    let radius = |i: usize| by_axis[i].map(|j: usize| radii[j]);
    thicken_given(tree, species, girth, &radius, threads);
    place(tree, species, None, threads)?;
    landing.frame(tree, (seeds, live, bearers), (scales, &radii));
    stages.mark("thicken and place");
    // Lay on each bearer's frame, and load the tree by each root's wood;
    // a root that can reach the ground is laid on the CPU (host decision
    // 3, `landing.rs`).
    hold(false);
    let mut loads = clock.timed(roots.lay(tree, live, bearers)).await?;
    stages.mark("phase 3 (executor)");
    hold(true);
    let at_roots = (seeds, live, bearers);
    landing.lay((seeds, live), (scales, &radii), &mut loads)?;
    stages.mark("landing, unbent");
    if any(species) {
        let load = |i: usize| by_axis[i].map(|j: usize| Load::of(loads[j].0, loads[j].1));
        let (levers, _) = levers_given(tree, species, &load, threads);
        place(tree, species, Some(&levers), threads)?;
        stages.mark("levers and place");
        landing.spare.inspect(|s| s.stop());
        // The device's bent lay set going while the CPU lands its roots.
        let tree = &*tree;
        let bending = roots.bend(tree, live, bearers);
        #[cfg(not(target_family = "wasm"))]
        let (landed, bent) = crate::cores::beside(
            || landing.bend(tree, at_roots, (scales, &radii)),
            || {
                let bent = clock.time(|| super::step::wait(bending));
                stages.mark("phase 4 (executor)");
                bent
            },
        );
        #[cfg(target_family = "wasm")]
        let (landed, bent) = {
            let bent = clock.timed(bending).await;
            stages.mark("phase 4 (executor)");
            (landing.bend(tree, at_roots, (scales, &radii)), bent)
        };
        bent?;
        landed?;
        stages.mark("landing, bent");
    }
    Ok(radii)
}
