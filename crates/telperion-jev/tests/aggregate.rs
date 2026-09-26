//! fn-157: a species' values are the aggregate of what its documents say.
//! The beech pages recorded on 2026-09-25 (`fixtures/aggregate/tape`, trimmed
//! to what the run quoted) are the regression cases: the heritage page's
//! single giant beech (a 4.36 m trunk) is the field's maximum and never its
//! typical value (R2), and the NC State page's leaf length, read as a width
//! on 2026-09-25, is set aside by the words beside it (R1). The labels come
//! from the labelled set (`data/cases/label.json`) through the case
//! transport; no network, no key.

mod common;

use std::fs;
use std::path::{Path, PathBuf};

use common::CaseTransport;
use serde_json::{json, Value};
use telperion_jev::caller::{HttpRequest, HttpResponse, Transport};
use telperion_jev::pipeline::adapter::FixtureAdapter;
use telperion_jev::pipeline::canon::{read_json, write_canonical};
use telperion_jev::pipeline::judge::Judge;
use telperion_jev::pipeline::requirements::table;
use telperion_jev::pipeline::stage::{Context, Paths};
use telperion_jev::pipeline::stages::gather::GATHERED;
use telperion_jev::pipeline::stages::{aggregate, fetch, inputs, read};
use telperion_jev::tape::{self, Tape};

const WSU: &str =
    "https://extension.wsu.edu/clark/master-gardeners-home/community-education/heritage-tree/european-beech/";
const NCSU: &str = "https://plants.ces.ncsu.edu/plants/fagus-sylvatica/";
const VDBERK: &str = "https://www.vdberk.com/trees/fagus-sylvatica/";

fn fixture() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/replay/european-beech")
}

fn scratch(tag: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "jev-aggregate-{tag}-{}-{}",
        std::process::id(),
        telperion_jev::ledger::new_entry_id()
    ));
    fs::create_dir_all(&dir).unwrap();
    dir
}

/// The labelled set, except that the NC State "Leaf Length:" span is read as
/// a width, as the recorded run read it; every appearance trait unstated.
struct Misread;

impl Transport for Misread {
    fn send(&self, request: &HttpRequest) -> Result<HttpResponse, String> {
        let body: Value = serde_json::from_slice(request.body.as_deref().unwrap_or(b"{}")).unwrap();
        if body["questions"].get("level").is_some() {
            return Levels(json!({})).send(request);
        }
        let marked = body["state"]["candidate"]["marked"].as_str().unwrap_or("");
        if !marked.contains("Leaf Length:⟦3-6 inches⟧") {
            return CaseTransport.send(request);
        }
        let choice = |key: &str| json!({"type": "choice", "choice": key, "confidence": 0.9, "probabilities": {key: 0.9}});
        let answers = json!({"field": choice("leaf_width_m"), "basis": choice("typical"),
                             "age": choice("mature"), "condition": choice("unstated")});
        Ok(HttpResponse {
            status: 200,
            body: serde_json::to_vec(&json!({"model": "jev-latest", "answers": answers})).unwrap(),
        })
    }
}

/// The beech's bare seed with the three recorded pages gathered, fetched
/// from the recording, read and aggregated.
fn recorded_beech() -> PathBuf {
    let dir = scratch("recorded");
    let mut manifest = read_json(&fixture().join("seed/manifest.json")).unwrap();
    let source =
        |id: &str, url: &str| json!({"id": id, "url": url, "title": id, "rights": GATHERED});
    manifest["sources"] = json!([source("P1", WSU), source("P2", NCSU), source("P3", VDBERK)]);
    write_canonical(&dir.join("manifest.json"), &manifest).unwrap();
    let paths = Paths::new(&dir);
    let recorded = tape::Fetch {
        inner: Box::new(FixtureAdapter::new(dir.join("no-fixture"))),
        tape: Tape::Replay(
            Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/aggregate/tape"),
        ),
    };
    fetch::run(&paths, &recorded).unwrap();
    let judge = Judge {
        transport: &Misread,
        key: "test-key",
        ledger_dir: common::ledger_dir("aggregate"),
    };
    read::run(&paths, &judge).unwrap();
    aggregate::run(&paths, &judge).unwrap();
    dir
}

fn metric(dir: &Path, field: &str) -> Value {
    read_json(&dir.join("packet/profile.json")).unwrap()["profiles"][0]["metrics"][field].clone()
}

fn near(value: &Value, want: f64) -> bool {
    value.as_f64().is_some_and(|v| (v - want).abs() < 1e-6)
}

/// R2: the heritage beech's 171.6-inch trunk, its 125 ft and its 85 ft
/// spread are one tree's: each is its field's maximum, and none is a typical
/// value. The trunk and the crown have no typical value on these pages and
/// stay unsourced; the height's typical value is NC State's, an extension
/// page that outranks the nursery's 35 m.
#[test]
fn a_single_specimen_is_the_fields_maximum_never_its_typical_value() {
    let dir = recorded_beech();
    let dbh = metric(&dir, "dbh_m");
    assert_eq!(dbh["classification"], "unsourced", "{dbh}");
    assert!(dbh["range"].is_null(), "{dbh}");
    assert!(near(&dbh["maximum"]["value"], 171.6 * 0.0254), "{dbh}");
    assert_eq!(dbh["maximum"]["source"], "P1");
    let crown = metric(&dir, "crown_width_m");
    assert_eq!(crown["classification"], "unsourced", "{crown}");
    assert!(near(&crown["maximum"]["value"], 85.0 * 0.3048), "{crown}");

    let height = metric(&dir, "height_m");
    assert_eq!(height["source"], json!(["P2"]), "{height}");
    assert_eq!(height["tier"], "extension", "{height}");
    assert_eq!(
        height["tiers"],
        json!({"extension": 1, "nursery": 1}),
        "{height}"
    );
    assert!(near(&height["range"][1], 60.0 * 0.3048), "{height}");
    assert!(near(&height["value"], 55.0 * 0.3048), "{height}");
    assert!(
        near(&height["maximum"]["value"], 125.0 * 0.3048),
        "{height}"
    );
    assert_eq!(height["maximum"]["source"], "P1");
    assert_eq!(height["confidence"], "thin");
}

/// R1's regression: the NC State page's "Leaf Length: 3-6 inches", labelled
/// a width as on 2026-09-25, is set aside by the label beside it; the width
/// keeps only the span the page calls a width, and the length its "2 to 4
/// inches long".
#[test]
fn a_length_labelled_as_a_width_is_set_aside_by_the_words_beside_it() {
    let dir = recorded_beech();
    let width = metric(&dir, "leaf_width_m");
    assert_eq!(width["source"], json!(["P2"]), "{width}");
    assert!(near(&width["range"][1], 6.0 * 0.0254), "{width}");
    let aside = width["set_aside"].as_array().unwrap();
    assert_eq!(aside.len(), 1, "{width}");
    assert_eq!(
        aside[0]["reason"],
        "the words beside the span give its length, not its width"
    );
    let length = metric(&dir, "leaf_length_m");
    assert!(near(&length["value"], 3.0 * 0.0254), "{length}");
    let provenance = read_json(&dir.join("provenance.json")).unwrap();
    let read = &provenance["entries"]["/profiles/0/metrics/leaf_length_m"]["contributions"];
    assert_eq!(read[0]["span"], "2 to 4 inches", "{read}");
}

/// A palm run whose read found nothing: only the appearance route speaks.
fn palm_read_nothing(tag: &str, levels: Value) -> (PathBuf, Levels) {
    let dir = scratch(tag);
    let fixtures = dir.join("fixtures");
    fs::create_dir_all(&fixtures).unwrap();
    let frond = "The pinnately compound blue-green to gray-green leaves or fronds can grow to 20 feet in length.";
    fs::write(fixtures.join("f1.md"), format!("{frond}\n")).unwrap();
    fs::write(fixtures.join("f1.html"), format!("<p>{frond}</p>")).unwrap();
    let url = "https://example.test/F1";
    write_canonical(
        &fixtures.join("index.json"),
        &json!({"scrape": {url: {"final_url": url, "content_type": "text/html", "raw": "f1.html", "markdown": "f1.md"}}}),
    )
    .unwrap();
    let form = &table().growth_forms["palm"];
    let fields: Vec<Value> = form
        .fields
        .iter()
        .map(|(name, bar)| json!({"field": name, "condition": "open_grown", "bar": bar.key(), "question": name}))
        .collect();
    let appearance: Vec<Value> = form
        .appearance
        .iter()
        .map(|name| json!({"trait_name": name, "sources": []}))
        .collect();
    let manifest = json!({
        "schema": "manifest", "schema_version": 2, "species": "date-palm",
        "taxon": {"scientific_name": "Phoenix dactylifera", "common_name": "Date palm", "rank": "species"},
        "context": "mature open-grown", "growth_form": "palm",
        "preset": "date-palm", "profile_id": "date-palm", "seed": 7,
        "sources": [{"id": "F1", "url": url, "title": "F1", "rights": "cited"}],
        "fields": fields, "appearance": appearance,
        "versions": {"question_sets": {"described": 1, "label": 1}, "tools": {}},
        "model": "jev-latest"
    });
    write_canonical(&dir.join("manifest.json"), &manifest).unwrap();
    let paths = Paths::new(&dir);
    fetch::run(&paths, &FixtureAdapter::new(fixtures)).unwrap();
    let (ctx, _) = Context::open(&paths, "read").unwrap();
    ctx.write(
        &ctx.header("read", "read", inputs(&[]), vec![]),
        json!({"spans": []}),
    )
    .unwrap();
    (dir, Levels(levels))
}

/// Answers each appearance trait with the level index `0.0` names, the
/// no-match level for a trait it does not name.
struct Levels(Value);

impl Transport for Levels {
    fn send(&self, request: &HttpRequest) -> Result<HttpResponse, String> {
        let body: Value = serde_json::from_slice(request.body.as_deref().unwrap_or(b"{}")).unwrap();
        let name = body["state"]["trait"].as_str().unwrap_or_default();
        let count = table().levels(name).map_or(0, |l| l.len());
        let at = self.0[name]
            .as_u64()
            .map_or(count, |i| i as usize)
            .to_string();
        let answers = json!({"level": {"type": "score", "score": 0.0, "confidence": 0.9,
                                       "probabilities": {at: 0.9}}});
        Ok(HttpResponse {
            status: 200,
            body: serde_json::to_vec(&json!({"model": "jev-latest", "answers": answers})).unwrap(),
        })
    }
}

fn judge_levels<'a>(levels: &'a Levels, dir: &Path) -> Judge<'a> {
    Judge {
        transport: levels,
        key: "test-key",
        ledger_dir: dir.join("ledger"),
    }
}

/// fn-139 R1 under aggregate: a trait naming no source reads every
/// document; an unstated back with a sourced front takes the front's level,
/// its ranges on the back's fields, as a default citing the front's source;
/// an unstated bark stays unstated and files nothing (no search is left to
/// send it to).
#[test]
fn an_unstated_back_defaults_to_the_sourced_front_and_bark_stays_unstated() {
    let (dir, levels) = palm_read_nothing("back", json!({"leaf_front_colour": 3}));
    aggregate::run(&Paths::new(&dir), &judge_levels(&levels, &dir)).unwrap();
    let body = &read_json(&dir.join("aggregate.json")).unwrap()["body"]["appearance"];
    assert_eq!(body["leaf_front_colour"]["level"], "grey_green", "{body}");
    assert_eq!(body["leaf_front_colour"]["source"], "F1", "{body}");
    assert_eq!(body["leaf_back_colour"]["level"], "grey_green", "{body}");
    assert!(body["leaf_back_colour"]["default"].is_string(), "{body}");
    assert_eq!(body["bark_colour"]["level"], "unstated", "{body}");
    let profile = read_json(&dir.join("packet/profile.json")).unwrap();
    let back = &profile["profiles"][0]["appearance"]["leaf_back_colour"];
    assert_eq!(back["sources"], json!(["F1"]), "{back}");
    assert!(back["ranges"].get("leaf_back_red").is_some(), "{back}");
    assert!(!dir.join("decisions.json").exists());
}

/// fn-142 under aggregate: a rerun keeps the recorded reference photographs
/// byte for byte and rewrites only the references record's sources.
#[test]
fn a_rerun_keeps_the_recorded_references_and_only_rewrites_sources() {
    let (dir, levels) = palm_read_nothing("references", json!({}));
    let recorded = json!([{"id": "P-WHOLE", "source_id": "R1", "kind": "real"}]);
    let stale = json!([{"id": "OLD", "url": "https://example.test/old"}]);
    write_canonical(
        &dir.join("packet/references.json"),
        &json!({"reference_version": "fn19-references-v1", "sources": stale, "references": recorded}),
    )
    .unwrap();
    aggregate::run(&Paths::new(&dir), &judge_levels(&levels, &dir)).unwrap();
    let after = read_json(&dir.join("packet/references.json")).unwrap();
    assert_eq!(after["references"], recorded, "{after}");
    assert_eq!(after["sources"][0]["id"], "F1", "{after}");
}
