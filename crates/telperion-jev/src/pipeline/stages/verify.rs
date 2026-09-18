//! Packet verification: fn-57's citation check over every filled value
//! against its source, the semantic obligation questions, and the structural
//! obligations in code. Contradicted, unsupported and unmet items become
//! decisions of their kinds.

use serde_json::{json, Value};

use crate::cite::{cite, ResearchClaim, SourceLoad};
use crate::pipeline::canon::read_json;
use crate::pipeline::decision::{append_decisions, Decision, DecisionParts};
use crate::pipeline::judge::Judge;
use crate::pipeline::sets::{measurement_state, obligation_questions};
use crate::pipeline::stage::{Context, Paths, StageError};
use crate::questions::thresholds;

use super::extract::cached_markdown;
use super::{body, inputs};

pub const STAGE: &str = "verify";

#[derive(Debug)]
pub enum Outcome {
    Current,
    Ran { decisions: Vec<String> },
}

pub fn run(paths: &Paths, judge: &Judge<'_>) -> Result<Outcome, StageError> {
    let (ctx, _) = Context::open(paths, STAGE)?;
    let (fetch, fetch_sha) = body(&ctx, STAGE, "fetch")?;
    let (_, select_sha) = body(&ctx, STAGE, "select")?;
    let mut header = ctx.header(
        STAGE,
        "verify",
        inputs(&[("fetch.json", &fetch_sha), ("select.json", &select_sha)]),
        vec![],
    );
    if ctx.is_current(STAGE, &header.idempotence_key) {
        return Ok(Outcome::Current);
    }
    let sidecar = read_json(&ctx.paths.sidecar())?;
    let references = read_json(&ctx.paths.packet("references"))?;
    let manifest = &ctx.admitted.manifest;
    let species = manifest.species.as_str();
    let mut decisions = Vec::new();

    let (claims, loads) = claims_for(&ctx, &sidecar, &fetch)?;
    let report = cite(
        judge.transport,
        judge.key,
        &judge.ledger_dir,
        &claims,
        &loads,
    )
    .map_err(|err| StageError::Failed {
        stage: STAGE.into(),
        reason: err.to_string(),
    })?;
    let mut rows = Vec::new();
    for (claim, row) in claims.iter().zip(report.rows.iter()) {
        if !row.identity.is_empty() {
            header.ledger.push(row.identity.clone());
        }
        rows.push(json!({
            "claim": row.claim, "relation": row.relation, "listed": row.listed,
            "reason": row.reason, "section": row.section, "ledger": row.identity,
        }));
        if row.listed && row.relation != "unchecked" {
            let kind = if row.relation == "contradicts" {
                "claim-contradicted"
            } else {
                "claim-unsupported"
            };
            decisions.push(Decision::new(
                DecisionParts { species, stage: STAGE, kind, field: Some(&claim.source_id), age_years: None },
                &["generate"],
                [("select.json".to_string(), select_sha.clone())].into_iter().collect(),
                vec![row.identity.clone()],
                json!({"claim": row.claim, "section": row.section, "relation": row.relation, "reason": row.reason}),
                &["accept", "replace-source", "drop-value"],
                "The citation check listed this claim for a person.",
            ));
        }
    }

    let mut obligations = Vec::new();
    for reference in references["references"].as_array().into_iter().flatten() {
        let state = json!({"observation": reference["observation"]});
        let judgment = judge
            .ask(
                "obligation:inspected_image",
                None,
                &state,
                &obligation_questions("inspected_image"),
            )
            .map_err(|err| StageError::Failed {
                stage: STAGE.into(),
                reason: err.to_string(),
            })?;
        let held =
            judgment.entry.noul("inspected_image").unwrap_or(0.0) >= thresholds().obligation_cut;
        header.ledger.push(judgment.reference.clone());
        obligations.push(json!({"obligation": "inspected_image", "reference": reference["id"], "held": held, "ledger": judgment.reference}));
        if !held {
            decisions.push(unmet(
                species,
                "inspected_image",
                reference["id"].as_str().unwrap_or(""),
                &judgment.reference,
                &select_sha,
            ));
        }
    }
    for (pointer, entry) in sidecar["entries"].as_object().into_iter().flatten() {
        let span = entry["span"].as_str().unwrap_or_default();
        let source_id = entry["source"].as_str().unwrap_or_default();
        let Some(excerpt) = excerpt_for(&ctx, source_id, &fetch["sources"][source_id], span) else {
            obligations.push(json!({"obligation": "measurement_not_invention", "pointer": pointer, "held": Value::Null, "unchecked": "source not cached"}));
            continue;
        };
        let state = measurement_state(span, &excerpt);
        let judgment = judge
            .ask(
                "obligation:measurement_not_invention",
                None,
                &state,
                &obligation_questions("measurement_not_invention"),
            )
            .map_err(|err| StageError::Failed {
                stage: STAGE.into(),
                reason: err.to_string(),
            })?;
        let held = judgment
            .entry
            .noul("measurement_not_invention")
            .unwrap_or(0.0)
            >= thresholds().obligation_cut;
        header.ledger.push(judgment.reference.clone());
        obligations.push(json!({"obligation": "measurement_not_invention", "pointer": pointer, "held": held, "ledger": judgment.reference}));
        if !held {
            decisions.push(unmet(
                species,
                "measurement_not_invention",
                pointer,
                &judgment.reference,
                &select_sha,
            ));
        }
    }

    let structural = structural_checks(manifest, &sidecar, &references);
    for failure in &structural {
        decisions.push(Decision::new(
            DecisionParts {
                species,
                stage: STAGE,
                kind: "structural-unmet",
                field: failure["value"].as_str(),
                age_years: None,
            },
            &["generate"],
            [("select.json".to_string(), select_sha.clone())]
                .into_iter()
                .collect(),
            vec![],
            failure.clone(),
            &["fix-packet", "drop-value"],
            "A structural obligation failed in code.",
        ));
    }
    let ids: Vec<String> = decisions.iter().map(|d| d.id.clone()).collect();
    if !decisions.is_empty() {
        append_decisions(&ctx.paths.decisions(), decisions)?;
    }
    ctx.write(
        &header,
        json!({"claims": rows, "obligations": obligations, "structural": structural}),
    )?;
    Ok(Outcome::Ran { decisions: ids })
}

/// One claim per filled value: the copied span, checked against the cached
/// markdown of its source. An uncached source lists its claim as unchecked.
fn claims_for(
    ctx: &Context,
    sidecar: &Value,
    fetch: &Value,
) -> Result<(Vec<ResearchClaim>, Vec<SourceLoad>), StageError> {
    let mut claims = Vec::new();
    let mut loads = Vec::new();
    for (pointer, entry) in sidecar["entries"].as_object().into_iter().flatten() {
        let source_id = entry["source"].as_str().unwrap_or_default().to_string();
        let record = &fetch["sources"][&source_id];
        claims.push(ResearchClaim {
            claim: format!(
                "{}: {}",
                pointer.rsplit('/').next().unwrap_or_default(),
                entry["span"].as_str().unwrap_or_default()
            ),
            url: record["final_url"].as_str().unwrap_or_default().to_string(),
            source_id: source_id.clone(),
            unresolved: None,
        });
        loads.push(match cached_markdown(ctx, STAGE, &source_id, record) {
            Ok(markdown) => SourceLoad::Bytes(markdown.into_bytes()),
            Err(err) => SourceLoad::Unreachable(err.to_string()),
        });
    }
    Ok((claims, loads))
}

/// The cached source text around the span, bounded to a few hundred
/// characters each side, or None when the source is not cached: the value's
/// evidence is its source, never the span itself.
fn excerpt_for(ctx: &Context, source_id: &str, record: &Value, span: &str) -> Option<String> {
    let markdown = cached_markdown(ctx, STAGE, source_id, record).ok()?;
    let at = markdown.find(span).unwrap_or(0);
    let start = markdown[..at]
        .char_indices()
        .rev()
        .nth(600)
        .map_or(0, |(i, _)| i);
    let end = markdown[at..]
        .char_indices()
        .nth(span.chars().count() + 600)
        .map_or(markdown.len(), |(i, _)| at + i);
    Some(markdown[start..end].to_string())
}

fn unmet(species: &str, obligation: &str, value: &str, ledger: &str, select_sha: &str) -> Decision {
    Decision::new(
        DecisionParts {
            species,
            stage: STAGE,
            kind: "obligation-unmet",
            field: Some(&format!("{obligation}:{value}")),
            age_years: None,
        },
        &["generate"],
        [("select.json".to_string(), select_sha.to_string())]
            .into_iter()
            .collect(),
        vec![ledger.to_string()],
        json!({"obligation": obligation, "value": value}),
        &["accept", "replace-reference", "drop-value"],
        "Jev judged this semantic obligation unmet.",
    )
}

/// Provenance on every filled value, an asset hash on every inspected image,
/// a non-empty rights field, a resolvable source id: code checks, each
/// failure naming the value and the rule.
pub fn structural_checks(
    manifest: &crate::pipeline::manifest::Manifest,
    sidecar: &Value,
    references: &Value,
) -> Vec<Value> {
    let mut failures = Vec::new();
    for (pointer, entry) in sidecar["entries"].as_object().into_iter().flatten() {
        let source = entry["source"].as_str().unwrap_or_default();
        if manifest.source(source).is_none() {
            failures
                .push(json!({"value": pointer, "rule": "source id resolves", "source": source}));
        }
        if entry["ledger"].as_array().is_none_or(|l| l.is_empty()) {
            failures
                .push(json!({"value": pointer, "rule": "provenance carries a ledger reference"}));
        }
    }
    for reference in references["references"].as_array().into_iter().flatten() {
        if reference["observation"]
            .as_str()
            .is_some_and(|o| !o.is_empty())
            && reference["asset_sha256"].as_str().is_none_or(str::is_empty)
        {
            failures.push(json!({"value": reference["id"], "rule": "an inspected image carries an asset hash"}));
        }
    }
    for source in &manifest.sources {
        if source.rights.trim().is_empty() {
            failures.push(json!({"value": source.id, "rule": "rights field is non-empty"}));
        }
    }
    failures
}
