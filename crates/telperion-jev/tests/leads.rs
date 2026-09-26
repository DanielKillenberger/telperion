//! fn-149, owner 2026-09-25: Wikipedia is a lead, never a citation. A
//! gather whose only hit is a Wikipedia page adds the primary sources it
//! cites and never the page (fn-157); a tertiary source in the manifest is
//! never fetched. Fixture adapter: no network, no key.
use std::fs;
use std::path::PathBuf;

use serde_json::{json, Value};
use telperion_jev::pipeline::adapter::FixtureAdapter;
use telperion_jev::pipeline::canon::{read_json, write_canonical};
use telperion_jev::pipeline::known::KnownSources;
use telperion_jev::pipeline::stage::Paths;
use telperion_jev::pipeline::stages::fetch;
use telperion_jev::pipeline::stages::gather::{self, queries};

const WIKI: &str = "https://en.wikipedia.org/wiki/Fagus_sylvatica";
const SILVICS: &str = "https://example.test/silvics/beech";
const PAPER: &str = "https://doi.org/10.1000/beech";
const TAXON: &str = "Fagus sylvatica";

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
    let (web, research) = queries(TAXON, "European beech");
    let hit = json!([{"url": WIKI, "title": "Fagus sylvatica - Wikipedia", "snippet": "beech"}]);
    let search: serde_json::Map<String, Value> = web
        .iter()
        .enumerate()
        .map(|(i, q)| (q.clone(), if i == 0 { hit.clone() } else { json!([]) }))
        .collect();
    write_canonical(
        &fixtures.join("index.json"),
        &json!({
            "scrape": {WIKI: page(WIKI, "wiki.html", "wiki.md"),
                       SILVICS: page(SILVICS, "primary.html", "primary.md"),
                       PAPER: page(PAPER, "primary.html", "primary.md")},
            "search": search, "research": {research: []},
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
fn a_wikipedia_hit_yields_its_primary_references_and_no_wikipedia_source() {
    let (dir, adapter) = scratch("gather", manifest(json!([])));
    let known = KnownSources::default();
    gather::run(&Paths::new(&dir), &adapter, &known).unwrap();
    let body = read_json(&dir.join("gather.json")).unwrap()["body"].clone();
    assert_eq!(urls(&body["hits"]), [SILVICS, PAPER], "{body}");
    let manifest = read_json(&dir.join("manifest.json")).unwrap();
    assert_eq!(urls(&manifest["sources"]), [SILVICS, PAPER], "{manifest}");
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

/// The beech run of 2026-09-25 ranked nothing for five of six fields: each
/// lead reached Jev as a bare URL "cited by Wikipedia". A lead now carries
/// its citation and the sentence that cites it, one link per reference.
#[test]
fn a_lead_carries_its_citation_and_the_sentence_that_cites_it() {
    let page = fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures/leads/fagus-sylvatica.md"),
    )
    .unwrap();
    let found = telperion_jev::pipeline::leads::primary(&page, "Fagus sylvatica - Wikipedia");
    let urls: Vec<&str> = found.iter().map(|h| h.url.as_str()).collect();
    assert_eq!(
        urls,
        [
            "https://doi.org/10.2305%2FIUCN.UK.2018-1.RLTS.T62004722A62004725.en",
            "https://powo.science.kew.org/taxon/urn:lsid:ipni.org:names:305836-2",
            "https://mortonarb.org/plant-and-protect/trees-and-plants/european-beech/",
            "http://bomeninfo.nl/tall%20trees.htm",
            "https://doi.org/10.1080%2F03071375.2013.767078",
        ]
    );
    let tall = &found[3];
    assert!(
        tall.title.starts_with("Tall Trees. Bomeninfo.nl."),
        "{}",
        tall.title
    );
    assert!(
        tall.snippet
            .contains("capable of reaching heights of up to 50 metres (160 feet) tall"),
        "{}",
        tall.snippet
    );
    let old = &found[4];
    assert!(
        old.snippet
            .contains("Undisturbed, the European beech has a lifespan of 300 years"),
        "{}",
        old.snippet
    );
    assert!(!old.snippet.contains("sapling"), "{}", old.snippet);
}

/// fn-157 review: a native-range query that returns a page the broad search
/// found past its cap still adds it; documents are chosen against the
/// sources, never against every hit seen.
#[test]
fn a_native_query_reads_a_page_the_broad_search_left_past_its_cap() {
    let full: Vec<Value> = (1..=15)
        .map(|i| {
            json!({"id": format!("S{i}"), "url": format!("https://example.test/s{i}"),
                        "title": "s", "rights": "cited"})
        })
        .collect();
    let mut seed = manifest(json!(full));
    seed["taxon"]["native_range"] = json!({"region": "Europe"});
    let (dir, _) = scratch("native", seed.clone());
    let fixtures = dir.parent().unwrap().join("fixtures");
    let mut index = read_json(&fixtures.join("index.json")).unwrap();
    let taxon: telperion_jev::pipeline::manifest::Taxon =
        serde_json::from_value(seed["taxon"].clone()).unwrap();
    for query in gather::native_queries(&taxon) {
        let hit = json!([{"url": PAPER, "title": "A paper", "snippet": "beech"}]);
        index["search"][query] = hit;
    }
    write_canonical(&fixtures.join("index.json"), &index).unwrap();
    gather::run(
        &Paths::new(&dir),
        &FixtureAdapter::new(fixtures),
        &KnownSources::default(),
    )
    .unwrap();
    let manifest = read_json(&dir.join("manifest.json")).unwrap();
    let added = &urls(&manifest["sources"])[15..];
    // The broad search had room for one of the lead's two references; the
    // native query adds the other.
    assert_eq!(added, [SILVICS, PAPER], "{manifest}");
}
