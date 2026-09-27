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
    gap_passing(id, status, moves, feasible, reviewed, false)
}

/// As `gap`, with the attempt's own visual passing the trait when `passed`.
fn gap_passing(
    id: &str,
    status: &str,
    moves: Value,
    feasible: bool,
    reviewed: bool,
    passed: bool,
) -> Value {
    let review = match reviewed {
        true => json!({"per_priority": {id: "slight"}}),
        false => Value::Null,
    };
    let visual = match passed {
        true => json!([{"item": format!("owner-priority:{id}: the trait"), "view": "P-WHOLE",
                        "seed": 1, "status": "pass"}]),
        false => Value::Null,
    };
    let attempts = match moves.as_array().is_some_and(|m| !m.is_empty()) {
        true => json!([{"dial": "bundle", "round": 1, "action_ledger": null,
            "score_before_round": null, "score_after": null, "feasible": feasible,
            "reason": null, "visual_outcome": visual, "moves": moves, "review": review,
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

/// The dial table is compiled into the runner and keyed by the checkout's
/// reference: the reference this runner renders is accepted, one that moved
/// is refused, and a named-dial config resolves at the table's revision.
#[test]
fn a_revision_is_refused_by_a_runner_built_from_another_catalogue() {
    let on_disk = std::fs::read_to_string(tune::reference()).unwrap();
    assert_eq!(tune::same_catalogue(&on_disk), Ok(()));
    let moved = on_disk.replacen("window", "range", 1);
    let refused = tune::same_catalogue(&moved).unwrap_err();
    assert!(refused.contains("rebuild it"), "{refused}");
    let named =
        json!({"catalogue": telperion_jev::tuning::table::revision(), "ids": ["leaf_bases"]});
    let (rows, gone) = tune::live_dials(&named).unwrap();
    assert_eq!(
        (rows.len(), rows[0].id.as_str(), gone.len()),
        (1, "leaf_bases", 0)
    );
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
    // The traits the host assessed (fn-157): one the generator cannot draw,
    // waiting on its spec, and one a missing capability classed an
    // improvement covers.
    let traits = json!([
        {"trait": "trunk-texture", "need": "a lattice of leaf bases", "outcome": "unsupported-anatomy",
         "capability": null, "depends_on": "fn-144", "confidence": "high", "note": ""},
        {"trait": "fruit-look", "need": "hanging date clusters", "outcome": "unreachable-value",
         "capability": "infructescence", "depends_on": null, "confidence": "high", "note": ""},
        {"trait": "leaf-sheen", "need": "a glossy leaf", "outcome": "reachable",
         "capability": null, "depends_on": "fn-999", "confidence": "high", "note": ""},
        {"trait": "skeleton.habit.lateral_pitch (by height)", "need": "lower limbs spread wider",
         "outcome": "unreachable-value", "capability": null, "depends_on": "fn-61",
         "confidence": "medium", "note": "", "covers": ["limb-arch"]},
    ]);
    let classes = json!({"traits": traits, "classes": [
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
                moved.clone(),
                true,
                false
            ),
            gap_passing(
                "crown-openness",
                "failing on the current tree",
                moved.clone(),
                true,
                true,
                true
            ),
            gap(
                "fruit-look",
                "failing on the current tree",
                moved.clone(),
                true,
                true
            ),
            gap(
                "limb-arch",
                "failing on the current tree",
                moved,
                true,
                true
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
    // Host, 2026-09-26: reachable is a dial that moved the trait to passing
    // in a rendered attempt the reviewer judged. A trait moved but still
    // failing is the assessment's: identity with the spec it waits on,
    // global when a capability classed an improvement covers it, else
    // identity for the host to assess.
    assert_eq!(kind("crown-openness"), Some(Kind::Reachable));
    assert_eq!(kind("crown-density"), Some(Kind::Identity));
    let specs = |id: &str| {
        classed
            .iter()
            .find(|g| g.trait_id == id)
            .unwrap()
            .specs
            .clone()
    };
    let said = |id: &str| {
        classed
            .iter()
            .find(|g| g.trait_id == id)
            .unwrap()
            .evidence
            .join(" ")
    };
    assert!(specs("crown-density").is_empty());
    assert!(
        said("crown-density").contains("host to assess"),
        "{}",
        said("crown-density")
    );
    assert_eq!(kind("trunk-texture"), Some(Kind::Identity));
    assert_eq!(specs("trunk-texture"), ["fn-144"]);
    // An assessed entry names a failing trait by `trait` or in `covers`,
    // nothing else (host decision A, 2026-09-26).
    assert_eq!(kind("limb-arch"), Some(Kind::Identity));
    assert_eq!(specs("limb-arch"), ["fn-61"]);
    assert_eq!(kind("fruit-look"), Some(Kind::Global));
    assert_eq!(specs("fruit-look"), ["fn-111"]);
    assert_eq!(kind("frond-count"), None, "a passing trait is no gap");
    // A move that never rendered, or that no reviewer judged, reaches
    // nothing; an assessment entry the host found reachable names no spec.
    assert_eq!(kind("leaf-sheen"), Some(Kind::Identity));
    assert!(specs("leaf-sheen").is_empty());
    assert_eq!(kind("crown-shape"), Some(Kind::Identity));
    let reachable = classed
        .iter()
        .find(|g| g.trait_id == "crown-openness")
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
    assert_eq!(files, ["p.json", "shots.json", "sheet.py"]);
}

#[test]
fn a_failed_revision_leaves_the_last_kept_result_and_fails_the_stage() {
    let dir = scratch("failed-revision");
    std::fs::write(dir.join("start.json"), json!({"overrides": {}}).to_string()).unwrap();
    let kept = result(tree("k1"), json!([]), json!([])).to_string();
    std::fs::write(tune::result(&dir), &kept).unwrap();
    // A config the revision cannot read fails before any tree is kept.
    let template = dir.join("tuning.json");
    let config = json!({"dials": [], "references": [{"path": "r.png", "sha256": "r", "view": "whole", "seed": 0}]});
    std::fs::write(&template, config.to_string()).unwrap();
    let inventory = telperion_jev::runner::inventory::dir(&template, &dir).unwrap();
    std::fs::create_dir_all(&inventory).unwrap();
    for file in ["inventory.json", "preparation.json"] {
        std::fs::write(inventory.join(file), "{}").unwrap();
    }
    let tools = telperion_jev::runner::tools::Tools::at(&dir);
    let err = tune::run(&template, &tools, &dir).unwrap_err();
    assert!(err.starts_with("revision 1 failed"), "{err}");
    assert_eq!(std::fs::read_to_string(tune::result(&dir)).unwrap(), kept);
}

/// No photograph found and none in the config (the beech, 2026-09-25): the
/// Profile stage skips the inventory and notes a references gap that stops
/// nothing, and Tune refuses, naming why.
#[test]
fn no_reference_photograph_skips_the_inventory_notes_a_gap_and_tune_refuses() {
    use telperion_jev::runner::inventory;
    let dir = scratch("no-references");
    let template = dir.join("tuning.json");
    std::fs::write(&template, json!({"references": []}).to_string()).unwrap();
    let word = inventory::build(&template, &dir).unwrap();
    assert!(word.contains("inventory skipped"), "{word}");
    assert_eq!(
        inventory::files(&template, &dir).unwrap(),
        [inventory::found(&dir)]
    );
    gaps::note_references(&template, &dir).unwrap();
    let md = std::fs::read_to_string(gaps::files(&dir).1).unwrap();
    assert!(md.contains("**references**"), "{md}");
    assert!(
        gaps::identity(&dir).unwrap().is_empty(),
        "a missing photograph stops nothing"
    );
    let tools = telperion_jev::runner::tools::Tools::at(&dir);
    let err = tune::run(&template, &tools, &dir).unwrap_err();
    assert!(err.starts_with("no reference photograph"), "{err}");
}

/// An article claim the citation check left open is logged and never
/// waited on, unless the run leaves claims to a person (`--settle-claims`),
/// when it stops the run; no other decision is a claim (fn-157).
#[test]
fn an_open_article_claim_stops_only_a_run_that_leaves_claims_to_a_person() {
    use telperion_jev::pipeline::decision::{append_decisions, Decision, DecisionParts};
    use telperion_jev::pipeline::stage::Paths;
    use telperion_jev::runner::{literature, Run};
    let decision = |kind: &str, field: &str| {
        Decision::new(
            DecisionParts {
                species: "s",
                stage: "document",
                kind,
                field: Some(field),
                age_years: None,
            },
            &["accept"],
            [("article".to_string(), "a".to_string())]
                .into_iter()
                .collect(),
            vec![],
            json!({}),
            &["accept", "rewrite"],
            "",
        )
    };
    let stop = |tag: &str, person: bool| {
        let dir = scratch(tag);
        append_decisions(
            &dir.join("decisions.json"),
            vec![
                decision("article-claim-unsupported", "C1"),
                decision("level-miss", "crown"),
            ],
        )
        .unwrap();
        let mut run = Run::new("s", Paths::new(&dir), dir.join("c"), dir.join("t.json"));
        run.settle_claims = person;
        literature::claims(&run).unwrap()
    };
    assert_eq!(stop("claims-logged", false), None);
    let ids = vec!["s/document/article-claim-unsupported/C1".to_string()];
    assert_eq!(
        stop("claims-person", true),
        Some(telperion_jev::runner::Stop::Claims(ids))
    );
}

/// fn-157: gaps.md is read by a person. Each gap is a line with its first
/// few pieces of evidence, each cut short; the rest stays in gaps.json. The
/// beech's reached 1.3 MB.
#[test]
fn gaps_md_keeps_each_gap_to_a_readable_line() {
    let dir = scratch("gaps-md");
    let long = format!(
        "spread 0.16 -> 0.21 ({})",
        "[photo seed 1](run/matched/a.png) ".repeat(40)
    );
    let gap = gaps::Gap {
        trait_id: "broad-domed-crown".into(),
        kind: Kind::Identity,
        evidence: vec![long; 50],
        specs: vec!["fn-61".into()],
    };
    gaps::write(&dir, &[gap]).unwrap();
    let (json_path, md_path) = gaps::files(&dir);
    let md = std::fs::read_to_string(md_path).unwrap();
    assert!(md.len() < 3000, "{} bytes", md.len());
    assert!(md.contains("**broad-domed-crown** (fn-61)"), "{md}");
    assert!(md.contains("more in gaps.json"), "{md}");
    let json: Value = serde_json::from_slice(&std::fs::read(json_path).unwrap()).unwrap();
    assert_eq!(json["gaps"][0]["evidence"].as_array().unwrap().len(), 50);
}

/// fn-170 R8: the recorded beech's assessment names fn-170 for its
/// codominant V fork, and the gap classifier classes the fork identity
/// against it while the limbs and the crown stay against fn-61.
#[test]
fn the_recorded_beech_classes_its_v_fork_against_fn_170() {
    let assessment = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/replay/european-beech/seed/packet/capability.json");
    let gate = json!({"body": {"capability": {
        "required": ["woody-axes", "entire-blade", "alternate-petiole"],
        "missing": [], "unrecognised": [],
    }}});
    let moved = json!([{"dial": "codominance", "direction": "up", "from": 0.0, "to": 0.4}]);
    let failing = |id: &str| gap(id, "failing on the current tree", moved.clone(), true, true);
    let tuned: EndResult = serde_json::from_value(result(
        tree("k"),
        json!([
            failing("codominant-v-fork"),
            failing("ascending-then-arching-limbs"),
            failing("broad-domed-crown"),
        ]),
        json!([]),
    ))
    .unwrap();
    let classed = gaps::classify(&gate, &assessment, &tuned).unwrap();
    let class = |id: &str| {
        let gap = classed.iter().find(|g| g.trait_id == id).unwrap();
        (gap.kind, gap.specs.clone())
    };
    let against = |spec: &str| (Kind::Identity, vec![spec.to_string()]);
    assert_eq!(
        class("codominant-v-fork"),
        against("fn-170-codominant-forks-one-rule-from-the")
    );
    let fn61 = against("fn-61-trolls-model-limbs-pitch-by-height-and");
    assert_eq!(class("ascending-then-arching-limbs"), fn61);
    assert_eq!(class("broad-domed-crown"), fn61);
}
