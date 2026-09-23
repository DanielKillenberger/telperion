//! The literature screen from fn-57 over extract.json's candidates: one row
//! per candidate sentence with its kind, growing condition, anchor
//! probability and ledger identity. Screen judges exactly what extract wrote;
//! it never re-extracts from the cached text.

use serde_json::{json, Value};

use crate::extract::CandidateSentence;
use crate::ledger::SourceRef;
use crate::pipeline::judge::Judge;
use crate::pipeline::stage::{Context, Paths, StageError};
use crate::screen::screen_candidates;

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
    let (extract, extract_sha) = body(&ctx, STAGE, "extract")?;
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
    for (id, candidates) in by_source(&extract) {
        let record = &fetch["sources"][&id];
        let source = SourceRef {
            id: id.clone(),
            url: record["final_url"].as_str().unwrap_or_default().to_string(),
            sha256: record["markdown_sha256"]
                .as_str()
                .unwrap_or_default()
                .to_string(),
            bytes: record["markdown_bytes"].as_u64().unwrap_or_default(),
        };
        let report = screen_candidates(
            judge.transport,
            judge.key,
            &judge.ledger_dir,
            &source,
            &candidates,
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

/// extract.json's candidates grouped by source, sources in first-seen order
/// and candidates in file order.
fn by_source(extract: &Value) -> Vec<(String, Vec<CandidateSentence>)> {
    let mut groups: Vec<(String, Vec<CandidateSentence>)> = Vec::new();
    for candidate in extract["candidates"].as_array().into_iter().flatten() {
        let id = candidate["source"].as_str().unwrap_or_default().to_string();
        let sentence = CandidateSentence {
            sentence: candidate["sentence"]
                .as_str()
                .unwrap_or_default()
                .to_string(),
            context: candidate["context"]
                .as_str()
                .unwrap_or_default()
                .to_string(),
        };
        match groups.iter_mut().find(|(seen, _)| *seen == id) {
            Some((_, list)) => list.push(sentence),
            None => groups.push((id, vec![sentence])),
        }
    }
    groups
}
