//! fn-130: the literature step reads its sources whole. The fixtures under
//! `fixtures/palm` are short excerpts of the date palm's live run (fn-80,
//! `.flow/evidence/date-palm/pipeline/cache`): M1, P5 and P6 are lines of the
//! cached markdown, P7 and P8 are the cookie wall the scrape returned, and
//! `p7.html` is a cut of P7's raw GET that holds the frond paragraph. No test
//! here reaches Firecrawl or Jev.

use std::cell::RefCell;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};

use serde_json::{json, Value};
use telperion_jev::caller::{HttpRequest, HttpResponse, Transport};
use telperion_jev::extract::candidate_sentences;
use telperion_jev::pipeline::adapter::{FetchAdapter, FirecrawlCli, FixtureAdapter};
use telperion_jev::pipeline::canon::{read_json, write_canonical};
use telperion_jev::pipeline::judge::Judge;
use telperion_jev::pipeline::stage::Paths;
use telperion_jev::pipeline::stages::{extract, fetch, screen};

const P8_ID: &str = "date-palm/fetch/unavailable-source/P8";

fn fixture(name: &str) -> String {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/palm")
        .join(name);
    fs::read_to_string(&path).unwrap_or_else(|err| panic!("{}: {err}", path.display()))
}

fn scratch(tag: &str) -> PathBuf {
    let root = std::env::temp_dir().join(format!(
        "jev-literature-{tag}-{}-{}",
        std::process::id(),
        telperion_jev::ledger::new_entry_id()
    ));
    fs::create_dir_all(&root).unwrap();
    root
}

/// One admitted source: its id, the markdown the scrape returns, and the raw
/// body (`None` wraps the markdown in a page, as a plain article's GET is).
struct Page {
    id: &'static str,
    content_type: &'static str,
    markdown: String,
    raw: Option<String>,
}

fn page(id: &'static str, markdown: &str) -> Page {
    Page {
        id,
        content_type: "text/html; charset=utf-8",
        markdown: markdown.into(),
        raw: None,
    }
}

fn url(id: &str) -> String {
    format!("https://example.org/palm/{id}")
}

/// A pipeline directory with an admitted date-palm manifest over `pages` and
/// a fixture adapter that scrapes each of them.
fn palm(tag: &str, pages: &[Page]) -> (PathBuf, FixtureAdapter) {
    let root = scratch(tag);
    let dir = root.join("pipeline");
    let fixtures = root.join("fixtures");
    fs::create_dir_all(&dir).unwrap();
    fs::create_dir_all(&fixtures).unwrap();
    let mut scrape = serde_json::Map::new();
    for p in pages {
        let raw = p
            .raw
            .clone()
            .unwrap_or_else(|| format!("<html><body>{}</body></html>", p.markdown));
        fs::write(fixtures.join(format!("{}.md", p.id)), &p.markdown).unwrap();
        fs::write(fixtures.join(format!("{}.raw", p.id)), raw).unwrap();
        scrape.insert(
            url(p.id),
            json!({"final_url": url(p.id), "content_type": p.content_type,
                   "raw": format!("{}.raw", p.id), "markdown": format!("{}.md", p.id)}),
        );
    }
    write_canonical(&fixtures.join("index.json"), &json!({"scrape": scrape})).unwrap();
    let sources: Vec<Value> = pages
        .iter()
        .map(|p| json!({"id": p.id, "url": url(p.id), "title": p.id, "rights": "cited"}))
        .collect();
    let manifest = json!({
        "schema": "manifest", "schema_version": 1, "species": "date-palm",
        "taxon": {"scientific_name": "Phoenix dactylifera", "common_name": "Date palm", "rank": "species"},
        "context": "mature cultivated", "growth_form": "broadleaf",
        "preset": "date-palm", "profile_id": "date-palm", "seed": 7,
        "sources": sources,
        "fields": [{"field": "height_m", "condition": "open_grown", "required_ages_years": [30],
                    "bar": "partial", "question": "Which span states the palm's height?"}],
        "versions": {"question_sets": {"screen": 1, "sufficiency": 1}, "tools": {}},
        "model": "jev-latest"
    });
    fs::write(
        dir.join("manifest.json"),
        serde_json::to_vec_pretty(&manifest).unwrap(),
    )
    .unwrap();
    (dir, FixtureAdapter::new(fixtures))
}

fn candidates(dir: &Path) -> Vec<Value> {
    read_json(&dir.join("extract.json")).unwrap()["body"]["candidates"]
        .as_array()
        .unwrap()
        .clone()
}

fn decision(dir: &Path, id: &str) -> Value {
    read_json(&dir.join("decisions.json")).unwrap()["decisions"]
        .as_array()
        .unwrap()
        .iter()
        .find(|d| d["id"] == id)
        .unwrap_or_else(|| panic!("no decision {id}"))
        .clone()
}

/// The cookie wall P8's scrape returned, served as its raw body too: a
/// source with no usable content on either route.
fn cookie_wall() -> Page {
    let wall = fixture("p8.md");
    Page {
        raw: Some(format!(
            "<!DOCTYPE html><html><head><title>PMC</title></head><body>{wall}</body></html>"
        )),
        ..page("P8", &wall)
    }
}

/// R1: the "<" of a p-value in M1 and P6 deletes nothing after it, so M1's
/// leaflet sentence and P6's rainfall sentence reach extract.json.
#[test]
fn a_p_value_deletes_nothing_and_m1s_leaflet_sentence_is_a_candidate() {
    let (dir, adapter) = palm(
        "r1",
        &[page("M1", &fixture("m1.md")), page("P6", &fixture("p6.md"))],
    );
    fetch::run(&Paths::new(&dir), &adapter).unwrap();
    extract::run(&Paths::new(&dir)).unwrap();
    let found = candidates(&dir);
    let has = |source: &str, needle: &str| {
        found
            .iter()
            .any(|c| c["source"] == source && c["sentence"].as_str().unwrap().contains(needle))
    };
    assert!(has("M1", "62.33 cm (Ajwah) to 34.5 cm"), "{found:#?}");
    assert!(has("M1", "4.9 cm (Safawi) to 2.87 cm"), "{found:#?}");
    assert!(has("P6", "only 50 mm on average"), "{found:#?}");
}

/// R2: a decimal point inside "5.5-6.1 m" does not end P5's frond sentence,
/// so its "(0.6 m) wide" stays with the frond's "18-20 ft ... long".
#[test]
fn the_fronds_width_stays_in_the_fronds_sentence() {
    let found = candidate_sentences(&fixture("p5.md"));
    let width = found
        .iter()
        .find(|c| c.sentence.contains("(0.6 m) wide"))
        .expect("the width is a candidate");
    assert!(
        width
            .sentence
            .starts_with("The large greenish or bluish gray pinnate leaves are typically 18-20 ft (5.5-6.1 m) long"),
        "{}",
        width.sentence
    );
    assert!(found.iter().all(|c| !c.sentence.starts_with("1 m)")), "{found:#?}");
    assert!(found.iter().any(|c| c.sentence.contains("3-4 in (7.6-10.2 cm) spines")));
}

/// R3: P7's cookie wall is refused and the raw body's conversion is cached
/// in its place; P8, unusable on both routes, files unavailable-source while
/// P5 is still fetched.
#[test]
fn a_cookie_wall_is_refused_and_a_source_with_no_usable_content_files_unavailable() {
    let p7 = Page {
        raw: Some(fixture("p7.html")),
        ..page("P7", &fixture("p7.md"))
    };
    let (dir, adapter) = palm("r3", &[p7, cookie_wall(), page("P5", &fixture("p5.md"))]);
    let outcome = fetch::run(&Paths::new(&dir), &adapter).unwrap();
    assert!(
        matches!(&outcome, fetch::Outcome::Ran { decisions } if decisions == &[P8_ID]),
        "{outcome:?}"
    );
    let body = &read_json(&dir.join("fetch.json")).unwrap()["body"];
    let p7 = &body["sources"]["P7"];
    assert_eq!(p7["markdown_from"], "raw");
    assert!(
        p7["refused"].as_str().unwrap().contains("Cookies must be enabled"),
        "{p7}"
    );
    let cached = fs::read_to_string(dir.join("cache/P7.md")).unwrap();
    assert!(cached.contains("545.33 cm (Barni Al-Madinah)"), "{cached}");
    assert!(!cached.contains("citeCookieName"), "{cached}");
    assert!(body["sources"].get("P5").is_some());
    assert!(body["sources"].get("P8").is_none());
    let p8 = decision(&dir, P8_ID);
    assert_eq!(p8["status"], "open");
    assert!(
        p8["payload"]["error"].as_str().unwrap().contains("Cookies must be enabled"),
        "{p8}"
    );

    extract::run(&Paths::new(&dir)).unwrap();
    assert!(candidates(&dir)
        .iter()
        .any(|c| c["source"] == "P7" && c["sentence"].as_str().unwrap().contains("545.33 cm")));
}

/// R3: a PDF the adapter cannot parse files unavailable-source for that
/// source and the rest are still fetched.
#[test]
fn a_pdf_parse_failure_files_unavailable_source_and_the_rest_are_fetched() {
    let pdf = Page {
        content_type: "application/pdf",
        raw: Some("%PDF-1.4 not parseable".into()),
        ..page("X1", "")
    };
    let (dir, adapter) = palm("pdf", &[pdf, page("P5", &fixture("p5.md"))]);
    let outcome = fetch::run(&Paths::new(&dir), &adapter).unwrap();
    let id = "date-palm/fetch/unavailable-source/X1";
    assert!(
        matches!(&outcome, fetch::Outcome::Ran { decisions } if decisions == &[id]),
        "{outcome:?}"
    );
    assert!(decision(&dir, id)["payload"]["error"]
        .as_str()
        .unwrap()
        .contains("parse"));
    let body = &read_json(&dir.join("fetch.json")).unwrap()["body"];
    assert!(body["sources"].get("P5").is_some());
}

/// R3: a retry that fails again reopens the decision, and stays open.
#[test]
fn a_retry_that_fails_again_reopens_the_decision() {
    let (dir, adapter) = palm("retry", &[cookie_wall()]);
    fetch::run(&Paths::new(&dir), &adapter).unwrap();
    let filed = decision(&dir, P8_ID);
    write_canonical(
        &dir.join("resolutions.json"),
        &json!({"resolutions": [{"id": P8_ID, "inputs_sha256": filed["inputs_sha256"],
                                 "option": "retry", "by": "owner", "at": "2026-09-23"}]}),
    )
    .unwrap();
    fetch::run(&Paths::new(&dir), &adapter).unwrap();
    let reopened = decision(&dir, P8_ID);
    assert_eq!(reopened["status"], "open", "{reopened}");
    assert_ne!(reopened["inputs_sha256"], filed["inputs_sha256"]);
    fetch::run(&Paths::new(&dir), &adapter).unwrap();
    assert_eq!(decision(&dir, P8_ID)["status"], "open");
    assert_eq!(decision(&dir, P8_ID)["inputs_sha256"], reopened["inputs_sha256"]);
}

/// R3: a scrape whose metadata carries no status is not a success.
#[test]
fn a_scrape_with_no_status_is_not_a_success() {
    let dir = scratch("status");
    let program = dir.join("fake-firecrawl");
    let script = "#!/bin/sh\nprintf '%s' '{\"markdown\":\"# Palm\",\"rawHtml\":\"<html>palm</html>\",\"metadata\":{\"contentType\":\"text/html\"}}'\n";
    fs::write(&program, script).unwrap();
    fs::set_permissions(&program, fs::Permissions::from_mode(0o755)).unwrap();
    let cli = FirecrawlCli::with_program(program.display().to_string(), dir.join("cache"));
    let err = cli.scrape(&url("P1")).unwrap_err().to_string();
    assert!(err.contains("no status"), "{err}");
}

/// Answers every screen question the same way and records each sentence.
struct Recording(RefCell<Vec<String>>);

impl Transport for Recording {
    fn send(&self, request: &HttpRequest) -> Result<HttpResponse, String> {
        let body: Value = serde_json::from_slice(request.body.as_deref().unwrap_or(b"{}")).unwrap();
        let sentence = body["state"]["candidate"]["sentence"].as_str().unwrap_or_default();
        self.0.borrow_mut().push(sentence.to_string());
        let answers = json!({
            "kind": {"type": "choice", "choice": "not_about_tree_size", "confidence": 0.9,
                     "probabilities": {"not_about_tree_size": 0.9}},
            "condition": {"type": "choice", "choice": "unstated", "confidence": 0.9,
                          "probabilities": {"unstated": 0.9}},
            "anchor_usable": {"type": "noul", "noul": 0.05},
        });
        Ok(HttpResponse {
            status: 200,
            body: serde_json::to_vec(&json!({"model": "jev-latest", "answers": answers})).unwrap(),
        })
    }
}

/// R4: screen judges exactly extract.json's candidates, not a re-extraction.
#[test]
fn screen_judges_exactly_the_candidates_extract_json_holds() {
    let (dir, adapter) = palm("r4", &[page("P5", &fixture("p5.md"))]);
    fetch::run(&Paths::new(&dir), &adapter).unwrap();
    extract::run(&Paths::new(&dir)).unwrap();
    let path = dir.join("extract.json");
    let mut artifact = read_json(&path).unwrap();
    let first = artifact["body"]["candidates"][0].clone();
    let injected = json!({"source": "P5", "sentence": "An injected palm is 99 m tall.", "context": "An injected palm is 99 m tall."});
    artifact["body"]["candidates"] = json!([first, injected]);
    write_canonical(&path, &artifact).unwrap();

    let transport = Recording(RefCell::new(Vec::new()));
    let judge = Judge {
        transport: &transport,
        key: "test-key",
        ledger_dir: dir.join("ledger").join("entries"),
    };
    screen::run(&Paths::new(&dir), &judge).unwrap();
    let judged = transport.0.borrow().clone();
    let expected = vec![
        first["sentence"].as_str().unwrap().to_string(),
        "An injected palm is 99 m tall.".to_string(),
    ];
    assert_eq!(judged, expected);
    let rows = read_json(&dir.join("screen.json")).unwrap()["body"]["rows"].clone();
    assert_eq!(rows.as_array().unwrap().len(), 2);
}
