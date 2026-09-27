//! fn-179 R3: a run seeded with a species' curated references uses them as
//! the catalogue stores them. fn-62's beech run needed two hand patches: a
//! `view` on each record with a shot, and each photograph copied into the
//! run's cache by its hash. Now the view is read off the record's scale
//! (`runner::cells::view`), the photograph is found under the tuning
//! config's `refs` by its address and checked against its hash, and the
//! photograph search does not run. No network, no key, no image committed:
//! the photographs here are a few stand-in bytes.
use std::fs;
use std::path::{Path, PathBuf};

use serde_json::{json, Value};
use telperion_jev::caller::{HttpRequest, HttpResponse, Transport};
use telperion_jev::pipeline::canon::{read_json, write_canonical};
use telperion_jev::pipeline::judge::Judge;
use telperion_jev::pipeline::photos::{self, Screen, Verdict, Web};
use telperion_jev::pipeline::stage::Paths;
use telperion_jev::runner::inventory;
use telperion_jev::tuning::evaluation::Image;

/// Any use of the search is a failure: the curated references are enough.
struct Unused;

impl Web for Unused {
    fn get(&self, url: &str) -> Result<(Vec<u8>, String), String> {
        panic!("the photograph search fetched {url}")
    }
}

impl Transport for Unused {
    fn send(&self, _: &HttpRequest) -> Result<HttpResponse, String> {
        panic!("the photograph search asked Jev")
    }
}

impl Screen for Unused {
    fn screen(&self, _: &str, _: &[Image]) -> Result<(Vec<Verdict>, Value), String> {
        panic!("the photograph search looked")
    }
}

fn scratch() -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "jev-curated-{}-{}",
        std::process::id(),
        telperion_jev::ledger::new_entry_id()
    ));
    fs::create_dir_all(dir.join("packet")).unwrap();
    fs::create_dir_all(dir.join("refs")).unwrap();
    dir
}

/// The beech's curated references, as the shipped catalogue stores them.
fn beech() -> Value {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../catalogue/european-beech/packet/references.json");
    read_json(&path).unwrap()
}

#[test]
fn seeded_with_the_beech_s_curated_references_the_search_does_not_run() {
    let dir = scratch();
    write_canonical(&dir.join("packet/references.json"), &beech()).unwrap();
    let judge = Judge {
        transport: &Unused,
        key: "test-key",
        ledger_dir: dir.join("ledger"),
    };
    let said = photos::find(&Paths::new(&dir), &Unused, &judge, &Unused).unwrap();
    assert_eq!(said, "5 reference photographs recorded");
}

/// A curated record, as stored: no view and no copy in the run's cache, its
/// photograph under `refs` by the file name its address ends in.
fn curated(id: &str, name: &str, bytes: &[u8]) -> Value {
    json!({"id": id, "kept": false, "scale": ["whole"],
        "asset_sha256": telperion_jev::sha256_hex(bytes),
        "url": format!("https://landscapeplants.example/plantimage/{name}")})
}

#[test]
fn a_curated_photograph_is_copied_into_the_run_by_its_hash_once_verified() {
    let dir = scratch();
    let paths = Paths::new(&dir);
    let whole = b"\xFF\xD8\xFF\xE0whole".to_vec();
    fs::write(dir.join("refs/fasy951.jpg"), &whole).unwrap();
    let doc = json!({"references": [
        curated("B-WHOLE", "fasy951.jpg", &whole),
        curated("B-ABSENT", "fasy0.jpg", b"never stored"),
    ]});
    write_canonical(&paths.packet("references"), &doc).unwrap();
    let out = dir.join("runner");
    inventory::record(&paths, &out, Some(&dir.join("refs"))).unwrap();

    let sha = telperion_jev::sha256_hex(&whole);
    let copy = photos::dir(&paths).join(format!("{sha}.jpg"));
    assert_eq!(fs::read(&copy).unwrap(), whole);
    let found = read_json(&inventory::found(&out)).unwrap();
    assert_eq!(
        found,
        json!([{"path": copy, "sha256": sha, "view": "B-WHOLE", "seed": 0}]),
        "a photograph in neither place is left out"
    );
}

#[test]
fn a_curated_photograph_whose_bytes_are_not_its_hash_is_refused() {
    let dir = scratch();
    let paths = Paths::new(&dir);
    fs::write(dir.join("refs/fasy951.jpg"), b"other bytes").unwrap();
    let doc = json!({"references": [curated("B-WHOLE", "fasy951.jpg", b"the record's")]});
    write_canonical(&paths.packet("references"), &doc).unwrap();
    let err = inventory::record(&paths, &dir.join("runner"), Some(&dir.join("refs"))).unwrap_err();
    assert!(
        err.contains("fasy951.jpg") && err.contains("asset_sha256"),
        "{err}"
    );
    assert!(!photos::dir(&paths).exists(), "an unverified copy was made");
}
