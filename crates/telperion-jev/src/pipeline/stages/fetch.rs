//! Fetch and checksum every admitted source through the adapter.
//!
//! The stage records the final URL, the content type, the checksum of the raw
//! response and the checksum of the markdown it judges, saves a PDF locally
//! before parsing it, and yields the age-indexed rows of every admitted table
//! against the row count the manifest states. Source bytes stay in the cache
//! directory, outside the repository.

use std::path::Path;

use serde_json::{json, Map, Value};

use crate::pipeline::adapter::{
    checksums, is_pdf, table_rows_for, AdapterError, FetchAdapter, Scrape,
};
use crate::pipeline::canon::write_atomic;
use crate::pipeline::decision::{append_decisions, Decision, DecisionParts};
use crate::pipeline::manifest::{AdmittedTable, Source};
use crate::pipeline::stage::{Context, StageError, STAGES};

use super::inputs;

pub const STAGE: &str = "fetch";

#[derive(Debug)]
pub enum Outcome {
    Current,
    Ran { decisions: Vec<String> },
}

pub fn run(dir: &Path, adapter: &dyn FetchAdapter) -> Result<Outcome, StageError> {
    let (ctx, _) = Context::open(dir, STAGE)?;
    let header = ctx.header(STAGE, "sources", inputs(&[]), vec![]);
    if ctx.is_current(STAGE, &header.idempotence_key) {
        return Ok(Outcome::Current);
    }
    let manifest = &ctx.admitted.manifest;
    let cache = ctx.paths.cache();
    let mut sources = Map::new();
    let mut tables = Map::new();
    let mut decisions = Vec::new();
    for source in &manifest.sources {
        let scrape = match adapter.scrape(&source.url) {
            Ok(scrape) => scrape,
            Err(err) => {
                let decision = unavailable(manifest.species.as_str(), source, &err.to_string());
                let id = decision.id.clone();
                append_decisions(&ctx.paths.decisions(), vec![decision])?;
                return Err(StageError::Failed {
                    stage: STAGE.into(),
                    reason: format!("unavailable-source {id}: {err}"),
                });
            }
        };
        let (record, markdown) = cache_source(adapter, &cache, source, scrape)?;
        if let Some(recorded) = &source.sha256 {
            if recorded != &record["raw_sha256"] {
                let error = format!(
                    "checksum mismatch: manifest records {recorded}, fetched {}",
                    record["raw_sha256"]
                );
                let decision = unavailable(manifest.species.as_str(), source, &error);
                let id = decision.id.clone();
                append_decisions(&ctx.paths.decisions(), vec![decision])?;
                return Err(StageError::Failed {
                    stage: STAGE.into(),
                    reason: format!("unavailable-source {id}: {error}"),
                });
            }
        }
        for table in &source.tables {
            let (body, gap) = parse_table(manifest.species.as_str(), source, table, &markdown);
            tables.insert(table.id.clone(), body);
            decisions.extend(gap);
        }
        sources.insert(source.id.clone(), record);
    }
    let ids: Vec<String> = decisions.iter().map(|d| d.id.clone()).collect();
    if !decisions.is_empty() {
        append_decisions(&ctx.paths.decisions(), decisions)?;
    }
    ctx.write(&header, json!({"sources": sources, "tables": tables}))?;
    Ok(Outcome::Ran { decisions: ids })
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

/// The age-indexed rows of one admitted table, and a coverage-gap decision
/// when fewer rows than the manifest states came out of the parse.
fn parse_table(
    species: &str,
    source: &Source,
    table: &AdmittedTable,
    markdown: &str,
) -> (Value, Option<Decision>) {
    let rows: Vec<Value> = table_rows_for(markdown, table.table_index)
        .unwrap_or_default()
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
    let body = json!({
        "source": source.id,
        "dimension": table.dimension,
        "unit": table.unit,
        "condition": table.condition,
        "taxon": table.taxon,
        "expected_rows": table.expected_rows,
        "found_rows": found,
        "rows": rows,
    });
    let gap = (found < table.expected_rows).then(|| {
        Decision::new(
            DecisionParts {
                species,
                stage: STAGE,
                kind: "coverage-gap",
                field: Some(&table.dimension),
                age_years: None,
            },
            &["fit"],
            [("markdown".to_string(), crate::sha256_hex(markdown.as_bytes()))]
                .into_iter()
                .collect(),
            vec![],
            json!({"source": source.id, "table": table.id, "expected_rows": table.expected_rows, "found_rows": found}),
            &["re-fetch", "lower-expected-rows", "reject-table"],
            "The parsed table yields fewer age-indexed rows than the manifest states; no selection runs from a flattened table.",
        )
    });
    (body, gap)
}

fn unavailable(species: &str, source: &Source, error: &str) -> Decision {
    Decision::new(
        DecisionParts {
            species,
            stage: STAGE,
            kind: "unavailable-source",
            field: Some(&source.id),
            age_years: None,
        },
        &STAGES[2..],
        [("url".to_string(), crate::sha256_hex(source.url.as_bytes()))]
            .into_iter()
            .collect(),
        vec![],
        json!({"source": source.id, "url": source.url, "error": error}),
        &["retry", "replace-source", "drop-source"],
        "The adapter could not fetch this source; nothing is fetched by another route in its place.",
    )
}
