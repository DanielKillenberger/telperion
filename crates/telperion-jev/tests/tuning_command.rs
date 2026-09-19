use serde_json::{json, Value};
use std::{fs, path::Path};
use telperion_jev::tuning::{command, live::Config};

fn write(path: &Path, value: &Value) {
    fs::write(path, serde_json::to_vec_pretty(value).unwrap()).unwrap();
}
fn fixture(root: &Path) -> Value {
    let asset = root.join("asset");
    fs::write(&asset, "fixture fingerprint").unwrap();
    let absent = root.join("missing-calibration.json");
    let validation = json!({"manifest":absent,"result":absent});
    json!({"preset":"european-beech","seed":1,"initial_overrides":{},"owner_notes":"owner notes",
        "dials":[{"id":"limbs","path":"/skeleton/habit/lateralsPerStation","meaning":"limbs","min":1.,"max":4.,"integer":true,"small":1.,"substantial":2.}],
        "measure_binary":asset,"profiles":asset,"profile_id":"european-beech",
        "matched":{"headless":asset,"compare_script":asset,"references":asset,"refs":root,"catalogue":root,"scratch":root.join("scratch"),"height":1440,"numeric_references":["whole"]},
        "vision":{"program":"not-executed","args":[],"model":"mock","effort":"medium","timeout_seconds":10,"ledger":root.join("ledger")},
        "references":[],"required":[{"item":"crown","view":"whole","seed":1},{"item":"crown","view":"whole","seed":42}],
        "checklist":"recognizable species at established quality","quality_anchors":[],
        "adjustments":validation,"direction":validation,"continuation":validation,"visual_validation":validation,
        "vision_protocol":asset,"convergence_run":null,"judgment_model":"jev-1.13.0","ledger":root.join("ledger"),
        "budget":{"evaluations":0,"images":0,"tokens":0,"rounds":0,"max_evaluations":13,"max_images":52,"max_tokens":200000,"max_rounds":3}})
}
#[test]
fn cli_pauses_without_key_and_scoped_changed_revision_preserves_spend() {
    let root = std::env::temp_dir().join(format!(
        "tuning-command-{}",
        telperion_jev::ledger::new_entry_id()
    ));
    fs::create_dir(&root).unwrap();
    let config_path = root.join("config.json");
    let out = root.join("out");
    let mut config = fixture(&root);
    write(&config_path, &config);
    assert!(command::run(&config_path, &out, None)
        .unwrap_err()
        .contains("calibration prerequisite"));
    let path = out.join("run.json");
    let mut saved: Value = serde_json::from_slice(&fs::read(&path).unwrap()).unwrap();
    assert_eq!(saved["machine_ready"], false);
    saved["budget"]["tokens"] = json!(17);
    write(&path, &saved);
    config["owner_notes"] = json!("verified repair needs fresh assessment");
    write(&config_path, &config);
    let next: Config = serde_json::from_value(config.clone()).unwrap();
    let decision = root.join("decision.json");
    let mut d = json!({"pause_id":saved["pause"]["id"],"identity":saved["identity"],
        "action":saved["pause"]["basis"]["proposed_action"],"by":"test owner","rationale":"verified scoped repair"});
    write(&decision, &d);
    assert!(command::run(&config_path, &out, Some(&decision))
        .unwrap_err()
        .contains("next_identity"));
    d["next_identity"] = json!(next.identity().unwrap());
    write(&decision, &d);
    assert!(command::run(&config_path, &out, Some(&decision))
        .unwrap_err()
        .contains("calibration prerequisite"));
    let saved: Value = serde_json::from_slice(&fs::read(&path).unwrap()).unwrap();
    assert_eq!(saved["budget"]["tokens"], 17);
    assert_eq!(saved["identity"], d["next_identity"]);
    assert_eq!(saved["visual"], Value::Null);
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn stale_lock_file_and_partial_temp_do_not_strand_interrupted_run() {
    let root = std::env::temp_dir().join(format!(
        "tuning-recovery-{}",
        telperion_jev::ledger::new_entry_id()
    ));
    fs::create_dir(&root).unwrap();
    let config_path = root.join("config.json");
    let out = root.join("out");
    write(&config_path, &fixture(&root));
    command::run(&config_path, &out, None).unwrap_err();
    let path = out.join("run.json");
    let mut state: Value = serde_json::from_slice(&fs::read(&path).unwrap()).unwrap();
    state["pending"] = json!("baseline");
    state["pause"] = Value::Null;
    state["budget"]["evaluations"] = json!(1);
    state["budget"]["images"] = json!(4);
    write(&path, &state);
    fs::write(out.join("run.pending-old"), "incomplete").unwrap();
    assert!(command::run(&config_path, &out, None)
        .unwrap_err()
        .contains("interruption recorded"));
    let state: Value = serde_json::from_slice(&fs::read(&path).unwrap()).unwrap();
    let decision = root.join("decision.json");
    write(
        &decision,
        &json!({"pause_id":state["pause"]["id"],"identity":state["identity"],
        "action":"recover interrupted attempt","by":"test owner","rationale":"measurement process ended; keep reservation","recover_interrupted":true}),
    );
    assert!(command::run(&config_path, &out, Some(&decision))
        .unwrap_err()
        .contains("calibration prerequisite"));
    let state: Value = serde_json::from_slice(&fs::read(&path).unwrap()).unwrap();
    assert_eq!(state["budget"]["evaluations"], 1);
    assert_eq!(state["budget"]["images"], 4);
    assert_eq!(state["pending"], Value::Null);
    fs::remove_dir_all(root).unwrap();
}
