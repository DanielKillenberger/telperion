//! fn-136: a species run finishes. A missing identity capability blocks at
//! the gate; a missing improvement is captured by its backlog specs and
//! carried as a known gap, and a reference trait the generator cannot draw
//! yet is left out of core coverage. Nothing here calls a model.
use std::path::PathBuf;

use serde_json::{json, Value};
use telperion_jev::pipeline::stages::capability_class::{self, Class, Classified};
use telperion_jev::tuning::reference_first::Inventory;
use telperion_jev::tuning::state::{CellStatus, TraitStatus};
use telperion_jev::tuning::unexpressed::{core_coverage, Unexpressed};

const PALM: &str = include_str!("fixtures/fn136-palm-capability.json");
const RECORDED: &str = include_str!("fixtures/fn136-palm-recorded-coverage.json");
const INVENTORY: &str = include_str!("fixtures/fn80-palm-inventory.json");

fn scratch(tag: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "jev-finish-{tag}-{}-{}",
        std::process::id(),
        telperion_jev::ledger::new_entry_id()
    ));
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

/// Writes an assessment holding `classes` and reads its classes back.
fn read(tag: &str, classes: Value) -> Result<Vec<Classified>, String> {
    let path = scratch(tag).join("capability.json");
    std::fs::write(
        &path,
        json!({"species": "x", "classes": classes}).to_string(),
    )
    .unwrap();
    capability_class::read(&path)
}

fn class(capability: &str, class: &str, captured_by: &[&str]) -> Value {
    json!({"capability": capability, "class": class, "reason": "why",
           "captured_by": captured_by, "decided_by": "host", "decided_on": "2026-09-24"})
}

fn names(list: &[&str]) -> Vec<String> {
    list.iter().map(|n| (*n).to_string()).collect()
}

#[test]
fn the_palms_assessment_records_its_date_cluster_as_an_owner_decided_improvement() {
    let path = scratch("palm").join("capability.json");
    std::fs::write(&path, PALM).unwrap();
    let classes = capability_class::read(&path).unwrap();
    assert_eq!(classes.len(), 1);
    let fruit = &classes[0];
    assert_eq!(fruit.capability, "infructescence");
    assert_eq!(fruit.class, Class::Improvement);
    assert_eq!(
        fruit.captured_by,
        names(&[
            "fn-33-flowers-cones-and-compound-leaves-as",
            "fn-111-the-palms-infructescence-a-hanging-date"
        ])
    );
    assert_eq!(
        (fruit.decided_by.as_str(), fruit.decided_on.as_str()),
        ("owner", "2026-09-24")
    );
}

#[test]
fn a_missing_improvement_is_a_known_gap_and_a_missing_identity_still_blocks() {
    let classes = read(
        "split",
        json!([
            class("infructescence", "improvement", &["fn-33", "fn-111"]),
            class("acanthophyll", "identity", &[]),
            class("woody-axes", "improvement", &["fn-9"]),
        ]),
    )
    .unwrap();
    let missing = names(&["infructescence", "acanthophyll", "pinnate-frond"]);
    let (blocking, known) = capability_class::split(&missing, &classes);
    // An identity capability blocks, and so does one nobody classified.
    assert_eq!(blocking, names(&["acanthophyll", "pinnate-frond"]));
    // A class on a capability the generator expresses says nothing.
    assert_eq!(known.len(), 1);
    assert_eq!(known[0].capability, "infructescence");
    assert_eq!(known[0].captured_by, names(&["fn-33", "fn-111"]));
    assert_eq!(known[0].reason, "why");
}

#[test]
fn an_assessment_whose_classes_cannot_be_trusted_is_refused() {
    let cases = [
        (
            "no-spec",
            json!([class("infructescence", "improvement", &[])]),
            "names no backlog spec",
        ),
        (
            "blank-spec",
            json!([class("infructescence", "improvement", &[" "])]),
            "names no backlog spec",
        ),
        (
            "twice",
            json!([
                class("infructescence", "improvement", &["fn-33"]),
                class("infructescence", "identity", &[])
            ]),
            "classified twice",
        ),
        (
            "no-reason",
            json!([{"capability": "infructescence", "class": "improvement", "reason": " ",
                    "captured_by": ["fn-33"], "decided_by": "host", "decided_on": "2026-09-24"}]),
            "no reason",
        ),
        (
            "unknown-class",
            json!([class("infructescence", "nice-to-have", &["fn-33"])]),
            "unknown variant",
        ),
    ];
    for (tag, classes, expected) in cases {
        let err = read(tag, classes).unwrap_err();
        assert!(err.contains(expected), "{tag}: {err}");
    }
}

#[test]
fn an_assessment_without_classes_classifies_nothing() {
    let dir = scratch("absent");
    assert_eq!(
        capability_class::read(&dir.join("capability.json")).unwrap(),
        vec![]
    );
    let path = dir.join("unclassified.json");
    std::fs::write(&path, json!({"species": "x", "traits": []}).to_string()).unwrap();
    assert_eq!(capability_class::read(&path).unwrap(), vec![]);
    // A list of rounds is read at its latest round.
    let rounds = dir.join("rounds.json");
    let latest = json!({"classes": [class("infructescence", "improvement", &["fn-111"])]});
    std::fs::write(&rounds, json!([{"round": 1}, latest]).to_string()).unwrap();
    let classes = capability_class::read(&rounds).unwrap();
    assert_eq!(classes[0].captured_by, names(&["fn-111"]));
}

/// The palm's recorded assessment of revision 3: the date cluster, which the
/// generator cannot draw until fn-111 lands, is one of the four core traits
/// that kept coverage from passing. Listed unexpressed, it is left out, and
/// what still blocks is what the generator can draw.
#[test]
fn the_palms_date_cluster_leaves_core_coverage_on_its_recorded_assessment() {
    let inventory: Inventory = serde_json::from_str(INVENTORY).unwrap();
    let recorded: Value = serde_json::from_str(RECORDED).unwrap();
    let statuses: Vec<TraitStatus> = serde_json::from_value(recorded["coverage"].clone()).unwrap();
    let unexpressed: Vec<Unexpressed> =
        serde_json::from_value(recorded["unexpressed"].clone()).unwrap();
    let (status, blocking) = core_coverage(&inventory, &statuses, &[]);
    assert_eq!(status, CellStatus::Fail);
    assert!(blocking.contains(&"fruit-clusters-pendent".to_string()));
    let (status, blocking) = core_coverage(&inventory, &statuses, &unexpressed);
    assert_eq!(status, CellStatus::Fail);
    assert_eq!(
        blocking,
        names(&[
            "leaflet-arrangement-stiff-narrow",
            "trunk-leaf-base-diamond-pattern",
            "trunk-fibrous-matting"
        ])
    );
    // With every drawable core trait passing, the date cluster alone no
    // longer keeps coverage from passing.
    let drawn: Vec<TraitStatus> = statuses
        .iter()
        .map(|s| TraitStatus {
            trait_id: s.trait_id.clone(),
            status: if s.trait_id == "fruit-clusters-pendent" {
                CellStatus::Fail
            } else {
                CellStatus::Pass
            },
        })
        .collect();
    assert_eq!(
        core_coverage(&inventory, &drawn, &[]),
        (CellStatus::Fail, names(&["fruit-clusters-pendent"]))
    );
    assert_eq!(
        core_coverage(&inventory, &drawn, &unexpressed),
        (CellStatus::Pass, vec![])
    );
}
