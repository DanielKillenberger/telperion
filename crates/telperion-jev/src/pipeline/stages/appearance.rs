//! The select stage's appearance route (fn-118). Jev scores each appearance
//! trait over the requirements table's levels on the source sections that
//! carry its terms; code copies the chosen level's range for every material
//! field the trait feeds into the profile. Nothing renders or measures it. A
//! trait the table requires that the sources leave unstated files a
//! requirements-unmet decision for the owner.

use serde_json::{json, Map, Value};

use crate::extract::{key_terms, section_for_terms};
use crate::pipeline::consume::{sources_sha256, REQUIREMENTS_UNMET};
use crate::pipeline::decision::{Decision, DecisionParts};
use crate::pipeline::judge::Judge;
use crate::pipeline::manifest::Appearance;
use crate::pipeline::requirements::{requires_appearance, table};
use crate::pipeline::sets::{described_questions, level_from_score, DescribedLevel};
use crate::pipeline::stage::{Context, StageError};

use super::extract::cached_markdown;
use super::select::STAGE;

const SECTION_RADIUS: usize = 600;

/// What the route leaves for the select stage to write.
#[derive(Default)]
pub struct Copied {
    /// `select.body.appearance`: trait -> level, sentence, ledger.
    pub body: Map<String, Value>,
    /// `profiles[0].appearance`: trait -> level, summary, ranges, sources.
    pub profile: Map<String, Value>,
    /// Sidecar entries keyed by JSON Pointer.
    pub sidecar: Map<String, Value>,
    pub ledger: Vec<String>,
    pub decisions: Vec<Decision>,
}

pub fn run(judge: &Judge<'_>, ctx: &Context, fetch: &Value) -> Result<Copied, StageError> {
    let manifest = &ctx.admitted.manifest;
    let mut copied = Copied::default();
    for trait_ in &manifest.appearance {
        let name = trait_.trait_name.as_str();
        let levels = table().levels(name).unwrap_or_default();
        let (level, entry) = score_levels(judge, ctx, name, &trait_.sources, &levels, fetch)?;
        let ledger = entry["ledger"].as_str().unwrap_or_default().to_string();
        copied.ledger.push(ledger.clone());
        copied.body.insert(
            name.into(),
            json!({"level": level, "sentence": entry["sentence"], "ledger": ledger}),
        );
        match table().level(name, &level) {
            Some(row) => {
                let pointer = format!("/profiles/0/appearance/{name}");
                copied.profile.insert(
                    name.into(),
                    json!({"level": row.key, "summary": row.summary, "ranges": row.ranges, "sources": trait_.sources}),
                );
                copied.sidecar.insert(
                    pointer,
                    json!({"route": "appearance", "level": row.key, "sources": trait_.sources, "ledger": [ledger]}),
                );
            }
            None if requires_appearance(manifest, name) => {
                let sources = sources_sha256(&ctx.paths.manifest())?;
                copied
                    .decisions
                    .push(unstated(ctx, trait_, &ledger, &sources));
            }
            None => {}
        }
    }
    Ok(copied)
}

/// NEEDS_HUMAN: a required appearance trait no admitted source describes.
fn unstated(ctx: &Context, trait_: &Appearance, ledger: &str, sources: &str) -> Decision {
    let manifest = &ctx.admitted.manifest;
    Decision::new(
        DecisionParts {
            species: &manifest.species,
            stage: STAGE,
            kind: REQUIREMENTS_UNMET,
            field: Some(&trait_.trait_name),
            age_years: None,
        },
        &["generate"],
        [("manifest".to_string(), ctx.admitted.sha256.clone())].into_iter().collect(),
        vec![ledger.to_string()],
        json!({
            "field": trait_.trait_name, "level": "unstated", "bar": "stated",
            "sources_tried": trait_.sources, "sources_sha256": sources,
        }),
        &["add-sources"],
        "NEEDS_HUMAN: the requirements table asks for this appearance trait and no admitted source describes it. The owner adds a source that does; a resolution that adds none stays open.",
    )
}

/// Jev scores a trait over `levels` on the sections of `sources` that carry
/// the trait's terms. Returns the chosen level's key, or the no-match level,
/// with the sentences read and the ledger reference.
pub fn score_levels(
    judge: &Judge<'_>,
    ctx: &Context,
    trait_name: &str,
    sources: &[String],
    levels: &[DescribedLevel],
    fetch: &Value,
) -> Result<(String, Value), StageError> {
    let summaries: Vec<&str> = levels.iter().map(|l| l.summary.as_str()).collect();
    let terms = key_terms(&format!("{} {}", trait_name, summaries.join(" ")));
    let mut sentences = Vec::new();
    for id in sources {
        let Some(record) = fetch["sources"].get(id) else {
            continue;
        };
        let markdown = cached_markdown(ctx, STAGE, id, record)?;
        if let Some(section) = section_for_terms(&markdown, &terms, SECTION_RADIUS) {
            sentences.push(section);
        }
    }
    let state = json!({"trait": trait_name, "sentences": sentences});
    let judgment = judge
        .ask("described", None, &state, &described_questions(levels))
        .map_err(|err| StageError::Failed {
            stage: STAGE.into(),
            reason: err.to_string(),
        })?;
    let index = level_from_score(
        judgment.entry.score("level").unwrap_or(f64::NAN),
        levels.len() + 1,
    );
    let level = index
        .and_then(|i| levels.get(i))
        .map(|l| l.key.clone())
        .unwrap_or_else(|| "unstated".into());
    Ok((
        level,
        json!({"sentence": sentences.join("\n"), "ledger": judgment.reference}),
    ))
}
