//! Stage A. Produces the reference-only inventory and the attributed receipt
//! that `RuntimeConfig::load` and `charge_preparation` consume, so the format
//! the admission gate demands is one a shipped command can emit.
use super::{
    live::Config,
    reference_first::{Inventory, ReferenceImage, ReferenceRequest, INVENTORY_PROMPT, VERSION},
};
use crate::sha256_hex;
use serde_json::{json, Value};
use std::{
    fs,
    io::Write,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

/// The reference-only request. The species field is the preset id, which is
/// what the production comparison path already sends.
pub fn request(config: &Config) -> Result<ReferenceRequest, String> {
    let request = ReferenceRequest {
        protocol: VERSION.into(),
        target_species: config.preset.clone(),
        references: config
            .references
            .iter()
            .enumerate()
            .map(|(i, image)| ReferenceImage {
                id: format!("reference-{i}"),
                image: image.clone(),
            })
            .collect(),
        specimen_relationship: "unknown".into(),
    };
    request.verify()?;
    Ok(request)
}

fn fresh(path: &Path) -> Result<(), String> {
    if path.exists() {
        return Err(format!(
            "refusing to overwrite {}; a paid attempt is never repeated over its own output",
            path.display()
        ));
    }
    Ok(())
}

fn write(path: &Path, value: &Value) -> Result<String, String> {
    let bytes = serde_json::to_vec_pretty(value).unwrap();
    fs::OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(path)
        .map_err(|e| e.to_string())?
        .write_all(&bytes)
        .map_err(|e| e.to_string())?;
    Ok(sha256_hex(&bytes))
}

/// Exactly one adapter dispatch, never retried. The attempt is journalled
/// before its outcome is known, because a failed paid call is still charged.
pub fn run(config: &Config, out: &Path) -> Result<PathBuf, String> {
    let (inventory_path, preparation_path) =
        (out.join("inventory.json"), out.join("preparation.json"));
    let journal_path = out.join("inventory-journal.json");
    fs::create_dir_all(out).map_err(|e| e.to_string())?;
    for path in [&inventory_path, &preparation_path, &journal_path] {
        fresh(path)?;
    }
    let adapter = &config.vision;
    if adapter.timeout_seconds == 0
        || adapter.timeout_seconds > 600
        || adapter.model.is_empty()
        || adapter.effort.is_empty()
    {
        return Err("invalid reference-first adapter".into());
    }
    let request = request(config)?;
    let envelope = json!({"stage":"inventory","request":request,"request_sha256":request.hash(),
        "prompt":INVENTORY_PROMPT,"prompt_sha256":request.prompt_hash()});
    write(
        &journal_path,
        &json!({"stage":"inventory","status":"dispatched","attempts":1,"retries":false,
            "model":adapter.model,"effort":adapter.effort,
            "request_sha256":request.hash(),"prompt_sha256":request.prompt_hash(),
            "usage":null,"note":"one paid attempt; a failure here is still charged"}),
    )?;

    let mut child = Command::new("timeout")
        .arg(adapter.timeout_seconds.to_string())
        .arg(&adapter.program)
        .args(&adapter.args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| e.to_string())?;
    child
        .stdin
        .take()
        .ok_or("missing adapter stdin")?
        .write_all(&serde_json::to_vec(&envelope).unwrap())
        .map_err(|e| e.to_string())?;
    let output = child.wait_with_output().map_err(|e| e.to_string())?;

    fs::create_dir_all(&adapter.ledger).map_err(|e| e.to_string())?;
    let ledger = adapter
        .ledger
        .join(format!("{}.json", crate::ledger::new_entry_id()));
    let record = json!({"stage":"inventory","request":request,"model":adapter.model,
        "effort":adapter.effort,"exit":output.status.code(),
        "stdout":String::from_utf8_lossy(&output.stdout),
        "stderr":String::from_utf8_lossy(&output.stderr)});
    let ledger_sha256 = write(&ledger, &record)?;

    let settle = |journal: &Path, status: &str, usage: &Value| {
        let _ = fs::remove_file(journal);
        let _ = write(
            journal,
            &json!({"stage":"inventory","status":status,"attempts":1,"retries":false,
                "model":adapter.model,"effort":adapter.effort,
                "request_sha256":request.hash(),"prompt_sha256":request.prompt_hash(),
                "usage":usage,"ledger":ledger,"ledger_sha256":ledger_sha256}),
        );
    };
    let raw: Value = match serde_json::from_slice(&output.stdout) {
        Ok(raw) if output.status.success() => raw,
        _ => {
            settle(&journal_path, "failed", &Value::Null);
            return Err(format!(
                "reference-first Stage A failed; the attempt is charged. Ledger {}",
                ledger.display()
            ));
        }
    };
    settle(&journal_path, "settled", &raw["usage"]);

    if raw["status"] != "ok"
        || raw["request_sha256"] != request.hash()
        || raw["prompt_sha256"] != request.prompt_hash()
        || raw["model"] != adapter.model
        || raw["effort"] != adapter.effort
    {
        return Err("stale or failed Stage A response; the attempt is charged".into());
    }
    let inventory = Inventory {
        request_sha256: request.hash(),
        prompt_sha256: request.prompt_hash(),
        request,
        model: adapter.model.clone(),
        effort: adapter.effort.clone(),
        ledger: format!("{}#sha256:{ledger_sha256}", ledger.display()),
        traits: serde_json::from_value(raw["answer"]["traits"].clone())
            .map_err(|e| format!("Stage A traits: {e}"))?,
        observations: serde_json::from_value(raw["answer"]["observations"].clone())
            .map_err(|e| format!("Stage A observations: {e}"))?,
    };
    inventory.verify()?;
    // The receipt is the adapter's own bytes: `RuntimeConfig::load` re-checks
    // them against the inventory, so nothing here may reshape them.
    write(&preparation_path, &raw)?;
    write(&inventory_path, &serde_json::to_value(&inventory).unwrap())?;
    Ok(inventory_path)
}
