//! The pipeline question sets (fn-58), their labelled cases and scoring.
//!
//! Each set is versioned JSON under `data/questions`, its labelled cases are
//! JSON under `data/cases`, and every question offers a no-match answer:
//! `none` for the ranking Choice and the dominant gap, the `none` level for
//! sufficiency and the mature size, the trailing `unstated` level for a described trait, and the
//! false criterion of each obligation Noul. Code lays out the state and owns
//! every count; Jev only picks a level, a candidate or a side.
//!
//! A case is `holdout` when it is held out of the labelled set that tuned the
//! wording, and `negative` when the admitted answer is the one that rejects:
//! evidence that misses its requirement, a candidate list with no usable
//! source, a trait the sentences never describe, an image never inspected, or
//! a value the author composed rather than measured.

pub mod cases;

use serde::Deserialize;
use serde_json::{json, Map, Value};

pub const SUFFICIENCY_JSON: &str = include_str!("../../data/questions/sufficiency.json");
pub const MATURE_JSON: &str = include_str!("../../data/questions/mature_size.json");
pub const RANKING_JSON: &str = include_str!("../../data/questions/ranking.json");
pub const DESCRIBED_JSON: &str = include_str!("../../data/questions/described.json");
pub const OBLIGATIONS_JSON: &str = include_str!("../../data/questions/obligations.json");
pub const SUFFICIENCY_CASES: &str = include_str!("../../data/cases/sufficiency.json");
pub const MATURE_CASES: &str = include_str!("../../data/cases/mature_size.json");
pub const RANKING_CASES: &str = include_str!("../../data/cases/ranking.json");
pub const DESCRIBED_CASES: &str = include_str!("../../data/cases/described.json");
pub const OBLIGATION_CASES: &str = include_str!("../../data/cases/obligations.json");

/// The sufficiency Score's levels, lowest first. The index Jev scores.
pub const SUFFICIENCY_LEVELS: [&str; 4] = ["none", "proxy_only", "partial", "sufficient"];
/// The no-match level appended to every described trait's table.
pub const DESCRIBED_UNSTATED: &str = "unstated";
/// The no-match key of the ranking Choice.
pub const RANKING_NONE: &str = "none";
/// The obligation Nouls, each asked alone with its own state.
pub const OBLIGATION_NAMES: [&str; 2] = ["inspected_image", "measurement_not_invention"];

fn parse(raw: &str, what: &str) -> Value {
    serde_json::from_str(raw).unwrap_or_else(|err| panic!("{what}: {err}"))
}

/// The version recorded in one set's JSON. `0` when the name is unknown.
pub fn set_version(name: &str) -> u32 {
    let raw = match name {
        "sufficiency" => SUFFICIENCY_JSON,
        "mature_size" => MATURE_JSON,
        "ranking" => RANKING_JSON,
        "described" => DESCRIBED_JSON,
        "obligations" => OBLIGATIONS_JSON,
        _ => return 0,
    };
    parse(raw, name)["version"].as_u64().unwrap_or(0) as u32
}

/// Sufficiency Score and dominant-gap Choice over one requirement's evidence.
pub fn sufficiency_questions() -> Value {
    let raw = parse(SUFFICIENCY_JSON, "sufficiency.json");
    json!({
        "sufficiency": raw["sufficiency"],
        "dominant_gap": raw["dominant_gap"],
    })
}

/// Mature-size Score over the same four levels, and its gap Choice (fn-127):
/// a stated mature value or range for the taxon, with no age asked.
pub fn mature_questions() -> Value {
    let raw = parse(MATURE_JSON, "mature_size.json");
    json!({
        "mature_size": raw["mature_size"],
        "mature_gap": raw["mature_gap"],
    })
}

/// Source-ranking Choice: every candidate source id plus the no-match key.
pub fn ranking_questions(candidates: &[String]) -> Value {
    let raw = parse(RANKING_JSON, "ranking.json");
    let mut criteria = Map::new();
    for id in candidates {
        criteria.insert(id.clone(), Value::Null);
    }
    criteria.insert(RANKING_NONE.into(), raw["source"][RANKING_NONE].clone());
    json!({
        "source": {
            "type": "choice",
            "instructions": raw["source"]["instructions"],
            "criteria": criteria,
        }
    })
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

/// One semantic obligation Noul, asked alone with its own state.
pub fn obligation_questions(name: &str) -> Value {
    let raw = parse(OBLIGATIONS_JSON, "obligations.json");
    let mut out = Map::new();
    out.insert(name.to_string(), raw[name].clone());
    Value::Object(out)
}

/// Round a Score to the nearest level index, clamped into the table. A
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
pub struct SufficiencyCase {
    pub id: String,
    pub requirement: Value,
    pub evidence: Value,
    pub counts: Value,
    pub expect_level: String,
    pub expect_gap: String,
    pub holdout: bool,
    pub negative: bool,
}

/// A mature-size case: the state the quality stage lays out for a mature
/// field, with the level and gap a person admits.
#[derive(Debug, Clone, Deserialize)]
pub struct MatureCase {
    pub id: String,
    pub requirement: Value,
    pub evidence: Value,
    pub counts: Value,
    pub expect_level: String,
    pub expect_gap: String,
    pub holdout: bool,
    pub negative: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RankingCandidate {
    pub id: String,
    pub title: String,
    pub snippet: String,
    pub kind: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RankingCase {
    pub id: String,
    pub requirement: Value,
    pub candidates: Vec<RankingCandidate>,
    pub expect_source: String,
    pub holdout: bool,
    pub negative: bool,
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

#[derive(Debug, Clone, Deserialize)]
pub struct InspectedImageCase {
    pub id: String,
    pub observation: String,
    pub expect: bool,
    pub holdout: bool,
    pub negative: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MeasurementCase {
    pub id: String,
    pub value_statement: String,
    pub source_excerpt: String,
    pub expect: bool,
    pub holdout: bool,
    pub negative: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ObligationCases {
    pub inspected_image: Vec<InspectedImageCase>,
    pub measurement_not_invention: Vec<MeasurementCase>,
}

pub fn sufficiency_cases() -> Vec<SufficiencyCase> {
    serde_json::from_str(SUFFICIENCY_CASES).expect("sufficiency cases")
}

pub fn mature_cases() -> Vec<MatureCase> {
    serde_json::from_str(MATURE_CASES).expect("mature size cases")
}

pub fn ranking_cases() -> Vec<RankingCase> {
    serde_json::from_str(RANKING_CASES).expect("ranking cases")
}

pub fn described_cases() -> Vec<DescribedCase> {
    serde_json::from_str(DESCRIBED_CASES).expect("described cases")
}

pub fn obligation_cases() -> ObligationCases {
    serde_json::from_str(OBLIGATION_CASES).expect("obligation cases")
}

/// The state the data-quality gate lays out: the requirement, the screened
/// evidence, and the counts code owns.
pub fn sufficiency_state(case: &SufficiencyCase) -> Value {
    json!({
        "requirement": case.requirement,
        "evidence": case.evidence,
        "counts": case.counts,
    })
}

pub fn mature_state(case: &MatureCase) -> Value {
    json!({
        "requirement": case.requirement,
        "evidence": case.evidence,
        "counts": case.counts,
    })
}

pub fn ranking_state(case: &RankingCase) -> Value {
    let candidates: Vec<Value> = case
        .candidates
        .iter()
        .map(|candidate| {
            json!({
                "id": candidate.id,
                "title": candidate.title,
                "snippet": candidate.snippet,
                "kind": candidate.kind,
            })
        })
        .collect();
    json!({ "requirement": case.requirement, "candidates": candidates })
}

pub fn described_state(case: &DescribedCase) -> Value {
    json!({ "trait": case.trait_name, "sentences": case.sentences })
}

pub fn inspected_image_state(observation: &str) -> Value {
    json!({ "observation": observation })
}

pub fn measurement_state(value_statement: &str, source_excerpt: &str) -> Value {
    json!({ "value_statement": value_statement, "source_excerpt": source_excerpt })
}

/// The ids of the cases a set answered wrongly, for the miss report.
pub fn missed_ids(set: &crate::cases::SetScore) -> Vec<String> {
    set.rows
        .iter()
        .filter(|row| !row.hit)
        .map(|row| row.id.clone())
        .collect()
}
