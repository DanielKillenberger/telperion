//! Fetch and checksum every admitted source through the adapter.
//!
//! The stage records the final URL, the content type, the checksum of the raw
//! response and the checksum of the markdown it judges, saves a PDF locally
//! before parsing it, and yields the age-indexed rows of every admitted table
//! against the row count the manifest states, cut to the table's block when
//! the manifest names one. It consumes the resolutions of its own decisions:
//! `retry` fetches again, `drop-source` skips the source and records it as
//! dropped, `replace-source` fetches the resolution's url under the same
//! source id and records both urls; `accept-rows` keeps a table's rows as
//! parsed, `drop-table` records the table as dropped with no rows, and
//! `fix-table` reads the table entry the manifest now admits. Source bytes
//! stay in the cache directory, outside the repository.

use std::collections::BTreeMap;
use std::path::Path;

use serde_json::{json, Map, Value};

use crate::pipeline::adapter::{
    age_indexed_rows, block_rows, checksums, is_pdf, markdown_tables, AdapterError, FetchAdapter,
    Scrape,
};
use crate::pipeline::canon::{canonical_sha256, write_atomic};
use crate::pipeline::cost::Cost;
use crate::pipeline::decision::{
    append_decisions, decision_id, Decision, DecisionParts, Resolution,
};
use crate::pipeline::manifest::{AdmittedTable, Source};
use crate::pipeline::stage::{Context, Paths, StageError, STAGES};

use super::inputs;

pub const STAGE: &str = "fetch";

#[derive(Debug)]
pub enum Outcome {
    Current,
    Ran { decisions: Vec<String> },
}

pub fn run(paths: &Paths, adapter: &dyn FetchAdapter) -> Result<Outcome, StageError> {
    let (ctx, _) = Context::open(paths, STAGE)?;
    let resolutions = ctx.resolutions_sha256(STAGE);
    let mut header = ctx.header(
        STAGE,
        "sources",
        inputs(&[("resolutions:fetch", &resolutions)]),
        vec![],
    );
    if ctx.is_current(STAGE, &header.idempotence_key) {
        return Ok(Outcome::Current);
    }
    let before = adapter.spent();
    let manifest = &ctx.admitted.manifest;
    let species = manifest.species.as_str();
    let cache = ctx.paths.cache();
    let mut sources = Map::new();
    let mut dropped = Map::new();
    let mut tables = Map::new();
    let mut decisions = Vec::new();
    for source in &manifest.sources {
        let resolved = bound(
            &ctx,
            &decision_id(&unavailable_parts(species, source)),
            &unavailable_inputs(source),
        );
        let option = resolved.map(|r| r.option.as_str());
        if option == Some("drop-source") {
            dropped.insert(
                source.id.clone(),
                json!({"url": source.url, "option": "drop-source"}),
            );
            continue;
        }
        let replacement = (option == Some("replace-source")).then(|| {
            resolved
                .and_then(|r| r.payload["url"].as_str())
                .unwrap_or_default()
                .to_string()
        });
        let url = replacement.clone().unwrap_or_else(|| source.url.clone());
        let scrape = match adapter.scrape(&url) {
            Ok(scrape) => scrape,
            Err(err) => return Err(stop(&ctx, species, source, &err.to_string())),
        };
        let (mut record, markdown) = cache_source(adapter, &cache, source, scrape)?;
        match &replacement {
            Some(replacement) => {
                record["url"] = json!(replacement);
                record["replaced_url"] = json!(source.url);
            }
            None => {
                if let Some(recorded) = &source.sha256 {
                    if recorded != &record["raw_sha256"] {
                        let error = format!(
                            "checksum mismatch: manifest records {recorded}, fetched {}",
                            record["raw_sha256"]
                        );
                        return Err(stop(&ctx, species, source, &error));
                    }
                }
            }
        }
        for table in &source.tables {
            let (body, gap) = parse_table(&ctx, source, table, &markdown)?;
            tables.insert(table.id.clone(), body);
            decisions.extend(gap);
        }
        sources.insert(source.id.clone(), record);
    }
    let ids: Vec<String> = decisions.iter().map(|d| d.id.clone()).collect();
    if !decisions.is_empty() {
        append_decisions(&ctx.paths.decisions(), decisions)?;
    }
    header.cost = Cost::from_spent(&adapter.spent().since(&before));
    ctx.write(
        &header,
        json!({"sources": sources, "dropped": dropped, "tables": tables}),
    )?;
    Ok(Outcome::Ran { decisions: ids })
}

/// Files unavailable-source for `source` and names it in the stop.
fn stop(ctx: &Context, species: &str, source: &Source, error: &str) -> StageError {
    let decision = unavailable(species, source, error);
    let id = decision.id.clone();
    match append_decisions(&ctx.paths.decisions(), vec![decision]) {
        Ok(_) => StageError::Failed {
            stage: STAGE.into(),
            reason: format!("unavailable-source {id}: {error}"),
        },
        Err(err) => StageError::File(err),
    }
}

/// Writes the raw bytes and the markdown under the cache; a PDF is saved as
/// a file and parsed from that file.
fn cache_source(
    adapter: &dyn FetchAdapter,
    cache: &Path,
    source: &Source,
    mut scrape: Scrape,
) -> Result<(Value, String), StageError> {
    let pdf = is_pdf(&scrape.content_type, &scrape.final_url);
    let raw_path = cache.join(format!("{}.{}", source.id, if pdf { "pdf" } else { "raw" }));
    write_atomic(&raw_path, &scrape.raw)?;
    if pdf {
        scrape.markdown =
            adapter
                .parse_pdf(&raw_path)
                .map_err(|err: AdapterError| StageError::Failed {
                    stage: STAGE.into(),
                    reason: format!("{}: parse: {err}", source.id),
                })?;
    }
    let markdown_path = cache.join(format!("{}.md", source.id));
    write_atomic(&markdown_path, scrape.markdown.as_bytes())?;
    let mut record = serde_json::to_value(checksums(&scrape)).expect("record serializes");
    record["cached"] = json!({
        "raw": raw_path.file_name().map(|n| n.to_string_lossy().into_owned()),
        "markdown": markdown_path.file_name().map(|n| n.to_string_lossy().into_owned()),
    });
    Ok((record, scrape.markdown))
}

/// The age-indexed rows of one admitted table, cut to its block when the
/// manifest names one, and a coverage-gap decision when the count differs
/// from the one the manifest states in either direction or the block is not
/// there. A bound resolution changes the outcome as its option says.
fn parse_table(
    ctx: &Context,
    source: &Source,
    table: &AdmittedTable,
    markdown: &str,
) -> Result<(Value, Option<Decision>), StageError> {
    let species = ctx.admitted.manifest.species.as_str();
    let id = decision_id(&gap_parts(species, table));
    let inputs: BTreeMap<String, String> = [
        (
            "markdown".to_string(),
            crate::sha256_hex(markdown.as_bytes()),
        ),
        (
            "table".to_string(),
            canonical_sha256(&serde_json::to_value(table).expect("table serializes")),
        ),
    ]
    .into_iter()
    .collect();
    let option = bound(ctx, &id, &inputs).map(|r| r.option.as_str());
    let mut body = json!({
        "source": source.id,
        "dimension": table.dimension,
        "unit": table.unit,
        "condition": table.condition,
        "taxon": table.taxon,
        "expected_rows": table.expected_rows,
        "found_rows": 0,
        "rows": [],
    });
    if let Some(block) = &table.block {
        body["block"] = json!(block);
    }
    if option == Some("drop-table") {
        body["dropped"] = json!(true);
        return Ok((body, None));
    }
    let parsed = markdown_tables(markdown)
        .get(table.table_index)
        .map(|found| match &table.block {
            Some(label) => block_rows(found, label),
            None => Ok(age_indexed_rows(found)),
        });
    let gap = |payload: Value, note: &str| {
        Decision::new(
            gap_parts(species, table),
            &["fit"],
            inputs.clone(),
            vec![],
            payload,
            &["accept-rows", "fix-table", "drop-table"],
            note,
        )
    };
    let shared =
        json!({"source": source.id, "table": table.id, "expected_rows": table.expected_rows});
    let with = |mut payload: Value, extra: Value| {
        for (key, value) in extra.as_object().into_iter().flatten() {
            payload[key] = value.clone();
        }
        payload
    };
    let decision = match parsed {
        None => Some(gap(
            with(shared, json!({"found_rows": 0, "table_index": table.table_index})),
            "The markdown carries no table at the admitted index; no selection runs from a table that is not there.",
        )),
        Some(Err(labels)) => Some(gap(
            with(shared, json!({"found_rows": 0, "block": table.block, "labels_found": labels})),
            "The admitted block label is not in the parsed table; the labels the table carries are listed.",
        )),
        Some(Ok(rows)) => {
            let rows: Vec<Value> = rows
                .into_iter()
                .filter_map(|row| {
                    let value = row
                        .values
                        .get(table.value_column.saturating_sub(1))
                        .copied()
                        .flatten()?;
                    Some(json!({"age_years": row.age_years, "value": value}))
                })
                .collect();
            let found = rows.len();
            body["found_rows"] = json!(found);
            body["rows"] = json!(rows);
            let mismatch = found != table.expected_rows;
            if mismatch && option == Some("fix-table") {
                return Err(StageError::Failed {
                    stage: STAGE.into(),
                    reason: format!(
                        "coverage-gap {id}: resolved fix-table, but the table still yields {found} rows against {}",
                        table.expected_rows
                    ),
                });
            }
            if mismatch && option == Some("accept-rows") {
                body["accepted_rows"] = json!(true);
            }
            (mismatch && option != Some("accept-rows")).then(|| {
                gap(
                    with(shared, json!({"found_rows": found})),
                    "The parsed table yields a different count of age-indexed rows than the manifest states: fewer is a flattened table, more is a merged one; no selection runs from either.",
                )
            })
        }
    };
    Ok((body, decision))
}

fn gap_parts<'a>(species: &'a str, table: &'a AdmittedTable) -> DecisionParts<'a> {
    DecisionParts {
        species,
        stage: STAGE,
        kind: "coverage-gap",
        field: Some(&table.dimension),
        age_years: None,
    }
}

fn unavailable_parts<'a>(species: &'a str, source: &'a Source) -> DecisionParts<'a> {
    DecisionParts {
        species,
        stage: STAGE,
        kind: "unavailable-source",
        field: Some(&source.id),
        age_years: None,
    }
}

fn unavailable_inputs(source: &Source) -> BTreeMap<String, String> {
    [("url".to_string(), crate::sha256_hex(source.url.as_bytes()))]
        .into_iter()
        .collect()
}

/// The resolution to decision `id` that binds to the inputs this run
/// computes; one bound to earlier inputs no longer changes anything.
fn bound<'a>(
    ctx: &'a Context,
    id: &str,
    inputs: &BTreeMap<String, String>,
) -> Option<&'a Resolution> {
    ctx.resolved(id).filter(|r| &r.inputs_sha256 == inputs)
}

fn unavailable(species: &str, source: &Source, error: &str) -> Decision {
    Decision::new(
        unavailable_parts(species, source),
        &STAGES[2..],
        unavailable_inputs(source),
        vec![],
        json!({"source": source.id, "url": source.url, "error": error}),
        &["retry", "replace-source", "drop-source"],
        "The adapter could not fetch this source; nothing is fetched by another route in its place. A person resolves it: retry fetches again, replace-source fetches the url in the resolution's payload under this id, drop-source skips it.",
    )
}
