//! The select stage's appearance route (fn-118). Code extracts, from each
//! admitted source's cached text in the manifest's order, every sentence
//! that carries the trait's words in the requirements table, and Jev scores
//! each sentence alone over the table's levels, as a field's sentences are
//! screened one at a time (fn-128). The first sentence it places on a level
//! is the chosen span; code copies that level's range for every material
//! field the trait feeds into the profile, with that sentence and its source
//! (fn-127): a value with no source is never written. Nothing renders or
//! measures it. A trait the table requires that the sources leave unstated
//! files a requirements-unmet decision, which the pipeline searches again
//! before the owner has it.

use std::collections::BTreeMap;

use serde_json::{json, Map, Value};

use crate::extract::{key_terms, section_for_terms, sentences_with_terms};
use crate::pipeline::consume::{sources_sha256, REQUIREMENTS_UNMET};
use crate::pipeline::decision::{Decision, DecisionParts};
use crate::pipeline::judge::Judge;
use crate::pipeline::manifest::Appearance;
use crate::pipeline::requirements::{requires_appearance, table};
use crate::pipeline::sets::{
    chosen_level, described_questions, DescribedLevel, DESCRIBED_UNSTATED,
};
use crate::pipeline::stage::{Context, StageError};

use super::extract::cached_markdown;
use super::flagged::{Flag, REPLACE_SOURCE};
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
        let chosen = chosen_sentence(judge, ctx, name, &trait_.sources, &levels, fetch)?;
        copied.ledger.extend(chosen.ledger.iter().cloned());
        copied.body.insert(
            name.into(),
            json!({"level": chosen.level, "source": chosen.source, "sentence": chosen.span, "ledger": chosen.ledger}),
        );
        let row = table().level(name, &chosen.level);
        match (row, &chosen.source) {
            (Some(row), Some(source)) => {
                let pointer = format!("/profiles/0/appearance/{name}");
                copied.profile.insert(
                    name.into(),
                    json!({"level": row.key, "summary": row.summary, "ranges": row.ranges, "sources": [source]}),
                );
                copied.sidecar.insert(
                    pointer,
                    json!({"route": "appearance", "level": row.key, "source": source, "span": chosen.span, "ledger": chosen.ledger}),
                );
            }
            _ if requires_appearance(manifest, name) => {
                let sources = sources_sha256(&ctx.paths.manifest())?;
                copied
                    .decisions
                    .push(unstated(ctx, trait_, &chosen.ledger, &sources));
            }
            _ => {}
        }
    }
    Ok(copied)
}

/// Takes out every appearance value a resolution dropped (fn-131): the
/// trait reads unstated in the select body, naming the decision, and a
/// trait the table requires, or one sent for another source, files
/// requirements-unmet, which the pipeline searches again for.
pub fn drop_flagged(
    ctx: &Context,
    copied: &mut Copied,
    flags: &BTreeMap<String, Flag>,
) -> Result<(), StageError> {
    let manifest = &ctx.admitted.manifest;
    for trait_ in &manifest.appearance {
        let name = trait_.trait_name.as_str();
        let pointer = format!("/profiles/0/appearance/{name}");
        let Some(flag) = flags.get(&pointer) else {
            continue;
        };
        if !copied.sidecar.get(&pointer).is_some_and(|e| flag.names(e)) {
            continue;
        }
        copied.sidecar.remove(&pointer);
        copied.profile.remove(name);
        copied.body.insert(
            name.into(),
            json!({"level": DESCRIBED_UNSTATED, "source": null, "sentence": "", "ledger": [], "dropped": flag.reason()}),
        );
        if flag.option == REPLACE_SOURCE || requires_appearance(manifest, name) {
            let sources = sources_sha256(&ctx.paths.manifest())?;
            copied.decisions.push(unstated(ctx, trait_, &[], &sources));
        }
    }
    Ok(())
}

/// The sentence a trait's level was read from, its source, and every
/// ledger reference asked. With no sentence placed on a level the level is
/// the no-match level and there is no source.
struct Chosen {
    level: String,
    source: Option<String>,
    span: String,
    ledger: Vec<String>,
}

/// Asks each admitted source's sentences that carry the trait's words, one
/// sentence per judgment, in source then text order, and stops at the first
/// that states the trait.
fn chosen_sentence(
    judge: &Judge<'_>,
    ctx: &Context,
    trait_name: &str,
    sources: &[String],
    levels: &[DescribedLevel],
    fetch: &Value,
) -> Result<Chosen, StageError> {
    let terms = table()
        .appearance
        .get(trait_name)
        .map(|t| t.terms.as_slice())
        .unwrap_or_default();
    let mut ledger = Vec::new();
    for id in sources {
        if ctx.admitted.manifest.source(id).is_none() {
            continue;
        }
        let Some(record) = fetch["sources"].get(id) else {
            continue;
        };
        let text = cached_markdown(ctx, STAGE, id, record)?;
        for sentence in sentences_with_terms(&text, terms) {
            let one = std::slice::from_ref(&sentence);
            let (level, entry) = ask_levels(judge, trait_name, one, levels)?;
            ledger.push(entry["ledger"].as_str().unwrap_or_default().to_string());
            if level != DESCRIBED_UNSTATED {
                return Ok(Chosen {
                    level,
                    source: Some(id.clone()),
                    span: sentence,
                    ledger,
                });
            }
        }
    }
    Ok(Chosen {
        level: DESCRIBED_UNSTATED.into(),
        source: None,
        span: String::new(),
        ledger,
    })
}

/// A required appearance trait no source it names describes: searched again
/// by the pipeline (fn-129 R6), then the owner's.
fn unstated(ctx: &Context, trait_: &Appearance, ledger: &[String], sources: &str) -> Decision {
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
        ledger.to_vec(),
        json!({
            "field": trait_.trait_name, "level": DESCRIBED_UNSTATED, "bar": "stated",
            "sources_tried": trait_.sources, "sources_sha256": sources,
        }),
        &["add-sources"],
        "The requirements table asks for this appearance trait and no source it names describes it. The pipeline searches again, two rounds at most, and adds the source it finds to the trait's list; after them it is NEEDS_HUMAN and the owner adds a source. A resolution that adds none stays open.",
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
    let sentences = sections(ctx, trait_name, sources, levels, fetch)?;
    ask_levels(judge, trait_name, &sentences, levels)
}

/// The section of each source's cached text that carries a described
/// trait's terms.
fn sections(
    ctx: &Context,
    trait_name: &str,
    sources: &[String],
    levels: &[DescribedLevel],
    fetch: &Value,
) -> Result<Vec<String>, StageError> {
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
    Ok(sentences)
}

fn ask_levels(
    judge: &Judge<'_>,
    trait_name: &str,
    sentences: &[String],
    levels: &[DescribedLevel],
) -> Result<(String, Value), StageError> {
    let state = json!({"trait": trait_name, "sentences": sentences});
    let judgment = judge
        .ask("described", None, &state, &described_questions(levels))
        .map_err(|err| StageError::Failed {
            stage: STAGE.into(),
            reason: err.to_string(),
        })?;
    // The most probable level, `unstated` when it is (fn-131).
    let index = chosen_level(
        judgment.entry.probabilities("level"),
        levels.len() + 1,
        levels.len(),
        crate::questions::thresholds().level_floor,
    );
    let level = levels
        .get(index)
        .map(|l| l.key.clone())
        .unwrap_or_else(|| DESCRIBED_UNSTATED.into());
    Ok((
        level,
        json!({"sentence": sentences.join("\n"), "ledger": judgment.reference}),
    ))
}
