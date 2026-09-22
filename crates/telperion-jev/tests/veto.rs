//! Where an adoption is put back to, and what two assessments say went
//! backwards between them. Pure rules: no dispatch, no render, no model.
use serde_json::json;
use telperion_jev::tuning::{
    state::{Cell, CellStatus, TraitStatus, Visual},
    veto,
};

fn cell(item: &str) -> Cell {
    Cell {
        item: item.into(),
        view: "whole".into(),
        seed: 1,
    }
}

fn visual(cells: &[(&str, CellStatus)], coverage: &[(&str, CellStatus)]) -> Visual {
    Visual {
        identity: "candidate".into(),
        model: "mock".into(),
        ledger: "receipt".into(),
        cells: cells
            .iter()
            .map(|(item, status)| (cell(item), *status))
            .collect(),
        defects: vec![],
        observations: vec![],
        findings: vec![],
        joint: None,
        coverage: coverage
            .iter()
            .map(|(trait_id, status)| TraitStatus {
                trait_id: (*trait_id).into(),
                status: *status,
            })
            .collect(),
    }
}

#[test]
fn only_a_disposition_that_went_backwards_is_a_reason_to_roll_back() {
    use CellStatus::{Fail, Pass, Unknown};
    let required = vec![cell("crown"), cell("bark")];
    let before = visual(
        &[("crown", Pass), ("bark", Fail)],
        &[
            ("habit", Pass),
            ("twigs", Pass),
            ("bark", Unknown),
            ("leaf", Fail),
            ("stable", Pass),
        ],
    );
    let after = visual(
        &[("crown", Fail), ("bark", Pass)],
        &[
            ("habit", Fail),
            ("twigs", Unknown),
            ("bark", Fail),
            ("leaf", Unknown),
            ("stable", Pass),
        ],
    );
    let reasons = veto::worsened(&before, &after, &required);
    assert_eq!(reasons.len(), 3, "{reasons:?}");
    assert!(reasons[0].contains("required cell crown") && reasons[0].contains("pass to fail"));
    assert!(
        reasons[1].contains("habit") && reasons[1].contains("Pass") && reasons[1].contains("Fail")
    );
    assert!(reasons[2].contains("bark") && reasons[2].contains("Unknown"));
    assert!(
        !reasons.iter().any(|r| r.contains("twigs")),
        "a pass that became unknown is the reviewer declining to judge, not a regression"
    );
    // A cell that was already failing, a trait that was already failing and a
    // trait nothing changed are not reasons.
    assert!(!reasons
        .iter()
        .any(|r| r.contains("leaf") || r.contains("stable")));
    assert!(!reasons.iter().any(|r| r.contains("required cell bark")));
    assert!(veto::worsened(&before, &before, &required).is_empty());
}

#[test]
fn the_restore_point_puts_every_field_back_as_it_was() {
    let mut state: telperion_jev::tuning::engine::Run = serde_json::from_value(json!({
        "identity":"run","preset":"european-beech","seed":1,
        "effective":{"skeleton":{"twigs":{"hang":0.25}}},"overrides":{},
        "dials":[],"owner_notes":"notes","required":[],
        "budget":{"evaluations":0,"images":0,"tokens":0,"rounds":0,"max_evaluations":1,
            "max_images":1,"max_tokens":1,"max_rounds":1},
        "usage_known":true,"trials":[],"current":null,
        "visual":serde_json::to_value(visual(&[("crown",CellStatus::Pass)],&[])).unwrap(),
        "pause":null,"machine_ready":false,"pending":null,"routes":[]
    }))
    .unwrap();
    state.current = Some(0);
    let restore = veto::restore_point(&state);
    let (effective, overrides) = (state.effective.clone(), state.overrides.clone());

    state.current = Some(9);
    state.effective = json!({"skeleton":{"twigs":{"hang":0.75}}});
    state.overrides = json!({"skeleton":{"twigs":{"hang":0.75}}});
    state.visual = Some(visual(&[("crown", CellStatus::Fail)], &[]));
    state.machine_ready = true;
    state.reviewer_passed_unqualified = true;
    restore.apply(&mut state);

    assert_eq!(state.current, Some(0));
    assert_eq!(state.effective, effective);
    assert_eq!(state.overrides, overrides);
    assert_eq!(state.visual.unwrap().cells[0].1, CellStatus::Pass);
    assert!(!state.machine_ready && !state.reviewer_passed_unqualified);
}
