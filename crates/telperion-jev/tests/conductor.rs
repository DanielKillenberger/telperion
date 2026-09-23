//! The species conductor end to end (fn-89): stages, a tuning revision, the
//! gap check's three verdicts, the human handoff on a new gap, resume, a
//! dependency through design and implementation on separately judged tiers,
//! obsolete and interrupted results, landing, the second tuning revision
//! under the continuation check, the packet and the report. Jev answers
//! through a scripted transport; nothing here reaches the network.
use std::collections::BTreeMap;
use std::sync::Mutex;

use serde_json::{json, Value};
use telperion_jev::conductor::dispatch::Outcome;
use telperion_jev::conductor::plan::{self, Next};
use telperion_jev::conductor::state::{DependencyStatus, Run};
use telperion_jev::conductor::{handoff, report, BudgetConfig, Config};

mod conductor_support;
use conductor_support::*;

#[test]
fn a_run_walks_the_stages_tunes_checks_every_gap_and_escalates_the_new_one() {
    let root = scratch("loop");
    let config = config(&root);
    let executor = Scripted {
        results: Mutex::new(vec![result(
            &root,
            "run-1",
            false,
            vec![
                gap(
                    "finding-0",
                    "reachable trait: crown too narrow",
                    "stalled in tuning",
                    None,
                ),
                gap(
                    "finding-1",
                    "covered trait: leaders lose girth",
                    "stalled in tuning",
                    None,
                ),
                gap(
                    "finding-2",
                    "tied by the loop",
                    "handed off",
                    Some("fn-103"),
                ),
                gap(
                    "finding-3",
                    "new trait: bark plates peel",
                    "stalled in tuning",
                    None,
                ),
                gap("finding-4", "fine", "passing on the current tree", None),
            ],
        )]),
        stage_stops: Mutex::new(BTreeMap::new()),
    };
    let script = Script::new();
    let mut run = Run::open(&config).unwrap();
    assert!(matches!(
        plan::next(&config, &run).unwrap(),
        Next::Stages { .. }
    ));
    let (word, next) = drive(&script, &config, &mut run, &executor);
    assert!(word.contains("report: ran"), "{word}");
    assert_eq!(
        next,
        Next::Tune {
            revision: 1,
            focus: vec![]
        }
    );
    // A second look finds the stages current under the same fingerprint.
    let (_, next) = drive(&script, &config, &mut run, &executor);
    assert!(
        matches!(next, Next::GapCheck { revision: 1, .. }),
        "{next:?}"
    );
    let (word, next) = drive(&script, &config, &mut run, &executor);
    assert!(word.contains("finding-0: reachable with taper"), "{word}");
    assert!(word.contains("finding-1: covered by fn-103"), "{word}");
    assert!(word.contains("finding-2: covered by fn-103"), "{word}");
    assert!(word.contains("finding-3: new, packaged"), "{word}");
    assert!(matches!(next, Next::Paused { .. }), "{next:?}");
    let package: Value =
        serde_json::from_slice(&std::fs::read(config.gaps_dir().join("finding-3.json")).unwrap())
            .unwrap();
    assert_eq!(package["check"], "new");
    assert_eq!(package["overlay"]["taper"], 0.4);
    assert_eq!(package["rounds"][0]["dial"], "crown_width");
    // The covered spec is one dependency, not two.
    assert_eq!(run.dependencies.len(), 1);
    assert_eq!(run.dependencies[0].spec, "fn-103");
    let handoff_path = config
        .conductor_dir()
        .join(format!("handoff-{}.json", run.pause.as_ref().unwrap().id));
    let handoff: Value = serde_json::from_slice(&std::fs::read(&handoff_path).unwrap()).unwrap();
    assert_eq!(handoff["renders"][0]["view"], "whole");
    assert_eq!(handoff["decision_requested"], handoff::REQUESTED_ON_NEW_GAP);
    assert!(handoff["spend"]["tokens"].as_u64().unwrap() > 1000);
    assert!(handoff_path.with_extension("md").exists());
    // Paused: nothing dispatches, and a step changes nothing.
    let before = serde_json::to_value(&run).unwrap();
    let (_, next) = drive(&script, &config, &mut run, &executor);
    assert!(matches!(next, Next::Paused { .. }));
    assert_eq!(
        serde_json::to_value(&run).unwrap()["dispatches"],
        before["dispatches"]
    );
    // A decision that names another pause is refused; the scoped one resumes.
    let pause = run.pause.clone().unwrap();
    let wrong = json!({"pause_id": "other", "identity": pause.identity, "action": pause.basis.proposed_action, "by": "owner", "rationale": "go"});
    write(&root.join("wrong.json"), &wrong);
    assert!(handoff::resume(&mut run, &root.join("wrong.json")).is_err());
    let right = json!({"pause_id": pause.id, "identity": pause.identity, "action": pause.basis.proposed_action, "by": "owner", "rationale": "minted fn-200"});
    write(&root.join("right.json"), &right);
    handoff::resume(&mut run, &root.join("right.json")).unwrap();
    assert!(handoff::attach(&mut run, "fn-200", "finding-3", "minted"));
    assert!(!handoff::attach(&mut run, "fn-200", "finding-3", "minted"));
    let tokens_at_resume = run.budget.tokens;
    // Reopening the record keeps the budget as spent.
    run.save(&config).unwrap();
    let reopened = Run::open(&config).unwrap();
    assert_eq!(reopened.budget.tokens, tokens_at_resume);
    assert_eq!(reopened.dependencies.len(), 2);
}

#[test]
fn a_dependency_is_designed_then_implemented_on_separately_judged_tiers_and_lands() {
    let root = scratch("dependency");
    let config = config(&root);
    let executor = Scripted {
        results: Mutex::new(vec![
            result(
                &root,
                "run-1",
                false,
                vec![gap("finding-1", "covered trait", "stalled in tuning", None)],
            ),
            result(
                &root,
                "run-2",
                true,
                vec![gap(
                    "finding-1",
                    "fine now",
                    "passing on the current tree",
                    None,
                )],
            ),
        ]),
        stage_stops: Mutex::new(BTreeMap::new()),
    };
    let script = Script::new();
    let mut run = Run::open(&config).unwrap();
    for _ in 0..3 {
        drive(&script, &config, &mut run, &executor);
    }
    assert_eq!(run.dependencies[0].status, DependencyStatus::Awaiting);
    // Design complexity: complex, so a high-reasoning designer at medium effort.
    let (word, next) = drive(&script, &config, &mut run, &executor);
    assert!(
        word.contains("design on high_reasoning at medium"),
        "{word}"
    );
    assert!(matches!(next, Next::AwaitDispatch { .. }));
    let design = run.dispatches[0].clone();
    assert_eq!(design.route, "design");
    assert_eq!(
        design.judgments.len(),
        2,
        "a design judgment and a continuation assessment"
    );
    // A step while the dispatch is open re-dispatches nothing.
    drive(&script, &config, &mut run, &executor);
    assert_eq!(run.dispatches.len(), 1);
    // A result for another input identity is obsolete and advances nothing.
    let stale = run
        .ingest(&design.id, verified("other", None, None))
        .unwrap();
    assert_eq!(stale, Outcome::Obsolete);
    assert!(
        run.ingest(&design.id, verified(&design.input_identity, None, None))
            .is_err(),
        "a result is recorded once"
    );
    assert_eq!(run.dependencies[0].status, DependencyStatus::Awaiting);
    // The next hop judges again and opens a fresh design dispatch.
    let (_, _) = drive(&script, &config, &mut run, &executor);
    let design = run.dispatches[1].clone();
    assert_eq!(design.route, "design");
    // An interrupted attempt (no attribution) is retained and advances nothing.
    let mut interrupted = verified(&design.input_identity, None, None);
    interrupted.actual_model = String::new();
    assert_eq!(
        run.ingest(&design.id, interrupted).unwrap(),
        Outcome::Interrupted
    );
    assert_eq!(run.dependencies[0].status, DependencyStatus::Awaiting);
    let (_, _) = drive(&script, &config, &mut run, &executor);
    let design = run.dispatches[2].clone();
    let handoff = root.join("design.md");
    std::fs::write(
        &handoff,
        "interfaces, invariants, difficult cases, verification",
    )
    .unwrap();
    assert_eq!(
        run.ingest(
            &design.id,
            verified(&design.input_identity, None, Some(&handoff))
        )
        .unwrap(),
        Outcome::Verified
    );
    telperion_jev::conductor::dependency::observe(&mut run, &design.id).unwrap();
    assert_eq!(run.dependencies[0].status, DependencyStatus::Designed);
    let revision = run.dependencies[0].design_revision.clone().unwrap();
    // Implementation complexity is judged from the handoff: straightforward, cheap.
    let (word, _) = drive(&script, &config, &mut run, &executor);
    assert!(
        word.contains("implement_cheap on cheap at default"),
        "{word}"
    );
    let implement = run.dispatches[3].clone();
    assert_eq!(
        implement.design_revision.as_deref(),
        Some(revision.as_str())
    );
    // A failed cheap implementation goes to the strong tier at low effort, not back to design.
    let mut failed = verified(&implement.input_identity, Some(&revision), None);
    failed.verification = "failed".into();
    failed.failure = Some("tests red".into());
    assert_eq!(run.ingest(&implement.id, failed).unwrap(), Outcome::Failed);
    let (word, _) = drive(&script, &config, &mut run, &executor);
    assert!(
        word.contains("implement_strong_low on high_reasoning at low"),
        "{word}"
    );
    let strong = run.dispatches[4].clone();
    assert_eq!(
        run.ingest(
            &strong.id,
            verified(&strong.input_identity, Some(&revision), None)
        )
        .unwrap(),
        Outcome::Verified
    );
    telperion_jev::conductor::dependency::observe(&mut run, &strong.id).unwrap();
    assert_eq!(
        run.dependencies[0].status,
        DependencyStatus::AwaitingLanding
    );
    let (word, next) = drive(&script, &config, &mut run, &executor);
    assert!(word.contains("awaiting the landing"), "{word}");
    assert_eq!(
        next,
        Next::AwaitLanding {
            spec: "fn-103".into()
        }
    );
    assert!(telperion_jev::conductor::dependency::land(&mut run, "fn-103", "").is_err());
    telperion_jev::conductor::dependency::land(&mut run, "fn-103", "abc123").unwrap();
    // The landing expires the stages, which rerun, and asks for a second tuning revision.
    assert!(matches!(
        plan::next(&config, &run).unwrap(),
        Next::Stages { .. }
    ));
    let (word, next) = drive(&script, &config, &mut run, &executor);
    assert!(word.contains("report: ran"), "{word}");
    assert_eq!(
        next,
        Next::Tune {
            revision: 2,
            focus: vec![]
        }
    );
    let (word, next) = drive(&script, &config, &mut run, &executor);
    assert!(word.contains("tuning revision 2"), "{word}");
    assert!(run
        .routes
        .iter()
        .any(|r| r == "tuning:2=tune: continuation justified"));
    assert_eq!(next, Next::Packet);
    let (word, next) = drive(&script, &config, &mut run, &executor);
    assert!(word.contains("packet withheld"), "{word}");
    assert!(word.contains("metrics.json"), "{word}");
    assert!(word.contains("ARTICLE.md"), "{word}");
    assert_eq!(next, Next::Packet);
    // With the artifacts in place the packet is ready, and only the owner accepts.
    write(&config.dir.join("metrics.json"), &json!({"autonomy": {}}));
    std::fs::write(config.dir.join("ARTICLE.md"), "# Beech\n").unwrap();
    let (word, next) = drive(&script, &config, &mut run, &executor);
    assert!(word.contains("packet ready"), "{word}");
    assert_eq!(next, Next::Ready);
    let packet: Value =
        serde_json::from_slice(&std::fs::read(config.packet_file()).unwrap()).unwrap();
    assert_eq!(packet["ready_for_owner_review"], true);
    assert!(packet["owner_acceptance"]
        .as_str()
        .unwrap()
        .starts_with("pending"));
    // The report counts every dispatch, by role, tier and effort, with unknown cost explicit.
    let report = report::compute(&config, &run);
    let groups = report["dispatches"].as_array().unwrap();
    let design = groups
        .iter()
        .find(|g| g["role"] == "design" && g["effort"] == "medium")
        .unwrap();
    assert_eq!(design["count"], 3);
    assert_eq!(design["obsolete"], 1);
    assert_eq!(design["interrupted"], 1);
    assert_eq!(design["verified"], 1);
    assert_eq!(design["cost_unknown"], 3);
    assert_eq!(design["actual_models"][0], "model-x at medium");
    assert_eq!(report["failed_fixes"], 1);
    assert_eq!(report["wrong_routes"], 2);
    assert!(report["jev"]["calls"].as_u64().unwrap() >= 8);
    assert!(report["comparison"].is_null());
    assert_eq!(report["human_escalations"], 0);
}

#[test]
fn an_unjustified_next_attempt_pauses_with_budget_remaining_and_a_stage_halt_routes_by_policy() {
    let root = scratch("pause");
    let config = config(&root);
    let executor = Scripted {
        results: Mutex::new(vec![result(
            &root,
            "run-1",
            false,
            vec![gap("finding-1", "covered trait", "stalled in tuning", None)],
        )]),
        stage_stops: Mutex::new(BTreeMap::new()),
    };
    let script = Script::new();
    script.set("risk", "unusual");
    let mut run = Run::open(&config).unwrap();
    for _ in 0..3 {
        drive(&script, &config, &mut run, &executor);
    }
    let (word, next) = drive(&script, &config, &mut run, &executor);
    assert!(word.starts_with("paused"), "{word}");
    assert!(word.contains("human decision required"), "{word}");
    assert!(matches!(next, Next::Paused { .. }));
    assert!(
        run.dispatches.is_empty(),
        "no frontier attempt was bought first"
    );
    assert!(run.budget.remaining() > Some(100_000));
    assert!(run
        .routes
        .iter()
        .any(|r| r.starts_with("dependency:fn-103=human")));
    // Decisions: a halt is the gap loop's, a source decision the cheap agent's, the rest the owner's.
    let table = telperion_jev::conductor::policy::load();
    let decision = |kind: &str| {
        telperion_jev::pipeline::decision::Decision::new(
            telperion_jev::pipeline::decision::DecisionParts {
                species: "beech",
                stage: "fetch",
                kind,
                field: None,
                age_years: None,
            },
            &["fetch"],
            BTreeMap::new(),
            vec![],
            json!({}),
            &[],
            "",
        )
    };
    assert!(matches!(
        plan::decision_action(&table, &run, &decision("onboarding-gate")),
        Next::GapLoop { .. }
    ));
    assert!(matches!(
        plan::decision_action(&table, &run, &decision("unavailable-source")),
        Next::Routine { .. }
    ));
    assert!(matches!(
        plan::decision_action(&table, &run, &decision("tolerance-miss")),
        Next::AwaitOwner { .. }
    ));
    assert!(matches!(
        plan::decision_action(&table, &run, &decision("manifest-proposed")),
        Next::AwaitOwner { .. }
    ));
    // A required field below the requirements table's bar is NEEDS_HUMAN.
    assert!(matches!(
        plan::decision_action(&table, &run, &decision("requirements-unmet")),
        Next::AwaitOwner { .. }
    ));
    // A routine dispatch that returned without resolving hands the decision to the owner.
    let source = decision("unavailable-source");
    let mut run = Run::open(&config).unwrap();
    run.dispatches
        .push(telperion_jev::conductor::dispatch::Dispatch {
            id: "dispatch-9".into(),
            role: telperion_jev::conductor::dispatch::Role::Routine,
            route: "routine".into(),
            tier: "cheap".into(),
            effort: "default".into(),
            dependency: None,
            input_identity: "i".into(),
            design_revision: None,
            scope: format!(
                "resolve {} (unavailable-source) with one of retry",
                source.id
            ),
            judgments: vec![],
            reserved_tokens: Some(1),
            opened_at: String::new(),
            result: None,
        });
    assert!(matches!(
        plan::decision_action(&table, &run, &source),
        Next::Routine { .. }
    ));
    run.ingest("dispatch-9", verified("i", None, None)).unwrap();
    assert!(matches!(
        plan::decision_action(&table, &run, &source),
        Next::AwaitOwner { .. }
    ));
}

/// fn-117: a config with no budget block carries no cap. The tuning revision
/// runs where a set cap pauses, and the spend is recorded and reported either way.
#[test]
fn a_run_without_a_budget_block_runs_where_a_set_cap_pauses_and_records_its_spend() {
    for (cap, expect) in [
        (None, "tuning revision 1"),
        (Some(0), "paused: tuning revision cap"),
    ] {
        let root = scratch("uncapped");
        let mut config = config(&root);
        let mut raw = serde_json::to_value(&config).unwrap();
        raw.as_object_mut().unwrap().remove("budget");
        write(&root.join("conductor.json"), &raw);
        config = Config::load(&root.join("conductor.json")).unwrap();
        assert_eq!(config.budget, BudgetConfig::default());
        config.budget.max_tuning_revisions = cap;
        let executor = Scripted {
            results: Mutex::new(vec![result(&root, "run-1", false, vec![])]),
            stage_stops: Mutex::new(BTreeMap::new()),
        };
        let script = Script::new();
        let mut run = Run::open(&config).unwrap();
        drive(&script, &config, &mut run, &executor);
        let (word, _) = drive(&script, &config, &mut run, &executor);
        assert!(word.contains(expect), "{cap:?}: {word}");
        if cap.is_none() {
            assert!(run.budget.tokens >= 1000, "{}", run.budget.tokens);
            let report = report::compute(&config, &run);
            assert_eq!(report["tokens_total_known"], run.budget.tokens);
        }
    }
}
