use serde_json::json;
use telperion_jev::tuning::actions::{Action, Dial};
use telperion_jev::tuning::state::{distance, ready, Budget, Cell, CellStatus, Visual};

#[test]
fn weighted_score_penalizes_unreadable_measurements() {
    assert_eq!(
        distance(&[(Some(12.), 10., 1.), (None, 4., 3.)]).unwrap(),
        0.8
    );
    assert!(distance(&[]).is_err());
    assert!(distance(&[(Some(1.), 0., 1.)]).is_err());
}

#[test]
fn readiness_requires_every_checklist_view_seed_and_no_defects() {
    let cells = vec![
        Cell {
            item: "bark".into(),
            view: "base".into(),
            seed: 1,
        },
        Cell {
            item: "bark".into(),
            view: "base".into(),
            seed: 42,
        },
    ];
    let mut assessment = Visual {
        coverage: vec![],
        identity: "v1".into(),
        model: "vision".into(),
        ledger: "record".into(),
        cells: vec![(cells[0].clone(), CellStatus::Pass)],
        defects: vec![],
        observations: vec![],
        findings: vec![],
        joint: None,
        known_gaps: vec![],
    };
    assert!(!ready(&cells, "v1", &assessment));
    assessment.cells.push((cells[1].clone(), CellStatus::Pass));
    assert!(ready(&cells, "v1", &assessment));
    assert!(!ready(&cells, "v2", &assessment));
    assessment.defects.push("too dense".into());
    assert!(!ready(&cells, "v1", &assessment));
}

/// A budget counts every spend and refuses none: what ends a revision that
/// keeps nothing is the no-progress stop, never a cap (fn-149).
#[test]
fn a_budget_counts_every_spend_and_refuses_none() {
    let mut budget: Budget = serde_json::from_value(json!({})).unwrap();
    budget.reserve(1_000, 4_000, u64::MAX / 2, 1_000);
    budget.reserve(1, 0, u64::MAX, 0);
    budget.reserve_visual();
    assert_eq!(
        serde_json::to_value(&budget).unwrap(),
        json!({"evaluations": 1001, "images": 4000, "tokens": u64::MAX,
               "rounds": 1000, "visual_passes": 1})
    );
    let caps = json!({"max_tokens": 10});
    assert!(
        serde_json::from_value::<Budget>(caps).is_err(),
        "caps are gone"
    );
}

#[test]
fn authored_actions_preserve_integer_bounds_and_abstention() {
    let dial = Dial {
        id: "limbs".into(),
        path: "/skeleton/habit/lateralsPerStation".into(),
        meaning: "limbs per station".into(),
        min: 1.,
        max: 4.,
        integer: true,
        small: 1.,
        substantial: 2.,
        group: None,
        score_visible: None,
        meaning_basis: None,
        range_basis: None,
        source: None,
        preset_span: None,
        cap: None,
    };
    assert_eq!(
        dial.value(2., Action::SmallDecrease).unwrap(),
        Some(json!(1))
    );
    assert!(dial.value(2., Action::SubstantialDecrease).is_err());
    assert_eq!(dial.value(2., Action::Hold).unwrap(), None);
    assert_eq!(dial.value(2., Action::InsufficientEvidence).unwrap(), None);
    assert!(dial.value(1.5, Action::SmallIncrease).is_err());
}

#[test]
fn measurement_gates_accept_context_but_stop_failed_candidates_before_render() {
    use std::{cell::Cell as Counter, fs};
    use telperion_jev::{
        pipeline::render::{Measured, Measurer, RenderError},
        tuning::evaluation::{evaluate, gates, Comparison, MatchedRenderer},
    };
    let mut event = json!({"event":"completed","numeric_status":"pass",
        "checks":{"height":{"status":"pass"},"foliage":{"status":"contextual"}},
        "metrics":{"growth":{"node_capped":false,"level_capped":false,"attraction_capped":false}}});
    assert!(gates(&event.to_string()).is_ok());
    event["numeric_status"] = json!("fail");
    // What `species_measure` writes for a gate outside its range.
    event["checks"]["height"] = json!({"status":"fail","target":{"range":[18.0,24.0]},
        "actual":{"status":"measured","value":31.4},
        "reason":"outside target or target unavailable"});
    struct Measure(std::path::PathBuf);
    impl Measurer for Measure {
        fn measure(&self, _: &str, _: u32, _: &serde_json::Value) -> Result<Measured, RenderError> {
            Ok(Measured {
                metrics: json!({}),
                receipt_path: self.0.clone(),
            })
        }
    }
    struct Renderer(Counter<usize>);
    impl MatchedRenderer for Renderer {
        fn render(
            &self,
            _: &str,
            _: u32,
            _: &serde_json::Value,
            _: &str,
        ) -> Result<Vec<Comparison>, String> {
            self.0.set(self.0.get() + 1);
            Err("still shows no tree".into())
        }
    }
    let path = std::env::temp_dir().join(format!(
        "tuning-test-{}",
        telperion_jev::ledger::new_entry_id()
    ));
    fs::write(&path, event.to_string()).unwrap();
    let measure = Measure(path.clone());
    let renderer = Renderer(Counter::new(0));
    let failed = evaluate(
        &measure,
        &renderer,
        "european-beech",
        "revision",
        1,
        0,
        "test",
        json!({}),
        None,
    );
    assert!(!failed.feasible);
    assert_eq!(renderer.0.get(), 0);
    // The router is told which gate, what it measured and what it wanted.
    let reason = failed.reason.clone().unwrap();
    assert!(
        reason.starts_with("numeric gate failed: ")
            && reason.contains("height 31.4")
            && reason.contains("outside 18.0 to 24.0")
            && reason.contains("(fail)"),
        "{reason}"
    );
    assert!(
        !reason.contains("foliage"),
        "a contextual metric is not a failing gate: {reason}"
    );
    assert_eq!(failed.measurement["checks"]["height"]["status"], "fail");
    event["metrics"]["growth"]["node_capped"] = json!(true);
    assert!(gates(&event.to_string())
        .unwrap_err()
        .contains("node_capped"));
    event["metrics"]["growth"]["node_capped"] = json!(false);
    // An unassessed gate under a passing numeric status is named too.
    event["numeric_status"] = json!("pass");
    event["checks"]["height"] = json!({"status":"unassessed","target":{"range":[18.0,24.0]},
        "actual":{"status":"missing"},"reason":"measurement missing, ambiguous or estimated"});
    let unassessed = gates(&event.to_string()).unwrap_err();
    assert!(
        unassessed.starts_with("failed or unassessed gate: ")
            && unassessed.contains("height unmeasured")
            && unassessed.contains("(unassessed)"),
        "{unassessed}"
    );
    event["checks"]["height"] = json!({"status":"pass"});
    fs::write(&path, event.to_string()).unwrap();
    let empty = evaluate(
        &measure,
        &renderer,
        "european-beech",
        "revision",
        1,
        0,
        "test",
        json!({}),
        None,
    );
    assert!(!empty.feasible);
    assert_eq!(empty.reason.as_deref(), Some("still shows no tree"));
    let invalid = evaluate(
        &measure,
        &renderer,
        "european-beech",
        "revision",
        1,
        0,
        "invalid",
        json!({"unknown_row":4}),
        None,
    );
    assert!(!invalid.feasible);
    assert_eq!(renderer.0.get(), 1);
    fs::remove_file(path).unwrap();
}
