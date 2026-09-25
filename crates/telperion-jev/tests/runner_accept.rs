//! The Accept stage (fn-149 R5): the accepted tree becomes the species'
//! preset in core and its pins, with no hand editing.
use std::path::{Path, PathBuf};

use serde_json::{json, Value};
use telperion_core::presets::Preset;
use telperion_jev::runner::preset::{self, Names};
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

/// A repository root holding copies of the two preset sources, a catalogue
/// check that prints `stderr` and exits `code`, and a page renderer.
fn root(dir: &Path, stderr: &str, code: i32) -> PathBuf {
    let root = dir.join("repo");
    let presets = root.join("crates/telperion-core/src/presets");
    std::fs::create_dir_all(&presets).unwrap();
    std::fs::create_dir_all(root.join("scripts")).unwrap();
    for file in [
        "crates/telperion-core/src/presets.rs",
        "crates/telperion-core/src/presets/species.rs",
        "crates/telperion-core/src/presets/originals.rs",
    ] {
        std::fs::copy(repo().join(file), root.join(file)).unwrap();
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
fn an_accepted_palm_sets_the_moved_rows_of_its_function_and_its_pins() {
    let dir = scratch("palm");
    let root = root(&dir, "", 0);
    let folder = folder(&dir);
    let path = tune::result(&dir);
    let moved = json!({"canopy": {"leafBases": 200, "skirtFronds": 16}, "shellDepth": 0.8});
    std::fs::write(&path, result("date-palm", "k1", moved).to_string()).unwrap();
    let word = accept::run(&root, &palm(), &folder, &path, &dir).unwrap();
    assert!(word.contains("2 rows of fn date_palm set"), "{word}");
    let species =
        std::fs::read_to_string(root.join("crates/telperion-core/src/presets/species.rs")).unwrap();
    assert!(species.contains("    p.canopy.leaf_bases = 200;\n"));
    assert!(species.contains("    p.shell_depth = 0.8;\n"));
    assert!(
        !species.contains("p.canopy.leaf_bases = 256;"),
        "the old line stayed"
    );
    // Its comments and every row it did not move stay as they were.
    assert!(species.contains("// The trunk organs (fn-110)"));
    assert!(species.contains("    p.canopy.skirt_fronds = 16;\n"));
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
    assert!(word.contains("fn canary_palm written"), "{word}");
    let read = |p: &str| std::fs::read_to_string(root.join(p)).unwrap();
    let species = read("crates/telperion-core/src/presets/species.rs");
    assert!(species.contains("pub(super) fn canary_palm(p: &mut Family) {"));
    assert!(species.contains("    p.canopy.leaf_bases = 64;\n"));
    assert!(species.contains("    p.skeleton.envelope.height = 22.86;\n"));
    let presets = read("crates/telperion-core/src/presets.rs");
    for line in [
        "    CanaryPalm,\n",
        "    (8, \"canary-palm\", \"Canary Island date palm\", \"Phoenix canariensis\"),\n",
        "            Self::CanaryPalm => Some(\"canary-palm\"),\n",
        "            \"canary-palm\" => Some(Self::CanaryPalm),\n",
        "        if self == Self::CanaryPalm {\n            species::canary_palm(&mut p);",
    ] {
        assert!(presets.contains(line), "presets.rs lacks {line:?}");
    }
}

#[test]
fn a_row_off_the_default_family_is_its_rust_literal() {
    assert_eq!(
        preset::field("/skeleton/habit/lateralPitch"),
        "skeleton.habit.lateral_pitch"
    );
    assert_eq!(preset::literal(&json!(2.0), false).unwrap(), "2.0");
    assert_eq!(preset::literal(&json!(256), false).unwrap(), "256");
    assert_eq!(preset::literal(&json!(0.5), true).unwrap(), "Some(0.5)");
    assert_eq!(preset::literal(&Value::Null, true).unwrap(), "None");
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
    let species = refused.join("crates/telperion-core/src/presets/species.rs");
    assert!(
        std::fs::read_to_string(&species)
            .unwrap()
            .contains("p.canopy.leaf_bases = 256;"),
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
fn an_accepted_oak_sets_its_function_where_it_lives() {
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
    assert!(word.contains("1 rows of fn oregon_white_oak set"), "{word}");
    let originals =
        std::fs::read_to_string(root.join("crates/telperion-core/src/presets/originals.rs"))
            .unwrap();
    // Its height sits in a struct literal, so the acceptance adds the row.
    assert!(
        originals.contains("    p.skeleton.envelope.height = 25.0;\n"),
        "{originals}"
    );
}
