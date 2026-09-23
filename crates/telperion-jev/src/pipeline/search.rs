//! A requirement unmet searches again (fn-129).
//!
//! `quality` files `requirements-unmet` for a required field below the
//! requirements table's bar. Its `add-sources` option is the pipeline's
//! first: one round searches the web and the research index with a query
//! aimed at the field's dominant gap, leaves out every URL the manifest
//! holds or an earlier round tried, and Jev ranks the rest. The chosen
//! source is admitted when its rights class admits (`pipeline::admission`);
//! the manifest gains it, the decision is resolved `add-sources` by the
//! pipeline, and the stages rerun. A field gets two rounds at most; after
//! them the decision is the owner's, with every source tried. An appearance
//! trait `select` found unstated is the owner's at once: a source for it
//! must be named on the trait, which is a change to the trait. No bar is
//! lowered here and no field is edited.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use super::adapter::FetchAdapter;
use super::admission::{self, Checked};
use super::canon::{read_json, write_canonical, CanonError};
use super::consume::REQUIREMENTS_UNMET;
use super::decision::{Decision, Resolution, Status};
use super::judge::Judge;
use super::manifest::{Manifest, Source};
use super::stage::{Context, Paths, StageError};
use super::stages::discover::{hit_value, plain_query, rank, stem};

pub const COMMAND: &str = "search-again";
/// Rounds a field gets before the owner has it.
pub const MAX_ROUNDS: usize = 2;
const HITS_PER_QUERY: usize = 5;

/// One round for one field: the query, what it found, what was tried and
/// what was admitted, or the error that stopped it.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Round {
    pub round: usize,
    pub decision: String,
    pub gap: String,
    pub query: String,
    pub hits: Vec<Value>,
    pub tried: Vec<Checked>,
    pub admitted: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    pub ledger: Vec<String>,
    pub at: String,
}

pub type Rounds = BTreeMap<String, Vec<Round>>;

pub fn read_rounds(paths: &Paths) -> Result<Rounds, CanonError> {
    let path = paths.search_rounds();
    if !path.exists() {
        return Ok(Rounds::new());
    }
    let value = read_json(&path)?;
    serde_json::from_value(value["fields"].clone()).map_err(|err| CanonError::Json {
        path,
        error: err.to_string(),
    })
}

fn write_rounds(paths: &Paths, rounds: &Rounds) -> Result<(), CanonError> {
    let value = json!({"schema": "search-rounds", "schema_version": 1, "fields": rounds});
    write_canonical(&paths.search_rounds(), &value).map(|_| ())
}

/// The open requirements-unmet decisions the pipeline still searches for:
/// filed by `quality` for a field with a round left. An unreadable rounds
/// file leaves nothing to search, so the owner has every decision.
pub fn searchable(paths: &Paths, decisions: &[Decision]) -> Vec<String> {
    let Ok(rounds) = read_rounds(paths) else {
        return Vec::new();
    };
    decisions
        .iter()
        .filter(|d| d.kind == REQUIREMENTS_UNMET && d.stage == "quality")
        .filter(|d| d.status == Status::Open)
        .filter(|d| {
            let field = d.field.as_deref().unwrap_or_default();
            rounds.get(field).map_or(0, Vec::len) < MAX_ROUNDS
        })
        .map(|d| d.id.clone())
        .collect()
}

/// Every URL the rounds for `field` tried, in order: what the owner is
/// handed when the rounds run out.
pub fn tried(rounds: &Rounds, field: &str) -> Vec<String> {
    rounds
        .get(field)
        .into_iter()
        .flatten()
        .flat_map(|round| round.tried.iter().map(|c| c.url.clone()))
        .collect()
}

/// The query aimed at the field's dominant gap, in plain words.
pub fn gap_query(taxon: &str, field: &str, condition: &str, gap: &str, ages: &[f64]) -> String {
    let (what, grown) = (stem(field), condition.replace('_', " "));
    match gap {
        "no_age_indexed_points" => format!("{taxon} {what} at stated ages in years, {grown}"),
        "age_range_uncovered" => {
            let ages: Vec<String> = ages.iter().map(f64::to_string).collect();
            format!("{taxon} {what} at {} years, {grown}", ages.join(", "))
        }
        "wrong_condition" => format!("{taxon} {what} by age of {grown} trees"),
        "wrong_taxon" => format!("\"{taxon}\" {what} at age, {grown}"),
        "no_mature_size" | "single_source" => format!("{taxon} mature {what}, typical range"),
        "bound_only" => format!("{taxon} typical mature {what}, not the maximum"),
        _ => plain_query(taxon, field, condition),
    }
}

#[derive(Debug)]
pub enum Outcome {
    /// No decision had a round left.
    Nothing,
    Ran {
        words: Vec<String>,
    },
}

/// One round for every decision with a round left. The manifest and the
/// resolutions are written once, after every round, when a source was
/// admitted; the rounds file always.
pub fn run(
    paths: &Paths,
    adapter: &dyn FetchAdapter,
    judge: &Judge<'_>,
) -> Result<Outcome, StageError> {
    let (ctx, _) = Context::open(paths, COMMAND)?;
    let proposal_open = ctx
        .decisions
        .iter()
        .any(|d| d.kind == "manifest-proposed" && d.status == Status::Open);
    if proposal_open {
        return Err(failed(
            "a manifest proposal is open; the owner admits it first",
        ));
    }
    let ids = searchable(paths, &ctx.decisions);
    if ids.is_empty() {
        return Ok(Outcome::Nothing);
    }
    let mut rounds = read_rounds(paths)?;
    let mut manifest = ctx.admitted.manifest.clone();
    let mut resolved = Vec::new();
    let mut words = Vec::new();
    for decision in ctx.decisions.iter().filter(|d| ids.contains(&d.id)) {
        let field = decision.field.clone().unwrap_or_default();
        let before = rounds.get(&field).map_or(0, Vec::len);
        let round = search_round(adapter, judge, &mut manifest, decision, &rounds);
        words.push(match round.admitted.first() {
            Some(id) => format!("{field}: admitted {id}"),
            None => format!(
                "{field}: nothing admitted (round {} of {MAX_ROUNDS})",
                before + 1
            ),
        });
        if !round.admitted.is_empty() {
            resolved.push((decision.clone(), round.admitted.clone()));
        }
        rounds.entry(field).or_default().push(round);
    }
    if !resolved.is_empty() {
        admission::write_manifest(&paths.manifest(), &manifest)?;
        for (decision, added) in resolved {
            admission::record_resolution(&paths.resolutions(), &resolution(&decision, &added))?;
        }
    }
    write_rounds(paths, &rounds)?;
    Ok(Outcome::Ran { words })
}

fn failed(reason: impl Into<String>) -> StageError {
    StageError::Failed {
        stage: COMMAND.into(),
        reason: reason.into(),
    }
}

/// One round for one decision. An adapter or Jev error ends the round with
/// its error recorded; it still counts, so a failing search cannot loop.
fn search_round(
    adapter: &dyn FetchAdapter,
    judge: &Judge<'_>,
    manifest: &mut Manifest,
    decision: &Decision,
    rounds: &Rounds,
) -> Round {
    let field = decision.field.clone().unwrap_or_default();
    let condition = manifest
        .field(&field)
        .map(|f| f.condition.clone())
        .unwrap_or_default();
    let gap = decision.payload["dominant_gap"]
        .as_str()
        .unwrap_or("none")
        .to_string();
    let ages: Vec<f64> = decision.payload["required_ages_uncovered"]
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(Value::as_f64)
        .collect();
    let taxon = manifest.taxon.scientific_name.clone();
    let query = gap_query(&taxon, &field, &condition, &gap, &ages);
    let mut round = Round {
        round: rounds.get(&field).map_or(0, Vec::len) + 1,
        decision: decision.id.clone(),
        gap,
        query: query.clone(),
        hits: Vec::new(),
        tried: Vec::new(),
        admitted: Vec::new(),
        error: None,
        ledger: Vec::new(),
        at: crate::pipeline::gap::now(),
    };
    let mut excluded: Vec<String> = manifest.sources.iter().map(|s| s.url.clone()).collect();
    excluded.extend(tried(rounds, &field));
    match candidates(adapter, &query, &excluded) {
        Ok(hits) => round.hits = hits,
        Err(err) => {
            round.error = Some(err);
            return round;
        }
    }
    match rank(judge, manifest, &field, &condition, &mut round.hits) {
        Ok(ranked) => round.ledger.extend(ranked),
        Err(err) => {
            round.error = Some(err.to_string());
            return round;
        }
    }
    let Some(chosen) = round.hits.iter().find(|h| h["ranked_first"] == json!(true)) else {
        return round;
    };
    let source = Source {
        id: next_id(manifest),
        url: chosen["url"].as_str().unwrap_or_default().to_string(),
        title: chosen["title"].as_str().unwrap_or_default().to_string(),
        sha256: None,
        rights: "unstated: the rights are confirmed before admission".into(),
        rights_class: None,
        tables: vec![],
    };
    let checked = match admission::classify(adapter, judge, &[(source.clone(), vec![field])]) {
        Ok(checked) => checked,
        Err(err) => {
            round.error = Some(err.to_string());
            return round;
        }
    };
    round
        .ledger
        .extend(checked.iter().filter_map(|c| c.ledger.clone()));
    if let Some(passing) = checked.iter().find(|c| c.passes()) {
        manifest
            .sources
            .push(admission::with_rights(&source, passing));
        round.admitted.push(source.id.clone());
    }
    round.tried = checked;
    round
}

/// The web and research hits for `query`, one per URL, none the manifest
/// holds or an earlier round tried. An adapter error is the round's error.
fn candidates(
    adapter: &dyn FetchAdapter,
    query: &str,
    excluded: &[String],
) -> Result<Vec<Value>, String> {
    let mut seen: Vec<String> = excluded.to_vec();
    let mut out = Vec::new();
    let web = adapter.search(query, HITS_PER_QUERY);
    let research = adapter.research(query, HITS_PER_QUERY);
    for (kind, list) in [("web", web), ("research", research)] {
        for hit in list.map_err(|err| err.to_string())? {
            if !seen.contains(&hit.url) {
                seen.push(hit.url.clone());
                out.push(hit_value(&hit, kind, out.len()));
            }
        }
    }
    Ok(out)
}

/// The first `P<n>` id the manifest does not hold.
fn next_id(manifest: &Manifest) -> String {
    (manifest.sources.len() + 1..)
        .map(|n| format!("P{n}"))
        .find(|id| manifest.source(id).is_none())
        .expect("an unused id")
}

fn resolution(decision: &Decision, added: &[String]) -> Resolution {
    Resolution {
        id: decision.id.clone(),
        inputs_sha256: decision.inputs_sha256.clone(),
        option: "add-sources".into(),
        by: admission::BY.into(),
        at: crate::pipeline::gap::now(),
        note: format!(
            "the pipeline searched again for the field's gap and admitted {}",
            added.join(", ")
        ),
        payload: Value::Null,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_query_aims_at_the_dominant_gap() {
        let q = |gap: &str, ages: &[f64]| {
            gap_query("Phoenix dactylifera", "height_m", "open_grown", gap, ages)
        };
        let table = [
            (
                "no_age_indexed_points",
                "Phoenix dactylifera height at stated ages in years, open grown",
            ),
            (
                "wrong_condition",
                "Phoenix dactylifera height by age of open grown trees",
            ),
            (
                "wrong_taxon",
                "\"Phoenix dactylifera\" height at age, open grown",
            ),
            ("none", "Phoenix dactylifera height at age, open grown"),
        ];
        for (gap, expect) in table {
            assert_eq!(q(gap, &[]), expect, "{gap}");
        }
        assert_eq!(
            q("age_range_uncovered", &[10.0, 25.0]),
            "Phoenix dactylifera height at 10, 25 years, open grown"
        );
        assert_eq!(
            gap_query(
                "Phoenix dactylifera",
                "frond_length_m",
                "open_grown",
                "bound_only",
                &[]
            ),
            "Phoenix dactylifera typical mature frond length, not the maximum"
        );
    }
}
