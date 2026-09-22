//! Halving a bundle that is better but breaks something, until the breaking
//! dials are on one side of the cut.
//!
//! Each round of the loop draws both halves at the parent's strength, shows
//! them on one sheet beside the current tree, keeps the largest half the
//! reviewer judged clean, and follows the half that is still better and still
//! breaks something. It stops at a single dial, at the split budget, or when
//! there is no reservation left to draw with.
use super::{
    overlay_of, part,
    round::{evaluate, note_failure, note_unshown, read_back, Variant},
    split,
    track::Track,
};
use crate::tuning::{
    engine::{Run, Services},
    progress, sheet,
};

type Save<'a> = &'a mut dyn FnMut(&Run) -> Result<(), String>;

/// The largest clean sub-bundle the split found, by trial key.
#[allow(clippy::too_many_arguments)]
pub(super) fn isolate(
    state: &mut Run,
    services: &mut dyn Services,
    save: Save<'_>,
    old: usize,
    base: &str,
    track: &Track,
    ledger: Option<String>,
    variants: &mut Vec<Variant>,
    start: (String, f64),
) -> Result<Option<String>, String> {
    let budget = services.max_split_reviews();
    let mut spent = 0;
    let mut best: Option<(String, usize)> = None;
    let mut parent = start;
    while spent < budget {
        let Some(index) = state.trials.iter().position(|t| t.key == parent.0) else {
            break;
        };
        let Some(bundle) = state.trials[index].bundle.clone() else {
            break;
        };
        // One dial is already isolated: there is nothing left to cut.
        if bundle.moves.len() < 2 {
            break;
        }
        let (a, b) = split(&bundle.moves, &state.dials);
        let mut halves = vec![];
        let mut stopped = false;
        for (side, moves) in [("a", a), ("b", b)] {
            if moves.is_empty() {
                continue;
            }
            let overlay = overlay_of(&moves, &state.dials);
            let drawn = part(&moves, parent.1, base, &track.name);
            let label = format!("bundle@{} half {side}", parent.1);
            match evaluate(
                state,
                services,
                save,
                &drawn,
                &overlay,
                old,
                Some(bundle.id.clone()),
                &label,
                ledger.clone(),
            ) {
                Ok(trial) => {
                    if state.trials[trial].feasible {
                        variants.push(Variant { trial, overlay });
                        halves.push(trial);
                    }
                }
                Err(reason) => {
                    state
                        .routes
                        .push(format!("split stopped before {label}: {reason}"));
                    stopped = true;
                    break;
                }
            }
        }
        if halves.is_empty() {
            break;
        }
        let priorities = progress::tuning_priorities(state);
        // A track judged at one view is split at that view too: a material
        // half on the whole-tree still would read inert.
        let look =
            services.sheet_request(state, old, &halves, &priorities, track.view.as_deref())?;
        note_unshown(state, &look);
        let Some(plan) = &look.plan else {
            break;
        };
        let verdict = match sheet::review(state, services, save, plan) {
            Ok(verdict) => verdict,
            Err(reason) => {
                note_failure(state, &look);
                save(state)?;
                return Err(reason);
            }
        };
        spent += 1;
        let shown = read_back(state, &look, &verdict, old);
        save(state)?;
        if let Some(key) = sheet::adopt(&verdict, &shown) {
            let dials = state
                .trials
                .iter()
                .find(|t| t.key == key)
                .and_then(|t| t.bundle.as_ref())
                .map_or(0, |b| b.moves.len());
            if best.as_ref().is_none_or(|(_, kept)| dials > *kept) {
                best = Some((key, dials));
            }
        }
        if stopped {
            break;
        }
        match sheet::to_split(&verdict, &shown) {
            Some(next) => parent = next,
            None => break,
        }
    }
    Ok(best.map(|(key, _)| key))
}
