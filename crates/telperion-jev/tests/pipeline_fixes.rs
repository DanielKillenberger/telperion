//! The six fixes from the first ash run (fn-75), on the ash's own shape over
//! the fixture adapter and the mock transport: the seed manifest, the
//! admitted manifest with the Ertragstafeln extract's Esche block, a source
//! the adapter cannot fetch, and a repository that already knows the extract.
//! No network, no key, no binary.

mod common;

use std::fs;
use std::path::{Path, PathBuf};

use common::{ledger_dir, CaseTransport};
use serde_json::{json, Value};
use telperion_jev::caller::{HttpRequest, HttpResponse, Transport};
use telperion_jev::pipeline::adapter::FixtureAdapter;
use telperion_jev::pipeline::canon::{read_json, write_canonical};
use telperion_jev::pipeline::judge::Judge;
use telperion_jev::pipeline::known::KnownSources;
use telperion_jev::pipeline::stage::{Context, Paths, StageError};
use telperion_jev::pipeline::stages::{discover, fetch};

const ERTRAGSTAFELN: &str = "https://www.forstpraxis.de/sites/forstpraxis.de/files/2023-07/AFZ_FHJ_Kalender_2024_306_318_Ertragstafeln_ste_OK.pdf";
const OSU: &str = "https://landscapeplants.oregonstate.edu/plants/fraxinus-excelsior";
const MOBOT: &str = "https://plantfinder.mobot.org/PlantFinderDetails.aspx?taxonid=282928";
/// A source an earlier run failed on and then retried: known, without an error.
const RETRIED: &str = "https://www.tree-guide.com/ash";
const TLS_ERROR: &str = "fetch failed for https://plantfinder.mobot.org/PlantFinderDetails.aspx?taxonid=282928: Connection Failed: tls connection init failed: invalid peer certificate: UnknownIssuer";
const HEIGHT_QUERY: &str = "Fraxinus excelsior height at age, open grown";
const DBH_QUERY: &str = "Fraxinus excelsior trunk diameter at breast height at age, open grown";

/// Jev ranks the Ertragstafeln extract when it is a candidate, else the
/// first one; everything else answers as the upstream mock.
struct RankExtract(CaseTransport);

impl Transport for RankExtract {
    fn send(&self, request: &HttpRequest) -> Result<HttpResponse, String> {
        let body: Value = serde_json::from_slice(request.body.as_deref().unwrap_or(b"{}")).unwrap();
        if body["questions"].get("source").is_none() {
            return self.0.send(request);
        }
        let choice = body["state"]["candidates"]
            .as_array()
            .into_iter()
            .flatten()
            .find(|hit| hit["url"] == ERTRAGSTAFELN)
            .and_then(|hit| hit["key"].as_str())
            .unwrap_or("h1")
            .to_string();
        Ok(HttpResponse {
            status: 200,
            body: serde_json::to_vec(&json!({"model": "jev-latest", "answers": {
                "source": {"type": "choice", "choice": choice, "confidence": 0.9, "probabilities": {choice.clone(): 0.9}}
            }, "usage": {"input_tokens": 1, "output_tokens": 1}}))
            .unwrap(),
        })
    }
}

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
        "versions": {"question_sets": {"ranking": 1, "screen": 1, "sufficiency": 1}, "tools": {"firecrawl": "1.23.3"}},
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

/// The seed with sources admitted: the extract's Esche block and the OSU page.
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
    /// answers the two plain-word queries, the extract as a PDF and the OSU
    /// page, and a `.flow` tree that already knows the extract.
    fn new() -> Self {
        let root = ledger_dir("fn75");
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
        let web = |title: &str, url: &str| json!([{"url": url, "title": title, "snippet": "Fraxinus excelsior"}]);
        write_canonical(
            &fixture_dir.join("index.json"),
            &json!({
                "scrape": {
                    ERTRAGSTAFELN: {"final_url": ERTRAGSTAFELN, "content_type": "application/pdf", "raw": "ertragstafeln.pdf", "markdown": "ertragstafeln.md"},
                    OSU: {"final_url": OSU, "content_type": "text/html; charset=utf-8", "raw": "osu.html", "markdown": "osu.md"},
                },
                "search": {HEIGHT_QUERY: web("Landscape Plants", OSU), DBH_QUERY: web("Landscape Plants", OSU)},
                "research": {HEIGHT_QUERY: [], DBH_QUERY: []},
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

    /// An evidence tree with the spruce manifest that admits the extract for
    /// height and diameter, a manifest whose run recorded the MoBot page as
    /// unavailable, and a spec citing a URL in its research section.
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
        manifest["sources"] = json!([source("M1", MOBOT, vec![]), source("O1", RETRIED, vec![])]);
        write_manifest(&earlier, &manifest);
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

    fn discover(&self) -> Result<discover::Outcome, StageError> {
        let transport = RankExtract(CaseTransport);
        let judge = Judge {
            transport: &transport,
            key: "test-key",
            ledger_dir: self.dir.join("ledger").join("entries"),
        };
        discover::run(&self.paths(), &self.adapter, &judge, &self.known())
    }

    fn fetch(&self) -> Result<fetch::Outcome, StageError> {
        fetch::run(&self.paths(), &self.adapter)
    }

    fn decisions(&self) -> Vec<Value> {
        read_json(&self.dir.join("decisions.json")).unwrap()["decisions"]
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

    fn admit(&self, manifest: &Value) -> Value {
        write_manifest(&self.dir, manifest);
        self.resolution(
            "european-ash/discover/manifest-proposed",
            "admit",
            Value::Null,
        )
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

/// R1: admitting the manifest keeps the proposal and reaches fetch on the
/// first dispatch; a seed edit reruns discovery and the stop names the new
/// proposal. R3: the Esche block yields exactly eleven rows from 20 to 120.
#[test]
fn admitting_the_manifest_does_not_rerun_discovery_and_a_seed_edit_does() {
    let run = Run::new();
    assert!(matches!(
        run.discover().unwrap(),
        discover::Outcome::Ran { .. }
    ));
    let proposal = run.decision("european-ash/discover/manifest-proposed");
    assert!(proposal["inputs_sha256"].get("seed").is_some());
    assert!(proposal["inputs_sha256"].get("draft").is_none());

    run.resolve(vec![run.admit(&admitted(Some("Esche"), vec![]))]);
    assert!(matches!(
        run.discover().unwrap(),
        discover::Outcome::Current
    ));
    assert!(matches!(run.fetch().unwrap(), fetch::Outcome::Ran { .. }));
    assert_eq!(
        run.decision("european-ash/discover/manifest-proposed")["consumed_by"],
        "fetch"
    );
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

    // A seed edit: one more required age. Discovery reruns and reissues the
    // proposal under a new seed; the old admission is void and fetch stops on it.
    let mut edited = admitted(Some("Esche"), vec![]);
    edited["fields"][0]["required_ages_years"] = json!([20, 50, 80, 100]);
    write_manifest(&run.dir, &edited);
    assert!(matches!(
        run.discover().unwrap(),
        discover::Outcome::Ran { .. }
    ));
    let err = run.fetch().unwrap_err();
    assert_eq!(
        err.to_string(),
        "fetch: open decision: european-ash/discover/manifest-proposed"
    );
}

/// R2: each option on unavailable-source changes the next fetch as it says
/// and records the consuming stage; an option no stage consumes and a
/// replace-source without a url are refused by name; a retry that fails
/// again reopens the decision (fn-130).
#[test]
fn each_unavailable_source_option_changes_the_next_fetch_and_the_rest_are_refused() {
    let run = Run::new();
    run.discover().unwrap();
    let admission = run.admit(&admitted(Some("Esche"), vec![source("M1", MOBOT, vec![])]));
    run.resolve(vec![admission.clone()]);
    let id = "european-ash/fetch/unavailable-source/M1";
    assert!(matches!(run.fetch().unwrap(), fetch::Outcome::Ran { decisions } if decisions == [id]));
    let error = run.decision(id)["payload"]["error"].clone();
    assert!(
        error.as_str().unwrap().starts_with("fetch failed for"),
        "{error}"
    );

    let refused = run.resolution(id, "ignore", Value::Null);
    run.resolve(vec![admission.clone(), refused]);
    let err = run.fetch().unwrap_err().to_string();
    assert_eq!(
        err,
        format!("fetch: resolution refused: {id}: option ignore of kind unavailable-source is consumed by no stage; fetch consumes retry, replace-source, drop-source")
    );

    let without_url = run.resolution(id, "replace-source", json!({"note": "the OSU page"}));
    run.resolve(vec![admission.clone(), without_url]);
    let err = run.fetch().unwrap_err().to_string();
    assert!(
        err.ends_with("replace-source names no replacement url in its payload"),
        "{err}"
    );

    let drop = run.resolution(id, "drop-source", Value::Null);
    run.resolve(vec![admission.clone(), drop]);
    assert!(matches!(run.fetch().unwrap(), fetch::Outcome::Ran { .. }));
    let body = run.fetch_body();
    assert_eq!(body["dropped"]["M1"]["option"], "drop-source");
    assert!(body["sources"].get("M1").is_none());
    assert_eq!(run.decision(id)["consumed_by"], "fetch");
    assert!(matches!(run.fetch().unwrap(), fetch::Outcome::Current));

    let replace = run.resolution(id, "replace-source", json!({"url": OSU}));
    run.resolve(vec![admission.clone(), replace]);
    assert!(matches!(run.fetch().unwrap(), fetch::Outcome::Ran { .. }));
    let record = &run.fetch_body()["sources"]["M1"];
    assert_eq!(record["url"], OSU);
    assert_eq!(record["replaced_url"], MOBOT);
    assert_eq!(record["final_url"], OSU);
    assert!(run.fetch_body()["dropped"].as_object().unwrap().is_empty());

    let retry = run.resolution(id, "retry", Value::Null);
    run.resolve(vec![admission, retry]);
    assert!(matches!(run.fetch().unwrap(), fetch::Outcome::Ran { .. }));
    assert!(run.fetch_body()["sources"].get("M1").is_none());
    assert_eq!(run.decision(id)["status"], "open");
}

/// R3: the merged table files coverage-gap at 31 against 11, a missing block
/// label names the labels present, and the three coverage-gap options are
/// consumed by fetch.
#[test]
fn a_merged_table_and_a_missing_block_file_coverage_gap_and_its_options_are_consumed() {
    let run = Run::new();
    run.discover().unwrap();
    let admission = run.admit(&admitted(None, vec![]));
    run.resolve(vec![admission.clone()]);
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
    run.resolve(vec![admission.clone(), fix]);
    let err = run.fetch().unwrap_err().to_string();
    assert_eq!(
        err,
        format!("fetch: coverage-gap {id}: resolved fix-table, but the table still yields 31 rows against 11")
    );

    let accept = run.resolution(id, "accept-rows", Value::Null);
    run.resolve(vec![admission.clone(), accept]);
    assert!(
        matches!(run.fetch().unwrap(), fetch::Outcome::Ran { decisions } if decisions.is_empty())
    );
    let table = &run.fetch_body()["tables"]["E1-ash-height-I"];
    assert_eq!(table["accepted_rows"], true);
    assert_eq!(table["found_rows"], 31);
    assert_eq!(run.decision(id)["consumed_by"], "fetch");

    let drop = run.resolution(id, "drop-table", Value::Null);
    run.resolve(vec![admission.clone(), drop]);
    assert!(
        matches!(run.fetch().unwrap(), fetch::Outcome::Ran { decisions } if decisions.is_empty())
    );
    let table = &run.fetch_body()["tables"]["E1-ash-height-I"];
    assert_eq!(table["dropped"], true);
    assert_eq!(table["rows"], json!([]));

    // Fixing the table entry to name the block voids the old resolution
    // (its inputs changed) and the block yields the eleven rows, no gap.
    write_manifest(&run.dir, &admitted(Some("Esche"), vec![]));
    run.resolve(vec![admission.clone()]);
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

/// R4: the repository's sources lead the candidate list before any search,
/// with their origin; the extract is ranked for height on the ash seed; a
/// source whose run recorded a fetch error is listed with it and never
/// proposed.
#[test]
fn discovery_lists_the_repository_sources_first_and_never_proposes_one_with_a_fetch_error() {
    let run = Run::new();
    let known = run.known();
    let urls: Vec<&str> = known.sources.iter().map(|s| s.url.as_str()).collect();
    assert!(urls.contains(&ERTRAGSTAFELN), "{urls:?}");
    assert!(urls.contains(&MOBOT), "{urls:?}");
    assert!(
        urls.contains(&"https://algorithmicbotany.org/papers/selforg.sig2009.html"),
        "{urls:?}"
    );
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

    run.discover().unwrap();
    let body = read_json(&run.dir.join("discover.json")).unwrap();
    let proposals = body["body"]["proposals"].as_array().unwrap();
    let height = proposals.iter().find(|p| p["field"] == "height_m").unwrap();
    assert_eq!(height["query"], HEIGHT_QUERY);
    let hits = height["hits"].as_array().unwrap();
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
        .expect("the extract is a candidate");
    assert_eq!(extract["kind"], "known");
    assert!(
        extract["origin"].as_str().unwrap().starts_with("manifest:"),
        "{}",
        extract["origin"]
    );
    assert!(extract["origin"].as_str().unwrap().ends_with("#E1"));
    assert_eq!(extract["ranked_first"], true);
    assert!(extract["snippet"].as_str().unwrap().contains("height_m"));

    let mobot = hits
        .iter()
        .find(|h| h["url"] == MOBOT)
        .expect("the errored source is listed");
    assert_eq!(mobot["error"], TLS_ERROR);
    let proposed = body["body"]["draft_manifest"]["proposed_sources"]
        .as_object()
        .unwrap();
    assert!(proposed.values().any(|u| u == ERTRAGSTAFELN));
    assert!(!proposed.values().any(|u| u == MOBOT));

    // The web hit is listed after the known ones, once, and the ranking asked once per field.
    assert_eq!(hits.iter().filter(|h| h["url"] == OSU).count(), 1);
    assert_eq!(body["ledger"].as_array().unwrap().len(), 2);
}

/// R5's error case: a page the store rejects files unavailable-source with
/// the adapter's error verbatim, nothing is fetched in its place, and the
/// other sources are still fetched (fn-130).
#[test]
fn a_page_the_store_rejects_files_unavailable_source_with_the_error_verbatim() {
    let run = Run::new();
    run.discover().unwrap();
    run.resolve(vec![
        run.admit(&admitted(Some("Esche"), vec![source("M1", MOBOT, vec![])]))
    ]);
    run.fetch().unwrap();
    let decision = run.decision("european-ash/fetch/unavailable-source/M1");
    assert_eq!(decision["status"], "open");
    let recorded = decision["payload"]["error"].as_str().unwrap();
    assert!(
        recorded.starts_with("fetch failed for https://plantfinder.mobot.org/"),
        "{recorded}"
    );
    let sources = &run.fetch_body()["sources"];
    assert!(sources.get("M1").is_none());
    assert!(sources.get("E1").is_some() && sources.get("O1").is_some());
}

/// R6: every artifact records what its stage spent, Firecrawl credits and
/// Jev calls, and a rerun from the recorded seed shows one discovery.
#[test]
fn every_artifact_records_its_cost_and_a_rerun_shows_one_discovery() {
    let run = Run::new();
    run.discover().unwrap();
    run.resolve(vec![run.admit(&admitted(Some("Esche"), vec![]))]);
    run.fetch().unwrap();
    assert!(matches!(
        run.discover().unwrap(),
        discover::Outcome::Current
    ));
    assert!(matches!(run.fetch().unwrap(), fetch::Outcome::Current));

    let discover_cost = read_json(&run.dir.join("discover.json")).unwrap()["cost"].clone();
    assert_eq!(discover_cost["runs"], 1);
    assert_eq!(discover_cost["jev_calls"], 2);
    // Two fields, one web search and one research search each; the fixture prices nothing.
    assert_eq!(discover_cost["firecrawl_credits"], 4);
    assert!(discover_cost["firecrawl_method"]
        .as_str()
        .unwrap()
        .starts_with("estimated"));
    let fetch_cost = read_json(&run.dir.join("fetch.json")).unwrap()["cost"].clone();
    // Two scrapes and one PDF parse.
    assert_eq!(fetch_cost["firecrawl_credits"], 3);
    assert_eq!(fetch_cost["jev_calls"], 0);

    // The rejected proposal stops fetch.
    let (ctx, _) = Context::open(&run.paths(), "fetch").unwrap();
    assert_eq!(ctx.decisions.len(), 1);
    run.resolve(vec![run.resolution(
        "european-ash/discover/manifest-proposed",
        "reject",
        Value::Null,
    )]);
    let err = run.fetch().unwrap_err().to_string();
    assert_eq!(
        err,
        "fetch: manifest-proposed european-ash/discover/manifest-proposed was rejected; edit the seed and run discover again"
    );
}
