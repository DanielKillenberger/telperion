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
