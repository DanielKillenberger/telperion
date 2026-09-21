use serde_json::json;
use telperion_jev::tuning::{
    actions::{Action, Dial},
    continuation::Basis,
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
    /// Owner-priority cells for these gap ids are reported as still failing.
    fail_owner_gaps: Vec<String>,
    /// Every candidate scores the same, so no round can improve on the baseline.
    stall: bool,
    /// Continuation calls whose basis names an fn-89 handoff.
    pre_dispatch_calls: u64,
    /// Evidence-difference questions asked.
    evidence_calls: u64,
    evidence_answer: String,
    /// The adjustment the router proposes; a test changes it to offer a move
    /// the repeat filter has not already seen.
    proposal_action: Action,
    /// Overrides what `propose` returns; empty means the default single move.
    proposals: Vec<Proposal>,
}

fn mock() -> Mock {
    Mock {
        evaluations: 0,
        routes: 0,
        visuals: 0,
        capability: false,
        route_plan: vec![],
        continuations: vec![],
        fail_owner_gaps: vec![],
        stall: false,
        pre_dispatch_calls: 0,
        evidence_calls: 0,
        evidence_answer: "different".into(),
        proposal_action: Action::SmallIncrease,
        proposals: vec![Proposal {
            dial: "crookedness".into(),
            action: Action::SmallIncrease,
            ledger: "jev:2".into(),
            direction_mass: Some(0.9),
            rule: Some(telperion_jev::tuning::direction::RULE.into()),
        }],
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
        answer.value.cells = required
            .iter()
            .cloned()
            .map(|c| {
                let open = self
                    .fail_owner_gaps
                    .iter()
                    .any(|id| c.item.starts_with(&format!("owner-priority:{id}: ")));
                let status = if open { CellStatus::Fail } else { status };
                (c, status)
            })
            .collect();
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
            score: Some(if self.stall {
                1.
            } else {
                1. / self.evaluations as f64
            }),
            seconds: 0.,
            base: None,
            action: None,
            evidence: None,
            direction_mass: None,
            rule: None,
        }
    }
    fn visual(&mut self, trial: &Trial) -> Result<Answer<Visual>, String> {
        self.visuals += 1;
        Ok(Answer {
            ledger: Some("visual:1".into()),
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
    fn risk(&mut self, basis: &Basis) -> Result<Answer<String>, String> {
        if basis
            .proposed_action
            .starts_with("fn-89 handoff for owner priority")
        {
            self.pre_dispatch_calls += 1;
        }
        let bounded = if self.continuations.is_empty() {
            true
        } else {
            self.continuations.remove(0)
        };
        Ok(Answer {
            ledger: Some("jev:risk".into()),
            tokens: Some(20),
            value: if bounded {
                "bounded".into()
            } else {
                "insufficient_evidence".into()
            },
        })
    }
    fn evidence(&mut self, _: &serde_json::Value) -> Result<Answer<String>, String> {
        self.evidence_calls += 1;
        Ok(Answer {
            ledger: Some("jev:evidence".into()),
            tokens: Some(20),
            value: self.evidence_answer.clone(),
        })
    }
    fn propose(&mut self, _: &Run) -> Result<Answer<Vec<Proposal>>, String> {
        let value = self
            .proposals
            .iter()
            .cloned()
            .map(|mut p| {
                p.action = self.proposal_action;
                p
            })
            .collect();
        Ok(Answer {
            tokens: Some(20),
            value,
            ledger: Some("jev:propose".into()),
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
            ledger: Some("jev:route".into()),
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
        visual_bootstrap: false,
        reviewer_passed_unqualified: false,
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

#[test]
fn mixed_routes_tune_and_hand_off_without_claiming_readiness() {
    let mut state = run();
    state.budget.max_rounds = 1;
    let mut mock = Mock {
        route_plan: vec![
            ("tuning".into(), 0.9),
            ("existing:fn-77".into(), 0.9),
            ("appearance".into(), 0.31),
        ],
        fail_owner_gaps: vec!["owner-hanging".into(), "owner-materials".into()],
        ..mock()
    };
    state.execute(&mut mock, &mut |_| Ok(())).unwrap();
    approve_priorities(
        &mut state,
        &mock,
        json!([
            {"id":"owner-crown","observation":"Crown shape and foliage organization","evidence_ids":["render-0","reference-0"],"views":["whole"]},
            {"id":"owner-hanging","observation":"Hanging outer foliage","evidence_ids":["render-0","reference-0"],"views":["whole"]},
            {"id":"owner-materials","observation":"Materials, including bark and foliage","evidence_ids":["render-0","reference-0"],"views":["whole"]}
        ]),
    );
    let spent_before = state.budget.tokens;
    state.execute(&mut mock, &mut |_| Ok(())).unwrap();

    // One routing call covered all three priorities, and tuning still ran.
    assert_eq!(mock.routes, 1);
    assert!(mock.evaluations > 1, "the tuning round did not run");
    assert_eq!(
        state.handoffs.len(),
        2,
        "one handoff per non-tuning priority"
    );

    let grounded = state
        .handoffs
        .iter()
        .find(|h| h.gap_id.as_deref() == Some("owner-hanging"))
        .unwrap();
    assert_eq!(grounded.route, "existing:fn-77");
    assert_eq!(grounded.existing_spec.as_deref(), Some("fn-77"));
    assert_eq!(grounded.rank, 2);
    assert_eq!(grounded.priority, "Hanging outer foliage");
    assert!(grounded.dispatch_authorized, "grounded route was assessed");
    assert_eq!(grounded.raw_choice.as_deref(), Some("existing:fn-77"));
    assert_eq!(grounded.proposed_spending, None);
    assert!(grounded
        .observed_defect
        .evidence
        .iter()
        .any(|e| e.role == "render"));
    assert!(grounded
        .observed_defect
        .evidence
        .iter()
        .any(|e| e.role == "reference"));

    // The below-threshold priority is an uncertainty handoff, never a route.
    let uncertain = state
        .handoffs
        .iter()
        .find(|h| h.gap_id.as_deref() == Some("owner-materials"))
        .unwrap();
    assert_eq!(uncertain.route, "insufficient_evidence");
    assert_eq!(uncertain.raw_choice.as_deref(), Some("appearance"));
    assert_eq!(uncertain.confidence, Some(0.31));
    assert!(!uncertain.dispatch_authorized);
    assert_eq!(uncertain.existing_spec, None);
    assert_eq!(
        uncertain.proposed_investigation,
        "uncertainty handoff: no supported diagnosis"
    );

    // Dials are reported as seen by the router, never as exhausted.
    assert_eq!(
        grounded.dials_in_router_state,
        vec!["crookedness".to_string()]
    );
    assert!(grounded
        .dials_note
        .contains("Not evidence that any was tried"));
    assert!(grounded
        .attempts
        .iter()
        .all(|a| a.dial == "crookedness" && a.score_after.is_some()));

    // Outstanding gaps are never machine readiness, and both stay unresolved.
    assert!(!state.machine_ready);
    let unresolved = state.unresolved_priorities();
    assert!(unresolved.contains(&"owner-hanging".to_string()));
    assert!(unresolved.contains(&"owner-materials".to_string()));

    // Every judgment recorded the exact value it transmitted, before dispatch.
    let labels = state
        .judgment_inputs
        .iter()
        .map(|i| i.label.as_str())
        .collect::<Vec<_>>();
    assert!(labels.contains(&"defect routing"));
    assert!(labels.contains(&"pre-dispatch risk"));
    assert!(labels.contains(&"targeted proposals"));
    for input in &state.judgment_inputs {
        assert_eq!(
            input.state_sha256,
            telperion_jev::sha256_hex(&serde_json::to_vec(&input.state).unwrap())
        );
    }
    assert!(state.budget.tokens > spent_before, "spend was not recorded");

    // A repeated round replaces this revision's handoffs instead of piling up.
    state.budget.max_rounds = 2;
    state.pause = None;
    state.execute(&mut mock, &mut |_| Ok(())).unwrap();
    assert_eq!(state.handoffs.len(), 2);
    assert!(!state.machine_ready);
}

#[test]
fn a_handoffs_owner_cell_not_passing_keeps_the_run_unready() {
    let mut state = run();
    let handoff: telperion_jev::tuning::handoff::Handoff = serde_json::from_value(json!({
        "run_identity":"input1","candidate_key":"candidate1","checkpoint_sha256":"c",
        "gap_id":"owner-hanging","rank":1,"priority":"Hanging outer foliage",
        "observed_defect":{"observation":"missing hanging foliage","reviewer_model":"mock",
            "visual_ledger":"visual:1","evidence":[]},
        "route":"new_capability","existing_spec":null,"judgment_ledger":"jev:route",
        "raw_choice":"new_capability","confidence":0.9,"threshold":0.5,
        "dispatch_authorized":true,"dials_in_router_state":["crookedness"],
        "dials_note":"note","attempts":[],"observations":[],"hypotheses":[],
        "hypotheses_note":"note","unknowns":[],"proposed_investigation":"investigate",
        "proposed_spending":null,"proposed_spending_note":"host owns repair estimate"}))
    .unwrap();
    state.handoffs.push(handoff);

    // The owner cell for a handed-off priority is part of required_cells, so the
    // ordinary cell check already withholds readiness. No extra guard is needed.
    let owner = Cell {
        item: "owner-priority:owner-hanging: Hanging outer foliage".into(),
        view: "whole".into(),
        seed: 1,
    };
    let required = vec![cell(), owner.clone()];
    let mut visual: Visual = serde_json::from_value(
        json!({"identity":"candidate1","model":"mock","ledger":"visual:1","cells":[],"defects":[],"findings":[]}),
    )
    .unwrap();
    visual.cells = vec![
        (cell(), CellStatus::Pass),
        (owner.clone(), CellStatus::Fail),
    ];
    assert!(!telperion_jev::tuning::state::ready(
        &required,
        "candidate1",
        &visual
    ));
    assert!(state
        .unresolved_priorities()
        .contains(&"owner-hanging".to_string()));

    // Resolving that cell clears both.
    visual.cells = vec![(cell(), CellStatus::Pass), (owner, CellStatus::Pass)];
    assert!(telperion_jev::tuning::state::ready(
        &required,
        "candidate1",
        &visual
    ));
    state.handoffs.clear();
    assert!(state.unresolved_priorities().is_empty());
}

#[test]
fn re_routing_an_unchanged_candidate_reuses_its_pre_dispatch_judgment() {
    let mut state = run();
    state.budget.max_rounds = 2;
    let mut mock = Mock {
        route_plan: vec![
            ("tuning".into(), 0.9),
            ("existing:fn-77".into(), 0.9),
            ("appearance".into(), 0.31),
        ],
        stall: true,
        ..mock()
    };
    state.execute(&mut mock, &mut |_| Ok(())).unwrap();
    approve_priorities(
        &mut state,
        &mock,
        json!([
            {"id":"owner-crown","observation":"Crown shape and foliage organization","evidence_ids":["render-0","reference-0"],"views":["whole"]},
            {"id":"owner-hanging","observation":"Hanging outer foliage","evidence_ids":["render-0","reference-0"],"views":["whole"]},
            {"id":"owner-materials","observation":"Materials, including bark and foliage","evidence_ids":["render-0","reference-0"],"views":["whole"]}
        ]),
    );
    state.execute(&mut mock, &mut |_| Ok(())).unwrap();

    // Every round stalls numerically, so the candidate never changes and the
    // router runs again each time.
    assert!(mock.routes >= 2, "expected a re-route, got {}", mock.routes);
    assert_eq!(
        mock.pre_dispatch_calls, 1,
        "the grounded handoff was judged more than once"
    );
    assert_eq!(state.handoffs.len(), 2);

    // The basis says what is proposed, not just the bare route name.
    let grounded = state
        .handoffs
        .iter()
        .find(|h| h.gap_id.as_deref() == Some("owner-hanging"))
        .unwrap();
    assert!(grounded.dispatch_authorized);
    let basis = state
        .judgment_inputs
        .iter()
        .find(|i| i.label == "pre-dispatch risk")
        .unwrap();
    let action = basis.state["proposed_action"].as_str().unwrap();
    assert!(
        action.contains("owner priority 2 (owner-hanging)"),
        "{action}"
    );
    assert!(action.contains("Hanging outer foliage"), "{action}");
    assert!(action.contains("Route: existing:fn-77"), "{action}");
    assert!(
        action.contains("No repair is dispatched by this run"),
        "{action}"
    );

    // A changed candidate is a new question, so the judgment runs again.
    state
        .trials
        .push(mock.evaluate(json!({}), 9, "crookedness", None));
    state.current = Some(state.trials.len() - 1);
    state.pause = None;
    state.budget.max_rounds = 3;
    state.execute(&mut mock, &mut |_| Ok(())).unwrap();
    assert_eq!(mock.pre_dispatch_calls, 2);
}

/// Drives one approved-priority run to the point where rounds begin.
fn ready_to_round(mock: &mut Mock) -> Run {
    let mut state = run();
    state.execute(mock, &mut |_| Ok(())).unwrap();
    approve_priorities(&mut state, mock, json!([]));
    state
}

#[test]
fn a_first_round_against_a_candidate_buys_no_continuation_judgment() {
    let mut mock = mock();
    let mut state = ready_to_round(&mut mock);
    assert_eq!(
        state.round_decision(),
        telperion_jev::tuning::round::Decision::Proceed
    );
    state.execute(&mut mock, &mut |_| Ok(())).unwrap();
    // The round ran, and nothing was asked to justify starting it.
    assert!(mock.evaluations > 1, "the round did not run");
    assert_eq!(mock.evidence_calls, 0, "a first attempt bought a judgment");
    assert!(!state
        .judgment_inputs
        .iter()
        .any(|i| i.label.contains("continuation")));
}

#[test]
fn a_stall_without_new_evidence_pauses_without_asking_anything() {
    let mut mock = Mock {
        stall: true,
        ..mock()
    };
    let mut state = ready_to_round(&mut mock);
    state.budget.max_rounds = 3;
    state.execute(&mut mock, &mut |_| Ok(())).unwrap();
    assert_eq!(
        mock.evidence_calls, 0,
        "a stall on old evidence paid for a question"
    );
    let reason = &state.pause.as_ref().unwrap().reason;
    assert!(
        reason.contains("numeric stall without new evidence"),
        "{reason}"
    );
}

#[test]
fn a_stall_with_new_evidence_asks_exactly_one_question_and_obeys_it() {
    for (answer, proceeds) in [
        ("different", true),
        ("same", false),
        ("insufficient_evidence", false),
    ] {
        let mut mock = Mock {
            stall: true,
            evidence_answer: answer.into(),
            ..mock()
        };
        let mut state = ready_to_round(&mut mock);
        state.budget.max_rounds = 3;
        state.execute(&mut mock, &mut |_| Ok(())).unwrap();
        // New evidence arrives: a fresh assessment the last attempt never saw.
        let mut fresh = state.visual.clone().unwrap();
        fresh.ledger = "visual:fresh-evidence".into();
        state.visual = Some(fresh);
        state.pause = None;
        // A genuinely different move, so the repeat filter is not what decides.
        mock.proposal_action = Action::SubstantialIncrease;
        let before = mock.evaluations;
        state.execute(&mut mock, &mut |_| Ok(())).unwrap();
        assert_eq!(
            mock.evidence_calls, 1,
            "{answer}: expected exactly one question"
        );
        if proceeds {
            assert!(mock.evaluations > before, "{answer}: the round did not run");
        } else {
            assert_eq!(
                mock.evaluations, before,
                "{answer}: a refused round still ran"
            );
            let reason = &state.pause.as_ref().unwrap().reason;
            assert!(
                reason.contains("repeat attempt unjustified"),
                "{answer}: {reason}"
            );
            assert!(reason.contains("uncalibrated"), "{answer}: {reason}");
        }
        // The uncalibrated question is persisted like any other judgment.
        let input = state
            .judgment_inputs
            .iter()
            .find(|i| i.label.contains("uncalibrated"))
            .unwrap();
        assert_eq!(
            input.state_sha256,
            telperion_jev::sha256_hex(&serde_json::to_vec(&input.state).unwrap())
        );
    }
}

#[test]
fn a_repeated_dial_and_action_is_refused_before_it_is_evaluated() {
    let mut mock = Mock {
        stall: true,
        ..mock()
    };
    let mut state = ready_to_round(&mut mock);
    state.budget.max_rounds = 3;
    state.execute(&mut mock, &mut |_| Ok(())).unwrap();
    let spent = mock.evaluations;

    // Same candidate, new evidence: the mock proposes the same move again.
    let mut fresh = state.visual.clone().unwrap();
    fresh.ledger = "visual:fresh-evidence".into();
    state.visual = Some(fresh);
    state.pause = None;
    state.execute(&mut mock, &mut |_| Ok(())).unwrap();

    assert_eq!(
        mock.evaluations, spent,
        "the same dial and action was evaluated twice"
    );
    assert!(
        state
            .routes
            .iter()
            .any(|r| r.starts_with("repeat refused: crookedness small_increase")),
        "{:?}",
        state.routes
    );
    let reason = &state.pause.as_ref().unwrap().reason;
    assert!(reason.contains("no supported proposal"), "{reason}");
}

#[test]
fn handoff_authorization_now_depends_only_on_risk() {
    for (bounded, authorized) in [(true, true), (false, false)] {
        let mut mock = Mock {
            route_plan: vec![("new_capability".into(), 0.9)],
            continuations: vec![bounded],
            ..mock()
        };
        let mut state = run();
        state.execute(&mut mock, &mut |_| Ok(())).unwrap();
        approve_priorities(
            &mut state,
            &mock,
            json!([{"id":"owner-crown","observation":"Crown shape","evidence_ids":["render-0","reference-0"],"views":["whole"]}]),
        );
        state.execute(&mut mock, &mut |_| Ok(())).unwrap();
        let handoff = state.handoffs.first().unwrap();
        assert_eq!(handoff.dispatch_authorized, authorized, "bounded={bounded}");
    }
}

#[test]
fn a_passing_visual_is_readiness_normally_and_an_owner_prompt_under_bootstrap() {
    for bootstrap in [false, true] {
        let mut mock = mock();
        // The third assessment passes every cell, which is where the engine
        // would otherwise finish.
        mock.visuals = 2;
        let mut state = run();
        state.visual_bootstrap = bootstrap;
        state.execute(&mut mock, &mut |_| Ok(())).unwrap();
        approve_priorities(&mut state, &mock, json!([]));
        state.execute(&mut mock, &mut |_| Ok(())).unwrap();

        assert_eq!(
            state.machine_ready, !bootstrap,
            "bootstrap={bootstrap}: machine_ready"
        );
        assert_eq!(
            state.reviewer_passed_unqualified, bootstrap,
            "bootstrap={bootstrap}: reviewer_passed_unqualified"
        );
        if bootstrap {
            let pause = state.pause.as_ref().expect("bootstrap must pause");
            assert!(
                pause.reason.contains("owner look required"),
                "{}",
                pause.reason
            );
            assert!(
                pause.reason.contains("reviewer unqualified for positives"),
                "{}",
                pause.reason
            );
            assert_eq!(
                pause.basis.proposed_action,
                "owner look at bootstrap finalist"
            );
        } else {
            assert!(state.pause.is_none(), "{:?}", state.pause);
        }
    }
}

#[test]
fn every_judgment_input_records_the_ledger_of_the_call_it_made() {
    // The live pilot left `ledger` null for the pre-dispatch risk call and for
    // a proposal call that returned nothing, so a spent call had no receipt.
    let mut m = Mock {
        route_plan: vec![("new_capability".into(), 0.9)],
        proposals: vec![],
        ..mock()
    };
    let mut state = run();
    state.execute(&mut m, &mut |_| Ok(())).unwrap();
    approve_priorities(
        &mut state,
        &m,
        json!([{"id":"owner-crown","observation":"Crown shape","evidence_ids":["render-0","reference-0"],"views":["whole"]}]),
    );
    state.execute(&mut m, &mut |_| Ok(())).unwrap();
    let risk = state
        .judgment_inputs
        .iter()
        .find(|i| i.label == "pre-dispatch risk")
        .expect("the risk call was made");
    assert_eq!(
        risk.ledger.as_deref(),
        Some("jev:risk"),
        "the pre-dispatch risk call left no ledger"
    );

    // A proposal call that yields nothing still spent a call.
    let mut m = Mock {
        proposals: vec![],
        ..mock()
    };
    let mut state = run();
    state.execute(&mut m, &mut |_| Ok(())).unwrap();
    approve_priorities(&mut state, &m, json!([]));
    state.execute(&mut m, &mut |_| Ok(())).unwrap();
    let proposals = state
        .judgment_inputs
        .iter()
        .find(|i| i.label == "targeted proposals")
        .expect("the proposal call was made");
    assert_eq!(
        proposals.ledger.as_deref(),
        Some("jev:propose"),
        "a proposal call returning nothing left no ledger"
    );
}

#[test]
fn the_proposal_state_is_focused_and_carries_nothing_it_should_not() {
    let mut mock = Mock {
        route_plan: vec![("tuning".into(), 0.9)],
        ..mock()
    };
    let mut state = run();
    state.execute(&mut mock, &mut |_| Ok(())).unwrap();
    approve_priorities(
        &mut state,
        &mock,
        json!([{"id":"owner-crown","observation":"Crown shape and foliage organization","evidence_ids":["render-0","reference-0"],"views":["whole"]}]),
    );
    // Give the run the history a realistic proposal would be shown.
    state.authorizations.push(
        serde_json::from_value(json!({"pause_id":"p","identity":state.identity,
            "action":"reassess","by":"owner","rationale":"scoped","preserve_evidence":true}))
        .unwrap(),
    );
    state.execute(&mut mock, &mut |_| Ok(())).unwrap();

    let focused = telperion_jev::tuning::judgments::proposal_state(&state);
    let bytes = serde_json::to_vec(&focused).unwrap();
    assert!(
        bytes.len() < 6144,
        "proposal state is {} bytes; the live pilot sent 25 KB",
        bytes.len()
    );
    let text = String::from_utf8(bytes).unwrap();
    for excluded in [
        "budget",
        "max_tokens",
        "authorizations",
        "resource_amendments",
        "agent_diagnoses",
        "verified_evidence_reuse",
        "joint",
        "cells",
        "evidence_ids",
        "priority_checkpoints",
    ] {
        assert!(
            !text.contains(excluded),
            "proposal state still carries {excluded}"
        );
    }
    // And it does carry what the judgment needs.
    for wanted in [
        "dials",
        "owner_notes",
        "measured_views",
        "attempts_from_this_candidate",
    ] {
        assert!(focused.get(wanted).is_some(), "missing {wanted}");
    }
    assert_eq!(
        focused["owner_priorities_routed_to_tuning"][0]["gap_id"],
        "owner-crown"
    );

    // The hash persisted before dispatch is the hash of that exact value.
    let recorded = state
        .judgment_inputs
        .iter()
        .find(|i| i.label == "targeted proposals")
        .unwrap();
    assert_eq!(
        recorded.state_sha256,
        telperion_jev::sha256_hex(&serde_json::to_vec(&recorded.state).unwrap())
    );
    assert!(!serde_json::to_string(&recorded.state)
        .unwrap()
        .contains("max_tokens"));
}

/// An accepted cap-only resume that preserved evidence, from `a` to `b`.
fn preserving(a: &str, b: &str, preserve: bool) -> serde_json::Value {
    json!({"pause_id":"p","identity":a,"action":"reassess","by":"owner",
        "rationale":"caps only","next_identity":b,"preserve_evidence":preserve})
}

#[test]
fn evidence_measured_under_an_earlier_revision_survives_a_cap_only_resume() {
    let mut mock = mock();
    let mut state = run();
    // A baseline and a candidate round, both measured under the old revision.
    let mut baseline = mock.evaluate(json!({}), 0, "baseline", None);
    baseline.identity = "old-revision".into();
    baseline.key = "candidate-old".into();
    baseline.score = Some(0.5);
    let mut attempt = mock.evaluate(json!({}), 1, "crookedness", None);
    attempt.identity = "old-revision".into();
    attempt.base = Some("candidate-old".into());
    attempt.action = Some(Action::SmallIncrease);
    attempt.evidence = Some("visual:1".into());
    attempt.score = Some(0.9);
    state.trials = vec![baseline, attempt];
    state.current = Some(0);
    state.identity = "new-revision".into();
    state
        .authorizations
        .push(serde_json::from_value(preserving("old-revision", "new-revision", true)).unwrap());

    // The finalist list still sees the preserved candidate.
    assert_eq!(
        state.finalists().len(),
        2,
        "preserved trials dropped out of finalists"
    );
    // The round history still sees the attempt, so the repeat filter works.
    let repeat = vec![Proposal {
        dial: "crookedness".into(),
        action: Action::SmallIncrease,
        ledger: "jev:2".into(),
        direction_mass: Some(0.9),
        rule: Some(telperion_jev::tuning::direction::RULE.into()),
    }];
    assert!(
        state.filter_repeats(repeat).is_empty(),
        "a move already tried under the earlier revision was offered again"
    );
    assert!(state
        .routes
        .iter()
        .any(|r| r.starts_with("repeat refused: crookedness")));

    // And a stall on the same evidence still pauses rather than re-buying.
    assert!(matches!(
        state.round_decision(),
        telperion_jev::tuning::round::Decision::Pause(_)
    ));
}

#[test]
fn a_resume_without_preservation_breaks_the_evidence_chain() {
    let mut mock = mock();
    let mut state = run();
    let mut baseline = mock.evaluate(json!({}), 0, "baseline", None);
    baseline.identity = "old-revision".into();
    baseline.key = "candidate-old".into();
    let mut attempt = mock.evaluate(json!({}), 1, "crookedness", None);
    attempt.identity = "old-revision".into();
    attempt.base = Some("candidate-old".into());
    attempt.action = Some(Action::SmallIncrease);
    state.trials = vec![baseline, attempt];
    state.current = Some(0);
    state.identity = "new-revision".into();
    // The resume in between dropped its evidence, so nothing before it counts.
    state
        .authorizations
        .push(serde_json::from_value(preserving("old-revision", "mid-revision", false)).unwrap());
    state
        .authorizations
        .push(serde_json::from_value(preserving("mid-revision", "new-revision", true)).unwrap());

    assert_eq!(
        state.finalists().len(),
        0,
        "a broken chain still counted the older trials"
    );
    let offered = vec![Proposal {
        dial: "crookedness".into(),
        action: Action::SmallIncrease,
        ledger: "jev:2".into(),
        direction_mass: Some(0.9),
        rule: Some(telperion_jev::tuning::direction::RULE.into()),
    }];
    assert_eq!(
        state.filter_repeats(offered).len(),
        1,
        "an unreachable attempt was treated as already tried"
    );
}
