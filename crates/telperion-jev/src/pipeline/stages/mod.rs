//! The stages, in the fixed order `stage::STAGES` names them.
//!
//! Every artifact is `{header fields..., "body": {...}}` at `<dir>/<stage>.json`.
//! The bodies the later stages read:
//!
//! - `gather.body`: `{"queries": [...], "hits": [{"url", "title", "snippet", "kind"}]}`;
//!   the documents it found join the manifest's `sources`.
//! - `fetch.body`: `{"sources": {"<id>": FetchRecord + {"cached": {"raw", "markdown"},
//!   "licence": [statement]}}, "dropped": {"<id>": {"url", "error" | "option"}},
//!   "tables": {"<table id>": {"dimension", "unit", "condition", "taxon", "expected_rows",
//!   "found_rows", "rows": [{"age_years", "value"}]} with {"block"} when the manifest names
//!   one, and {"accepted_rows": true} or {"dropped": true} when a resolution said so}}`.
//! - `read.body`: `{"spans": [{"source", "sentence", "at", "span", "field", "basis",
//!   "age", "condition", "ledger"}]}`.
//! - `aggregate.body`: `{"filled": {"<json pointer>": metric}, "unavailable": {"<field>":
//!   reason}, "described": {"<trait>": {"level", "sentence", "ledger"}}}`, plus
//!   `"appearance": {"<trait>": {"level", "sentence", "ledger"}}` when the manifest
//!   lists appearance traits, beside the packet records under `packet/` and the
//!   sidecar `provenance.json`.
//! - `fit.body`: the curve module's `FitReport` plus `{"points": {...}}`.
//! - `gate.body`: `{"capability", "registry", "seeds", "unresolved": [...]}`.
//! - `generate.body`: `{"metrics", "described": {...}, "transfers": {...}, "stills": [...]}`,
//!   plus `"appearance": {"<trait>": note}` recording each appearance trait it skipped.
//! - `document.body`: `{"sources": [{"id", "form", "cached"}], "article",
//!   "article_validated", "article_work": [...], "claims": [CiteRow],
//!   "tokens": {"input", "output"}}`.

pub mod aggregate;
pub mod appearance;
pub mod capability_class;
pub mod document;
pub mod fetch;
pub mod fit;
pub mod gate;
pub mod gather;
pub mod generate;
pub mod read;

use std::collections::BTreeMap;

use serde_json::Value;

use crate::pipeline::canon::file_sha256;

use super::stage::{Context, StageError};

/// The input map every header records: artifact file name -> checksum.
pub fn inputs(pairs: &[(&str, &str)]) -> BTreeMap<String, String> {
    pairs
        .iter()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect()
}

/// Reads `<name>.json`, returning its body and the checksum of the file.
pub fn body(ctx: &Context, stage: &str, name: &str) -> Result<(Value, String), StageError> {
    let (value, sha) = ctx.input(stage, name)?;
    Ok((value["body"].clone(), sha))
}

/// The cached markdown of a fetched source, checked against the checksum
/// the fetch stage recorded for it.
pub fn cached_markdown(
    ctx: &Context,
    stage: &str,
    id: &str,
    record: &Value,
) -> Result<String, StageError> {
    let name = record["cached"]["markdown"]
        .as_str()
        .unwrap_or_default()
        .to_string();
    let path = ctx.paths.cache().join(&name);
    if !path.exists() {
        return Err(StageError::MissingInput {
            stage: stage.into(),
            path,
        });
    }
    let found = file_sha256(&path)?;
    let recorded = record["markdown_sha256"].as_str().unwrap_or_default();
    if found != recorded {
        return Err(StageError::ChecksumChanged {
            stage: stage.into(),
            path,
            recorded: recorded.into(),
            found,
        });
    }
    std::fs::read_to_string(&path).map_err(|err| StageError::Failed {
        stage: stage.into(),
        reason: format!("{id}: {err}"),
    })
}
