//! The three numbers a species run records (R5).
//!
//! Autonomy is the share of gap decisions the loop took itself. Quality is
//! the rounds each verdict took to accept and the routes the owner reversed,
//! by decision id. Efficiency is the tokens, the wall clock and the captures
//! the run spent. Every number is read from artifacts already on disk, so the
//! record is derived and never a second source of truth; `metrics.json` sits
//! beside `report.json` and the report's own cost table.
//!
//! A reversal is a gap the loop resolved itself whose gap-fix decision now
//! carries a resolution written by someone else naming another option. It is
//! recorded, never counted as a failure: it is the signal the next threshold
//! tuning reads.

use serde_json::{json, Map, Value};

use super::table::{load as table, Route};
use super::{now, rounds, GapError, STAGE};
use crate::pipeline::canon::{read_json, write_canonical};
use crate::pipeline::cost::Cost;
use crate::pipeline::decision::read_decisions;
use crate::pipeline::stage::{Paths, STAGES};

pub const METRICS_SCHEMA_VERSION: u32 = 1;
/// The resolver the loop writes on its own resolutions; anything else is a
/// person, and a person naming another option is a reversal.
pub const BY_LOOP: &str = "loop";

pub fn metrics_path(paths: &Paths) -> std::path::PathBuf {
    paths.dir.join("metrics.json")
}

/// One reversal: what the loop chose, what the owner wrote instead.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Reversal {
    pub decision: String,
    pub gap: String,
    pub chose: String,
    pub instead: String,
    pub by: String,
}

/// Every gap the loop decided that a person then decided differently.
pub fn reversals(paths: &Paths) -> Result<Vec<Reversal>, GapError> {
    let decisions = read_decisions(&paths.decisions())?;
    let mut out = Vec::new();
    for record in super::all(&paths.dir)? {
        let gap = record["gap"].as_str().unwrap_or_default();
        let Some(chose) = loop_choice(&record) else {
            continue;
        };
        let id = format!(
            "{}/{STAGE}/gap-fix/{}",
            record["species"].as_str().unwrap_or_default(),
            super::slug(gap)
        );
        let Some(resolution) = decisions
            .iter()
            .find(|d| d.id == id)
            .and_then(|d| d.resolution.as_ref())
        else {
            continue;
        };
        if resolution.by != BY_LOOP && resolution.option != chose {
            out.push(Reversal {
                decision: id,
                gap: gap.to_string(),
                chose,
                instead: resolution.option.clone(),
                by: resolution.by.clone(),
            });
        }
    }
    Ok(out)
}

/// The option the loop itself proceeded with, when it did.
fn loop_choice(record: &Value) -> Option<String> {
    let last = record["routes"].as_array()?.last()?;
    (last["route"] == Route::Proceed.key())
        .then(|| last["chosen"].as_str().map(str::to_string))
        .flatten()
}

/// Gap routes by their latest route, and the share the loop took itself.
fn autonomy(paths: &Paths) -> Result<Value, GapError> {
    let mut counts: Map<String, Value> = [Route::Proceed, Route::Stronger, Route::Owner]
        .into_iter()
        .map(|route| (route.key().to_string(), json!(0)))
        .collect();
    let mut total = 0u64;
    for record in super::all(&paths.dir)? {
        let Some(route) = record["route"].as_str() else {
            continue;
        };
        let count = counts.entry(route.to_string()).or_insert_with(|| json!(0));
        *count = json!(count.as_u64().unwrap_or(0) + 1);
        total += 1;
    }
    let taken = counts[Route::Proceed.key()].as_u64().unwrap_or(0);
    Ok(json!({
        "gaps": total,
        "decisions": counts,
        "share_taken": if total == 0 { 0.0 } else { taken as f64 / total as f64 },
    }))
}

/// Tokens and wall clock from the run's ledger entries, credits and calls
/// from the stage artifacts' own cost, captures from the stills generation
/// drew. An unreadable entry is skipped and counted, so a number the record
/// shows is always a number it could read.
fn efficiency(paths: &Paths) -> Value {
    let mut input = 0u64;
    let mut output = 0u64;
    let mut wall = 0u64;
    let mut entries = 0u64;
    let mut unreadable = 0u64;
    let dir = paths.ledger().join("entries");
    if let Ok(read_dir) = std::fs::read_dir(&dir) {
        for path in read_dir.filter_map(Result::ok).map(|e| e.path()) {
            if path.extension().is_none_or(|ext| ext != "json") {
                continue;
            }
            match read_json(&path) {
                Ok(entry) => {
                    entries += 1;
                    input += entry["usage"]["input_tokens"].as_u64().unwrap_or(0);
                    output += entry["usage"]["output_tokens"].as_u64().unwrap_or(0);
                    wall += entry["elapsed_ms"].as_u64().unwrap_or(0);
                }
                Err(_) => unreadable += 1,
            }
        }
    }
    let mut cost = Cost::default();
    for stage in STAGES {
        let artifact = paths.artifact(stage);
        if !artifact.exists() {
            continue;
        }
        if let Ok(value) = read_json(&artifact) {
            if let Ok(stage_cost) = serde_json::from_value::<Cost>(value["cost"].clone()) {
                cost.add(&stage_cost);
            }
        }
    }
    json!({
        "ledger_entries": entries,
        "unreadable_entries": unreadable,
        "input_tokens": input,
        "output_tokens": output,
        "wall_clock_ms": wall,
        "jev_calls": cost.jev_calls,
        "firecrawl_credits": cost.firecrawl_credits,
        "captures": captures(paths),
    })
}

/// The stills generation drew, counted from its artifact: a still without a
/// path was never drawn.
fn captures(paths: &Paths) -> u64 {
    let path = paths.artifact("generate");
    if !path.exists() {
        return 0;
    }
    read_json(&path)
        .ok()
        .and_then(|value| {
            Some(
                value["body"]["stills"]
                    .as_array()?
                    .iter()
                    .filter(|still| still.get("path").is_some_and(|p| !p.is_null()))
                    .count() as u64,
            )
        })
        .unwrap_or(0)
}

/// The run's metrics, derived from the artifacts on disk.
pub fn compute(paths: &Paths, species: &str) -> Result<Value, GapError> {
    let reversed: Vec<Value> = reversals(paths)?
        .into_iter()
        .map(|r| {
            json!({"decision": r.decision, "gap": r.gap, "chose": r.chose,
                   "instead": r.instead, "by": r.by})
        })
        .collect();
    let to_acceptance: Map<String, Value> = rounds::to_acceptance(paths)?
        .into_iter()
        .map(|(verdict, round)| (verdict, json!(round)))
        .collect();
    Ok(json!({
        "schema": "metrics",
        "schema_version": METRICS_SCHEMA_VERSION,
        "species": species,
        "at": now(),
        "table_version": table().version,
        "autonomy": autonomy(paths)?,
        "quality": {
            "rounds_to_acceptance": to_acceptance,
            "rounds_total": rounds::total(paths)?,
            "reversals": reversed,
        },
        "efficiency": efficiency(paths),
    }))
}

/// Computes and writes `metrics.json`. The report's own status is unchanged:
/// a run with no record fails its report, it does not fail a stage.
pub fn write(paths: &Paths, species: &str) -> Result<Value, GapError> {
    let record = compute(paths, species)?;
    write_canonical(&metrics_path(paths), &record)?;
    Ok(record)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pipeline::decision::{write_decisions, Decision, DecisionParts, Resolution, Status};

    fn scratch() -> Paths {
        let dir = std::env::temp_dir().join(format!(
            "jev-metrics-{}-{}",
            std::process::id(),
            crate::ledger::new_entry_id()
        ));
        std::fs::create_dir_all(&dir).unwrap();
        Paths::new(&dir)
    }

    fn gap_record(paths: &Paths, gap: &str, route: &str, chosen: &str) {
        let record = json!({
            "gap": gap,
            "species": "silver-birch",
            "halt": {"stage": "gate"},
            "routes": [{"route": route, "chosen": chosen}],
            "route": route,
            "chosen": chosen,
            "landed": Value::Null,
        });
        super::super::write(paths, &record).unwrap();
    }

    fn gap_fix(field: &str, option: &str, by: &str) -> Decision {
        let mut decision = Decision::new(
            DecisionParts {
                species: "silver-birch",
                stage: STAGE,
                kind: "gap-fix",
                field: Some(field),
                age_years: None,
            },
            &[],
            Default::default(),
            vec![],
            json!({}),
            &[option],
            "",
        );
        decision.status = Status::Resolved;
        decision.resolution = Some(Resolution {
            id: decision.id.clone(),
            inputs_sha256: Default::default(),
            option: option.into(),
            by: by.into(),
            at: "2026-09-18T00:00:00Z".into(),
            note: String::new(),
            payload: Value::Null,
        });
        decision
    }

    #[test]
    fn autonomy_counts_every_route_and_the_share_the_loop_took() {
        let paths = scratch();
        assert_eq!(autonomy(&paths).unwrap()["share_taken"], 0.0);
        gap_record(
            &paths,
            "silver-birch/gate/onboarding-gate/curtain",
            "proceed",
            "a",
        );
        gap_record(
            &paths,
            "silver-birch/gate/onboarding-gate/stems",
            "owner",
            "b",
        );
        let counts = autonomy(&paths).unwrap();
        assert_eq!(counts["gaps"], 2);
        assert_eq!(counts["decisions"]["proceed"], 1);
        assert_eq!(counts["decisions"]["owner"], 1);
        assert_eq!(counts["decisions"]["stronger"], 0);
        assert_eq!(counts["share_taken"], 0.5);
    }

    #[test]
    fn an_owner_resolution_naming_another_option_is_the_reversal_the_record_carries() {
        let paths = scratch();
        gap_record(
            &paths,
            "silver-birch/gate/onboarding-gate/curtain",
            "proceed",
            "rows",
        );
        // The loop's own resolution is no reversal.
        write_decisions(
            &paths.decisions(),
            &[gap_fix("gate-onboarding-gate-curtain", "rows", BY_LOOP)],
        )
        .unwrap();
        assert!(reversals(&paths).unwrap().is_empty());
        // The owner agreeing is no reversal either.
        write_decisions(
            &paths.decisions(),
            &[gap_fix("gate-onboarding-gate-curtain", "rows", "owner")],
        )
        .unwrap();
        assert!(reversals(&paths).unwrap().is_empty());
        write_decisions(
            &paths.decisions(),
            &[gap_fix("gate-onboarding-gate-curtain", "constant", "owner")],
        )
        .unwrap();
        let found = reversals(&paths).unwrap();
        assert_eq!(found.len(), 1);
        assert_eq!(found[0].chose, "rows");
        assert_eq!(found[0].instead, "constant");
        assert_eq!(
            found[0].decision,
            "silver-birch/gap/gap-fix/gate-onboarding-gate-curtain"
        );
    }

    #[test]
    fn a_gap_the_owner_took_is_never_a_reversal_and_the_record_carries_all_three_numbers() {
        let paths = scratch();
        gap_record(
            &paths,
            "silver-birch/gate/onboarding-gate/stems",
            "owner",
            "clump",
        );
        write_decisions(
            &paths.decisions(),
            &[gap_fix("gate-onboarding-gate-stems", "one-stem", "owner")],
        )
        .unwrap();
        assert!(reversals(&paths).unwrap().is_empty());
        rounds::open_round(&paths, "silver-birch", "crown/whole", "").unwrap();
        rounds::accept(&paths, "crown/whole").unwrap();
        let record = write(&paths, "silver-birch").unwrap();
        assert_eq!(record["autonomy"]["share_taken"], 0.0);
        assert_eq!(record["quality"]["rounds_to_acceptance"]["crown/whole"], 1);
        assert_eq!(record["quality"]["rounds_total"], 1);
        assert_eq!(record["efficiency"]["captures"], 0);
        assert_eq!(record["efficiency"]["input_tokens"], 0);
        assert_eq!(record["table_version"], table().version);
        assert!(metrics_path(&paths).exists());
    }
}
