//! The upstream chain end to end over the fixture adapter and the mock
//! transport: discover files the manifest-proposed decision and stops fetch;
//! an admitting resolution lets fetch, extract, screen, quality, select and
//! verify run; a rerun with unchanged inputs does nothing; a changed cache
//! file stops extract by name. No network, no key, no binary.

mod common;

use std::fs;
use std::path::{Path, PathBuf};

use common::{ledger_dir, CaseTransport};
use serde_json::{json, Value};
use telperion_jev::caller::{HttpRequest, HttpResponse, Transport};
use telperion_jev::pipeline::adapter::FixtureAdapter;
use telperion_jev::pipeline::canon::{read_json, write_canonical};
use telperion_jev::pipeline::judge::Judge;
use telperion_jev::pipeline::stage::{Context, Paths, StageError};
use telperion_jev::pipeline::stages::{discover, extract, fetch, quality, screen, select, verify};

/// The fn-57 mock for screen, select and cite; fixed answers for the
/// pipeline's own sets, whose states this test does not enumerate.
struct PipelineTransport(CaseTransport);

impl Transport for PipelineTransport {
    fn send(&self, request: &HttpRequest) -> Result<HttpResponse, String> {
        let body: Value = serde_json::from_slice(request.body.as_deref().unwrap_or(b"{}")).unwrap();
        let questions = &body["questions"];
        let answers = if questions.get("sufficiency").is_some() {
            json!({
                "sufficiency": {"type": "score", "score": 3.0, "confidence": 0.9, "probabilities": {"3": 0.9}},
                "dominant_gap": {"type": "choice", "choice": "none", "confidence": 0.9, "probabilities": {"none": 0.9}},
            })
        } else if questions.get("source").is_some() {
            json!({"source": {"type": "choice", "choice": "h1", "confidence": 0.9, "probabilities": {"h1": 0.9}}})
        } else if questions.get("level").is_some() {
            json!({"level": {"type": "score", "score": 0.0, "confidence": 0.9, "probabilities": {"0": 0.9}}})
        } else if questions.get("inspected_image").is_some() {
            json!({"inspected_image": {"type": "noul", "noul": 0.9}})
        } else if questions.get("measurement_not_invention").is_some() {
            json!({"measurement_not_invention": {"type": "noul", "noul": 0.9}})
        } else if questions.get("relation").is_some() {
            json!({"relation": {"type": "choice", "choice": "supports", "confidence": 0.95, "probabilities": {"supports": 0.95}}})
        } else {
            return self.0.send(request);
        };
        Ok(HttpResponse {
            status: 200,
            body: serde_json::to_vec(&json!({"model": "jev-latest", "answers": answers, "usage": {"input_tokens": 1, "output_tokens": 1}})).unwrap(),
        })
    }
}

const OWIC: &str = "https://research.fs.usda.gov/silvics/oregon-white-oak";
const QUERY: &str = "Quercus garryana height_m open_grown by age";

fn manifest() -> Value {
    json!({
        "schema": "manifest", "schema_version": 1,
        "species": "oregon-white-oak",
        "taxon": {"scientific_name": "Quercus garryana", "common_name": "Oregon white oak", "rank": "species"},
        "context": "mature open-grown", "growth_form": "broadleaf",
        "preset": "oregon-white-oak", "profile_id": "oregon-white-oak", "seed": 7,
        "sources": [{"id": "S1", "url": OWIC, "title": "Silvics", "rights": "public domain"}],
        "fields": [{"field": "height_m", "condition": "open_grown", "required_ages_years": [100],
                    "bar": "partial",
                    "question": "Which candidate span states the height of a mature Oregon white oak under ordinary conditions (not the maximum)?"}],
        "versions": {"question_sets": {"screen": 1, "sufficiency": 1}, "tools": {}},
        "model": "jev-latest"
    })
}

/// A scratch pipeline directory beside a fixture directory that answers this
/// test's discovery query and the OWIC page.
fn scratch() -> (PathBuf, FixtureAdapter) {
    let root = std::env::temp_dir().join(format!(
        "jev-pipeline-{}-{}",
        std::process::id(),
        telperion_jev::ledger::new_entry_id()
    ));
    let dir = root.join("pipeline");
    let fixtures = root.join("fixtures");
    fs::create_dir_all(&dir).unwrap();
    fs::create_dir_all(&fixtures).unwrap();
    // The page carries the labelled sentences the fn-57 mock screens.
    let page = "# Oregon white oak\n\nMature Oregon white oaks are 50 to 90 ft tall (120 ft maximum) and 24 to 40 in. in DBH (97 in. maximum). Oregon white oaks may live 500 years.\n";
    fs::write(fixtures.join("owic.md"), page).unwrap();
    fs::write(
        fixtures.join("owic.html"),
        format!("<html><body>{page}</body></html>"),
    )
    .unwrap();
    write_canonical(
        &fixtures.join("index.json"),
        &json!({
            "scrape": {OWIC: {"final_url": OWIC, "content_type": "text/html", "raw": "owic.html", "markdown": "owic.md"}},
            "search": {QUERY: [{"url": OWIC, "title": "Silvics", "snippet": "Mature Oregon white oaks are 50 to 90 ft tall."}]},
            "research": {QUERY: []},
        }),
    )
    .unwrap();
    fs::write(
        dir.join("manifest.json"),
        serde_json::to_vec_pretty(&manifest()).unwrap(),
    )
    .unwrap();
    (dir, FixtureAdapter::new(fixtures))
}

fn judge<'a>(transport: &'a PipelineTransport, dir: &Path) -> Judge<'a> {
    let _ = ledger_dir("pipeline");
    Judge {
        transport,
        key: "test-key",
        ledger_dir: dir.join("ledger").join("entries"),
    }
}

#[test]
fn the_upstream_chain_runs_over_fixtures_and_fills_the_packet_with_provenance() {
    let (dir, adapter) = scratch();
    let transport = PipelineTransport(CaseTransport);
    let judge = judge(&transport, &dir);

    // Discovery proposes; the decision stops fetch until a person admits.
    assert!(matches!(
        discover::run(&Paths::new(&dir), &catalogue_absent(&dir), &adapter, &judge).unwrap(),
        discover::Outcome::Ran { .. }
    ));
    let discover_body = read_json(&dir.join("discover.json")).unwrap();
    assert_eq!(
        discover_body["body"]["proposals"][0]["hits"][0]["ranked_first"],
        json!(true)
    );
    let err = fetch::run(&Paths::new(&dir), &adapter).unwrap_err();
    assert!(matches!(err, StageError::OpenDecision { .. }), "{err}");
    let decisions = read_json(&dir.join("decisions.json")).unwrap();
    let proposed = &decisions["decisions"][0];
    assert_eq!(proposed["kind"], "manifest-proposed");
    write_canonical(
        &dir.join("resolutions.json"),
        &json!({"resolutions": [{"id": proposed["id"], "inputs_sha256": proposed["inputs_sha256"], "option": "admit", "by": "test", "at": "2026-09-18"}]}),
    )
    .unwrap();

    assert!(matches!(
        fetch::run(&Paths::new(&dir), &adapter).unwrap(),
        fetch::Outcome::Ran { .. }
    ));
    let fetch_body = read_json(&dir.join("fetch.json")).unwrap();
    let record = &fetch_body["body"]["sources"]["S1"];
    assert_eq!(record["final_url"], OWIC);
    assert_ne!(record["raw_sha256"], record["markdown_sha256"]);
    assert!(dir.join("cache").join("S1.md").exists());

    assert!(
        matches!(extract::run(&Paths::new(&dir)).unwrap(), extract::Outcome::Ran { candidates } if candidates > 0)
    );
    assert!(
        matches!(screen::run(&Paths::new(&dir), &judge).unwrap(), screen::Outcome::Ran { rows } if rows > 0)
    );
    let screen_body = read_json(&dir.join("screen.json")).unwrap();
    let mature = screen_body["body"]["rows"]
        .as_array()
        .unwrap()
        .iter()
        .find(|r| r["sentence"].as_str().unwrap().contains("50 to 90 ft"))
        .expect("the mature sentence is screened");
    assert_eq!(mature["kind"], "mature_size_range");
    assert_eq!(mature["ledger"].as_str().unwrap().len(), 24);

    assert!(
        matches!(quality::run(&Paths::new(&dir), &judge).unwrap(), quality::Outcome::Ran { decisions } if decisions.is_empty())
    );
    let quality_body = read_json(&dir.join("quality.json")).unwrap();
    assert_eq!(
        quality_body["body"]["fields"]["height_m"]["level"],
        "sufficient"
    );
    assert_eq!(
        quality_body["body"]["fields"]["height_m"]["passed"],
        json!(true)
    );

    assert!(matches!(
        select::run(&Paths::new(&dir), &judge).unwrap(),
        select::Outcome::Ran {
            filled: 1,
            unavailable: 0
        }
    ));
    let profile = read_json(&dir.join("packet").join("profile.json")).unwrap();
    let metric = &profile["profiles"][0]["metrics"]["height_m"];
    assert_eq!(metric["range"], json!([15.24, 27.432]));
    assert_eq!(metric["source"], json!(["S1"]));
    let sidecar = read_json(&dir.join("provenance.json")).unwrap();
    let entry = &sidecar["entries"]["/profiles/0/metrics/height_m"];
    assert_eq!(entry["span"], "50 to 90 ft");
    assert_eq!(entry["route"], "copied");
    assert!(!sidecar.to_string().contains("probabilit"));

    assert!(
        matches!(verify::run(&Paths::new(&dir), &judge).unwrap(), verify::Outcome::Ran { decisions } if decisions.is_empty())
    );
    let verify_body = read_json(&dir.join("verify.json")).unwrap();
    assert_eq!(verify_body["body"]["claims"][0]["relation"], "supports");
    assert_eq!(verify_body["body"]["structural"], json!([]));

    // A rerun with unchanged inputs does nothing on every stage.
    let before = fs::read(dir.join("select.json")).unwrap();
    assert!(matches!(
        fetch::run(&Paths::new(&dir), &adapter).unwrap(),
        fetch::Outcome::Current
    ));
    assert!(matches!(
        extract::run(&Paths::new(&dir)).unwrap(),
        extract::Outcome::Current
    ));
    assert!(matches!(
        screen::run(&Paths::new(&dir), &judge).unwrap(),
        screen::Outcome::Current
    ));
    assert!(matches!(
        quality::run(&Paths::new(&dir), &judge).unwrap(),
        quality::Outcome::Current
    ));
    assert!(matches!(
        select::run(&Paths::new(&dir), &judge).unwrap(),
        select::Outcome::Current
    ));
    assert!(matches!(
        verify::run(&Paths::new(&dir), &judge).unwrap(),
        verify::Outcome::Current
    ));
    assert_eq!(fs::read(dir.join("select.json")).unwrap(), before);

    // A changed cache file stops the stage that reads it, by name.
    fs::write(dir.join("cache").join("S1.md"), "edited\n").unwrap();
    fs::remove_file(dir.join("extract.json")).unwrap();
    let err = extract::run(&Paths::new(&dir)).unwrap_err();
    assert!(matches!(err, StageError::ChecksumChanged { .. }), "{err}");
    assert!(
        err.to_string().starts_with("extract: input changed"),
        "{err}"
    );
}

#[test]
fn a_missing_earlier_artifact_names_the_stage_and_the_path() {
    let (dir, _) = scratch();
    let err = extract::run(&Paths::new(&dir)).unwrap_err();
    assert_eq!(
        err.to_string(),
        format!(
            "extract: input missing: {}",
            dir.join("fetch.json").display()
        )
    );
    let (_, blocked) = Context::open(&Paths::new(&dir), "select").unwrap();
    assert!(blocked.is_empty());
}

#[test]
fn a_field_below_the_bar_files_data_insufficient_and_select_records_it_unavailable() {
    struct Insufficient(PipelineTransport);
    impl Transport for Insufficient {
        fn send(&self, request: &HttpRequest) -> Result<HttpResponse, String> {
            let body: Value =
                serde_json::from_slice(request.body.as_deref().unwrap_or(b"{}")).unwrap();
            if body["questions"].get("sufficiency").is_none() {
                return self.0.send(request);
            }
            Ok(HttpResponse {
                status: 200,
                body: serde_json::to_vec(&json!({"model": "jev-latest", "answers": {
                    "sufficiency": {"type": "score", "score": 1.0, "confidence": 0.9, "probabilities": {"1": 0.9}},
                    "dominant_gap": {"type": "choice", "choice": "wrong_condition", "confidence": 0.9, "probabilities": {"wrong_condition": 0.9}},
                }})).unwrap(),
            })
        }
    }
    let (dir, adapter) = scratch();
    let transport = Insufficient(PipelineTransport(CaseTransport));
    let judge = Judge {
        transport: &transport,
        key: "test-key",
        ledger_dir: dir.join("ledger").join("entries"),
    };
    discover::run(&Paths::new(&dir), &catalogue_absent(&dir), &adapter, &judge).unwrap();
    let decisions = read_json(&dir.join("decisions.json")).unwrap();
    let proposed = &decisions["decisions"][0];
    write_canonical(
        &dir.join("resolutions.json"),
        &json!({"resolutions": [{"id": proposed["id"], "inputs_sha256": proposed["inputs_sha256"], "option": "admit", "by": "test", "at": "2026-09-18"}]}),
    )
    .unwrap();
    fetch::run(&Paths::new(&dir), &adapter).unwrap();
    extract::run(&Paths::new(&dir)).unwrap();
    screen::run(&Paths::new(&dir), &judge).unwrap();
    let outcome = quality::run(&Paths::new(&dir), &judge).unwrap();
    let quality::Outcome::Ran { decisions } = outcome else {
        panic!("ran")
    };
    assert_eq!(
        decisions,
        vec!["oregon-white-oak/quality/data-insufficient/height_m"]
    );
    let list = read_json(&dir.join("decisions.json")).unwrap();
    let insufficient = list["decisions"]
        .as_array()
        .unwrap()
        .iter()
        .find(|d| d["kind"] == "data-insufficient")
        .unwrap();
    assert_eq!(insufficient["payload"]["level"], "proxy_only");
    assert_eq!(insufficient["payload"]["dominant_gap"], "wrong_condition");
    assert_eq!(insufficient["blocks"], json!(["select", "fit", "generate"]));
    assert!(insufficient["payload"].get("probabilities").is_none());
    assert!(matches!(
        select::run(&Paths::new(&dir), &judge).unwrap(),
        select::Outcome::Ran {
            filled: 0,
            unavailable: 1
        }
    ));
    let sidecar = read_json(&dir.join("provenance.json")).unwrap();
    assert_eq!(
        sidecar["unavailable"]["height_m"],
        "below the data-quality bar"
    );
}

/// Discovery takes a catalogue directory; a test points it at one that does not
/// exist, so only the adapter's hits are listed, exactly as before fn-35.
fn catalogue_absent(dir: &std::path::Path) -> std::path::PathBuf {
    dir.join("no-catalogue")
}

/// A catalogue holding one species folder with one source.
fn catalogue_with_one_source(dir: &Path) -> PathBuf {
    let catalogue = dir.join("catalogue").join("oregon-white-oak");
    fs::create_dir_all(&catalogue).unwrap();
    write_canonical(
        &catalogue.join("sources.json"),
        &json!({
            "schema": "sources", "schema_version": 1, "species": "oregon-white-oak",
            "sources": [{
                "id": "USFS-OAK",
                "url": "https://research.fs.usda.gov/silvics/oregon-white-oak-catalogued",
                "title": "Silvics of North America",
                "attribution": "William I. Stein, US Forest Service",
                "rights": "US Forest Service publication, public domain",
                "sha256": null, "verified": "2026-09-06",
                "use": "Species and habitat context.", "tables": [],
            }],
        }),
    )
    .unwrap();
    dir.join("catalogue")
}

/// Every source the catalogue already holds is a candidate, and it is listed
/// before the adapter's first web hit. The first ash run spent six driver
/// dispatches searching for a table the repository already named.
#[test]
fn discovery_lists_every_catalogued_source_before_it_searches_the_web() {
    let (dir, adapter) = scratch();
    let transport = PipelineTransport(CaseTransport);
    let judge = judge(&transport, &dir);
    let catalogue = catalogue_with_one_source(&dir);

    discover::run(&Paths::new(&dir), &catalogue, &adapter, &judge).unwrap();

    let hits = read_json(&dir.join("discover.json")).unwrap()["body"]["proposals"][0]["hits"]
        .as_array()
        .cloned()
        .unwrap();
    assert_eq!(hits[0]["kind"], json!("catalogue"), "{hits:?}");
    assert_eq!(
        hits[0]["url"],
        json!("https://research.fs.usda.gov/silvics/oregon-white-oak-catalogued")
    );
    assert!(
        hits[1..]
            .iter()
            .all(|hit| hit["kind"] != json!("catalogue")),
        "a web hit was listed among the catalogue's own: {hits:?}"
    );
}

/// With a run directory of its own, a stage leaves the species folder holding
/// canonical artifacts only: the fetch cache, the ledger and the command log
/// stay in the run, so none of a run's scratch enters the catalogue.
#[test]
fn a_run_directory_keeps_a_runs_scratch_out_of_the_species_folder() {
    let (species, adapter) = scratch();
    let run = species.join("..").join("run-elsewhere");
    fs::create_dir_all(&run).unwrap();
    let paths = Paths::with_run(&species, &run);
    let transport = PipelineTransport(CaseTransport);
    let judge = judge(&transport, &run);

    discover::run(&paths, &catalogue_absent(&species), &adapter, &judge).unwrap();
    let proposed = read_json(&species.join("decisions.json")).unwrap()["decisions"][0].clone();
    write_canonical(
        &species.join("resolutions.json"),
        &json!({"resolutions": [{"id": proposed["id"], "inputs_sha256": proposed["inputs_sha256"], "option": "admit", "by": "test", "at": "2026-09-18"}]}),
    )
    .unwrap();
    fetch::run(&paths, &adapter).unwrap();

    for scratch_name in ["cache", "ledger", "stills", "command-log.json"] {
        assert!(
            !species.join(scratch_name).exists(),
            "oregon-white-oak: {scratch_name} was written into the species folder"
        );
    }
    assert!(
        run.join("cache").join("S1.md").exists(),
        "the run kept no cache"
    );
    assert!(
        species.join("fetch.json").exists(),
        "the species folder kept no artifact"
    );
}
