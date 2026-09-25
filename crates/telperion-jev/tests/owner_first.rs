//! fn-127 R3: on the palm's live run the conductor opened the gap loop on
//! the capability gate while seven owner-only requirements-unmet decisions
//! stood open; the gate's id sorted first. An open owner-only decision now
//! comes before the gap loop, the stages and tuning: the run pauses with its
//! handoff and dispatches nothing. No Jev call is made on this path.
//!
//! fn-129: a requirements-unmet field is the pipeline's to search again
//! first. The run pauses for it only once its two rounds are spent, and the
//! handoff names every source they tried.

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
    fn search(&self, _: &Config) -> Result<String, String> {
        panic!("the pipeline searched after its rounds were spent")
    }
}

/// Records each search the step asks for and reports one round.
struct Searching(std::sync::Mutex<usize>);

impl Executor for Searching {
    fn stage(&self, _: &Config, stage: &str) -> Result<StageOutcome, String> {
        panic!("stage {stage} ran before the search")
    }
    fn tune(
        &self,
        _: &Config,
        _: u64,
        _: &[String],
        _: &Path,
        _: Option<&Path>,
    ) -> Result<(), String> {
        panic!("tuning ran before the search")
    }
    fn search(&self, _: &Config) -> Result<String, String> {
        *self.0.lock().unwrap() += 1;
        Ok("search-again: ran: leaflet_length_m: nothing admitted (round 1 of 2)".into())
    }
}

const TRIED: [&str; 2] = ["https://example.test/one", "https://example.test/two"];

/// Records `count` empty rounds for `field` in the rounds file, each trying
/// one URL; other fields' rounds are kept.
fn spent_rounds(config: &Config, field: &str, id: &str, count: usize) {
    let rounds: Vec<serde_json::Value> = (0..count)
        .map(|n| json!({"round": n + 1, "decision": id, "gap": "no_mature_size", "query": "q",
            "hits": [], "tried": [{"id": "P9", "url": TRIED[n], "fields": [field], "class": "restricted"}],
            "admitted": [], "ledger": [], "at": "2026-09-23"}))
        .collect();
    let path = config.paths().search_rounds();
    let mut value = std::fs::read(&path)
        .ok()
        .and_then(|bytes| serde_json::from_slice(&bytes).ok())
        .unwrap_or_else(|| json!({"schema": "search-rounds", "schema_version": 1, "fields": {}}));
    value["fields"][field] = json!(rounds);
    std::fs::write(
        config.paths().search_rounds(),
        serde_json::to_vec(&value).unwrap(),
    )
    .unwrap();
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
        spent_rounds(&config, "leaflet_length_m", &owner.id, 2);
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
        if kind == "requirements-unmet" {
            assert_eq!(
                written["signals"]["sources_tried"][&owner.id],
                json!(TRIED),
                "the owner is handed the sources the rounds tried"
            );
        }
        // A second step on the same state observes the pause and adds nothing.
        let (again, _) = step::drive(&asker, &config, &mut run, &NoWork).unwrap();
        assert!(again.starts_with("paused"), "{kind}: {again}");
        assert!(run.dispatches.is_empty());
    }
}

#[test]
fn an_unmet_requirement_with_a_round_left_is_searched_again_before_the_owner_has_it() {
    let root = std::env::temp_dir().join(format!(
        "jev-owner-first-search-{}-{}",
        std::process::id(),
        telperion_jev::ledger::new_entry_id()
    ));
    let config = config(&root);
    let unmet = decision(
        "quality",
        "requirements-unmet",
        "leaflet_length_m",
        &["select"],
    );
    // A trait `select` found unstated is searched again like a field (R6).
    let trait_ = decision("select", "requirements-unmet", "bark_colour", &["generate"]);
    write_decisions(
        &config.paths().decisions(),
        &[unmet.clone(), trait_.clone()],
    )
    .unwrap();
    spent_rounds(&config, "leaflet_length_m", &unmet.id, 1);
    let mut run = Run::open(&config).unwrap();
    let asker = Asker {
        judge: Judge {
            transport: &NoJev,
            key: "k",
            ledger_dir: config.ledger_dir(),
        },
        config: &config,
    };
    let searching = Searching(std::sync::Mutex::new(0));
    let (word, _) = step::drive(&asker, &config, &mut run, &searching).unwrap();
    assert!(word.starts_with("search-again: ran"), "{word}");
    assert_eq!(*searching.0.lock().unwrap(), 1);
    assert!(run.pause.is_none() && run.dispatches.is_empty());
    // Both spent, the field and the trait go to the owner together.
    spent_rounds(&config, "leaflet_length_m", &unmet.id, 2);
    spent_rounds(&config, "bark_colour", &trait_.id, 2);
    let (word, _) = step::drive(&asker, &config, &mut run, &NoWork).unwrap();
    assert!(
        word.contains(&unmet.id) && word.contains(&trait_.id),
        "{word}"
    );
    // An open manifest proposal holds the search: the owner admits first.
    let root = root.join("proposal");
    let config = self::config(&root);
    let proposal = decision("discover", "manifest-proposed", "all", &["fetch"]);
    write_decisions(&config.paths().decisions(), &[proposal.clone(), unmet]).unwrap();
    let mut run = Run::open(&config).unwrap();
    let (word, _) = step::drive(&asker, &config, &mut run, &NoWork).unwrap();
    assert!(
        word.starts_with("paused") && word.contains(&proposal.id),
        "{word}"
    );
}
