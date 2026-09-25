//! fn-149, owner 2026-09-25: Wikipedia is a lead, never a citation. A
//! discovery whose only candidate is a Wikipedia page proposes the primary
//! sources it cites and never the page; a tertiary source in the manifest is
//! never fetched. Fixture adapter and a mock transport: no network, no key.
use std::fs;
use std::path::PathBuf;

use serde_json::{json, Value};
use telperion_jev::caller::{HttpRequest, HttpResponse, Transport};
use telperion_jev::pipeline::adapter::FixtureAdapter;
use telperion_jev::pipeline::canon::{read_json, write_canonical};
use telperion_jev::pipeline::judge::Judge;
use telperion_jev::pipeline::known::KnownSources;
use telperion_jev::pipeline::stage::Paths;
use telperion_jev::pipeline::stages::discover::{self, plain_query};
use telperion_jev::pipeline::stages::fetch;

const WIKI: &str = "https://en.wikipedia.org/wiki/Fagus_sylvatica";
const SILVICS: &str = "https://example.test/silvics/beech";
const PAPER: &str = "https://doi.org/10.1000/beech";
const TAXON: &str = "Fagus sylvatica";

/// Ranks the first candidate first and classes every page open-licence.
struct Mock;

impl Transport for Mock {
    fn send(&self, request: &HttpRequest) -> Result<HttpResponse, String> {
        let body: Value = serde_json::from_slice(request.body.as_deref().unwrap_or(b"{}")).unwrap();
        let choice = |q: &str, key: &str| json!({q: {"type": "choice", "choice": key, "confidence": 0.9, "probabilities": {key: 0.9}}});
        let answers = if body["questions"].get("source").is_some() {
            choice("source", "h1")
        } else if body["questions"].get("rights").is_some() {
            choice("rights", "open-licence")
        } else {
            return Err(format!("unexpected question {}", body["questions"]));
        };
        let body = json!({"model": "jev-latest", "answers": answers});
        Ok(HttpResponse {
            status: 200,
            body: serde_json::to_vec(&body).unwrap(),
        })
    }
}

fn manifest(sources: Value) -> Value {
    json!({
        "schema": "manifest", "schema_version": 1, "species": "european-beech",
        "taxon": {"scientific_name": TAXON, "common_name": "European beech", "rank": "species"},
        "context": "mature open-grown", "growth_form": "broadleaf",
        "preset": "european-beech", "profile_id": "european-beech", "seed": 7,
        "sources": sources,
        "fields": [{"field": "height_m", "condition": "open_grown", "required_ages_years": [100], "bar": "partial", "question": "q"}],
        "versions": {"question_sets": {}, "tools": {}}, "model": "jev-latest"
    })
}

/// A species folder on `manifest` and a fixture whose search finds only the
/// Wikipedia page, which cites the silvics manual and a paper.
fn scratch(tag: &str, manifest: Value) -> (PathBuf, FixtureAdapter) {
    let root = std::env::temp_dir().join(format!(
        "jev-leads-{tag}-{}-{}",
        std::process::id(),
        telperion_jev::ledger::new_entry_id()
    ));
    let (dir, fixtures) = (root.join("species"), root.join("fixtures"));
    fs::create_dir_all(&dir).unwrap();
    fs::create_dir_all(&fixtures).unwrap();
    write_canonical(&dir.join("manifest.json"), &manifest).unwrap();
    let wiki = format!(
        "Fagus sylvatica is a beech [see](https://en.wikipedia.org/wiki/Beech).\n\n## References\n\n\
         1. [Silvics of beech]({SILVICS})\n2. [A paper]({PAPER})\n3. [Q](https://www.wikidata.org/wiki/Q1)\n"
    );
    fs::write(fixtures.join("wiki.md"), wiki).unwrap();
    fs::write(fixtures.join("wiki.html"), "<p>wiki</p>").unwrap();
    let licence = "<html><head><link rel=\"license\" href=\"https://creativecommons.org/licenses/by/4.0/\"></head><body>Beech reaches 30 m.</body></html>";
    fs::write(fixtures.join("primary.html"), licence).unwrap();
    fs::write(fixtures.join("primary.md"), "Beech reaches 30 m.").unwrap();
    let page = |url: &str, raw: &str, md: &str| json!({"final_url": url, "content_type": "text/html", "raw": raw, "markdown": md});
    let query = plain_query(TAXON, "height_m", "open_grown");
    let hit = json!([{"url": WIKI, "title": "Fagus sylvatica - Wikipedia", "snippet": "beech"}]);
    write_canonical(
        &fixtures.join("index.json"),
        &json!({
            "scrape": {WIKI: page(WIKI, "wiki.html", "wiki.md"),
                       SILVICS: page(SILVICS, "primary.html", "primary.md"),
                       PAPER: page(PAPER, "primary.html", "primary.md")},
            "search": {query.clone(): hit}, "research": {query: []},
        }),
    )
    .unwrap();
    (dir, FixtureAdapter::new(fixtures))
}

fn urls(sources: &Value) -> Vec<&str> {
    sources
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(|s| s["url"].as_str())
        .collect()
}

#[test]
fn a_wikipedia_candidate_yields_its_primary_references_and_no_wikipedia_source() {
    let (dir, adapter) = scratch("discover", manifest(json!([])));
    let judge = Judge {
        transport: &Mock,
        key: "test-key",
        ledger_dir: dir.join("ledger").join("entries"),
    };
    let known = KnownSources::default();
    discover::run(&Paths::new(&dir), &adapter, &judge, &known).unwrap();
    let body = read_json(&dir.join("discover.json")).unwrap()["body"].clone();
    let hits = &body["proposals"][0]["hits"];
    assert_eq!(urls(hits), [SILVICS, PAPER], "{hits}");
    let draft = &body["draft_manifest"]["sources"];
    assert_eq!(urls(draft), [SILVICS], "{draft}");
}

#[test]
fn a_tertiary_source_in_the_manifest_is_never_fetched() {
    let wiki = json!([{"id": "S1", "url": WIKI, "title": "Wikipedia", "rights": "cc-by-sa"}]);
    let (dir, adapter) = scratch("fetch", manifest(wiki));
    fetch::run(&Paths::new(&dir), &adapter).unwrap();
    let body = read_json(&dir.join("fetch.json")).unwrap()["body"].clone();
    assert!(body["sources"].get("S1").is_none(), "{body}");
    assert!(body["dropped"]["S1"]["option"]
        .as_str()
        .unwrap()
        .starts_with("tertiary"));
}
