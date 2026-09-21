use serde_json::json;
use telperion_jev::tuning::{
    actions::{Action, Dial},
    continuation::Basis,
    engine::{Answer, Proposal, Run, Services},
    evaluation::{Image, Trial},
    handoff::PriorityRoute,
    priority::Gap,
    progress::{self, Choice, Selection},
    sheet::{self, Movement},
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
    /// The round's candidate bound, as a config would set it.
    candidates: u64,
    /// Dials per proposal call; 0 means one call for all of them.
    batch: usize,
    /// The visual attempt fails, as an adapter that exits non-zero does.
    visual_error: bool,
    /// What decides between the current tree and a candidate.
    selection: Selection,
    /// One reviewer answer per candidate, consumed in order, stated as what the
    /// CANDIDATE did: `ABetter` means the candidate is the better render,
    /// whichever side code showed it on.
    reviews: Vec<Vec<Choice>>,
    /// The progress review fails, as a refused answer does.
    progress_error: bool,
    /// Candidates whose render matches the current tree's, in review order.
    inert: Vec<bool>,
    /// How many of them have been answered, since no call is made for one.
    inert_seen: std::cell::Cell<usize>,
    progress_calls: u64,
    proposal_calls: u64,
    /// Bundle mode: the strengths one bundle is drawn at.
    strengths: Vec<f64>,
    /// Sheet reviews one round may spend isolating what breaks.
    splits: u64,
    /// Variants the look keeps off the sheet, by trial label and reason.
    not_shown: Vec<(String, String)>,
    /// One sheet per review, consumed in order: what each trial key did, and
    /// anything it breaks. A key the sheet does not name did nothing.
    sheets: Vec<Vec<(String, Movement, Option<String>)>>,
    /// The sheet review fails, as a refused answer does.
    sheet_error: bool,
    sheet_calls: u64,
    /// The cell status each visual reports, in order. Empty keeps the default
    /// rule, where the third look onward passes.
    cell_status: Vec<CellStatus>,
    /// The raw answer and confidence the side-effect question comes back with.
    side_effect_answer: Option<(String, f64)>,
    side_effect_calls: u64,
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
        candidates: 4,
        batch: 0,
        visual_error: false,
        selection: Selection::Score,
        reviews: vec![],
        progress_error: false,
        inert: vec![],
        inert_seen: std::cell::Cell::new(0),
        progress_calls: 0,
        proposal_calls: 0,
        strengths: vec![0.5, 1.0, 2.0, 4.0],
        splits: 6,
        not_shown: vec![],
        sheets: vec![],
        sheet_error: false,
        sheet_calls: 0,
        cell_status: vec![],
        side_effect_answer: None,
        side_effect_calls: 0,
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
        if self.visual_error {
            return Err("reference-first adapter failed; reservation retained".into());
        }
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
    fn selection(&self) -> Selection {
        self.selection
    }
    fn progress_request(
        &self,
        state: &Run,
        current: usize,
        candidate: usize,
        priorities: &[telperion_jev::tuning::priority::Gap],
    ) -> Result<progress::Look, String> {
        if priorities.is_empty() {
            return Err("no tuning-routed priority to review".into());
        }
        if self
            .inert
            .get(self.progress_calls as usize + self.inert_seen.get())
            .copied()
            .unwrap_or(false)
        {
            self.inert_seen.set(self.inert_seen.get() + 1);
            return Ok(progress::Look::Inert);
        }
        let side =
            progress::candidate_side(&state.trials[current].key, &state.trials[candidate].key);
        Ok(progress::Look::Ask(
            progress::Request {
                schema: progress::VERSION.into(),
                target_species: "european-beech".into(),
                view: "whole".into(),
                seed: state.seed,
                references: vec![still("reference")],
                a: still("a"),
                b: still("b"),
                priorities: priorities
                    .iter()
                    .map(|g| progress::Priority {
                        id: g.id.clone(),
                        observation: g.observation.clone(),
                    })
                    .collect(),
                owner_notes: state.owner_notes.clone(),
            },
            side,
            true,
        ))
    }
    fn progress_tokens(&self, _: &progress::Request) -> u64 {
        1000
    }
    fn progress(
        &mut self,
        request: &progress::Request,
        side: &str,
    ) -> Result<Answer<progress::Verdict>, String> {
        if self.progress_error {
            return Err("progress adapter failed; reservation retained".into());
        }
        let plan = self
            .reviews
            .get(self.progress_calls as usize)
            .cloned()
            .unwrap_or_else(|| vec![Choice::Same; request.priorities.len()]);
        self.progress_calls += 1;
        let answer = progress::Answer {
            verdicts: request
                .priorities
                .iter()
                .zip(plan)
                .map(|(p, verdict)| progress::Judgment {
                    priority_id: p.id.clone(),
                    verdict: match (verdict, side) {
                        (Choice::ABetter, "b") => Choice::BBetter,
                        (Choice::BBetter, "b") => Choice::ABetter,
                        (other, _) => other,
                    },
                })
                .collect(),
            improved: "the outer foliage hangs further".into(),
            missing: "the crown is still enclosed".into(),
            regressions: vec![],
        };
        Ok(Answer {
            value: progress::bind(request, &answer, side, "review:1".into(), "mock".into())?,
            tokens: Some(10),
            ledger: Some("review:1".into()),
        })
    }
    fn bundle_strengths(&self) -> Vec<f64> {
        self.strengths.clone()
    }
    fn max_split_reviews(&self) -> u64 {
        self.splits
    }
    fn sheet_request(
        &self,
        state: &Run,
        current: usize,
        variants: &[usize],
        priorities: &[Gap],
    ) -> Result<sheet::Look, String> {
        if priorities.is_empty() {
            return Err("no tuning-routed priority to review".into());
        }
        let (mut shown, mut not_shown, mut stills) = (vec![], vec![], vec![]);
        for index in variants {
            let trial = &state.trials[*index];
            match self.not_shown.iter().find(|(l, _)| l == &trial.label) {
                Some((_, reason)) => not_shown.push(sheet::NotShown {
                    trial: *index,
                    inert: reason.contains("inert"),
                    reason: reason.clone(),
                }),
                None => {
                    stills.push((trial.key.clone(), still(&trial.key)));
                    shown.push(*index);
                }
            }
        }
        if shown.is_empty() {
            return Ok(sheet::Look {
                shown,
                not_shown,
                plan: None,
            });
        }
        let here = still(&format!("standing on {}", state.trials[current].key));
        let plan = sheet::plan(
            "european-beech",
            "whole",
            state.seed,
            &[still("reference")],
            (&state.trials[current].key, &here),
            &stills,
            &priorities
                .iter()
                .map(|g| progress::Priority {
                    id: g.id.clone(),
                    observation: g.observation.clone(),
                })
                .collect::<Vec<_>>(),
            &state.owner_notes,
        )?;
        Ok(sheet::Look {
            shown,
            not_shown,
            plan: Some(plan),
        })
    }
    fn sheet_tokens(&self, _: &sheet::Request) -> u64 {
        1000
    }
    fn sheet(&mut self, plan: &sheet::Plan) -> Result<Answer<sheet::Verdict>, String> {
        if self.sheet_error {
            return Err("sheet adapter failed; reservation retained".into());
        }
        let spec = self
            .sheets
            .get(self.sheet_calls as usize)
            .cloned()
            .unwrap_or_default();
        self.sheet_calls += 1;
        let answer = sheet_answer(plan, &spec);
        Ok(Answer {
            value: sheet::bind(plan, &answer, "sheet:1".into(), "mock".into())?,
            tokens: Some(10),
            ledger: Some("sheet:1".into()),
        })
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
            progress: None,
            adopted_over: vec![],
            bundle: None,
            parent_bundle: None,
            sheet: None,
            vetoed: None,
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
                coverage: vec![],
                cells: vec![(
                    cell(),
                    match self.cell_status.get(self.visuals as usize - 1) {
                        Some(status) => *status,
                        None if self.visuals >= 3 => CellStatus::Pass,
                        None => CellStatus::Fail,
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
    fn side_effect_tokens(&self, _: &serde_json::Value) -> u64 {
        500
    }
    fn side_effects(
        &mut self,
        _: &serde_json::Value,
    ) -> Result<Answer<telperion_jev::tuning::veto::Judged>, String> {
        self.side_effect_calls += 1;
        let (raw, confidence) = self
            .side_effect_answer
            .clone()
            .unwrap_or_else(|| ("no_new_defect".into(), 0.9));
        Ok(Answer {
            ledger: Some("jev:side-effects".into()),
            tokens: Some(20),
            value: telperion_jev::tuning::veto::Judged {
                choice: if confidence >= 0.5 {
                    raw.clone()
                } else {
                    "insufficient_evidence".into()
                },
                raw_choice: Some(raw),
                confidence: Some(confidence),
                threshold: 0.5,
                ledger: Some("jev:side-effects".into()),
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
    fn max_candidates(&self) -> u64 {
        self.candidates
    }
    fn proposal_batches(&self, _: &Run) -> usize {
        if self.batch == 0 {
            1
        } else {
            self.proposals.len().div_ceil(self.batch).max(1)
        }
    }
    fn propose(&mut self, _: &Run, batch: usize) -> Result<Answer<Vec<Proposal>>, String> {
        self.proposal_calls += 1;
        let slice: Vec<Proposal> = if self.batch == 0 {
            self.proposals.clone()
        } else {
            self.proposals
                .chunks(self.batch)
                .nth(batch)
                .map(<[Proposal]>::to_vec)
                .unwrap_or_default()
        };
        let value = slice
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
            ledger: Some(format!("jev:propose:{}", batch + 1)),
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
fn still(body: &str) -> Image {
    let path = std::env::temp_dir().join(format!(
        "engine-progress-{}.png",
        telperion_jev::ledger::new_entry_id()
    ));
    std::fs::write(&path, body.as_bytes()).unwrap();
    Image {
        path,
        sha256: telperion_jev::sha256_hex(body.as_bytes()),
        view: "whole".into(),
        seed: 1,
    }
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
            group: None,
            score_visible: None,
            meaning_basis: None,
            range_basis: None,
            source: None,
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
    assert!(labels.iter().any(|l| l.starts_with("targeted proposals")));
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
fn stalled_one_candidate_rounds_work_through_the_dials_before_stopping() {
    let mut state = run();
    // Three real dials the router can move, and room for four rounds.
    state.dials = serde_json::from_value(json!([
        {"id":"limbs","path":"/skeleton/habit/lateralsPerStation","meaning":"limbs",
         "min":1,"max":4,"small":1,"substantial":2,"integer":true},
        {"id":"leaves","path":"/canopy/shortShootLeaves","meaning":"leaves",
         "min":2,"max":12,"small":2,"substantial":4,"integer":true},
        {"id":"spacing","path":"/canopy/shortShootSpacing","meaning":"spacing",
         "min":0.01,"max":0.08,"small":0.01,"substantial":0.02,"integer":false}
    ]))
    .unwrap();
    state.budget.max_rounds = 5;
    let proposal = |id: &str, mass: f64| Proposal {
        dial: id.into(),
        action: Action::SmallIncrease,
        ledger: "jev:2".into(),
        direction_mass: Some(mass),
        rule: Some(telperion_jev::tuning::direction::RULE.into()),
    };
    let mut mock = Mock {
        stall: true,
        candidates: 1,
        // Offered every round in direction-mass order; the filter thins them.
        proposals: vec![
            proposal("limbs", 0.95),
            proposal("leaves", 0.85),
            proposal("spacing", 0.75),
        ],
        ..mock()
    };
    state.execute(&mut mock, &mut |_| Ok(())).unwrap();
    approve_priorities(&mut state, &mock, json!([]));
    state.execute(&mut mock, &mut |_| Ok(())).unwrap();

    // One candidate per round, a different dial each time, in mass order, and
    // no judgment was bought to justify carrying on.
    let tried = state
        .trials
        .iter()
        .filter(|t| t.round > 0)
        .map(|t| t.label.clone())
        .collect::<Vec<_>>();
    assert_eq!(
        tried,
        vec!["limbs", "leaves", "spacing"],
        "the rounds did not work through the dials in order"
    );
    assert_eq!(
        mock.evidence_calls, 0,
        "a stall on old evidence bought a judgment"
    );

    // The fourth round has nothing untried left, and says so.
    let reason = &state.pause.as_ref().unwrap().reason;
    assert!(
        reason.contains("every supported move was already tried on this candidate"),
        "{reason}"
    );
    assert_eq!(
        state
            .routes
            .iter()
            .filter(|r| r.starts_with("repeat refused:"))
            .count(),
        6,
        "{:?}",
        state.routes
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
        .find(|i| i.label.starts_with("targeted proposals"))
        .expect("the proposal call was made");
    assert_eq!(
        proposals.ledger.as_deref(),
        Some("jev:propose:1"),
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
        .find(|i| i.label.starts_with("targeted proposals"))
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
        state.filter_repeats(repeat).0.is_empty(),
        "a move already tried under the earlier revision was offered again"
    );
    assert!(state
        .routes
        .iter()
        .any(|r| r.starts_with("repeat refused: crookedness")));

    // A stall on the same evidence proceeds without buying a judgment; the
    // repeat filter above is what stops a move already tried.
    assert_eq!(
        state.round_decision(),
        telperion_jev::tuning::round::Decision::Proceed
    );
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
        state.filter_repeats(offered).0.len(),
        1,
        "an unreachable attempt was treated as already tried"
    );
}

#[test]
fn a_cap_only_resume_keeps_one_handoff_per_priority_and_rebuys_no_risk() {
    let gap = json!([{"id":"owner-materials","observation":"Materials",
        "evidence_ids":["render-0","reference-0"],"views":["whole"]}]);
    let mut mock = Mock {
        route_plan: vec![("appearance".into(), 0.9)],
        stall: true,
        ..mock()
    };
    let mut state = run();
    state.budget.max_rounds = 4;
    state.execute(&mut mock, &mut |_| Ok(())).unwrap();
    approve_priorities(&mut state, &mock, gap.clone());
    state.execute(&mut mock, &mut |_| Ok(())).unwrap();

    assert_eq!(
        mock.pre_dispatch_calls, 1,
        "the first handoff was judged once"
    );
    assert_eq!(state.current_handoffs().len(), 1);

    // A cap-only resume that preserved its evidence: new identity, same run.
    let previous = state.identity.clone();
    state.identity = "raised-caps".into();
    state.authorizations.push(
        serde_json::from_value(
            json!({"pause_id":"p","identity":previous,"action":"reassess","by":"owner",
                "rationale":"caps only","next_identity":"raised-caps",
                "preserve_evidence":true}),
        )
        .unwrap(),
    );
    state.pause = None;
    state.budget.max_rounds = 6;
    state.execute(&mut mock, &mut |_| Ok(())).unwrap();

    // The handoff carried across, so its judgment was not bought again and no
    // second copy was written.
    assert_eq!(
        mock.pre_dispatch_calls, 1,
        "the risk judgment was re-bought after a cap-only resume"
    );
    assert_eq!(
        state.current_handoffs().len(),
        1,
        "a second copy of the same handoff was written"
    );
    assert_eq!(
        state.handoffs.len(),
        1,
        "the stored journal grew a duplicate"
    );
    assert_eq!(
        state.unresolved_priorities(),
        vec!["owner-materials".to_string()]
    );
}

#[test]
fn a_large_dial_table_is_asked_in_batches_each_its_own_judgment() {
    let dials = json!([
        {"id":"limbs","path":"/skeleton/habit/lateralsPerStation","meaning":"limbs",
         "min":1,"max":4,"small":1,"substantial":2,"integer":true},
        {"id":"leaves","path":"/canopy/shortShootLeaves","meaning":"leaves",
         "min":2,"max":12,"small":2,"substantial":4,"integer":true},
        {"id":"spacing","path":"/canopy/shortShootSpacing","meaning":"spacing",
         "min":0.01,"max":0.08,"small":0.01,"substantial":0.02,"integer":false},
        {"id":"crookedness","path":"/skeleton/habit/crookedness","meaning":"turn",
         "min":0,"max":15,"small":1,"substantial":3,"integer":false},
        {"id":"irregularity","path":"/skeleton/envelope/irregularity","meaning":"lobes",
         "min":0,"max":0.5,"small":0.08,"substantial":0.16,"integer":false}
    ]);
    let proposal = |id: &str, mass: f64| Proposal {
        dial: id.into(),
        action: Action::SmallIncrease,
        ledger: "jev:2".into(),
        direction_mass: Some(mass),
        rule: Some(telperion_jev::tuning::direction::RULE.into()),
    };
    let mut state = run();
    state.dials = serde_json::from_value(dials).unwrap();
    // One round at a time, so the call count is the batch count.
    state.budget.max_rounds = 1;
    let mut mock = Mock {
        stall: true,
        candidates: 1,
        batch: 2,
        // Ascending mass, so a correct merge must reorder across batches.
        proposals: vec![
            proposal("limbs", 0.55),
            proposal("leaves", 0.65),
            proposal("spacing", 0.75),
            proposal("crookedness", 0.85),
            proposal("irregularity", 0.95),
        ],
        ..mock()
    };
    state.execute(&mut mock, &mut |_| Ok(())).unwrap();
    approve_priorities(&mut state, &mock, json!([]));
    state.execute(&mut mock, &mut |_| Ok(())).unwrap();

    // Five dials at two per call is three calls, each its own judgment input
    // with its own ledger.
    assert_eq!(mock.proposal_calls, 3, "expected three batched calls");
    let inputs = state
        .judgment_inputs
        .iter()
        .filter(|i| i.label.starts_with("targeted proposals"))
        .collect::<Vec<_>>();
    assert_eq!(inputs.len(), 3);
    for (n, input) in inputs.iter().enumerate() {
        assert_eq!(input.label, format!("targeted proposals {}", n + 1));
        assert_eq!(
            input.ledger.as_deref(),
            Some(format!("jev:propose:{}", n + 1).as_str()),
            "batch {} lost its receipt",
            n + 1
        );
        assert_eq!(
            input.state_sha256,
            telperion_jev::sha256_hex(&serde_json::to_vec(&input.state).unwrap())
        );
    }
    // Merged across batches, the highest direction mass went first.
    assert_eq!(state.trials.last().unwrap().label, "irregularity");
    // Only one round was charged for the three calls.
    assert_eq!(state.budget.rounds, 1);

    // The next round re-asks, and the repeat refused does not eat the slot.
    let spent = mock.evaluations;
    state.budget.max_rounds = 2;
    state.pause = None;
    state.execute(&mut mock, &mut |_| Ok(())).unwrap();
    assert_eq!(
        mock.evaluations,
        spent + 1,
        "a refused repeat consumed the round's only candidate"
    );
    assert_eq!(state.trials.last().unwrap().label, "crookedness");
}

/// A visual that fails still spent its pass, so the reservation stays charged
/// and the attempt stays recorded as pending. Recovery is the owner's scoped
/// decision, never a silent retry.
#[test]
fn a_failed_visual_keeps_its_reservation_and_leaves_the_attempt_pending() {
    let mut state = run();
    let mut mock = Mock {
        visual_error: true,
        ..mock()
    };
    state.execute(&mut mock, &mut |_| Ok(())).unwrap();
    assert_eq!(state.pending.as_deref(), Some("visual assessment"));
    assert_eq!(state.budget.visual_passes, Some(1));
    assert!(state
        .pause
        .as_ref()
        .unwrap()
        .reason
        .contains("reservation retained"));
    let charged = state.budget.clone();

    // A second invocation cannot retry it: the spend is unsettled.
    state.pause = None;
    state.execute(&mut mock, &mut |_| Ok(())).unwrap();
    assert_eq!(
        state.pause.as_ref().unwrap().reason,
        "interrupted attempt; reservation retained"
    );
    assert_eq!(state.pending.as_deref(), Some("visual assessment"));
    assert_eq!(
        serde_json::to_value(&state.budget).unwrap(),
        serde_json::to_value(&charged).unwrap(),
        "a refused retry charges nothing further"
    );

    // Cleared the way `recover_interrupted` clears it, the run goes on and the
    // pass and the tokens it spent stay spent.
    state.pause = None;
    state.pending = None;
    mock.visual_error = false;
    state.execute(&mut mock, &mut |_| Ok(())).unwrap();
    assert_eq!(state.budget.visual_passes, Some(2));
    assert!(state.budget.tokens >= charged.tokens);
    assert!(state.visual.is_some());
}

fn reviewed_run() -> (Run, Mock) {
    let mut state = run();
    state.budget.max_rounds = 2;
    let mock = Mock {
        selection: Selection::Visual,
        route_plan: vec![("tuning".into(), 0.9)],
        proposals: vec![
            Proposal {
                dial: "crookedness".into(),
                action: Action::SmallIncrease,
                ledger: "jev:2".into(),
                direction_mass: Some(0.9),
                rule: Some(telperion_jev::tuning::direction::RULE.into()),
            },
            Proposal {
                dial: "crookedness".into(),
                action: Action::SmallDecrease,
                ledger: "jev:3".into(),
                direction_mass: Some(0.7),
                rule: Some(telperion_jev::tuning::direction::RULE.into()),
            },
        ],
        candidates: 2,
        ..mock()
    };
    (state, mock)
}

fn approve_one(state: &mut Run, mock: &Mock) {
    approve_priorities(
        state,
        mock,
        json!([{"id":"owner-crown","observation":"Crown shape and foliage organization",
            "evidence_ids":["render-0","reference-0"],"views":["whole"]}]),
    );
}

#[test]
fn under_visual_selection_the_reviewer_adopts_the_candidate_it_judged_better() {
    let (mut state, mut mock) = reviewed_run();
    mock.reviews = vec![vec![Choice::Same], vec![Choice::ABetter]];
    state.execute(&mut mock, &mut |_| Ok(())).unwrap();
    approve_one(&mut state, &mock);
    let passes_before = state.budget.visual_passes.unwrap();
    state.execute(&mut mock, &mut |_| Ok(())).unwrap();

    assert_eq!(
        mock.progress_calls, 2,
        "every feasible candidate is reviewed"
    );
    let reviewed: Vec<&Trial> = state
        .trials
        .iter()
        .filter(|t| t.progress.is_some())
        .collect();
    assert_eq!(reviewed.len(), 2);
    for trial in &reviewed {
        let verdict = trial.progress.as_ref().unwrap();
        assert!(verdict.uncalibrated.contains("uncalibrated"));
        assert!(verdict.candidate_is == "a" || verdict.candidate_is == "b");
    }
    // The one the reviewer called better is the one the round kept, whatever
    // the scores did, and each review spent a pass of its own.
    let adopted = &state.trials[state.current.unwrap()];
    assert_eq!(adopted.progress.as_ref().unwrap().better(), 1);
    assert!(state.budget.visual_passes.unwrap() >= passes_before + 2);
    // The reviewer's words reach whoever is asked next.
    let projected = telperion_jev::tuning::judgments::summary(&state);
    assert!(
        serde_json::to_string(&projected["recent_attempts"])
            .unwrap()
            .contains("the outer foliage hangs further"),
        "the routing state lost the reviewer's words"
    );
}

#[test]
fn a_round_whose_candidates_are_all_the_same_is_a_visual_stall() {
    let (mut state, mut mock) = reviewed_run();
    mock.reviews = vec![vec![Choice::Same], vec![Choice::Same]];
    state.execute(&mut mock, &mut |_| Ok(())).unwrap();
    approve_one(&mut state, &mock);
    let before = state.current;
    state.execute(&mut mock, &mut |_| Ok(())).unwrap();
    assert_eq!(state.current, before, "nothing was adopted");
    assert!(
        state
            .routes
            .iter()
            .any(|r| r == "visual stall; no candidate judged better"),
        "{:?}",
        state.routes
    );
    assert!(!state.routes.iter().any(|r| r.starts_with("numeric stall")));
}

#[test]
fn a_refused_progress_answer_leaves_a_recoverable_attempt() {
    let (mut state, mut mock) = reviewed_run();
    state.execute(&mut mock, &mut |_| Ok(())).unwrap();
    approve_one(&mut state, &mock);
    mock.progress_error = true;
    state.execute(&mut mock, &mut |_| Ok(())).unwrap();
    assert_eq!(state.pending.as_deref(), Some("progress review"));
    assert!(state
        .pause
        .as_ref()
        .unwrap()
        .reason
        .contains("reservation retained"));
    let charged = serde_json::to_value(&state.budget).unwrap();
    state.pause = None;
    state.execute(&mut mock, &mut |_| Ok(())).unwrap();
    assert_eq!(
        state.pause.as_ref().unwrap().reason,
        "interrupted attempt; reservation retained"
    );
    assert_eq!(serde_json::to_value(&state.budget).unwrap(), charged);
}

#[test]
fn a_reviewed_attempt_that_was_not_adopted_is_never_proposed_again() {
    let (mut state, mut mock) = reviewed_run();
    mock.reviews = vec![vec![Choice::Same], vec![Choice::Same]];
    state.execute(&mut mock, &mut |_| Ok(())).unwrap();
    approve_one(&mut state, &mock);
    state.execute(&mut mock, &mut |_| Ok(())).unwrap();
    // The mock proposes both moves under one action, so both were tried here.
    let tried: Vec<Proposal> = mock
        .proposals
        .iter()
        .cloned()
        .map(|mut p| {
            p.action = mock.proposal_action;
            p
        })
        .collect();
    let (kept, refused) = state.filter_repeats(tried);
    assert!(
        kept.is_empty() && refused == 2,
        "a reviewed attempt that was not adopted is still on offer: {kept:?}"
    );
}

#[test]
fn the_proposal_state_carries_the_reviewer_words_and_stays_small() {
    let (mut state, mut mock) = reviewed_run();
    // Neither is adopted, so both attempts stay under the current candidate,
    // which is what the next proposal is shown.
    mock.reviews = vec![vec![Choice::Same], vec![Choice::Same]];
    state.execute(&mut mock, &mut |_| Ok(())).unwrap();
    approve_one(&mut state, &mock);
    state.execute(&mut mock, &mut |_| Ok(())).unwrap();
    let table: Vec<Dial> = serde_json::from_slice(include_bytes!("../data/dials.json")).unwrap();
    state.dials = table
        .into_iter()
        .filter(|d| d.score_visible == Some(true))
        .collect();
    assert!(state.dials.len() > 100, "{}", state.dials.len());
    let projected = telperion_jev::tuning::judgments::proposal_state(&state);
    let bytes = serde_json::to_vec(&projected).unwrap();
    assert!(
        String::from_utf8_lossy(&bytes).contains("the crown is still enclosed"),
        "the proposal state lost the reviewer's words"
    );
    assert!(
        bytes.len() < 24_576,
        "proposal state is {} bytes",
        bytes.len()
    );
}

fn three_proposals() -> Vec<Proposal> {
    ["twig_hang", "irregularity", "rise_secondary"]
        .iter()
        .enumerate()
        .map(|(i, dial)| Proposal {
            dial: (*dial).into(),
            action: Action::SmallIncrease,
            ledger: format!("jev:{i}"),
            direction_mass: Some(0.9 - i as f64 / 10.),
            rule: Some(telperion_jev::tuning::direction::RULE.into()),
        })
        .collect()
}

fn reviewed_three(inert: Vec<bool>, reviews: Vec<Vec<Choice>>) -> (Run, Mock) {
    let mut state = run();
    state.budget.max_rounds = 2;
    state.dials = ["twig_hang", "irregularity", "rise_secondary"]
        .iter()
        .map(|id| Dial {
            id: (*id).into(),
            path: "/skeleton/habit/crookedness".into(),
            meaning: "turn variation".into(),
            min: 0.,
            max: 15.,
            small: 1.,
            substantial: 3.,
            integer: false,
            group: None,
            score_visible: None,
            meaning_basis: None,
            range_basis: None,
            source: None,
        })
        .collect();
    let mock = Mock {
        selection: Selection::Visual,
        route_plan: vec![("tuning".into(), 0.9)],
        proposals: three_proposals(),
        candidates: 3,
        inert,
        reviews,
        ..mock()
    };
    (state, mock)
}

#[test]
fn an_inert_candidate_costs_no_review_and_does_not_end_the_round() {
    let (mut state, mut mock) = reviewed_three(
        vec![true, false, false],
        vec![vec![Choice::Same], vec![Choice::ABetter]],
    );
    state.execute(&mut mock, &mut |_| Ok(())).unwrap();
    approve_one(&mut state, &mock);
    state.execute(&mut mock, &mut |_| Ok(())).unwrap();

    assert_eq!(
        mock.progress_calls, 2,
        "the inert candidate was not sent to the reviewer"
    );
    let reviewed: Vec<&Trial> = state
        .trials
        .iter()
        .filter(|t| t.progress.is_some())
        .collect();
    assert_eq!(reviewed.len(), 3, "every candidate was still evaluated");
    let settled = reviewed[0].progress.as_ref().unwrap();
    assert!(settled.inert && !settled.adoptable() && settled.model == "code");
    assert!(state
        .routes
        .iter()
        .any(|r| r.starts_with("inert: twig_hang")));
    // The candidate the reviewer called better is the one the round kept.
    let adopted = state.trials[state.current.unwrap()]
        .progress
        .as_ref()
        .unwrap();
    assert!(!adopted.inert && adopted.better() == 1);
    assert!(state.pending.is_none() && state.pause.is_none());
}

#[test]
fn a_round_of_inert_candidates_is_a_visual_stall_that_cost_no_reviewer() {
    let (mut state, mut mock) = reviewed_three(vec![true, true, true], vec![]);
    state.execute(&mut mock, &mut |_| Ok(())).unwrap();
    approve_one(&mut state, &mock);
    let before = state.current;
    state.execute(&mut mock, &mut |_| Ok(())).unwrap();
    assert_eq!(mock.progress_calls, 0, "nobody was paid to look");
    assert_eq!(state.current, before);
    assert_eq!(
        state
            .routes
            .iter()
            .filter(|r| r.starts_with("inert: "))
            .count(),
        3
    );
    assert!(state
        .routes
        .iter()
        .any(|r| r == "visual stall; no candidate judged better"));
}

#[test]
fn the_proposal_state_says_which_attempt_did_nothing_at_all() {
    let (mut state, mut mock) = reviewed_three(
        vec![true, false, false],
        vec![vec![Choice::Same], vec![Choice::Same]],
    );
    state.execute(&mut mock, &mut |_| Ok(())).unwrap();
    approve_one(&mut state, &mock);
    state.execute(&mut mock, &mut |_| Ok(())).unwrap();
    let projected = telperion_jev::tuning::judgments::proposal_state(&state);
    let attempts = serde_json::to_string(&projected["attempts_from_this_candidate"]).unwrap();
    assert!(
        attempts.contains("\"inert\":true") && attempts.contains("\"inert\":false"),
        "{attempts}"
    );
    let summary =
        serde_json::to_string(&telperion_jev::tuning::judgments::summary(&state)).unwrap();
    assert!(summary.contains("\"inert\":true"));
}

#[test]
fn a_round_with_two_adoptable_moves_records_the_one_it_did_not_keep() {
    let (mut state, mut mock) = reviewed_three(
        vec![false, false, false],
        vec![
            vec![Choice::Same],
            vec![Choice::ABetter],
            vec![Choice::ABetter],
        ],
    );
    state.execute(&mut mock, &mut |_| Ok(())).unwrap();
    approve_one(&mut state, &mock);
    state.execute(&mut mock, &mut |_| Ok(())).unwrap();
    let adopted = &state.trials[state.current.unwrap()];
    assert_eq!(adopted.label, "irregularity", "the earlier tie wins");
    assert_eq!(
        adopted.adopted_over.len(),
        1,
        "the other eligible move is not recorded: {:?}",
        adopted.adopted_over
    );
    assert!(state
        .trials
        .iter()
        .any(|t| t.label == "rise_secondary" && t.key == adopted.adopted_over[0]));
}

/// A sheet answer that says exactly what each variant did, in the terms a test
/// states them in. The reviewer only ever ranks and grades, so the ranking is
/// built so that the binding reads those movements back: the clear ones above
/// the slight ones above the current tree, the indistinguishable ones just
/// below it, and everything the test called worse below a visible step down.
fn sheet_answer(plan: &sheet::Plan, spec: &[(String, Movement, Option<String>)]) -> sheet::Answer {
    let did = |key: &String| {
        spec.iter()
            .find(|(k, _, _)| k == key)
            .map_or(Movement::None, |(_, m, _)| *m)
    };
    let label =
        |key: &String| sheet::Request::label(plan.order.iter().position(|k| k == key).unwrap());
    let current_key = &plan.order[plan.current.parse::<usize>().unwrap() - 1];
    let group = |want: Movement| {
        plan.order
            .iter()
            .filter(|key| *key != current_key && did(key) == want)
            .map(label)
            .collect::<Vec<String>>()
    };
    let (clear, slight) = (group(Movement::Clear), group(Movement::Slight));
    let (none, worse) = (group(Movement::None), group(Movement::Worse));
    let mut ranking = clear.clone();
    ranking.extend(slight.clone());
    ranking.push(plan.current.clone());
    ranking.extend(none.clone());
    ranking.extend(worse.clone());
    let mut grades = vec![sheet::Grade::None; ranking.len() - 1];
    if !clear.is_empty() {
        grades[clear.len() - 1] = sheet::Grade::Clear;
    }
    if !slight.is_empty() {
        grades[clear.len() + slight.len() - 1] = sheet::Grade::Slight;
    }
    if !worse.is_empty() {
        grades[clear.len() + slight.len() + none.len()] = sheet::Grade::Clear;
    }
    let steps = ranking
        .windows(2)
        .zip(&grades)
        .map(|(pair, grade)| sheet::Step {
            from: pair[0].clone(),
            to: pair[1].clone(),
            grade: *grade,
        })
        .collect::<Vec<_>>();
    sheet::Answer {
        // The overall ranking follows the same order, so a variant the test
        // called better is also the more believable tree.
        overall: ranking.clone(),
        wrong: vec![sheet::Wrong {
            render: ranking[0].clone(),
            text: "the outer branches are too thick".into(),
        }],
        priorities: plan
            .request
            .priorities
            .iter()
            .map(|p| sheet::PriorityAnswer {
                priority_id: p.id.clone(),
                closest: ranking[0].clone(),
                ranking: ranking.clone(),
                steps: steps.clone(),
            })
            .collect(),
        breaks: spec
            .iter()
            .filter_map(|(key, _, text)| {
                text.as_ref().map(|text| sheet::Break {
                    render: label(key),
                    text: text.clone(),
                })
            })
            .collect(),
        improved: "the outer foliage hangs further".into(),
        missing: "the crown is still enclosed".into(),
    }
}

fn bundle_dial(id: &str, path: &str, min: f64, max: f64, small: f64, group: &str) -> Dial {
    Dial {
        id: id.into(),
        path: path.into(),
        meaning: "a row".into(),
        min,
        max,
        small,
        substantial: small * 2.,
        integer: false,
        group: Some(group.into()),
        score_visible: Some(true),
        meaning_basis: None,
        range_basis: None,
        source: None,
    }
}

/// A run whose round moves every supported dial together. Two of the three
/// dials share a group, so a split cuts the pair against the single.
fn bundle_run() -> (Run, Mock) {
    let mut state = run();
    state.budget.max_rounds = 2;
    state.budget.max_visual_passes = Some(8);
    state.dials = vec![
        bundle_dial("twig_hang", "/skeleton/twigs/hang", 0., 3., 0.5, "twigs"),
        bundle_dial(
            "rise_secondary",
            "/skeleton/habit/riseSecondary",
            -1.,
            1.,
            0.2,
            "habit",
        ),
        bundle_dial(
            "crookedness",
            "/skeleton/habit/crookedness",
            0.,
            15.,
            1.,
            "habit",
        ),
    ];
    let proposals = ["twig_hang", "rise_secondary", "crookedness"]
        .iter()
        .enumerate()
        .map(|(i, dial)| Proposal {
            dial: (*dial).into(),
            action: Action::SmallIncrease,
            ledger: format!("jev:{i}"),
            direction_mass: Some(0.9 - i as f64 / 10.),
            rule: Some(telperion_jev::tuning::direction::RULE.into()),
        })
        .collect();
    let mock = Mock {
        selection: Selection::Bundle,
        route_plan: vec![("tuning".into(), 0.9)],
        proposals,
        ..mock()
    };
    (state, mock)
}

/// Runs the baseline, takes the owner's approval, then runs the round.
fn to_the_round(state: &mut Run, mock: &mut Mock) {
    state.execute(mock, &mut |_| Ok(())).unwrap();
    approve_one(state, mock);
    state.execute(mock, &mut |_| Ok(())).unwrap();
}

fn did(label: &str, movement: Movement) -> (String, Movement, Option<String>) {
    (label.into(), movement, None)
}

/// The trial keys the mock hands out, in evaluation order: the baseline first,
/// then one per variant the round drew.
fn key(n: u64) -> String {
    format!("candidate{n}")
}

#[test]
fn four_strengths_are_judged_on_one_sheet_and_the_smallest_clear_one_is_kept() {
    let (mut state, mut mock) = bundle_run();
    mock.sheets = vec![vec![
        did(&key(2), Movement::Clear),
        did(&key(3), Movement::Clear),
        did(&key(4), Movement::Slight),
        did(&key(5), Movement::None),
    ]];
    to_the_round(&mut state, &mut mock);

    assert_eq!(mock.sheet_calls, 1, "one sheet judged the whole round");
    assert_eq!(mock.evaluations, 5, "the baseline and four strengths");
    let variants: Vec<&Trial> = state.trials.iter().filter(|t| t.bundle.is_some()).collect();
    assert_eq!(variants.len(), 4);
    assert!(variants.iter().all(|t| t.sheet.is_some()));
    assert!(variants
        .iter()
        .all(|t| t.bundle.as_ref().unwrap().moves.len() == 3));
    let adopted = &state.trials[state.current.unwrap()];
    assert_eq!(
        adopted.label, "bundle@0.5",
        "a clear variant at a larger strength was kept instead"
    );
    assert_eq!(
        adopted.adopted_over.len(),
        2,
        "the other clear variant and the slight one are not recorded: {:?}",
        adopted.adopted_over
    );
    assert!(state
        .judgment_inputs
        .iter()
        .any(|i| i.label == "uncalibrated sheet review"));
    assert!(state.pending.is_none());
}

#[test]
fn a_variant_that_draws_the_current_tree_or_another_variant_is_never_shown() {
    let (mut state, mut mock) = bundle_run();
    mock.not_shown = vec![
        (
            "bundle@0.5".into(),
            telperion_jev::tuning::sheet::INERT.into(),
        ),
        ("bundle@2".into(), "draws the same tree as bundle@1".into()),
    ];
    mock.sheets = vec![vec![did(&key(3), Movement::Clear)]];
    to_the_round(&mut state, &mut mock);

    assert_eq!(mock.sheet_calls, 1);
    let unshown: Vec<&Trial> = state
        .trials
        .iter()
        .filter(|t| t.label == "bundle@0.5" || t.label == "bundle@2")
        .collect();
    assert_eq!(unshown.len(), 2, "both variants were still evaluated");
    assert!(unshown[0].sheet.as_ref().unwrap().inert);
    assert!(!unshown[1].sheet.as_ref().unwrap().inert);
    assert!(unshown
        .iter()
        .all(|t| t.sheet.as_ref().unwrap().per_priority.is_empty()));
    assert_eq!(
        state
            .routes
            .iter()
            .filter(|r| r.contains("not shown"))
            .count(),
        2
    );
    assert_eq!(state.trials[state.current.unwrap()].label, "bundle@1");
}

#[test]
fn a_break_halves_the_bundle_and_the_clean_half_is_the_one_kept() {
    let (mut state, mut mock) = bundle_run();
    mock.sheets = vec![
        // The whole bundle is better and breaks something; nothing else moved.
        vec![(key(3), Movement::Clear, Some("the bole is bare".into()))],
        // Of its two halves, the pair is clean and the single one did nothing.
        vec![did(&key(6), Movement::Clear), did(&key(7), Movement::None)],
    ];
    to_the_round(&mut state, &mut mock);

    assert_eq!(
        mock.sheet_calls, 2,
        "one sheet for the bundle, one to split"
    );
    let halves: Vec<&Trial> = state
        .trials
        .iter()
        .filter(|t| t.parent_bundle.is_some())
        .collect();
    assert_eq!(halves.len(), 2);
    let parent = state
        .trials
        .iter()
        .find(|t| t.label == "bundle@1")
        .and_then(|t| t.bundle.as_ref())
        .unwrap();
    assert!(halves
        .iter()
        .all(|t| t.parent_bundle.as_deref() == Some(parent.id.as_str())));
    assert_eq!(
        halves
            .iter()
            .map(|t| t.bundle.as_ref().unwrap().moves.len())
            .sum::<usize>(),
        3,
        "the halves together are the bundle"
    );
    let adopted = &state.trials[state.current.unwrap()];
    assert!(
        adopted.label.starts_with("bundle@1 half"),
        "{}",
        adopted.label
    );
    assert_eq!(adopted.bundle.as_ref().unwrap().moves.len(), 2);
    assert!(state.pause.is_none(), "{:?}", state.pause);
}

#[test]
fn a_round_nothing_improves_stalls_and_the_same_bundle_is_not_bought_twice() {
    let (mut state, mut mock) = bundle_run();
    mock.sheets = vec![vec![]];
    state.execute(&mut mock, &mut |_| Ok(())).unwrap();
    approve_one(&mut state, &mock);
    let before = state.current;
    state.execute(&mut mock, &mut |_| Ok(())).unwrap();

    assert_eq!(state.current, before, "nothing was adopted");
    assert!(
        state
            .routes
            .iter()
            .any(|r| r == "bundle stall; no strength judged better"),
        "{:?}",
        state.routes
    );
    // The reviewer's words survive on every variant it looked at.
    assert!(state
        .trials
        .iter()
        .filter_map(|t| t.sheet.as_ref())
        .all(|s| s.missing == "the crown is still enclosed"));
    let attempts = serde_json::to_string(
        &telperion_jev::tuning::judgments::proposal_state(&state)["attempts_from_this_candidate"],
    )
    .unwrap();
    assert!(attempts.contains("twig_hang") && attempts.contains("\"strength\":0.5"));
    assert!(attempts.contains("the crown is still enclosed"));
    // The next round proposes the same directions from the same tree, so it
    // buys nothing at all.
    assert_eq!(mock.evaluations, 5);
    assert_eq!(mock.sheet_calls, 1);
    assert_eq!(
        state.pause.as_ref().unwrap().reason,
        "bundle already tried; no new direction"
    );
}

#[test]
fn a_refused_sheet_answer_leaves_a_recoverable_attempt() {
    let (mut state, mut mock) = bundle_run();
    state.execute(&mut mock, &mut |_| Ok(())).unwrap();
    approve_one(&mut state, &mock);
    mock.sheet_error = true;
    state.execute(&mut mock, &mut |_| Ok(())).unwrap();

    assert_eq!(state.pending.as_deref(), Some("sheet review"));
    assert!(state
        .pause
        .as_ref()
        .unwrap()
        .reason
        .contains("reservation retained"));
    // Nothing was judged, so the round is over and every variant that would
    // have been on the sheet says what it cost.
    assert!(state
        .routes
        .iter()
        .any(|r| r == "sheet review failed; attempt charged"));
    let variants: Vec<&Trial> = state.trials.iter().filter(|t| t.bundle.is_some()).collect();
    assert_eq!(variants.len(), 4);
    assert!(variants
        .iter()
        .all(|t| t.reason.as_deref() == Some("review failed; attempt charged")));
    assert!(variants.iter().all(|t| t.sheet.is_none()));
    assert_eq!(
        state.current,
        Some(0),
        "a variant nobody judged was adopted"
    );

    // The attempt is still pending, so the run refuses to go on without the
    // owner's scoped recovery.
    let charged = serde_json::to_value(&state.budget).unwrap();
    state.pause = None;
    state.execute(&mut mock, &mut |_| Ok(())).unwrap();
    assert_eq!(
        state.pause.as_ref().unwrap().reason,
        "interrupted attempt; reservation retained"
    );
    assert_eq!(serde_json::to_value(&state.budget).unwrap(), charged);

    // After that recovery the same bundle is not drawn again, at any strength.
    state.pause = None;
    state.pending = None;
    mock.sheet_error = false;
    state.execute(&mut mock, &mut |_| Ok(())).unwrap();
    assert_eq!(mock.evaluations, 5, "the same bundle was drawn twice");
    assert_eq!(mock.sheet_calls, 0, "the same sheet was bought twice");
    assert_eq!(
        state.pause.as_ref().unwrap().reason,
        "bundle already tried; no new direction"
    );
}

#[test]
fn a_failed_progress_review_is_recorded_and_never_bought_again() {
    let (mut state, mut mock) = reviewed_three(vec![false, false, false], vec![]);
    state.execute(&mut mock, &mut |_| Ok(())).unwrap();
    approve_one(&mut state, &mock);
    mock.progress_error = true;
    state.execute(&mut mock, &mut |_| Ok(())).unwrap();

    assert_eq!(state.pending.as_deref(), Some("progress review"));
    assert_eq!(mock.evaluations, 2, "one candidate was drawn and reviewed");
    let failed = state.trials.last().unwrap();
    assert_eq!(failed.label, "twig_hang");
    assert_eq!(
        failed.reason.as_deref(),
        Some("review failed; attempt charged")
    );
    assert!(failed.progress.is_none(), "nothing was bound");

    // The owner's scoped recovery clears the attempt and refunds nothing.
    let charged = serde_json::to_value(&state.budget).unwrap();
    state.pause = None;
    state.pending = None;
    mock.progress_error = false;
    state.execute(&mut mock, &mut |_| Ok(())).unwrap();

    assert!(
        serde_json::to_value(&state.budget).unwrap()["visual_passes"]
            .as_u64()
            .unwrap()
            > charged["visual_passes"].as_u64().unwrap(),
        "the failed pass was refunded"
    );
    assert!(
        state
            .routes
            .iter()
            .any(|r| r.starts_with("repeat refused: twig_hang")),
        "the failed review was bought again: {:?}",
        state.routes
    );
    assert_eq!(
        state
            .trials
            .iter()
            .filter(|t| t.label == "twig_hang")
            .count(),
        1,
        "the same move was drawn a second time"
    );
    // The round ran to its normal end on the two moves that were left.
    assert_eq!(mock.progress_calls, 2);
    assert!(state
        .routes
        .iter()
        .any(|r| r == "visual stall; no candidate judged better"));
}

/// The whole point of the veto: bundles 2 and 3 of the beech run were graded
/// clear on crown shape by the sheet, and the closing all-view review reported
/// what the owner then called garbage. These drive that closing review.
fn adopting_run() -> (Run, Mock) {
    let (state, mut mock) = bundle_run();
    mock.sheets = vec![vec![did(&key(3), Movement::Clear)]];
    (state, mock)
}

#[test]
fn an_adoption_the_closing_review_does_not_fault_is_kept() {
    let (mut state, mut mock) = adopting_run();
    to_the_round(&mut state, &mut mock);

    assert_eq!(mock.side_effect_calls, 1, "the question is asked once");
    let adopted = &state.trials[state.current.unwrap()];
    assert_eq!(adopted.label, "bundle@1");
    assert!(adopted.vetoed.is_none());
    assert!(!state
        .routes
        .iter()
        .any(|r| r.starts_with("adoption rolled back")));
    assert!(state
        .routes
        .iter()
        .any(|r| r.contains("adoption kept") && r.contains("uncalibrated side-effect question")));
}

#[test]
fn a_required_cell_that_went_backwards_rolls_the_adoption_back_unasked() {
    let (mut state, mut mock) = adopting_run();
    // The baseline look and the post-approval look pass the cell; the closing
    // look after the move does not.
    mock.cell_status = vec![CellStatus::Pass, CellStatus::Pass, CellStatus::Fail];
    let before = state.effective.clone();
    to_the_round(&mut state, &mut mock);

    assert_eq!(mock.side_effect_calls, 0, "nobody was paid to confirm it");
    assert_eq!(state.current, Some(0), "the vetoed tree is still current");
    assert_eq!(state.effective, before, "the wire was not put back");
    let vetoed = state
        .trials
        .iter()
        .find(|t| t.vetoed.is_some())
        .expect("no trial records the rollback");
    let reasons = &vetoed.vetoed.as_ref().unwrap().reasons;
    assert!(
        reasons[0].contains("went from pass to fail") && reasons[0].contains("crown"),
        "{reasons:?}"
    );
    assert!(vetoed.vetoed.as_ref().unwrap().ledger.is_none());
    assert!(state
        .routes
        .iter()
        .any(|r| r.starts_with("adoption rolled back:")));
    // The move counts as tried, so the next round draws nothing.
    assert_eq!(mock.evaluations, 5);
    assert_eq!(
        state.pause.as_ref().unwrap().reason,
        "bundle already tried; no new direction"
    );
}

#[test]
fn a_new_defect_only_the_text_reports_rolls_the_adoption_back() {
    let (mut state, mut mock) = adopting_run();
    mock.side_effect_answer = Some(("new_defect".into(), 0.9));
    let (before, overrides) = (state.effective.clone(), state.overrides.clone());
    to_the_round(&mut state, &mut mock);

    assert_eq!(mock.side_effect_calls, 1);
    assert_eq!(state.current, Some(0));
    assert_eq!(state.effective, before);
    assert_eq!(state.overrides, overrides);
    assert_eq!(state.visual.as_ref().unwrap().identity, "candidate1");
    let veto = state
        .trials
        .iter()
        .find_map(|t| t.vetoed.as_ref())
        .expect("no trial records the rollback");
    assert_eq!(veto.ledger.as_deref(), Some("jev:side-effects"));
    assert!(veto.reasons[0].contains("uncalibrated side-effect question"));
    // What the move broke reaches whoever proposes the next one.
    let projected = telperion_jev::tuning::judgments::proposal_state(&state);
    let attempts = serde_json::to_string(&projected["attempts_from_this_candidate"]).unwrap();
    assert!(attempts.contains("rolled_back"), "{attempts}");
    let summary =
        serde_json::to_string(&telperion_jev::tuning::judgments::summary(&state)).unwrap();
    assert!(summary.contains("rolled_back"));
    assert!(state
        .judgment_inputs
        .iter()
        .any(|i| i.label == "uncalibrated side-effect question"));
}

#[test]
fn a_new_defect_below_the_threshold_keeps_the_adoption_and_records_what_was_said() {
    let (mut state, mut mock) = adopting_run();
    mock.side_effect_answer = Some(("new_defect".into(), 0.3));
    to_the_round(&mut state, &mut mock);

    assert_eq!(mock.side_effect_calls, 1);
    assert_eq!(state.trials[state.current.unwrap()].label, "bundle@1");
    assert!(state.trials.iter().all(|t| t.vetoed.is_none()));
    assert!(
        state
            .routes
            .iter()
            .any(|r| r.contains("adoption kept") && r.contains("new_defect") && r.contains("0.3")),
        "the raw answer was not recorded: {:?}",
        state.routes
    );
}
