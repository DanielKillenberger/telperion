//! Discovery proposes sources and admits the ones it may (fn-129).
//!
//! For every evidence field the manifest requires, the candidates are what
//! the repository already knows about this species for that field - its own
//! catalogue bibliography and the admitted manifests of the same species or
//! taxon (`pipeline::known`) - then the adapter's web search and
//! research index on a plain-word query. Jev ranks them per field and the
//! stage files a manifest-proposed decision carrying the draft manifest and
//! the ranking judgment behind every proposal. When the draft only adds
//! sources and every one passes the rights and relevance checks
//! (`pipeline::admission`), the stage writes the manifest and resolves the
//! decision `admit` by the pipeline; any other draft waits for the owner.
//! The stage keys on the seed (species, taxon, fields), not the whole
//! manifest, so admitting sources does not rerun it.
//!
//! A tertiary encyclopedic page (Wikipedia and its kind) is never proposed:
//! it is a lead, and the primary sources it cites are the candidates in its
//! place (`pipeline::leads`).
//!
//! What the repository knows comes first because a source it has already
//! verified is the cheapest evidence there is: the first ash run spent six
//! driver dispatches searching the web for a yield table a reference file in
//! the repository already named.

use std::collections::{BTreeMap, BTreeSet};

use serde_json::{json, Value};

use crate::pipeline::adapter::{FetchAdapter, SearchHit};
use crate::pipeline::admission::{self, Verdict};
use crate::pipeline::canon::canonical_sha256;
use crate::pipeline::cost::Cost;
use crate::pipeline::decision::{
    append_decisions, read_decisions, Decision, DecisionParts, Resolution, Status,
};
use crate::pipeline::judge::Judge;
use crate::pipeline::known::KnownSources;
use crate::pipeline::leads;
use crate::pipeline::manifest::{seed_sha256, Manifest, Source, MANIFEST_SCHEMA_VERSION};
use crate::pipeline::sets::ranking_questions;
use crate::pipeline::stage::{Context, Paths, StageError, STAGES};

use super::inputs;

pub const STAGE: &str = "discover";
const HITS_PER_QUERY: usize = 5;

#[derive(Debug)]
pub enum Outcome {
    Current,
    Ran { decisions: Vec<String> },
}

pub fn run(
    paths: &Paths,
    adapter: &dyn FetchAdapter,
    judge: &Judge<'_>,
    known: &KnownSources,
) -> Result<Outcome, StageError> {
    let (ctx, _) = Context::open(paths, STAGE)?;
    let manifest = &ctx.admitted.manifest;
    let seed = seed_sha256(manifest);
    let mut header = ctx.header_keyed(STAGE, "discover", inputs(&[]), vec![], &seed);
    if ctx.is_current(STAGE, &header.idempotence_key) {
        return Ok(Outcome::Current);
    }
    let before = adapter.spent();
    let mut proposals = Vec::new();
    let mut ledger = Vec::new();
    for field in &manifest.fields {
        let query = plain_query(
            &manifest.taxon.scientific_name,
            &field.field,
            &field.condition,
        );
        let mut hits = known_hits(known, &field.field);
        searched_hits(adapter, &query, &mut hits)?;
        let ranked = rank(
            judge,
            manifest,
            field.field.as_str(),
            &field.condition,
            &mut hits,
        )
        .map_err(|err| StageError::Failed {
            stage: STAGE.into(),
            reason: err.to_string(),
        })?;
        ledger.extend(ranked.iter().cloned());
        proposals
            .push(json!({"field": field.field, "query": query, "hits": hits, "ledger": ranked}));
    }
    let (draft_manifest, draft) = draft_manifest(manifest, &proposals);
    let draft_sha256 = canonical_sha256(&draft);
    let mut decision = Decision::new(
        DecisionParts {
            species: &manifest.species,
            stage: STAGE,
            kind: "manifest-proposed",
            field: None,
            age_years: None,
        },
        &STAGES[1..],
        [("seed".to_string(), seed)].into_iter().collect(),
        ledger.clone(),
        json!({
            "draft_sha256": draft_sha256,
            "proposals": proposals,
        }),
        &["admit", "reject", "skip"],
        "The pipeline admits a draft that only adds sources, each chosen for its field and carrying an open licence or a public cite-only page; any other draft a person admits, edited or not, by writing it to manifest.json and resolving this decision. The species runner skips a draft it cannot admit and goes on with the manifest as it stands. The resolution binds to the seed, so admitting sources keeps it.",
    );
    let id = decision.id.clone();
    let verdict = if already_resolved(&ctx.paths, &decision)? {
        None
    } else {
        let proposed = proposed_sources(manifest, &draft_manifest, &proposals);
        let verdict = admission::judge_draft(adapter, judge, manifest, &draft_manifest, &proposed)
            .map_err(|err| StageError::Failed {
                stage: STAGE.into(),
                reason: err.to_string(),
            })?;
        let asked = verdict.sources.iter().filter_map(|c| c.ledger.clone());
        ledger.extend(asked);
        decision.ledger = ledger.clone();
        decision.payload["admission"] = json!(verdict);
        Some(verdict)
    };
    append_decisions(&ctx.paths.decisions(), vec![decision.clone()])?;
    if let Some(verdict) = verdict.as_ref().filter(|v| v.admitted) {
        admit(&ctx.paths, &draft_manifest, verdict, &decision)?;
    }
    header.ledger = ledger;
    header.cost = Cost::from_spent(&adapter.spent().since(&before));
    ctx.write(
        &header,
        json!({"proposals": proposals, "draft_manifest": draft, "admission": verdict}),
    )?;
    Ok(Outcome::Ran {
        decisions: vec![id],
    })
}

/// True when the decision is already resolved under the same inputs: a
/// rerun keeps a person's or the pipeline's admission and admits nothing.
fn already_resolved(paths: &Paths, decision: &Decision) -> Result<bool, StageError> {
    Ok(read_decisions(&paths.decisions())?.iter().any(|d| {
        d.id == decision.id
            && d.status == Status::Resolved
            && d.inputs_sha256 == decision.inputs_sha256
    }))
}

/// Each source the draft adds, with the fields whose ranking chose it.
fn proposed_sources(
    manifest: &Manifest,
    draft: &Manifest,
    proposals: &[Value],
) -> Vec<(Source, Vec<String>)> {
    draft
        .sources
        .iter()
        .skip(manifest.sources.len())
        .map(|source| {
            let fields = proposals
                .iter()
                .filter(|p| {
                    p["hits"].as_array().into_iter().flatten().any(|hit| {
                        hit["ranked_first"] == json!(true) && hit["url"] == json!(source.url)
                    })
                })
                .filter_map(|p| p["field"].as_str().map(str::to_string))
                .collect();
            (source.clone(), fields)
        })
        .collect()
}

/// Writes the admitted manifest and resolves the proposal by the pipeline.
fn admit(
    paths: &Paths,
    draft: &Manifest,
    verdict: &Verdict,
    decision: &Decision,
) -> Result<(), StageError> {
    admission::write_manifest(
        &paths.manifest(),
        &admission::admitted_draft(draft, verdict),
    )?;
    let ids: Vec<&str> = verdict.sources.iter().map(|c| c.id.as_str()).collect();
    admission::record_resolution(
        &paths.resolutions(),
        &Resolution {
            id: decision.id.clone(),
            inputs_sha256: decision.inputs_sha256.clone(),
            option: "admit".into(),
            by: admission::BY.into(),
            at: crate::pipeline::stage::now(),
            note: format!(
                "admitted {}: each chosen for its field, each open-licence or public-cite-only",
                ids.join(", ")
            ),
            payload: Value::Null,
        },
    )?;
    Ok(())
}

/// The search query in plain words: the taxon, the field's reading and the
/// condition, as a person would type them, never the field id.
pub fn plain_query(taxon: &str, field: &str, condition: &str) -> String {
    format!(
        "{taxon} {}, {}",
        reading(field),
        condition.replace('_', " ")
    )
}

/// What a field id asks for, in words, without the age: `height`.
pub(crate) fn stem(field: &str) -> String {
    let words = reading(field);
    words
        .strip_suffix(" at age")
        .map_or(words.clone(), str::to_string)
}

/// What a field id asks for, in words. An id outside the table reads as its
/// words without the unit suffix.
fn reading(field: &str) -> String {
    match field {
        "height_m" => "height at age".into(),
        "dbh_m" => "trunk diameter at breast height at age".into(),
        "crown_width_m" => "crown width at age".into(),
        "height_growth_m_per_year" => "height growth rate per year".into(),
        other => {
            let stem = other
                .rsplit_once('_')
                .filter(|(_, unit)| matches!(*unit, "m" | "cm" | "mm" | "years"))
                .map_or(other, |(stem, _)| stem);
            format!("{} at age", stem.replace('_', " "))
        }
    }
}

/// The repository's known sources for `field`, first in the candidate list,
/// each with its origin and any fetch error its run recorded.
fn known_hits(known: &KnownSources, field: &str) -> Vec<Value> {
    known
        .for_field(field)
        .into_iter()
        .filter(|source| !leads::is_tertiary(&source.url))
        .enumerate()
        .map(|(index, source)| {
            let mut hit = hit_value(
                &SearchHit {
                    url: source.url.clone(),
                    title: source.title.clone(),
                    snippet: source.snippet.clone(),
                },
                "known",
                index,
            );
            hit["origin"] = json!(source.origin);
            if let Some(error) = &source.error {
                hit["error"] = json!(error);
            }
            hit
        })
        .collect()
}

/// Appends the web and research hits for `query` after the known ones, one
/// entry per URL.
fn searched_hits(
    adapter: &dyn FetchAdapter,
    query: &str,
    out: &mut Vec<Value>,
) -> Result<(), StageError> {
    let failed = |err: crate::pipeline::adapter::AdapterError| StageError::Failed {
        stage: STAGE.into(),
        reason: err.to_string(),
    };
    let mut seen: BTreeSet<String> = out
        .iter()
        .filter_map(|hit| hit["url"].as_str().map(str::to_string))
        .collect();
    // A tertiary page is a lead: its cited primary sources stand in for it.
    for (kind, list) in [
        (
            "web",
            leads::follow(
                adapter,
                adapter.search(query, HITS_PER_QUERY).map_err(failed)?,
            ),
        ),
        (
            "research",
            leads::follow(
                adapter,
                adapter.research(query, HITS_PER_QUERY).map_err(failed)?,
            ),
        ),
    ] {
        for hit in list {
            if seen.insert(hit.url.clone()) {
                out.push(hit_value(&hit, kind, out.len()));
            }
        }
    }
    Ok(())
}

pub(crate) fn hit_value(hit: &SearchHit, kind: &str, index: usize) -> Value {
    json!({
        "key": format!("h{}", index + 1),
        "url": hit.url,
        "title": hit.title,
        "snippet": hit.snippet,
        "kind": kind,
        "ranked_first": false,
    })
}

/// Asks Jev which candidate is the best evidence for the field; marks it.
pub(crate) fn rank(
    judge: &Judge<'_>,
    manifest: &Manifest,
    field: &str,
    condition: &str,
    hits: &mut [Value],
) -> Result<Vec<String>, crate::caller::CallerError> {
    if hits.is_empty() {
        return Ok(vec![]);
    }
    let keys: Vec<String> = hits
        .iter()
        .map(|h| h["key"].as_str().unwrap_or("").to_string())
        .collect();
    let state = json!({
        "requirement": {"taxon": manifest.taxon.scientific_name, "field": field, "condition": condition},
        "candidates": hits,
    });
    let judgment = judge.ask("ranking", None, &state, &ranking_questions(&keys))?;
    let choice = judgment
        .entry
        .choice("source")
        .unwrap_or_else(|| "none".into());
    for hit in hits.iter_mut() {
        if hit["key"] == choice {
            hit["ranked_first"] = json!(true);
        }
    }
    Ok(vec![judgment.reference])
}

/// The admitted manifest with every ranked-first hit appended as a proposed
/// source whose rights are still to be confirmed, and its value with the
/// proposed ids. A known hit that carries a fetch error is listed but never
/// proposed.
fn draft_manifest(manifest: &Manifest, proposals: &[Value]) -> (Manifest, Value) {
    let mut draft = manifest.clone();
    // A proposal is written at the current schema version, where the
    // requirements table binds its coverage at admission.
    draft.schema_version = MANIFEST_SCHEMA_VERSION;
    let mut next = draft.sources.len() + 1;
    for proposal in proposals {
        for hit in proposal["hits"].as_array().into_iter().flatten() {
            if hit["ranked_first"] != json!(true) || hit.get("error").is_some() {
                continue;
            }
            let url = hit["url"].as_str().unwrap_or("").to_string();
            if draft.sources.iter().any(|s| s.url == url) {
                continue;
            }
            draft.sources.push(Source {
                id: format!("P{next}"),
                url,
                title: hit["title"].as_str().unwrap_or("").to_string(),
                sha256: None,
                rights: "unstated: the rights are confirmed before admission".into(),
                rights_class: None,
                tables: vec![],
            });
            next += 1;
        }
    }
    let mut value = serde_json::to_value(&draft).expect("manifest serializes");
    let proposed: BTreeMap<String, String> = draft
        .sources
        .iter()
        .skip(manifest.sources.len())
        .map(|s| (s.id.clone(), s.url.clone()))
        .collect();
    value["proposed_sources"] = json!(proposed);
    (draft, value)
}

#[cfg(test)]
mod tests {
    use super::plain_query;

    #[test]
    fn the_query_reads_the_field_in_words_not_its_id() {
        assert_eq!(
            plain_query("Fraxinus excelsior", "height_m", "open_grown"),
            "Fraxinus excelsior height at age, open grown"
        );
        assert_eq!(
            plain_query("Fraxinus excelsior", "dbh_m", "stand_grown"),
            "Fraxinus excelsior trunk diameter at breast height at age, stand grown"
        );
        assert_eq!(
            plain_query("Picea abies", "crown_base_m", "open_grown"),
            "Picea abies crown base at age, open grown"
        );
    }
}
