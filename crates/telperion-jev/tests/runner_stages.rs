//! The runner's own stages (fn-149): the live dials a revision offers, the
//! classes of the gap list, and what an acceptance names.
use std::path::{Path, PathBuf};

use serde_json::{json, Value};
use telperion_jev::runner::gaps::{self, Kind};
use telperion_jev::runner::{accept, tune};
use telperion_jev::tuning::result::EndResult;

fn scratch(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "jev-runner-stages-{name}-{}-{}",
        std::process::id(),
        telperion_jev::ledger::new_entry_id()
    ));
    std::fs::create_dir_all(dir.join("tuning")).unwrap();
    dir
}

fn result(current: Value, gaps: Value, known: Value) -> Value {
    json!({
        "meaning": "", "gaps_note": "", "gaps": gaps, "known_gaps": known,
        "outcome": {
            "run_identity": "r", "preset": "date-palm", "seed": 1, "bootstrap": true,
            "machine_ready": false, "reviewer_passed_unqualified": false,
            "owner_acceptance": "pending", "stopped": "ended", "adoptions_kept": 1,
            "adoptions_rolled_back": 0, "budget": {}, "current": current, "finalists": [],
        },
    })
}

fn tree(key: &str) -> Value {
    json!({"key": key, "round": 2, "label": "bundle@1", "score_telemetry": null,
           "overrides": {"canopy": {"leafBases": 256, "size": 2.35}}, "stills": []})
}

fn gap(id: &str, status: &str, moves: Value) -> Value {
    let attempts = match moves.as_array().is_some_and(|m| !m.is_empty()) {
        true => json!([{"dial": "bundle", "round": 1, "action_ledger": null,
            "score_before_round": null, "score_after": null, "feasible": true, "reason": null,
            "visual_outcome": null, "moves": moves}]),
        false => json!([]),
    };
    json!({"id": id, "rank": 1, "priority": id, "status": status, "latest_route": null,
           "existing_spec": null, "attempts": attempts, "reviewer_words": ["still thin"],
           "stills": [], "check": "pending"})
}

#[test]
fn a_revision_offers_the_compiled_table_rows_it_names_and_reports_the_gone() {
    let (rows, gone) = tune::live_dials(&json!([{"id": "leaf_bases"}, "no_such_dial"])).unwrap();
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].id, "leaf_bases");
    assert_eq!(gone, vec!["no_such_dial"]);
    let (all, gone) = tune::live_dials(&Value::Null).unwrap();
    assert!(all.len() > 100 && gone.is_empty());
}

#[test]
fn a_revision_starts_from_the_last_kept_tree() {
    let dir = scratch("base");
    std::fs::write(
        dir.join("start.json"),
        json!({"overrides": {"canopy": {"size": 1.0}}}).to_string(),
    )
    .unwrap();
    assert_eq!(tune::base(&dir).unwrap(), json!({"canopy": {"size": 1.0}}));
    let written = result(tree("k1"), json!([]), json!([]));
    std::fs::write(tune::result(&dir), written.to_string()).unwrap();
    assert_eq!(tune::base(&dir).unwrap(), tree("k1")["overrides"]);
}

fn assessment(dir: &Path) -> PathBuf {
    let path = dir.join("capability.json");
    let class = |name: &str, class: &str, specs: Value| {
        json!({"capability": name, "class": class, "reason": "read", "captured_by": specs,
               "decided_by": "host", "decided_on": "2026-09-25"})
    };
    let classes = json!({"classes": [
        class("infructescence", "improvement", json!(["fn-111"])),
        class("leaf-base-lattice", "identity", json!(["fn-144"])),
    ]});
    std::fs::write(&path, classes.to_string()).unwrap();
    path
}

#[test]
fn every_failing_trait_is_classed_reachable_identity_or_global_with_its_evidence() {
    let dir = scratch("gaps");
    let gate = json!({"body": {"capability": {
        "required": ["woody-axes", "infructescence", "leaf-base-lattice", "mystery-organ"],
        "missing": ["infructescence", "leaf-base-lattice", "mystery-organ"],
        "unrecognised": [],
    }}});
    let moved = json!([{"dial": "leaf_base_width", "direction": "up", "from": 0.5, "to": 0.9}]);
    let tuned: EndResult = serde_json::from_value(result(
        tree("k"),
        json!([
            gap("crown-density", "stalled in tuning", moved),
            gap("trunk-texture", "stalled in tuning", json!([])),
            gap("frond-count", "passing on the current tree", json!([])),
        ]),
        json!([{"trait": "fruit-clusters-pendent", "spec": "fn-111"}]),
    ))
    .unwrap();
    let classed = gaps::classify(&gate, &assessment(&dir), &tuned).unwrap();
    let kind = |id: &str| classed.iter().find(|g| g.trait_id == id).map(|g| g.kind);
    assert_eq!(kind("infructescence"), Some(Kind::Global));
    assert_eq!(kind("leaf-base-lattice"), Some(Kind::Identity));
    assert_eq!(
        kind("mystery-organ"),
        Some(Kind::Identity),
        "unclassed blocks"
    );
    assert_eq!(kind("fruit-clusters-pendent"), Some(Kind::Global));
    assert_eq!(kind("crown-density"), Some(Kind::Reachable));
    assert_eq!(kind("trunk-texture"), Some(Kind::Identity));
    assert_eq!(kind("frond-count"), None, "a passing trait is no gap");
    let reachable = classed
        .iter()
        .find(|g| g.trait_id == "crown-density")
        .unwrap();
    assert_eq!(reachable.evidence[0], "leaf_base_width 0.5 -> 0.9");
    assert!(classed.iter().all(|g| !g.evidence.is_empty()));
    assert_eq!(kind("capability-assessment"), None);
    // No assessment at all blocks as an unclassed capability does.
    let unassessed = json!({"body": {"capability": {"required": [], "missing": []}}});
    let classed = gaps::classify(&unassessed, &dir.join("absent.json"), &tuned).unwrap();
    assert_eq!(classed[0].trait_id, "capability-assessment");
    assert_eq!(classed[0].kind, Kind::Identity);
}

#[test]
fn an_acceptance_names_the_tree_the_owner_looked_at() {
    let dir = scratch("accept");
    let path = tune::result(&dir);
    std::fs::write(&path, result(tree("k1"), json!([]), json!([])).to_string()).unwrap();
    assert!(!accept::accepted(&path, &dir).unwrap());
    accept::run(&path, &dir).unwrap();
    assert!(accept::accepted(&path, &dir).unwrap());
    let record: Value =
        serde_json::from_str(&std::fs::read_to_string(accept::file(&dir)).unwrap()).unwrap();
    assert_eq!(record["values"]["/canopy/leafBases"], 256);
    // A later revision's tree waits for a look of its own.
    std::fs::write(&path, result(tree("k2"), json!([]), json!([])).to_string()).unwrap();
    assert!(!accept::accepted(&path, &dir).unwrap());
}
