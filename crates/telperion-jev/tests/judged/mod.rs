//! The palm's rows as the live run on fn-80 screened them (sentences
//! copied from its cache, 2026-09-23), with the organ-size and cultivar
//! classes the screen now offers, and a transport that answers the
//! mature-size set, the span choice and the level scores the way each test
//! states. Nothing reaches the network.
#![allow(dead_code)]

use super::common;

use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;

use serde_json::{json, Value};
use telperion_jev::caller::{HttpRequest, HttpResponse, Transport};
use telperion_jev::pipeline::adapter::FixtureAdapter;
use telperion_jev::pipeline::canon::write_canonical;
use telperion_jev::pipeline::judge::Judge;
use telperion_jev::pipeline::requirements::table;
use telperion_jev::pipeline::stage::{Context, Paths};
use telperion_jev::pipeline::stages::{fetch, inputs};

pub const P4_LEAFLET: &str = "The leaflets are 30 centimetres (12 inches) long and 2cm (1in) wide.";
pub const P4_CROWN: &str = "The full span of the crown ranges from 6\u{2013}10m (20\u{2013}33ft).";
pub const P5_LEAFLET: &str = "Leaflets are 1-2 ft (0.3-0.6 m) long and arranged in V-shape ranks that run the length of the leaf stem.";
pub const P5_FROND: &str = "Leaves Evergreen; pinnately compound; blue-green to gray-green; to 20 ft (6 m) in length; leaflets are 1-2 ft (0.3-0.6 m) long.";
pub const A1_FROND: &str = "Leaves resemble a \u{2018}feather-duster\u{2019} as fronds are pinnately compound, 5-7 m (15 to 20 feet) long, with armed petioles and stout midribs that hold the slender pinnae (1, 5).";
pub const A1_LEAFLET: &str = "The leaflets are \u{bd} m (18 inches) long (1, 3, 5) and are induplicate, meaning they appear to have been folded in half lengthwise (5).";
pub const A1_RATE: &str = "Date palms have a moderate growth rate of 30-45 cm (1 to 1.5 feet) a year, reaching 5 m (20 feet) in 15 to 20 years depending on the cultivar and soil and water conditions.";
pub const F1_FROND: &str = "The pinnately compound blue-green to gray-green leaves or fronds can grow to 20 feet in length; leaflets are 1 to 2 feet long and form a \"V\" shape down the rachis.";
pub const M1_CULTIVAR: &str = "the range of frond length was from 545.33 cm (Barni Al-Madinah) to 297 cm (Majdool) while the range of frond width was from 106.17 cm (Barni Al-Madinah) to 55.67 cm (Mabroom Al-Ula).";

/// The rows with the classes the new screen gives them.
pub fn rows() -> Value {
    let row = |source: &str, sentence: &str, kind: &str| json!({"source": source, "sentence": sentence, "kind": kind, "condition": "unstated", "anchor_usable": false, "ledger": "live"});
    json!({"rows": [
        row("P4", P4_LEAFLET, "leaflet_size"),
        row("P4", P4_CROWN, "mature_size_range"),
        row("P5", P5_LEAFLET, "leaflet_size"),
        row("P5", P5_FROND, "frond_size"),
        row("A1", A1_FROND, "frond_size"),
        row("A1", A1_LEAFLET, "leaflet_size"),
        row("A1", A1_RATE, "typical_growth_rate"),
        row("F1", F1_FROND, "frond_size"),
        row("M1", M1_CULTIVAR, "cultivar_size"),
    ]})
}

/// The palm's manifest at version 2 over the five sources.
pub fn manifest() -> Value {
    let form = &table().growth_forms["palm"];
    let fields: Vec<Value> = form
        .fields
        .iter()
        .map(|(name, bar)| json!({"field": name, "condition": "open_grown", "required_ages_years": [20.0], "bar": bar.key(), "question": format!("Which span states {name}?")}))
        .collect();
    let appearance: Vec<Value> = form
        .appearance
        .iter()
        .map(|name| json!({"trait_name": name, "sources": ["F1"]}))
        .collect();
    let source = |id: &str| json!({"id": id, "url": format!("https://example.test/{id}"), "title": id, "rights": "cited"});
    json!({
        "schema": "manifest", "schema_version": 2, "species": "date-palm",
        "taxon": {"scientific_name": "Phoenix dactylifera", "common_name": "Date palm", "rank": "species"},
        "context": "mature open-grown", "growth_form": "palm",
        "preset": "date-palm", "profile_id": "date-palm", "seed": 7,
        "sources": (["A1", "F1", "M1", "P4", "P5"].map(source)),
        "fields": fields, "appearance": appearance,
        "versions": {"question_sets": {"sufficiency": 1, "mature_size": 1, "growth_rate": 1}, "tools": {}},
        "model": "jev-latest"
    })
}

/// A pipeline directory with the manifest, an empty fetch and the rows.
pub fn palm(tag: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "jev-judged-{tag}-{}-{}",
        std::process::id(),
        telperion_jev::ledger::new_entry_id()
    ));
    fs::create_dir_all(&dir).unwrap();
    write_canonical(&dir.join("manifest.json"), &manifest()).unwrap();
    for (stage, body) in [
        ("fetch", json!({"sources": {}, "tables": {}})),
        ("screen", rows()),
    ] {
        let (ctx, _) = Context::open(&Paths::new(&dir), stage).unwrap();
        ctx.write(&ctx.header(stage, stage, inputs(&[]), vec![]), body)
            .unwrap();
    }
    dir
}

/// A level Score with its probabilities over level indices.
pub fn score(probabilities: Value, score: f64) -> Value {
    json!({"type": "score", "score": score, "confidence": 0.5, "probabilities": probabilities})
}

/// Jev as each test states it. Every mature field and growth rate is
/// `partial` with no gap, every age-indexed one `none`; each span question is answered by
/// `pick(field, candidates)` at `confidence`.
pub struct Palm {
    pub pick: fn(&str, &[String]) -> Option<String>,
    pub confidence: f64,
    pub spans: Mutex<Vec<Value>>,
    pub levels: fn(&str) -> Option<Value>,
}

impl Palm {
    pub fn new(pick: fn(&str, &[String]) -> Option<String>) -> Self {
        Self {
            pick,
            confidence: 0.9,
            spans: Mutex::new(Vec::new()),
            levels: |_| None,
        }
    }
    /// The span state the select stage laid out for `field`.
    pub fn asked(&self, field: &str) -> Value {
        let spans = self.spans.lock().unwrap();
        spans
            .iter()
            .find(|s| s["question"].as_str().unwrap().contains(field))
            .cloned()
            .unwrap_or_else(|| panic!("no span question for {field}"))
    }
}

impl Transport for Palm {
    fn send(&self, request: &HttpRequest) -> Result<HttpResponse, String> {
        let body: Value = serde_json::from_slice(request.body.as_deref().unwrap_or(b"{}")).unwrap();
        let (questions, state) = (&body["questions"], &body["state"]);
        let answers = if questions.get("mature_size").is_some() {
            json!({"mature_size": score(json!({"0": 0.02, "1": 0.03, "2": 0.9, "3": 0.05}), 2.0),
                   "mature_gap": {"type": "choice", "choice": "none", "confidence": 0.9, "probabilities": {"none": 0.9}}})
        } else if questions.get("growth_rate").is_some() {
            json!({"growth_rate": score(json!({"0": 0.02, "1": 0.03, "2": 0.9, "3": 0.05}), 2.0),
                   "rate_gap": {"type": "choice", "choice": "none", "confidence": 0.9, "probabilities": {"none": 0.9}}})
        } else if questions.get("sufficiency").is_some() {
            json!({"sufficiency": score(json!({"0": 0.9, "1": 0.1, "2": 0.0, "3": 0.0}), 0.1),
                   "dominant_gap": {"type": "choice", "choice": "no_age_indexed_points", "confidence": 0.9, "probabilities": {}}})
        } else if questions.get("span").is_some() {
            self.spans.lock().unwrap().push(state.clone());
            let candidates: Vec<String> =
                serde_json::from_value(state["candidates"].clone()).unwrap();
            let question = state["question"].as_str().unwrap();
            let chosen = (self.pick)(question, &candidates).unwrap_or_else(|| "none".into());
            json!({"span": {"type": "choice", "choice": chosen, "confidence": self.confidence, "probabilities": {}}})
        } else if questions.get("level").is_some() {
            let name = state["trait"].as_str().unwrap();
            let count = table().levels(name).unwrap().len();
            let unstated = count.to_string();
            (self.levels)(name).map_or_else(
                || json!({"level": score(json!({unstated: 1.0}), count as f64)}),
                |answer| json!({"level": answer}),
            )
        } else if let Some(name) = ["appearance_supported", "measurement_not_invention"]
            .into_iter()
            .find(|k| questions.get(*k).is_some())
        {
            json!({name: {"type": "noul", "noul": 0.9}})
        } else {
            return common::CaseTransport.send(request);
        };
        Ok(HttpResponse {
            status: 200,
            body: serde_json::to_vec(&json!({"model": "jev-latest", "answers": answers})).unwrap(),
        })
    }
}

pub fn judge(palm: &Palm) -> Judge<'_> {
    Judge {
        transport: palm,
        key: "test-key",
        ledger_dir: common::ledger_dir("judged"),
    }
}

/// F1 fetched through the fixture adapter, with empty screen and quality.
pub fn fetched_f1(tag: &str) -> PathBuf {
    let root = std::env::temp_dir().join(format!(
        "jev-judged-{tag}-{}-{}",
        std::process::id(),
        telperion_jev::ledger::new_entry_id()
    ));
    let (dir, fixtures) = (root.join("pipeline"), root.join("fixtures"));
    fs::create_dir_all(&dir).unwrap();
    fs::create_dir_all(&fixtures).unwrap();
    fs::write(fixtures.join("f1.md"), format!("{F1_FROND}\n")).unwrap();
    fs::write(fixtures.join("f1.html"), format!("<p>{F1_FROND}</p>")).unwrap();
    let url = "https://example.test/F1";
    write_canonical(
        &fixtures.join("index.json"),
        &json!({"scrape": {url: {"final_url": url, "content_type": "text/html", "raw": "f1.html", "markdown": "f1.md"}}}),
    )
    .unwrap();
    let mut m = manifest();
    m["sources"] = json!([{"id": "F1", "url": url, "title": "F1", "rights": "cited"}]);
    write_canonical(&dir.join("manifest.json"), &m).unwrap();
    let paths = Paths::new(&dir);
    fetch::run(&paths, &FixtureAdapter::new(fixtures)).unwrap();
    for (stage, body) in [
        ("screen", json!({"rows": []})),
        ("quality", json!({"fields": {}})),
    ] {
        let (ctx, _) = Context::open(&paths, stage).unwrap();
        ctx.write(&ctx.header(stage, stage, inputs(&[]), vec![]), body)
            .unwrap();
    }
    dir
}
