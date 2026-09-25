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

fn fixture(root: &Path) -> Value {
    let asset = root.join("asset");
    fs::write(&asset, "fixture fingerprint").unwrap();
    json!({"preset":"european-beech","seed":1,"initial_overrides":{},"owner_notes":"owner notes",
        "dials":[{"id":"limbs","path":"/skeleton/habit/lateralsPerStation","meaning":"limbs","min":1.,"max":4.,"integer":true,"small":1.,"substantial":2.}],
        "measure_binary":asset,"profiles":asset,"profile_id":"european-beech",
        "matched":{"headless":asset,"compare_script":asset,"references":asset,"refs":root,"catalogue":root,"scratch":root.join("scratch"),"height":1440,"numeric_references":["whole"]},
        "vision":{"program":"not-executed","args":[],"model":"mock","effort":"medium","timeout_seconds":10,"ledger":root.join("ledger")},
        "sheet":{"adapter":{"program":"not-executed","args":[],"model":"mock","effort":"medium","timeout_seconds":10,"ledger":root.join("ledger")},"protocol":asset},
        "references":[],"required":[{"item":"crown","view":"whole","seed":1},{"item":"crown","view":"whole","seed":42}],
        "checklist":"recognizable species at established quality","quality_anchors":[],
        "judgment_model":"jev-1.13.0","ledger":root.join("ledger"),
        "budget":{"evaluations":0,"images":0,"tokens":0,"rounds":0}})
}

fn opening() -> Value {
    json!({"evaluations":6,"images":30,"tokens":688_550,"rounds":2,"visual_passes":25})
}

#[test]
fn a_revision_charges_preparation_once_and_never_runs_twice_into_one_directory() {
    let _serial = serial();
    let f = fixture::verifying_fixture(opening());
    let config: Config = serde_json::from_slice(&fs::read(&f.config_path).unwrap()).unwrap();
    config
        .verify()
        .expect("the fixture must pass the real Config::verify");
    let no_key = || Err::<String, String>("fixture stops before dispatch".into());
    // Nothing asks for authority or calibration: the revision reaches its key.
    let error = command::run_with(&f.config_path, &f.out, &NoDispatch, &no_key).unwrap_err();
    assert!(error.contains("fixture stops before dispatch"), "{error}");
    let charged = f.run_json();
    assert_eq!(charged["stopped"], Value::Null);
    let charge = charged["preparation_charge"].clone();
    assert!(charge.is_object(), "preparation was not charged");
    assert_eq!(charge["visual_attempts"], 1);
    let tokens = charge["tokens"].as_u64().unwrap();
    assert_eq!(charged["budget"]["tokens"], 688_550 + tokens);
    assert_eq!(charged["budget"]["visual_passes"], 26);
    assert!(f.out.join("result.json").exists());
    // A revision runs once; the next starts in its own directory.
    let error = command::run_with(&f.config_path, &f.out, &NoDispatch, &no_key).unwrap_err();
    assert!(error.contains("a revision runs once"), "{error}");
    assert_eq!(f.run_json()["budget"], charged["budget"]);
    f.cleanup();
}

/// Answers the real caller. Records the exact state each judgment was sent.
struct Jev {
    states: std::sync::Mutex<Vec<Value>>,
}
impl telperion_jev::caller::Transport for Jev {
    fn send(
        &self,
        request: &telperion_jev::caller::HttpRequest,
    ) -> Result<telperion_jev::caller::HttpResponse, String> {
        let body: Value = serde_json::from_slice(request.body.as_ref().unwrap()).unwrap();
        self.states.lock().unwrap().push(body["state"].clone());
        let mut answers = serde_json::Map::new();
        for key in body["questions"].as_object().unwrap().keys() {
            answers.insert(
                key.clone(),
                json!({"choice":"small_increase","confidence":0.9,
                    "probabilities":{"small_increase":0.55,"substantial_increase":0.39,
                        "hold":0.03,"insufficient_evidence":0.03}}),
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

#[test]
#[ignore = "drives the compare stub, which runs under uv; run with --ignored"]
fn the_real_command_runs_a_revision_to_its_end_without_a_pause() {
    let _serial = serial();
    let f = fixture::verifying_fixture(json!({"evaluations":0,"images":0,"tokens":0,"rounds":0}));
    let key = || Ok::<String, String>("fixture-key".into());
    let jev = Jev {
        states: std::sync::Mutex::new(vec![]),
    };
    command::run_with(&f.config_path, &f.out, &jev, &key).unwrap();
    let ended = f.run_json();
    // The checkpoint's proposal is the revision's objectives, taken as it stands.
    assert!(ended["approval"]["ordered"].is_array(), "{ended}");
    assert!(ended["budget"]["evaluations"].as_u64().unwrap() >= 1);
    let result: Value =
        serde_json::from_slice(&fs::read(f.out.join("result.json")).unwrap()).unwrap();
    assert!(result["outcome"]["current"].is_object());
    // Rounds that keep nothing end the revision; nothing asks permission.
    let stopped = result["outcome"]["stopped"].as_str().unwrap();
    assert!(stopped.starts_with("stopped: "), "{stopped}");
    assert!(
        !stopped.contains("authority") && !stopped.contains("approval"),
        "{stopped}"
    );
    assert!(
        ended["budget"]["rounds"].as_u64().unwrap() >= 1,
        "no round ran"
    );
    let sheets = ended["judgment_inputs"].as_array().unwrap();
    assert!(
        sheets
            .iter()
            .any(|i| i["label"] == "uncalibrated sheet review"),
        "no round bought a contact sheet"
    );
    eprintln!("the revision ended {stopped}");
    // Every judgment's recorded input is the exact value the transport saw.
    let sent = jev
        .states
        .lock()
        .unwrap()
        .iter()
        .map(|s| telperion_jev::sha256_hex(&serde_json::to_vec(s).unwrap()))
        .collect::<Vec<_>>();
    // The contact sheet goes to the reviewer adapter, never to Jev.
    let to_jev = |i: &&Value| i["label"] != "uncalibrated sheet review";
    for input in ended["judgment_inputs"]
        .as_array()
        .unwrap()
        .iter()
        .filter(to_jev)
    {
        let hash = input["state_sha256"].as_str().unwrap();
        assert!(
            sent.contains(&hash.to_string()),
            "recorded a judgment the transport never received: {}",
            input["label"]
        );
    }
    f.cleanup();
}

#[test]
fn an_unexpressed_trait_must_be_in_the_inventory_and_name_its_spec() {
    let _serial = serial();
    let f = fixture::verifying_fixture(opening());
    let mut config: Config = serde_json::from_slice(&fs::read(&f.config_path).unwrap()).unwrap();
    let listed = |trait_id: &str, spec: &str| -> Value { json!([{"trait":trait_id,"spec":spec}]) };
    config.unexpressed = serde_json::from_value(listed("trait-core", "fn-111")).unwrap();
    config.verify().unwrap();
    for (trait_id, spec, refused) in [
        ("fruit-clusters-pendent", "fn-111", "not in the inventory"),
        ("trait-core", " ", "names no spec"),
    ] {
        config.unexpressed = serde_json::from_value(listed(trait_id, spec)).unwrap();
        let error = config.verify().unwrap_err();
        assert!(error.contains(refused), "{trait_id}/{spec:?}: {error}");
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
fn a_judgment_the_service_refuses_says_so() {
    let _serial = serial();
    struct Refusing;
    impl telperion_jev::caller::Transport for Refusing {
        fn send(
            &self,
            _: &telperion_jev::caller::HttpRequest,
        ) -> Result<telperion_jev::caller::HttpResponse, String> {
            Ok(telperion_jev::caller::HttpResponse {
                status: 400,
                body: br#"{"error":{"type":"max_tokens_exceeded","message":"too large"}}"#.to_vec(),
            })
        }
    }
    let root = std::env::temp_dir().join(telperion_jev::ledger::new_entry_id());
    fs::create_dir(&root).unwrap();
    let config_value = fixture(&root);
    let config: Config = serde_json::from_value(config_value).unwrap();
    let state = command::fresh(&config, "identity").unwrap();
    let mut services = telperion_jev::tuning::live::Live {
        config: &config,
        transport: &Refusing,
        key: "not-a-key",
    };
    let error = telperion_jev::tuning::engine::Services::propose(&mut services, &state, 0)
        .err()
        .expect("the refusal was not reported");
    assert!(
        error.starts_with(
            "judgment refused by the service: 400 max_tokens_exceeded; request bytes "
        ),
        "{error}"
    );
    let bytes = error
        .rsplit(' ')
        .next()
        .and_then(|n| n.parse::<usize>().ok())
        .unwrap();
    assert!(bytes > 100, "{error}");

    fs::remove_dir_all(&root).ok();
}

/// fn-149: a first revision from a name has no owner notes, and runs; what
/// cannot run is a revision with no reference photograph of the whole tree.
#[test]
fn a_first_revision_needs_no_owner_notes_but_a_whole_tree_reference() {
    let _serial = serial();
    let f = fixture::verifying_fixture(opening());
    let mut value: Value = serde_json::from_slice(&fs::read(&f.config_path).unwrap()).unwrap();
    value["owner_notes"] = json!("");
    let config: Config = serde_json::from_value(value.clone()).unwrap();
    config.verify().expect("no owner notes is a first revision");
    for reference in value["references"].as_array_mut().unwrap() {
        reference["view"] = json!("bark");
    }
    let config: Config = serde_json::from_value(value).unwrap();
    let err = config.verify().unwrap_err();
    assert_eq!(err, "no reference photograph of the whole tree");
    f.cleanup();
}
