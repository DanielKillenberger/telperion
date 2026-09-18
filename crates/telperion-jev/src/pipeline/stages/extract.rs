//! Candidate extraction: every sentence with a length, age or rate unit,
//! from the cached markdown of every fetched source, with its context.

use std::path::Path;

use serde_json::{json, Value};

use crate::extract::{candidate_sentences, visible_text};
use crate::pipeline::canon::file_sha256;
use crate::pipeline::stage::{Context, StageError};

use super::{body, inputs};

pub const STAGE: &str = "extract";

#[derive(Debug)]
pub enum Outcome {
    Current,
    Ran { candidates: usize },
}

pub fn run(dir: &Path) -> Result<Outcome, StageError> {
    let (ctx, _) = Context::open(dir, STAGE)?;
    let (fetch, fetch_sha) = body(&ctx, STAGE, "fetch")?;
    let header = ctx.header(
        STAGE,
        "candidates",
        inputs(&[("fetch.json", &fetch_sha)]),
        vec![],
    );
    if ctx.is_current(STAGE, &header.idempotence_key) {
        return Ok(Outcome::Current);
    }
    let mut candidates = Vec::new();
    for (id, record) in fetch["sources"].as_object().into_iter().flatten() {
        let markdown = cached_markdown(&ctx, STAGE, id, record)?;
        let text = visible_text(markdown.as_bytes());
        for candidate in candidate_sentences(&text) {
            candidates.push(json!({
                "source": id,
                "sentence": candidate.sentence,
                "context": candidate.context,
            }));
        }
    }
    let count = candidates.len();
    ctx.write(&header, json!({"candidates": candidates}))?;
    Ok(Outcome::Ran { candidates: count })
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
