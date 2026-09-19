//! The fix is its own spec; the run resumes where it halted (R4).
//!
//! The chosen fix is minted as a spec the species spec depends on and worked
//! under the repo's review, never inside the species run. The record carries
//! the spec id, each review verdict, and the landing. A landing is a tool
//! version: every stage from the halted one down carries `fix:<spec>` at the
//! landing commit in its idempotence key, so those stages rerun and the
//! earlier ones stay current. The halting decision is resolved by the loop
//! with the landing in its inputs, so a rerun that still halts reissues it
//! open. A landing that moves a pin is refused without the fn-53 record.

use std::collections::BTreeMap;
use std::path::Path;

use serde_json::{json, Value};

use super::option::{latest_set, GapOption};
use super::table::{load as table, Route};
use super::{now, slug, GapError, STAGE};
use crate::pipeline::decision::{
    append_decisions, read_decisions, write_decisions, Decision, DecisionParts, Resolution, Status,
};
use crate::pipeline::stage::{Paths, STAGES};

/// The review verdicts a gap spec's record takes.
pub const VERDICTS: [&str; 2] = ["needs-work", "ship"];
/// The needs-work verdicts a gap spec may take before the owner has it.
pub const NEEDS_WORK_LIMIT: usize = 2;
/// The option the loop resolves a halting decision with once its fix lands.
pub const FIX_LANDED: &str = "fix-landed";

/// The fix the run will work: a resolution to the gap-fix decision when one
/// binds, else the loop's own choice on the proceed route alone. A gap the
/// table handed to the owner or to the stronger model has no chosen fix until
/// someone writes a resolution: Jev's best match is an answer, never a
/// decision, and minting a spec from it would take the owner's route back.
pub fn fix(paths: &Paths, record: &Value) -> Result<Option<GapOption>, GapError> {
    let gap_id = record["gap"].as_str().unwrap_or_default();
    let (_, _, options) = latest_set(record)?;
    let decision_id = format!(
        "{}/{STAGE}/gap-fix/{}",
        record["species"].as_str().unwrap_or_default(),
        slug(gap_id)
    );
    let resolved = read_decisions(&paths.decisions())?
        .into_iter()
        .find(|d| d.id == decision_id)
        .and_then(|d| d.resolution.map(|r| r.option));
    let proceeded = record["route"] == Route::Proceed.key();
    let chosen = resolved.or_else(|| {
        proceeded
            .then(|| record["chosen"].as_str().map(str::to_string))
            .flatten()
    });
    Ok(chosen.and_then(|id| options.into_iter().find(|o| o.option == id)))
}

/// Records the spec minted for the fix. The route must have settled on one:
/// proceed, or the owner's resolution naming an option.
pub fn record_spec(paths: &Paths, gap_id: &str, spec: &str) -> Result<(), GapError> {
    let mut record = super::read(paths, gap_id)?;
    if spec.trim().is_empty() {
        return Err(GapError::Invalid("the spec id is empty".into()));
    }
    let Some(chosen) = fix(paths, &record)? else {
        return Err(GapError::Invalid(format!(
            "no fix is chosen for {gap_id}; the route is {} and no owner resolution names an option",
            record["route"].as_str().unwrap_or("not taken")
        )));
    };
    record["spec"] = json!(spec);
    record["fix"] = json!(chosen.option);
    super::write(paths, &record)?;
    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Reviewed {
    /// The verdict is recorded; the count of needs-work so far.
    Recorded { needs_work: usize },
    /// The second needs-work: the gap spec is the owner's now.
    Owner { decision: String },
}

/// Records one review verdict on the gap spec. The second needs-work files a
/// `gap-review` decision for the owner.
pub fn record_review(paths: &Paths, gap_id: &str, verdict: &str) -> Result<Reviewed, GapError> {
    if !VERDICTS.contains(&verdict) {
        return Err(GapError::Invalid(format!(
            "verdict {verdict} is not one of {}",
            VERDICTS.join(", ")
        )));
    }
    let mut record = super::read(paths, gap_id)?;
    if record["spec"].is_null() {
        return Err(GapError::Invalid(
            "no spec is recorded for this gap yet".into(),
        ));
    }
    record["reviews"]
        .as_array_mut()
        .expect("reviews is a list")
        .push(json!({"verdict": verdict, "at": now()}));
    let needs_work = record["reviews"]
        .as_array()
        .into_iter()
        .flatten()
        .filter(|r| r["verdict"] == "needs-work")
        .count();
    let outcome = if needs_work >= NEEDS_WORK_LIMIT {
        let species = record["species"].as_str().unwrap_or_default().to_string();
        let field = slug(gap_id);
        let decision = Decision::new(
            DecisionParts {
                species: &species,
                stage: STAGE,
                kind: "gap-review",
                field: Some(&field),
                age_years: None,
            },
            &[],
            [("spec".to_string(), record["spec"].as_str().unwrap_or_default().to_string())].into(),
            vec![],
            json!({"gap": gap_id, "spec": record["spec"], "needs_work": needs_work}),
            &["rework", "drop-fix", "waive-gap"],
            "The gap spec came back needs-work twice; the owner decides whether it is reworked, dropped for another option, or the gap waived.",
        );
        let id = decision.id.clone();
        append_decisions(&paths.decisions(), vec![decision])?;
        record["route"] = json!(Route::Owner.key());
        Reviewed::Owner { decision: id }
    } else {
        Reviewed::Recorded { needs_work }
    };
    super::write(paths, &record)?;
    Ok(outcome)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Resumed {
    pub spec: String,
    /// The stages whose keys expired, from the halted one down.
    pub reruns: Vec<String>,
}

/// Records the landing and expires the run from the halted stage down.
pub fn resume(
    paths: &Paths,
    gap_id: &str,
    commit: &str,
    pin_note: Option<&str>,
) -> Result<Resumed, GapError> {
    let mut record = super::read(paths, gap_id)?;
    let Some(spec) = record["spec"].as_str().map(str::to_string) else {
        return Err(GapError::Invalid(
            "no spec is recorded for this gap; mint it and record it first".into(),
        ));
    };
    if !record["landed"].is_null() {
        return Err(GapError::Invalid(format!(
            "{gap_id} already resumed at {}",
            record["landed"]["commit"].as_str().unwrap_or_default()
        )));
    }
    if commit.trim().is_empty() {
        return Err(GapError::Invalid("the landing commit is empty".into()));
    }
    let chosen = fix(paths, &record)?;
    let moves_pin = chosen.as_ref().is_some_and(|o| o.moves_pin);
    if moves_pin && pin_note.is_none_or(str::is_empty) {
        return Err(GapError::Invalid(format!(
            "the fix {} moves a pin; pass --pin-note '<preset>: <change>: <reason>' (fn-53: the record names the preset, the change and the reason, and the pins move once)",
            chosen.map(|o| o.option).unwrap_or_default()
        )));
    }
    let halt_stage = record["halt"]["stage"]
        .as_str()
        .unwrap_or_default()
        .to_string();
    let Some(from) = STAGES.iter().position(|s| *s == halt_stage) else {
        return Err(GapError::Invalid(format!(
            "the halt names no stage of the run: {halt_stage}"
        )));
    };
    record["landed"] = json!({
        "spec": spec,
        "commit": commit,
        "at": now(),
        "pin": pin_note,
    });
    resolve_halt(paths, &record, &spec, commit)?;
    super::write(paths, &record)?;
    Ok(Resumed {
        spec,
        reruns: STAGES[from..].iter().map(|s| s.to_string()).collect(),
    })
}

/// Resolves the halting decision by the loop, with the landing in its
/// inputs: a rerun that files the same halt again carries the plain inputs,
/// so the reissued decision replaces this one and opens.
fn resolve_halt(paths: &Paths, record: &Value, spec: &str, commit: &str) -> Result<(), GapError> {
    let halt_id = record["halt"]["id"].as_str().unwrap_or_default();
    let mut decisions = read_decisions(&paths.decisions())?;
    let Some(decision) = decisions.iter_mut().find(|d| d.id == halt_id) else {
        return Err(GapError::Invalid(format!(
            "the halting decision {halt_id} is no longer in the list"
        )));
    };
    decision
        .inputs_sha256
        .insert(format!("fix:{spec}"), commit.to_string());
    if !decision.options.iter().any(|o| o == FIX_LANDED) {
        decision.options.push(FIX_LANDED.into());
    }
    decision.status = Status::Resolved;
    decision.consumed_by = Some(STAGE.into());
    decision.resolution = Some(Resolution {
        id: decision.id.clone(),
        inputs_sha256: decision.inputs_sha256.clone(),
        option: FIX_LANDED.into(),
        by: "loop".into(),
        at: now(),
        note: format!("{spec} landed at {commit}"),
        payload: json!({"spec": spec, "commit": commit}),
    });
    write_decisions(&paths.decisions(), &decisions)?;
    Ok(())
}

/// The tool versions a landing adds to `stage`'s idempotence key:
/// `fix:<spec>` at its commit for every landed gap whose halt is at or before
/// `stage`. Fail-open on an unreadable record: a missing key reruns nothing
/// silently, so the read error surfaces on the next gap command instead.
pub fn landed_tools(dir: &Path, stage: &str) -> BTreeMap<String, String> {
    let Some(index) = STAGES.iter().position(|s| *s == stage) else {
        return BTreeMap::new();
    };
    super::all(dir)
        .unwrap_or_default()
        .iter()
        .filter(|record| !record["landed"].is_null())
        .filter(|record| {
            let halt = record["halt"]["stage"].as_str().unwrap_or_default();
            STAGES
                .iter()
                .position(|s| *s == halt)
                .is_some_and(|h| h <= index)
        })
        .filter_map(|record| {
            let spec = record["landed"]["spec"].as_str()?;
            let commit = record["landed"]["commit"].as_str()?;
            Some((format!("fix:{spec}"), commit.to_string()))
        })
        .collect()
}

/// True once the halted gap's rerun no longer halts: the loop's resolution
/// still stands on the halting decision. The route table's version is
/// recorded so a reader knows which rules the gap ran under.
pub fn table_version() -> u32 {
    table().version
}
