//! fn-127 R1: the palm's live quality stage scored leaflet length and crown
//! width `none` on A1's sentences, because a mature size was asked for
//! points at an age. A field the requirements table marks `mature` is judged
//! by the mature-size set instead. The screen rows are A1's, copied from the
//! fn-80 live run; the transport answers the age-indexed set as Jev did
//! there and the mature-size set from its labelled cases. No network.

mod common;

use std::fs;
use std::path::PathBuf;

use common::{ledger_dir, CaseTransport};
use serde_json::{json, Value};
use telperion_jev::caller::{HttpRequest, HttpResponse, Transport};
use telperion_jev::pipeline::canon::{read_json, write_canonical};
use telperion_jev::pipeline::consume::{sources_sha256, REQUIREMENTS_UNMET};
use telperion_jev::pipeline::decision::{append_decisions, Decision, DecisionParts};
use telperion_jev::pipeline::judge::Judge;
use telperion_jev::pipeline::requirements::table;
use telperion_jev::pipeline::stage::{Context, Paths};
use telperion_jev::pipeline::stages::{inputs, quality, select};

const A1_FROND: &str = "Leaves resemble a \u{2018}feather-duster\u{2019} as fronds are pinnately compound, 5-7 m (15 to 20 feet) long, with armed petioles and stout midribs that hold the slender pinnae (1, 5).";
const A1_LEAFLET: &str = "The leaflets are \u{bd} m (18 inches) long (1, 3, 5) and are induplicate, meaning they appear to have been folded in half lengthwise (5).";
const A1_LISTING: &str = "* * * **Height:** 50 - 100 feet **Width:** 20 - 50 feet **Growth Rate:** Slow Growing **Grow Season:** Summer **Flower Season:** Summer **Color:** White **Function:** Accent **Spread:** Non-spreading **Allergen:** Non-allergenic **Invasive:** Benign **Toxicity:** Benign **Hardy:** Semi-hardy **Water Use:** Low water Use * * * **Citations:** 1.";
const A1_TRUNK: &str = "The trunk has a characteristic diamond pattern and is rough gray and up to \u{bd} m (18 inches) in diameter (1, 3).";
const A1_RATE: &str = "Date palms have a moderate growth rate of 30-45 cm (1 to 1.5 feet) a year, reaching 5 m (20 feet) in 15 to 20 years depending on the cultivar and soil and water conditions.";

/// The age-indexed set answers `none` over no point at an age, as Jev did
/// live; everything else, the mature-size set among it, goes to the fixture
/// transport, which answers from the labelled case whose state the stage
/// laid out.
/// The selection tool's span is each field's first candidate; this test is
/// about which fields select asks.
struct Palm;

impl Transport for Palm {
    fn send(&self, request: &HttpRequest) -> Result<HttpResponse, String> {
        let body: Value = serde_json::from_slice(request.body.as_deref().unwrap_or(b"{}")).unwrap();
        let answers = if body["questions"].get("sufficiency").is_some() {
            json!({
                "sufficiency": {"type": "score", "score": 0.0, "confidence": 0.9, "probabilities": {}},
                "dominant_gap": {"type": "choice", "choice": "no_age_indexed_points", "confidence": 0.9, "probabilities": {}},
            })
        } else if body["questions"].get("span").is_some() {
            // Each field's first candidate: the field's own rows (fn-131).
            let chosen = body["state"]["candidates"][0].clone();
            json!({"span": {"type": "choice", "choice": chosen, "confidence": 0.9, "probabilities": {}}})
        } else {
            return CaseTransport.send(request);
        };
        Ok(HttpResponse {
            status: 200,
            body: serde_json::to_vec(&json!({"model": "jev-latest", "answers": answers})).unwrap(),
        })
    }
}

/// The palm's manifest at version 2, its mature fields carrying `ages`.
fn manifest(ages: Value) -> Value {
    let form = &table().growth_forms["palm"];
    let fields: Vec<Value> = form
        .fields
        .iter()
        .map(|(name, bar)| {
            let at = if ["height_m", "dbh_m"].contains(&name.as_str()) {
                json!([20.0, 50.0, 80.0])
            } else {
                ages.clone()
            };
            json!({"field": name, "condition": "open_grown", "required_ages_years": at, "bar": bar.key(), "question": "Which span states this size?"})
        })
        .collect();
    let appearance: Vec<Value> = form
        .appearance
        .iter()
        .map(|name| json!({"trait_name": name, "sources": ["A1"]}))
        .collect();
    json!({
        "schema": "manifest", "schema_version": 2, "species": "date-palm",
        "taxon": {"scientific_name": "Phoenix dactylifera", "common_name": "Date palm", "rank": "species"},
        "context": "mature open-grown", "growth_form": "palm",
        "preset": "date-palm", "profile_id": "date-palm", "seed": 7,
        "sources": [{"id": "A1", "url": "https://example.test/a1", "title": "UA Campus Arboretum", "rights": "cited"}],
        "fields": fields, "appearance": appearance,
        "versions": {"question_sets": {"sufficiency": 1, "mature_size": 1}, "tools": {}},
        "model": "jev-latest"
    })
}

fn row(sentence: &str, kind: &str) -> Value {
    json!({"source": "A1", "sentence": sentence, "kind": kind, "condition": "unstated", "anchor_usable": false, "ledger": "live"})
}

/// A pipeline directory holding the manifest and the fetch and screen
/// artifacts quality reads, with A1's screened rows as the live run kept
/// them, the organ sentences in the organ classes fn-131's screen offers.
fn prepare(ages: Value) -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "jev-mature-{}-{}",
        std::process::id(),
        telperion_jev::ledger::new_entry_id()
    ));
    fs::create_dir_all(&dir).unwrap();
    write_canonical(&dir.join("manifest.json"), &manifest(ages)).unwrap();
    let rows = json!({"rows": [
        row(A1_TRUNK, "mature_size_range"),
        row(A1_FROND, "frond_size"),
        row(A1_LEAFLET, "leaflet_size"),
        row(A1_RATE, "typical_growth_rate"),
        row(A1_LISTING, "mature_size_range"),
    ]});
    for (stage, body) in [
        ("fetch", json!({"sources": {}, "tables": {}})),
        ("screen", rows),
    ] {
        let (ctx, _) = Context::open(&Paths::new(&dir), stage).unwrap();
        let header = ctx.header(stage, stage, inputs(&[]), vec![]);
        ctx.write(&header, body).unwrap();
    }
    dir
}

#[test]
fn a_mature_field_reaches_its_bar_from_a_stated_mature_size_with_or_without_an_age() {
    // The live manifest gave the mature sizes [50.0] to pass admission; a
    // mature field now needs no age at all.
    for ages in [json!([50.0]), json!([])] {
        let dir = prepare(ages.clone());
        let judge = Judge {
            transport: &Palm,
            key: "test-key",
            ledger_dir: ledger_dir("mature"),
        };
        let quality::Outcome::Ran { decisions } = quality::run(&Paths::new(&dir), &judge).unwrap()
        else {
            panic!("quality ran")
        };
        let fields = &read_json(&dir.join("quality.json")).unwrap()["body"]["fields"];
        for name in ["leaflet_length_m", "crown_width_m", "frond_length_m"] {
            assert_eq!(fields[name]["level"], "partial", "{ages}: {name}");
            assert_eq!(fields[name]["passed"], true, "{ages}: {name}");
            assert_eq!(
                fields[name]["dominant_gap"], "single_source",
                "{ages}: {name}"
            );
        }
        // A1 states the leaflet's length, never its width.
        assert_eq!(fields["leaflet_width_m"]["level"], "none", "{ages}");
        // Height and trunk diameter stay age-indexed.
        assert_eq!(fields["height_m"]["dominant_gap"], "no_age_indexed_points");
        assert_eq!(
            decisions,
            ["dbh_m", "height_m", "leaflet_width_m"]
                .map(|f| format!("date-palm/quality/requirements-unmet/{f}")),
            "{ages}"
        );
    }
}

/// fn-128 R1: on the palm's second pass quality passed crown width and the
/// leaflet length, yet the `requirements-unmet` decisions its first pass had
/// filed on them stayed open, and select skipped both as "below the
/// data-quality bar". The first pass's decisions are rebuilt here with its
/// `fetch.json` checksum and the manifest's unchanged sources, as the live
/// `decisions.json` holds them.
#[test]
fn a_rerun_supersedes_the_unmet_decisions_it_no_longer_files_and_select_fills_them() {
    let dir = prepare(json!([]));
    let paths = Paths::new(&dir);
    let sources = sources_sha256(&paths.manifest()).unwrap();
    let first_pass = |field: &str| {
        Decision::new(
            DecisionParts {
                species: "date-palm",
                stage: "quality",
                kind: REQUIREMENTS_UNMET,
                field: Some(field),
                age_years: None,
            },
            &["select", "fit", "generate"],
            inputs(&[(
                "fetch.json",
                "c2f89838ac59b3ab23f2c1b4fb6a30caaf4439c03363c0160bd3bd95d8cced1a",
            )]),
            vec![],
            json!({"field": field, "bar": "proxy_only", "sources_sha256": sources}),
            &["add-sources"],
            "NEEDS_HUMAN",
        )
    };
    let stale = ["crown_width_m", "leaflet_length_m", "leaflet_width_m"];
    append_decisions(&paths.decisions(), stale.map(first_pass).to_vec()).unwrap();
    let judge = Judge {
        transport: &Palm,
        key: "test-key",
        ledger_dir: ledger_dir("mature-retire"),
    };
    quality::run(&paths, &judge).unwrap();

    let list = read_json(&paths.decisions()).unwrap();
    let status = |field: &str| {
        let id = format!("date-palm/quality/requirements-unmet/{field}");
        let d = list["decisions"]
            .as_array()
            .unwrap()
            .iter()
            .find(|d| d["id"] == id)
            .unwrap()
            .clone();
        (d["status"].clone(), d["resolution"]["option"].clone())
    };
    for field in ["crown_width_m", "leaflet_length_m"] {
        assert_eq!(
            status(field),
            (json!("resolved"), json!("superseded")),
            "{field}"
        );
    }
    // A1 still states no leaflet width: filed again, open.
    assert_eq!(status("leaflet_width_m").0, "open");

    // The manifest's sources are unchanged, and a superseded decision is
    // not an owner's resolution that added none: select opens with it
    // retired and fills the fields quality passed.
    select::run(&paths, &judge).unwrap();
    let body = &read_json(&dir.join("select.json")).unwrap()["body"];
    for field in ["crown_width_m", "leaflet_length_m"] {
        let pointer = format!("/profiles/0/metrics/{field}");
        assert!(body["filled"].get(&pointer).is_some(), "{field}: {body}");
    }
    assert_eq!(
        body["unavailable"]["leaflet_width_m"],
        "below the data-quality bar"
    );
}
