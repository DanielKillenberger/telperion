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
fn priority_resume_refuses_missing_or_stale_owner_decision_without_mutating_journal() {
    use telperion_jev::tuning::{
        engine::Run,
        evaluation::Image,
        priority::{Checkpoint, Evidence},
        state::Visual,
    };
    let root = std::env::temp_dir().join(telperion_jev::ledger::new_entry_id());
    fs::create_dir(&root).unwrap();
    let config_value = fixture(&root);
    let config: Config = serde_json::from_value(config_value.clone()).unwrap();
    let config_path = root.join("config.json");
    write(&config_path, &config_value);
    let out = root.join("out");
    command::run(&config_path, &out, None).unwrap_err();
    let path = out.join("run.json");
    let mut state: Run = serde_json::from_slice(&fs::read(&path).unwrap()).unwrap();
    let image = Image {
        path: root.join("asset"),
        sha256: telperion_jev::sha256_hex(&fs::read(root.join("asset")).unwrap()),
        view: "whole".into(),
        seed: 1,
    };
    let visual:Visual=serde_json::from_value(json!({"identity":"candidate","model":"mock","ledger":"fixture","cells":[],"defects":[],"findings":[]})).unwrap();
    let checkpoint = Checkpoint::new(
        &state.identity,
        &config.priority_scope(&state),
        visual,
        vec![
            Evidence {
                id: "render-0".into(),
                role: "render".into(),
                image: image.clone(),
            },
            Evidence {
                id: "reference-0".into(),
                role: "reference".into(),
                image,
            },
        ],
    )
    .unwrap();
    state.priority_checkpoints.push(checkpoint.clone());
    state.pause.as_mut().unwrap().basis.proposed_action = "approve gap priorities".into();
    write(&path, &serde_json::to_value(&state).unwrap());
    let original = fs::read(&path).unwrap();
    let pause = state.pause.as_ref().unwrap();
    let mut decision = json!({"pause_id":pause.id,"identity":state.identity,"action":"approve gap priorities","by":"test owner","rationale":"reviewed priorities"});
    let decision_path = root.join("decision.json");
    write(&decision_path, &decision);
    assert!(command::run(&config_path, &out, Some(&decision_path))
        .unwrap_err()
        .contains("priority approval required"));
    assert_eq!(fs::read(&path).unwrap(), original);
    decision["priority_approval"] =
        json!({"checkpoint_sha256":"stale","scope_sha256":checkpoint.scope_sha256,"ordered":[]});
    write(&decision_path, &decision);
    assert!(command::run(&config_path, &out, Some(&decision_path))
        .unwrap_err()
        .contains("stale owner priority"));
    assert_eq!(fs::read(&path).unwrap(), original);
    fs::remove_dir_all(root).unwrap();
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
    config["budget"]["max_rounds"] = json!(4);
    write(&cfg, &config);
    let next: Config = serde_json::from_value(config.clone()).unwrap();
    let decision = root.join("decision.json");
    let source = root.join("diagnosis-source.txt");
    fs::write(&source, "Measured node cap prevented rendering.").unwrap();
    let diagnosis = json!({"target_identity":next.identity().unwrap(),"author":"diagnostic worker","model":"mock-reasoner","findings":[{"claim":"Prior trial was resource limited, not visually judged","source":source,"sha256":telperion_jev::sha256_hex(b"Measured node cap prevented rendering."),"excerpt":"node cap prevented rendering"}]});
    let mut d = json!({"pause_id":original["pause"]["id"],"identity":original["identity"],"action":original["pause"]["basis"]["proposed_action"],"by":"test owner","rationale":"policy-only scoped extension","next_identity":next.identity().unwrap(),"preserve_evidence":true,"token_cap_extension":{"previous":200000,"next":205000},"round_cap_extension":{"previous":3,"next":4},"diagnosis":diagnosis});
    let external_path = root.join("external.json");
    let q = json!({"q":{"type":"noul"}});
    let state_hash = "a".repeat(64);
    let external_identity = telperion_jev::ledger::derived_identity(&state_hash, &q, "mock");
    let external = json!({"id":"external","tool":"study","model":"mock","identity":external_identity,"state_sha256":state_hash,"questions":q,"answers":{"q":{"type":"noul","noul":0.9}},"usage":{"input_tokens":7,"output_tokens":3},"elapsed_ms":1,"recorded_at":"now","source":null});
    write(&external_path, &external);
    d["external_usage"] = json!({"previous_tokens":123,"next_tokens":133,"reason":"approved study","ledgers":[{"path":external_path,"sha256":telperion_jev::sha256_hex(&fs::read(&external_path).unwrap()),"id":"external","tool":"study","model":"mock","identity":external_identity}]});
    for variant in [
        "wrong_cap",
        "no_extension",
        "no_round_extension",
        "wrong_external_sum",
        "artifact",
        "model",
        "trial_identity",
        "photo",
        "missing_author",
        "missing_model",
        "empty_findings",
        "malformed_hash",
        "wrong_hash",
        "wrong_excerpt",
        "wrong_target",
        "valid",
    ] {
        let mut state = original.clone();
        let mut choice = d.clone();
        fs::write(&photo, "photo bytes").unwrap();
        fs::write(root.join("asset"), "fixture fingerprint").unwrap();
        match variant {
            "wrong_cap" => choice["token_cap_extension"]["previous"] = json!(199999),
            "no_extension" => choice["token_cap_extension"] = Value::Null,
            "no_round_extension" => choice["round_cap_extension"] = Value::Null,
            "wrong_external_sum" => choice["external_usage"]["next_tokens"] = json!(132),
            "artifact" => {
                fs::write(root.join("asset"), "changed render binary").unwrap();
            }
            "model" => state["visual"]["model"] = json!("other"),
            "trial_identity" => state["trials"][0]["identity"] = json!("stale"),
            "photo" => {
                fs::write(&photo, "changed photo").unwrap();
            }
            "missing_author" => choice["diagnosis"]["author"] = json!(""),
            "missing_model" => choice["diagnosis"]["model"] = json!(""),
            "empty_findings" => choice["diagnosis"]["findings"] = json!([]),
            "malformed_hash" => choice["diagnosis"]["findings"][0]["sha256"] = json!("bad"),
            "wrong_hash" => choice["diagnosis"]["findings"][0]["sha256"] = json!("0".repeat(64)),
            "wrong_excerpt" => {
                choice["diagnosis"]["findings"][0]["excerpt"] = json!("not in source")
            }
            "wrong_target" => choice["diagnosis"]["target_identity"] = json!("stale"),
            _ => {}
        }
        write(&path, &state);
        write(&decision, &choice);
        let error = command::run(&cfg, &out, Some(&decision)).unwrap_err();
        if variant == "valid" {
            assert!(error.contains("calibration prerequisite"), "{error}");
            let saved: Value = serde_json::from_slice(&fs::read(&path).unwrap()).unwrap();
            assert_eq!(saved["budget"]["tokens"], 133);
            assert_eq!(saved["budget"]["max_tokens"], 205000);
            assert_eq!(saved["budget"]["max_rounds"], 4);
            assert_eq!(saved["current"], 0);
            assert_eq!(saved["visual"]["identity"], "trial");
            assert_eq!(saved["authorizations"][0]["diagnosis"], diagnosis);
        } else {
            let unchanged: Value = serde_json::from_slice(&fs::read(&path).unwrap()).unwrap();
            assert_eq!(unchanged, state);
            assert!(
                !error.contains("calibration prerequisite"),
                "{variant}: {error}"
            );
        }
    }
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn experimental_revision_amendment_is_scoped_and_preserves_history() {
    let root = std::env::temp_dir().join(format!(
        "tuning-amend-{}",
        telperion_jev::ledger::new_entry_id()
    ));
    fs::create_dir(&root).unwrap();
    let cfg = root.join("config.json");
    let out = root.join("out");
    let mut config = fixture(&root);
    write(&cfg, &config);
    command::run(&cfg, &out, None).unwrap_err();
    let path = out.join("run.json");
    let mut state: Value = serde_json::from_slice(&fs::read(&path).unwrap()).unwrap();
    state["budget"]["tokens"] = json!(170312);
    state["budget"]["rounds"] = json!(1);
    state["budget"]["max_rounds"] = json!(1);
    state["budget"]["visual_passes"] = json!(7);
    state["budget"]["max_visual_passes"] = json!(8);
    state["visual"] =
        json!({"identity":"old","model":"mock","ledger":"old","cells":[],"defects":[]});
    let overlay = json!({"skeleton":{"growth":{"maxNodes":1000000}}});
    config["initial_overrides"] = overlay.clone();
    config["budget"]["max_tokens"] = json!(270000);
    config["budget"]["max_rounds"] = json!(2);
    config["budget"]["max_visual_passes"] = json!(9);
    write(&cfg, &config);
    let next: Config = serde_json::from_value(config.clone()).unwrap();
    let identity = next.identity().unwrap();
    let authority = json!({"purpose":"bounded experiment","reason":"owner approved repaired generator","next_identity":identity,"max_tokens":270000,"max_rounds":2,"max_evaluations":13,"max_images":52,"max_visual_passes":9});
    let d = json!({"pause_id":state["pause"]["id"],"identity":state["identity"],"action":state["pause"]["basis"]["proposed_action"],"by":"owner","rationale":"scoped repaired baseline","next_identity":identity,"token_cap_extension":{"previous":200000,"next":270000},"round_cap_extension":{"previous":1,"next":2},"visual_cap_extension":{"previous":8,"next":9},"baseline_amendment":{"previous":{},"next":overlay},"experimental_pilot":authority});
    let decision = root.join("decision.json");
    for variant in [
        "no_amendment",
        "wrong_round",
        "wrong_authority",
        "preserve",
        "valid",
    ] {
        let mut choice = d.clone();
        match variant {
            "no_amendment" => choice["baseline_amendment"] = Value::Null,
            "wrong_round" => choice["round_cap_extension"]["previous"] = json!(0),
            "wrong_authority" => choice["experimental_pilot"]["max_tokens"] = json!(999999),
            "preserve" => choice["preserve_evidence"] = json!(true),
            _ => {}
        }
        write(&path, &state);
        write(&decision, &choice);
        let error = command::run(&cfg, &out, Some(&decision)).unwrap_err();
        if variant == "valid" {
            assert!(error.contains("calibration prerequisite"), "{error}");
            let saved: Value = serde_json::from_slice(&fs::read(&path).unwrap()).unwrap();
            assert_eq!(saved["budget"]["tokens"], 170312);
            assert_eq!(saved["budget"]["rounds"], 1);
            assert_eq!(saved["budget"]["visual_passes"], 7);
            assert_eq!(saved["overrides"], overlay);
            assert_eq!(saved["visual"], Value::Null);
            assert_eq!(saved["current"], Value::Null);
            assert_eq!(saved["authorizations"].as_array().unwrap().len(), 1);
            let parsed: telperion_jev::tuning::engine::Run = serde_json::from_value(saved).unwrap();
            assert!(parsed.pilot_authority().is_ok());
        } else {
            assert!(
                !error.contains("calibration prerequisite"),
                "{variant}:{error}"
            );
        }
    }
    let mut accepted: Value = serde_json::from_slice(&fs::read(&path).unwrap()).unwrap();
    accepted["overrides"]["skeleton"]["habit"] = json!({"crookedness":4.0});
    write(&path, &accepted);
    let mut resume = d.clone();
    resume["pause_id"] = accepted["pause"]["id"].clone();
    resume["identity"] = accepted["identity"].clone();
    resume["action"] = accepted["pause"]["basis"]["proposed_action"].clone();
    for field in [
        "token_cap_extension",
        "round_cap_extension",
        "visual_cap_extension",
        "baseline_amendment",
    ] {
        resume[field] = Value::Null;
    }
    write(&decision, &resume);
    let error = command::run(&cfg, &out, Some(&decision)).unwrap_err();
    assert!(error.contains("calibration prerequisite"), "{error}");
    let saved: Value = serde_json::from_slice(&fs::read(&path).unwrap()).unwrap();
    assert_eq!(saved["overrides"], accepted["overrides"]);
    assert_eq!(saved["authorizations"].as_array().unwrap().len(), 2);
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn preflight_plans_the_sequence_without_writing_state_or_taking_a_lock() {
    let root = std::env::temp_dir().join(format!(
        "tuning-preflight-{}",
        telperion_jev::ledger::new_entry_id()
    ));
    fs::create_dir(&root).unwrap();
    let cfg = root.join("config.json");
    let out = root.join("out");
    write(&cfg, &fixture(&root));
    // A fresh run has no state at all: preflight must still plan.
    let plan = telperion_jev::tuning::preflight::plan(&cfg, &out, None).unwrap();
    assert!(!out.exists(), "preflight created the output directory");
    assert_eq!(plan["calibration_verified"], false);
    assert_eq!(plan["pilot_authority"], false);
    assert_eq!(plan["estimate_from_current_state"]["candidate"], true);
    let steps = plan["steps"].as_array().unwrap();
    assert_eq!(steps.len(), 5);
    assert_eq!(steps[0]["step"], "baseline evaluation");
    assert_eq!(steps[3]["evaluations"], 4);
    // Totals are the engine's own estimators summed against the opening caps.
    assert_eq!(plan["totals"]["evaluations"]["needed"], 5);
    assert_eq!(plan["totals"]["visual_passes"]["needed"], 3);
    assert_eq!(plan["totals"]["images"]["cap"], 52);
    assert_eq!(plan["balances"]["opening"]["tokens"], 0);

    // Now with real state, and an opening balance that leaves no visual room.
    let mut config = fixture(&root);
    config["budget"]["visual_passes"] = json!(2);
    config["budget"]["max_visual_passes"] = json!(3);
    write(&cfg, &config);
    command::run(&cfg, &out, None).unwrap_err();
    let path = out.join("run.json");
    let before = fs::read(&path).unwrap();
    let saved: Value = serde_json::from_slice(&before).unwrap();
    let decision = root.join("decision.json");
    write(
        &decision,
        &json!({"pause_id":saved["pause"]["id"],"identity":saved["identity"],
            "action":saved["pause"]["basis"]["proposed_action"],"by":"owner","rationale":"plan only"}),
    );
    let plan = telperion_jev::tuning::preflight::plan(&cfg, &out, Some(&decision)).unwrap();
    assert_eq!(
        fs::read(&path).unwrap(),
        before,
        "preflight wrote run state"
    );
    assert!(
        !out.join("plan.json").exists(),
        "plan.json is the CLI's job, not the planner's"
    );
    assert_eq!(plan["totals"]["visual_passes"]["fits"], false);
    assert_eq!(plan["totals"]["visual_passes"]["spent"], 2);
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn opening_balance_over_its_own_caps_is_refused_fresh_and_on_resume() {
    let root = std::env::temp_dir().join(format!(
        "tuning-balance-{}",
        telperion_jev::ledger::new_entry_id()
    ));
    fs::create_dir(&root).unwrap();
    let cfg = root.join("config.json");
    let out = root.join("out");
    let mut config = fixture(&root);
    config["budget"]["tokens"] = json!(200_001);
    write(&cfg, &config);
    let error = command::run(&cfg, &out, None).unwrap_err();
    assert!(error.contains("opening balance"), "{error}");
    assert!(!out.join("run.json").exists(), "refused run left state");

    // An opening balance inside its caps is carried, not reset.
    config["budget"]["tokens"] = json!(688_550 - 19_456);
    config["budget"]["max_tokens"] = json!(902_431);
    config["budget"]["visual_passes"] = json!(24);
    config["budget"]["max_visual_passes"] = json!(26);
    write(&cfg, &config);
    command::run(&cfg, &out, None).unwrap_err();
    let path = out.join("run.json");
    let saved: Value = serde_json::from_slice(&fs::read(&path).unwrap()).unwrap();
    assert_eq!(saved["budget"]["tokens"], 669_094);
    assert_eq!(saved["budget"]["visual_passes"], 24);

    // A resume may not carry a balance past its caps either.
    let mut over = saved.clone();
    over["budget"]["images"] = json!(53);
    write(&path, &over);
    let decision = root.join("decision.json");
    write(
        &decision,
        &json!({"pause_id":saved["pause"]["id"],"identity":saved["identity"],
            "action":saved["pause"]["basis"]["proposed_action"],"by":"owner","rationale":"resume"}),
    );
    let error = command::run(&cfg, &out, Some(&decision)).unwrap_err();
    assert!(error.contains("opening balance"), "{error}");
    fs::remove_dir_all(root).unwrap();
}

/// A ledger entry another tool really wrote, shaped so `ExternalUsage` accepts it.
fn external_ledger(dir: &Path, id: &str, tokens: u64) -> (std::path::PathBuf, String) {
    let questions = json!({"q":{"type":"choice","criteria":{"a":"a","b":"b"}}});
    let state_sha256 = telperion_jev::sha256_hex(b"external state");
    let model = "jev-1.13.0";
    let entry = json!({"id":id,"tool":"screen","state_sha256":state_sha256,"source":null,
        "model":model,"questions":questions,"answers":{"q":{"choice":"a","confidence":0.9}},
        "usage":{"input_tokens":tokens,"output_tokens":0},"elapsed_ms":1,
        "recorded_at":"2026-09-20T00:00:00Z",
        "identity":telperion_jev::ledger::derived_identity(&state_sha256, &questions, model)});
    let path = dir.join(format!("{id}.json"));
    let bytes = serde_json::to_vec_pretty(&entry).unwrap();
    fs::write(&path, &bytes).unwrap();
    (path, telperion_jev::sha256_hex(&bytes))
}

#[test]
fn preparation_and_external_usage_are_charged_once_across_repeated_resume() {
    use telperion_jev::tuning::reference_first::{charge_preparation, PreparationCharge};
    use telperion_jev::tuning::state::Budget;

    // An opening balance that excludes preparation ends at opening + preparation
    // exactly once, however many times the run is resumed.
    let mut budget: Budget = serde_json::from_value(
        json!({"evaluations":6,"images":30,"tokens":688_550 - 19_456,"rounds":2,
            "max_evaluations":13,"max_images":52,"max_tokens":902_431,"max_rounds":3,
            "visual_passes":24,"max_visual_passes":26}),
    )
    .unwrap();
    let charge = PreparationCharge {
        inventory_sha256: "a".repeat(64),
        preparation_sha256: "b".repeat(64),
        tokens: 19_456,
        visual_attempts: 1,
    };
    let mut record = None;
    charge_preparation(&mut budget, &mut record, &charge).unwrap();
    assert_eq!(budget.tokens, 688_550, "audited total is restored exactly");
    assert_eq!(budget.visual_passes, Some(25));
    assert_eq!(record.as_ref(), Some(&charge));
    for _ in 0..2 {
        charge_preparation(&mut budget, &mut record, &charge).unwrap();
        assert_eq!(budget.tokens, 688_550, "preparation was charged twice");
        assert_eq!(budget.visual_passes, Some(25));
    }
    // A different preparation cannot be swapped in over a recorded one.
    let mut other = charge.clone();
    other.tokens = 1;
    assert!(charge_preparation(&mut budget, &mut record, &other).is_err());
    assert_eq!(budget.tokens, 688_550);

    // The same external ledger cannot be imported twice.
    let root = std::env::temp_dir().join(format!(
        "tuning-import-{}",
        telperion_jev::ledger::new_entry_id()
    ));
    fs::create_dir(&root).unwrap();
    let cfg = root.join("config.json");
    let out = root.join("out");
    write(&cfg, &fixture(&root));
    command::run(&cfg, &out, None).unwrap_err();
    let path = out.join("run.json");
    let saved: Value = serde_json::from_slice(&fs::read(&path).unwrap()).unwrap();
    let (ledger, sha256) = external_ledger(&root, "outside-1", 8_109);
    let usage = json!({"previous_tokens":0,"next_tokens":8_109,"reason":"prior screening run",
        "ledgers":[{"path":ledger,"sha256":sha256,"id":"outside-1","tool":"screen",
            "model":"jev-1.13.0","identity":telperion_jev::ledger::derived_identity(
                &telperion_jev::sha256_hex(b"external state"),
                &json!({"q":{"type":"choice","criteria":{"a":"a","b":"b"}}}),
                "jev-1.13.0")}]});
    let decision = root.join("decision.json");
    let mut d = json!({"pause_id":saved["pause"]["id"],"identity":saved["identity"],
        "action":saved["pause"]["basis"]["proposed_action"],"by":"owner",
        "rationale":"import audited outside spend","external_usage":usage});
    write(&decision, &d);
    command::run(&cfg, &out, Some(&decision)).unwrap_err();
    let after: Value = serde_json::from_slice(&fs::read(&path).unwrap()).unwrap();
    assert_eq!(after["budget"]["tokens"], 8_109);

    // Re-submitting the same import is refused and changes nothing.
    let before = fs::read(&path).unwrap();
    d["pause_id"] = after["pause"]["id"].clone();
    d["identity"] = after["identity"].clone();
    d["external_usage"]["previous_tokens"] = json!(8_109);
    d["external_usage"]["next_tokens"] = json!(16_218);
    write(&decision, &d);
    let error = command::run(&cfg, &out, Some(&decision)).unwrap_err();
    assert!(error.contains("duplicate"), "{error}");
    assert_eq!(fs::read(&path).unwrap(), before);
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn priority_approval_resume_must_re_carry_exact_cap_pilot_authority() {
    use telperion_jev::tuning::{
        command::{prepare, Prepared},
        engine::Run,
        evaluation::Image,
        priority::{Checkpoint, Evidence},
        state::Visual,
    };
    let root = std::env::temp_dir().join(format!(
        "tuning-authority-{}",
        telperion_jev::ledger::new_entry_id()
    ));
    fs::create_dir(&root).unwrap();
    let mut config_value = fixture(&root);
    config_value["budget"]["visual_passes"] = json!(0);
    config_value["budget"]["max_visual_passes"] = json!(5);
    let config: Config = serde_json::from_value(config_value.clone()).unwrap();
    let cfg = root.join("config.json");
    write(&cfg, &config_value);
    let out = root.join("out");
    command::run(&cfg, &out, None).unwrap_err();
    let path = out.join("run.json");
    let mut state: Run = serde_json::from_slice(&fs::read(&path).unwrap()).unwrap();
    let image = Image {
        path: root.join("asset"),
        sha256: telperion_jev::sha256_hex(&fs::read(root.join("asset")).unwrap()),
        view: "whole".into(),
        seed: 1,
    };
    let visual: Visual = serde_json::from_value(
        json!({"identity":"candidate","model":"mock","ledger":"fixture","cells":[],"defects":[],"findings":[]}),
    )
    .unwrap();
    let checkpoint = Checkpoint::new(
        &state.identity,
        &config.priority_scope(&state),
        visual,
        vec![
            Evidence {
                id: "render-0".into(),
                role: "render".into(),
                image: image.clone(),
            },
            Evidence {
                id: "reference-0".into(),
                role: "reference".into(),
                image,
            },
        ],
    )
    .unwrap();
    state.priority_checkpoints.push(checkpoint.clone());
    state.pause.as_mut().unwrap().basis.proposed_action = "approve gap priorities".into();
    write(&path, &serde_json::to_value(&state).unwrap());
    let before = fs::read(&path).unwrap();
    let identity = config.identity().unwrap();
    let pause = state.pause.as_ref().unwrap();
    let mut decision = json!({"pause_id":pause.id,"identity":state.identity,
        "action":"approve gap priorities","by":"test owner","rationale":"owner ranked the gaps",
        "priority_approval":{"checkpoint_sha256":checkpoint.hash(),
            "scope_sha256":checkpoint.scope_sha256,"ordered":[]}});
    let decision_path = root.join("decision.json");
    write(&decision_path, &decision);

    // The approval decision becomes the last authorization, so it must itself
    // carry the experimental authority the pilot check reads.
    let Prepared::Ready(resumed) =
        prepare(&config, &path, Some(&decision_path), &identity).unwrap()
    else {
        panic!("unexpected interruption")
    };
    let error = resumed.pilot_authority().unwrap_err();
    assert!(error.contains("unvalidated"), "{error}");
    assert_eq!(fs::read(&path).unwrap(), before, "preparation wrote state");

    // Wrong caps are refused just as firmly as no authority at all.
    let authority = json!({"purpose":"bounded experiment","reason":"owner approved pilot",
        "next_identity":identity,"max_tokens":200000,"max_rounds":3,"max_evaluations":13,
        "max_images":52,"max_visual_passes":5});
    let mut wrong = authority.clone();
    wrong["max_tokens"] = json!(999999);
    decision["experimental_pilot"] = wrong;
    write(&decision_path, &decision);
    let error = command::run(&cfg, &out, Some(&decision_path)).unwrap_err();
    assert!(error.contains("authority mismatch"), "{error}");
    assert_eq!(fs::read(&path).unwrap(), before);

    // Exact caps clear the authority check and the run proceeds past it.
    decision["experimental_pilot"] = authority;
    write(&decision_path, &decision);
    let Prepared::Ready(resumed) =
        prepare(&config, &path, Some(&decision_path), &identity).unwrap()
    else {
        panic!("unexpected interruption")
    };
    resumed.pilot_authority().unwrap();
    assert_eq!(resumed.authorizations.len(), 1);
    fs::remove_dir_all(root).unwrap();
}
