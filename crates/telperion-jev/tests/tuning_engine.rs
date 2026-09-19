use serde_json::json;
use telperion_jev::tuning::{
    actions::{Action, Dial},
    continuation::{Assessment, Basis},
    engine::{Answer, Proposal, Run, Services},
    evaluation::Trial,
    state::{Budget, Cell, CellStatus, Visual},
};

struct Mock {
    evaluations: u64,
    routes: u64,
    visuals: u64,
    capability: bool,
}
impl Services for Mock {
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
                cells: vec![(
                    cell(),
                    if self.visuals == 3 {
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
        Ok(Answer {
            tokens: Some(20),
            value: Assessment {
                identity: basis.identity.clone(),
                ledger: "jev:1".into(),
                tractability: "supported".into(),
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
    fn route(&mut self, _: &Run) -> Result<Answer<String>, String> {
        self.routes += 1;
        Ok(Answer {
            tokens: Some(20),
            value: if self.capability {
                "new_capability"
            } else {
                "tuning"
            }
            .into(),
        })
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
        }],
        owner_notes: "irregular outline".into(),
        required: vec![cell()],
        budget: Budget {
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
    }
}

#[test]
fn numeric_wins_keep_refining_until_visual_cells_pass() {
    let mut state = run();
    let mut mock = Mock {
        evaluations: 0,
        routes: 0,
        visuals: 0,
        capability: false,
    };
    let mut checkpoints = vec![];
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
        evaluations: 0,
        routes: 0,
        visuals: 0,
        capability: true,
    };
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
    let mut mock = Mock {
        evaluations: 0,
        routes: 0,
        visuals: 0,
        capability: false,
    };
    state.budget.max_rounds = 0;
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
