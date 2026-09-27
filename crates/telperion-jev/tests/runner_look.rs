//! `species <id> --look` (fn-166): the kept tree, written where the harness
//! opens it, as the family `headless --family` builds from the same overlay;
//! a run with no kept tree, or an overlay row the family lacks, is refused
//! by name.
use std::path::{Path, PathBuf};

use serde_json::{json, Value};
use telperion_core::{params, presets::Preset};
use telperion_jev::runner::look;

fn fixture() -> Value {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/look/result.json");
    serde_json::from_slice(&std::fs::read(path).unwrap()).unwrap()
}

/// A run directory holding `result` as the stage's artifact and as
/// revision 1's; returns the artifact's path.
fn run(name: &str, result: &Value) -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "jev-runner-look-{name}-{}-{}",
        std::process::id(),
        telperion_jev::ledger::new_entry_id()
    ));
    let tuning = dir.join("run/runner/tuning");
    std::fs::create_dir_all(tuning.join("1")).unwrap();
    let bytes = serde_json::to_vec(result).unwrap();
    std::fs::write(tuning.join("1/result.json"), &bytes).unwrap();
    std::fs::write(tuning.join("result.json"), &bytes).unwrap();
    tuning.join("result.json")
}

#[test]
fn the_look_is_the_family_headless_builds_from_the_overlay() {
    let tuned = fixture();
    let result = run("kept", &tuned);
    let looks = result.parent().unwrap().join("looks");
    let written = look::write("european-beech", &result, &looks).unwrap();
    assert_eq!(written.path, looks.join("european-beech.json"));
    assert_eq!(
        written.url,
        "http://localhost:5173/?look=european-beech&seed=1"
    );
    let shown: Value = serde_json::from_slice(&std::fs::read(&written.path).unwrap()).unwrap();
    // `headless --family`: the preset's parameters with the overlay laid over.
    let preset = Preset::from_id("european-beech").unwrap().parameters();
    let overrides = &tuned["outcome"]["current"]["overrides"];
    let headless = params::overlay(&preset, overrides).unwrap();
    assert_eq!(shown["family"], params::metadata(&headless));
    assert_eq!(shown["preset_family"], params::metadata(&preset));
    assert_eq!(&shown["overrides"], overrides);
    let run_dir = result.parent().unwrap().parent().unwrap().parent().unwrap();
    assert_eq!(
        shown["source"],
        json!({"run": run_dir.display().to_string(), "revision": 1, "round": 2,
            "label": "bundle@0.5", "kept": 1, "key": tuned["outcome"]["current"]["key"]})
    );
}

#[test]
fn a_look_without_a_kept_tree_or_with_a_foreign_row_is_refused_by_name() {
    let with = |overrides: Value| {
        let mut tuned = fixture();
        tuned["outcome"]["current"]["overrides"] = overrides;
        tuned
    };
    let mut none = fixture();
    none["outcome"]["current"] = Value::Null;
    let cases = [
        ("none", none, "has no outcome.current"),
        (
            "row",
            with(json!({"skeleton": {"habit": {"nope": 1}}})),
            "/skeleton/habit/nope",
        ),
        (
            "retired",
            with(json!({"skeleton": {"habit": {"stemDivergence": 1.25}}})),
            "/skeleton/habit/stemDivergence is retired: use /skeleton/habit/forkDivergence",
        ),
        (
            "group",
            with(json!({"skeleton": 3})),
            "the overlay path /skeleton ",
        ),
        (
            "value",
            with(json!({"shellDepth": {"x": 1}})),
            "the overlay path /shellDepth ",
        ),
    ];
    for (name, tuned, says) in cases {
        let result = run(name, &tuned);
        let looks = result.parent().unwrap().join("looks");
        let err = look::write("european-beech", &result, &looks)
            .err()
            .expect(name);
        assert!(err.contains(&result.display().to_string()), "{name}: {err}");
        assert!(err.contains(says), "{name}: {err}");
        assert!(
            !looks.exists(),
            "{name}: a refused look wrote {}",
            looks.display()
        );
    }
    let missing = run("missing", &fixture());
    std::fs::remove_file(&missing).unwrap();
    let err = look::write("european-beech", &missing, Path::new("unused"))
        .err()
        .unwrap();
    assert!(err.contains(&format!(
        "no kept tree: {} does not exist",
        missing.display()
    )));
}
