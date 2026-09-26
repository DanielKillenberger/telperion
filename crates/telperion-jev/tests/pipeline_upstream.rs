//! The upstream chain end to end over the fixture adapter and a mock
//! transport (fn-157): gather adds what it finds, fetch reads it, read
//! labels every span, aggregate composes the profile with its provenance; a
//! rerun with unchanged inputs does nothing; a changed cache file stops read
//! by name. No network, no key, no binary.

use std::fs;
use std::path::{Path, PathBuf};

use serde_json::{json, Value};
use telperion_jev::caller::{HttpRequest, HttpResponse, Transport};
use telperion_jev::pipeline::adapter::FixtureAdapter;
use telperion_jev::pipeline::canon::{read_json, write_canonical};
use telperion_jev::pipeline::judge::Judge;
use telperion_jev::pipeline::known::KnownSources;
use telperion_jev::pipeline::stage::{Paths, StageError};
use telperion_jev::pipeline::stages::{aggregate, fetch, gather, read};

/// Labels the oak's two height spans: the usual range typical, the maximum
/// a record; every other span measures nothing asked.
struct Labels;

impl Transport for Labels {
    fn send(&self, request: &HttpRequest) -> Result<HttpResponse, String> {
        let body: Value = serde_json::from_slice(request.body.as_deref().unwrap_or(b"{}")).unwrap();
        let choice = |key: &str| json!({"type": "choice", "choice": key, "confidence": 0.9, "probabilities": {key: 0.9}});
        let answers = if body["questions"].get("field").is_some() {
            let (field, basis) = match body["state"]["candidate"]["span"].as_str() {
                Some("50 to 90 ft") => ("height_m", "typical"),
                Some("120 ft") => ("height_m", "record"),
                _ => ("none", "unclear"),
            };
            json!({"field": choice(field), "basis": choice(basis), "age": choice("mature"),
                   "condition": choice("unstated")})
        } else if body["questions"].get("level").is_some() {
            json!({"level": {"type": "score", "score": 0.0, "confidence": 0.9, "probabilities": {"0": 0.9}}})
        } else {
            return Err(format!("unexpected question {}", body["questions"]));
        };
        Ok(HttpResponse {
            status: 200,
            body: serde_json::to_vec(&json!({"model": "jev-latest", "answers": answers, "usage": {"input_tokens": 1, "output_tokens": 1}})).unwrap(),
        })
    }
}

const OWIC: &str = "https://research.fs.usda.gov/silvics/oregon-white-oak";
const TAXON: &str = "Quercus garryana";

fn manifest() -> Value {
    json!({
        "schema": "manifest", "schema_version": 1,
        "species": "oregon-white-oak",
        "taxon": {"scientific_name": TAXON, "common_name": "Oregon white oak", "rank": "species"},
        "context": "mature open-grown", "growth_form": "broadleaf",
        "preset": "oregon-white-oak", "profile_id": "oregon-white-oak", "seed": 7,
        "sources": [{"id": "S1", "url": OWIC, "title": "Silvics", "rights": "public domain"}],
        "fields": [{"field": "height_m", "condition": "open_grown", "required_ages_years": [100],
                    "bar": "partial",
                    "question": "Which candidate span states the height of a mature Oregon white oak?"}],
        "versions": {"question_sets": {"label": 1}, "tools": {}},
        "model": "jev-latest"
    })
}

/// A scratch pipeline directory beside a fixture directory that answers the
/// gather queries (the first finds the OWIC page again) and the page.
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
    let page = "# Oregon white oak\n\nMature Oregon white oaks are 50 to 90 ft tall (120 ft maximum) and 24 to 40 in. in DBH (97 in. maximum). Oregon white oaks may live 500 years.\n";
    fs::write(fixtures.join("owic.md"), page).unwrap();
    fs::write(
        fixtures.join("owic.html"),
        format!("<html><body>{page}</body></html>"),
    )
    .unwrap();
    let (web, research) = gather::queries(TAXON, "Oregon white oak");
    let hit = json!([{"url": OWIC, "title": "Silvics", "snippet": "Mature Oregon white oaks are 50 to 90 ft tall."}]);
    let search: serde_json::Map<String, Value> = web
        .iter()
        .enumerate()
        .map(|(i, q)| (q.clone(), if i == 0 { hit.clone() } else { json!([]) }))
        .collect();
    write_canonical(
        &fixtures.join("index.json"),
        &json!({
            "scrape": {OWIC: {"final_url": OWIC, "content_type": "text/html", "raw": "owic.html", "markdown": "owic.md"}},
            "search": search,
            "research": {research: []},
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

fn judge(dir: &Path) -> Judge<'static> {
    Judge {
        transport: &Labels,
        key: "test-key",
        ledger_dir: dir.join("ledger").join("entries"),
    }
}

#[test]
fn the_upstream_chain_runs_over_fixtures_and_fills_the_profile_with_provenance() {
    let (dir, adapter) = scratch();
    let paths = Paths::new(&dir);
    let judge = judge(&dir);
    let known = KnownSources::default();
    // The only hit is the page the manifest already names: nothing is added.
    assert!(matches!(
        gather::run(&paths, &adapter, &known).unwrap(),
        gather::Outcome::Ran { documents: 0 }
    ));
    fetch::run(&paths, &adapter).unwrap();
    read::run(&paths, &judge).unwrap();
    aggregate::run(&paths, &judge).unwrap();

    let profile = read_json(&dir.join("packet/profile.json")).unwrap();
    let height = &profile["profiles"][0]["metrics"]["height_m"];
    let range = height["range"].as_array().unwrap();
    assert!(
        (range[0].as_f64().unwrap() - 15.24).abs() < 1e-9,
        "{height}"
    );
    assert!(
        (range[1].as_f64().unwrap() - 27.432).abs() < 1e-9,
        "{height}"
    );
    assert_eq!(height["classification"], "gating");
    assert_eq!(height["source"], json!(["S1"]));
    assert_eq!(height["confidence"], "thin");
    assert!((height["maximum"]["value"].as_f64().unwrap() - 36.576).abs() < 1e-9);
    let sidecar = read_json(&dir.join("provenance.json")).unwrap();
    let entry = &sidecar["entries"]["/profiles/0/metrics/height_m"];
    assert_eq!(entry["route"], "aggregated");
    assert_eq!(entry["contributions"][0]["span"], "50 to 90 ft");
    assert!(entry["contributions"][0]["ledger"].is_string());

    // Unchanged inputs: every stage is current.
    assert!(matches!(
        gather::run(&paths, &adapter, &known).unwrap(),
        gather::Outcome::Current
    ));
    assert!(matches!(
        fetch::run(&paths, &adapter).unwrap(),
        fetch::Outcome::Current
    ));
    assert!(matches!(
        read::run(&paths, &judge).unwrap(),
        read::Outcome::Current
    ));
    assert!(matches!(
        aggregate::run(&paths, &judge).unwrap(),
        aggregate::Outcome::Current
    ));

    // A changed cache file stops the next reader by name.
    fs::write(dir.join("cache/S1.md"), "tampered").unwrap();
    fs::remove_file(dir.join("read.json")).unwrap();
    let err = read::run(&paths, &judge).unwrap_err();
    assert!(matches!(err, StageError::ChecksumChanged { .. }), "{err}");
    assert!(err.to_string().starts_with("read: input changed"), "{err}");
}

#[test]
fn a_missing_earlier_artifact_names_the_stage_and_the_path() {
    let (dir, _) = scratch();
    let err = read::run(&Paths::new(&dir), &judge(&dir)).unwrap_err();
    assert_eq!(
        err.to_string(),
        format!("read: input missing: {}", dir.join("fetch.json").display())
    );
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

/// Every source the catalogue already holds is listed before the adapter's
/// first web hit and joins the manifest. The first ash run spent six driver
/// dispatches searching for a table the repository already named.
#[test]
fn gather_lists_every_catalogued_source_before_it_searches_the_web() {
    let (dir, adapter) = scratch();
    let catalogue = catalogue_with_one_source(&dir);
    // No evidence tree and no specs: the catalogue is the only thing known.
    let known = KnownSources::scan(&catalogue, &dir.join("no-flow"), &dir.join("manifest.json"));
    gather::run(&Paths::new(&dir), &adapter, &known).unwrap();
    let hits = read_json(&dir.join("gather.json")).unwrap()["body"]["hits"]
        .as_array()
        .cloned()
        .unwrap();
    assert_eq!(hits[0]["kind"], json!("known"), "{hits:?}");
    assert_eq!(
        hits[0]["origin"],
        json!("catalogue:oregon-white-oak#USFS-OAK")
    );
    assert!(
        hits[1..].iter().all(|hit| hit["kind"] != json!("known")),
        "a web hit was listed among the catalogue's own: {hits:?}"
    );
    let manifest = read_json(&dir.join("manifest.json")).unwrap();
    assert_eq!(
        manifest["sources"][1]["url"],
        "https://research.fs.usda.gov/silvics/oregon-white-oak-catalogued"
    );
    assert_eq!(manifest["sources"][1]["id"], "P2");
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
    gather::run(&paths, &adapter, &KnownSources::default()).unwrap();
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
