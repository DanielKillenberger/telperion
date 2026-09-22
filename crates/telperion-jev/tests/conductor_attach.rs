//! The first live run reported its route in `observed`, and the conductor
//! attached that sentence as a spec. Only a spec the Flow tree holds is a
//! minted dependency; anything else leaves the halt with the owner.
use std::path::PathBuf;
use telperion_jev::conductor::{cli::minted_by_gap_loop, state::Run, BudgetConfig, Config};

fn scratch() -> PathBuf {
    let dir = std::env::temp_dir().join(format!("conductor-attach-{}", telperion_jev::ledger::new_entry_id()));
    std::fs::create_dir_all(dir.join(".flow/specs")).unwrap();
    dir
}

fn config(root: &PathBuf) -> Config {
    Config {
        species: "date-palm".into(),
        spec: "fn-82".into(),
        dir: root.clone(),
        run_dir: root.clone(),
        flow: root.join(".flow"),
        tuning_config: root.join("tuning.json"),
        species_pipeline: PathBuf::from("unused"),
        tuning_loop: PathBuf::from("unused"),
        stage_args: vec![],
        budget: BudgetConfig { max_tokens: 1, attempt_max_tokens: 1, max_dispatches: 1, max_tuning_revisions: 1 },
        judgment_model: "jev-test".into(),
        continuation_validated: true,
    }
}

fn run() -> Run {
    serde_json::from_slice(include_bytes!("fixtures/conductor-run-gap-loop-sentence.json")).unwrap()
}

#[test]
fn a_sentence_in_observed_is_not_a_minted_spec() {
    let root = scratch();
    let mut run = run();
    minted_by_gap_loop(&mut run, &config(&root), "dispatch-1");
    assert!(run.dependencies.is_empty(), "{:?}", run.dependencies);
}

#[test]
fn a_spec_the_flow_tree_holds_is_attached_once() {
    let root = scratch();
    std::fs::write(root.join(".flow/specs/fn-108.json"), b"{\"id\":\"fn-108\"}").unwrap();
    let mut run = run();
    run.dispatches[0].result.as_mut().unwrap().observed = "fn-108".into();
    minted_by_gap_loop(&mut run, &config(&root), "dispatch-1");
    minted_by_gap_loop(&mut run, &config(&root), "dispatch-1");
    assert_eq!(run.dependencies.len(), 1);
    assert_eq!(run.dependencies[0].spec, "fn-108");
    assert_eq!(run.dependencies[0].origin, "minted");
}

#[test]
fn a_zero_usage_is_unknown_and_the_estimate_falls_back_to_the_attempt_bound() {
    let mut run = run();
    let mut cfg = config(&scratch());
    cfg.budget.attempt_max_tokens = 200_000;
    run.budget.attempt_max_tokens = 200_000;
    run.dispatches[0].result.as_mut().unwrap().usage =
        Some(serde_json::from_value(serde_json::json!({"input_tokens": 0, "output_tokens": 0})).unwrap());
    let (tokens, basis) = telperion_jev::conductor::dependency::estimate(&run);
    assert_eq!(tokens, 200_000, "{basis}");
    run.dispatches[0].result.as_mut().unwrap().usage =
        Some(serde_json::from_value(serde_json::json!({"input_tokens": 70_000, "output_tokens": 10_000})).unwrap());
    let (tokens, _) = telperion_jev::conductor::dependency::estimate(&run);
    assert_eq!(tokens, 80_000);
}

#[test]
fn a_landed_round_that_left_the_halt_standing_loops_again() {
    use telperion_jev::conductor::plan::{decision_action, Next};
    use telperion_jev::pipeline::decision::{Decision, DecisionParts};
    let root = scratch();
    let cfg = config(&root);
    let run = run(); // carries the gap-loop dispatch for the capability gate
    let table = telperion_jev::conductor::policy::load();
    let halt = Decision::new(
        DecisionParts { species: "date-palm", stage: "gate", kind: "onboarding-gate", field: Some("capability"), age_years: None },
        &["generate"],
        std::collections::BTreeMap::new(),
        vec![],
        serde_json::json!({}),
        &[],
        "",
    );
    assert!(matches!(decision_action(&cfg, &table, &run, &halt), Next::AwaitOwner { .. }), "no landing yet: the halt is the owner's");
    let gap = root.join("gaps").join(telperion_jev::pipeline::gap::slug(&halt.id));
    std::fs::create_dir_all(&gap).unwrap();
    std::fs::write(gap.join("gap.json"), b"{\"landed\":{\"commit\":\"b8b29448\",\"at\":\"2026-09-22T11:42:06Z\"},\"routes\":[{\"at\":\"2026-09-22T11:09:20Z\",\"route\":\"proceed\"}]}").unwrap();
    assert!(matches!(decision_action(&cfg, &table, &run, &halt), Next::GapLoop { .. }), "a landed round loops again");
    // The next round ran and routed to the owner: the halt is the owner's, not a third loop.
    std::fs::write(gap.join("gap.json"), b"{\"landed\":{\"commit\":\"b8b29448\",\"at\":\"2026-09-22T11:42:06Z\"},\"routes\":[{\"at\":\"2026-09-22T11:09:20Z\",\"route\":\"proceed\"},{\"at\":\"2026-09-22T11:52:57Z\",\"route\":\"owner\"}]}").unwrap();
    assert!(matches!(decision_action(&cfg, &table, &run, &halt), Next::AwaitOwner { .. }), "a route after the landing is that round's outcome");
}
