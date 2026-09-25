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
//!
//! A scrape's markdown that is not the source (`adapter::content`) is
//! replaced by the raw body's own conversion, recorded as `markdown_from:
//! raw` with the reason it was refused. A source neither route can read, a
//! PDF that does not parse, an adapter error or a checksum mismatch files
//! unavailable-source for that source, and the rest are still fetched.

use std::collections::BTreeMap;
use std::path::Path;

use serde_json::{json, Map, Value};

use crate::pipeline::adapter::content::readable;
use crate::pipeline::adapter::{
    age_indexed_rows, block_rows, checksums, is_pdf, markdown_tables, FetchAdapter, Scrape,
};
use crate::pipeline::canon::{canonical_sha256, write_atomic};
use crate::pipeline::cost::Cost;
use crate::pipeline::decision::{
    append_decisions, decision_id, Decision, DecisionParts, Resolution,
};
use crate::pipeline::leads;
use crate::pipeline::manifest::{AdmittedTable, Source};
use crate::pipeline::stage::{Context, Paths, StageError};

use super::{inputs, unavailable};

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
    let mut sources = Map::new();
    let mut dropped = Map::new();
    let mut tables = Map::new();
    let mut decisions = Vec::new();
    for source in &manifest.sources {
        // A tertiary page is a lead, never a citation: nothing is read from it.
        if leads::is_tertiary(&source.url) {
            dropped.insert(
                source.id.clone(),
                json!({"url": source.url, "option": "tertiary: a lead, never a citation"}),
            );
            continue;
        }
        let resolved = unavailable::bound(&ctx, species, source);
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
        let (mut record, markdown) = match read_source(&ctx, adapter, source, &replacement)? {
            Ok(read) => read,
            Err(error) => {
                decisions.push(unavailable::decision(&ctx, species, source, &error)?);
                continue;
            }
        };
        if let Some(replacement) = &replacement {
            record["url"] = json!(replacement);
            record["replaced_url"] = json!(source.url);
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

/// What one source yields: its record and the text the stages read, or why
/// it could not be read, which files unavailable-source.
type Read = Result<(Value, String), String>;

/// One source fetched, checked and cached. The outer error is a file the
/// stage could not write.
fn read_source(
    ctx: &Context,
    adapter: &dyn FetchAdapter,
    source: &Source,
    replacement: &Option<String>,
) -> Result<Read, StageError> {
    let url = replacement.as_deref().unwrap_or(&source.url);
    let scrape = match adapter.scrape(url) {
        Ok(scrape) => scrape,
        Err(err) => return Ok(Err(err.to_string())),
    };
    if let (None, Some(recorded)) = (replacement, &source.sha256) {
        let fetched = crate::sha256_hex(&scrape.raw);
        if recorded != &fetched {
            return Ok(Err(format!(
                "checksum mismatch: manifest records {recorded}, fetched {fetched}"
            )));
        }
    }
    cache_source(adapter, &ctx.paths.cache(), source, scrape)
}

/// Writes the raw bytes under the cache, a PDF as a file parsed from that
/// file, then the text the stages read: the scrape's markdown when it is the
/// source, else the raw body's conversion, recorded with why the markdown
/// was refused.
fn cache_source(
    adapter: &dyn FetchAdapter,
    cache: &Path,
    source: &Source,
    mut scrape: Scrape,
) -> Result<Read, StageError> {
    let pdf = is_pdf(&scrape.content_type, &scrape.final_url);
    let raw_path = cache.join(format!("{}.{}", source.id, if pdf { "pdf" } else { "raw" }));
    write_atomic(&raw_path, &scrape.raw)?;
    if pdf {
        match adapter.parse_pdf(&raw_path) {
            Ok(markdown) => scrape.markdown = markdown,
            Err(err) => return Ok(Err(format!("parse: {err}"))),
        }
    }
    let readable = match readable(&scrape.markdown, &scrape.raw, pdf) {
        Ok(readable) => readable,
        Err(error) => return Ok(Err(error)),
    };
    scrape.markdown = readable.markdown;
    let markdown_path = cache.join(format!("{}.md", source.id));
    write_atomic(&markdown_path, scrape.markdown.as_bytes())?;
    let mut record = serde_json::to_value(checksums(&scrape)).expect("record serializes");
    record["cached"] = json!({
        "raw": raw_path.file_name().map(|n| n.to_string_lossy().into_owned()),
        "markdown": markdown_path.file_name().map(|n| n.to_string_lossy().into_owned()),
    });
    if let Some(refused) = readable.refused {
        record["markdown_from"] = json!("raw");
        record["refused"] = json!(refused);
    }
    Ok(Ok((record, scrape.markdown)))
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

/// The resolution to decision `id` that binds to the inputs this run
/// computes; one bound to earlier inputs no longer changes anything.
fn bound<'a>(
    ctx: &'a Context,
    id: &str,
    inputs: &BTreeMap<String, String>,
) -> Option<&'a Resolution> {
    ctx.resolved(id).filter(|r| &r.inputs_sha256 == inputs)
}
