//! Discovery proposes sources for a person to admit into the manifest.
//!
//! For every evidence field the manifest requires, every source the catalogue
//! already holds is listed first, then the adapter's web search and research
//! index add candidates, Jev ranks them per field, and the stage files a
//! manifest-proposed decision carrying the draft manifest and the ranking
//! judgment behind every proposal. Nothing is admitted here.
//!
//! The catalogue comes first because a source the repository has already
//! verified is the cheapest evidence there is: the first ash run spent six
//! driver dispatches searching the web for a yield table a reference file in
//! the repository already named.

use std::collections::BTreeMap;
use std::path::Path;

use serde_json::{json, Value};

use crate::pipeline::adapter::{FetchAdapter, SearchHit};
use crate::pipeline::canon::canonical_sha256;
use crate::pipeline::decision::{append_decisions, Decision, DecisionParts};
use crate::pipeline::judge::Judge;
use crate::pipeline::manifest::{Manifest, Source};
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
    catalogue: &Path,
    adapter: &dyn FetchAdapter,
    judge: &Judge<'_>,
) -> Result<Outcome, StageError> {
    let (ctx, _) = Context::open(paths, STAGE)?;
    let mut header = ctx.header(STAGE, "discover", inputs(&[]), vec![]);
    if ctx.is_current(STAGE, &header.idempotence_key) {
        return Ok(Outcome::Current);
    }
    let manifest = &ctx.admitted.manifest;
    let known = catalogue_hits(catalogue);
    let mut proposals = Vec::new();
    let mut ledger = Vec::new();
    for field in &manifest.fields {
        let query = format!(
            "{} {} {} by age",
            manifest.taxon.scientific_name, field.field, field.condition
        );
        let mut hits = hits_for(adapter, &query, &known)?;
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
    let draft = draft_manifest(manifest, &proposals);
    let draft_sha256 = canonical_sha256(&draft);
    let decision = Decision::new(
        DecisionParts {
            species: &manifest.species,
            stage: STAGE,
            kind: "manifest-proposed",
            field: None,
            age_years: None,
        },
        &STAGES[1..],
        [("draft".to_string(), draft_sha256.clone())].into_iter().collect(),
        ledger.clone(),
        json!({
            "draft_sha256": draft_sha256,
            "proposals": proposals,
        }),
        &["admit", "reject"],
        "A person admits the draft manifest, edited or not, by writing it to manifest.json and resolving this decision.",
    );
    let id = decision.id.clone();
    append_decisions(&ctx.paths.decisions(), vec![decision])?;
    header.ledger = ledger;
    ctx.write(
        &header,
        json!({"proposals": proposals, "draft_manifest": draft}),
    )?;
    Ok(Outcome::Ran {
        decisions: vec![id],
    })
}

/// Every source the catalogue already holds, one candidate each, read before
/// the adapter is asked. A catalogue that is absent or unreadable yields
/// nothing and the stage searches as it always did.
fn catalogue_hits(catalogue: &Path) -> Vec<SearchHit> {
    let Ok(entries) = std::fs::read_dir(catalogue) else {
        return vec![];
    };
    let mut folders: Vec<_> = entries.flatten().map(|entry| entry.path()).collect();
    folders.sort();
    let mut out = Vec::new();
    for folder in folders {
        let Ok(text) = std::fs::read_to_string(folder.join("sources.json")) else {
            continue;
        };
        let Ok(doc) = serde_json::from_str::<Value>(&text) else {
            continue;
        };
        let species = doc["species"].as_str().unwrap_or("unnamed species");
        for source in doc["sources"].as_array().into_iter().flatten() {
            let (Some(url), Some(title)) = (source["url"].as_str(), source["title"].as_str())
            else {
                continue;
            };
            out.push(SearchHit {
                url: url.to_string(),
                title: title.to_string(),
                snippet: format!(
                    "{species} catalogue source {}: {}",
                    source["id"].as_str().unwrap_or("?"),
                    source["use"].as_str().unwrap_or(""),
                ),
            });
        }
    }
    out
}

fn hits_for(
    adapter: &dyn FetchAdapter,
    query: &str,
    known: &[SearchHit],
) -> Result<Vec<Value>, StageError> {
    let failed = |err: crate::pipeline::adapter::AdapterError| StageError::Failed {
        stage: STAGE.into(),
        reason: err.to_string(),
    };
    let mut seen = std::collections::BTreeSet::new();
    let mut out = Vec::new();
    for hit in known {
        if seen.insert(hit.url.clone()) {
            out.push(hit_value(hit, "catalogue", out.len()));
        }
    }
    for (kind, list) in [
        (
            "web",
            adapter.search(query, HITS_PER_QUERY).map_err(failed)?,
        ),
        (
            "research",
            adapter.research(query, HITS_PER_QUERY).map_err(failed)?,
        ),
    ] {
        for hit in list {
            if seen.insert(hit.url.clone()) {
                out.push(hit_value(&hit, kind, out.len()));
            }
        }
    }
    Ok(out)
}

fn hit_value(hit: &SearchHit, kind: &str, index: usize) -> Value {
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
fn rank(
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
/// source whose rights a person confirms.
fn draft_manifest(manifest: &Manifest, proposals: &[Value]) -> Value {
    let mut draft = manifest.clone();
    let mut next = draft.sources.len() + 1;
    for proposal in proposals {
        for hit in proposal["hits"].as_array().into_iter().flatten() {
            if hit["ranked_first"] != json!(true) {
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
                rights: "unstated: a person confirms the rights before admission".into(),
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
    value
}
