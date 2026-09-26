//! The fixes from the first ash run (fn-75), on the ash's own shape over the
//! fixture adapter: the seed manifest, a person's manifest with the
//! Ertragstafeln extract's Esche block, a source the adapter cannot fetch,
//! and a repository that already knows the extract. Gather adds documents
//! and fetch reads them with no admission (fn-157). No network, no key, no
//! binary.

use std::fs;
use std::path::{Path, PathBuf};

use serde_json::{json, Value};
use telperion_jev::pipeline::adapter::FixtureAdapter;
use telperion_jev::pipeline::canon::{read_json, write_canonical};
use telperion_jev::pipeline::known::KnownSources;
use telperion_jev::pipeline::stage::{Paths, StageError};
use telperion_jev::pipeline::stages::{fetch, gather};

const ERTRAGSTAFELN: &str = "https://www.forstpraxis.de/sites/forstpraxis.de/files/2023-07/AFZ_FHJ_Kalender_2024_306_318_Ertragstafeln_ste_OK.pdf";
const OSU: &str = "https://landscapeplants.oregonstate.edu/plants/fraxinus-excelsior";
const MOBOT: &str = "https://plantfinder.mobot.org/PlantFinderDetails.aspx?taxonid=282928";
/// A source an earlier run failed on and then retried: known, without an error.
const RETRIED: &str = "https://www.tree-guide.com/ash";
const TLS_ERROR: &str = "fetch failed for https://plantfinder.mobot.org/PlantFinderDetails.aspx?taxonid=282928: Connection Failed: tls connection init failed: invalid peer certificate: UnknownIssuer";
/// A source only a proof run's raw scratch names.
const RAW_ONLY: &str = "https://example.test/raw-ash";

fn fixtures() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/ash")
}

/// The ash seed as the run of 2026-09-18 wrote it: two fields, no source.
fn seed() -> Value {
    json!({
        "schema": "manifest", "schema_version": 1,
        "species": "european-ash",
        "taxon": {"scientific_name": "Fraxinus excelsior", "common_name": "European ash", "rank": "species"},
        "context": "Mature tree-form, open-grown European parkland", "growth_form": "broadleaf",
        "preset": "european-ash", "profile_id": "european-ash", "seed": 7,
        "sources": [],
        "fields": [
            {"field": "height_m", "condition": "open_grown", "required_ages_years": [20, 50, 80], "bar": "proxy_only",
             "question": "Which candidate span states the height a European ash reaches at a stated age?"},
            {"field": "dbh_m", "condition": "open_grown", "required_ages_years": [20, 50, 80], "bar": "partial",
             "question": "Which candidate span states the trunk diameter of a European ash at a stated age?"}
        ],
        "versions": {"question_sets": {"label": 1}, "tools": {"firecrawl": "1.23.3"}},
        "model": "jev-latest"
    })
}

fn table(block: Option<&str>) -> Value {
    let mut table = json!({
        "id": "E1-ash-height-I", "table_index": 5, "expected_rows": 11, "dimension": "height_m",
        "unit": "m", "value_column": 1, "condition": "stand_grown", "taxon": "Fraxinus excelsior"
    });
    if let Some(block) = block {
        table["block"] = json!(block);
    }
    table
}

fn source(id: &str, url: &str, tables: Vec<Value>) -> Value {
    json!({"id": id, "url": url, "title": id, "rights": "cited with attribution", "tables": tables})
}

/// The seed with a person's sources: the extract's Esche block and the OSU
/// page.
fn admitted(block: Option<&str>, more: Vec<Value>) -> Value {
    let mut manifest = seed();
    let mut sources = vec![
        source("E1", ERTRAGSTAFELN, vec![table(block)]),
        source("O1", OSU, vec![]),
    ];
    sources.extend(more);
    manifest["sources"] = json!(sources);
    manifest["fields"][0]["proxy"] =
        json!({"source": "E1", "taxon": "Fraxinus excelsior stand-grown site class I"});
    manifest["engineering"] = json!({"curves.mature_dbh_m": {"value": 0.55, "rationale": "midpoint of the profile range"}});
    manifest
}

struct Run {
    dir: PathBuf,
    adapter: FixtureAdapter,
    flow: PathBuf,
}

impl Run {
    /// A scratch pipeline directory on the seed, a fixture directory that
    /// answers the gather queries (the first finds the OSU page), the extract
    /// as a PDF and the OSU page, and a `.flow` tree that already knows the
    /// extract.
    fn new() -> Self {
        let root = std::env::temp_dir().join(format!(
            "jev-fn75-{}-{}",
            std::process::id(),
            telperion_jev::ledger::new_entry_id()
        ));
        let dir = root.join("pipeline");
        let fixture_dir = root.join("fixtures");
        let flow = root.join("flow");
        fs::create_dir_all(&dir).unwrap();
        fs::create_dir_all(&fixture_dir).unwrap();
        for name in [
            "ertragstafeln.md",
            "ertragstafeln.pdf",
            "osu.md",
            "osu.html",
        ] {
            fs::copy(fixtures().join(name), fixture_dir.join(name)).unwrap();
        }
        let (web, research) = gather::queries("Fraxinus excelsior", "European ash");
        let osu =
            json!([{"url": OSU, "title": "Landscape Plants", "snippet": "Fraxinus excelsior"}]);
        let search: serde_json::Map<String, Value> = web
            .iter()
            .enumerate()
            .map(|(i, q)| (q.clone(), if i == 0 { osu.clone() } else { json!([]) }))
            .collect();
        write_canonical(
            &fixture_dir.join("index.json"),
            &json!({
                "scrape": {
                    ERTRAGSTAFELN: {"final_url": ERTRAGSTAFELN, "content_type": "application/pdf", "raw": "ertragstafeln.pdf", "markdown": "ertragstafeln.md"},
                    OSU: {"final_url": OSU, "content_type": "text/html; charset=utf-8", "raw": "osu.html", "markdown": "osu.md"},
                },
                "search": search,
                "research": {research: []},
                "parse": {"E1.pdf": "ertragstafeln.md"},
            }),
        )
        .unwrap();
        write_manifest(&dir, &seed());
        Self::known_tree(&flow);
        Self {
            dir,
            adapter: FixtureAdapter::new(fixture_dir),
            flow,
        }
    }

    /// An evidence tree with an earlier ash manifest that admits the extract
    /// for height and whose run recorded the MoBot page as unavailable, and
    /// what the ash must never see (owner, 2026-09-25): the spruce manifest,
    /// an ash manifest under a proof run's `raw/`, and a spec citing a method
    /// paper in its research section.
    fn known_tree(flow: &Path) {
        let spruce = flow.join("evidence/fn58/validation/norway-spruce");
        fs::create_dir_all(&spruce).unwrap();
        fs::copy(
            Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("../../.flow/evidence/fn58/validation/norway-spruce/manifest.json"),
            spruce.join("manifest.json"),
        )
        .unwrap();
        let earlier = flow.join("evidence/earlier-ash/pipeline");
        fs::create_dir_all(&earlier).unwrap();
        let mut manifest = seed();
        manifest["species"] = json!("earlier-ash");
        manifest["sources"] = json!([
            source("E1", ERTRAGSTAFELN, vec![table(Some("Esche"))]),
            source("M1", MOBOT, vec![]),
            source("O1", RETRIED, vec![])
        ]);
        write_manifest(&earlier, &manifest);
        let raw = flow.join("evidence/beech-proof/raw/ash-proof");
        fs::create_dir_all(&raw).unwrap();
        manifest["sources"] = json!([source("R9", RAW_ONLY, vec![])]);
        write_manifest(&raw, &manifest);
        write_canonical(
            &earlier.join("decisions.json"),
            &json!({"schema": "decisions", "schema_version": 1, "decisions": [{
                "id": "earlier-ash/fetch/unavailable-source/M1", "species": "earlier-ash", "stage": "fetch",
                "kind": "unavailable-source", "status": "open", "blocks": ["extract"], "field": "M1",
                "inputs_sha256": {"url": "x"}, "ledger": [], "options": ["retry", "replace-source", "drop-source"],
                "note": "", "payload": {"source": "M1", "url": MOBOT, "error": TLS_ERROR}
            }, {
                "id": "earlier-ash/fetch/unavailable-source/O1", "species": "earlier-ash", "stage": "fetch",
                "kind": "unavailable-source", "status": "resolved", "blocks": ["extract"], "field": "O1",
                "inputs_sha256": {"url": "y"}, "ledger": [], "options": ["retry", "replace-source", "drop-source"],
                "note": "", "payload": {"source": "O1", "url": RETRIED, "error": "connect ETIMEDOUT"},
                "resolution": {"id": "earlier-ash/fetch/unavailable-source/O1", "inputs_sha256": {"url": "y"}, "option": "retry", "by": "owner", "at": "2026-09-18", "note": ""}
            }]}),
        )
        .unwrap();
        fs::create_dir_all(flow.join("specs")).unwrap();
        fs::write(
            flow.join("specs/fn-11-growth-over-time.md"),
            "# Growth\n\n## Resolved via Research\n\nThe self-organising tree paper: https://algorithmicbotany.org/papers/selforg.sig2009.html.\n\n## Boundaries\n",
        )
        .unwrap();
    }

    /// The run's own paths: this harness keeps the record and the run's
    /// scratch in one directory, as a test and a swap trial do.
    fn paths(&self) -> Paths {
        Paths::new(&self.dir)
    }

    /// What the repository knows, with no catalogue of its own: this run's
    /// tree is the evidence tree and the specs it wrote above.
    fn known(&self) -> KnownSources {
        KnownSources::scan(
            &self.flow.join("catalogue"),
            &self.flow,
            &self.dir.join("manifest.json"),
        )
    }

    fn gather(&self) -> Result<gather::Outcome, StageError> {
        gather::run(&self.paths(), &self.adapter, &self.known())
    }

    fn fetch(&self) -> Result<fetch::Outcome, StageError> {
        fetch::run(&self.paths(), &self.adapter)
    }

    fn decisions(&self) -> Vec<Value> {
        read_json(&self.dir.join("decisions.json")).unwrap_or_default()["decisions"]
            .as_array()
            .cloned()
            .unwrap_or_default()
    }

    fn decision(&self, id: &str) -> Value {
        self.decisions()
            .into_iter()
            .find(|d| d["id"] == id)
            .unwrap_or_else(|| panic!("decision {id}"))
    }

    fn resolve(&self, resolutions: Vec<Value>) {
        write_canonical(
            &self.dir.join("resolutions.json"),
            &json!({"resolutions": resolutions}),
        )
        .unwrap();
    }

    /// A resolution bound to the decision's current inputs.
    fn resolution(&self, id: &str, option: &str, payload: Value) -> Value {
        let mut resolution = json!({
            "id": id, "inputs_sha256": self.decision(id)["inputs_sha256"],
            "option": option, "by": "owner", "at": "2026-09-18", "note": ""
        });
        if !payload.is_null() {
            resolution["payload"] = payload;
        }
        resolution
    }

    fn fetch_body(&self) -> Value {
        read_json(&self.dir.join("fetch.json")).unwrap()["body"].clone()
    }
}

fn write_manifest(dir: &Path, manifest: &Value) {
    fs::write(
        dir.join("manifest.json"),
        serde_json::to_vec_pretty(manifest).unwrap(),
    )
    .unwrap();
}

fn ages(rows: &Value) -> Vec<f64> {
    rows.as_array()
        .unwrap()
        .iter()
        .map(|r| r["age_years"].as_f64().unwrap())
        .collect()
}

/// R1: the documents gather adds to the manifest, and a person's tables on
/// it, do not rerun gather; a seed edit does. R3: the Esche block yields
/// exactly eleven rows from 20 to 120.
#[test]
fn adding_documents_does_not_rerun_gather_and_a_seed_edit_does() {
    let run = Run::new();
    assert!(matches!(run.gather().unwrap(), gather::Outcome::Ran { .. }));
    write_manifest(&run.dir, &admitted(Some("Esche"), vec![]));
    assert!(matches!(run.gather().unwrap(), gather::Outcome::Current));
    assert!(matches!(run.fetch().unwrap(), fetch::Outcome::Ran { .. }));
    let table = &run.fetch_body()["tables"]["E1-ash-height-I"];
    assert_eq!(table["block"], "Esche");
    assert_eq!(table["found_rows"], 11);
    assert_eq!(
        ages(&table["rows"]),
        vec![20.0, 30.0, 40.0, 50.0, 60.0, 70.0, 80.0, 90.0, 100.0, 110.0, 120.0]
    );
    assert_eq!(table["rows"][0]["value"], 12.0);
    assert_eq!(table["rows"][10]["value"], 33.1);
    assert!(run.decisions().iter().all(|d| d["kind"] != "coverage-gap"));

    // A seed edit: one more required age. Gather runs again.
    let mut edited = admitted(Some("Esche"), vec![]);
    edited["fields"][0]["required_ages_years"] = json!([20, 50, 80, 100]);
    write_manifest(&run.dir, &edited);
    assert!(matches!(run.gather().unwrap(), gather::Outcome::Ran { .. }));
    assert!(matches!(run.fetch().unwrap(), fetch::Outcome::Ran { .. }));
}

/// R3: the merged table files coverage-gap at 31 against 11, a missing block
/// label names the labels present, and the three coverage-gap options are
/// consumed by fetch.
#[test]
fn a_merged_table_and_a_missing_block_file_coverage_gap_and_its_options_are_consumed() {
    let run = Run::new();
    write_manifest(&run.dir, &admitted(None, vec![]));
    assert!(
        matches!(run.fetch().unwrap(), fetch::Outcome::Ran { decisions } if decisions == ["european-ash/fetch/coverage-gap/height_m"])
    );
    let id = "european-ash/fetch/coverage-gap/height_m";
    let gap = run.decision(id);
    assert_eq!(gap["payload"]["found_rows"], 31);
    assert_eq!(gap["payload"]["expected_rows"], 11);
    assert_eq!(
        gap["options"],
        json!(["accept-rows", "fix-table", "drop-table"])
    );
    assert_eq!(
        run.fetch_body()["tables"]["E1-ash-height-I"]["found_rows"],
        31
    );

    let fix = run.resolution(id, "fix-table", Value::Null);
    run.resolve(vec![fix]);
    let err = run.fetch().unwrap_err().to_string();
    assert_eq!(
        err,
        format!("fetch: coverage-gap {id}: resolved fix-table, but the table still yields 31 rows against 11")
    );

    let accept = run.resolution(id, "accept-rows", Value::Null);
    run.resolve(vec![accept]);
    assert!(
        matches!(run.fetch().unwrap(), fetch::Outcome::Ran { decisions } if decisions.is_empty())
    );
    let table = &run.fetch_body()["tables"]["E1-ash-height-I"];
    assert_eq!(table["accepted_rows"], true);
    assert_eq!(table["found_rows"], 31);
    assert_eq!(run.decision(id)["consumed_by"], "fetch");

    let drop = run.resolution(id, "drop-table", Value::Null);
    run.resolve(vec![drop]);
    assert!(
        matches!(run.fetch().unwrap(), fetch::Outcome::Ran { decisions } if decisions.is_empty())
    );
    let table = &run.fetch_body()["tables"]["E1-ash-height-I"];
    assert_eq!(table["dropped"], true);
    assert_eq!(table["rows"], json!([]));

    // Fixing the table entry to name the block voids the old resolution
    // (its inputs changed) and the block yields the eleven rows, no gap.
    write_manifest(&run.dir, &admitted(Some("Esche"), vec![]));
    run.resolve(vec![]);
    assert!(
        matches!(run.fetch().unwrap(), fetch::Outcome::Ran { decisions } if decisions.is_empty())
    );
    assert_eq!(
        run.fetch_body()["tables"]["E1-ash-height-I"]["found_rows"],
        11
    );

    write_manifest(&run.dir, &admitted(Some("Eiche"), vec![]));
    assert!(matches!(run.fetch().unwrap(), fetch::Outcome::Ran { decisions } if decisions == [id]));
    let gap = run.decision(id);
    assert_eq!(gap["status"], "open");
    assert_eq!(gap["payload"]["block"], "Eiche");
    assert_eq!(
        gap["payload"]["labels_found"],
        json!([
            "Esche",
            "Schwarzerle",
            "Birke",
            "Robusta-Pappel",
            "Marilandica-Pappel, Nordbaden"
        ])
    );
    assert_eq!(
        run.fetch_body()["tables"]["E1-ash-height-I"]["found_rows"],
        0
    );
}

/// R4: the repository's sources lead the hits before any search, with their
/// origin, and join the manifest as documents; a source whose run recorded a
/// fetch error is listed with it and never added.
#[test]
fn gather_lists_the_repository_sources_first_and_never_adds_one_with_a_fetch_error() {
    let run = Run::new();
    let known = run.known();
    let urls: Vec<&str> = known.sources.iter().map(|s| s.url.as_str()).collect();
    assert!(urls.contains(&ERTRAGSTAFELN), "{urls:?}");
    assert!(urls.contains(&MOBOT), "{urls:?}");
    // Another species, a spec's method paper and raw evidence are never known.
    for never in [
        "https://hortnews.extension.iastate.edu/norway-spruce",
        "https://algorithmicbotany.org/papers/selforg.sig2009.html",
        RAW_ONLY,
    ] {
        assert!(!urls.contains(&never), "{never}: {urls:?}");
    }
    let for_height: Vec<&str> = known
        .for_field("height_m")
        .iter()
        .map(|s| s.url.as_str())
        .collect();
    assert!(for_height.contains(&ERTRAGSTAFELN), "{for_height:?}");
    // A source with tables is named for their dimensions; one without, for every field.
    let for_crown: Vec<&str> = known
        .for_field("crown_width_m")
        .iter()
        .map(|s| s.url.as_str())
        .collect();
    assert!(!for_crown.contains(&ERTRAGSTAFELN), "{for_crown:?}");
    assert!(for_crown.contains(&MOBOT), "{for_crown:?}");
    // An error stands only while its decision is open: the retried source
    // carries none, the open one keeps its error.
    let by_url = |url: &str| known.sources.iter().find(|s| s.url == url).unwrap();
    assert!(
        by_url(RETRIED).error.is_none(),
        "{:?}",
        by_url(RETRIED).error
    );
    assert!(by_url(MOBOT).error.is_some());

    run.gather().unwrap();
    let body = read_json(&run.dir.join("gather.json")).unwrap();
    let hits = body["body"]["hits"].as_array().unwrap();
    let kinds: Vec<&str> = hits.iter().map(|h| h["kind"].as_str().unwrap()).collect();
    let first_searched = kinds.iter().position(|k| *k != "known").unwrap();
    assert!(
        kinds[..first_searched].iter().all(|k| *k == "known"),
        "{kinds:?}"
    );
    assert!(
        kinds[first_searched..].iter().all(|k| *k != "known"),
        "{kinds:?}"
    );
    let extract = hits
        .iter()
        .find(|h| h["url"] == ERTRAGSTAFELN)
        .expect("the extract is a hit");
    assert!(
        extract["origin"].as_str().unwrap().starts_with("manifest:"),
        "{}",
        extract["origin"]
    );
    assert!(extract["origin"].as_str().unwrap().ends_with("#E1"));
    let mobot = hits
        .iter()
        .find(|h| h["url"] == MOBOT)
        .expect("the errored source is listed");
    assert_eq!(mobot["error"], TLS_ERROR);
    let manifest = read_json(&run.dir.join("manifest.json")).unwrap();
    let added: Vec<&str> = manifest["sources"]
        .as_array()
        .unwrap()
        .iter()
        .map(|s| s["url"].as_str().unwrap())
        .collect();
    assert_eq!(added, [ERTRAGSTAFELN, RETRIED, OSU], "{added:?}");
    assert_eq!(hits.iter().filter(|h| h["url"] == OSU).count(), 1);
    assert!(body["ledger"].as_array().unwrap().is_empty());
}

/// R5's error case: a page the store rejects is dropped with the adapter's
/// error verbatim, nothing is fetched in its place, and the other sources
/// are still fetched (fn-130, fn-157).
#[test]
fn a_page_the_store_rejects_is_dropped_with_the_error_verbatim() {
    let run = Run::new();
    write_manifest(
        &run.dir,
        &admitted(Some("Esche"), vec![source("M1", MOBOT, vec![])]),
    );
    run.fetch().unwrap();
    let body = run.fetch_body();
    let recorded = body["dropped"]["M1"]["error"].as_str().unwrap();
    assert!(
        recorded.starts_with("fetch failed for https://plantfinder.mobot.org/"),
        "{recorded}"
    );
    assert!(body["sources"].get("M1").is_none());
    assert!(body["sources"].get("E1").is_some() && body["sources"].get("O1").is_some());
    assert!(run.decisions().is_empty());
}

/// R6: every artifact records what its stage spent, Firecrawl credits and
/// Jev calls, and a rerun from the recorded seed shows one gather.
#[test]
fn every_artifact_records_its_cost_and_a_rerun_shows_one_gather() {
    let run = Run::new();
    run.gather().unwrap();
    write_manifest(&run.dir, &admitted(Some("Esche"), vec![]));
    run.fetch().unwrap();
    assert!(matches!(run.gather().unwrap(), gather::Outcome::Current));
    assert!(matches!(run.fetch().unwrap(), fetch::Outcome::Current));

    let gather_cost = read_json(&run.dir.join("gather.json")).unwrap()["cost"].clone();
    assert_eq!(gather_cost["runs"], 1);
    assert_eq!(gather_cost["jev_calls"], 0);
    // Five web searches and one research search; the fixture prices nothing.
    assert_eq!(gather_cost["firecrawl_credits"], 6);
    assert!(gather_cost["firecrawl_method"]
        .as_str()
        .unwrap()
        .starts_with("estimated"));
    let fetch_cost = read_json(&run.dir.join("fetch.json")).unwrap()["cost"].clone();
    // Two scrapes and one PDF parse.
    assert_eq!(fetch_cost["firecrawl_credits"], 3);
    assert_eq!(fetch_cost["jev_calls"], 0);
}
