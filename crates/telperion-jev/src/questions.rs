//! Versioned question sets and the thresholds chosen on the labelled set.

use serde::Deserialize;
use serde_json::{json, Map, Value};

pub const SCREEN_JSON: &str = include_str!("../data/questions/screen.json");
pub const CITATION_JSON: &str = include_str!("../data/questions/citation.json");
pub const TRIAGE_JSON: &str = include_str!("../data/questions/triage.json");
pub const THRESHOLDS_JSON: &str = include_str!("../data/thresholds.json");
pub const SCREEN_CASES: &str = include_str!("../data/cases/screen.json");
pub const CITATION_CASES: &str = include_str!("../data/cases/citation.json");
pub const SELECTION_CASES: &str = include_str!("../data/cases/selection.json");
pub const TRIAGE_CASES: &str = include_str!("../data/cases/triage.json");

#[derive(Debug, Clone, Deserialize)]
pub struct Thresholds {
    pub citation_auto_accept: f64,
    pub duplicate_same_defect: f64,
    pub route_min_probability: f64,
    pub selection_spread_confidence: f64,
    pub anchor_usable: f64,
    pub severity_cosmetic: f64,
    pub severity_noticeable: f64,
}

pub fn thresholds() -> Thresholds {
    serde_json::from_str(THRESHOLDS_JSON).expect("thresholds.json")
}

pub fn screen_questions() -> Value {
    let raw: Value = serde_json::from_str(SCREEN_JSON).expect("screen.json");
    json!({
        "kind": raw["kind"],
        "condition": raw["condition"],
        "anchor_usable": raw["anchor_usable"],
    })
}

pub fn citation_questions() -> Value {
    let raw: Value = serde_json::from_str(CITATION_JSON).expect("citation.json");
    json!({ "relation": raw["relation"] })
}

pub fn triage_spec_questions(open_specs: &Map<String, Value>) -> Value {
    let raw: Value = serde_json::from_str(TRIAGE_JSON).expect("triage.json");
    let mut criteria = Map::new();
    for (id, summary) in open_specs {
        criteria.insert(id.clone(), summary.clone());
    }
    criteria.insert(
        "new_spec".into(),
        Value::String(
            raw["spec"]["new_spec"]
                .as_str()
                .unwrap_or("No open spec covers this defect; it needs a new spec")
                .to_string(),
        ),
    );
    json!({
        "spec": {
            "type": "choice",
            "instructions": raw["spec"]["instructions"],
            "criteria": criteria,
        }
    })
}

pub fn same_defect_questions() -> Value {
    let raw: Value = serde_json::from_str(TRIAGE_JSON).expect("triage.json");
    json!({ "same_defect": raw["same_defect"] })
}

pub fn severity_questions() -> Value {
    let raw: Value = serde_json::from_str(TRIAGE_JSON).expect("triage.json");
    json!({ "severity": raw["severity"] })
}

/// Selection Choice: every extracted span plus `none`.
pub fn selection_questions(question: &str, spans: &[String]) -> Value {
    let mut criteria = Map::new();
    for span in spans {
        criteria.insert(span.clone(), Value::Null);
    }
    criteria.insert("none".into(), json!("No candidate span states this"));
    json!({
        "span": {
            "type": "choice",
            "instructions": question,
            "criteria": criteria,
        }
    })
}

#[derive(Debug, Clone, Deserialize)]
pub struct ScreenCase {
    pub id: String,
    pub source_id: String,
    pub sentence: String,
    pub expect_kind: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CitationCase {
    pub id: String,
    pub claim: String,
    pub section: String,
    pub expect_relation: String,
    pub true_claim: bool,
    pub height_at_age: bool,
    #[serde(default)]
    pub expect_kind: Option<String>,
    #[serde(default)]
    pub expect_confidence: Option<f64>,
    #[serde(default)]
    pub list_for_owner: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SelectionCase {
    pub id: String,
    pub document: String,
    pub question: String,
    pub expect_span: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TriageCases {
    pub open_specs: Map<String, Value>,
    pub routes: Vec<RouteCase>,
    pub duplicates: Vec<DuplicateCase>,
    pub severity: Vec<SeverityCase>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RouteCase {
    pub id: String,
    pub observation: String,
    pub expect: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DuplicateCase {
    pub id: String,
    pub new_observation: String,
    pub prior_finding: String,
    pub true_pair: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SeverityCase {
    pub id: String,
    pub observation: String,
    pub standard: String,
    pub expect_level: String,
}

pub fn screen_cases() -> Vec<ScreenCase> {
    serde_json::from_str(SCREEN_CASES).expect("screen cases")
}

pub fn citation_cases() -> Vec<CitationCase> {
    serde_json::from_str(CITATION_CASES).expect("citation cases")
}

pub fn selection_cases() -> Vec<SelectionCase> {
    serde_json::from_str(SELECTION_CASES).expect("selection cases")
}

pub fn triage_cases() -> TriageCases {
    serde_json::from_str(TRIAGE_CASES).expect("triage cases")
}

pub fn severity_level(score: f64) -> &'static str {
    let cuts = thresholds();
    if score < cuts.severity_cosmetic {
        "cosmetic"
    } else if score < cuts.severity_noticeable {
        "noticeable"
    } else {
        "blocking"
    }
}
