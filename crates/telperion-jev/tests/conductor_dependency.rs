//! A dependency's two exits the first live run lacked (fn-80 amendment):
//! an answer still under the confidence floor after one verified
//! investigation pauses for the human instead of buying a second one, and a
//! dependency the host built outside the conductor's dispatches is adopted
//! and landed in one step. Jev answers through a scripted transport.
use std::collections::BTreeMap;
use std::path::PathBuf;
use std::sync::Mutex;

use telperion_jev::conductor::adopt::adopt;
use telperion_jev::conductor::dependency;
use telperion_jev::conductor::dispatch::{read_result, DispatchResult, Outcome, Role};
use telperion_jev::conductor::plan::{self, Next};
use telperion_jev::conductor::state::{DependencyStatus, Run};
use telperion_jev::conductor::Config;

mod conductor_support;
use conductor_support::*;

const PAUSE: &str = "insufficient evidence after one investigation";

/// A run with fn-103 attached and awaiting its first judgment.
fn awaiting(tag: &str) -> (PathBuf, Config, Run, Scripted, Script) {
    let root = scratch(tag);
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
    let mut run = Run::open(&config).unwrap();
    for _ in 0..3 {
        drive(&script, &config, &mut run, &executor);
    }
    assert_eq!(run.dependencies[0].status, DependencyStatus::Awaiting);
    (root, config, run, executor, script)
}

/// Opens the investigation an under-floor answer buys, verifies it, and
/// returns what the next hop does.
fn investigate_then_step(
    script: &Script,
    config: &Config,
    run: &mut Run,
    executor: &Scripted,
    revision: Option<&str>,
) -> (String, Next) {
    let (word, _) = drive(script, config, run, executor);
    assert!(word.contains("investigate on cheap at default"), "{word}");
    let investigation = run.dispatches.last().unwrap().clone();
    assert_eq!(investigation.role, Role::Investigate);
    let found = verified(&investigation.input_identity, revision, None);
    assert_eq!(
        run.ingest(&investigation.id, found).unwrap(),
        Outcome::Verified
    );
    dependency::observe(run, &investigation.id).unwrap();
    drive(script, config, run, executor)
}

fn assert_paused_once(run: &Run, dispatches: usize, word: &str, next: &Next) {
    assert!(word.contains(PAUSE), "{word}");
    assert!(matches!(next, Next::Paused { .. }), "{next:?}");
    assert_eq!(
        run.dispatches.len(),
        dispatches,
        "no second investigation of the same revision"
    );
    assert!(run.pause.is_some());
}

#[test]
fn an_under_floor_design_answer_after_one_investigation_pauses_for_the_human() {
    let (_root, config, mut run, executor, script) = awaiting("design-floor");
    script.set_confidence("design_complexity", 0.5);
    let (word, next) = investigate_then_step(&script, &config, &mut run, &executor, None);
    assert_paused_once(&run, 1, &word, &next);
    let (word, _) = drive(&script, &config, &mut run, &executor);
    assert!(word.contains("paused"), "{word}");
    assert_eq!(run.dispatches.len(), 1);
}

#[test]
fn an_under_floor_implementation_answer_after_one_investigation_pauses_for_the_human() {
    let (root, config, mut run, executor, script) = awaiting("implementation-floor");
    let (word, _) = drive(&script, &config, &mut run, &executor);
    assert!(word.contains("design on high_reasoning"), "{word}");
    let design = run.dispatches[0].clone();
    let handoff = root.join("design.md");
    std::fs::write(&handoff, "interfaces, invariants, verification").unwrap();
    let designed = verified(&design.input_identity, None, Some(&handoff));
    run.ingest(&design.id, designed).unwrap();
    dependency::observe(&mut run, &design.id).unwrap();
    let revision = run.dependencies[0].design_revision.clone().unwrap();
    script.set_confidence("implementation_complexity", 0.5);
    let (word, next) =
        investigate_then_step(&script, &config, &mut run, &executor, Some(&revision));
    assert_paused_once(&run, 2, &word, &next);
}

/// A result file the host wrote, read the way `dispatch --result` reads one.
fn host_result(root: &std::path::Path, verification: &str) -> DispatchResult {
    let mut built = verified("host-built", None, None);
    built.verification = verification.into();
    built.usage = None;
    let path = root.join(format!("adopt-{verification}.json"));
    write(&path, &serde_json::to_value(&built).unwrap());
    read_result(&path).unwrap()
}

#[test]
fn the_host_adopts_a_dependency_it_built_and_it_lands_in_one_step() {
    let (root, config, mut run, executor, script) = awaiting("adopt");
    let (word, _) = drive(&script, &config, &mut run, &executor);
    assert!(word.contains("design on high_reasoning"), "{word}");
    let open = run.dispatches[0].id.clone();
    let result = host_result(&root, "verified");
    let word = adopt(&mut run, "fn-103", "abc123", result).unwrap();
    assert!(word.contains("landed"), "{word}");
    let closed = &run.dispatches[0];
    assert_eq!(closed.id, open);
    assert_eq!(closed.outcome(), Some(Outcome::Obsolete));
    let note = closed.result.as_ref().unwrap().failure.clone().unwrap();
    assert!(note.contains("host built"), "{note}");
    let adopted = run.dispatches.last().unwrap();
    assert_eq!(adopted.role, Role::Implement);
    assert_eq!(adopted.route, "host_built");
    assert_eq!(adopted.reserved_tokens, Some(0));
    assert_eq!(adopted.design_revision, run.dependencies[0].design_revision);
    assert_eq!(adopted.outcome(), Some(Outcome::Verified));
    assert_eq!(run.dependencies[0].status, DependencyStatus::Landed);
    assert_eq!(run.dependencies[0].landed_commit.as_deref(), Some("abc123"));
    assert!(run.open_dispatch().is_none());
    assert!(matches!(
        plan::next(&config, &run).unwrap(),
        Next::Stages { .. }
    ));
}

#[test]
fn adopt_refuses_a_landed_dependency_an_unverified_result_and_an_unknown_spec() {
    let (root, _config, mut run, _executor, _script) = awaiting("adopt-refusals");
    let good = host_result(&root, "verified");
    let failed = host_result(&root, "failed");
    let before = run.dispatches.len();
    let err = adopt(&mut run, "fn-103", "abc123", failed).unwrap_err();
    assert!(err.to_string().contains("not verified"), "{err}");
    let err = adopt(&mut run, "fn-999", "abc123", good.clone()).unwrap_err();
    assert!(err.to_string().contains("no dependency fn-999"), "{err}");
    assert_eq!(run.dispatches.len(), before, "a refusal records nothing");
    assert_eq!(run.dependencies[0].status, DependencyStatus::Awaiting);
    adopt(&mut run, "fn-103", "abc123", good.clone()).unwrap();
    let err = adopt(&mut run, "fn-103", "def456", good).unwrap_err();
    assert!(err.to_string().contains("Landed"), "{err}");
    assert_eq!(run.dependencies[0].landed_commit.as_deref(), Some("abc123"));
}
