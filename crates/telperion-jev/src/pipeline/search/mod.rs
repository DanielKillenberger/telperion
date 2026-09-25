//! A requirement unmet searches again (fn-129).
//!
//! `quality` files `requirements-unmet` for a required field below the
//! requirements table's bar. Its `add-sources` option is the pipeline's
//! first: one round searches the web and the research index with a query
//! aimed at the field's dominant gap, leaves out every URL the manifest
//! holds or an earlier round tried, and Jev ranks the rest. The chosen
//! source is admitted when its rights class admits (`pipeline::admission`);
//! the manifest gains it, the decision is resolved `add-sources` by the
//! pipeline, and the stages rerun. An appearance trait `select` found
//! unstated is searched the same way (host design, 2026-09-23): an admitted
//! source not yet on the trait is a candidate too, and the chosen source's
//! id joins the trait's `sources` list, a sources-only change. A field or
//! trait gets two rounds at most; after them the decision is the owner's,
//! with every source tried. No bar is lowered, no field is edited, and a
//! trait's name and level table are never touched.

mod rounds;

use serde_json::{json, Value};

use super::adapter::{FetchAdapter, SearchHit};
use super::admission;
use super::decision::{Decision, Resolution, Status};
use super::judge::Judge;
use super::manifest::{Manifest, Source};
use super::stage::{Context, Paths, StageError};
use super::stages::discover::{hit_value, rank};

pub use rounds::{gap_query, read_rounds, searchable, tried, Round, Rounds, MAX_ROUNDS};

pub const COMMAND: &str = "search-again";
const HITS_PER_QUERY: usize = 5;

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
    rounds::write_rounds(paths, &rounds)?;
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
    // Select files a field it could not fill too (fn-131): a trait is one
    // the manifest lists as appearance.
    let on_trait = manifest.appearance.iter().any(|t| t.trait_name == field);
    let condition = manifest
        .field(&field)
        .map(|f| f.condition.clone())
        .unwrap_or_default();
    let gap = if on_trait {
        "unstated"
    } else {
        decision.payload["dominant_gap"].as_str().unwrap_or("none")
    }
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
        at: crate::pipeline::stage::now(),
    };
    let on_list = trait_sources(manifest, &field);
    let mut hits: Vec<(&str, SearchHit)> = if on_trait {
        manifest
            .sources
            .iter()
            .filter(|s| !on_list.contains(&s.id))
            .map(|s| {
                (
                    "admitted",
                    SearchHit {
                        url: s.url.clone(),
                        title: s.title.clone(),
                        snippet: format!("Admitted as {}.", s.id),
                    },
                )
            })
            .collect()
    } else {
        Vec::new()
    };
    let mut excluded: Vec<String> = manifest.sources.iter().map(|s| s.url.clone()).collect();
    excluded.extend(tried(rounds, &field));
    match searched(adapter, &query, &excluded) {
        Ok(found) => hits.extend(found),
        Err(err) => {
            round.error = Some(err);
            return round;
        }
    }
    round.hits = hits
        .iter()
        .enumerate()
        .map(|(i, (kind, hit))| hit_value(hit, kind, i))
        .collect();
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
    let url = chosen["url"].as_str().unwrap_or_default().to_string();
    if let Some(known) = manifest.sources.iter().find(|s| s.url == url) {
        // Already admitted: its id joins the trait's list with no rights call.
        let id = known.id.clone();
        add_to_trait(manifest, &field, &id);
        round.admitted.push(id);
        return round;
    }
    let source = Source {
        id: next_id(manifest),
        url,
        title: chosen["title"].as_str().unwrap_or_default().to_string(),
        sha256: None,
        rights: "unstated: the rights are confirmed before admission".into(),
        rights_class: None,
        tables: vec![],
    };
    let checked =
        match admission::classify(adapter, judge, &[(source.clone(), vec![field.clone()])]) {
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
        if on_trait {
            add_to_trait(manifest, &field, &source.id);
        }
        round.admitted.push(source.id.clone());
    }
    round.tried = checked;
    round
}

/// The source ids an appearance trait names; empty for a field.
fn trait_sources(manifest: &Manifest, name: &str) -> Vec<String> {
    manifest
        .appearance
        .iter()
        .find(|t| t.trait_name == name)
        .map(|t| t.sources.clone())
        .unwrap_or_default()
}

/// Appends `id` to the trait's `sources` list, the one trait edit the
/// pipeline makes.
fn add_to_trait(manifest: &mut Manifest, name: &str, id: &str) {
    if let Some(t) = manifest
        .appearance
        .iter_mut()
        .find(|t| t.trait_name == name)
    {
        if !t.sources.iter().any(|s| s == id) {
            t.sources.push(id.to_string());
        }
    }
}

/// The web and research hits for `query`, one per URL, none the manifest
/// holds or an earlier round tried. An adapter error is the round's error.
fn searched(
    adapter: &dyn FetchAdapter,
    query: &str,
    excluded: &[String],
) -> Result<Vec<(&'static str, SearchHit)>, String> {
    let mut seen: Vec<String> = excluded.to_vec();
    let mut out = Vec::new();
    let web = adapter.search(query, HITS_PER_QUERY);
    let research = adapter.research(query, HITS_PER_QUERY);
    for (kind, list) in [("web", web), ("research", research)] {
        for hit in list.map_err(|err| err.to_string())? {
            if !seen.contains(&hit.url) {
                seen.push(hit.url.clone());
                out.push((kind, hit));
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
        at: crate::pipeline::stage::now(),
        note: format!(
            "the pipeline searched again for the field's gap and admitted {}",
            added.join(", ")
        ),
        payload: Value::Null,
    }
}
