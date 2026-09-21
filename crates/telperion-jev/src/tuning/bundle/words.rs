//! What a bundle attempt says to whoever is asked next: what it moved, how
//! far, and what the reviewer made of it, small enough to sit beside every
//! other attempt in one proposal state.
use crate::tuning::evaluation::Trial;
use serde_json::{json, Value};

pub(in crate::tuning) fn words(trial: &Trial) -> Option<Value> {
    let bundle = trial.bundle.as_ref()?;
    let mut out = json!({"strength":bundle.strength,"feasible":trial.feasible,
        "moves":bundle.moves.iter().map(|m| json!({"dial":m.dial,"direction":m.direction}))
            .collect::<Vec<_>>(),
        "dropped":bundle.dropped.iter().map(|d| d.dial.clone()).collect::<Vec<_>>()});
    if let Some(parent) = &trial.parent_bundle {
        out["half_of"] = json!(parent);
    }
    if let Some(sheet) = &trial.sheet {
        out["per_priority"] = json!(sheet.per_priority);
        out["breaks"] = json!(sheet.breaks);
        out["improved"] = json!(sheet.improved);
        out["missing"] = json!(sheet.missing);
        out["inert"] = json!(sheet.inert);
        out["note"] = json!(sheet.note);
        out["uncalibrated"] = json!(sheet.uncalibrated);
    }
    Some(out)
}
