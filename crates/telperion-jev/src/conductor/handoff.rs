//! The human escalation: a paused run with everything the decision needs in
//! one file, and the scoped decision that resumes it. Nothing dispatches
//! while the pause stands; resuming retains every budget as spent and
//! rechecks the evidence on the next step rather than trusting the pause.
use std::path::PathBuf;

use serde_json::{json, Value};

use super::state::{Dependency, DependencyStatus, Run};
use super::{now, Config, Result};
use crate::pipeline::canon::{read_json, write_canonical};
use crate::tuning::continuation::{Basis, HumanDecision};

pub const REQUESTED_ON_NEW_GAP: &str =
    "mint a generator spec for each packaged gap and attach it, or park the species";

fn renders(config: &Config, run: &Run) -> Value {
    match run.latest_tuning() {
        Some(latest) => match super::gapcheck::read_result(&latest.out) {
            Ok(result) => match result.outcome.current {
                Some(current) => json!(current
                    .stills
                    .iter()
                    .take(4)
                    .map(|s| json!({
                        "view": s.view, "seed": s.seed, "sha256": s.sha256, "path": s.path,
                    }))
                    .collect::<Vec<_>>()),
                None => json!({"missing": "the tuning run never had a current candidate"}),
            },
            Err(err) => json!({"missing": format!("the tuning result could not be read: {err}")}),
        },
        None => json!({"missing": format!("no tuning revision has run for {}", config.species)}),
    }
}

fn attempts(run: &Run) -> Vec<Value> {
    run.dispatches
        .iter()
        .map(|d| {
            json!({
                "id": d.id, "role": d.role, "route": d.route, "tier": d.tier, "effort": d.effort,
                "dependency": d.dependency,
                "outcome": d.result.as_ref().and_then(|r| r.outcome),
                "actual_model": d.result.as_ref().map(|r| r.actual_model.clone()),
                "actual_effort": d.result.as_ref().map(|r| r.actual_effort.clone()),
                "failure": d.result.as_ref().and_then(|r| r.failure.clone()),
            })
        })
        .collect()
}

pub fn handoff_file(config: &Config, id: &str) -> PathBuf {
    config.conductor_dir().join(format!("handoff-{id}.json"))
}

/// Pauses the run and writes the handoff: current renders or why they are
/// missing, the unresolved requirement, every attempt and its outcome, the
/// spend and the remaining budget, the proposed next action with its
/// allowance and basis, the risk signals, and the decision requested.
pub fn pause(
    config: &Config,
    run: &mut Run,
    id: &str,
    reason: &str,
    basis: Basis,
    signals: &Value,
    requested: &str,
) -> Result<PathBuf> {
    let handoff = json!({
        "schema": "conductor-handoff",
        "schema_version": super::SCHEMA_VERSION,
        "pause_id": id,
        "species": run.species,
        "spec": run.spec,
        "reason": reason,
        "renders": renders(config, run),
        "unresolved": basis.evidence,
        "attempts": attempts(run),
        "recent_outcomes": basis.recent_outcomes,
        "spend": {"tokens": run.budget.tokens, "usage_known": run.budget.usage_known,
                  "jev_calls": run.budget.jev_calls, "dispatches": run.dispatches.len(),
                  "tuning_revisions": run.tuning.len()},
        "remaining": {"tokens": run.budget.remaining(),
                      "dispatches": run.budget.max_dispatches
                          .map(|cap| cap.saturating_sub(run.dispatches.len() as u64)),
                      "tuning_revisions": run.budget.max_tuning_revisions
                          .map(|cap| cap.saturating_sub(run.tuning.len() as u64))},
        "proposed": {"action": basis.proposed_action, "allowance_tokens": basis.next_tokens,
                     "estimate_basis": basis.estimate_basis},
        "signals": signals,
        "decision_requested": requested,
        "resume": {"pause_id": id, "identity": basis.identity, "action": basis.proposed_action,
                   "note": "the decision file names pause_id, identity and action exactly, with by and rationale"},
        "gap_packages": packages(config),
        "at": now(),
    });
    let path = handoff_file(config, id);
    write_canonical(&path, &handoff)?;
    std::fs::write(path.with_extension("md"), markdown(&handoff))
        .map_err(|err| super::ConductorError::Invalid(format!("{}: {err}", path.display())))?;
    run.pause(id, reason, basis, requested);
    Ok(path)
}

fn packages(config: &Config) -> Vec<String> {
    let dir = config.gaps_dir();
    let Ok(entries) = std::fs::read_dir(&dir) else {
        return Vec::new();
    };
    let mut paths: Vec<String> = entries
        .flatten()
        .map(|e| e.path().display().to_string())
        .collect();
    paths.sort();
    paths
}

fn markdown(h: &Value) -> String {
    let mut s = String::new();
    s.push_str(&format!(
        "# Human handoff {} for {}\n\n",
        h["pause_id"].as_str().unwrap_or_default(),
        h["species"].as_str().unwrap_or_default()
    ));
    s.push_str(&format!(
        "**Reason.** {}\n\n",
        h["reason"].as_str().unwrap_or_default()
    ));
    s.push_str(&format!(
        "**Decision requested.** {}\n\n",
        h["decision_requested"].as_str().unwrap_or_default()
    ));
    s.push_str(&format!(
        "**Proposed next action.** {} (allowance {} tokens; {})\n\n",
        h["proposed"]["action"].as_str().unwrap_or_default(),
        h["proposed"]["allowance_tokens"],
        h["proposed"]["estimate_basis"].as_str().unwrap_or_default()
    ));
    s.push_str(&format!("**Spend.** {} tokens ({}), {} Jev calls, {} dispatches, {} tuning revisions; remaining {} tokens.\n\n",
        h["spend"]["tokens"],
        if h["spend"]["usage_known"] == true { "known" } else { "partly unknown" },
        h["spend"]["jev_calls"], h["spend"]["dispatches"], h["spend"]["tuning_revisions"],
        h["remaining"]["tokens"]));
    s.push_str("## Renders\n\n");
    match h["renders"].as_array() {
        Some(list) => {
            for r in list {
                s.push_str(&format!(
                    "- {} seed {}: `{}` ({})\n",
                    r["view"].as_str().unwrap_or_default(),
                    r["seed"],
                    r["path"].as_str().unwrap_or_default(),
                    &r["sha256"].as_str().unwrap_or_default()
                        [..12.min(r["sha256"].as_str().unwrap_or_default().len())]
                ));
            }
        }
        None => s.push_str(&format!(
            "Missing: {}\n",
            h["renders"]["missing"].as_str().unwrap_or_default()
        )),
    }
    s.push_str("\n## Unresolved\n\n");
    for e in h["unresolved"].as_array().into_iter().flatten() {
        s.push_str(&format!("- {}\n", e.as_str().unwrap_or_default()));
    }
    s.push_str("\n## Attempts\n\n");
    for a in h["attempts"].as_array().into_iter().flatten() {
        s.push_str(&format!(
            "- {} {} on {} at {}: {}{}\n",
            a["id"].as_str().unwrap_or_default(),
            a["route"].as_str().unwrap_or_default(),
            a["tier"].as_str().unwrap_or_default(),
            a["effort"].as_str().unwrap_or_default(),
            a["outcome"].as_str().unwrap_or("open"),
            a["failure"]
                .as_str()
                .map(|f| format!(" ({f})"))
                .unwrap_or_default()
        ));
    }
    s.push_str("\n## Signals\n\n```json\n");
    s.push_str(&serde_json::to_string_pretty(&h["signals"]).unwrap_or_default());
    s.push_str("\n```\n");
    if let Some(list) = h["gap_packages"].as_array().filter(|l| !l.is_empty()) {
        s.push_str("\n## Gap packages\n\n");
        for p in list {
            s.push_str(&format!("- `{}`\n", p.as_str().unwrap_or_default()));
        }
    }
    s
}

/// Resumes from the decision file. The decision must name the pause, the
/// identity and the proposed action exactly, with who and why.
pub fn resume(run: &mut Run, decision: &std::path::Path) -> Result<String> {
    let decision: HumanDecision = serde_json::from_value(read_json(decision)?)
        .map_err(|err| super::ConductorError::Invalid(format!("{}: {err}", decision.display())))?;
    let id = decision.pause_id.clone();
    run.resume(decision)?;
    Ok(format!(
        "resumed from {id}; budgets retained, evidence rechecked on the next step"
    ))
}

/// Attaches a gap spec as a dependency: an open spec the check found, or
/// the spec the owner minted for a packaged new gap. Attaching twice is a
/// no-op, so a repeated resume creates no duplicate.
pub fn attach(run: &mut Run, spec: &str, gap_id: &str, origin: &str) -> bool {
    if run.dependencies.iter().any(|d| d.spec == spec) {
        return false;
    }
    run.dependencies.push(Dependency {
        spec: spec.into(),
        gap_id: gap_id.into(),
        origin: origin.into(),
        status: DependencyStatus::Awaiting,
        design_revision: None,
        handoff: None,
        landed_commit: None,
        attached_at: now(),
        judged: Default::default(),
    });
    true
}
