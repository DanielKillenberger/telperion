//! fn-149 D: a claim the search could not settle is kept as the range its
//! sources span (`keep-range`), every source cited, or dropped, leaving the
//! field `unsourced` for the generator's default (`drop-value`).

mod common;
mod judged;

use judged::*;
use serde_json::{json, Value};
use telperion_jev::pipeline::canon::{read_json, write_canonical};
use telperion_jev::pipeline::decision::{append_decisions, Decision, DecisionParts};
use telperion_jev::pipeline::stage::Paths;
use telperion_jev::pipeline::stages::flagged::CLAIM_OPTIONS;
use telperion_jev::pipeline::stages::{quality, select};

const FROND: &str = "/profiles/0/metrics/frond_length_m";

fn frond_from_f1(question: &str, candidates: &[String]) -> Option<String> {
    let f1 = |c: &&String| c.starts_with("F1") && c.ends_with(": 20 feet");
    question
        .contains("frond_length_m")
        .then(|| candidates.iter().find(f1).cloned())
        .flatten()
}

/// F1's "20 feet" is the most probable span, A1's "5-7 m" its own source's.
fn spread(candidates: &[String]) -> Value {
    let p = |c: &String| match c {
        c if c.starts_with("F1") && c.ends_with(": 20 feet") => 0.6,
        c if c.starts_with("A1") && c.contains("5-7 m") => 0.3,
        _ => 0.0,
    };
    candidates
        .iter()
        .map(|c| (c.clone(), json!(p(c))))
        .collect()
}

/// Selects, files a contradicted claim on F1's frond value resolved with
/// `option`, and selects again; the profile's frond metric.
fn resolved(tag: &str, option: &str) -> Value {
    let dir = palm(tag);
    let mut palm = Palm::new(frond_from_f1);
    palm.spread = spread;
    let paths = Paths::new(&dir);
    quality::run(&paths, &judge(&palm)).unwrap();
    select::run(&paths, &judge(&palm)).unwrap();
    let sidecar = read_json(&dir.join("provenance.json")).unwrap();
    let entry = &sidecar["entries"][FROND];
    assert_eq!(entry["span"], "20 feet", "{sidecar}");
    let claim = Decision::new(
        DecisionParts {
            species: "date-palm",
            stage: "verify",
            kind: "claim-contradicted",
            field: Some(FROND),
            age_years: None,
        },
        &["generate"],
        [("select.json".into(), "test-fixture".into())]
            .into_iter()
            .collect(),
        vec![],
        json!({"pointer": FROND, "source": entry["source"], "span": entry["span"]}),
        &CLAIM_OPTIONS,
        "contradicted",
    );
    let (id, inputs) = (claim.id.clone(), claim.inputs_sha256.clone());
    append_decisions(&paths.decisions(), vec![claim]).unwrap();
    write_canonical(
        &paths.resolutions(),
        &json!({"resolutions": [{"id": id, "inputs_sha256": inputs,
                                 "option": option, "by": "species runner", "at": "2026-09-25"}]}),
    )
    .unwrap();
    select::run(&paths, &judge(&palm)).unwrap();
    let profile = read_json(&dir.join("packet").join("profile.json")).unwrap();
    profile["profiles"][0]["metrics"]["frond_length_m"].clone()
}

#[test]
fn a_kept_range_spans_each_sources_value_and_cites_every_source() {
    let frond = resolved("keep-range", "keep-range");
    assert_eq!(frond["range"], json!([5.0, 7.0]), "{frond}");
    assert_eq!(frond["source"], json!(["A1", "F1"]), "{frond}");
    assert_eq!(frond["confidence"], "spanned", "{frond}");
}

#[test]
fn a_dropped_value_leaves_its_field_unsourced() {
    let frond = resolved("drop-value", "drop-value");
    assert_eq!(frond["classification"], "unsourced", "{frond}");
    assert_eq!(frond["range"], Value::Null, "{frond}");
}
