//! The fetch stage's unavailable-source decision: one per source the fetch
//! could not read, filed while the other sources are still fetched.
//!
//! A resolution that sent the fetch back to the source (`retry`, or
//! `replace-source` with a url that fails too) and failed again reopens the
//! decision: the refiled decision's inputs carry the checksum of that
//! resolution, so the one that was spent no longer binds and a person
//! resolves the decision again. Filed again with no new resolution, the
//! inputs are the same and nothing changes.

use std::collections::BTreeMap;

use serde_json::json;

use crate::pipeline::canon::canonical_sha256;
use crate::pipeline::decision::{
    decision_id, read_resolutions, Decision, DecisionParts, Resolution,
};
use crate::pipeline::manifest::Source;
use crate::pipeline::stage::{Context, StageError, STAGES};

use super::fetch::STAGE;

const KIND: &str = "unavailable-source";

pub fn id(species: &str, source: &Source) -> String {
    decision_id(&parts(species, source))
}

/// The bound resolution for `source`: one a person wrote against the
/// decision as filed, for the url the manifest admits now.
pub fn bound<'a>(ctx: &'a Context, species: &str, source: &Source) -> Option<&'a Resolution> {
    let url = url_sha256(source);
    ctx.resolved(&id(species, source))
        .filter(|r| r.inputs_sha256.get("url") == Some(&url))
}

/// The decision for a source the fetch could not read, `error` verbatim.
pub fn decision(
    ctx: &Context,
    species: &str,
    source: &Source,
    error: &str,
) -> Result<Decision, StageError> {
    Ok(Decision::new(
        parts(species, source),
        &STAGES[2..],
        inputs(ctx, species, source)?,
        vec![],
        json!({"source": source.id, "url": source.url, "error": error}),
        &["retry", "replace-source", "drop-source"],
        "The fetch could not read this source: the adapter failed, the checksum changed, or the page it returned was not the source and its raw body did not hold it either. Nothing is fetched by another route in its place. A person resolves it: retry fetches again, replace-source fetches the url in the resolution's payload under this id, drop-source skips it; a retry or replacement that fails again reopens it.",
    ))
}

fn parts<'a>(species: &'a str, source: &'a Source) -> DecisionParts<'a> {
    DecisionParts {
        species,
        stage: STAGE,
        kind: KIND,
        field: Some(&source.id),
        age_years: None,
    }
}

fn url_sha256(source: &Source) -> String {
    crate::sha256_hex(source.url.as_bytes())
}

/// The url, and the resolution written for this decision when there is one:
/// the attempt this failure answers.
fn inputs(
    ctx: &Context,
    species: &str,
    source: &Source,
) -> Result<BTreeMap<String, String>, StageError> {
    let id = id(species, source);
    let mut inputs: BTreeMap<String, String> = [("url".to_string(), url_sha256(source))].into();
    let written = read_resolutions(&ctx.paths.resolutions())?;
    if let Some(attempt) = written.iter().find(|r| r.id == id) {
        let spent = json!({"option": attempt.option, "payload": attempt.payload,
                           "by": attempt.by, "at": attempt.at});
        inputs.insert("attempt".into(), canonical_sha256(&spent));
    }
    Ok(inputs)
}
