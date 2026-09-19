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

#[test]
fn scoped_cap_extension_reuses_only_unchanged_verified_evidence() {
    let root = std::env::temp_dir().join(format!(
        "tuning-reuse-{}",
        telperion_jev::ledger::new_entry_id()
    ));
    fs::create_dir(&root).unwrap();
    let photo = root.join("photo");
    fs::write(&photo, "photo bytes").unwrap();
    let image = json!({"path":photo,"sha256":telperion_jev::sha256_hex(b"photo bytes"),"view":"whole","seed":1});
    let mut config = fixture(&root);
    config["references"] = json!([image]);
    config["quality_anchors"] = json!([{"image":image,"provenance":"test","scope":"finish"}]);
    let cfg = root.join("config.json");
    let out = root.join("out");
    write(&cfg, &config);
    command::run(&cfg, &out, None).unwrap_err();
    let path = out.join("run.json");
    let mut original: Value = serde_json::from_slice(&fs::read(&path).unwrap()).unwrap();
    original["budget"]["tokens"] = json!(123);
    original["current"] = json!(0);
    original["trials"] = json!([{"key":"trial","identity":original["identity"],"seed":1,"round":0,"label":"baseline","overrides":{},"ledger":null,"feasible":true,"reason":null,"measurement":{},"comparisons":[{"reference":"whole","reference_weight":1.0,"metric_weights":[1.,1.,1.,1.,1.],"target":[1.,1.,1.,1.,1.],"observed":[1.,1.,1.,1.,1.],"images":[image]}],"score":0.2,"seconds":1.0}]);
    original["visual"] =
        json!({"identity":"trial","model":"mock","ledger":"test","cells":[],"defects":["defect"]});
    config["budget"]["max_tokens"] = json!(205000);
    write(&cfg, &config);
    let next: Config = serde_json::from_value(config.clone()).unwrap();
    let decision = root.join("decision.json");
    let d = json!({"pause_id":original["pause"]["id"],"identity":original["identity"],"action":original["pause"]["basis"]["proposed_action"],"by":"test owner","rationale":"policy-only scoped extension","next_identity":next.identity().unwrap(),"preserve_evidence":true,"token_cap_extension":{"previous":200000,"next":205000}});
    for variant in [
        "wrong_cap",
        "no_extension",
        "artifact",
        "model",
        "trial_identity",
        "photo",
        "valid",
    ] {
        let mut state = original.clone();
        let mut choice = d.clone();
        fs::write(&photo, "photo bytes").unwrap();
        fs::write(root.join("asset"), "fixture fingerprint").unwrap();
        match variant {
            "wrong_cap" => choice["token_cap_extension"]["previous"] = json!(199999),
            "no_extension" => choice["token_cap_extension"] = Value::Null,
            "artifact" => {
                fs::write(root.join("asset"), "changed render binary").unwrap();
            }
            "model" => state["visual"]["model"] = json!("other"),
            "trial_identity" => state["trials"][0]["identity"] = json!("stale"),
            "photo" => {
                fs::write(&photo, "changed photo").unwrap();
            }
            _ => {}
        }
        write(&path, &state);
        write(&decision, &choice);
        let error = command::run(&cfg, &out, Some(&decision)).unwrap_err();
        if variant == "valid" {
            assert!(error.contains("calibration prerequisite"), "{error}");
            let saved: Value = serde_json::from_slice(&fs::read(&path).unwrap()).unwrap();
            assert_eq!(saved["budget"]["tokens"], 123);
            assert_eq!(saved["budget"]["max_tokens"], 205000);
            assert_eq!(saved["current"], 0);
            assert_eq!(saved["visual"]["identity"], "trial");
        } else {
            assert!(
                !error.contains("calibration prerequisite"),
                "{variant}: {error}"
            );
        }
    }
    fs::remove_dir_all(root).unwrap();
}
