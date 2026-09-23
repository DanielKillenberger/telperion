//! The stages, in the fixed order `stage::STAGES` names them.
//!
//! Every artifact is `{header fields..., "body": {...}}` at `<dir>/<stage>.json`.
//! The bodies the later stages read:
//!
//! - `discover.body`: `{"proposals": [{"field", "hits": [SearchHit + "kind"]}],
//!   "draft_manifest": Manifest}`.
//! - `fetch.body`: `{"sources": {"<id>": FetchRecord + {"cached": {"raw", "markdown"}}
//!   with {"url", "replaced_url"} when replaced}, "dropped": {"<id>": {"url", "option"}},
//!   "tables": {"<table id>": {"dimension", "unit", "condition", "taxon", "expected_rows",
//!   "found_rows", "rows": [{"age_years", "value"}]} with {"block"} when the manifest names
//!   one, and {"accepted_rows": true} or {"dropped": true} when a resolution said so}}`.
//! - `extract.body`: `{"candidates": [{"source", "sentence", "context"}]}`.
//! - `screen.body`: `{"rows": [{"source", "sentence", "kind", "condition",
//!   "anchor_usable", "ledger"}]}`.
//! - `quality.body`: `{"fields": {"<field>": {"level", "dominant_gap", "points": [...],
//!   "required_ages_covered", "required_ages_uncovered", "bar", "passed", "ledger"}}}`.
//! - `select.body`: `{"filled": {"<json pointer>": value}, "unavailable": {"<field>": reason},
//!   "described": {"<trait>": {"level", "sentence", "ledger"}}}`, plus
//!   `"appearance": {"<trait>": {"level", "sentence", "ledger"}}` when the manifest
//!   lists appearance traits, beside the packet
//!   records under `packet/` and the sidecar `provenance.json`.
//! - `verify.body`: `{"claims": [CiteRow], "obligations": [...], "structural": [...]}`.
//! - `fit.body`: the curve module's `FitReport` plus `{"points": {...}}`.
//! - `gate.body`: `{"capability", "registry", "seeds", "unresolved": [...]}`.
//! - `generate.body`: `{"metrics", "described": {...}, "transfers": {...}, "stills": [...]}`,
//!   plus `"appearance": {"<trait>": note}` recording each appearance trait it skipped.
//! - `document.body`: `{"sources": [{"id", "form", "cached"}], "article",
//!   "article_validated", "article_work": [...], "claims": [CiteRow],
//!   "tokens": {"input", "output"}}`.
//! - `report.body`: the report's sections, also rendered to `report.md`, with
//!   `costs.stages.<stage>` and `costs.total` summed from every artifact's `cost`.

pub mod appearance;
pub mod discover;
pub mod document;
pub mod extract;
pub mod fetch;
pub mod fit;
pub mod gate;
pub mod generate;
pub mod quality;
pub mod report;
pub mod screen;
pub mod select;
pub mod verify;

use std::collections::BTreeMap;

use serde_json::Value;

use super::stage::Context;

/// The input map every header records: artifact file name -> checksum.
pub fn inputs(pairs: &[(&str, &str)]) -> BTreeMap<String, String> {
    pairs
        .iter()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect()
}

/// Reads `<name>.json`, returning its body and the checksum of the file.
pub fn body(
    ctx: &Context,
    stage: &str,
    name: &str,
) -> Result<(Value, String), super::stage::StageError> {
    let (value, sha) = ctx.input(stage, name)?;
    Ok((value["body"].clone(), sha))
}
