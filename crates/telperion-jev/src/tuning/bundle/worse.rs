//! What a round does when its sheet judged every variant worse.
//!
//! Halving only ever ran on better-with-breaks, so five live rounds of 33 to
//! 52 dials each came back worse and the search never learned which part of a
//! bad bundle was good (`bundle-search-run-2-2026-09-21.md`). A worse sheet is
//! now cut once, by dial sub-family, into at most four parts judged on one
//! more sheet. A part that is adoptable ends the round kept; otherwise each
//! family keeps the verdict it earned, and the next bundle from this same tree
//! leaves the families that were judged worse out.
use super::{
    families, overlay_of, part,
    round::{evaluate, note_failure, note_unshown, read_back, Variant},
    track::Track,
};
use crate::tuning::{
    engine::{Run, Services},
    look, sheet,
};

type Save<'a> = &'a mut dyn FnMut(&Run) -> Result<(), String>;

/// What the round records when a family is left out of the next bundle.
pub const EXCLUDED: &str = "family excluded after an isolated worse verdict";

/// The family an isolated part's label names, or none for a whole bundle.
pub fn part_family(label: &str) -> Option<&str> {
    label
        .strip_prefix("bundle@")?
        .split_once('/')
        .map(|(_, family)| family)
}

/// True when the reviewer put this render below the current tree, or graded it
/// worse on a priority. Either is a verdict against the part, not a stall.
fn judged_worse(outcome: &sheet::Outcome) -> bool {
    outcome.below_current
        || outcome
            .per_priority
            .values()
            .any(|m| *m == sheet::Movement::Worse)
}

/// Every family an isolated part was judged worse on, from the tree the loop
/// is standing on. A merged family excludes each name it was merged from.
pub fn excluded_families(state: &Run, base: &str) -> Vec<String> {
    let mut out: Vec<String> = vec![];
    for trial in &state.trials {
        if !state.measured_here(&trial.identity) || trial.base.as_deref() != Some(base) {
            continue;
        }
        let (Some(family), Some(outcome)) = (part_family(&trial.label), trial.sheet.as_ref())
        else {
            continue;
        };
        if !judged_worse(outcome) {
            continue;
        }
        for name in family.split('+') {
            if !out.iter().any(|known| known == name) {
                out.push(name.to_string());
            }
        }
    }
    out
}

/// The moves whose family no isolated part has already lost, each dropped
/// family noted. A bundle that keeps proposing a family the reviewer refused
/// is the same bundle again. `None` when nothing eligible is left.
pub(super) fn eligible(
    state: &mut Run,
    base: &str,
    track: &Track,
    wanted: &[(String, i8)],
) -> Option<Vec<(String, i8)>> {
    let excluded = excluded_families(state, base);
    let mut kept = vec![];
    let mut dropped: Vec<String> = vec![];
    for (id, sign) in wanted {
        let name = state
            .dials
            .iter()
            .find(|d| &d.id == id)
            .map_or_else(String::new, |d| super::family(&d.path, &state.dials));
        if excluded.iter().any(|e| e == &name) {
            if !dropped.iter().any(|d| d == &name) {
                dropped.push(name);
            }
            continue;
        }
        kept.push((id.clone(), *sign));
    }
    for family in dropped {
        let note = super::round::note(track, format!("{EXCLUDED}: {family}"));
        state.routes.push(note);
    }
    (!kept.is_empty()).then_some(kept)
}

/// One family's verdict, in the reviewer's own terms, for the run's log.
fn verdict(label: &str, outcome: &sheet::Outcome) -> String {
    let grades = outcome
        .per_priority
        .iter()
        .map(|(id, m)| format!("{id}={m:?}"))
        .collect::<Vec<_>>()
        .join(", ");
    let wrong = outcome.wrong.join("; ");
    format!("family verdict {label}: {grades}; looks wrong: {wrong}")
}

/// The smallest strength the sheet actually showed, and the trial that drew it.
fn smallest(shown: &[(String, f64)]) -> Option<(String, f64)> {
    shown
        .iter()
        .min_by(|a, b| a.1.total_cmp(&b.1).then(a.0.cmp(&b.0)))
        .cloned()
}

/// One cut of a worse bundle by family: at most four evaluations and one
/// sheet, no recursion. `Ok(None)` leaves the round to stall, with each
/// family's verdict recorded.
#[allow(clippy::too_many_arguments)]
pub(super) fn split(
    state: &mut Run,
    services: &mut dyn Services,
    save: Save<'_>,
    old: usize,
    base: &str,
    track: &Track,
    ledger: Option<String>,
    variants: &mut Vec<Variant>,
    shown: &[(String, f64)],
) -> Result<Option<String>, String> {
    if services.max_split_reviews() == 0 {
        return Ok(None);
    }
    let Some((key, strength)) = smallest(shown) else {
        return Ok(None);
    };
    let Some(index) = state.trials.iter().position(|t| t.key == key) else {
        return Ok(None);
    };
    let Some(bundle) = state.trials[index].bundle.clone() else {
        return Ok(None);
    };
    let parts = families(&bundle.moves, &state.dials);
    if parts.len() < 2 {
        return Ok(None);
    }
    let mut drawn = vec![];
    for (family, moves) in &parts {
        let overlay = overlay_of(moves, &state.dials);
        let cut = part(moves, strength, base, &track.name);
        let label = format!("bundle@{strength}/{family}");
        let trial = evaluate(
            state,
            services,
            save,
            &cut,
            &overlay,
            old,
            Some(bundle.id.clone()),
            &label,
            ledger.clone(),
        )?;
        if state.trials[trial].feasible {
            variants.push(Variant { trial, overlay });
            drawn.push(trial);
        }
    }
    if drawn.is_empty() {
        return Ok(None);
    }
    let priorities = look::track_objectives(state, track);
    let look = services.sheet_request(state, old, &drawn, &priorities, track.view.as_deref())?;
    note_unshown(state, &look);
    let Some(plan) = &look.plan else {
        return Ok(None);
    };
    let verdict = match sheet::review(state, services, save, plan) {
        Ok(verdict) => verdict,
        Err(reason) => {
            note_failure(state, &look);
            save(state)?;
            return Err(reason);
        }
    };
    let judged = read_back(state, &look, &verdict, old);
    if let Some(key) = sheet::adopt(&verdict, &judged) {
        save(state)?;
        return Ok(Some(key));
    }
    for index in &look.shown {
        let (label, outcome) = (
            state.trials[*index].label.clone(),
            state.trials[*index].sheet.clone(),
        );
        if let Some(outcome) = outcome {
            state.routes.push(self::verdict(&label, &outcome));
        }
    }
    save(state)?;
    Ok(None)
}
