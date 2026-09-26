//! Read (fn-157): every fetched document is read once. Code finds each
//! number-and-unit span that states a length in a sentence naming one of the
//! manifest's fields, every occurrence on its own, so two equal spans in one
//! sentence ("Leaf Length:3-6 inches Leaf Width:3-6 inches") are two
//! readings. Jev labels each occurrence, marked in its sentence, with the
//! field it states, its basis (typical, a record, one specimen, a cultivar),
//! the age and the growing condition; it never chooses between sources and
//! never supplies a number. Code parses the number and checks it against
//! the words beside the span: a span the text calls long, labelled a width,
//! is set aside, never counted. Jev also classes each document's kind (a
//! flora, a forestry manual, a garden's, an extension's or a nursery's
//! page), which ranks it for the aggregate (host decision, fn-157).

use serde_json::{json, Value};

use crate::extract::{bound_state_text, candidate_sentences};
use crate::ledger::SourceRef;
use crate::pipeline::judge::Judge;
use crate::pipeline::manifest::Manifest;
use crate::pipeline::requirements::{names_field, table};
use crate::pipeline::sets::{kind_questions, kind_state, label_questions, KIND_UNCLEAR};
use crate::pipeline::stage::{Context, Paths, StageError};
use crate::quantity::{first_length, unit_re};

use super::{body, cached_markdown, inputs};

pub const STAGE: &str = "read";
/// The occurrences read per document, in text order: a bound on the Jev
/// calls one long paper can cost (fn-157, a proposal with the document
/// count).
pub const SPANS_PER_DOCUMENT: usize = 40;

#[derive(Debug)]
pub enum Outcome {
    Current,
    Ran { spans: usize },
}

/// One span as it stands in its sentence.
#[derive(Debug, Clone, PartialEq)]
pub struct Occurrence {
    pub sentence: String,
    pub context: String,
    /// The span's byte offset in `sentence`.
    pub at: usize,
    pub span: String,
}

impl Occurrence {
    /// The sentence with this span marked, the way Jev reads it.
    pub fn marked(&self) -> String {
        let end = self.at + self.span.len();
        format!(
            "{}⟦{}⟧{}",
            &self.sentence[..self.at],
            self.span,
            &self.sentence[end..]
        )
    }
}

pub fn run(paths: &Paths, judge: &Judge<'_>) -> Result<Outcome, StageError> {
    let (ctx, _) = Context::open(paths, STAGE)?;
    let (fetch, fetch_sha) = body(&ctx, STAGE, "fetch")?;
    let mut header = ctx.header(STAGE, "read", inputs(&[("fetch.json", &fetch_sha)]), vec![]);
    if ctx.is_current(STAGE, &header.idempotence_key) {
        return Ok(Outcome::Current);
    }
    let manifest = &ctx.admitted.manifest;
    let fields: Vec<String> = manifest.fields.iter().map(|f| f.field.clone()).collect();
    let questions = label_questions(&described(manifest));
    let mut spans = Vec::new();
    let mut documents = serde_json::Map::new();
    for (id, record) in fetch["sources"].as_object().into_iter().flatten() {
        let markdown = cached_markdown(&ctx, STAGE, id, record)?;
        let source = SourceRef {
            id: id.clone(),
            url: record["final_url"].as_str().unwrap_or_default().into(),
            sha256: record["markdown_sha256"]
                .as_str()
                .unwrap_or_default()
                .into(),
            bytes: record["markdown_bytes"].as_u64().unwrap_or_default(),
        };
        let found = occurrences(&markdown, &fields);
        let title = manifest.source(id).map_or("", |s| s.title.as_str());
        let kind = kind(
            judge,
            manifest,
            &source,
            title,
            &passages(&markdown, &found),
        )?;
        header
            .ledger
            .push(kind["ledger"].as_str().unwrap_or_default().into());
        documents.insert(id.clone(), kind);
        for found in found {
            let row = label(judge, manifest, &source, &found, &questions)?;
            header
                .ledger
                .push(row["ledger"].as_str().unwrap_or_default().into());
            spans.push(row);
        }
    }
    let count = spans.len();
    ctx.write(&header, json!({"documents": documents, "spans": spans}))?;
    Ok(Outcome::Ran { spans: count })
}

/// What a document's kind is read from: up to three of the sentences read
/// asks about, else its first three candidate sentences.
fn passages(markdown: &str, found: &[Occurrence]) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    for o in found {
        if !out.contains(&o.sentence) {
            out.push(o.sentence.clone());
        }
    }
    if out.is_empty() {
        out = candidate_sentences(markdown)
            .into_iter()
            .map(|c| c.sentence)
            .collect();
    }
    out.truncate(3);
    out.iter().map(|p| bound_state_text(p)).collect()
}

/// A document's kind as Jev classes it, `unclear` when it cannot.
fn kind(
    judge: &Judge<'_>,
    manifest: &Manifest,
    source: &SourceRef,
    title: &str,
    passages: &[String],
) -> Result<Value, StageError> {
    let taxon = &manifest.taxon;
    let species = format!("{} ({})", taxon.common_name, taxon.scientific_name);
    let state = kind_state(&species, &source.id, &source.url, title, passages);
    let judgment = judge
        .ask("kind", Some(source), &state, &kind_questions())
        .map_err(|err| StageError::Failed {
            stage: STAGE.into(),
            reason: err.to_string(),
        })?;
    let kind = judgment
        .entry
        .choice("document")
        .unwrap_or_else(|| KIND_UNCLEAR.into());
    Ok(json!({"kind": kind, "ledger": judgment.reference}))
}

/// Each manifest field with the words that say what it measures.
fn described(manifest: &Manifest) -> Vec<(String, String)> {
    manifest
        .fields
        .iter()
        .map(|f| {
            let what = table()
                .fields
                .get(&f.field)
                .map_or_else(|| f.question.clone(), |t| t.what.clone());
            (f.field.clone(), what)
        })
        .collect()
}

/// Every length span above zero of every sentence that names one of
/// `fields`, each occurrence once, in text order, at most
/// `SPANS_PER_DOCUMENT`.
pub fn occurrences(markdown: &str, fields: &[String]) -> Vec<Occurrence> {
    let mut out = Vec::new();
    for candidate in candidate_sentences(markdown) {
        let sentence = candidate.sentence;
        if !fields.iter().any(|f| names_field(f, &sentence)) {
            continue;
        }
        for found in unit_re().find_iter(&sentence) {
            // A zero is no size: "Height: 50 ft. 0 in." states no height of 0.
            if !first_length(found.as_str()).is_some_and(|(range, _)| range[1] > 0.0) {
                continue;
            }
            out.push(Occurrence {
                sentence: sentence.clone(),
                context: candidate.context.clone(),
                at: found.start(),
                span: found.as_str().to_string(),
            });
        }
    }
    out.truncate(SPANS_PER_DOCUMENT);
    out
}

/// One occurrence labelled: the row the read body keeps.
fn label(
    judge: &Judge<'_>,
    manifest: &Manifest,
    source: &SourceRef,
    found: &Occurrence,
    questions: &Value,
) -> Result<Value, StageError> {
    let taxon = &manifest.taxon;
    let species = format!("{} ({})", taxon.common_name, taxon.scientific_name);
    let described =
        json!({"id": source.id, "url": source.url, "sha256": source.sha256, "bytes": source.bytes});
    let state = label_state(&species, described, found);
    let judgment = judge
        .ask("label", Some(source), &state, questions)
        .map_err(|err| StageError::Failed {
            stage: STAGE.into(),
            reason: err.to_string(),
        })?;
    let entry = &judgment.entry;
    let choice = |q: &str, none: &str| entry.choice(q).unwrap_or_else(|| none.into());
    Ok(json!({
        "source": source.id,
        "sentence": found.sentence,
        "at": found.at,
        "span": found.span,
        "field": choice("field", "none"),
        "basis": choice("basis", "unclear"),
        "age": choice("age", "mature"),
        "condition": choice("condition", "unstated"),
        "ledger": judgment.reference,
    }))
}

/// What Jev reads for one occurrence: the species, the document, the
/// sentence as it stands (the passage a trimmed recording keeps), the same
/// sentence with the span marked, and the text around it.
pub fn label_state(species: &str, source: Value, found: &Occurrence) -> Value {
    json!({
        "species": species,
        "source": source,
        "candidate": {
            "sentence": bound_state_text(&found.sentence),
            "marked": bound_state_text(&found.marked()),
            "context": bound_state_text(&found.context),
            "span": found.span,
        },
    })
}

/// The dimension the words right beside a span name, when they name one:
/// a label before it ("Leaf Width: 3-6 inches") or a word after it ("2 to 4
/// inches long"), past a conversion in brackets.
pub fn beside(sentence: &str, at: usize, span: &str) -> Option<&'static str> {
    let lower = sentence.to_ascii_lowercase();
    let after = lower.get(at + span.len()..).unwrap_or_default();
    let after = skip_bracket(after.trim_start()).trim_start();
    const AFTER: [(&str, &str); 9] = [
        ("long", "length"),
        ("in length", "length"),
        ("wide", "width"),
        ("broad", "width"),
        ("across", "width"),
        ("in width", "width"),
        ("tall", "height"),
        ("in height", "height"),
        ("in diameter", "diameter"),
    ];
    if let Some((_, d)) = AFTER.iter().find(|(w, _)| starts_word(after, w)) {
        return Some(d);
    }
    let before = lower.get(..at).unwrap_or_default().trim_end();
    const LABELS: [(&str, &str); 6] = [
        ("breast height:", "diameter"),
        ("length:", "length"),
        ("width:", "width"),
        ("height:", "height"),
        ("spread:", "width"),
        ("diameter:", "diameter"),
    ];
    LABELS
        .iter()
        .find(|(w, _)| before.ends_with(w))
        .map(|(_, d)| *d)
}

/// The dimension a field measures, by its name.
pub fn dimension(field: &str) -> Option<&'static str> {
    [
        ("length", "length"),
        ("width", "width"),
        ("dbh", "diameter"),
        ("diameter", "diameter"),
        ("height_m", "height"),
    ]
    .iter()
    .find(|(w, _)| field.contains(w))
    .map(|(_, d)| *d)
}

fn skip_bracket(text: &str) -> &str {
    match text.strip_prefix('(').and_then(|rest| rest.split_once(')')) {
        Some((_, rest)) => rest,
        None => text,
    }
}

fn starts_word(text: &str, word: &str) -> bool {
    text.strip_prefix(word)
        .is_some_and(|rest| !rest.starts_with(|c: char| c.is_alphanumeric()))
}

#[cfg(test)]
mod tests {
    use super::*;

    const NC_STATE: &str = "Leaf Type:SimpleLeaf Length:3-6 inchesLeaf Width:3-6 inchesLeaf Description:The leaves measure 2 to 4 inches long and are alternate.";

    fn fields() -> Vec<String> {
        ["leaf_length_m", "leaf_width_m", "dbh_m"]
            .map(String::from)
            .to_vec()
    }

    #[test]
    fn two_equal_spans_in_one_sentence_are_two_occurrences() {
        let found = occurrences(NC_STATE, &fields());
        let spans: Vec<(&str, usize)> = found.iter().map(|o| (o.span.as_str(), o.at)).collect();
        assert_eq!(spans.len(), 3, "{spans:?}");
        assert_eq!((spans[0].0, spans[1].0), ("3-6 inches", "3-6 inches"));
        assert_ne!(spans[0].1, spans[1].1);
        assert!(found[1].marked().contains("Leaf Width:⟦3-6 inches⟧Leaf"));
    }

    #[test]
    fn the_words_beside_a_span_name_its_dimension() {
        let found = occurrences(NC_STATE, &fields());
        let named: Vec<Option<&str>> = found
            .iter()
            .map(|o| beside(&o.sentence, o.at, &o.span))
            .collect();
        assert_eq!(named, [Some("length"), Some("width"), Some("length")]);
        let s = "The leaflets are 30 centimetres (12 inches) long.";
        assert_eq!(beside(s, 17, "30 centimetres"), Some("length"));
        let heritage = "Trunk DBH (diameter at breast height: 171.6 inches";
        assert_eq!(beside(heritage, 38, "171.6 inches"), Some("diameter"));
        assert_eq!(beside("It is 20 m longer", 6, "20 m"), None);
    }

    #[test]
    fn a_field_names_its_dimension() {
        assert_eq!(dimension("leaf_width_m"), Some("width"));
        assert_eq!(dimension("crown_width_m"), Some("width"));
        assert_eq!(dimension("dbh_m"), Some("diameter"));
        assert_eq!(dimension("height_m"), Some("height"));
        assert_eq!(dimension("crown_base_m"), None);
        assert_eq!(dimension("height_growth_m_per_year"), None);
    }

    #[test]
    fn a_sentence_naming_no_field_an_age_and_a_zero_are_not_read() {
        let text = "The nuts are 2 cm across. It lives 300 years. The tree grows 30 m tall. Height: 0 in. tall.";
        let found = occurrences(text, &["height_m".to_string()]);
        let spans: Vec<&str> = found.iter().map(|o| o.span.as_str()).collect();
        assert_eq!(spans, ["30 m"]);
    }
}
