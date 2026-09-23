//! fn-133: the palm's last four literature stops, from its rerun on fn-132
//! (`fn-80` branch, `.flow/evidence/date-palm/pipeline/`). The screen rows
//! are `fixtures/palm/fn132-screen-rows.json`; the transport answers with
//! the live run's probabilities where the test names them. No sources are
//! fetched, so every appearance trait reads unstated. No network.

use std::fs;
use std::path::PathBuf;

use serde_json::{json, Value};
use telperion_jev::caller::{HttpRequest, HttpResponse, Transport};
use telperion_jev::pipeline::canon::{read_json, write_canonical};
use telperion_jev::pipeline::floors::{calibrated, selection_floor, FloorCase};
use telperion_jev::pipeline::judge::Judge;
use telperion_jev::pipeline::manifest::Sufficiency;
use telperion_jev::pipeline::requirements::table;
use telperion_jev::pipeline::stage::{Context, Paths};
use telperion_jev::pipeline::stages::{inputs, quality, select};

/// The level probabilities and gap probabilities Jev gave each field.
fn judged(field: &str) -> (Value, Value) {
    let level = |i: usize| {
        let p: serde_json::Map<String, Value> = (0..4)
            .map(|k| (k.to_string(), json!(if k == i { 0.9 } else { 0.03 })))
            .collect();
        Value::Object(p)
    };
    match field {
        // Live, ledger 1b73e1-2: proxy_only at 0.7, `no_mature_size` 0.44
        // over `bound_only` 0.34.
        "dbh_m" => (
            json!({"0": 0.07, "1": 0.7, "2": 0.14, "3": 0.09}),
            json!({"bound_only": 0.34, "no_mature_size": 0.44, "none": 0.03,
                   "single_source": 0.09, "wrong_taxon": 0.1}),
        ),
        "height_m" => (level(1), json!({"bound_only": 0.8, "none": 0.2})),
        "leaflet_length_m" | "leaflet_width_m" => (level(3), json!({"none": 0.9})),
        _ => (level(2), json!({"single_source": 0.9})),
    }
}

/// The span each leaflet field's pick lands on live, with its confidence
/// and probability: both below the 0.34 floor (fn-131).
fn picked(field: &str) -> Option<(&'static str, &'static str, f64, f64)> {
    match field {
        "leaflet_length_m" => Some(("A1.", "18 inches", 0.17, 0.22)),
        "leaflet_width_m" => Some(("P4.", "2cm", 0.19, 0.24)),
        _ => None,
    }
}

fn top(probabilities: &Value) -> String {
    let map = probabilities.as_object().unwrap();
    let best = map
        .iter()
        .max_by(|a, b| a.1.as_f64().unwrap().total_cmp(&b.1.as_f64().unwrap()));
    best.unwrap().0.clone()
}

struct Palm;

impl Transport for Palm {
    fn send(&self, request: &HttpRequest) -> Result<HttpResponse, String> {
        let body: Value = serde_json::from_slice(request.body.as_deref().unwrap_or(b"{}")).unwrap();
        let questions = &body["questions"];
        let field = body["state"]["requirement"]["field"].as_str().unwrap_or("");
        let (levels, gaps) = judged(field);
        let score = json!({"type": "score", "score": top(&levels).parse::<f64>().unwrap(),
                           "confidence": 0.6, "probabilities": levels});
        let gap = json!({"type": "choice", "choice": top(&gaps), "confidence": 0.3, "probabilities": gaps});
        let answers = if questions.get("mature_size").is_some() {
            json!({"mature_size": score, "mature_gap": gap})
        } else if questions.get("growth_rate").is_some() {
            json!({"growth_rate": score, "rate_gap": gap})
        } else if questions.get("span").is_some() {
            let asked = body["state"]["question"].as_str().unwrap_or("");
            let (chosen, confidence, probability) = match picked(asked) {
                Some((label, text, confidence, probability)) => {
                    let found = body["state"]["candidates"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .filter_map(Value::as_str)
                        .find(|c| c.starts_with(label) && c.contains(text))
                        .expect("the live span is a candidate");
                    (found.to_string(), confidence, probability)
                }
                None => ("none".to_string(), 0.9, 0.9),
            };
            json!({"span": {"type": "choice", "choice": chosen, "confidence": confidence,
                            "probabilities": {chosen.clone(): probability}}})
        } else {
            return Err(format!("unexpected question set: {questions}"));
        };
        Ok(HttpResponse {
            status: 200,
            body: serde_json::to_vec(&json!({"model": "jev-latest", "answers": answers})).unwrap(),
        })
    }
}

/// The palm's manifest at version 2: every field and trait its row asks.
fn manifest() -> Value {
    let form = &table().growth_forms["palm"];
    let fields: Vec<Value> = form
        .fields
        .iter()
        .map(|(name, bar)| json!({"field": name, "condition": "open_grown", "required_ages_years": [], "bar": bar.key(), "question": name}))
        .collect();
    let appearance: Vec<Value> = form
        .appearance
        .iter()
        .map(|name| json!({"trait_name": name, "sources": ["A1"]}))
        .collect();
    let sources: Vec<Value> = ["A1", "F1", "M1", "P4", "P5", "P6", "P7", "P8"]
        .iter()
        .map(|id| json!({"id": id, "url": format!("https://example.test/{id}"), "title": id, "rights": "cited"}))
        .collect();
    json!({
        "schema": "manifest", "schema_version": 2, "species": "date-palm",
        "taxon": {"scientific_name": "Phoenix dactylifera", "common_name": "Date palm", "rank": "species"},
        "context": "mature open-grown", "growth_form": "palm",
        "preset": "date-palm", "profile_id": "date-palm", "seed": 7,
        "sources": sources, "fields": fields, "appearance": appearance,
        "versions": {"question_sets": {"mature_size": 1, "growth_rate": 1}, "tools": {}},
        "model": "jev-latest"
    })
}

fn unique(tag: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "jev-palm-literature-{tag}-{}-{}",
        std::process::id(),
        telperion_jev::ledger::new_entry_id()
    ));
    fs::create_dir_all(&dir).unwrap();
    dir
}

/// Quality then select over the palm's rows; returns the run directory.
fn run(tag: &str) -> PathBuf {
    let dir = unique(tag);
    write_canonical(&dir.join("manifest.json"), &manifest()).unwrap();
    let rows = read_json(
        &PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures/palm/fn132-screen-rows.json"),
    )
    .unwrap();
    for (stage, body) in [
        ("fetch", json!({"sources": {}, "tables": {}})),
        ("screen", json!({"rows": rows["rows"]})),
    ] {
        let (ctx, _) = Context::open(&Paths::new(&dir), stage).unwrap();
        let header = ctx.header(stage, stage, inputs(&[]), vec![]);
        ctx.write(&header, body).unwrap();
    }
    let judge = Judge {
        transport: &Palm,
        key: "test-key",
        ledger_dir: unique(&format!("{tag}-ledger")),
    };
    quality::run(&Paths::new(&dir), &judge).unwrap();
    select::run(&Paths::new(&dir), &judge).unwrap();
    dir
}

fn decision_ids(dir: &std::path::Path) -> Vec<String> {
    read_json(&dir.join("decisions.json")).unwrap()["decisions"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|d| d["status"] == "open")
        .map(|d| d["id"].as_str().unwrap().to_string())
        .collect()
}

/// R1: the palm's trunk diameter, `proxy_only` on 3 points against a
/// `proxy_only` bar, failed live on `no_mature_size`. A field with a point
/// never carries that gap; the most probable stated gap is its own.
#[test]
fn a_proxy_only_trunk_diameter_with_points_passes_its_proxy_only_bar() {
    let dir = run("r1");
    let dbh = &read_json(&dir.join("quality.json")).unwrap()["body"]["fields"]["dbh_m"];
    assert_eq!(dbh["level"], "proxy_only", "{dbh}");
    assert_eq!(dbh["points"].as_array().unwrap().len(), 3, "{dbh}");
    assert_eq!(dbh["dominant_gap"], "bound_only", "{dbh}");
    assert_eq!(dbh["passed"], true, "{dbh}");
    let ids = decision_ids(&dir);
    assert!(
        !ids.iter()
            .any(|d| d.starts_with("date-palm/quality/") && d.ends_with("/dbh_m")),
        "{ids:?}"
    );
}

/// R2: the palm asks its mature height at `proxy_only`, a bound like its
/// trunk diameter; the live `proxy_only` height passes.
#[test]
fn the_palm_asks_its_mature_height_at_proxy_only() {
    let palm = &table().growth_forms["palm"];
    assert_eq!(palm.fields.get("height_m"), Some(&Sufficiency::ProxyOnly));
    for form in ["broadleaf", "conifer"] {
        let row = &table().growth_forms[form];
        assert_eq!(
            row.fields.get("height_m"),
            Some(&Sufficiency::Partial),
            "{form}"
        );
    }
    let dir = run("r2");
    let height = &read_json(&dir.join("quality.json")).unwrap()["body"]["fields"]["height_m"];
    assert_eq!(height["passed"], true, "{height}");
    let ids = decision_ids(&dir);
    assert!(
        !ids.contains(&"date-palm/quality/requirements-unmet/height_m".to_string()),
        "{ids:?}"
    );
}

fn case(probability: f64, correct: bool) -> FloorCase {
    FloorCase {
        id: String::new(),
        asked: String::new(),
        picked: String::new(),
        probability,
        correct,
    }
}

/// R3: the labelled set holds fewer than 20 cases or 5 wrong picks, so no
/// selection floor applies; select fills the leaflets from their most
/// probable span and records that span's probability.
#[test]
fn below_the_calibration_minimum_select_fills_the_leaflets_and_records_the_probability() {
    assert_eq!(selection_floor(), None, "the shipped set is not calibrated");
    let wrong = |n: usize| (0..20).map(move |i| case(0.5, i >= n)).collect::<Vec<_>>();
    assert!(calibrated(&wrong(5)));
    assert!(!calibrated(&wrong(4)));
    assert!(!calibrated(&wrong(5)[..19]));
    let dir = run("r3");
    let body = read_json(&dir.join("select.json")).unwrap()["body"].clone();
    let sidecar = read_json(&dir.join("provenance.json")).unwrap();
    for (name, range, probability) in [
        ("leaflet_length_m", [0.4572, 0.4572], 0.22),
        ("leaflet_width_m", [0.02, 0.02], 0.24),
    ] {
        let pointer = format!("/profiles/0/metrics/{name}");
        let got: Vec<f64> = serde_json::from_value(body["filled"][&pointer]["range"].clone())
            .unwrap_or_else(|_| panic!("{name} unfilled: {}", body["unavailable"]));
        assert!(
            (got[0] - range[0]).abs() < 1e-9 && (got[1] - range[1]).abs() < 1e-9,
            "{name}: {got:?}"
        );
        assert_eq!(
            sidecar["entries"][&pointer]["pick_probability"],
            json!(probability),
            "{name}"
        );
    }
}

/// R4: every source leaves the brightness range unstated: it takes the
/// zero-width level as a default with its reason, not as a sourced value,
/// and files nothing; an unstated colour still files `requirements-unmet`.
#[test]
fn an_unstated_brightness_range_is_zero_width_and_an_unstated_colour_still_stops() {
    let dir = run("r4");
    let body = read_json(&dir.join("select.json")).unwrap()["body"]["appearance"].clone();
    let brightness = &body["leaf_brightness_range"];
    assert_eq!(brightness["level"], "uniform", "{body}");
    assert!(
        brightness["default"]
            .as_str()
            .is_some_and(|r| !r.is_empty()),
        "{body}"
    );
    assert_eq!(brightness["source"], Value::Null);
    let profile = read_json(&dir.join("packet/profile.json")).unwrap();
    let entry = &profile["profiles"][0]["appearance"]["leaf_brightness_range"];
    assert_eq!(entry["level"], "uniform", "{entry}");
    assert_eq!(entry["sources"], json!([]));
    for (feed, range) in entry["ranges"].as_object().unwrap() {
        let [low, high]: [f64; 2] = serde_json::from_value(range.clone()).unwrap();
        assert!(
            low <= 0.0 && 0.0 <= high,
            "{feed}: zero width lies in the level"
        );
    }
    let sidecar = read_json(&dir.join("provenance.json")).unwrap();
    let pointer = "/profiles/0/appearance/leaf_brightness_range";
    assert!(
        sidecar["entries"].get(pointer).is_none(),
        "a default is no sourced value"
    );
    assert_eq!(
        sidecar["defaults"][pointer]["level"], "uniform",
        "{sidecar}"
    );
    let ids = decision_ids(&dir);
    let unmet = |name: &str| format!("date-palm/select/requirements-unmet/{name}");
    assert!(!ids.contains(&unmet("leaf_brightness_range")), "{ids:?}");
    assert!(!ids.contains(&unmet("leaf_hue_range")), "{ids:?}");
    assert!(ids.contains(&unmet("leaf_front_colour")), "{ids:?}");
    assert!(ids.contains(&unmet("bark_roughness")), "{ids:?}");
}
