//! fn-149, owner 2026-09-25: a rate-limited fetch waits the delay the error
//! names and tries again; the source ends fetched, never dropped. Fixture
//! adapter, no network, no key, no real wait.
use std::cell::Cell;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::time::Duration;

use serde_json::json;
use telperion_jev::pipeline::adapter::{
    AdapterError, FetchAdapter, FixtureAdapter, Retrying, Scrape, SearchHit, Spent,
};
use telperion_jev::pipeline::canon::{read_json, write_canonical};
use telperion_jev::pipeline::stage::Paths;
use telperion_jev::pipeline::stages::fetch;

const PAGE: &str = "https://example.test/beech";
const LIMIT: &str = "Rate limit exceeded. Consumed (req/min): 11, Remaining (req/min): 0. Upgrade your plan or retry after 9s";

/// The fixture, with the first `failures` scrapes refused by the rate limit.
struct Limited {
    fixture: FixtureAdapter,
    failures: Cell<u32>,
}

impl FetchAdapter for Limited {
    fn search(&self, q: &str, n: usize) -> Result<Vec<SearchHit>, AdapterError> {
        self.fixture.search(q, n)
    }
    fn research(&self, q: &str, n: usize) -> Result<Vec<SearchHit>, AdapterError> {
        self.fixture.research(q, n)
    }
    fn scrape(&self, url: &str) -> Result<Scrape, AdapterError> {
        if self.failures.get() > 0 {
            self.failures.set(self.failures.get() - 1);
            let error = LIMIT.to_string();
            return Err(AdapterError::Failed {
                url: url.into(),
                error,
            });
        }
        self.fixture.scrape(url)
    }
    fn parse_pdf(&self, path: &Path) -> Result<String, AdapterError> {
        self.fixture.parse_pdf(path)
    }
    fn spent(&self) -> Spent {
        self.fixture.spent()
    }
}

static WAITED: Mutex<Vec<Duration>> = Mutex::new(Vec::new());

fn record(wait: Duration) {
    WAITED.lock().unwrap().push(wait);
}

/// A pipeline directory whose manifest admits one source, and its fixture.
fn scratch(tag: &str) -> (PathBuf, PathBuf) {
    let root = std::env::temp_dir().join(format!(
        "jev-fetch-retry-{tag}-{}-{}",
        std::process::id(),
        telperion_jev::ledger::new_entry_id()
    ));
    let (dir, fixtures) = (root.join("species"), root.join("fixtures"));
    fs::create_dir_all(&dir).unwrap();
    fs::create_dir_all(&fixtures).unwrap();
    let manifest = json!({
        "schema": "manifest", "schema_version": 1, "species": "european-beech",
        "taxon": {"scientific_name": "Fagus sylvatica", "common_name": "European beech", "rank": "species"},
        "context": "mature open-grown", "growth_form": "broadleaf",
        "preset": "european-beech", "profile_id": "european-beech", "seed": 7,
        "sources": [{"id": "P2", "url": PAGE, "title": "Beech", "rights": "public domain"}],
        "fields": [], "versions": {"question_sets": {}, "tools": {}}, "model": "jev-latest"
    });
    write_canonical(&dir.join("manifest.json"), &manifest).unwrap();
    fs::write(fixtures.join("p.html"), "<p>Beech reaches 30 m.</p>").unwrap();
    fs::write(fixtures.join("p.md"), "Beech reaches 30 m.").unwrap();
    let scrape = json!({PAGE: {"final_url": PAGE, "content_type": "text/html", "raw": "p.html", "markdown": "p.md"}});
    write_canonical(&fixtures.join("index.json"), &json!({"scrape": scrape})).unwrap();
    (dir, fixtures)
}

fn limited(fixtures: &Path, failures: u32) -> Box<dyn FetchAdapter> {
    Box::new(Limited {
        fixture: FixtureAdapter::new(fixtures),
        failures: Cell::new(failures),
    })
}

#[test]
fn a_rate_limited_source_waits_the_named_delay_and_is_fetched() {
    let (dir, fixtures) = scratch("once");
    let adapter = Retrying::with_sleep(limited(&fixtures, 1), record);
    fetch::run(&Paths::new(&dir), &adapter).unwrap();
    let body = read_json(&dir.join("fetch.json")).unwrap()["body"].clone();
    assert!(body["sources"].get("P2").is_some(), "{body}");
    assert!(
        !dir.join("decisions.json").exists(),
        "a transient failure filed a decision"
    );
    assert!(WAITED.lock().unwrap().contains(&Duration::from_secs(9)));
}

#[test]
fn a_limit_that_outlasts_every_attempt_files_unavailable_source() {
    let (dir, fixtures) = scratch("always");
    let adapter = Retrying::with_sleep(limited(&fixtures, u32::MAX), record);
    fetch::run(&Paths::new(&dir), &adapter).unwrap();
    let list = read_json(&dir.join("decisions.json")).unwrap();
    assert_eq!(
        list["decisions"][0]["id"],
        "european-beech/fetch/unavailable-source/P2"
    );
}
