//! fn-132 R4: on the palm's rerun after fn-130 and fn-131 the conductor's
//! next action was `search_again` while `fetch` to `verify` still carried
//! the old build in their keys; two empty rounds then went to the owner
//! before the stages had read P8's raw body. Stages whose key a code change
//! expired rerun first, once per build, before any search or owner pause.
//! No Jev call is made on this path.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

use serde_json::json;
use telperion_jev::caller::{HttpRequest, HttpResponse, Transport};
use telperion_jev::conductor::plan::{self, Next};
use telperion_jev::conductor::questions::Asker;
use telperion_jev::conductor::state::Run;
use telperion_jev::conductor::step::{self, Executor, StageOutcome};
use telperion_jev::conductor::{BudgetConfig, Config};
use telperion_jev::pipeline::decision::{write_decisions, Decision, DecisionParts};
use telperion_jev::pipeline::judge::Judge;
use telperion_jev::pipeline::stage::STAGES;

struct NoJev;

impl Transport for NoJev {
    fn send(&self, _: &HttpRequest) -> Result<HttpResponse, String> {
        Err("the stale stages ask Jev nothing".into())
    }
}

/// Records every stage and search the step asks for.
#[derive(Default)]
struct Recording {
    stages: Mutex<Vec<String>>,
    searches: Mutex<usize>,
}

impl Executor for Recording {
    fn stage(&self, _: &Config, stage: &str) -> Result<StageOutcome, String> {
        self.stages.lock().unwrap().push(stage.into());
        // Select stops on the field quality filed, as it does live.
        if stage == "select" {
            return Err("requirements-unmet".into());
        }
        Ok(StageOutcome::Ran)
    }
    fn tune(
        &self,
        _: &Config,
        _: u64,
        _: &[String],
        _: &Path,
        _: Option<&Path>,
    ) -> Result<(), String> {
        panic!("tuning ran with an open requirement")
    }
    fn search(&self, _: &Config) -> Result<String, String> {
        *self.searches.lock().unwrap() += 1;
        Ok("search-again: ran".into())
    }
}

/// The palm's folder as the live run left it: discover predates the build
/// id, fetch to verify carry an older build, the rest predate it too.
fn config(tag: &str) -> Config {
    let root = std::env::temp_dir().join(format!(
        "jev-stale-first-{tag}-{}-{}",
        std::process::id(),
        telperion_jev::ledger::new_entry_id()
    ));
    let dir = root.join("catalogue/date-palm");
    std::fs::create_dir_all(&dir).unwrap();
    let built = ["fetch", "extract", "screen", "quality", "select", "verify"];
    for stage in STAGES {
        let mut value = json!({"stage": stage, "tools": {}, "body": {}});
        if built.contains(&stage) {
            value["tools"]["species-pipeline-build"] = json!("d63b466237b8a53c134ec0b0");
        }
        std::fs::write(
            dir.join(format!("{stage}.json")),
            serde_json::to_vec(&value).unwrap(),
        )
        .unwrap();
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

fn unmet(config: &Config, rounds: usize) -> Decision {
    let decision = Decision::new(
        DecisionParts {
            species: "date-palm",
            stage: "quality",
            kind: "requirements-unmet",
            field: Some("height_m"),
            age_years: None,
        },
        &["select", "fit", "generate"],
        BTreeMap::new(),
        vec![],
        json!({}),
        &["add-sources"],
        "",
    );
    write_decisions(&config.paths().decisions(), std::slice::from_ref(&decision)).unwrap();
    let spent: Vec<serde_json::Value> = (0..rounds)
        .map(|n| json!({"round": n + 1, "decision": decision.id, "gap": "age_range_uncovered",
            "query": "q", "hits": [], "tried": [], "admitted": [], "ledger": [], "at": "2026-09-23"}))
        .collect();
    let file =
        json!({"schema": "search-rounds", "schema_version": 1, "fields": {"height_m": spent}});
    std::fs::write(
        config.paths().search_rounds(),
        serde_json::to_vec(&file).unwrap(),
    )
    .unwrap();
    decision
}

#[test]
fn stale_stages_rerun_before_the_search_and_the_owner_once_per_build() {
    // One round left: the search waits for the stages. None left: the
    // owner's pause waits for them.
    for (rounds, after) in [(1, "search"), (2, "owner")] {
        let config = config(after);
        let decision = unmet(&config, rounds);
        let mut run = Run::open(&config).unwrap();
        assert_eq!(
            plan::next(&config, &run).unwrap(),
            Next::Stages {
                from: "fetch".into()
            },
            "{after}"
        );
        let asker = Asker {
            judge: Judge {
                transport: &NoJev,
                key: "k",
                ledger_dir: config.ledger_dir(),
            },
            config: &config,
        };
        let executor = Recording::default();
        let (word, next) = step::drive(&asker, &config, &mut run, &executor).unwrap();
        assert!(word.starts_with("fetch: ran"), "{after}: {word}");
        assert_eq!(
            *executor.stages.lock().unwrap(),
            ["fetch", "extract", "screen", "quality", "select"],
            "{after}"
        );
        assert_eq!(*executor.searches.lock().unwrap(), 0, "{after}");
        // Tried once at this build, the stages stay stale where select
        // stopped: the run moves on, never back to them.
        match after {
            "search" => assert!(matches!(next, Next::SearchAgain { .. }), "{next:?}"),
            _ => assert_eq!(
                next,
                Next::OwnerFirst {
                    decisions: vec![decision.id.clone()]
                }
            ),
        }
        step::drive(&asker, &config, &mut run, &executor).unwrap();
        assert_eq!(executor.stages.lock().unwrap().len(), 5, "{after}");
        let reopened = Run::open(&config).unwrap();
        assert!(!matches!(
            plan::next(&config, &reopened).unwrap(),
            Next::Stages { .. }
        ));
    }
}
