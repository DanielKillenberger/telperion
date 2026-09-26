//! The pipeline question sets (fn-58), their labelled cases and scoring.
//!
//! Each set is versioned JSON under `data/questions`, its labelled cases are
//! JSON under `data/cases`, and every question offers a no-match answer:
//! the trailing `unstated` level for a described trait, for the read
//! stage's label (fn-157) the `none` field, the `unclear` basis and the
//! `unstated` condition, and `unclear` for a document's kind. Code lays out the state and owns every count and
//! every number; Jev only picks a level or a label.
//!
//! A case is `holdout` when it is held out of the labelled set that tuned the
//! wording, and `negative` when the admitted answer is the one that rejects:
//! a trait the sentences never describe, or a span that measures nothing the
//! manifest asks for.

pub mod cases;

use serde::Deserialize;
use serde_json::{json, Map, Value};

pub const DESCRIBED_JSON: &str = include_str!("../../data/questions/described.json");
pub const LABEL_JSON: &str = include_str!("../../data/questions/label.json");
pub const KIND_JSON: &str = include_str!("../../data/questions/kind.json");
pub const DESCRIBED_CASES: &str = include_str!("../../data/cases/described.json");
pub const LABEL_CASES: &str = include_str!("../../data/cases/label.json");
pub const KIND_CASES: &str = include_str!("../../data/cases/kind.json");

/// The no-match level appended to every described trait's table.
pub const DESCRIBED_UNSTATED: &str = "unstated";
/// The no-match answer of the label's field question.
pub const LABEL_NONE: &str = "none";
/// The kinds of document, best first: a value comes from the best kind that
/// agrees (fn-157, host decision). `unclear` is the no-match answer and
/// ranks with `other`.
pub const KINDS: [&str; 6] = [
    "flora",
    "forestry",
    "garden",
    "extension",
    "nursery",
    "other",
];
pub const KIND_UNCLEAR: &str = "unclear";

/// A kind's rank among `KINDS`, 0 best; `unclear` and anything unknown rank
/// last.
pub fn tier(kind: &str) -> usize {
    KINDS
        .iter()
        .position(|k| *k == kind)
        .unwrap_or(KINDS.len() - 1)
}

/// The label's questions besides the field, as the read stage keeps them.
pub const LABEL_QUESTIONS: [&str; 3] = ["basis", "age", "condition"];

fn parse(raw: &str, what: &str) -> Value {
    serde_json::from_str(raw).unwrap_or_else(|err| panic!("{what}: {err}"))
}

/// The version recorded in one set's JSON. `0` when the name is unknown.
pub fn set_version(name: &str) -> u32 {
    let raw = match name {
        "described" => DESCRIBED_JSON,
        "label" => LABEL_JSON,
        "kind" => KIND_JSON,
        _ => return 0,
    };
    parse(raw, name)["version"].as_u64().unwrap_or(0) as u32
}

/// The read stage's label (fn-157): which of `fields` (id, what it
/// measures) the marked span states, `none` last, and its basis, age and
/// growing condition.
pub fn label_questions(fields: &[(String, String)]) -> Value {
    let raw = parse(LABEL_JSON, "label.json");
    let mut criteria = Map::new();
    for (id, what) in fields {
        criteria.insert(id.clone(), json!(what));
    }
    criteria.insert(LABEL_NONE.into(), raw["field"]["none"].clone());
    let mut questions = json!({"field": {
        "type": "choice",
        "instructions": raw["field"]["instructions"],
        "criteria": criteria,
    }});
    for name in LABEL_QUESTIONS {
        questions[name] = raw[name].clone();
    }
    questions
}

/// The document kind Choice (fn-157), `unclear` its no-match answer.
pub fn kind_questions() -> Value {
    let raw = parse(KIND_JSON, "kind.json");
    json!({"document": raw["document"]})
}

/// One described trait's level table, as a person wrote it in the manifest.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct DescribedLevel {
    pub key: String,
    pub summary: String,
}

/// Described-level Score over the manifest's levels in table order, with the
/// no-match level last.
pub fn described_questions(levels: &[DescribedLevel]) -> Value {
    let raw = parse(DESCRIBED_JSON, "described.json");
    let mut criteria: Vec<Value> = levels
        .iter()
        .map(|level| json!({"key": level.key, "summary": level.summary}))
        .collect();
    criteria.push(json!({
        "key": DESCRIBED_UNSTATED,
        "summary": raw["level"][DESCRIBED_UNSTATED],
    }));
    json!({
        "level": {
            "type": "score",
            "instructions": raw["level"]["instructions"],
            "criteria": criteria,
        }
    })
}

/// The level Jev gave the highest probability, by index among `count`
/// levels (fn-131). A level table is a set of choices, not a scale: the
/// rounded average once put a leaf underside on `silvery_white` at
/// probability 0. A tie goes to `no_match`; no probabilities, or a most
/// probable level below `floor`, answers `no_match` too.
pub fn chosen_level(
    probabilities: Option<&Value>,
    count: usize,
    no_match: usize,
    floor: f64,
) -> usize {
    let Some(map) = probabilities.and_then(Value::as_object) else {
        return no_match;
    };
    let p = |i: usize| {
        map.get(&i.to_string())
            .and_then(Value::as_f64)
            .unwrap_or(0.0)
    };
    let best = (0..count).fold(no_match, |best, i| if p(i) > p(best) { i } else { best });
    if p(best) < floor {
        no_match
    } else {
        best
    }
}

/// Round a Score to the nearest level index, clamped into the table: for an
/// ordered scale only (the transfer relation), never a level table. A
/// missing or non-finite score is no level at all: the caller routes it to
/// its no-match answer, never to the first row.
pub fn level_from_score(score: f64, level_count: usize) -> Option<usize> {
    if level_count == 0 || !score.is_finite() {
        return None;
    }
    let last = (level_count - 1) as f64;
    Some(score.round().clamp(0.0, last) as usize)
}

#[derive(Debug, Clone, Deserialize)]
pub struct DescribedCase {
    pub id: String,
    #[serde(rename = "trait")]
    pub trait_name: String,
    pub levels: Vec<DescribedLevel>,
    pub sentences: Vec<String>,
    pub expect_level: String,
    pub holdout: bool,
    pub negative: bool,
}

/// One labelled span: a sentence a recorded run read, the span and which
/// of its equal occurrences is marked, the manifest's fields, and the label
/// a person admits.
#[derive(Debug, Clone, Deserialize)]
pub struct LabelCase {
    pub id: String,
    pub species: String,
    pub url: String,
    pub sentence: String,
    pub context: String,
    pub span: String,
    #[serde(default)]
    pub occurrence: usize,
    pub fields: Vec<String>,
    pub expect_field: String,
    pub expect_basis: String,
    pub expect_age: String,
    pub expect_condition: String,
    pub holdout: bool,
    pub negative: bool,
}

/// One gathered document and the kind a person admits for it.
#[derive(Debug, Clone, Deserialize)]
pub struct KindCase {
    pub id: String,
    pub species: String,
    pub url: String,
    pub title: String,
    pub passages: Vec<String>,
    pub expect_kind: String,
    pub holdout: bool,
    pub negative: bool,
}

pub fn kind_cases() -> Vec<KindCase> {
    serde_json::from_str(KIND_CASES).expect("kind cases")
}

/// What Jev reads to class a document: the species, its address and title,
/// and up to three of its sentences.
pub fn kind_state(species: &str, id: &str, url: &str, title: &str, passages: &[String]) -> Value {
    json!({"species": species, "source": {"id": id, "url": url, "title": title},
           "passages": passages})
}

pub fn described_cases() -> Vec<DescribedCase> {
    serde_json::from_str(DESCRIBED_CASES).expect("described cases")
}

pub fn label_cases() -> Vec<LabelCase> {
    serde_json::from_str(LABEL_CASES).expect("label cases")
}

pub fn described_state(case: &DescribedCase) -> Value {
    json!({ "trait": case.trait_name, "sentences": case.sentences })
}

/// The ids of the cases a set answered wrongly, for the miss report.
pub fn missed_ids(set: &crate::cases::SetScore) -> Vec<String> {
    set.rows
        .iter()
        .filter(|row| !row.hit)
        .map(|row| row.id.clone())
        .collect()
}
