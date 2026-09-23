//! Packet verification: fn-57's citation check over every filled value
//! against its source, the semantic obligation questions, and the structural
//! obligations in code. Contradicted, unsupported and unmet items become
//! decisions of their kinds. A measured value is asked whether it is a
//! measurement; an appearance value is a level read from one sentence, so it
//! is asked whether that sentence describes the level instead (fn-128), and
//! a sentence that does not files a claim decision. Every claim decision is
//! keyed by its value's pointer and acts: select consumes `drop-value` and
//! `replace-source` (fn-131).

use serde_json::{json, Value};

use crate::cite::{cite, ResearchClaim, SourceLoad};
use crate::pipeline::canon::read_json;
use crate::pipeline::decision::{append_decisions, retire_unfiled, Decision, DecisionParts};
use crate::pipeline::judge::Judge;
use crate::pipeline::requirements::table;
use crate::pipeline::sets::{appearance_state, measurement_state, obligation_questions};
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

    let (claims, loads, pointers) = claims_for(&ctx, &sidecar, &fetch)?;
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
    for (pointer, row) in pointers.iter().zip(report.rows.iter()) {
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
            // One decision per value, keyed by its pointer (fn-131): two
            // values of one source are two claims.
            let entry = &sidecar["entries"][pointer];
            decisions.push(Decision::new(
                DecisionParts { species, stage: STAGE, kind, field: Some(pointer), age_years: None },
                &["generate"],
                [("select.json".to_string(), select_sha.clone())].into_iter().collect(),
                vec![row.identity.clone()],
                json!({"claim": row.claim, "section": row.section, "relation": row.relation, "reason": row.reason,
                       "pointer": pointer, "source": entry["source"], "span": entry["span"]}),
                &CLAIM_OPTIONS,
                "The citation check listed this claim. drop-value takes the value out of the packet and files its requirement again; replace-source files it for the pipeline's search; accept keeps it.",
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
        let checked = if entry["route"] == "appearance" {
            Some(supported(judge, pointer, entry)?)
        } else {
            measured(judge, &ctx, &fetch, pointer, entry)?
        };
        let Some((obligation, held, reference)) = checked else {
            obligations.push(json!({"obligation": "measurement_not_invention", "pointer": pointer, "held": Value::Null, "unchecked": "the value's sentence is not in its cached source"}));
            continue;
        };
        header.ledger.push(reference.clone());
        obligations.push(json!({"obligation": obligation, "pointer": pointer, "held": held, "ledger": reference}));
        if held {
            continue;
        }
        decisions.push(if obligation == SUPPORTED {
            unsupported(species, pointer, entry, &reference, &select_sha)
        } else {
            unmet(species, obligation, pointer, &reference, &select_sha)
        });
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
    // The citation check and the support question may both find one
    // appearance value unsupported: one decision per pointer.
    let mut seen = std::collections::BTreeSet::new();
    decisions.retain(|d| seen.insert(d.id.clone()));
    let ids: Vec<String> = decisions.iter().map(|d| d.id.clone()).collect();
    if !decisions.is_empty() {
        append_decisions(&ctx.paths.decisions(), decisions)?;
    }
    // A value this rerun found supported or measured files nothing: its
    // earlier decision is stale.
    retire_unfiled(
        &ctx.paths.decisions(),
        STAGE,
        &ids,
        &header.inputs,
        &crate::pipeline::gap::now(),
    )?;
    ctx.write(
        &header,
        json!({"claims": rows, "obligations": obligations, "structural": structural}),
    )?;
    Ok(Outcome::Ran { decisions: ids })
}

/// The obligation an appearance value is asked in place of a measurement's.
const SUPPORTED: &str = "appearance_supported";
/// A claim decision's options, each consumed by select (fn-131).
const CLAIM_OPTIONS: [&str; 3] = ["accept", "replace-source", "drop-value"];

/// One judged obligation: its name, whether it held, and the ledger reference.
type Checked = (&'static str, bool, String);

fn noul(judge: &Judge<'_>, name: &'static str, state: &Value) -> Result<Checked, StageError> {
    let judgment = judge
        .ask(
            &format!("obligation:{name}"),
            None,
            state,
            &obligation_questions(name),
        )
        .map_err(|err| StageError::Failed {
            stage: STAGE.into(),
            reason: err.to_string(),
        })?;
    let held = judgment.entry.noul(name).unwrap_or(0.0) >= thresholds().obligation_cut;
    Ok((name, held, judgment.reference))
}

/// A measured value, named by its field, against the cached source text
/// around its sentence; None when the source is not cached or does not
/// hold the sentence, since the span is never its own evidence.
fn measured(
    judge: &Judge<'_>,
    ctx: &Context,
    fetch: &Value,
    pointer: &str,
    entry: &Value,
) -> Result<Option<Checked>, StageError> {
    let span = entry["span"].as_str().unwrap_or_default();
    let source_id = entry["source"].as_str().unwrap_or_default();
    let record = &fetch["sources"][source_id];
    let Some(excerpt) = excerpt_for(ctx, source_id, record, entry) else {
        return Ok(None);
    };
    let field = pointer.rsplit('/').next();
    noul(
        judge,
        "measurement_not_invention",
        &measurement_state(field, span, &excerpt),
    )
    .map(Some)
}

/// An appearance value: does its cited sentence describe its level?
fn supported(judge: &Judge<'_>, pointer: &str, entry: &Value) -> Result<Checked, StageError> {
    let trait_name = pointer.rsplit('/').next().unwrap_or_default();
    let state = appearance_state(
        trait_name,
        entry["level"].as_str().unwrap_or_default(),
        entry["span"].as_str().unwrap_or_default(),
    );
    noul(judge, SUPPORTED, &state)
}

/// A claim decision on an appearance value whose sentence does not describe
/// its level.
fn unsupported(
    species: &str,
    pointer: &str,
    entry: &Value,
    ledger: &str,
    select_sha: &str,
) -> Decision {
    Decision::new(
        DecisionParts {
            species,
            stage: STAGE,
            kind: "claim-unsupported",
            field: Some(pointer),
            age_years: None,
        },
        &["generate"],
        [("select.json".to_string(), select_sha.to_string())]
            .into_iter()
            .collect(),
        vec![ledger.to_string()],
        json!({"value": pointer, "level": entry["level"], "sentence": entry["span"], "source": entry["source"],
               "pointer": pointer, "span": entry["span"]}),
        &CLAIM_OPTIONS,
        "Jev judged that the cited sentence does not describe this appearance level.",
    )
}

/// One claim per filled value, with its pointer, checked against the
/// cached markdown of its source: a measured value's copied span, and an
/// appearance value's level as the table words it beside the sentence it
/// was read from (fn-131), since the sentence alone always supports itself. An uncached source lists its
/// claim as unchecked.
type Claims = (Vec<ResearchClaim>, Vec<SourceLoad>, Vec<String>);

fn claims_for(ctx: &Context, sidecar: &Value, fetch: &Value) -> Result<Claims, StageError> {
    let mut claims = Vec::new();
    let mut loads = Vec::new();
    let mut pointers = Vec::new();
    for (pointer, entry) in sidecar["entries"].as_object().into_iter().flatten() {
        let source_id = entry["source"].as_str().unwrap_or_default().to_string();
        let record = &fetch["sources"][&source_id];
        let name = pointer.rsplit('/').next().unwrap_or_default();
        let stated = if entry["route"] == "appearance" {
            let summary = table()
                .level(name, entry["level"].as_str().unwrap_or_default())
                .map_or_else(String::new, |l| l.summary.clone());
            format!(
                "{summary}, as read from: {}",
                entry["span"].as_str().unwrap_or_default()
            )
        } else {
            entry["span"].as_str().unwrap_or_default().to_string()
        };
        pointers.push(pointer.clone());
        claims.push(ResearchClaim {
            claim: format!("{name}: {stated}"),
            url: record["final_url"].as_str().unwrap_or_default().to_string(),
            source_id: source_id.clone(),
            unresolved: None,
        });
        loads.push(match cached_markdown(ctx, STAGE, &source_id, record) {
            Ok(markdown) => SourceLoad::Bytes(markdown.into_bytes()),
            Err(err) => SourceLoad::Unreachable(err.to_string()),
        });
    }
    Ok((claims, loads, pointers))
}

/// The cached source text around the value's sentence (or, lacking one,
/// its span), whitespace collapsed, bounded to a few hundred characters
/// each side; None when the source is not cached or does not hold it
/// (fn-131): the value's evidence is its source, never the document's
/// first page and never the span itself.
fn excerpt_for(ctx: &Context, source_id: &str, record: &Value, entry: &Value) -> Option<String> {
    let markdown = cached_markdown(ctx, STAGE, source_id, record).ok()?;
    let text = markdown.split_whitespace().collect::<Vec<_>>().join(" ");
    let wanted = entry["sentence"]
        .as_str()
        .or_else(|| entry["span"].as_str())
        .unwrap_or_default();
    let wanted = wanted.split_whitespace().collect::<Vec<_>>().join(" ");
    if wanted.is_empty() {
        return None;
    }
    let at = text.find(&wanted)?;
    let start = text[..at]
        .char_indices()
        .rev()
        .nth(600)
        .map_or(0, |(i, _)| i);
    let end = text[at..]
        .char_indices()
        .nth(wanted.chars().count() + 600)
        .map_or(text.len(), |(i, _)| at + i);
    Some(text[start..end].to_string())
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
