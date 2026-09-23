//! fn-118: a manifest at schema version 2 through quality and select over
//! the fixture adapter and a mock transport. A required field below the
//! requirements table's bar, and a required appearance trait no source
//! describes, each file a requirements-unmet decision the owner holds; a
//! described appearance level's ranges are copied into the profile with the
//! source whose section stated it, and verify holds over them (fn-127). No
//! network, no key.

mod common;

use std::fs;
use std::path::PathBuf;

use common::{ledger_dir, CaseTransport};
use serde_json::{json, Value};
use telperion_jev::caller::{HttpRequest, HttpResponse, Transport};
use telperion_jev::pipeline::adapter::FixtureAdapter;
use telperion_jev::pipeline::canon::{read_json, write_canonical};
use telperion_jev::pipeline::consume::owner_stops;
use telperion_jev::pipeline::decision::reconcile;
use telperion_jev::pipeline::judge::Judge;
use telperion_jev::pipeline::requirements::table;
use telperion_jev::pipeline::stage::{Paths, StageError};
use telperion_jev::pipeline::stages::{extract, fetch, quality, screen, select, verify};

const PAGE_URL: &str = "https://example.test/oak";

/// Fixed answers for sufficiency, the mature size, the appearance level and
/// its support;
/// everything else goes to the fn-57 mock.
struct Answers {
    sufficiency: f64,
    level: f64,
}

impl Transport for Answers {
    fn send(&self, request: &HttpRequest) -> Result<HttpResponse, String> {
        let body: Value = serde_json::from_slice(request.body.as_deref().unwrap_or(b"{}")).unwrap();
        let questions = &body["questions"];
        // A level Score as Jev answers one: the level at 0.9 (fn-131 reads
        // the most probable level, never the score).
        let score = |s: f64| {
            let at = (s as usize).to_string();
            json!({"type": "score", "score": s, "confidence": 0.9, "probabilities": {at: 0.9}})
        };
        let answers = if questions.get("sufficiency").is_some() {
            json!({
                "sufficiency": score(self.sufficiency),
                "dominant_gap": {"type": "choice", "choice": "age_range_uncovered", "confidence": 0.9, "probabilities": {}},
            })
        } else if questions.get("mature_size").is_some() {
            json!({
                "mature_size": score(self.sufficiency),
                "mature_gap": {"type": "choice", "choice": "single_source", "confidence": 0.9, "probabilities": {}},
            })
        } else if questions.get("level").is_some() {
            json!({"level": score(self.level)})
        } else if questions.get("relation").is_some() {
            json!({"relation": {"type": "choice", "choice": "supports", "confidence": 0.95, "probabilities": {"supports": 0.95}}})
        } else if questions.get("measurement_not_invention").is_some() {
            json!({"measurement_not_invention": {"type": "noul", "noul": 0.9}})
        } else if questions.get("appearance_supported").is_some() {
            json!({"appearance_supported": {"type": "noul", "noul": 0.9}})
        } else {
            return CaseTransport.send(request);
        };
        Ok(HttpResponse {
            status: 200,
            body: serde_json::to_vec(&json!({"model": "jev-latest", "answers": answers})).unwrap(),
        })
    }
}

/// A broadleaf manifest at version 2 covering the table at its bars.
fn manifest() -> Value {
    let form = &table().growth_forms["broadleaf"];
    let fields: Vec<Value> = form
        .fields
        .iter()
        .map(|(name, bar)| json!({"field": name, "condition": "open_grown", "required_ages_years": [100], "bar": bar.key(), "question": "Which candidate span states this size?"}))
        .collect();
    let appearance: Vec<Value> = form
        .appearance
        .iter()
        .map(|name| json!({"trait_name": name, "sources": ["S1"]}))
        .collect();
    json!({
        "schema": "manifest", "schema_version": 2,
        "species": "oregon-white-oak",
        "taxon": {"scientific_name": "Quercus garryana", "common_name": "Oregon white oak", "rank": "species"},
        "context": "mature open-grown", "growth_form": "broadleaf",
        "preset": "oregon-white-oak", "profile_id": "oregon-white-oak", "seed": 7,
        "sources": [{"id": "S1", "url": PAGE_URL, "title": "Silvics", "rights": "public domain"}],
        "fields": fields, "appearance": appearance,
        "versions": {"question_sets": {"screen": 1, "sufficiency": 1, "described": 1}, "tools": {}},
        "model": "jev-latest"
    })
}

fn scratch() -> (PathBuf, FixtureAdapter) {
    let root = std::env::temp_dir().join(format!(
        "jev-requirements-{}-{}",
        std::process::id(),
        telperion_jev::ledger::new_entry_id()
    ));
    let (dir, fixtures) = (root.join("pipeline"), root.join("fixtures"));
    fs::create_dir_all(&dir).unwrap();
    fs::create_dir_all(&fixtures).unwrap();
    let page = "# Oregon white oak\n\nMature Oregon white oaks are 50 to 90 ft tall and 24 to 40 in. in DBH. The bark is light gray and scaly. The leaves are dark green above, paler beneath, and of one even colour across the crown.\n";
    fs::write(fixtures.join("page.md"), page).unwrap();
    fs::write(
        fixtures.join("page.html"),
        format!("<html><body>{page}</body></html>"),
    )
    .unwrap();
    write_canonical(
        &fixtures.join("index.json"),
        &json!({"scrape": {PAGE_URL: {"final_url": PAGE_URL, "content_type": "text/html", "raw": "page.html", "markdown": "page.md"}}}),
    )
    .unwrap();
    write_canonical(&dir.join("manifest.json"), &manifest()).unwrap();
    (dir, FixtureAdapter::new(fixtures))
}

/// Fetch through select; returns what quality filed.
fn run_to_select(
    transport: &Answers,
) -> (PathBuf, Vec<String>, Result<select::Outcome, StageError>) {
    let _ = ledger_dir("requirements");
    let (dir, adapter) = scratch();
    let paths = Paths::new(&dir);
    let judge = Judge {
        transport,
        key: "test-key",
        ledger_dir: dir.join("ledger").join("entries"),
    };
    fetch::run(&paths, &adapter).unwrap();
    extract::run(&paths).unwrap();
    screen::run(&paths, &judge).unwrap();
    let quality::Outcome::Ran { decisions } = quality::run(&paths, &judge).unwrap() else {
        panic!("quality ran")
    };
    let selected = select::run(&paths, &judge);
    (dir, decisions, selected)
}

fn resolve(dir: &std::path::Path, id: &str, option: &str) {
    let list = read_json(&dir.join("decisions.json")).unwrap();
    let decision = list["decisions"]
        .as_array()
        .unwrap()
        .iter()
        .find(|d| d["id"] == id)
        .unwrap();
    write_canonical(
        &dir.join("resolutions.json"),
        &json!({"resolutions": [{"id": id, "inputs_sha256": decision["inputs_sha256"], "option": option, "by": "cheap-driver", "at": "2026-09-23"}]}),
    )
    .unwrap();
}

#[test]
fn a_required_field_below_the_tables_bar_stops_the_run_for_the_owner() {
    let (dir, filed, selected) = run_to_select(&Answers {
        sufficiency: 1.0,
        level: 99.0,
    });
    let height = "oregon-white-oak/quality/requirements-unmet/height_m";
    // Proxy only passes the crown and leaf fields and fails the two at partial.
    assert_eq!(
        filed,
        ["oregon-white-oak/quality/requirements-unmet/dbh_m", height]
    );
    selected.unwrap();
    let paths = Paths::new(&dir);
    let stops = owner_stops(&reconcile(&paths).unwrap());
    // The four fields quality passed at proxy only and select could not
    // fill stop the run too (fn-131): no gap passes silently.
    let unfilled = [
        "crown_base_m",
        "crown_width_m",
        "leaf_length_m",
        "leaf_width_m",
    ]
    .map(|f| format!("oregon-white-oak/select/requirements-unmet/{f}"));
    // An unstated hue or brightness range reads zero width (fn-133); the
    // colours and the roughness stop.
    let traits = &table().growth_forms["broadleaf"].appearance;
    let stopping = traits.iter().filter(|t| table().zero_width(t).is_none());
    assert_eq!(stopping.count(), 4);
    assert_eq!(stops.len(), 2 + unfilled.len() + 4);
    assert!(unfilled.iter().all(|id| stops.contains(id)), "{stops:?}");
    assert!(stops.contains(&"oregon-white-oak/select/requirements-unmet/bark_colour".into()));
    let list = read_json(&dir.join("decisions.json")).unwrap();
    let unmet = list["decisions"]
        .as_array()
        .unwrap()
        .iter()
        .find(|d| d["id"] == height)
        .unwrap();
    assert_eq!(unmet["payload"]["bar"], "partial");
    assert_eq!(unmet["options"], json!(["add-sources"]));

    // Lowering the bar is refused by name.
    resolve(&dir, height, "lower-bar");
    let err = select::run(
        &paths,
        &Judge {
            transport: &Answers {
                sufficiency: 1.0,
                level: 99.0,
            },
            key: "test-key",
            ledger_dir: dir.join("ledger").join("entries"),
        },
    )
    .unwrap_err()
    .to_string();
    assert!(
        err.contains("option lower-bar of kind requirements-unmet is consumed by no stage"),
        "{err}"
    );

    // Adding sources without adding one leaves the decision open.
    resolve(&dir, height, "add-sources");
    assert!(owner_stops(&reconcile(&paths).unwrap()).contains(&height.to_string()));

    // A source added to the manifest lets the resolution bind.
    let mut added = manifest();
    added["sources"].as_array_mut().unwrap().push(json!({"id": "S2", "url": "https://example.test/yield", "title": "Yield table", "rights": "cited"}));
    write_canonical(&dir.join("manifest.json"), &added).unwrap();
    assert!(!owner_stops(&reconcile(&paths).unwrap()).contains(&height.to_string()));
}

#[test]
fn a_described_appearance_level_is_copied_into_the_profile_as_ranges() {
    let (dir, filed, selected) = run_to_select(&Answers {
        sufficiency: 3.0,
        level: 0.0,
    });
    assert!(filed.is_empty());
    selected.unwrap();
    let first = &table().appearance["bark_colour"].levels[0];
    let profile = read_json(&dir.join("packet").join("profile.json")).unwrap();
    let bark = &profile["profiles"][0]["appearance"]["bark_colour"];
    assert_eq!(bark["level"], json!(first.key));
    assert_eq!(bark["ranges"], json!(first.ranges));
    let sidecar = read_json(&dir.join("provenance.json")).unwrap();
    assert_eq!(
        sidecar["entries"]["/profiles/0/appearance/bark_colour"]["route"],
        "appearance"
    );
    let body = read_json(&dir.join("select.json")).unwrap();
    assert_eq!(
        body["body"]["appearance"]["bark_roughness"]["level"],
        "smooth"
    );
    // Every appearance trait is described; what stops the run is only the
    // measured fields no span of this page fills (fn-131).
    let stops = owner_stops(&reconcile(&Paths::new(&dir)).unwrap());
    let fields = &table().growth_forms["broadleaf"].fields;
    assert_eq!(stops.len(), fields.len(), "{stops:?}");
    for field in fields.keys() {
        let id = format!("oregon-white-oak/select/requirements-unmet/{field}");
        assert!(stops.contains(&id), "{id}");
    }
}

/// fn-127 R2: the palm's live select copied four appearance values with no
/// source, and verify filed `source id resolves` against each. Every copied
/// appearance value now names the source whose section stated it.
#[test]
fn every_appearance_value_carries_its_sources_id_and_verify_holds() {
    let answers = Answers {
        sufficiency: 3.0,
        level: 0.0,
    };
    let (dir, _, selected) = run_to_select(&answers);
    selected.unwrap();
    let sidecar = read_json(&dir.join("provenance.json")).unwrap();
    let entries = sidecar["entries"].as_object().unwrap();
    let appearance: Vec<_> = entries
        .iter()
        .filter(|(pointer, _)| pointer.starts_with("/profiles/0/appearance/"))
        .collect();
    assert_eq!(
        appearance.len(),
        table().growth_forms["broadleaf"].appearance.len()
    );
    for (pointer, entry) in appearance {
        assert_eq!(entry["source"], "S1", "{pointer}");
        assert!(
            entry["span"].as_str().is_some_and(|s| !s.is_empty()),
            "{pointer}"
        );
    }
    let judge = Judge {
        transport: &answers,
        key: "test-key",
        ledger_dir: dir.join("ledger").join("entries"),
    };
    let verify::Outcome::Ran { decisions } = verify::run(&Paths::new(&dir), &judge).unwrap() else {
        panic!("verify ran")
    };
    assert!(decisions.is_empty(), "{decisions:?}");
    let body = &read_json(&dir.join("verify.json")).unwrap()["body"];
    assert_eq!(body["structural"], json!([]));
    // Each span was checked against its cached source, not left unchecked.
    for claim in body["claims"].as_array().unwrap() {
        assert_eq!(claim["relation"], "supports", "{}", claim["claim"]);
    }
}
