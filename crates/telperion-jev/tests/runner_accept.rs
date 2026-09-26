//! The Accept stage (fn-149 R5): the accepted tree becomes the species'
//! preset in core and its pins, with no hand editing.
use std::path::{Path, PathBuf};

use serde_json::{json, Value};
use telperion_core::presets::{values, Preset};
use telperion_jev::runner::preset::Names;
use telperion_jev::runner::{accept, pins, tune};

fn scratch(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "jev-runner-accept-{name}-{}-{}",
        std::process::id(),
        telperion_jev::ledger::new_entry_id()
    ));
    std::fs::create_dir_all(dir.join("tuning")).unwrap();
    dir
}

fn repo() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

const VALUES: &str = "crates/telperion-core/presets";

/// A repository root holding copies of the preset registry and value files,
/// a catalogue check that prints `stderr` and exits `code`, and a page
/// renderer.
fn root(dir: &Path, stderr: &str, code: i32) -> PathBuf {
    let root = dir.join("repo");
    std::fs::create_dir_all(root.join(VALUES)).unwrap();
    std::fs::create_dir_all(root.join("crates/telperion-core/src")).unwrap();
    std::fs::create_dir_all(root.join("scripts")).unwrap();
    let registry = "crates/telperion-core/src/presets.rs";
    std::fs::copy(repo().join(registry), root.join(registry)).unwrap();
    for entry in std::fs::read_dir(repo().join(VALUES)).unwrap() {
        let name = entry.unwrap().file_name();
        std::fs::copy(
            repo().join(VALUES).join(&name),
            root.join(VALUES).join(&name),
        )
        .unwrap();
    }
    let script = format!(
        "process.stderr.write({});\nprocess.exit({code});\n",
        serde_json::to_string(stderr).unwrap()
    );
    std::fs::write(root.join("scripts/catalogue-check.mjs"), script).unwrap();
    std::fs::write(root.join("scripts/catalogue-pages.mjs"), "").unwrap();
    root
}

fn folder(dir: &Path) -> PathBuf {
    let folder = dir.join("catalogue/date-palm");
    std::fs::create_dir_all(folder.join("packet")).unwrap();
    std::fs::copy(
        repo().join("catalogue/date-palm/packet/profile.json"),
        folder.join("packet/profile.json"),
    )
    .unwrap();
    folder
}

fn result(preset: &str, key: &str, overrides: Value) -> Value {
    json!({
        "meaning": "", "gaps_note": "", "gaps": [], "known_gaps": [],
        "outcome": {
            "run_identity": "r", "preset": preset, "seed": 1, "bootstrap": true,
            "machine_ready": false, "reviewer_passed_unqualified": false,
            "owner_acceptance": "pending", "stopped": "ended", "adoptions_kept": 1,
            "adoptions_rolled_back": 0, "budget": {},
            "current": {"key": key, "round": 2, "label": "bundle@1", "score_telemetry": null,
                "overrides": overrides, "stills": []},
            "finalists": [],
        },
    })
}

fn palm() -> Names {
    Names {
        id: "date-palm".into(),
        common: "Date palm".into(),
        scientific: "Phoenix dactylifera".into(),
    }
}

#[test]
fn the_palm_s_pins_are_the_ones_its_catalogue_records() {
    let computed = pins::compute(&Preset::from_id("date-palm").unwrap().parameters()).unwrap();
    let text = std::fs::read_to_string(repo().join("catalogue/date-palm/pins.json")).unwrap();
    let recorded: Value = serde_json::from_str(&text).unwrap();
    assert_eq!(computed, recorded["pins"]);
    assert_eq!(pins::leaf_band(6554), [1000, 100000]);
}

#[test]
fn an_accepted_palm_sets_the_moved_rows_of_its_value_file_and_its_pins() {
    let dir = scratch("palm");
    let root = root(&dir, "", 0);
    let folder = folder(&dir);
    let path = tune::result(&dir);
    let moved = json!({"canopy": {"leafBases": 200, "skirtFronds": 16}, "shellDepth": 0.8});
    std::fs::write(&path, result("date-palm", "k1", moved).to_string()).unwrap();
    let word = accept::run(&root, &palm(), &folder, &path, &dir).unwrap();
    assert!(
        word.contains("2 rows of crates/telperion-core/presets/date-palm.values set"),
        "{word}"
    );
    let file = std::fs::read_to_string(root.join(VALUES).join("date-palm.values")).unwrap();
    assert!(file.contains("\n/canopy/leafBases = 200\n"));
    assert!(file.contains("\n/shellDepth = 0.8\n"));
    assert!(
        !file.contains("/canopy/leafBases = 256"),
        "the old line stayed"
    );
    // Its comments and every row it did not move stay as they were.
    assert!(file.contains("# The trunk organs (fn-110)"));
    assert!(file.contains("\n/canopy/skirtFronds = 16\n"));
    let accepted = accept::family(&path).unwrap();
    assert_eq!(
        format!("{:?}", values::read(&file).unwrap()),
        format!("{accepted:?}")
    );
    let pins: Value =
        serde_json::from_str(&std::fs::read_to_string(folder.join("pins.json")).unwrap()).unwrap();
    assert_eq!(pins["species"], "date-palm");
    assert_eq!(pins["seed"], 7);
    assert_eq!(pins["growth_reference"]["height_m"], 22.86);
    assert!(accept::accepted(&path, &dir).unwrap());
}

#[test]
fn a_new_species_is_written_and_registered_from_its_template() {
    let dir = scratch("new");
    let root = root(&dir, "", 0);
    let folder = folder(&dir);
    let path = tune::result(&dir);
    std::fs::write(
        &path,
        result("date-palm", "k9", json!({"canopy": {"leafBases": 64}})).to_string(),
    )
    .unwrap();
    let names = Names {
        id: "canary-palm".into(),
        common: "Canary Island date palm".into(),
        scientific: "Phoenix canariensis".into(),
    };
    let word = accept::run(&root, &names, &folder, &path, &dir).unwrap();
    assert!(word.contains("canary-palm.values written"), "{word}");
    let read = |p: &str| std::fs::read_to_string(root.join(p)).unwrap();
    let file = read("crates/telperion-core/presets/canary-palm.values");
    assert!(file.starts_with("# Canary Island date palm, Phoenix canariensis.\n"));
    assert!(file.contains("\n/canopy/leafBases = 64\n"));
    assert!(file.contains("\n/skeleton/envelope/height = 22.86\n"));
    let accepted = accept::family(&path).unwrap();
    assert_eq!(
        format!("{:?}", values::read(&file).unwrap()),
        format!("{accepted:?}")
    );
    let presets = read("crates/telperion-core/src/presets.rs");
    for line in [
        "    CanaryPalm,\n",
        "    (8, \"canary-palm\", \"Canary Island date palm\", \"Phoenix canariensis\"),\n",
        "            Self::CanaryPalm => Some(\"canary-palm\"),\n",
        "            \"canary-palm\" => Some(Self::CanaryPalm),\n",
    ] {
        assert!(presets.contains(line), "presets.rs lacks {line:?}");
    }
}

#[test]
fn an_acceptance_is_refused_while_the_catalogue_entry_fails_or_the_check_does_not_run() {
    let dir = scratch("refused");
    let folder = folder(&dir);
    let path = tune::result(&dir);
    std::fs::write(
        &path,
        result("date-palm", "k1", json!({"canopy": {"leafBases": 200}})).to_string(),
    )
    .unwrap();
    let failing =
        "catalogue/date-palm/sources.json: schema is not \"sources\"\n\ncatalogue: 1 failure\n";
    let refused = root(&dir, failing, 1);
    let err = accept::run(&refused, &palm(), &folder, &path, &dir).unwrap_err();
    assert!(err.contains("schema is not"), "{err}");
    let file = refused.join(VALUES).join("date-palm.values");
    assert!(
        std::fs::read_to_string(&file)
            .unwrap()
            .contains("\n/canopy/leafBases = 256\n"),
        "a refused acceptance wrote the preset"
    );
    let crashed = root(&dir, "TypeError: undefined\n", 1);
    let err = accept::run(&crashed, &palm(), &folder, &path, &dir).unwrap_err();
    assert!(err.starts_with("the catalogue check did not run"), "{err}");
    assert!(
        !accept::file(&dir).exists(),
        "a refused acceptance wrote a record"
    );
    // Another species' failure does not hold this one back.
    let other = root(
        &dir,
        "catalogue/oak/ARTICLE.md: stale\n\ncatalogue: 1 failure\n",
        1,
    );
    accept::run(&other, &palm(), &folder, &path, &dir).unwrap();
    // A later revision's tree waits for a look of its own.
    std::fs::write(&path, result("date-palm", "k2", json!({})).to_string()).unwrap();
    assert!(!accept::accepted(&path, &dir).unwrap());
}

#[test]
fn an_accepted_oak_adds_a_row_its_file_left_at_the_default() {
    let dir = scratch("oak");
    let root = root(&dir, "", 0);
    let folder = folder(&dir);
    let path = tune::result(&dir);
    let moved = json!({"skeleton": {"envelope": {"height": 25.0}}});
    std::fs::write(&path, result("oregon-white-oak", "k1", moved).to_string()).unwrap();
    let names = Names {
        id: "oregon-white-oak".into(),
        common: "Oregon white oak".into(),
        scientific: "Quercus garryana".into(),
    };
    let word = accept::run(&root, &names, &folder, &path, &dir).unwrap();
    assert!(
        word.contains("1 rows of crates/telperion-core/presets/oregon-white-oak.values set"),
        "{word}"
    );
    let file = std::fs::read_to_string(root.join(VALUES).join("oregon-white-oak.values")).unwrap();
    // The oak's height is the default's, so the acceptance adds the row.
    let added = "(species runner, fn-149).\n/skeleton/envelope/height = 25.0\n";
    assert!(file.ends_with(added), "{file}");
}

/// fn-149: the run's own catalogue (`species --catalogue`), not the
/// repository's. The check runs over the folder's catalogue and a failure it
/// names there refuses the acceptance.
#[test]
fn an_acceptance_checks_the_catalogue_the_run_keeps() {
    let dir = scratch("elsewhere");
    let elsewhere = dir.join("elsewhere/catalogue/date-palm");
    std::fs::create_dir_all(elsewhere.join("packet")).unwrap();
    std::fs::copy(
        folder(&dir).join("packet/profile.json"),
        elsewhere.join("packet/profile.json"),
    )
    .unwrap();
    let path = tune::result(&dir);
    std::fs::write(&path, result("date-palm", "k1", json!({})).to_string()).unwrap();
    let root = root(&dir, "", 0);
    // The check fails the folder only when it is pointed at that catalogue.
    let script = "const at = process.env.TELPERION_CATALOGUE ?? '';\n\
        if (at.endsWith('elsewhere/catalogue')) {\n\
          process.stderr.write('elsewhere/catalogue/date-palm/sources.json: no source R1\\n\\ncatalogue: 1 failure\\n');\n\
          process.exit(1);\n\
        }\n";
    std::fs::write(root.join("scripts/catalogue-check.mjs"), script).unwrap();
    let err = accept::run(&root, &palm(), &elsewhere, &path, &dir).unwrap_err();
    assert!(err.contains("no source R1"), "{err}");
}
