//! The runner's own stages (fn-149): the live dials a revision offers, the
//! classes of the gap list, and what an acceptance names.
use std::path::{Path, PathBuf};

use serde_json::{json, Value};
use telperion_jev::runner::gaps::{self, Kind};
use telperion_jev::runner::tune;
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

/// A gap entry whose one attempt made `moves`, rendered or not, and judged
/// by the reviewer or not.
fn gap(id: &str, status: &str, moves: Value, feasible: bool, reviewed: bool) -> Value {
    let review = match reviewed {
        true => json!({"per_priority": {id: "slight"}}),
        false => Value::Null,
    };
    let attempts = match moves.as_array().is_some_and(|m| !m.is_empty()) {
        true => json!([{"dial": "bundle", "round": 1, "action_ledger": null,
            "score_before_round": null, "score_after": null, "feasible": feasible,
            "reason": null, "visual_outcome": null, "moves": moves, "review": review,
            "before": [{"view": "P-WHOLE", "seed": 1, "sha256": "a", "path": "r/a.png"}],
            "after": [{"view": "P-WHOLE", "seed": 1, "sha256": "b", "path": "r/b.png"}]}]),
        false => json!([]),
    };
    json!({"id": id, "rank": 1, "priority": id, "status": status, "attempts": attempts, "reviewer_words": ["still thin"],
           "stills": [], "check": "pending"})
}

#[test]
fn a_revision_offers_the_table_rows_it_names_and_reports_the_gone() {
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
            gap(
                "crown-density",
                "failing on the current tree",
                moved.clone(),
                true,
                true
            ),
            gap(
                "trunk-texture",
                "failing on the current tree",
                json!([]),
                true,
                true
            ),
            gap(
                "frond-count",
                "passing on the current tree",
                json!([]),
                true,
                true
            ),
            gap(
                "leaf-sheen",
                "failing on the current tree",
                moved.clone(),
                false,
                true
            ),
            gap(
                "crown-shape",
                "failing on the current tree",
                moved,
                true,
                false
            ),
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
    // A move that never rendered, or that no reviewer judged, reaches nothing.
    assert_eq!(kind("leaf-sheen"), Some(Kind::Identity));
    assert_eq!(kind("crown-shape"), Some(Kind::Identity));
    let reachable = classed
        .iter()
        .find(|g| g.trait_id == "crown-density")
        .unwrap();
    assert_eq!(
        reachable.evidence[0],
        "leaf_base_width 0.5 -> 0.9 (A [P-WHOLE seed 1](r/a.png); B [P-WHOLE seed 1](r/b.png))"
    );
    assert!(classed.iter().all(|g| !g.evidence.is_empty()));
    assert_eq!(kind("capability-assessment"), None);
    // No assessment at all blocks as an unclassed capability does.
    let unassessed = json!({"body": {"capability": {"required": [], "missing": []}}});
    let classed = gaps::classify(&unassessed, &dir.join("absent.json"), &tuned).unwrap();
    assert_eq!(classed[0].trait_id, "capability-assessment");
    assert_eq!(classed[0].kind, Kind::Identity);
    // The capability half alone is what stops a run at its Capability stage.
    let capability = gaps::capability(&gate, &assessment(&dir)).unwrap();
    let names: Vec<&str> = capability.iter().map(|g| g.trait_id.as_str()).collect();
    assert_eq!(
        names,
        ["infructescence", "leaf-base-lattice", "mystery-organ"]
    );
}

#[test]
fn a_revision_reads_every_file_its_config_names() {
    let dir = scratch("referenced");
    let template = dir.join("tuning.json");
    let config = json!({"profiles": "p.json", "matched": {"references": "shots.json"},
        "reference_first": {"inventory": {"path": "inv.json"}, "preparation": {"path": "prep.json"}},
        "sheet": {"protocol": "sheet.py"}});
    std::fs::write(&template, config.to_string()).unwrap();
    let files: Vec<String> = tune::referenced(&template)
        .unwrap()
        .iter()
        .map(|p| p.display().to_string())
        .collect();
    assert_eq!(
        files,
        ["p.json", "shots.json", "inv.json", "prep.json", "sheet.py"]
    );
}

#[test]
fn a_failed_revision_leaves_the_last_kept_result_and_fails_the_stage() {
    let dir = scratch("failed-revision");
    std::fs::write(dir.join("start.json"), json!({"overrides": {}}).to_string()).unwrap();
    let kept = result(tree("k1"), json!([]), json!([])).to_string();
    std::fs::write(tune::result(&dir), &kept).unwrap();
    // A config the revision cannot read fails before any tree is kept.
    let template = dir.join("tuning.json");
    std::fs::write(&template, json!({"dials": []}).to_string()).unwrap();
    let tools = telperion_jev::runner::tools::Tools::at(&dir);
    let err = tune::run(&template, &tools, &dir).unwrap_err();
    assert!(err.starts_with("revision 1 failed"), "{err}");
    assert_eq!(std::fs::read_to_string(tune::result(&dir)).unwrap(), kept);
}
