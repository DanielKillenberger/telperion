//! The species conductor's test scaffolding: a scripted Jev transport, a
//! scripted executor, a staged species folder and the tuning results the
//! conductor tests stage. Nothing here reaches the network.
#![allow(dead_code)]
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

use serde_json::{json, Value};
use telperion_jev::caller::{HttpRequest, HttpResponse, Transport};
use telperion_jev::conductor::dispatch::DispatchResult;
use telperion_jev::conductor::plan::Next;
use telperion_jev::conductor::questions::Asker;
use telperion_jev::conductor::state::Run;
use telperion_jev::conductor::step::{self, Executor, StageOutcome};
use telperion_jev::conductor::{BudgetConfig, Config};
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
pub struct Script {
    pub choices: Mutex<BTreeMap<String, String>>,
}

impl Script {
    pub fn new() -> Self {
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
    pub fn set(&self, question: &str, answer: &str) {
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
pub struct Scripted {
    pub results: Mutex<Vec<EndResult>>,
    pub stage_stops: Mutex<BTreeMap<String, String>>,
}

impl Executor for Scripted {
    fn stage(&self, _config: &Config, stage: &str) -> Result<StageOutcome, String> {
        if let Some(stop) = self.stage_stops.lock().unwrap().get(stage) {
            return Err(stop.clone());
        }
        Ok(StageOutcome::Ran)
    }
    fn search(&self, _config: &Config) -> Result<String, String> {
        Err("the scripted executor runs no search".into())
    }
    fn tune(
        &self,
        _config: &Config,
        _revision: u64,
        _focus: &[String],
        out: &Path,
        _resume: Option<&Path>,
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

pub fn scratch(tag: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "jev-conductor-{tag}-{}-{}",
        std::process::id(),
        telperion_jev::ledger::new_entry_id()
    ));
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

pub fn write(path: &Path, value: &Value) {
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    std::fs::write(path, serde_json::to_vec_pretty(value).unwrap()).unwrap();
}

pub fn config(root: &Path) -> Config {
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
            max_tokens: Some(200_000),
            attempt_max_tokens: Some(40_000),
            max_dispatches: Some(8),
            max_tuning_revisions: Some(3),
        },
        judgment_model: "jev-latest".into(),
        continuation_validated: true,
    }
}

pub fn attempt(dial: &str) -> Attempt {
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
        moves: vec![],
        adopted: false,
        stood: None,
        rolled_back: None,
    }
}

pub fn gap(id: &str, priority: &str, status: &str, existing: Option<&str>) -> GapEntry {
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

pub fn result(root: &Path, identity: &str, machine_ready: bool, gaps: Vec<GapEntry>) -> EndResult {
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

pub fn verified(identity: &str, revision: Option<&str>, handoff: Option<&Path>) -> DispatchResult {
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
        outcome: None,
        finished_at: String::new(),
    }
}

pub fn drive(
    script: &Script,
    config: &Config,
    run: &mut Run,
    executor: &dyn Executor,
) -> (String, Next) {
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
