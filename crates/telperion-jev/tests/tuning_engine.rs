use serde_json::json;
use telperion_jev::tuning::{
    actions::{Action, Dial},
    continuation::{Assessment, Basis},
    engine::{Answer, Proposal, Run, Services},
    evaluation::Trial,
    handoff::PriorityRoute,
    state::{Budget, Cell, CellStatus, Visual},
};

struct Mock {
    evaluations: u64,
    routes: u64,
    visuals: u64,
    capability: bool,
    /// Per approved priority, in order: the raw choice and its confidence.
    route_plan: Vec<(String, f64)>,
    /// Continuation verdict per call, consumed in order; empty means supported.
    continuations: Vec<bool>,
}

fn mock() -> Mock {
    Mock {
        evaluations: 0,
        routes: 0,
        visuals: 0,
        capability: false,
        route_plan: vec![],
        continuations: vec![],
    }
}
impl Services for Mock {
    fn priority_references(&self) -> Vec<telperion_jev::tuning::evaluation::Image> {
        vec![priority_image("whole"), priority_image("bark")]
    }
    fn priority_evidence(
        &self,
        _: &Trial,
        visual: &Visual,
    ) -> Result<Vec<telperion_jev::tuning::priority::Evidence>, String> {
        telperion_jev::tuning::priority::evidence(
            visual,
            &[priority_image("whole"), priority_image("bark")],
            &self.priority_references(),
            &[],
        )
    }
    fn visual_for(
        &mut self,
        trial: &Trial,
        required: &[Cell],
        _: Option<&telperion_jev::tuning::priority::Approval>,
    ) -> Result<Answer<Visual>, String> {
        let mut answer = self.visual(trial)?;
        let status = answer.value.cells[0].1;
        answer.value.cells = required.iter().cloned().map(|c| (c, status)).collect();
        Ok(answer)
    }
    fn evaluation_images(&self) -> u64 {
        4
    }
    fn visual_images(&self, _: &Trial) -> u64 {
        0
    }
    fn evaluate(
        &mut self,
        overrides: serde_json::Value,
        round: u64,
        label: &str,
        ledger: Option<String>,
    ) -> Trial {
        self.evaluations += 1;
        Trial {
            key: format!("candidate{}", self.evaluations),
            identity: "input1".into(),
            seed: 1,
            round,
            label: label.into(),
            overrides,
            ledger,
            feasible: true,
            reason: None,
            measurement: json!({}),
            comparisons: vec![],
            score: Some(1. / self.evaluations as f64),
            seconds: 0.,
        }
    }
    fn visual(&mut self, trial: &Trial) -> Result<Answer<Visual>, String> {
        self.visuals += 1;
        Ok(Answer {
            tokens: Some(100),
            value: Visual {
                identity: trial.key.clone(),
                model: "mock".into(),
                ledger: "visual:1".into(),
                observations: vec![],
                findings: vec![],
                joint: None,
                cells: vec![(
                    cell(),
                    if self.visuals >= 3 {
                        CellStatus::Pass
                    } else {
                        CellStatus::Fail
                    },
                )],
                defects: if self.visuals < 3 {
                    vec!["visible defect".into()]
                } else {
                    vec![]
                },
            },
        })
    }
    fn continuation(&mut self, basis: &Basis) -> Result<Answer<Assessment>, String> {
        let supported = if self.continuations.is_empty() {
            true
        } else {
            self.continuations.remove(0)
        };
        Ok(Answer {
            tokens: Some(20),
            value: Assessment {
                identity: basis.identity.clone(),
                ledger: "jev:1".into(),
                tractability: if supported {
                    "supported".into()
                } else {
                    "insufficient_evidence".into()
                },
                progress: "supported".into(),
                risk: "bounded".into(),
            },
        })
    }
    fn propose(&mut self, _: &Run) -> Result<Answer<Vec<Proposal>>, String> {
        Ok(Answer {
            tokens: Some(20),
            value: vec![Proposal {
                dial: "crookedness".into(),
                action: Action::SmallIncrease,
                ledger: "jev:2".into(),
            }],
        })
    }
    fn route(&mut self, state: &Run) -> Result<Answer<Vec<PriorityRoute>>, String> {
        self.routes += 1;
        let fallback = if self.capability {
            "new_capability"
        } else {
            "tuning"
        };
        let ordered = state
            .approved_priorities()
            .map(|a| a.ordered.clone())
            .filter(|o| !o.is_empty());
        let value = match ordered {
            None => vec![PriorityRoute {
                gap_id: None,
                rank: 1,
                route: fallback.into(),
                raw_choice: Some(fallback.into()),
                confidence: Some(0.9),
                threshold: 0.5,
                ledger: "jev:route".into(),
            }],
            Some(ordered) => ordered
                .iter()
                .enumerate()
                .map(|(i, gap)| {
                    let (raw, confidence) = self
                        .route_plan
                        .get(i)
                        .cloned()
                        .unwrap_or_else(|| (fallback.to_string(), 0.9));
                    PriorityRoute {
                        gap_id: Some(gap.id.clone()),
                        rank: i + 1,
                        route: if confidence >= 0.5 {
                            raw.clone()
                        } else {
                            "insufficient_evidence".into()
                        },
                        raw_choice: Some(raw),
                        confidence: Some(confidence),
                        threshold: 0.5,
                        ledger: "jev:route".into(),
                    }
                })
                .collect(),
        };
        Ok(Answer {
            tokens: Some(20),
            value,
        })
    }
}

fn priority_image(view: &str) -> telperion_jev::tuning::evaluation::Image {
    static PATH: std::sync::OnceLock<std::path::PathBuf> = std::sync::OnceLock::new();
    let path = PATH.get_or_init(|| {
        let p = std::env::temp_dir().join(telperion_jev::ledger::new_entry_id());
        std::fs::write(&p, b"priority fixture").unwrap();
        p
    });
    telperion_jev::tuning::evaluation::Image {
        path: path.clone(),
        sha256: telperion_jev::sha256_hex(b"priority fixture"),
        view: view.into(),
        seed: 1,
    }
}
fn approve_priorities(state: &mut Run, mock: &Mock, ordered: serde_json::Value) {
    let checkpoint = state.priority_checkpoints.last().unwrap();
    let pause = state.pause.as_ref().unwrap();
    let decision:telperion_jev::tuning::continuation::HumanDecision=serde_json::from_value(json!({"pause_id":pause.id,"identity":state.identity,"action":pause.basis.proposed_action,"by":"explicit test owner","rationale":"reviewed ranking fixture","preserve_evidence":true,"priority_approval":{"checkpoint_sha256":checkpoint.hash(),"scope_sha256":checkpoint.scope_sha256,"ordered":ordered}})).unwrap();
    pause.resume(&decision).unwrap();
    state
        .accept_priorities(&decision, &mock.priority_scope(state))
        .unwrap();
    state.authorizations.push(decision);
    state.pause = None;
}
fn cell() -> Cell {
    Cell {
        item: "crown".into(),
        view: "whole".into(),
        seed: 1,
    }
}
fn run() -> Run {
    let effective = telperion_core::params::metadata(
        &telperion_core::presets::Preset::from_id("european-beech")
            .unwrap()
            .parameters(),
    );
    Run {
        identity: "input1".into(),
        preset: "european-beech".into(),
        seed: 1,
        effective,
        overrides: json!({}),
        dials: vec![Dial {
            id: "crookedness".into(),
            path: "/skeleton/habit/crookedness".into(),
            meaning: "turn variation".into(),
            min: 0.,
            max: 15.,
            small: 1.,
            substantial: 3.,
            integer: false,
        }],
        owner_notes: "irregular outline".into(),
        required: vec![cell()],
        budget: Budget {
            visual_passes: Some(0),
            max_visual_passes: Some(5),
            evaluations: 0,
            images: 0,
            tokens: 0,
            rounds: 0,
            max_evaluations: 13,
            max_images: 52,
            max_tokens: 150000,
            max_rounds: 3,
        },
        usage_known: true,
        trials: vec![],
        current: None,
        visual: None,
        pause: None,
        machine_ready: false,
        pending: None,
        routes: vec![],
        authorizations: vec![],
        preparation_charge: None,
        priority_checkpoints: vec![],
        handoffs: vec![],
        judgment_inputs: vec![],
    }
}

#[test]
fn joint_observations_and_findings_survive_state_and_judgment_projection() {
    let mut state = run();
    state.visual=Some(serde_json::from_value(json!({"identity":"candidate","model":"mock","ledger":"receipt","cells":[],"defects":[],"observations":["joint visible mismatch"],"findings":[{"observation":"cross-view constraint","evidence_ids":["render-0","reference-0"],"impact":"blocker","uncertain":false,"causal_hypothesis":"unproven mechanism"}]})).unwrap());
    let saved = serde_json::to_vec(&state).unwrap();
    let restored: Run = serde_json::from_slice(&saved).unwrap();
    let summary = telperion_jev::tuning::judgments::summary(&restored);
    assert_eq!(
        summary["visual"]["observations"][0],
        "joint visible mismatch"
    );
    assert_eq!(
        summary["visual"]["findings"][0]["causal_hypothesis"],
        "unproven mechanism"
    );
}

#[test]
fn initial_pass_still_pauses_and_owner_goals_require_scoped_fresh_coverage() {
    let mut state = run();
    state.required = vec![
        cell(),
        Cell {
            item: "material".into(),
            view: "bark".into(),
            seed: 1,
        },
        Cell {
            item: "character".into(),
            view: "whole".into(),
            seed: 42,
        },
        Cell {
            item: "material".into(),
            view: "bark".into(),
            seed: 42,
        },
    ];
    let mut mock = Mock {
        visuals: 2,
        ..mock()
    };
    let mut snapshots = vec![];
    state
        .execute(&mut mock, &mut |s| {
            snapshots.push(s.machine_ready);
            Ok(())
        })
        .unwrap();
    assert!(snapshots.iter().all(|ready| !*ready));
    assert_eq!(mock.routes, 0);
    assert_eq!(mock.evaluations, 1);
    assert_eq!(mock.visuals, 3);
    assert!(state
        .visual
        .as_ref()
        .unwrap()
        .cells
        .iter()
        .all(|(_, s)| *s == CellStatus::Pass));
    let spent = serde_json::to_value(&state.budget).unwrap();
    assert!(state.execute(&mut mock, &mut |_| Ok(())).is_err());
    assert_eq!(serde_json::to_value(&state.budget).unwrap(), spent);
    approve_priorities(
        &mut state,
        &mock,
        json!([
            {"id":"owner-leafy","observation":"Defining leafy form","evidence_ids":["render-0","reference-0"],"views":["whole"]},
            {"id":"owner-material","observation":"Defining material","evidence_ids":["render-1","reference-1"],"views":["bark"]}
        ]),
    );
    assert_eq!(serde_json::to_value(&state.budget).unwrap(), spent);
    assert!(!state.machine_ready);
    let required = state.required_cells();
    assert_eq!(required.len(), 8);
    assert!(!required
        .iter()
        .any(|c| c.item.contains("owner-leafy") && c.view == "bark"));
    assert!(state
        .round_basis(&mock)
        .unwrap()
        .evidence
        .join(" ")
        .contains("owner-leafy"));
    state.execute(&mut mock, &mut |_| Ok(())).unwrap();
    assert!(state.machine_ready);
    assert_eq!(mock.visuals, 4);
    assert_eq!(mock.evaluations, 1);
    assert_eq!(mock.routes, 0);
    let spent = state.budget.tokens;
    state.owner_notes = "changed objectives".into();
    state.execute(&mut mock, &mut |_| Ok(())).unwrap();
    assert!(!state.machine_ready);
    assert!(state.pause.is_some());
    assert_eq!(state.priority_checkpoints.len(), 2);
    assert_eq!(state.budget.tokens, spent);
}

#[test]
fn numeric_wins_keep_refining_until_visual_cells_pass() {
    let mut state = run();
    let mut mock = mock();
    let mut checkpoints = vec![];
    state
        .execute(&mut mock, &mut |s| {
            checkpoints.push(serde_json::to_string(s).unwrap());
            Ok(())
        })
        .unwrap();
    assert!(state.pause.as_ref().unwrap().reason.contains("priority"));
    assert_eq!(mock.routes, 0);
    approve_priorities(&mut state, &mock, json!([]));
    state
        .execute(&mut mock, &mut |s| {
            checkpoints.push(serde_json::to_string(s).unwrap());
            Ok(())
        })
        .unwrap();
    assert!(state.machine_ready, "{:?}", state.pause);
    assert_eq!(mock.evaluations, 3);
    assert_eq!(mock.visuals, 3);
    assert_eq!(mock.routes, 2);
    assert_eq!(state.finalists().len(), 3);
    assert!(checkpoints
        .iter()
        .any(|s| s.contains("candidate evaluation")));
    assert!(state.pause.is_none());
}

#[test]
fn baseline_capability_defect_routes_before_spending_tuning_evaluations() {
    let mut state = run();
    let mut mock = Mock {
        capability: true,
        ..mock()
    };
    state.execute(&mut mock, &mut |_| Ok(())).unwrap();
    approve_priorities(&mut state, &mock, json!([]));
    state.execute(&mut mock, &mut |_| Ok(())).unwrap();
    assert_eq!(mock.evaluations, 1);
    assert!(state
        .pause
        .as_ref()
        .unwrap()
        .reason
        .contains("fn-89 handoff"));
    let before = state.budget.tokens;
    assert!(state.execute(&mut mock, &mut |_| Ok(())).is_err());
    assert_eq!(state.budget.tokens, before);
}

#[test]
fn bounded_plan_and_round_limit_are_checked_before_paid_routing() {
    let mut state = run();
    let mut mock = mock();
    state.budget.max_rounds = 0;
    state.execute(&mut mock, &mut |_| Ok(())).unwrap();
    approve_priorities(&mut state, &mock, json!([]));
    state.execute(&mut mock, &mut |_| Ok(())).unwrap();
    assert_eq!(mock.routes, 0);
    assert!(state.pause.as_ref().unwrap().reason.contains("round limit"));
    let basis = state.round_basis(&mock).unwrap();
    assert!(basis.proposed_action.contains("max four single-dial"));
    assert!(basis.proposed_action.contains("BEFORE render"));
    assert!(basis.proposed_action.contains("current"));
    assert_eq!(basis.next_tokens, Some(29000));
    state.pause = None;
    state.budget.max_rounds = 1;
    state.budget.max_tokens = state.budget.tokens + 100;
    state.execute(&mut mock, &mut |_| Ok(())).unwrap();
    assert_eq!(mock.routes, 0);
    assert!(state
        .pause
        .as_ref()
        .unwrap()
        .reason
        .contains("preflight cannot fit"));
}

#[test]
fn stale_finalists_are_excluded_and_resource_history_is_revision_tagged() {
    let mut state = run();
    let mut mock = mock();
    let mut old = mock.evaluate(json!({}), 0, "old", None);
    old.identity = "old-revision".into();
    old.score = Some(0.001);
    let mut current = mock.evaluate(json!({}), 0, "new", None);
    current.measurement =
        json!({"metrics":{"nodes":{"value":187968},"growth":{"node_capped":false}}});
    state.trials = vec![old, current];
    state.current = Some(1);
    assert_eq!(state.finalists().len(), 1);
    assert_eq!(state.finalists()[0].label, "new");
    let summary = telperion_jev::tuning::judgments::summary(&state);
    assert_eq!(summary["recent_attempts"][1]["current_revision"], false);
    assert_eq!(summary["resource_limit"]["current_nodes"], 187968);
    state.effective["skeleton"]["growth"]["maxNodes"] = json!(1000000);
    let decision = serde_json::from_value(json!({
        "identity":"old-revision", "pause_id":"paused", "action":"reassess",
        "by":"owner", "rationale":"Prior pilot hit hidden node ceiling; explicit new caller limit",
        "next_identity":"input1", "baseline_amendment":{"previous":{},"next":{"skeleton":{"growth":{"maxNodes":1000000}}}}
    })).unwrap();
    state.authorizations.push(decision);
    let basis = state.round_basis(&mock).unwrap();
    let evidence = basis.evidence.join(" ");
    assert!(evidence.contains("812032"));
    assert!(evidence.contains("1000000"));
    assert!(evidence.contains("187968"));
    assert!(evidence.contains("hidden node ceiling"));
    assert!(evidence.contains("old-revision"));
    assert!(basis.recent_outcomes[1].contains("\"current_revision\":false"));
    assert!(state.pilot_authority().unwrap_err().contains("unvalidated"));
    state.budget.visual_passes = Some(5);
    state.execute(&mut mock, &mut |_| Ok(())).unwrap();
    assert_eq!(mock.visuals, 0);
}

#[test]
fn attributed_diagnosis_projects_to_both_judgments_and_rechecks_sources() {
    let source = std::env::temp_dir().join(format!(
        "diagnosis-{}",
        telperion_jev::ledger::new_entry_id()
    ));
    std::fs::write(&source, "source observation").unwrap();
    let mut state = run();
    let mut mock = mock();
    state
        .trials
        .push(mock.evaluate(json!({}), 0, "baseline", None));
    state.current = Some(0);
    let legacy = json!({"pause_id":"pause","identity":"input1","action":"diagnose","by":"owner","rationale":"bounded inspection"});
    let mut decision: telperion_jev::tuning::continuation::HumanDecision =
        serde_json::from_value(legacy).unwrap();
    assert!(decision.diagnosis.is_none());
    decision.diagnosis=Some(serde_json::from_value(json!({"target_identity":"input1","author":"worker","model":"reasoner","findings":[{"claim":"Interpretation, not owner ruling","source":source,"sha256":telperion_jev::sha256_hex(b"source observation"),"excerpt":"observation"}]})).unwrap());
    state.authorizations.push(decision);
    state.verify_diagnoses().unwrap();
    let summary = telperion_jev::tuning::judgments::summary(&state);
    assert_eq!(
        summary["agent_diagnoses"]["attachments"][0]["author"],
        "worker"
    );
    assert!(state
        .round_basis(&mock)
        .unwrap()
        .evidence
        .join(" ")
        .contains("Interpretation, not owner ruling"));
    std::fs::write(&source, "changed source").unwrap();
    assert!(state.verify_diagnoses().is_err());
    let old_budget = serde_json::to_value(&state.budget).unwrap();
    state.execute(&mut mock, &mut |_| Ok(())).unwrap();
    assert_eq!(serde_json::to_value(&state.budget).unwrap(), old_budget);
    assert_eq!(mock.routes, 0);
    assert_eq!(mock.visuals, 0);
    assert!(state.pause.as_ref().unwrap().reason.contains("diagnosis"));
    std::fs::write(&source, "source observation").unwrap();
    state.pause = None;
    state
        .execute(&mut mock, &mut |s| {
            if s.pending.is_some() {
                std::fs::write(&source, "changed during checkpoint").unwrap();
            }
            Ok(())
        })
        .unwrap();
    assert_eq!(mock.routes, 0);
    assert_eq!(mock.visuals, 0);
    assert!(state.pending.is_some());
    assert!(state.budget.tokens > old_budget["tokens"].as_u64().unwrap());
    state.identity = "new-identity".into();
    state.verify_diagnoses().unwrap();
    assert!(
        telperion_jev::tuning::judgments::summary(&state)["agent_diagnoses"]["attachments"]
            .as_array()
            .unwrap()
            .is_empty()
    );
    assert_eq!(state.authorizations.len(), 1);
    std::fs::remove_file(source).unwrap();
}
