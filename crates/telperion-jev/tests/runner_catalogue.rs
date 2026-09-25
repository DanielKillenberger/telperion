//! The catalogue records the runner writes (fn-149, owner decision 2): a
//! folder whose sources, stills, notes, pins and pages the runner wrote
//! passes `scripts/catalogue-check.mjs` as written.
use std::path::{Path, PathBuf};
use std::process::Command;

use serde_json::{json, Value};
use telperion_core::presets::Preset;
use telperion_jev::pipeline::manifest;
use telperion_jev::runner::{folder, pins};

fn repo() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn copy(from: &Path, to: &Path) {
    if from.is_dir() {
        std::fs::create_dir_all(to).unwrap();
        for entry in std::fs::read_dir(from).unwrap() {
            let entry = entry.unwrap();
            copy(&entry.path(), &to.join(entry.file_name()));
        }
    } else {
        std::fs::copy(from, to).unwrap();
    }
}

/// A repository root holding the catalogue scripts and the palm's folder.
fn root(name: &str) -> PathBuf {
    let root = std::env::temp_dir().join(format!(
        "jev-runner-catalogue-{name}-{}-{}",
        std::process::id(),
        telperion_jev::ledger::new_entry_id()
    ));
    std::fs::create_dir_all(root.join("scripts")).unwrap();
    for script in [
        "catalogue-check.mjs",
        "catalogue-pages.mjs",
        "catalogue-sources.mjs",
        "catalogue-article.mjs",
    ] {
        std::fs::copy(
            repo().join("scripts").join(script),
            root.join("scripts").join(script),
        )
        .unwrap();
    }
    std::fs::copy(repo().join(".gitattributes"), root.join(".gitattributes")).unwrap();
    copy(
        &repo().join("catalogue/date-palm"),
        &root.join("catalogue/date-palm"),
    );
    root
}

fn node(root: &Path, args: &[&str]) -> (bool, String) {
    let out = Command::new("node")
        .current_dir(root)
        .args(args)
        .output()
        .unwrap();
    (
        out.status.success(),
        String::from_utf8_lossy(&out.stderr).into_owned(),
    )
}

#[test]
fn a_folder_the_runner_writes_passes_the_catalogue_check() {
    let root = root("fresh");
    let folder = root.join("catalogue/date-palm");
    for file in [
        "sources.json",
        "stills.json",
        "NOTES.md",
        "README.md",
        "pins.json",
        "sources/R1.md",
        "sources/R2.md",
    ] {
        std::fs::remove_file(folder.join(file)).unwrap();
    }
    let admitted = manifest::load(&folder.join("manifest.json")).unwrap();
    let m = &admitted.manifest;
    let fetched: serde_json::Map<String, Value> = m
        .sources
        .iter()
        .map(|s| (s.id.clone(), json!({})))
        .collect();
    let fetch = json!({"body": {"sources": fetched}});

    let references: Value = serde_json::from_str(
        &std::fs::read_to_string(folder.join("packet/references.json")).unwrap(),
    )
    .unwrap();
    folder::sources(&folder, m, &fetch, &references).unwrap();
    folder::reference_copies(&root, &folder, &m.species, &references).unwrap();
    assert!(folder.join("sources/R1.md").exists() && folder.join("sources/R2.md").exists());
    folder::stills(&folder, &m.species, &m.preset, &[]).unwrap();
    folder::notes(&folder, m).unwrap();
    folder::pins_stub(&folder, &m.species).unwrap();
    let stub: Value =
        serde_json::from_str(&std::fs::read_to_string(folder.join("pins.json")).unwrap()).unwrap();
    assert_eq!(stub["empty"], true);
    // The document stage refreshes the article over the new records.
    let (ok, err) = node(
        &root,
        &["scripts/catalogue-article.mjs", "--species", "date-palm"],
    );
    assert!(ok, "{err}");
    folder::pages(&root).unwrap();
    let (ok, err) = node(&root, &["scripts/catalogue-check.mjs"]);
    assert!(ok, "the stub folder fails its check: {err}");

    // Accept fills the pins; the folder still passes.
    let family = Preset::from_id("date-palm").unwrap().parameters();
    pins::write(&folder, "date-palm", &family).unwrap();
    folder::pages(&root).unwrap();
    let (ok, err) = node(&root, &["scripts/catalogue-check.mjs"]);
    assert!(ok, "the accepted folder fails its check: {err}");
    let notes = std::fs::read_to_string(folder.join("NOTES.md")).unwrap();
    assert!(notes.starts_with("# Date palm notes"), "{notes}");
}

#[test]
fn a_record_that_holds_the_same_values_is_left_byte_for_byte() {
    let root = root("same");
    let folder = root.join("catalogue/date-palm");
    let before = std::fs::read(folder.join("sources.json")).unwrap();
    let admitted = manifest::load(&folder.join("manifest.json")).unwrap();
    let m = &admitted.manifest;
    let fetched: serde_json::Map<String, Value> = m
        .sources
        .iter()
        .map(|s| (s.id.clone(), json!({})))
        .collect();
    let references: Value = serde_json::from_str(
        &std::fs::read_to_string(folder.join("packet/references.json")).unwrap(),
    )
    .unwrap();
    folder::sources(
        &folder,
        m,
        &json!({"body": {"sources": fetched}}),
        &references,
    )
    .unwrap();
    assert_eq!(std::fs::read(folder.join("sources.json")).unwrap(), before);
}
