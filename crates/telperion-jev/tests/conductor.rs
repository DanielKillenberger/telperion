//! The species conductor end to end (fn-89): stages, a tuning revision, the
//! gap check's three verdicts, the human handoff on a new gap, resume, a
//! dependency through design and implementation on separately judged tiers,
//! obsolete and interrupted results, landing, the second tuning revision
//! under the continuation check, the packet and the report. Jev answers
//! through a scripted transport; nothing here reaches the network.
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

use serde_json::{json, Value};
use telperion_jev::caller::{HttpRequest, HttpResponse, Transport};
use telperion_jev::conductor::dispatch::{DispatchResult, Outcome};
use telperion_jev::conductor::plan::{self, Next};
use telperion_jev::conductor::questions::Asker;
use telperion_jev::conductor::state::{DependencyStatus, Run};
use telperion_jev::conductor::step::{self, Executor, StageOutcome};
use telperion_jev::conductor::{handoff, report, BudgetConfig, Config};
use telperion_jev::ledger::Usage;
use telperion_jev::pipeline::judge::Judge;
use telperion_jev::pipeline::stage::STAGES;
use telperion_jev::tuning::handoff::Attempt;
use telperion_jev::tuning::result::{
    CurrentTree, EndResult, GapEntry, Outcome as TuningOutcome, Still,
};

/// Answers by question name. Reach and cover read the trait's words: a
/// trait saying "reachable" gets the first untried dial, one saying
/// "covered" the first open spec, anything else the no-match answer.
struct Script {
    choices: Mutex<BTreeMap<String, String>>,
}

impl Script {
    fn new() -> Self {
        let mut choices = BTreeMap::new();
        for (q, a) in [
            ("design_complexity", "complex"),
            ("implementation_complexity", "straightforward"),
            ("tractability", "supported"),
            ("progress", "supported"),
            ("risk", "bounded"),
        ] {
            choices.insert(q.to_string(), a.to_string());
        }
        Self {
            choices: Mutex::new(choices),
        }
    }
    fn set(&self, question: &str, answer: &str) {
        self.choices
            .lock()
            .unwrap()
            .insert(question.into(), answer.into());
    }
}

impl Transport for Script {
    fn send(&self, request: &HttpRequest) -> Result<HttpResponse, String> {
        let body: Value = serde_json::from_slice(request.body.as_deref().unwrap_or(b"{}"))
            .map_err(|err| err.to_string())?;
        let trait_words = body["state"]["trait"]
            .as_str()
            .unwrap_or_default()
            .to_string();
        let mut answers = serde_json::Map::new();
        for (name, question) in body["questions"].as_object().into_iter().flatten() {
            let choice = match name.as_str() {
                "reachable_with" | "covered_by" => {
                    let wanted = if name == "reachable_with" {
                        "reachable"
                    } else {
                        "covered"
                    };
                    if trait_words.contains(wanted) {
                        question["criteria"]
                            .as_object()
                            .and_then(|c| c.keys().find(|k| *k != "none").cloned())
                            .unwrap_or_else(|| "none".into())
                    } else {
                        "none".into()
                    }
                }
                other => self
                    .choices
                    .lock()
                    .unwrap()
                    .get(other)
                    .cloned()
                    .unwrap_or_else(|| "none".into()),
            };
            answers.insert(
                name.clone(),
                json!({"type": "choice", "choice": choice, "probabilities": {choice.clone(): 0.9}, "confidence": 0.9}),
            );
        }
        Ok(HttpResponse {
            status: 200,
            body: serde_json::to_vec(&json!({"model": "jev-latest", "answers": answers,
                "usage": {"input_tokens": 50, "output_tokens": 5}}))
            .unwrap(),
        })
    }
}

/// Every stage runs; a tuning revision writes the result the test staged.
struct Scripted {
    results: Mutex<Vec<EndResult>>,
    stage_stops: Mutex<BTreeMap<String, String>>,
}

impl Executor for Scripted {
    fn stage(&self, _config: &Config, stage: &str) -> Result<StageOutcome, String> {
        if let Some(stop) = self.stage_stops.lock().unwrap().get(stage) {
            return Err(stop.clone());
        }
        Ok(StageOutcome::Ran)
    }
    fn tune(
        &self,
        _config: &Config,
        _revision: u64,
        _focus: &[String],
        out: &PathBuf,
    ) -> Result<(), String> {
        let mut results = self.results.lock().unwrap();
        let result = results.remove(0);
        std::fs::create_dir_all(out).unwrap();
        std::fs::write(
            out.join("result.json"),
            serde_json::to_vec_pretty(&result).unwrap(),
        )
        .unwrap();
        Ok(())
    }
}

fn scratch(tag: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "jev-conductor-{tag}-{}-{}",
        std::process::id(),
        telperion_jev::ledger::new_entry_id()
    ));
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

fn write(path: &Path, value: &Value) {
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    std::fs::write(path, serde_json::to_vec_pretty(value).unwrap()).unwrap();
}

fn config(root: &Path) -> Config {
    let flow = root.join(".flow");
    write(
        &flow.join("specs/fn-103.json"),
        &json!({"id": "fn-103", "title": "Leaders keep their girth", "status": "open"}),
    );
    std::fs::write(
        flow.join("specs/fn-103.md"),
        "# Leaders keep their girth\n\nA lateral takes girth from its parent.\n",
    )
    .unwrap();
    write(
        &flow.join("specs/fn-1.json"),
        &json!({"id": "fn-1", "title": "Closed", "status": "done"}),
    );
    write(
        &flow.join("specs/fn-200.json"),
        &json!({"id": "fn-200", "title": "Minted later", "status": "open"}),
    );
    std::fs::write(flow.join("specs/fn-200.md"), "# Minted later\n").unwrap();
    let tuning = root.join("tuning.json");
    write(
        &tuning,
        &json!({"dials": [{"id": "crown_width", "meaning": "how wide the crown spreads"}, {"id": "taper", "meaning": "how fast an axis thins"}]}),
    );
    let dir = root.join("catalogue/beech");
    std::fs::create_dir_all(&dir).unwrap();
    for stage in STAGES {
        write(
            &dir.join(format!("{stage}.json")),
            &json!({"stage": stage, "body": {"status": "complete"}}),
        );
    }
    write(&dir.join("manifest.json"), &json!({"species": "beech"}));
    Config {
        species: "beech".into(),
        spec: "fn-62".into(),
        dir,
        run_dir: root.join("run"),
        flow,
        tuning_config: tuning,
        species_pipeline: PathBuf::from("unused"),
        tuning_loop: PathBuf::from("unused"),
        stage_args: vec![],
        budget: BudgetConfig {
            max_tokens: 200_000,
            attempt_max_tokens: 40_000,
            max_dispatches: 8,
            max_tuning_revisions: 3,
        },
        judgment_model: "jev-latest".into(),
        continuation_validated: true,
    }
}

fn attempt(dial: &str) -> Attempt {
    Attempt {
        dial: dial.into(),
        round: 1,
        action_ledger: None,
        score_before_round: None,
        score_after: None,
        feasible: true,
        reason: Some("no better".into()),
        visual_outcome: None,
        review: None,
    }
}

fn gap(id: &str, priority: &str, status: &str, existing: Option<&str>) -> GapEntry {
    GapEntry {
        id: id.into(),
        rank: 1,
        priority: priority.into(),
        status: status.into(),
        latest_route: Some("tuning".into()),
        existing_spec: existing.map(str::to_string),
        attempts: vec![attempt("crown_width")],
        reviewer_words: vec![priority.into()],
        stills: vec![],
        check: String::new(),
    }
}

fn result(root: &Path, identity: &str, machine_ready: bool, gaps: Vec<GapEntry>) -> EndResult {
    let still = root.join("run/stills/whole-1.png");
    std::fs::create_dir_all(still.parent().unwrap()).unwrap();
    std::fs::write(&still, b"png").unwrap();
    EndResult {
        meaning: "test".into(),
        outcome: TuningOutcome {
            run_identity: identity.into(),
            preset: "beech".into(),
            seed: 1,
            bootstrap: false,
            machine_ready,
            reviewer_passed_unqualified: false,
            owner_acceptance: "pending".into(),
            stopped: "ended".into(),
            adoptions_kept: 1,
            adoptions_rolled_back: 0,
            budget: json!({"tokens": 1000, "images": 4}),
            current: Some(CurrentTree {
                key: "t1".into(),
                round: 2,
                label: "taper up".into(),
                score_telemetry: None,
                overrides: json!({"taper": 0.4}),
                stills: vec![Still {
                    view: "whole".into(),
                    seed: 1,
                    sha256: "ab".repeat(32),
                    path: still.display().to_string(),
                }],
            }),
            finalists: vec![],
        },
        gaps,
        gaps_note: String::new(),
    }
}

fn verified(identity: &str, revision: Option<&str>, handoff: Option<&Path>) -> DispatchResult {
    DispatchResult {
        input_identity: identity.into(),
        design_revision: revision.map(str::to_string),
        actual_model: "model-x".into(),
        actual_effort: "medium".into(),
        usage: Some(Usage {
            input_tokens: 1000,
            output_tokens: 200,
        }),
        cost_usd: None,
        wall_ms: Some(10),
        verification: "verified".into(),
        observed: "done".into(),
        handoff: handoff.map(Path::to_path_buf),
        failure: None,
        usage_is_reservation: false,
        outcome: None,
        finished_at: String::new(),
    }
}

fn drive(script: &Script, config: &Config, run: &mut Run, executor: &Scripted) -> (String, Next) {
    let asker = Asker {
        judge: Judge {
            transport: script,
            key: "k",
            ledger_dir: config.ledger_dir(),
        },
        config,
    };
    step::drive(&asker, config, run, executor).unwrap()
}

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
        1,
        "a design judgment only: a first attempt is bounded by construction and asks no continuation"
    );
    assert_eq!(
        run.dependencies[0].judged.len(),
        1,
        "the design judgment is kept with the evidence it read"
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
    // The first attempt is bounded by construction and proceeds without a
    // continuation question; it fails, and the repeat is what the trio judges.
    let (word, _) = drive(&script, &config, &mut run, &executor);
    assert!(word.starts_with("dispatch"), "{word}");
    let first = run.dispatches[0].clone();
    let mut failed = verified(&first.input_identity, None, None);
    failed.failure = Some("the design did not verify".into());
    run.ingest(&first.id, failed).unwrap();
    let (word, next) = drive(&script, &config, &mut run, &executor);
    assert!(word.starts_with("paused"), "{word}");
    assert!(word.contains("human decision required"), "{word}");
    assert!(matches!(next, Next::Paused { .. }));
    assert_eq!(
        run.dispatches.len(),
        1,
        "the first bounded attempt was bought; no second frontier attempt was"
    );
    // A scoped human resume authorizes the attempt it names: the next step
    // opens it without asking the trio again, even with risk still unusual.
    let pause = run.pause.clone().unwrap();
    let decision: telperion_jev::tuning::continuation::HumanDecision = serde_json::from_value(json!({
        "pause_id": pause.id, "identity": pause.basis.identity, "action": pause.basis.proposed_action,
        "by": "test owner", "rationale": "continue"
    }))
    .unwrap();
    // A cap moves only from the value the run holds: a stale `previous` is refused.
    let mut wrong = decision.clone();
    wrong.round_cap_extension =
        Some(serde_json::from_value(json!({"previous": 99, "next": 100})).unwrap());
    assert!(run.resume(wrong).is_err());
    let mut raise = decision.clone();
    let held = run.budget.max_dispatches;
    raise.round_cap_extension =
        Some(serde_json::from_value(json!({"previous": held, "next": held + 4})).unwrap());
    run.resume(raise).unwrap();
    assert_eq!(run.budget.max_dispatches, held + 4);
    assert_eq!(run.resumed_from.as_deref(), Some("pause-1"));
    let (word, _) = drive(&script, &config, &mut run, &executor);
    assert!(word.starts_with("dispatch"), "{word}");
    assert!(
        run.resumed_from.is_none(),
        "opening the attempt clears the authorization"
    );
    assert_eq!(run.dispatches.len(), 2);
    assert!(run.budget.remaining() > 100_000);
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
        plan::decision_action(&config, &table, &run, &decision("onboarding-gate")),
        Next::GapLoop { .. }
    ));
    assert!(matches!(
        plan::decision_action(&config, &table, &run, &decision("unavailable-source")),
        Next::Routine { .. }
    ));
    assert!(matches!(
        plan::decision_action(&config, &table, &run, &decision("tolerance-miss")),
        Next::AwaitOwner { .. }
    ));
    assert!(matches!(
        plan::decision_action(&config, &table, &run, &decision("manifest-proposed")),
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
            reserved_tokens: 1,
            opened_at: String::new(),
            result: None,
        });
    assert!(matches!(
        plan::decision_action(&config, &table, &run, &source),
        Next::Routine { .. }
    ));
    run.ingest("dispatch-9", verified("i", None, None)).unwrap();
    assert!(matches!(
        plan::decision_action(&config, &table, &run, &source),
        Next::AwaitOwner { .. }
    ));
}

#[test]
fn an_open_decision_that_blocks_nothing_does_not_stop_the_run_for_the_owner() {
    let root = scratch("nonblocking");
    let config = config(&root);
    let decisions = telperion_jev::pipeline::decision::Decision::new(
        telperion_jev::pipeline::decision::DecisionParts {
            species: "beech",
            stage: "generate",
            kind: "visual-unassessed",
            field: None,
            age_years: None,
        },
        &[],
        BTreeMap::new(),
        vec![],
        json!({}),
        &["accept", "reject"],
        "the owner's verdict on the stills, after tuning",
    );
    write(
        &config.paths().decisions(),
        &json!({"schema": "decisions", "schema_version": 1, "decisions": [decisions]}),
    );
    let run = Run::open(&config).unwrap();
    let next = plan::next(&config, &run).unwrap();
    assert!(!matches!(next, Next::AwaitOwner { .. }), "{next:?}");
}
