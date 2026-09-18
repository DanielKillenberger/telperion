//! The literature screen from fn-57 over every fetched source: one row per
//! candidate sentence with its kind, growing condition, anchor probability
//! and ledger identity.

use serde_json::{json, Value};

use crate::ledger::SourceRef;
use crate::pipeline::judge::Judge;
use crate::pipeline::stage::{Context, Paths, StageError};
use crate::screen::screen;

use super::extract::cached_markdown;
use super::{body, inputs};

pub const STAGE: &str = "screen";

#[derive(Debug)]
pub enum Outcome {
    Current,
    Ran { rows: usize },
}

pub fn run(paths: &Paths, judge: &Judge<'_>) -> Result<Outcome, StageError> {
    let (ctx, _) = Context::open(paths, STAGE)?;
    let (fetch, fetch_sha) = body(&ctx, STAGE, "fetch")?;
    let (_, extract_sha) = body(&ctx, STAGE, "extract")?;
    let mut header = ctx.header(
        STAGE,
        "screen",
        inputs(&[("fetch.json", &fetch_sha), ("extract.json", &extract_sha)]),
        vec![],
    );
    if ctx.is_current(STAGE, &header.idempotence_key) {
        return Ok(Outcome::Current);
    }
    let species = ctx.admitted.manifest.taxon.common_name.clone();
    let mut rows: Vec<Value> = Vec::new();
    for (id, record) in fetch["sources"].as_object().into_iter().flatten() {
        let markdown = cached_markdown(&ctx, STAGE, id, record)?;
        let source = SourceRef {
            id: id.clone(),
            url: record["final_url"].as_str().unwrap_or_default().to_string(),
            sha256: record["markdown_sha256"]
                .as_str()
                .unwrap_or_default()
                .to_string(),
            bytes: markdown.len() as u64,
        };
        let report = screen(
            judge.transport,
            judge.key,
            &judge.ledger_dir,
            &source,
            markdown.as_bytes(),
            &species,
        )
        .map_err(|err| StageError::Failed {
            stage: STAGE.into(),
            reason: err.to_string(),
        })?;
        for row in report.rows {
            header.ledger.push(row.identity.clone());
            rows.push(json!({
                "source": id,
                "sentence": row.sentence,
                "kind": row.kind,
                "condition": row.condition,
                "anchor_usable": row.anchor_usable,
                "ledger": row.identity,
            }));
        }
    }
    let count = rows.len();
    ctx.write(&header, json!({"rows": rows}))?;
    Ok(Outcome::Ran { rows: count })
}
