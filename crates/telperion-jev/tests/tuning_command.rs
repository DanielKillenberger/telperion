mod fixture;

use serde_json::{json, Value};
use std::{fs, path::Path};
use telperion_jev::tuning::{command, live::Config};

/// Never reached in these tests: the key source fails first, by design.
struct NoDispatch;
impl telperion_jev::caller::Transport for NoDispatch {
    fn send(
        &self,
        _: &telperion_jev::caller::HttpRequest,
    ) -> Result<telperion_jev::caller::HttpResponse, String> {
        panic!("a pre-dispatch test dispatched")
    }
}

/// A sibling test forking a stub can briefly hold a copy of a just-released
/// `run.lock` descriptor, so tests that take the run lock do not overlap.
fn serial() -> std::sync::MutexGuard<'static, ()> {
    static SERIAL: std::sync::Mutex<()> = std::sync::Mutex::new(());
    SERIAL.lock().unwrap_or_else(|e| e.into_inner())
}

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
    let _serial = serial();
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
    let _serial = serial();
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
    {
        let e = command::run(&config_path, &out, Some(&decision)).unwrap_err();
        assert!(e.contains("calibration prerequisite"), "ACTUAL: {e}");
    }
    let saved: Value = serde_json::from_slice(&fs::read(&path).unwrap()).unwrap();
    assert_eq!(saved["budget"]["tokens"], 17);
    assert_eq!(saved["identity"], d["next_identity"]);
    assert_eq!(saved["visual"], Value::Null);
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn stale_lock_file_and_partial_temp_do_not_strand_interrupted_run() {
    let _serial = serial();
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
    {
        let e = command::run(&config_path, &out, None).unwrap_err();
        assert!(e.contains("interruption recorded"), "ACTUAL: {e}");
    }
    let state: Value = serde_json::from_slice(&fs::read(&path).unwrap()).unwrap();
    let decision = root.join("decision.json");
    write(
        &decision,
        &json!({"pause_id":state["pause"]["id"],"identity":state["identity"],
        "action":"recover interrupted attempt","by":"test owner","rationale":"measurement process ended; keep reservation","recover_interrupted":true}),
    );
    {
        let e = command::run(&config_path, &out, Some(&decision)).unwrap_err();
        assert!(e.contains("calibration prerequisite"), "ACTUAL: {e}");
    }
    let state: Value = serde_json::from_slice(&fs::read(&path).unwrap()).unwrap();
    assert_eq!(state["budget"]["evaluations"], 1);
    assert_eq!(state["budget"]["images"], 4);
    assert_eq!(state["pending"], Value::Null);
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn scoped_cap_extension_reuses_only_unchanged_verified_evidence() {
    let _serial = serial();
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
    let _serial = serial();
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
    let _serial = serial();
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
    assert_eq!(plan["totals_one_candidate"]["evaluations"]["needed"], 2);
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
    let _serial = serial();
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
    let _serial = serial();
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
    let _serial = serial();
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

/// The audited opening balance the canonical run would carry.
fn opening() -> Value {
    json!({"evaluations":6,"images":30,"tokens":688_550,"rounds":2,
        "max_evaluations":13,"max_images":52,"max_tokens":902_431,"max_rounds":3,
        "visual_passes":25,"max_visual_passes":26})
}

#[test]
fn a_verifying_config_pauses_for_authority_then_charges_preparation_exactly_once() {
    let _serial = serial();
    let f = fixture::verifying_fixture(opening());
    let config: Config = serde_json::from_slice(&fs::read(&f.config_path).unwrap()).unwrap();
    config
        .verify()
        .expect("the fixture must pass the real Config::verify");
    let identity = config.identity().unwrap();
    let no_key = || Err::<String, String>("fixture stops before dispatch".into());

    // (i) Calibration is satisfied, so the run stops on authority, not calibration.
    let error = command::run_with(&f.config_path, &f.out, None, &NoDispatch, &no_key).unwrap_err();
    assert!(error.contains("scoped experimental authority"), "{error}");
    let state = f.run_json();
    assert_eq!(
        state["pause"]["basis"]["proposed_action"],
        "authorize bounded experimental pilot"
    );
    assert_eq!(state["preparation_charge"], Value::Null);
    assert_eq!(state["budget"]["tokens"], 688_550);
    assert_eq!(state["budget"]["visual_passes"], 25);

    // (ii) Scoped authority lets the run reach preparation, which is charged once.
    let decision = f.root.join("authority.json");
    let authority = json!({"purpose":"bounded offline fixture","reason":"synthetic",
        "next_identity":identity,"max_tokens":902_431,"max_rounds":3,"max_evaluations":13,
        "max_images":52,"max_visual_passes":26});
    let mut d = json!({"pause_id":state["pause"]["id"],"identity":state["identity"],
        "action":"authorize bounded experimental pilot","by":"fixture owner",
        "rationale":"synthetic scoped authority","experimental_pilot":authority});
    write(&decision, &d);
    let error = command::run_with(
        &f.config_path,
        &f.out,
        Some(&decision),
        &NoDispatch,
        &no_key,
    )
    .unwrap_err();
    assert!(error.contains("fixture stops before dispatch"), "{error}");
    let charged = f.run_json();
    let opening_tokens = 688_550u64;
    let charge = charged["preparation_charge"].clone();
    assert!(charge.is_object(), "preparation was not charged");
    let tokens = charge["tokens"].as_u64().unwrap();
    assert_eq!(charge["visual_attempts"], 1);
    assert_eq!(charged["budget"]["tokens"], opening_tokens + tokens);
    assert_eq!(charged["budget"]["visual_passes"], 26);

    // A repeated resume re-verifies the same pins without charging again.
    for _ in 0..2 {
        let paused = f.run_json();
        d["pause_id"] = paused["pause"]["id"].clone();
        d["identity"] = paused["identity"].clone();
        d["action"] = paused["pause"]["basis"]["proposed_action"].clone();
        write(&decision, &d);
        command::run_with(
            &f.config_path,
            &f.out,
            Some(&decision),
            &NoDispatch,
            &no_key,
        )
        .unwrap_err();
        let again = f.run_json();
        assert_eq!(again["budget"]["tokens"], opening_tokens + tokens);
        assert_eq!(again["budget"]["visual_passes"], 26);
        assert_eq!(again["preparation_charge"], charge);
    }

    // An interrupted attempt is recorded, and recovering it changes no counter.
    let mut interrupted = f.run_json();
    interrupted["pending"] = json!("candidate evaluation");
    interrupted["pause"] = Value::Null;
    write(&f.out.join("run.json"), &interrupted);
    let error = command::run_with(
        &f.config_path,
        &f.out,
        Some(&decision),
        &NoDispatch,
        &no_key,
    )
    .unwrap_err();
    assert!(error.contains("interruption recorded"), "{error}");
    let recorded = f.run_json();
    assert_eq!(recorded["budget"]["tokens"], opening_tokens + tokens);
    d["pause_id"] = recorded["pause"]["id"].clone();
    d["identity"] = recorded["identity"].clone();
    d["action"] = recorded["pause"]["basis"]["proposed_action"].clone();
    d["recover_interrupted"] = json!(true);
    write(&decision, &d);
    command::run_with(
        &f.config_path,
        &f.out,
        Some(&decision),
        &NoDispatch,
        &no_key,
    )
    .unwrap_err();
    let recovered = f.run_json();
    assert_eq!(recovered["budget"]["tokens"], opening_tokens + tokens);
    assert_eq!(recovered["budget"]["visual_passes"], 26);
    assert_eq!(recovered["preparation_charge"], charge);
    f.cleanup();
}

/// Answers the real caller. Records the exact state each judgment was sent.
struct Jev {
    states: std::sync::Mutex<Vec<Value>>,
    plan: Vec<(String, f64)>,
}
impl telperion_jev::caller::Transport for Jev {
    fn send(
        &self,
        request: &telperion_jev::caller::HttpRequest,
    ) -> Result<telperion_jev::caller::HttpResponse, String> {
        let body: Value = serde_json::from_slice(request.body.as_ref().unwrap()).unwrap();
        self.states.lock().unwrap().push(body["state"].clone());
        let mut answers = serde_json::Map::new();
        let mut rank = 0usize;
        for key in body["questions"].as_object().unwrap().keys() {
            let (choice, confidence) = if key.starts_with("route:") || key == "route" {
                let answer = self
                    .plan
                    .get(rank)
                    .cloned()
                    .unwrap_or(("tuning".into(), 0.9));
                rank += 1;
                answer
            } else if key == "risk" {
                ("bounded".into(), 0.9)
            } else if key == "tractability" || key == "progress" {
                ("supported".into(), 0.9)
            } else {
                ("small_increase".into(), 0.9)
            };
            answers.insert(
                key.clone(),
                json!({"choice":choice,"confidence":confidence}),
            );
        }
        Ok(telperion_jev::caller::HttpResponse {
            status: 200,
            body: serde_json::to_vec(&json!({"model":fixture::JUDGMENT_MODEL,
                "answers":answers,"usage":{"input_tokens":40,"output_tokens":10}}))
            .unwrap(),
        })
    }
}

fn gap(id: &str, observation: &str) -> Value {
    json!({"id":id,"observation":observation,
        "evidence_ids":["render-0","reference-0"],"views":["whole"]})
}

#[test]
#[ignore = "drives the compare stub, which runs under uv; run with --ignored"]
fn the_real_command_reaches_the_priority_pause_then_routes_and_writes_handoffs() {
    let _serial = serial();
    let f = fixture::verifying_fixture(json!({"evaluations":0,"images":0,"tokens":0,"rounds":0,
        "max_evaluations":13,"max_images":52,"max_tokens":2_000_000,"max_rounds":1,
        "visual_passes":0,"max_visual_passes":12}));
    let config: Config = serde_json::from_slice(&fs::read(&f.config_path).unwrap()).unwrap();
    let identity = config.identity().unwrap();
    let key = || Ok::<String, String>("fixture-key".into());
    let jev = Jev {
        states: std::sync::Mutex::new(vec![]),
        plan: vec![
            ("tuning".into(), 0.9),
            ("existing:fn-77".into(), 0.9),
            ("appearance".into(), 0.31),
        ],
    };
    let authority = json!({"purpose":"bounded offline fixture","reason":"synthetic",
        "next_identity":identity,"max_tokens":2_000_000,"max_rounds":1,"max_evaluations":13,
        "max_images":52,"max_visual_passes":12});
    let decision = f.root.join("decision.json");

    // Authority first, exactly as the contract requires.
    command::run_with(&f.config_path, &f.out, None, &jev, &key).unwrap_err();
    let paused = f.run_json();
    let d = json!({"pause_id":paused["pause"]["id"],"identity":paused["identity"],
        "action":"authorize bounded experimental pilot","by":"fixture owner",
        "rationale":"synthetic","experimental_pilot":authority});
    write(&decision, &d);

    // (iii) Baseline and the initial visual run, then the owner gate stops it.
    command::run_with(&f.config_path, &f.out, Some(&decision), &jev, &key).unwrap_err();
    let reviewed = f.run_json();
    assert_eq!(
        reviewed["pause"]["basis"]["proposed_action"], "approve gap priorities",
        "{:?}",
        reviewed["pause"]["reason"]
    );
    assert!(f.out.join("priority-review.json").exists());
    assert_eq!(reviewed["machine_ready"], false);
    let checkpoint = reviewed["priority_checkpoints"]
        .as_array()
        .unwrap()
        .last()
        .unwrap()
        .clone();
    let checkpoint_sha256 =
        serde_json::from_value::<telperion_jev::tuning::priority::Checkpoint>(checkpoint.clone())
            .unwrap()
            .hash();

    // An approval without re-carried authority re-pauses on authority.
    let ordered = json!([
        gap("owner-crown", "Crown shape and foliage organization"),
        gap("owner-hanging", "Hanging outer foliage"),
        gap("owner-materials", "Materials, including bark and foliage")
    ]);
    let approval = json!({"checkpoint_sha256":checkpoint_sha256,
        "scope_sha256":checkpoint["scope_sha256"],"ordered":ordered});
    let mut bare = json!({"pause_id":reviewed["pause"]["id"],"identity":reviewed["identity"],
        "action":"approve gap priorities","by":"fixture owner","rationale":"synthetic ranking",
        "preserve_evidence":true,"priority_approval":approval});
    write(&decision, &bare);
    let error = command::run_with(&f.config_path, &f.out, Some(&decision), &jev, &key).unwrap_err();
    assert!(error.contains("scoped experimental authority"), "{error}");

    // (iv) With authority re-carried, routing runs and grounds the handoffs.
    let reauth = f.run_json();
    bare["pause_id"] = reauth["pause"]["id"].clone();
    bare["action"] = reauth["pause"]["basis"]["proposed_action"].clone();
    bare["experimental_pilot"] = d["experimental_pilot"].clone();
    write(&decision, &bare);
    command::run_with(&f.config_path, &f.out, Some(&decision), &jev, &key).unwrap_err();
    let routed = f.run_json();

    assert_eq!(routed["machine_ready"], false);
    let handoffs: Value =
        serde_json::from_slice(&fs::read(f.out.join("handoffs.json")).unwrap()).unwrap();
    let list = handoffs["handoffs"].as_array().unwrap();
    assert_eq!(list.len(), 2, "one handoff per non-tuning priority");
    let grounded = list
        .iter()
        .find(|h| h["gap_id"] == "owner-hanging")
        .unwrap();
    assert_eq!(grounded["route"], "existing:fn-77");
    assert_eq!(grounded["existing_spec"], "fn-77");
    assert_eq!(grounded["dispatch_authorized"], true);
    let uncertain = list
        .iter()
        .find(|h| h["gap_id"] == "owner-materials")
        .unwrap();
    assert_eq!(uncertain["route"], "insufficient_evidence");
    assert_eq!(uncertain["raw_choice"], "appearance");
    assert_eq!(uncertain["dispatch_authorized"], false);
    assert!(handoffs["unresolved_priorities"]
        .as_array()
        .unwrap()
        .contains(&json!("owner-materials")));

    // The tuning priority still spent a round on real candidates.
    assert!(routed["budget"]["evaluations"].as_u64().unwrap() > 1);
    assert_eq!(routed["budget"]["rounds"], 1);

    // Every judgment's recorded input is the exact value the transport saw.
    let sent = jev
        .states
        .lock()
        .unwrap()
        .iter()
        .map(|s| telperion_jev::sha256_hex(&serde_json::to_vec(s).unwrap()))
        .collect::<Vec<_>>();
    let recorded = routed["judgment_inputs"].as_array().unwrap();
    assert!(!recorded.is_empty());
    for input in recorded {
        let hash = input["state_sha256"].as_str().unwrap();
        assert_eq!(
            hash,
            telperion_jev::sha256_hex(&serde_json::to_vec(&input["state"]).unwrap()),
            "recorded hash does not match its own recorded state"
        );
        assert!(
            sent.contains(&hash.to_string()),
            "recorded a judgment the transport never received: {}",
            input["label"]
        );
    }

    // Accounting stayed inside every cap it was given.
    let b = &routed["budget"];
    for (spent, cap) in [
        ("tokens", "max_tokens"),
        ("images", "max_images"),
        ("evaluations", "max_evaluations"),
        ("rounds", "max_rounds"),
        ("visual_passes", "max_visual_passes"),
    ] {
        assert!(
            b[spent].as_u64().unwrap() <= b[cap].as_u64().unwrap(),
            "{spent} exceeded {cap}"
        );
    }
    f.cleanup();
}

/// Answers every dial question with a supported adjustment.
struct EveryDial;
impl telperion_jev::caller::Transport for EveryDial {
    fn send(
        &self,
        request: &telperion_jev::caller::HttpRequest,
    ) -> Result<telperion_jev::caller::HttpResponse, String> {
        let body: Value = serde_json::from_slice(request.body.as_ref().unwrap()).unwrap();
        let answers = body["questions"]
            .as_object()
            .unwrap()
            .keys()
            .map(|k| {
                (
                    k.clone(),
                    json!({"choice":"small_increase","confidence":0.9,
                        "probabilities":{"small_increase":0.55,"substantial_increase":0.39,
                            "hold":0.03,"insufficient_evidence":0.03}}),
                )
            })
            .collect::<serde_json::Map<_, _>>();
        Ok(telperion_jev::caller::HttpResponse {
            status: 200,
            body: serde_json::to_vec(&json!({"model":fixture::JUDGMENT_MODEL,
                "answers":answers,"usage":{"input_tokens":5,"output_tokens":5}}))
            .unwrap(),
        })
    }
}

#[test]
fn max_candidates_is_validated_and_bounds_one_round() {
    let _serial = serial();
    use telperion_jev::tuning::{
        actions::Dial,
        engine::{Run, Services},
        live::Live,
        state::Budget,
    };
    let f = fixture::verifying_fixture(opening());
    let mut config: Config = serde_json::from_slice(&fs::read(&f.config_path).unwrap()).unwrap();

    // Validation: only 1..=4, and absence keeps today's behaviour.
    config.verify().unwrap();
    for bad in [0u64, 5, 99] {
        config.max_candidates = Some(bad);
        let error = config.verify().unwrap_err();
        assert!(error.contains("max_candidates"), "{bad}: {error}");
    }
    config.max_candidates = Some(1);
    config.verify().unwrap();

    // The bound is what stops the round, in dial-table order.
    let dials: Vec<Dial> = serde_json::from_value(json!([
        {"id":"limbs","path":"/skeleton/habit/lateralsPerStation","meaning":"limbs born at each station","min":1,"max":4,"integer":true,"small":1,"substantial":2},
        {"id":"leaves","path":"/canopy/shortShootLeaves","meaning":"leaves per cluster","min":2,"max":12,"integer":true,"small":2,"substantial":4},
        {"id":"spacing","path":"/canopy/shortShootSpacing","meaning":"metres between leaf clusters","min":0.01,"max":0.08,"integer":false,"small":0.01,"substantial":0.02},
        {"id":"irregularity","path":"/skeleton/envelope/irregularity","meaning":"crown envelope lobes and hollows","min":0,"max":0.5,"integer":false,"small":0.08,"substantial":0.16}
    ]))
    .unwrap();
    let effective = telperion_core::params::metadata(
        &telperion_core::presets::Preset::from_id("european-beech")
            .unwrap()
            .parameters(),
    );
    let state = Run {
        identity: "id".into(),
        preset: "european-beech".into(),
        seed: 1,
        effective,
        overrides: json!({}),
        dials: dials.clone(),
        owner_notes: "notes".into(),
        required: config.required.clone(),
        budget: serde_json::from_value::<Budget>(opening()).unwrap(),
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
    };
    // `propose` now returns every accepted move, ordered, and reports the
    // bound; the engine truncates after refusing repeats, so a move already
    // tried cannot consume the round's only slot.
    for (bound, expected) in [(Some(1u64), 1u64), (Some(2), 2), (None, 4)] {
        let mut bounded = config.clone();
        bounded.max_candidates = bound;
        let mut live = Live {
            config: &bounded,
            transport: &EveryDial,
            key: "unused",
        };
        assert_eq!(live.max_candidates(), expected, "bound {bound:?}");
        let proposals = live.propose(&state, 0).unwrap().value;
        assert_eq!(proposals.len(), 4, "every accepted move is returned");
        // Equal direction mass, so the dial table's own order decides.
        assert_eq!(proposals[0].dial, "limbs");
        let mut truncated = proposals;
        truncated.truncate(live.max_candidates() as usize);
        assert_eq!(truncated.len() as u64, expected);
    }
    f.cleanup();
}

#[test]
fn the_inventory_command_emits_what_the_runtime_pins_and_charges() {
    let _serial = serial();
    use telperion_jev::tuning::{
        inventory,
        reference_first::{FilePin, Inventory, RuntimeConfig},
    };
    let f = fixture::verifying_fixture(opening());
    let mut config: Config = serde_json::from_slice(&fs::read(&f.config_path).unwrap()).unwrap();

    // A three-reference species, which is what the canonical config carries.
    let raw: Value = serde_json::from_slice(&fs::read(&f.config_path).unwrap()).unwrap();
    let one = raw["references"][0].clone();
    let mut references = vec![];
    for view in ["B-WHOLE", "B-BARE", "B-BASE"] {
        let mut image = one.clone();
        image["view"] = json!(view);
        references.push(image);
    }
    config.references = serde_json::from_value(json!(references)).unwrap();
    config.reference_first = None;

    let out = f.root.join("stage-a");
    let path = inventory::run(&config, &out).unwrap();
    assert_eq!(path, out.join("inventory.json"));

    // The species field is the preset id, exactly as the comparison path sends.
    let produced: Inventory = serde_json::from_slice(&fs::read(&path).unwrap()).unwrap();
    produced.verify().unwrap();
    assert_eq!(produced.request.target_species, fixture::PRESET);
    assert_eq!(produced.request.references.len(), 3);
    assert!(
        produced.ledger.contains("#sha256:"),
        "inventory must cite a real ledger, got {}",
        produced.ledger
    );

    // The pair loads through the runtime's own gate and is accepted for this config.
    let pin = |p: std::path::PathBuf| FilePin {
        sha256: telperion_jev::sha256_hex(&fs::read(&p).unwrap()),
        path: p,
    };
    let prepared = RuntimeConfig {
        inventory: pin(path.clone()),
        preparation: pin(out.join("preparation.json")),
    };
    let (loaded, charge) = prepared
        .load(&config.vision.model, &config.vision.effort)
        .unwrap();
    assert_eq!(loaded.hash(), produced.hash());
    assert_eq!(charge.tokens, 1000);
    assert_eq!(charge.visual_attempts, 1);
    config.reference_first = Some(prepared);
    let accepted = config.preparation().unwrap().unwrap();
    assert_eq!(accepted, charge);

    // The journal records the settled attempt, and the output is never rewritten.
    let journal: Value =
        serde_json::from_slice(&fs::read(out.join("inventory-journal.json")).unwrap()).unwrap();
    assert_eq!(journal["status"], "settled");
    assert_eq!(journal["attempts"], 1);
    assert_eq!(journal["retries"], false);
    assert_eq!(journal["usage"]["input_tokens"], 700);
    let error = inventory::run(&config, &out).unwrap_err();
    assert!(error.contains("refusing to overwrite"), "{error}");

    // A failing adapter still leaves a charged attempt on the record.
    let mut broken = config.clone();
    broken.vision.args = vec!["-c".into(), "raise SystemExit(3)".into()];
    let failed_out = f.root.join("stage-a-failed");
    let error = inventory::run(&broken, &failed_out).unwrap_err();
    assert!(error.contains("charged"), "{error}");
    let journal: Value =
        serde_json::from_slice(&fs::read(failed_out.join("inventory-journal.json")).unwrap())
            .unwrap();
    assert_eq!(journal["status"], "failed");
    assert_eq!(journal["attempts"], 1);
    assert!(!failed_out.join("inventory.json").exists());
    f.cleanup();
}

#[test]
fn a_frozen_reference_first_replay_runs_and_qualifies_a_config() {
    let _serial = serial();
    use telperion_jev::tuning::{inventory, replay};
    let f = fixture::verifying_fixture(opening());
    let mut config: Config = serde_json::from_slice(&fs::read(&f.config_path).unwrap()).unwrap();
    config.reference_first = None;

    // Stage A first, through the shipped command.
    let stage_a = f.root.join("stage-a");
    let inv = inventory::run(&config, &stage_a).unwrap();

    let raw: Value = serde_json::from_slice(&fs::read(&f.config_path).unwrap()).unwrap();
    let render = raw["images_render"].clone();
    let image = if render.is_null() {
        raw["references"][0].clone()
    } else {
        render
    };
    let shots = raw["matched"]["references"].as_str().unwrap();
    let case = |id: &str, ready: bool, condition: &str, framing: &str| {
        json!({"id":id,"provenance":format!("synthetic fixture case {id}"),
            "expected_ready":ready,"identity":format!("candidate-{id}"),
            "target_species":fixture::PRESET,"inventory":inv,
            "required":[{"item":"reference_character","view":"whole","seed":1}],
            "renders":[{"image":image,"condition":condition,"framing":framing}],
            "references":raw["references"],
            "quality_anchors":raw["quality_anchors"],
            "shots":shots})
    };
    let job = f.root.join("job.json");
    write(
        &job,
        &json!({"schema":replay::JOB_SCHEMA,"model":fixture::VISION_MODEL,
            "effort":fixture::EFFORT,"protocol":raw["vision_protocol"],
            "cases":[case("positive", true, "same_geometry_visibility_view_not_unloaded_leaf_off", "complete"),
                     case("negative", false, "historical_reconstructed_still", "clipped")]}),
    );
    let manifest_path = f.root.join("case-replay.json");
    let frozen = replay::freeze(&job, &manifest_path).unwrap();
    assert_eq!(frozen.cases.len(), 2);

    // The expectation never reaches the dispatched request.
    for c in &frozen.cases {
        let bytes = serde_json::to_string(&c.request).unwrap();
        assert!(!bytes.contains("expected_ready"));
        assert_eq!(
            c.request.comparison.hash(),
            c.request.comparison.blind().hash(),
            "case {} is not blind",
            c.id
        );
    }
    // Relabelling an expectation cannot change what the adapter sees.
    let flipped = f.root.join("job-flipped.json");
    let mut j: Value = serde_json::from_slice(&fs::read(&job).unwrap()).unwrap();
    j["cases"][0]["expected_ready"] = json!(false);
    write(&flipped, &j);
    let other = replay::freeze(&flipped, &f.root.join("case-replay-flipped.json")).unwrap();
    assert_eq!(
        other.cases[0].request.hash(),
        frozen.cases[0].request.hash()
    );

    // Running it scores one positive and one negative with no false ready.
    let manifest = fs::read(&manifest_path).unwrap();
    let result_path = f.root.join("case-replay-result.json");
    let score = replay::run(
        &manifest,
        &config.vision,
        std::path::Path::new(raw["vision_protocol"].as_str().unwrap()),
        &f.root.join("case-replay-journal.json"),
        &result_path,
        200_000,
    )
    .unwrap();
    assert_eq!(score.positives, 1);
    assert_eq!(score.negatives, 1);
    assert_eq!(score.false_ready, 0);
    assert_eq!(score.false_rejections, 0);

    // A config pointing at the produced pair passes the real admission gate.
    config.visual_validation = telperion_jev::tuning::live::Validation {
        manifest: manifest_path,
        result: result_path,
    };
    let pin = |p: std::path::PathBuf| telperion_jev::tuning::reference_first::FilePin {
        sha256: telperion_jev::sha256_hex(&fs::read(&p).unwrap()),
        path: p,
    };
    config.reference_first = Some(telperion_jev::tuning::reference_first::RuntimeConfig {
        inventory: pin(inv),
        preparation: pin(stage_a.join("preparation.json")),
    });
    config
        .verify()
        .expect("the produced replay must qualify the config");
    f.cleanup();
}

#[test]
fn bootstrap_admits_a_positive_less_replay_only_with_owner_relabels() {
    use telperion_jev::tuning::{inventory, live::OwnerRelabel, replay};
    let _serial = serial();
    let f = fixture::verifying_fixture(opening());
    let mut config: Config = serde_json::from_slice(&fs::read(&f.config_path).unwrap()).unwrap();
    config.reference_first = None;
    let stage_a = f.root.join("stage-a");
    let inv = inventory::run(&config, &stage_a).unwrap();
    let raw: Value = serde_json::from_slice(&fs::read(&f.config_path).unwrap()).unwrap();
    let image = raw["references"][0].clone();
    let shots = raw["matched"]["references"].as_str().unwrap();
    // Both cases are labelled ready; the clipped one cannot be, so it is a
    // false rejection - exactly the shape the real qualification produced.
    let case = |id: &str, framing: &str, expected: bool| {
        json!({"id":id,"provenance":format!("synthetic {id}"),"expected_ready":expected,
            "identity":format!("candidate-{id}"),"target_species":fixture::PRESET,
            "inventory":inv,"required":[{"item":"reference_character","view":"whole","seed":1}],
            "renders":[{"image":image,"condition":"historical_reconstructed_still","framing":framing}],
            "references":raw["references"],"quality_anchors":raw["quality_anchors"],"shots":shots})
    };
    let job = f.root.join("bootstrap-job.json");
    write(
        &job,
        &json!({"schema":replay::JOB_SCHEMA,"model":fixture::VISION_MODEL,
            "effort":fixture::EFFORT,"protocol":raw["vision_protocol"],
            "cases":[case("clipped", "clipped", true), case("negative", "clipped", false)]}),
    );
    let manifest_path = f.root.join("bootstrap-replay.json");
    replay::freeze(&job, &manifest_path).unwrap();
    let result_path = f.root.join("bootstrap-result.json");
    let score = replay::run(
        &fs::read(&manifest_path).unwrap(),
        &config.vision,
        std::path::Path::new(raw["vision_protocol"].as_str().unwrap()),
        &f.root.join("bootstrap-journal.json"),
        &result_path,
        200_000,
    )
    .unwrap();
    assert_eq!(
        score.false_rejections, 1,
        "the clipped case must be rejected"
    );
    assert_eq!(score.false_ready, 0);

    let pin = |p: std::path::PathBuf| telperion_jev::tuning::reference_first::FilePin {
        sha256: telperion_jev::sha256_hex(&fs::read(&p).unwrap()),
        path: p,
    };
    config.reference_first = Some(telperion_jev::tuning::reference_first::RuntimeConfig {
        inventory: pin(inv),
        preparation: pin(stage_a.join("preparation.json")),
    });
    config.visual_validation = telperion_jev::tuning::live::Validation {
        manifest: manifest_path,
        result: result_path,
    };

    // Without bootstrap this is exactly today's behaviour: convergence demanded.
    let error = config.verify().unwrap_err();
    assert!(
        error.contains("bounded convergence remains unproven"),
        "{error}"
    );

    // Bootstrap alone is not enough; the rejection still needs an owner verdict.
    config.visual_bootstrap = true;
    let error = config.verify().unwrap_err();
    assert!(
        error.contains("bounded convergence remains unproven"),
        "{error}"
    );

    let verdict = "the render is too dark and streaky";
    let evidence = f.root.join("owner-verdict.md");
    fs::write(&evidence, format!("# owner look\n\n{verdict}\n")).unwrap();
    let relabel = |case_id: &str, text: &str| OwnerRelabel {
        case_id: case_id.into(),
        by: "owner".into(),
        verdict: text.into(),
        sha256: telperion_jev::sha256_hex(&fs::read(&evidence).unwrap()),
        evidence: evidence.clone(),
    };
    // A relabel naming the wrong case is refused.
    config.owner_relabels = vec![relabel("negative", verdict)];
    assert!(config
        .verify()
        .unwrap_err()
        .contains("not falsely rejected"));
    // A verdict absent from its evidence is refused.
    config.owner_relabels = vec![relabel("clipped", "words the owner never wrote")];
    assert!(config.verify().unwrap_err().contains("not in its evidence"));
    // The real one admits the replay, with no positive at all.
    config.owner_relabels = vec![relabel("clipped", verdict)];
    config
        .verify()
        .expect("bootstrap must admit a relabelled rejection");
    f.cleanup();
}

#[test]
fn bootstrap_never_claims_readiness_and_needs_authority_that_names_it() {
    use telperion_jev::tuning::engine::Run;
    let _serial = serial();
    let f = fixture::verifying_fixture(opening());
    let mut raw: Value = serde_json::from_slice(&fs::read(&f.config_path).unwrap()).unwrap();
    raw["visual_bootstrap"] = json!(true);
    f.rewrite(&raw);
    let config: Config = serde_json::from_value(raw).unwrap();
    let identity = config.identity().unwrap();
    let no_key = || Err::<String, String>("stops before dispatch".into());
    command::run_with(&f.config_path, &f.out, None, &NoDispatch, &no_key).unwrap_err();
    let state = f.run_json();
    assert_eq!(state["visual_bootstrap"], true);

    // Authority that does not name the mode is refused.
    let authority = |names: bool| {
        json!({"purpose":"bounded offline fixture","reason":"synthetic","visual_bootstrap":names,
            "next_identity":identity,"max_tokens":902_431,"max_rounds":3,"max_evaluations":13,
            "max_images":52,"max_visual_passes":26})
    };
    let decision = f.root.join("authority.json");
    let mut d = json!({"pause_id":state["pause"]["id"],"identity":state["identity"],
        "action":"authorize bounded experimental pilot","by":"owner","rationale":"synthetic",
        "experimental_pilot":authority(false)});
    write(&decision, &d);
    let mut parsed: Run =
        serde_json::from_slice(&fs::read(f.out.join("run.json")).unwrap()).unwrap();
    parsed
        .authorizations
        .push(serde_json::from_value(d.clone()).unwrap());
    let error = parsed.pilot_authority().unwrap_err();
    assert!(error.contains("naming visual_bootstrap"), "{error}");

    // Naming it is accepted.
    d["experimental_pilot"] = authority(true);
    let mut parsed: Run =
        serde_json::from_slice(&fs::read(f.out.join("run.json")).unwrap()).unwrap();
    parsed
        .authorizations
        .push(serde_json::from_value(d).unwrap());
    parsed.pilot_authority().unwrap();

    // And a run that is not bootstrap refuses authority that names it.
    parsed.visual_bootstrap = false;
    assert!(parsed
        .pilot_authority()
        .unwrap_err()
        .contains("naming visual_bootstrap"));
    f.cleanup();
}

#[test]
fn image_and_evaluation_caps_extend_only_on_an_exact_scoped_decision() {
    use telperion_jev::tuning::engine::Run;
    let _serial = serial();
    let f = fixture::verifying_fixture(opening());
    let cfg = f.config_path.clone();
    let raw_config: Value = serde_json::from_slice(&fs::read(&cfg).unwrap()).unwrap();
    let no_key = || Err::<String, String>("stops before dispatch".into());

    // Reach the authority pause, then give the run spend, evidence and an
    // approval worth preserving.
    command::run_with(&cfg, &f.out, None, &NoDispatch, &no_key).unwrap_err();
    let path = f.out.join("run.json");
    let mut state: Run = serde_json::from_slice(&fs::read(&path).unwrap()).unwrap();
    let trial = telperion_jev::tuning::evaluation::Trial {
        key: "candidate-1".into(),
        identity: state.identity.clone(),
        seed: 1,
        round: 0,
        label: "baseline".into(),
        overrides: json!({}),
        ledger: None,
        feasible: true,
        reason: None,
        measurement: json!({}),
        comparisons: vec![telperion_jev::tuning::evaluation::Comparison {
            reference: "whole".into(),
            reference_weight: 1.,
            metric_weights: [1.; 5],
            target: [1.; 5],
            observed: [Some(1.); 5],
            images: serde_json::from_value(json!([raw_config["references"][0]])).unwrap(),
        }],
        score: Some(0.5),
        seconds: 0.,
        base: None,
        action: None,
        evidence: None,
        direction_mass: None,
        rule: None,
    };
    state.trials.push(trial);
    state.current = Some(0);
    state.visual = Some(
        serde_json::from_value(
            json!({"identity":"candidate-1","model":fixture::VISION_MODEL,
            "ledger":"receipt","cells":[],"defects":[],"findings":[]}),
        )
        .unwrap(),
    );
    state.budget.images = 30;
    state.budget.evaluations = 6;
    state.budget.tokens = 1234;
    write(&path, &serde_json::to_value(&state).unwrap());
    let before = fs::read(&path).unwrap();

    let identity_of = |images: u64, evaluations: u64| {
        let mut raw: Value = serde_json::from_slice(&fs::read(&cfg).unwrap()).unwrap();
        raw["budget"]["max_images"] = json!(images);
        raw["budget"]["max_evaluations"] = json!(evaluations);
        let next: Config = serde_json::from_value(raw.clone()).unwrap();
        (raw, next.identity().unwrap())
    };
    let decision = f.root.join("decision.json");
    let base = |identity: &str| {
        json!({"pause_id":state.pause.as_ref().unwrap().id,"identity":state.identity,
            "action":"authorize bounded experimental pilot","by":"owner",
            "rationale":"owner raised the image cap mid-run","next_identity":identity,
            "preserve_evidence":true,
            "experimental_pilot":{"purpose":"bounded","reason":"owner authorized",
                "next_identity":identity,"max_tokens":902_431,"max_rounds":3,
                "max_evaluations":20,"max_images":60,"max_visual_passes":26}})
    };

    // (3) Changing max_images with no extension at all is still refused.
    let (raw, identity) = identity_of(60, 13);
    f.rewrite(&raw);
    let mut d = base(&identity);
    write(&decision, &d);
    let error = command::run_with(&cfg, &f.out, Some(&decision), &NoDispatch, &no_key).unwrap_err();
    assert!(error.contains("budget caps"), "{error}");
    assert_eq!(fs::read(&path).unwrap(), before);

    // (1) A wrong previous is refused and mutates nothing.
    d["image_cap_extension"] = json!({"previous": 99, "next": 60});
    write(&decision, &d);
    let error = command::run_with(&cfg, &f.out, Some(&decision), &NoDispatch, &no_key).unwrap_err();
    assert!(error.contains("image extension must name exact"), "{error}");
    assert_eq!(fs::read(&path).unwrap(), before);

    // A cap that does not increase is refused too.
    d["image_cap_extension"] = json!({"previous": 52, "next": 52});
    write(&decision, &d);
    assert!(
        command::run_with(&cfg, &f.out, Some(&decision), &NoDispatch, &no_key)
            .unwrap_err()
            .contains("image extension must name exact")
    );
    assert_eq!(fs::read(&path).unwrap(), before);

    // (2) Exact extensions for both caps, with preserved evidence, are accepted.
    let (raw, identity) = identity_of(60, 20);
    f.rewrite(&raw);
    let mut d = base(&identity);
    d["image_cap_extension"] = json!({"previous": 52, "next": 60});
    d["evaluation_cap_extension"] = json!({"previous": 13, "next": 20});
    write(&decision, &d);
    let error = command::run_with(&cfg, &f.out, Some(&decision), &NoDispatch, &no_key).unwrap_err();
    assert!(error.contains("stops before dispatch"), "{error}");

    let after: Run = serde_json::from_slice(&fs::read(&path).unwrap()).unwrap();
    assert_eq!(after.budget.max_images, 60);
    assert_eq!(after.budget.max_evaluations, 20);
    // Spend and evidence survive the extension.
    assert_eq!(after.budget.images, 30);
    assert_eq!(after.budget.evaluations, 6);
    // The only movement is the preparation charge the run makes itself.
    let charged = after
        .preparation_charge
        .as_ref()
        .map(|c| c.tokens)
        .unwrap_or(0);
    assert_eq!(after.budget.tokens, 1234 + charged);
    assert_eq!(after.trials.len(), 1);
    assert_eq!(after.current, Some(0));
    assert_eq!(
        after.visual.as_ref().unwrap().identity,
        "candidate-1",
        "preserved evidence was dropped"
    );
    assert_eq!(after.identity, identity);
    f.cleanup();
}

#[test]
fn two_consecutive_cap_only_resumes_keep_the_evidence_they_preserved() {
    use telperion_jev::tuning::engine::Run;
    let _serial = serial();
    let f = fixture::verifying_fixture(opening());
    let cfg = f.config_path.clone();
    let raw_config: Value = serde_json::from_slice(&fs::read(&cfg).unwrap()).unwrap();
    let no_key = || Err::<String, String>("stops before dispatch".into());

    command::run_with(&cfg, &f.out, None, &NoDispatch, &no_key).unwrap_err();
    let path = f.out.join("run.json");
    let mut state: Run = serde_json::from_slice(&fs::read(&path).unwrap()).unwrap();
    state.trials.push(telperion_jev::tuning::evaluation::Trial {
        key: "candidate-1".into(),
        identity: state.identity.clone(),
        seed: 1,
        round: 0,
        label: "baseline".into(),
        overrides: json!({}),
        ledger: None,
        feasible: true,
        reason: None,
        measurement: json!({}),
        comparisons: vec![telperion_jev::tuning::evaluation::Comparison {
            reference: "whole".into(),
            reference_weight: 1.,
            metric_weights: [1.; 5],
            target: [1.; 5],
            observed: [Some(1.); 5],
            images: serde_json::from_value(json!([raw_config["references"][0]])).unwrap(),
        }],
        score: Some(0.5),
        seconds: 0.,
        base: None,
        action: None,
        evidence: None,
        direction_mass: None,
        rule: None,
    });
    state.current = Some(0);
    state.visual = Some(
        serde_json::from_value(
            json!({"identity":"candidate-1","model":fixture::VISION_MODEL,
            "ledger":"receipt","cells":[],"defects":[],"findings":[]}),
        )
        .unwrap(),
    );
    state.budget.tokens = 4321;
    state.budget.images = 30;
    write(&path, &serde_json::to_value(&state).unwrap());

    let decision = f.root.join("decision.json");
    // Two cap-only resumes in a row, each preserving what the last measured.
    let pause = state.pause.clone().unwrap();
    for (previous, next) in [(52u64, 60u64), (60, 66)] {
        // A real run pauses again between resumes; re-arm the same pause shape.
        let mut paused: Run = serde_json::from_slice(&fs::read(&path).unwrap()).unwrap();
        if paused.pause.is_none() {
            let mut next_pause = pause.clone();
            next_pause.identity = paused.identity.clone();
            next_pause.basis.identity = paused.identity.clone();
            paused.pause = Some(next_pause);
            write(&path, &serde_json::to_value(&paused).unwrap());
        }
        let mut raw: Value = serde_json::from_slice(&fs::read(&cfg).unwrap()).unwrap();
        raw["budget"]["max_images"] = json!(next);
        f.rewrite(&raw);
        let identity = serde_json::from_value::<Config>(raw)
            .unwrap()
            .identity()
            .unwrap();
        write(
            &decision,
            &json!({"pause_id":paused.pause.as_ref().unwrap().id,"identity":paused.identity,
                "action":paused.pause.as_ref().unwrap().basis.proposed_action,
                "by":"owner","rationale":"owner raised the image cap",
                "next_identity":identity,"preserve_evidence":true,
                "image_cap_extension":{"previous":previous,"next":next},
                "experimental_pilot":{"purpose":"bounded","reason":"owner authorized",
                    "next_identity":identity,"max_tokens":902_431,"max_rounds":3,
                    "max_evaluations":13,"max_images":next,"max_visual_passes":26}}),
        );
        let error =
            command::run_with(&cfg, &f.out, Some(&decision), &NoDispatch, &no_key).unwrap_err();
        assert!(
            error.contains("stops before dispatch"),
            "cap {previous}->{next}: {error}"
        );
        let after: Run = serde_json::from_slice(&fs::read(&path).unwrap()).unwrap();
        assert_eq!(after.identity, identity);
        assert_eq!(after.budget.max_images, next);
        // The evidence measured under the first revision is still here, and
        // still visible to everything that reads it.
        assert_eq!(after.trials.len(), 1, "cap {next}: the trial was dropped");
        assert_eq!(after.current, Some(0));
        assert_eq!(after.visual.as_ref().unwrap().identity, "candidate-1");
        assert_eq!(after.budget.images, 30);
        assert_eq!(
            after.finalists().len(),
            1,
            "cap {next}: the preserved trial fell out of finalists"
        );
        assert!(after.measured_here(&after.trials[0].identity));
    }
    f.cleanup();
}
