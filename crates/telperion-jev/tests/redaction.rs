//! What the two reviewers' prompts are allowed to carry. Neither adapter
//! dispatches here: the test runs each script's own `prepare` and reads the
//! prompt it built, offline, with no model in the path.
use serde_json::{json, Value};
use std::{fs, path::PathBuf, process::Command};
use telperion_jev::tuning::{evaluation::Image, look, sheet};

/// A trial key, as the engine writes one into the file name of a render.
const KEY: &str = "7c0ffeea1b2c3d4e5f60718293a4b5c6";

fn scratch() -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "jev-redaction-{}",
        telperion_jev::ledger::new_entry_id()
    ));
    fs::create_dir_all(&dir).unwrap();
    dir
}

/// A still whose path says everything the reviewer must not be told.
fn still(dir: &std::path::Path, body: &str) -> Image {
    let path = dir.join(format!("{KEY}-{body}-seed1-whole.png"));
    fs::write(&path, body.as_bytes()).unwrap();
    Image {
        path,
        sha256: telperion_jev::sha256_hex(body.as_bytes()),
        view: "whole".into(),
        seed: 1,
    }
}

fn priorities() -> Vec<look::Priority> {
    vec![look::Priority {
        id: "owner-crown".into(),
        observation: "Crown shape and foliage organization".into(),
    }]
}

fn script(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("scripts")
        .join(name)
}

const DRIVER: &str = r#"
import importlib.util, json, sys
spec = importlib.util.spec_from_file_location("adapter", sys.argv[1])
module = importlib.util.module_from_spec(spec)
spec.loader.exec_module(module)
envelope = json.load(sys.stdin)
try:
    paths, _schema, prompt = module.prepare(envelope)
except Exception as error:
    print(json.dumps({"error": str(error)}))
else:
    print(json.dumps({"prompt": prompt, "paths": [str(p) for p in paths]}))
"#;

/// Runs one adapter's own `prepare` and returns what it would have dispatched.
fn prepared(name: &str, envelope: &Value) -> Value {
    use std::io::Write;
    use std::process::Stdio;
    let mut child = Command::new("python3")
        .args(["-c", DRIVER])
        .arg(script(name))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    child
        .stdin
        .take()
        .unwrap()
        .write_all(&serde_json::to_vec(envelope).unwrap())
        .unwrap();
    let out = child.wait_with_output().unwrap();
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    serde_json::from_slice(&out.stdout).unwrap()
}

/// The prompt names no file and no trial key, and the adapter still holds the
/// real paths it has to attach the images from.
fn assert_redacted(name: &str, envelope: &Value, images: usize) {
    let dispatched = prepared(name, envelope);
    let prompt = dispatched["prompt"].as_str().expect("no prompt built");
    for secret in [KEY, ".png", "path", "/tmp", "seed1"] {
        assert!(!prompt.contains(secret), "{name} prompt says {secret}");
    }
    assert!(prompt.contains("sha256"), "the prompt lost the digests");
    let paths = dispatched["paths"].as_array().unwrap();
    assert_eq!(paths.len(), images);
    assert!(paths.iter().all(|p| p.as_str().unwrap().contains(KEY)));

    let mut without = envelope.clone();
    without.as_object_mut().unwrap().remove("prompt_request");
    assert!(
        prepared(name, &without)["error"]
            .as_str()
            .unwrap_or_default()
            .contains("prompt_request"),
        "{name} built a prompt with no redacted request"
    );
}

#[test]
fn the_contact_sheet_prompt_carries_no_path_and_no_trial_key() {
    let dir = scratch();
    let current = still(&dir, "current");
    let variants = vec![
        ("half".to_string(), still(&dir, "half")),
        ("one".to_string(), still(&dir, "one")),
    ];
    let plan = sheet::plan(
        "european-beech",
        "whole",
        1,
        &[still(&dir, "reference")],
        ("current", &current),
        &variants,
        &priorities(),
        "irregular outline",
    )
    .unwrap();
    assert_redacted("contact-sheet.py", &sheet::envelope(&plan.request), 4);
    // The redacted renders are numbered, in the order the sheet shows them.
    let redacted = sheet::prompt_request(&plan.request);
    assert_eq!(redacted["renders"][0]["label"], json!("1"));
    assert_eq!(
        redacted["renders"][2]["sha256"],
        json!(plan.request.renders[2].sha256)
    );
    fs::remove_dir_all(dir).unwrap();
}
