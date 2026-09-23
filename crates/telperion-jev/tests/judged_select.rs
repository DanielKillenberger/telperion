//! fn-131 R1, R2, R3 and R7 on select, over the palm's rows (`judged`).

mod common;
mod judged;

use std::path::PathBuf;

use judged::*;
use serde_json::{json, Value};
use telperion_jev::caller::{HttpRequest, HttpResponse, Transport};
use telperion_jev::pipeline::canon::read_json;
use telperion_jev::pipeline::judge::Judge;
use telperion_jev::pipeline::stage::Paths;
use telperion_jev::pipeline::stages::{quality, select};

/// The candidate from `source` whose span is `span`.
fn from(candidates: &[String], source: &str, span: &str) -> Option<String> {
    candidates
        .iter()
        .find(|c| c.starts_with(source) && c.ends_with(&format!(": {span}")))
        .cloned()
}

fn frond_from_f1(question: &str, candidates: &[String]) -> Option<String> {
    if question.contains("frond_length_m") {
        return from(candidates, "F1", "20 feet");
    }
    if question.contains("crown_width_m") {
        return from(candidates, "P4", "6\u{2013}10m");
    }
    None
}

fn quality_then_select(dir: &PathBuf, palm: &Palm) -> Value {
    let paths = Paths::new(dir);
    quality::run(&paths, &judge(palm)).unwrap();
    select::run(&paths, &judge(palm)).unwrap();
    read_json(&dir.join("select.json")).unwrap()["body"].clone()
}

/// R1: the organ and cultivar classes reach the fields that name them, and
/// the crown's glued "6–10m" parses.
#[test]
fn leaflet_and_frond_sentences_reach_select_and_the_cultivar_fronds_count() {
    let dir = palm("reach");
    let palm = Palm::new(frond_from_f1);
    let body = quality_then_select(&dir, &palm);
    let document = |field: &str| palm.asked(field)["document"].as_str().unwrap().to_string();
    let leaflets = document("leaflet_length_m");
    for sentence in [P4_LEAFLET, P5_LEAFLET, A1_LEAFLET] {
        assert!(leaflets.contains(sentence), "{sentence}\n{leaflets}");
    }
    let fronds = document("frond_length_m");
    for sentence in [P5_FROND, A1_FROND, F1_FROND, M1_CULTIVAR] {
        assert!(fronds.contains(sentence), "{sentence}\n{fronds}");
    }
    // One document per field: the crown is no leaflet, a leaflet no crown.
    assert!(!leaflets.contains(P4_CROWN), "{leaflets}");
    assert!(!document("crown_width_m").contains(P4_LEAFLET));
    let quality = read_json(&dir.join("quality.json")).unwrap()["body"]["fields"].clone();
    let counted: Vec<&str> = quality["frond_length_m"]["points"]
        .as_array()
        .unwrap()
        .iter()
        .map(|p| p["source"].as_str().unwrap())
        .collect();
    assert!(counted.contains(&"M1"), "{counted:?}");
    assert_eq!(
        body["filled"]["/profiles/0/metrics/crown_width_m"]["range"],
        json!([6.0, 10.0])
    );
}

/// R2: frond length is credited to F1's frond sentence, the one its span
/// was chosen from, not the first sentence that contains "20 feet".
#[test]
fn frond_length_is_credited_to_f1s_frond_sentence() {
    let dir = palm("credit");
    let palm = Palm::new(frond_from_f1);
    let body = quality_then_select(&dir, &palm);
    let frond = &body["filled"]["/profiles/0/metrics/frond_length_m"];
    assert_eq!(frond["source"], json!(["F1"]), "{body}");
    assert_eq!(frond["range"], json!([6.096, 6.096]));
    let sidecar = read_json(&dir.join("provenance.json")).unwrap();
    let entry = &sidecar["entries"]["/profiles/0/metrics/frond_length_m"];
    assert_eq!(entry["source"], "F1");
    assert_eq!(entry["sentence"], F1_FROND);
    assert_eq!(entry["span"], "20 feet");
}

/// R2 and R7: a pick below the calibrated floor is not filled, and the
/// required field select could not fill files `requirements-unmet`.
#[test]
fn a_pick_below_the_floor_is_not_filled_and_the_required_field_is_filed() {
    let dir = palm("floor");
    let mut palm = Palm::new(frond_from_f1);
    palm.confidence = telperion_jev::thresholds().selection_floor - 0.01;
    let body = quality_then_select(&dir, &palm);
    assert!(
        body["filled"]
            .get("/profiles/0/metrics/frond_length_m")
            .is_none(),
        "{body}"
    );
    assert_eq!(
        body["unavailable"]["frond_length_m"],
        "pick below the selection floor"
    );
    let list = read_json(&dir.join("decisions.json")).unwrap();
    let filed = list["decisions"]
        .as_array()
        .unwrap()
        .iter()
        .find(|d| d["id"] == "date-palm/select/requirements-unmet/frond_length_m")
        .expect("select files the unfilled required field");
    assert_eq!(filed["status"], "open");
    assert_eq!(filed["options"], json!(["add-sources"]));
}

/// R3: the live palm's level probabilities. The rounded average put
/// `leaf_back_colour` on `silvery_white` and `leaf_brightness_range` on
/// `strongly_varied`, each at probability 0; the most probable is
/// `unstated`.
#[test]
fn a_level_is_the_most_probable_one_never_a_zero_probability_average() {
    let dir = fetched_f1("levels");
    let mut palm = Palm::new(|_, _| None);
    palm.levels = |name| match name {
        "leaf_back_colour" => Some(score(
            json!({"0": 0.0, "1": 0.0, "2": 0.46, "3": 0.0, "4": 0.54}),
            3.06,
        )),
        "leaf_brightness_range" => Some(score(
            json!({"0": 0.17, "1": 0.09, "2": 0.0, "3": 0.74}),
            2.3,
        )),
        "leaf_front_colour" => Some(score(
            json!({"0": 0.0, "1": 0.0, "2": 0.0, "3": 1.0, "4": 0.0, "5": 0.0}),
            3.0,
        )),
        _ => None,
    };
    select::run(&Paths::new(&dir), &judge(&palm)).unwrap();
    let body = &read_json(&dir.join("select.json")).unwrap()["body"]["appearance"];
    assert_eq!(body["leaf_back_colour"]["level"], "unstated", "{body}");
    assert_eq!(body["leaf_brightness_range"]["level"], "unstated", "{body}");
    assert_eq!(body["leaf_front_colour"]["level"], "grey_green", "{body}");
}

/// R3 on the gate: a sufficiency whose average rounds to `proxy_only`, a
/// level Jev gave probability 0, is the most probable level, `none`.
#[test]
fn a_sufficiency_level_is_the_most_probable_one() {
    struct Split;
    impl Transport for Split {
        fn send(&self, request: &HttpRequest) -> Result<HttpResponse, String> {
            let body: Value = serde_json::from_slice(request.body.as_deref().unwrap()).unwrap();
            let (key, gap) = [
                ("mature_size", "mature_gap"),
                ("growth_rate", "rate_gap"),
                ("sufficiency", "dominant_gap"),
            ]
            .into_iter()
            .find(|(k, _)| body["questions"].get(*k).is_some())
            .unwrap();
            let answers = json!({key: score(json!({"0": 0.6, "1": 0.0, "2": 0.0, "3": 0.4}), 1.2),
                                 gap: {"type": "choice", "choice": "single_source", "confidence": 0.9, "probabilities": {}}});
            Ok(HttpResponse {
                status: 200,
                body: serde_json::to_vec(&json!({"model": "jev-latest", "answers": answers}))
                    .unwrap(),
            })
        }
    }
    let dir = palm("sufficiency");
    let judge = Judge {
        transport: &Split,
        key: "test-key",
        ledger_dir: common::ledger_dir("split"),
    };
    quality::run(&Paths::new(&dir), &judge).unwrap();
    let fields = &read_json(&dir.join("quality.json")).unwrap()["body"]["fields"];
    assert_eq!(fields["frond_length_m"]["level"], "none", "{fields}");
    assert_eq!(fields["frond_length_m"]["passed"], false);
}
