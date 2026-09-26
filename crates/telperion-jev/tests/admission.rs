//! fn-129: the pipeline admits its own sources. Discovery admits a draft
//! whose new sources are chosen for their field and carry an admitting
//! rights class, and leaves any other to the owner (R1); a requirement
//! unmet is searched again with a query aimed at its gap, twice at most,
//! before the owner has it with the sources tried (R3). Fixture adapter and
//! a mock transport: no network, no key.

use std::fs;
use std::path::{Path, PathBuf};

use serde_json::{json, Value};
use telperion_jev::caller::{HttpRequest, HttpResponse, Transport};
use telperion_jev::pipeline::adapter::FixtureAdapter;
use telperion_jev::pipeline::canon::{read_json, write_canonical};
use telperion_jev::pipeline::consume::{open_requirements, sources_sha256, REQUIREMENTS_UNMET};
use telperion_jev::pipeline::decision::{
    reconcile, write_decisions, Decision, DecisionParts, Status,
};
use telperion_jev::pipeline::judge::Judge;
use telperion_jev::pipeline::known::KnownSources;
use telperion_jev::pipeline::requirements::table;
use telperion_jev::pipeline::search::{self, gap_query};
use telperion_jev::pipeline::stage::{Context, Paths, StageError};
use telperion_jev::pipeline::stages::discover::{self, plain_query};

const S1: &str = "https://example.test/s1";
const FOUND: &str = "https://example.test/found";
const TAXON: &str = "Quercus garryana";

/// Picks the first candidate for a ranking and the given class for rights.
struct Mock {
    class: &'static str,
}

impl Transport for Mock {
    fn send(&self, request: &HttpRequest) -> Result<HttpResponse, String> {
        let body: Value = serde_json::from_slice(request.body.as_deref().unwrap_or(b"{}")).unwrap();
        let choice = |q: &str, key: &str| json!({q: {"type": "choice", "choice": key, "confidence": 0.9, "probabilities": {key: 0.9}}});
        let answers = if let Some(source) = body["questions"].get("source") {
            let first = source["criteria"]
                .as_object()
                .and_then(|c| c.keys().find(|k| k.starts_with('h')).cloned())
                .unwrap_or_else(|| "none".into());
            choice("source", &first)
        } else if body["questions"].get("rights").is_some() {
            choice("rights", self.class)
        } else {
            return Err(format!("unexpected question {}", body["questions"]));
        };
        Ok(HttpResponse {
            status: 200,
            body: serde_json::to_vec(&json!({"model": "jev-latest", "answers": answers})).unwrap(),
        })
    }
}

/// A broadleaf manifest at version 2 covering the table, with one source.
fn manifest() -> Value {
    let form = &table().growth_forms["broadleaf"];
    let fields: Vec<Value> = form
        .fields
        .iter()
        .map(|(name, bar)| json!({"field": name, "condition": "open_grown", "required_ages_years": [100], "bar": bar.key(), "question": "q"}))
        .collect();
    let appearance: Vec<Value> = form
        .appearance
        .iter()
        .map(|name| json!({"trait_name": name, "sources": ["S1"]}))
        .collect();
    json!({
        "schema": "manifest", "schema_version": 2, "species": "oregon-white-oak",
        "taxon": {"scientific_name": TAXON, "common_name": "Oregon white oak", "rank": "species"},
        "context": "mature open-grown", "growth_form": "broadleaf",
        "preset": "oregon-white-oak", "profile_id": "oregon-white-oak", "seed": 7,
        "sources": [{"id": "S1", "url": S1, "title": "Silvics", "rights": "public domain"}],
        "fields": fields, "appearance": appearance,
        "versions": {"question_sets": {}, "tools": {}}, "model": "jev-latest"
    })
}

/// A species folder and a fixture index: every discovery query finds
/// nothing but `found` answers `hit_query`, and the found page carries a
/// licence line.
fn scratch(tag: &str, hit_queries: &[String]) -> (PathBuf, FixtureAdapter) {
    let root = std::env::temp_dir().join(format!(
        "jev-admission-{tag}-{}-{}",
        std::process::id(),
        telperion_jev::ledger::new_entry_id()
    ));
    let (dir, fixtures) = (root.join("species"), root.join("fixtures"));
    fs::create_dir_all(&dir).unwrap();
    fs::create_dir_all(&fixtures).unwrap();
    fs::write(
        dir.join("manifest.json"),
        serde_json::to_vec_pretty(&manifest()).unwrap(),
    )
    .unwrap();
    let page = "<html><head><link rel=\"license\" href=\"https://creativecommons.org/licenses/by-sa/4.0/\"></head><body>Oaks reach 20 m at 100 years.</body></html>";
    fs::write(fixtures.join("found.html"), page).unwrap();
    fs::write(fixtures.join("found.md"), "Oaks reach 20 m at 100 years.").unwrap();
    let mut search = serde_json::Map::new();
    let mut research = serde_json::Map::new();
    let names: Vec<String> = manifest()["fields"]
        .as_array()
        .unwrap()
        .iter()
        .map(|f| f["field"].as_str().unwrap().to_string())
        .collect();
    for name in names {
        let query = plain_query(TAXON, &name, "open_grown");
        search.insert(query.clone(), json!([]));
        research.insert(query, json!([]));
    }
    for query in hit_queries {
        let hit = json!([{"url": FOUND, "title": "Oak growth", "snippet": "Oaks reach 20 m at 100 years."}]);
        search.insert(query.clone(), hit);
        research.insert(query.clone(), json!([]));
    }
    write_canonical(
        &fixtures.join("index.json"),
        &json!({
            "scrape": {FOUND: {"final_url": FOUND, "content_type": "text/html", "raw": "found.html", "markdown": "found.md"}},
            "search": search, "research": research,
        }),
    )
    .unwrap();
    (dir, FixtureAdapter::new(fixtures))
}

fn judge<'a>(transport: &'a Mock, dir: &Path) -> Judge<'a> {
    Judge {
        transport,
        key: "test-key",
        ledger_dir: dir.join("ledger").join("entries"),
    }
}

fn discover(dir: &Path, adapter: &FixtureAdapter, class: &'static str) {
    let mock = Mock { class };
    discover::run(
        &Paths::new(dir),
        adapter,
        &judge(&mock, dir),
        &KnownSources::default(),
    )
    .unwrap();
}

#[test]
fn a_draft_whose_new_source_is_open_is_admitted_by_the_pipeline_with_its_class() {
    let height = plain_query(TAXON, "height_m", "open_grown");
    let (dir, adapter) = scratch("open", &[height]);
    discover(&dir, &adapter, "open-licence");
    let admitted = read_json(&dir.join("manifest.json")).unwrap();
    let added = &admitted["sources"][1];
    assert_eq!(added["url"], FOUND);
    assert_eq!(added["rights_class"], "open-licence");
    assert!(added["rights"]
        .as_str()
        .unwrap()
        .contains("no text is reproduced"));
    let resolution = &read_json(&dir.join("resolutions.json")).unwrap()["resolutions"][0];
    assert_eq!(resolution["option"], "admit");
    assert_eq!(resolution["by"], "pipeline");
    let list = reconcile(&Paths::new(&dir)).unwrap();
    assert_eq!(list[0].kind, "manifest-proposed");
    assert_eq!(list[0].status, Status::Resolved);
    assert!(Context::open(&Paths::new(&dir), "fetch").is_ok());
}

#[test]
fn a_draft_with_a_restricted_or_unclassifiable_source_goes_to_the_owner() {
    for class in ["restricted", "none"] {
        let height = plain_query(TAXON, "height_m", "open_grown");
        let (dir, adapter) = scratch(class, &[height]);
        let before = fs::read(dir.join("manifest.json")).unwrap();
        discover(&dir, &adapter, class);
        assert_eq!(
            fs::read(dir.join("manifest.json")).unwrap(),
            before,
            "{class}"
        );
        assert!(!dir.join("resolutions.json").exists(), "{class}");
        let decisions = read_json(&dir.join("decisions.json")).unwrap();
        let admission = &decisions["decisions"][0]["payload"]["admission"];
        assert_eq!(admission["admitted"], false, "{class}");
        assert_eq!(
            admission["reasons"],
            json!([format!("P2: rights class {class}")]),
            "{class}"
        );
        let err = Context::open(&Paths::new(&dir), "fetch").unwrap_err();
        assert!(
            matches!(err, StageError::OpenDecision { .. }),
            "{class}: {err}"
        );
    }
}

/// An open requirements-unmet decision on height, as `quality` files it.
fn unmet(dir: &Path) -> Decision {
    let sources = sources_sha256(&dir.join("manifest.json")).unwrap();
    let decision = Decision::new(
        DecisionParts {
            species: "oregon-white-oak",
            stage: "quality",
            kind: REQUIREMENTS_UNMET,
            field: Some("height_m"),
            age_years: None,
        },
        &["select", "fit", "generate"],
        [("fetch.json".to_string(), "aaa".to_string())]
            .into_iter()
            .collect(),
        vec![],
        json!({"field": "height_m", "dominant_gap": "no_age_indexed_points",
               "required_ages_uncovered": [100.0], "sources_sha256": sources}),
        &["add-sources"],
        "NEEDS_HUMAN",
    );
    write_decisions(&dir.join("decisions.json"), std::slice::from_ref(&decision)).unwrap();
    decision
}

fn aimed() -> String {
    gap_query(
        TAXON,
        "height_m",
        "open_grown",
        "no_age_indexed_points",
        &[100.0],
    )
}

#[test]
fn an_unmet_requirement_searched_again_admits_a_passing_source_and_resolves_it() {
    let (dir, adapter) = scratch("again", &[aimed()]);
    let decision = unmet(&dir);
    let mock = Mock {
        class: "public-cite-only",
    };
    let outcome = search::run(&Paths::new(&dir), &adapter, &judge(&mock, &dir)).unwrap();
    assert!(
        matches!(&outcome, search::Outcome::Ran { words } if words == &["height_m: admitted P2".to_string()]),
        "{outcome:?}"
    );
    let admitted = read_json(&dir.join("manifest.json")).unwrap();
    assert_eq!(admitted["sources"][1]["rights_class"], "public-cite-only");
    let resolution = &read_json(&dir.join("resolutions.json")).unwrap()["resolutions"][0];
    assert_eq!(resolution["option"], "add-sources");
    assert_eq!(resolution["by"], "pipeline");
    // The sources changed, so the resolution binds and the stages rerun.
    let list = reconcile(&Paths::new(&dir)).unwrap();
    assert_eq!(list[0].id, decision.id);
    assert_eq!(list[0].status, Status::Resolved);
    let rounds = search::read_rounds(&Paths::new(&dir)).unwrap();
    assert_eq!(rounds["height_m"][0].query, aimed());
    assert_eq!(rounds["height_m"][0].admitted, vec!["P2".to_string()]);
}

#[test]
fn two_empty_rounds_hand_the_owner_the_decision_with_the_sources_tried() {
    let (dir, adapter) = scratch("empty", &[aimed()]);
    let decision = unmet(&dir);
    let mock = Mock {
        class: "restricted",
    };
    let paths = Paths::new(&dir);
    for round in 1..=2 {
        let outcome = search::run(&paths, &adapter, &judge(&mock, &dir)).unwrap();
        let expect = format!("height_m: nothing admitted (round {round} of 2)");
        assert!(
            matches!(&outcome, search::Outcome::Ran { words } if words == std::slice::from_ref(&expect)),
            "{outcome:?}"
        );
    }
    assert!(!dir.join("resolutions.json").exists());
    let list = reconcile(&paths).unwrap();
    assert!(search::searchable(&paths, &list).is_empty());
    assert_eq!(open_requirements(&list), vec![decision.id]);
    assert!(matches!(
        search::run(&paths, &adapter, &judge(&mock, &dir)).unwrap(),
        search::Outcome::Nothing
    ));
    // The restricted page was tried once; the second round left it out.
    let rounds = search::read_rounds(&paths).unwrap();
    assert_eq!(search::tried(&rounds, "height_m"), vec![FOUND.to_string()]);
    assert!(rounds["height_m"][1].hits.is_empty());
}

const S2: &str = "https://example.test/s2";

/// An open requirements-unmet decision on `bark_colour`, as `select` files
/// it for the palm's unstated traits; `S2` is admitted but not on the trait.
fn unstated_trait(dir: &Path) -> Decision {
    let mut value = manifest();
    value["sources"]
        .as_array_mut()
        .unwrap()
        .push(json!({"id": "S2", "url": S2, "title": "Bark", "rights": "cited"}));
    fs::write(
        dir.join("manifest.json"),
        serde_json::to_vec_pretty(&value).unwrap(),
    )
    .unwrap();
    let sources = sources_sha256(&dir.join("manifest.json")).unwrap();
    let decision = Decision::new(
        DecisionParts {
            species: "oregon-white-oak",
            stage: "select",
            kind: REQUIREMENTS_UNMET,
            field: Some("bark_colour"),
            age_years: None,
        },
        &["generate"],
        [("manifest".to_string(), "aaa".to_string())]
            .into_iter()
            .collect(),
        vec![],
        json!({"field": "bark_colour", "level": "unstated", "bar": "stated", "sources_tried": ["S1"], "sources_sha256": sources}),
        &["add-sources"],
        "NEEDS_HUMAN",
    );
    write_decisions(&dir.join("decisions.json"), std::slice::from_ref(&decision)).unwrap();
    decision
}

fn trait_sources(dir: &Path) -> Value {
    let manifest = read_json(&dir.join("manifest.json")).unwrap();
    let bark = manifest["appearance"]
        .as_array()
        .unwrap()
        .iter()
        .find(|t| t["trait_name"] == "bark_colour")
        .unwrap();
    bark["sources"].clone()
}

/// fn-129 R6: an unmet appearance trait is searched again like a field. An
/// admitted source the ranking chose joins the trait's list with no rights
/// call; a new one passes the rights check first and joins both lists.
#[test]
fn an_unmet_appearance_trait_is_searched_again_and_its_source_joins_the_trait() {
    let query = gap_query(TAXON, "bark_colour", "", "unstated", &[]);
    for (tag, class, expect_source) in [("admitted", "none", "S2"), ("found", "open-licence", "P3")]
    {
        // The web finds the new page either way; an admitted source ranks first.
        let (dir, adapter) = scratch(tag, std::slice::from_ref(&query));
        let decision = unstated_trait(&dir);
        if tag == "found" {
            // S2 already on the trait: the page found is the only candidate.
            let mut value = read_json(&dir.join("manifest.json")).unwrap();
            for t in value["appearance"].as_array_mut().unwrap() {
                t["sources"] = json!(["S1", "S2"]);
            }
            write_canonical(&dir.join("manifest.json"), &value).unwrap();
        }
        let paths = Paths::new(&dir);
        let outcome = search::run(&paths, &adapter, &judge(&Mock { class }, &dir)).unwrap();
        let word = format!("bark_colour: admitted {expect_source}");
        assert!(
            matches!(&outcome, search::Outcome::Ran { words } if words.contains(&word)),
            "{tag}: {outcome:?}"
        );
        assert_eq!(
            trait_sources(&dir).as_array().unwrap().last().unwrap(),
            expect_source,
            "{tag}"
        );
        let list = reconcile(&paths).unwrap();
        let bark = list.iter().find(|d| d.id == decision.id).unwrap();
        assert_eq!(bark.status, Status::Resolved, "{tag}: {}", bark.note);
        assert_eq!(
            search::read_rounds(&paths).unwrap()["bark_colour"][0].query,
            query
        );
    }
}
