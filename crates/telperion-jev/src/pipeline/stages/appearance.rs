//! The aggregate stage's appearance route (fn-118). Code extracts, from
//! each fetched document's cached text in the manifest's order (the trait's
//! own list when it names one, fn-157), every sentence that carries the
//! trait's words in the requirements table, and Jev scores each sentence
//! alone over the table's levels (fn-128). The first sentence it places on a
//! level is the chosen span; code copies that level's range for every
//! material field the trait feeds into the profile, with that sentence and
//! its source (fn-127): a value with no source is never written. Nothing
//! renders or measures it. A variation trait (a hue or brightness range) the
//! sources leave unstated takes its zero-width level as a default with its
//! reason (fn-133). `leaf_back_colour` left unstated while
//! `leaf_front_colour` has a sourced level takes the front's level, its
//! ranges mapped onto the back's material fields, as a default citing the
//! front's source (fn-139). Bark colour and every other colour trait never
//! default this way; any other trait the sources leave unstated stays
//! unstated, and the generator's default stands.

use std::collections::BTreeMap;

use serde_json::{json, Map, Value};

use crate::extract::{key_terms, section_for_terms, sentences_with_terms};
use crate::pipeline::judge::Judge;
use crate::pipeline::manifest::Manifest;
use crate::pipeline::requirements::{table, AppearanceLevel};
use crate::pipeline::sets::{
    chosen_level, described_questions, DescribedLevel, DESCRIBED_UNSTATED,
};
use crate::pipeline::stage::{Context, StageError};

use super::aggregate::STAGE;
use super::cached_markdown;

const SECTION_RADIUS: usize = 600;

/// The two leaf-face colour traits the front-default rule (fn-139) names.
const LEAF_FRONT_COLOUR: &str = "leaf_front_colour";
const LEAF_BACK_COLOUR: &str = "leaf_back_colour";

/// What the route leaves for the aggregate stage to write.
#[derive(Default)]
pub struct Copied {
    /// `aggregate.body.appearance`: trait -> level, sentence, ledger.
    pub body: Map<String, Value>,
    /// `profiles[0].appearance`: trait -> level, summary, ranges, sources.
    pub profile: Map<String, Value>,
    /// Sidecar entries keyed by JSON Pointer.
    pub sidecar: Map<String, Value>,
    /// Sidecar defaults keyed by JSON Pointer: values no source states.
    pub defaults: Map<String, Value>,
    pub ledger: Vec<String>,
}

/// The documents a trait is read from: its own list, or every source.
pub fn read_from(manifest: &Manifest, listed: &[String]) -> Vec<String> {
    match listed.is_empty() {
        true => manifest.sources.iter().map(|s| s.id.clone()).collect(),
        false => listed.to_vec(),
    }
}

pub fn run(judge: &Judge<'_>, ctx: &Context, fetch: &Value) -> Result<Copied, StageError> {
    let manifest = &ctx.admitted.manifest;
    let mut copied = Copied::default();
    // Every trait's chosen sentence, gathered first so the back-colour
    // default (fn-139) can read the front's outcome regardless of the
    // manifest's own appearance order.
    let mut chosen_by_trait: BTreeMap<String, Chosen> = BTreeMap::new();
    for trait_ in &manifest.appearance {
        let name = trait_.trait_name.as_str();
        let levels = table().levels(name).unwrap_or_default();
        let sources = read_from(manifest, &trait_.sources);
        let chosen = chosen_sentence(judge, ctx, name, &sources, &levels, fetch)?;
        copied.ledger.extend(chosen.ledger.iter().cloned());
        copied.body.insert(
            name.into(),
            json!({"level": chosen.level, "source": chosen.source, "sentence": chosen.span, "ledger": chosen.ledger}),
        );
        chosen_by_trait.insert(name.to_string(), chosen);
    }
    for trait_ in &manifest.appearance {
        let name = trait_.trait_name.as_str();
        let chosen = &chosen_by_trait[name];
        let pointer = format!("/profiles/0/appearance/{name}");
        let row = table().level(name, &chosen.level);
        if let Some((row, source)) = row.zip(chosen.source.as_ref()) {
            copied.profile.insert(
                name.into(),
                json!({"level": row.key, "summary": row.summary, "ranges": row.ranges, "sources": [source]}),
            );
            copied.sidecar.insert(
                pointer,
                json!({"route": "appearance", "level": row.key, "source": source, "span": chosen.span, "ledger": chosen.ledger}),
            );
        } else if let Some(zero) = table().zero_width(name) {
            copied.default_to(name, pointer, zero, &chosen.ledger);
        } else if let Some((front_level, front_source)) = (name == LEAF_BACK_COLOUR)
            .then(|| front_sourced_level(&chosen_by_trait))
            .flatten()
        {
            copied.default_to_front(pointer, front_level, front_source, &chosen.ledger);
        }
    }
    Ok(copied)
}

/// `leaf_front_colour`'s chosen level and source when a source states it;
/// `None` when the front is itself unstated (fn-139).
fn front_sourced_level(
    chosen_by_trait: &BTreeMap<String, Chosen>,
) -> Option<(&AppearanceLevel, &str)> {
    let front = chosen_by_trait.get(LEAF_FRONT_COLOUR)?;
    let source = front.source.as_deref()?;
    let level = table().level(LEAF_FRONT_COLOUR, &front.level)?;
    Some((level, source))
}

/// Why an unstated `leaf_back_colour` reads the front's level (fn-139).
fn front_colour_default_reason(front_source: &str) -> String {
    format!(
        "no admitted source states the leaf underside colour; it takes leaf_front_colour's \
         sourced level, cited to {front_source}"
    )
}

/// Why an unstated variation trait reads zero width (fn-133).
pub const ZERO_WIDTH_DEFAULT: &str = "every source leaves this variation unstated; foliage no source calls varied takes the zero-width level";

impl Copied {
    /// Records a variation trait no source states at its zero-width level,
    /// as a default with its reason and no source, never a sourced value.
    fn default_to(
        &mut self,
        name: &str,
        pointer: String,
        zero: &AppearanceLevel,
        ledger: &[String],
    ) {
        self.body.insert(
            name.into(),
            json!({"level": zero.key, "source": null, "sentence": "", "ledger": ledger, "default": ZERO_WIDTH_DEFAULT}),
        );
        self.profile.insert(
            name.into(),
            json!({"level": zero.key, "summary": zero.summary, "ranges": zero.ranges, "sources": [], "default": ZERO_WIDTH_DEFAULT}),
        );
        self.defaults.insert(
            pointer,
            json!({"route": "default", "level": zero.key, "reason": ZERO_WIDTH_DEFAULT, "ledger": ledger}),
        );
    }

    /// Records `leaf_back_colour` at the front's chosen level and its
    /// ranges mapped onto the back's material fields, as a default citing
    /// the front's source (fn-139), never a sourced value.
    fn default_to_front(
        &mut self,
        pointer: String,
        front_level: &AppearanceLevel,
        front_source: &str,
        ledger: &[String],
    ) {
        let reason = front_colour_default_reason(front_source);
        let ranges: BTreeMap<String, [f64; 2]> = front_level
            .ranges
            .iter()
            .filter_map(|(field, range)| {
                field
                    .strip_prefix("leaf_front_")
                    .map(|suffix| (format!("leaf_back_{suffix}"), *range))
            })
            .collect();
        self.body.insert(
            LEAF_BACK_COLOUR.into(),
            json!({"level": front_level.key, "source": front_source, "sentence": "", "ledger": ledger, "default": reason}),
        );
        self.profile.insert(
            LEAF_BACK_COLOUR.into(),
            json!({"level": front_level.key, "summary": front_level.summary, "ranges": ranges, "sources": [front_source], "default": reason}),
        );
        self.defaults.insert(
            pointer,
            json!({"route": "default", "level": front_level.key, "reason": reason, "source": front_source, "ledger": ledger}),
        );
    }
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
