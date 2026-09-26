//! The look that follows an adoption can take it back.
//!
//! The contact sheet judges one view against two priorities. On the beech run
//! of 2026-09-21 it graded three half-step bundles "clear" on crown shape and
//! the owner called the third "garbage": thick outer branches, chaotic
//! branching and a long bare leader, all of which the closing all-view review
//! had reported and nothing in the loop acted on. So the closing review now
//! decides whether the move stands: a required cell or a reference-first trait
//! that went backwards takes it back with no model call at all, and otherwise
//! one uncalibrated question asks whether the review names a defect the
//! previous one did not, outside what the move was aiming at.
mod judge;
mod restore;
pub use judge::{questions, Judged, QUESTION, UNCALIBRATED, VERSION};
pub use restore::{restore_point, worsened, Restore, Veto, Worsened};

use super::engine::{Run, Services};

/// Whether the adoption stands. A vetoed one is put back exactly as it was,
/// without a further visual pass; what was already spent stays spent.
pub(in crate::tuning) fn settle(
    state: &mut Run,
    services: &mut dyn Services,
    save: &mut dyn FnMut(&Run) -> Result<(), String>,
    restore: Restore,
    trial: usize,
) -> Result<bool, String> {
    let (Some(before), Some(after)) = (restore.visual.clone(), state.visual.clone()) else {
        return Ok(true);
    };
    let (before, after) = (&before, &after);
    let worsened = restore::worsened(
        before,
        after,
        &state.required_cells(),
        &services.unexpressed(),
    );
    state.routes.extend(worsened.notes);
    let mut reasons = worsened.reasons;
    let mut ledger = None;
    // A structured regression is read off the two assessments; nobody is paid
    // to confirm what the cells already say.
    if reasons.is_empty() {
        let asked = judge::ask(state, services, save, before, after, trial)?;
        ledger = asked.ledger.clone();
        if asked.vetoes() {
            reasons.push(judge::reason(&asked, after));
        } else {
            state.routes.push(judge::kept_note(&asked));
        }
    }
    if reasons.is_empty() {
        save(state)?;
        return Ok(true);
    }
    state
        .routes
        .push(format!("adoption rolled back: {}", reasons.join("; ")));
    state.trials[trial].vetoed = Some(Veto { reasons, ledger });
    restore.apply(state);
    save(state)?;
    Ok(false)
}
