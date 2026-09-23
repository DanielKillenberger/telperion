//! fn-127 R3: on the palm's live run the conductor opened the gap loop on
//! the capability gate while seven owner-only requirements-unmet decisions
//! stood open; the gate's id sorted first. An open owner-only decision now
//! comes before the gap loop, the stages and tuning: the run pauses with its
//! handoff and dispatches nothing. No Jev call is made on this path.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use serde_json::json;
use telperion_jev::caller::{HttpRequest, HttpResponse, Transport};
use telperion_jev::conductor::plan::Next;
use telperion_jev::conductor::questions::Asker;
use telperion_jev::conductor::state::Run;
use telperion_jev::conductor::step::{self, Executor, StageOutcome};
use telperion_jev::conductor::{handoff, BudgetConfig, Config};
use telperion_jev::pipeline::decision::{write_decisions, Decision, DecisionParts};
use telperion_jev::pipeline::judge::Judge;
use telperion_jev::pipeline::stage::STAGES;

struct NoJev;

impl Transport for NoJev {
    fn send(&self, _: &HttpRequest) -> Result<HttpResponse, String> {
        Err("the owner's pause asks Jev nothing".into())
    }
}

struct NoWork;

impl Executor for NoWork {
    fn stage(&self, _: &Config, stage: &str) -> Result<StageOutcome, String> {
        panic!("stage {stage} ran before the owner's decision")
    }
    fn tune(
        &self,
        _: &Config,
        _: u64,
        _: &[String],
        _: &Path,
        _: Option<&Path>,
    ) -> Result<(), String> {
        panic!("tuning ran before the owner's decision")
    }
}

fn config(root: &Path) -> Config {
    let dir = root.join("catalogue/date-palm");
    std::fs::create_dir_all(&dir).unwrap();
    for stage in STAGES {
        let body = serde_json::to_vec(&json!({"stage": stage, "body": {}})).unwrap();
        std::fs::write(dir.join(format!("{stage}.json")), body).unwrap();
    }
    std::fs::write(dir.join("manifest.json"), br#"{"species": "date-palm"}"#).unwrap();
    Config {
        species: "date-palm".into(),
        spec: "fn-82".into(),
        dir,
        run_dir: root.join("run"),
        flow: root.join(".flow"),
        tuning_config: root.join("tuning.json"),
        species_pipeline: PathBuf::from("unused"),
        tuning_loop: PathBuf::from("unused"),
        stage_args: vec![],
        budget: BudgetConfig::default(),
        judgment_model: "jev-latest".into(),
        continuation_validated: true,
    }
}

fn decision(stage: &str, kind: &str, field: &str, blocks: &[&str]) -> Decision {
    Decision::new(
        DecisionParts {
            species: "date-palm",
            stage,
            kind,
            field: Some(field),
            age_years: None,
        },
        blocks,
        BTreeMap::new(),
        vec![],
        json!({}),
        &["add-sources"],
        "",
    )
}

#[test]
fn an_open_owner_decision_pauses_the_run_before_the_gap_loop() {
    for kind in ["requirements-unmet", "manifest-proposed"] {
        let root = std::env::temp_dir().join(format!(
            "jev-owner-first-{kind}-{}-{}",
            std::process::id(),
            telperion_jev::ledger::new_entry_id()
        ));
        let config = config(&root);
        // The live order: the gate's halt sorts before quality's decisions.
        let gate = decision("gate", "onboarding-gate", "capability", &["generate"]);
        let owner = decision("quality", kind, "leaflet_length_m", &["select", "fit"]);
        write_decisions(&config.paths().decisions(), &[gate, owner.clone()]).unwrap();
        let mut run = Run::open(&config).unwrap();
        let asker = Asker {
            judge: Judge {
                transport: &NoJev,
                key: "k",
                ledger_dir: config.ledger_dir(),
            },
            config: &config,
        };
        let (word, next) = step::drive(&asker, &config, &mut run, &NoWork).unwrap();
        assert!(word.starts_with("paused"), "{kind}: {word}");
        assert!(word.contains(&owner.id), "{kind}: {word}");
        assert!(matches!(next, Next::Paused { .. }), "{kind}: {next:?}");
        assert!(run.dispatches.is_empty(), "{kind}: no gap loop opened");
        let pause = run.pause.as_ref().expect("paused");
        let file = handoff::handoff_file(&config, &pause.id);
        let written: serde_json::Value =
            serde_json::from_slice(&std::fs::read(&file).unwrap()).unwrap();
        assert_eq!(written["unresolved"], json!([owner.id]), "{kind}");
        // A second step on the same state observes the pause and adds nothing.
        let (again, _) = step::drive(&asker, &config, &mut run, &NoWork).unwrap();
        assert!(again.starts_with("paused"), "{kind}: {again}");
        assert!(run.dispatches.is_empty());
    }
}
