//! fn-132: a palm's size is its growth rate and mature range. The screen
//! rows are the date palm's from the fn-80 live run after fn-131
//! (`fixtures/palm/fn132-screen-rows.json`). The transport answers each
//! field's level and gap as the test names them and picks the span the test
//! names; code lays out the evidence, gates on the level and parses the
//! numbers. No network.

use std::fs;
use std::path::PathBuf;

use serde_json::{json, Value};
use telperion_jev::caller::{HttpRequest, HttpResponse, Transport};
use telperion_jev::pipeline::canon::{read_json, write_canonical};
use telperion_jev::pipeline::judge::Judge;
use telperion_jev::pipeline::manifest::{validate, Manifest, Sufficiency};
use telperion_jev::pipeline::requirements::table;
use telperion_jev::pipeline::sets::{rate_cases, rate_questions, SUFFICIENCY_LEVELS};
use telperion_jev::pipeline::stage::{Context, Paths};
use telperion_jev::pipeline::stages::{inputs, quality, select};

const RATE: &str = "height_growth_m_per_year";

/// Each field's level, its gap and the span select is handed, as
/// `(source label prefix, text the span carries)`.
fn judged(field: &str) -> (usize, &'static str, (&'static str, &'static str)) {
    match field {
        // As Jev answered the leaflets live: sufficient, yet no mature size.
        "leaflet_length_m" | "leaflet_width_m" => (3, "no_mature_size", ("P4.", "30")),
        RATE => (3, "none", ("A1.", "30-45 cm")),
        "height_m" => (2, "single_source", ("P8.", "15 to 25 m")),
        // The grammar reads no "\u{bd}": the same sentence's inches are its span.
        "dbh_m" => (1, "bound_only", ("A1.", "18 inches")),
        _ => (2, "single_source", ("", "")),
    }
}

fn score(index: usize) -> Value {
    let probabilities: serde_json::Map<String, Value> = (0..4)
        .map(|i| (i.to_string(), json!(if i == index { 0.9 } else { 0.03 })))
        .collect();
    json!({"type": "score", "score": index as f64, "confidence": 0.9, "probabilities": probabilities})
}

fn choice(key: &str) -> Value {
    json!({"type": "choice", "choice": key, "confidence": 0.9, "probabilities": {key: 0.9}})
}

struct Palm;

impl Transport for Palm {
    fn send(&self, request: &HttpRequest) -> Result<HttpResponse, String> {
        let body: Value = serde_json::from_slice(request.body.as_deref().unwrap_or(b"{}")).unwrap();
        let questions = &body["questions"];
        let field = body["state"]["requirement"]["field"].as_str().unwrap_or("");
        let answers = if questions.get("mature_size").is_some() {
            let (level, gap, _) = judged(field);
            json!({"mature_size": score(level), "mature_gap": choice(gap)})
        } else if questions.get("growth_rate").is_some() {
            let (level, gap, _) = judged(field);
            json!({"growth_rate": score(level), "rate_gap": choice(gap)})
        } else if questions.get("span").is_some() {
            let asked = body["state"]["question"].as_str().unwrap_or("");
            let (_, _, (label, text)) = judged(asked);
            let chosen = body["state"]["candidates"]
                .as_array()
                .unwrap()
                .iter()
                .filter_map(Value::as_str)
                .find(|c| c.starts_with(label) && c.contains(text))
                .unwrap_or("none")
                .to_string();
            json!({"span": choice(&chosen)})
        } else {
            return Err(format!("unexpected question set: {questions}"));
        };
        Ok(HttpResponse {
            status: 200,
            body: serde_json::to_vec(&json!({"model": "jev-latest", "answers": answers})).unwrap(),
        })
    }
}

/// The palm's manifest at version 2, every field the row requires with no
/// required age: the row asks none of them at an age. Each field's
/// question is its own name, so the span transport knows the field.
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

fn prepare(tag: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "jev-palm-size-{tag}-{}-{}",
        std::process::id(),
        telperion_jev::ledger::new_entry_id()
    ));
    fs::create_dir_all(&dir).unwrap();
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
    dir
}

fn judge(tag: &str) -> Judge<'static> {
    let ledger_dir = std::env::temp_dir().join(format!(
        "jev-palm-size-ledger-{tag}-{}-{}",
        std::process::id(),
        telperion_jev::ledger::new_entry_id()
    ));
    fs::create_dir_all(&ledger_dir).unwrap();
    Judge {
        transport: &Palm,
        key: "test-key",
        ledger_dir,
    }
}

/// R1: the palm's leaflet rows, `sufficient` on 7 points, passed false on
/// `no_mature_size` live. A level that says the size is stated is never
/// failed on a gap that says it is not.
#[test]
fn a_sufficient_mature_field_passes_and_carries_no_mature_size_gap() {
    let dir = prepare("r1");
    let quality::Outcome::Ran { decisions } =
        quality::run(&Paths::new(&dir), &judge("r1")).unwrap()
    else {
        panic!("quality ran")
    };
    let fields = &read_json(&dir.join("quality.json")).unwrap()["body"]["fields"];
    for name in ["leaflet_length_m", "leaflet_width_m"] {
        let field = &fields[name];
        assert_eq!(field["level"], "sufficient", "{name}");
        assert_eq!(field["points"].as_array().unwrap().len(), 7, "{name}");
        assert_eq!(field["passed"], true, "{name}");
        assert_ne!(field["dominant_gap"], "no_mature_size", "{name}");
        assert!(
            !decisions.iter().any(|d| d.ends_with(name)),
            "{name}: {decisions:?}"
        );
    }
}

/// R2: the palm row asks a growth rate and mature height and trunk
/// diameter; A1's and P5's rate sentences are the rate's evidence, P8's
/// range fills the mature height, A1's "up to ½ m" the trunk diameter.
#[test]
fn the_palm_is_sized_by_its_growth_rate_and_its_mature_ranges() {
    let palm = &table().growth_forms["palm"];
    assert_eq!(palm.fields.get(RATE), Some(&Sufficiency::Partial));
    let dir = prepare("r2");
    let paths = Paths::new(&dir);
    let judge = judge("r2");
    quality::run(&paths, &judge).unwrap();
    let fields = &read_json(&dir.join("quality.json")).unwrap()["body"]["fields"];
    let sources = |name: &str| -> Vec<String> {
        fields[name]["points"]
            .as_array()
            .unwrap()
            .iter()
            .map(|p| p["source"].as_str().unwrap().to_string())
            .collect()
    };
    assert_eq!(sources(RATE), ["A1", "P5"], "the two rate sentences");
    assert!(sources("height_m").contains(&"P8".to_string()));
    for name in [RATE, "height_m", "dbh_m"] {
        assert_eq!(fields[name]["passed"], true, "{name}: {}", fields[name]);
        assert_eq!(fields[name]["required_ages_uncovered"], json!([]), "{name}");
    }
    select::run(&paths, &judge).unwrap();
    let body = read_json(&dir.join("select.json")).unwrap()["body"].clone();
    let metric = |name: &str| {
        let got = body["filled"][format!("/profiles/0/metrics/{name}")].clone();
        assert!(got.is_object(), "{name} unfilled: {}", body["unavailable"]);
        got
    };
    let close = |got: &Value, want: [f64; 2]| {
        let got: Vec<f64> = serde_json::from_value(got.clone()).unwrap();
        (got[0] - want[0]).abs() < 1e-9 && (got[1] - want[1]).abs() < 1e-9
    };
    let rate = metric(RATE);
    assert!(close(&rate["range"], [0.30, 0.45]), "{rate}");
    assert_eq!(rate["unit"], "m/yr");
    assert_eq!(rate["source"], json!(["A1"]));
    let height = metric("height_m");
    assert!(close(&height["range"], [15.0, 25.0]), "{height}");
    assert_eq!(height["source"], json!(["P8"]));
    let dbh = metric("dbh_m");
    assert!(close(&dbh["range"], [0.4572, 0.4572]), "{dbh}");
    assert_eq!(dbh["source"], json!(["A1"]));
}

/// R2: broadleaf and conifer keep their bars. Since fn-149 (host,
/// 2026-09-25) they ask height and trunk diameter mature too, as the palm
/// does: none of the three refuses a size named at no age.
#[test]
fn broadleaf_and_conifer_rows_ask_sizes_mature_at_the_proxy_bar() {
    let bars = |pairs: &[(&str, Sufficiency)]| -> Vec<(String, Sufficiency)> {
        pairs.iter().map(|(n, b)| (n.to_string(), *b)).collect()
    };
    let proxy = Sufficiency::ProxyOnly;
    let rows = [
        (
            "broadleaf",
            bars(&[
                ("crown_base_m", proxy),
                ("crown_width_m", proxy),
                ("dbh_m", proxy),
                ("height_m", proxy),
                ("leaf_length_m", proxy),
                ("leaf_width_m", proxy),
            ]),
        ),
        (
            "conifer",
            bars(&[
                ("crown_base_m", proxy),
                ("crown_width_m", proxy),
                ("dbh_m", proxy),
                ("height_m", proxy),
                ("needle_length_m", proxy),
            ]),
        ),
    ];
    for (form, want) in rows {
        let got: Vec<(String, Sufficiency)> = table().growth_forms[form]
            .fields
            .iter()
            .map(|(n, b)| (n.clone(), *b))
            .collect();
        assert_eq!(got, want, "{form}");
    }
    let refusal = |form: &str| {
        let mut value = manifest();
        value["growth_form"] = json!(form);
        let manifest: Manifest = serde_json::from_value(value).unwrap();
        validate(&manifest).err().map(|e| e.to_string())
    };
    for form in ["broadleaf", "conifer"] {
        // The palm's fields fall short of these rows, never for want of an age.
        let err = refusal(form).unwrap_or_default();
        assert!(!err.contains("names no required age"), "{form}: {err}");
    }
    assert_eq!(refusal("palm"), None);
}

/// R3: the growth-rate set asks no age, answers on the sufficiency levels,
/// offers a no-match gap and is labelled on the palm's rate sentences; the
/// runner in `sets.rs` scores it labelled and held out.
#[test]
fn the_rate_set_is_labelled_on_the_palms_rate_sentences_with_a_no_match_answer() {
    let gaps = rate_questions()["rate_gap"]["criteria"]
        .as_object()
        .unwrap()
        .keys()
        .cloned()
        .collect::<Vec<_>>();
    assert!(gaps.contains(&"none".to_string()), "a no-match gap");
    let cases = rate_cases();
    for case in &cases {
        assert!(
            SUFFICIENCY_LEVELS.contains(&case.expect_level.as_str()),
            "{}",
            case.id
        );
        assert!(gaps.contains(&case.expect_gap), "{}", case.id);
        assert!(
            case.requirement.get("required_ages_years").is_none(),
            "{}",
            case.id
        );
        assert_eq!(case.requirement["field"], RATE, "{}", case.id);
    }
    for id in ["palm-rate-a1", "palm-rate-p5", "palm-rate-a1-p5"] {
        assert!(
            cases.iter().any(|c| c.id == id),
            "the palm's rate sentences label {id}"
        );
    }
    assert!(cases.iter().any(|c| c.expect_level == "none" && c.negative));
}
