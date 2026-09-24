//! fn-131 R1, R2, R3 and R7 on select, over the palm's rows (`judged`).

mod common;
mod judged;

use std::path::PathBuf;

use judged::*;
use serde_json::{json, Value};
use telperion_jev::caller::{HttpRequest, HttpResponse, Transport};
use telperion_jev::pipeline::canon::{read_json, write_canonical};
use telperion_jev::pipeline::decision::{append_decisions, Decision, DecisionParts};
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

/// R2 and R7: until its labelled set is calibrated no selection floor
/// applies (fn-133), so a pick below 0.34 fills the field; a required field
/// select fills no value for files `requirements-unmet`.
#[test]
fn an_uncalibrated_floor_fills_the_pick_and_an_unfilled_required_field_is_filed() {
    let dir = palm("floor");
    let mut palm = Palm::new(frond_from_f1);
    palm.confidence = telperion_jev::thresholds().selection_floor - 0.01;
    let body = quality_then_select(&dir, &palm);
    assert_eq!(
        body["filled"]["/profiles/0/metrics/frond_length_m"]["source"],
        json!(["F1"]),
        "{body}"
    );
    let list = read_json(&dir.join("decisions.json")).unwrap();
    let filed = list["decisions"]
        .as_array()
        .unwrap()
        .iter()
        .find(|d| d["id"] == "date-palm/select/requirements-unmet/leaflet_length_m")
        .expect("select files the unfilled required field");
    assert_eq!(filed["status"], "open");
    assert_eq!(filed["options"], json!(["add-sources"]));
}

/// R3: the live palm's level probabilities. The rounded average put
/// `leaf_back_colour` on `silvery_white` and `leaf_brightness_range` on
/// `strongly_varied`, each at probability 0; the most probable is
/// `unstated` for both. An unstated brightness range reads zero width
/// (fn-133); an unstated back with `leaf_front_colour` sourced to
/// `grey_green` in this same fixture takes the front's level as a default
/// instead of staying unstated (fn-139) - never the rounded-average trap.
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
    // The argmax picked `unstated` (0.54), never `silvery_white` (the level
    // the rounded average of 3.06 would land on, at probability 0); with
    // the front sourced to grey_green the back defaults to it (fn-139).
    assert_eq!(body["leaf_back_colour"]["level"], "grey_green", "{body}");
    assert!(body["leaf_back_colour"]["default"].is_string(), "{body}");
    assert_eq!(body["leaf_brightness_range"]["level"], "uniform", "{body}");
    assert!(
        body["leaf_brightness_range"]["default"].is_string(),
        "{body}"
    );
    assert_eq!(body["leaf_front_colour"]["level"], "grey_green", "{body}");
}

/// fn-139 R1: the palm's shape - front sourced `grey_green` from F1, back
/// unstated by score. The back takes the front's level, its ranges mapped
/// onto the back's material fields, as a default citing F1; no
/// requirements-unmet is filed for it.
#[test]
fn an_unstated_back_with_a_sourced_front_defaults_to_the_fronts_level() {
    let dir = fetched_f1("back-default");
    let mut palm = Palm::new(|_, _| None);
    palm.levels = |name| match name {
        "leaf_front_colour" => Some(score(
            json!({"0": 0.0, "1": 0.0, "2": 0.0, "3": 1.0, "4": 0.0, "5": 0.0}),
            3.0,
        )),
        _ => None,
    };
    select::run(&Paths::new(&dir), &judge(&palm)).unwrap();
    let select = read_json(&dir.join("select.json")).unwrap();
    let body = &select["body"]["appearance"];
    assert_eq!(body["leaf_front_colour"]["level"], "grey_green", "{body}");
    assert_eq!(body["leaf_back_colour"]["level"], "grey_green", "{body}");
    assert_eq!(body["leaf_back_colour"]["source"], "F1", "{body}");
    assert!(body["leaf_back_colour"]["default"].is_string(), "{body}");

    let profile = read_json(&dir.join("packet").join("profile.json")).unwrap();
    let back = &profile["profiles"][0]["appearance"]["leaf_back_colour"];
    assert_eq!(back["level"], "grey_green", "{back}");
    assert_eq!(back["sources"], json!(["F1"]), "{back}");
    assert!(back["default"].is_string(), "{back}");
    let ranges = back["ranges"].as_object().unwrap();
    for field in ["leaf_back_red", "leaf_back_green", "leaf_back_blue"] {
        assert!(ranges.contains_key(field), "{back}");
    }
    // Not the front's own ranges relabelled with the front's field names.
    assert!(!ranges.contains_key("leaf_front_red"), "{back}");

    let list = read_json(&dir.join("decisions.json")).unwrap();
    let decisions = list["decisions"].as_array().unwrap();
    assert!(
        decisions
            .iter()
            .all(|d| d["id"] != "date-palm/select/requirements-unmet/leaf_back_colour"),
        "{list}"
    );
}

/// fn-139 R2: both faces unstated still file requirements-unmet for each,
/// and bark colour - which has no zero-width level either - never defaults
/// to anything.
#[test]
fn both_faces_unstated_file_requirements_unmet_and_bark_never_defaults() {
    let dir = fetched_f1("both-unstated");
    let palm = Palm::new(|_, _| None);
    select::run(&Paths::new(&dir), &judge(&palm)).unwrap();
    let select = read_json(&dir.join("select.json")).unwrap();
    let body = &select["body"]["appearance"];
    assert_eq!(body["leaf_front_colour"]["level"], "unstated", "{body}");
    assert_eq!(body["leaf_back_colour"]["level"], "unstated", "{body}");
    assert!(body["leaf_back_colour"]["default"].is_null(), "{body}");
    assert_eq!(body["bark_colour"]["level"], "unstated", "{body}");
    assert!(body["bark_colour"]["default"].is_null(), "{body}");

    let list = read_json(&dir.join("decisions.json")).unwrap();
    let ids: Vec<&str> = list["decisions"]
        .as_array()
        .unwrap()
        .iter()
        .map(|d| d["id"].as_str().unwrap())
        .collect();
    for field in ["leaf_front_colour", "leaf_back_colour", "bark_colour"] {
        let id = format!("date-palm/select/requirements-unmet/{field}");
        assert!(ids.contains(&id.as_str()), "{ids:?}");
    }
}

/// fn-139 R4 (host design): a back a claim resolution dropped, with no
/// replacement source found on the rerun, is an unstated one - it takes the
/// still-sourced front's level the same as a back that was never sourced at
/// all (R1). Front sourced `grey_green` from F1; back is first sourced
/// (wrongly) to `silvery_white` off the same F1 sentence, then a
/// `drop-value` resolution against that exact source and span takes it out.
#[test]
fn a_back_a_resolution_dropped_with_no_replacement_defaults_to_the_sourced_front() {
    let dir = fetched_f1("back-dropped");
    let mut palm = Palm::new(|_, _| None);
    palm.levels = |name| match name {
        "leaf_front_colour" => Some(score(
            json!({"0": 0.0, "1": 0.0, "2": 0.0, "3": 1.0, "4": 0.0, "5": 0.0}),
            3.0,
        )),
        "leaf_back_colour" => Some(score(json!({"0": 0.0, "1": 0.0, "2": 0.0, "3": 1.0}), 3.0)),
        _ => None,
    };
    let paths = Paths::new(&dir);
    select::run(&paths, &judge(&palm)).unwrap();

    let back_pointer = "/profiles/0/appearance/leaf_back_colour";
    let sidecar = read_json(&dir.join("provenance.json")).unwrap();
    let entry = sidecar["entries"][back_pointer].clone();
    // Sanity: the back really was sourced first, off F1's frond sentence,
    // before anything drops it - never plain-unstated (that's R1).
    assert_eq!(entry["level"], "silvery_white", "{entry}");
    assert_eq!(entry["source"], "F1", "{entry}");
    assert_eq!(entry["span"], F1_FROND, "{entry}");

    let claim = Decision::new(
        DecisionParts {
            species: "date-palm",
            stage: "verify",
            kind: "claim-unsupported",
            field: Some(back_pointer),
            age_years: None,
        },
        &["generate"],
        [("select.json".to_string(), "test-fixture".to_string())]
            .into_iter()
            .collect(),
        vec![],
        json!({"value": back_pointer, "level": entry["level"], "sentence": entry["span"],
               "source": entry["source"], "pointer": back_pointer, "span": entry["span"]}),
        &["accept", "replace-source", "drop-value"],
        "Jev judged that the cited sentence does not describe this appearance level.",
    );
    let (id, inputs) = (claim.id.clone(), claim.inputs_sha256.clone());
    append_decisions(&paths.decisions(), vec![claim]).unwrap();
    write_canonical(
        &paths.resolutions(),
        &json!({"resolutions": [{"id": id, "inputs_sha256": inputs,
                                 "option": "drop-value", "by": "owner", "at": "2026-09-24"}]}),
    )
    .unwrap();

    // No replacement source was added: the rerun re-derives the same
    // flagged (source, span), so the resolution's drop actually applies.
    select::run(&paths, &judge(&palm)).unwrap();
    let body = &read_json(&dir.join("select.json")).unwrap()["body"]["appearance"];
    assert_eq!(body["leaf_back_colour"]["level"], "grey_green", "{body}");
    assert_eq!(body["leaf_back_colour"]["source"], "F1", "{body}");
    assert!(body["leaf_back_colour"]["default"].is_string(), "{body}");
    assert_eq!(body["leaf_front_colour"]["level"], "grey_green", "{body}");

    let profile = read_json(&dir.join("packet").join("profile.json")).unwrap();
    let back = &profile["profiles"][0]["appearance"]["leaf_back_colour"];
    assert_eq!(back["level"], "grey_green", "{back}");
    assert_eq!(back["sources"], json!(["F1"]), "{back}");

    let list = read_json(&dir.join("decisions.json")).unwrap();
    let decisions = list["decisions"].as_array().unwrap();
    assert!(
        decisions
            .iter()
            .all(|d| d["id"] != "date-palm/select/requirements-unmet/leaf_back_colour"),
        "{list}"
    );
    let consumed = decisions.iter().find(|d| d["id"] == id).unwrap();
    assert_eq!(consumed["consumed_by"], "select", "{consumed}");
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
