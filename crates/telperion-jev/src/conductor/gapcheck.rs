//! The check every gap in a tuning run's result gets, in the order the
//! owner set on 2026-09-22: reachable with a dial not yet tried goes back to
//! tuning; covered by an open spec becomes a dependency; new is packaged in
//! the shape fn-95 shares and escalated to the host. Code answers what it
//! can without a call (a route the tuning loop already tied to a spec, a
//! trait every dial was tried on); Jev answers the two semantic questions
//! over explicit candidates, each with a no-match answer.
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use super::questions::{self, Asker};
use super::state::Run;
use super::{now, ConductorError, Config, Result};
use crate::pipeline::canon::{read_json, write_canonical};
use crate::tuning::result::{EndResult, GapEntry};

pub const PASSING: &str = "passing on the current tree";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "verdict", rename_all = "snake_case")]
pub enum Verdict {
    Reachable { dials: Vec<String> },
    Covered { spec: String },
    New { package: PathBuf },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Checked {
    pub gap_id: String,
    pub revision: u64,
    #[serde(flatten)]
    pub verdict: Verdict,
    pub ledger: Vec<String>,
    pub at: String,
}

pub fn read_result(dir: &Path) -> Result<EndResult> {
    let path = dir.join("result.json");
    serde_json::from_value(read_json(&path)?)
        .map_err(|err| ConductorError::Invalid(format!("{}: {err}", path.display())))
}

/// The authored dials the tuning config names: id and meaning only.
pub fn dials(config: &Config) -> Result<Vec<Value>> {
    let tuning = read_json(&config.tuning_config)?;
    Ok(tuning["dials"]
        .as_array()
        .into_iter()
        .flatten()
        .map(|d| json!({"id": d["id"], "meaning": d["meaning"]}))
        .filter(|d| d["id"].is_string())
        .collect())
}

/// Every open spec under the Flow tree, by id and title, the species' own
/// spec left out: a gap is never covered by the run that found it.
pub fn open_specs(config: &Config) -> Result<Vec<Value>> {
    let dir = config.flow.join("specs");
    let mut specs = Vec::new();
    let entries = std::fs::read_dir(&dir)
        .map_err(|err| ConductorError::Invalid(format!("{}: {err}", dir.display())))?;
    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().is_none_or(|e| e != "json") {
            continue;
        }
        let Ok(spec) = read_json(&path) else { continue };
        let id = spec["id"].as_str().unwrap_or_default();
        if spec["status"] == "open" && !id.is_empty() && id != config.spec {
            specs.push(json!({"id": id, "title": spec["title"]}));
        }
    }
    specs.sort_by(|a, b| a["id"].as_str().cmp(&b["id"].as_str()));
    Ok(specs)
}

fn untried(entry: &GapEntry, dials: &[Value]) -> Vec<Value> {
    dials
        .iter()
        .filter(|d| {
            let id = d["id"].as_str().unwrap_or_default();
            !entry.attempts.iter().any(|a| a.dial == id)
        })
        .cloned()
        .collect()
}

fn evidence(entry: &GapEntry, result: &EndResult) -> Value {
    json!({
        "trait": entry.priority,
        "status": entry.status,
        "latest_route": entry.latest_route,
        "reviewer_words": entry.reviewer_words,
        "attempts": entry.attempts.iter().map(|a| json!({
            "round": a.round, "dial": a.dial, "feasible": a.feasible, "reason": a.reason,
            "score_before": a.score_before_round, "score_after": a.score_after,
        })).collect::<Vec<_>>(),
        "seed": result.outcome.seed,
        "preset": result.outcome.preset,
    })
}

/// The package a new gap leaves for the host, in the shape fn-95 shares
/// with the studio: the trait in the reviewer's words, the rounds tried with
/// the overlay and seed, the words per attempt, the stills' identities and
/// the check's verdict. It mints nothing.
pub fn package(
    config: &Config,
    result: &EndResult,
    entry: &GapEntry,
    ledger: &[String],
) -> Result<PathBuf> {
    let path = config.gaps_dir().join(format!("{}.json", entry.id));
    let current = result.outcome.current.as_ref();
    write_canonical(
        &path,
        &json!({
            "schema": "gap-package",
            "schema_version": super::SCHEMA_VERSION,
            "species": config.species,
            "spec": config.spec,
            "gap_id": entry.id,
            "trait": entry.priority,
            "check": "new",
            "run_identity": result.outcome.run_identity,
            "seed": result.outcome.seed,
            "preset": result.outcome.preset,
            "overlay": current.map(|c| c.overrides.clone()).unwrap_or(Value::Null),
            "rounds": entry.attempts.iter().map(|a| json!({
                "round": a.round, "dial": a.dial, "feasible": a.feasible, "reason": a.reason,
                "review": a.review,
            })).collect::<Vec<_>>(),
            "reviewer_words": entry.reviewer_words,
            "stills": entry.stills.iter().take(4).map(|s| json!({
                "view": s.view, "seed": s.seed, "sha256": s.sha256, "path": s.path,
            })).collect::<Vec<_>>(),
            "ledger": ledger,
            "note": crate::tuning::result::NEW_GAP_NOTE,
            "at": now(),
        }),
    )?;
    Ok(path)
}

/// Checks one gap. Code first, then at most two calls.
pub fn check(
    asker: &Asker<'_>,
    run: &mut Run,
    result: &EndResult,
    entry: &GapEntry,
    revision: u64,
) -> Result<Option<Checked>> {
    if entry.status == PASSING {
        return Ok(None);
    }
    let mut ledger = Vec::new();
    let verdict = if let Some(spec) = &entry.existing_spec {
        Verdict::Covered { spec: spec.clone() }
    } else {
        let candidates = untried(entry, &dials(asker.config)?);
        let mut state = evidence(entry, result);
        let reached = if candidates.is_empty() {
            None
        } else {
            state["untried_dials"] = json!(candidates);
            let judgment = asker.ask(
                run,
                "conductor-reach",
                &state,
                &questions::reach_questions(&candidates),
            )?;
            ledger.push(judgment.reference.clone());
            questions::chosen(&judgment.entry, "reachable_with")
        };
        match reached {
            Some(dial) => Verdict::Reachable { dials: vec![dial] },
            None => {
                let specs = open_specs(asker.config)?;
                let covered = if specs.is_empty() {
                    None
                } else {
                    state["open_specs"] = json!(specs);
                    let judgment = asker.ask(
                        run,
                        "conductor-cover",
                        &state,
                        &questions::cover_questions(&specs),
                    )?;
                    ledger.push(judgment.reference.clone());
                    questions::chosen(&judgment.entry, "covered_by")
                };
                match covered {
                    Some(spec) => Verdict::Covered { spec },
                    None => Verdict::New {
                        package: package(asker.config, result, entry, &ledger)?,
                    },
                }
            }
        }
    };
    let checked = Checked {
        gap_id: entry.id.clone(),
        revision,
        verdict,
        ledger,
        at: now(),
    };
    run.gap_checks.insert(
        entry.id.clone(),
        serde_json::to_value(&checked).expect("checked serializes"),
    );
    Ok(Some(checked))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tuning::handoff::Attempt;

    fn attempt(dial: &str) -> Attempt {
        Attempt {
            dial: dial.into(),
            round: 1,
            action_ledger: None,
            score_before_round: None,
            score_after: None,
            feasible: true,
            reason: None,
            visual_outcome: None,
            review: None,
        }
    }

    #[test]
    fn untried_dials_are_the_table_minus_the_attempts() {
        let entry = GapEntry {
            id: "finding-0".into(),
            rank: 1,
            priority: "leaders lose girth".into(),
            status: "stalled in tuning".into(),
            latest_route: Some("tuning".into()),
            existing_spec: None,
            attempts: vec![attempt("crown_width"), attempt("crown_width")],
            reviewer_words: vec![],
            stills: vec![],
            check: String::new(),
        };
        let dials = vec![
            json!({"id": "crown_width", "meaning": "w"}),
            json!({"id": "taper", "meaning": "t"}),
        ];
        let left = untried(&entry, &dials);
        assert_eq!(left.len(), 1);
        assert_eq!(left[0]["id"], "taper");
    }
}
