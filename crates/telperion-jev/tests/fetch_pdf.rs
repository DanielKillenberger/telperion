//! fn-157: web.archive.org served the EUFORGEN beech guideline as
//! `text/html`, and fetch read the PDF's bytes as text, so its 30-35 m never
//! reached the aggregate. A body that starts `%PDF-` is a PDF whatever its
//! content type. The fixture keeps the recorded body's first 64 bytes and
//! the guideline's height passage as the parse returns it.
use std::fs;
use std::path::Path;

use serde_json::json;
use telperion_jev::pipeline::adapter::FixtureAdapter;
use telperion_jev::pipeline::canon::{read_json, write_canonical};
use telperion_jev::pipeline::stage::Paths;
use telperion_jev::pipeline::stages::fetch;

const ARCHIVED: &str = "https://web.archive.org/web/20190819110558/http://www.euforgen.org/fileadmin/templates/euforgen.org/upload/Publications/Technical_guidelines/1322_European_beech__Fagus_sylvatica_.pdf";

#[test]
fn a_pdf_served_as_html_is_parsed_as_a_pdf() {
    let dir = std::env::temp_dir().join(format!(
        "jev-fetch-pdf-{}-{}",
        std::process::id(),
        telperion_jev::ledger::new_entry_id()
    ));
    fs::create_dir_all(&dir).unwrap();
    let manifest = json!({
        "schema": "manifest", "schema_version": 1, "species": "european-beech",
        "taxon": {"scientific_name": "Fagus sylvatica", "common_name": "European beech", "rank": "species"},
        "context": "mature open-grown", "growth_form": "broadleaf",
        "preset": "european-beech", "profile_id": "european-beech", "seed": 7,
        "sources": [{"id": "P1", "url": ARCHIVED, "title": "EUFORGEN", "rights": "cited"}],
        "fields": [], "versions": {"question_sets": {}, "tools": {}}, "model": "jev-latest"
    });
    write_canonical(&dir.join("manifest.json"), &manifest).unwrap();
    let fixtures = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/pdf");
    fetch::run(&Paths::new(&dir), &FixtureAdapter::new(fixtures)).unwrap();
    let body = &read_json(&dir.join("fetch.json")).unwrap()["body"];
    assert!(body["sources"].get("P1").is_some(), "{body}");
    let cached = fs::read_to_string(dir.join("cache/P1.md")).unwrap();
    assert!(
        cached.contains("normally grows to 30–35 m tall"),
        "{cached}"
    );
    assert!(dir.join("cache/P1.pdf").exists());
}
