//! The one dispatch a contact sheet makes, and the receipt it leaves.
//!
//! The envelope carries the request twice: whole, for the hash and the image
//! files, and redacted, for the prompt. The adapter writes only the redacted
//! one into the prompt, because a render's file name can carry the trial key
//! the sheet exists to hide.
use super::{bind, Answer, Plan, Request, Verdict, PROMPT, UNCALIBRATED};
use crate::tuning::engine::Answer as Reply;
use crate::tuning::look::{redacted, Adapter};
use crate::tuning::vision;
use serde_json::{json, Value};
use std::path::PathBuf;

/// The request with every path taken out of it: each render is the number it
/// carries on the sheet and the digest of its bytes, and nothing else.
pub fn prompt_request(request: &Request) -> Value {
    json!({"schema":request.schema,"target_species":request.target_species,
        "view":request.view,"seed":request.seed,
        "references":request.references.iter().map(|i| redacted("reference",i))
            .collect::<Vec<_>>(),
        "renders":request.renders.iter().enumerate()
            .map(|(i,image)| json!({"label":Request::label(i),"sha256":image.sha256}))
            .collect::<Vec<_>>(),
        "priorities":request.priorities,"owner_notes":request.owner_notes})
}

/// What travels to the adapter: the request whole, for the hash and the image
/// files, and redacted, for the prompt.
pub fn envelope(request: &Request) -> Value {
    json!({"stage":"sheet","request":request,"request_sha256":request.hash(),
        "prompt_request":prompt_request(request),
        "prompt":PROMPT,"prompt_sha256":Request::prompt_hash()})
}

/// One isolated dispatch, one receipt, no retry. The plan stays here: the
/// reviewer is sent its request and never its order or its current label.
pub fn dispatch(adapter: &Adapter, plan: &Plan) -> Result<Reply<Verdict>, String> {
    let request = &plan.request;
    let envelope = envelope(request);
    let (raw, path) = shell("sheet", &adapter.adapter, &envelope, UNCALIBRATED)?;
    if raw["status"] != "ok"
        || raw["request_sha256"] != request.hash()
        || raw["prompt_sha256"] != Request::prompt_hash()
        || raw["model"] != adapter.adapter.model
        || raw["effort"] != adapter.adapter.effort
    {
        return Err("stale or failed sheet response; reservation retained".into());
    }
    let answer: Answer =
        serde_json::from_value(raw["answer"].clone()).map_err(|e| e.to_string())?;
    let tokens = raw["usage"]["input_tokens"]
        .as_u64()
        .zip(raw["usage"]["output_tokens"].as_u64())
        .map(|(a, b)| a + b);
    let ledger = path.display().to_string();
    let verdict = bind(plan, &answer, ledger.clone(), adapter.adapter.model.clone())?;
    Ok(Reply {
        value: verdict,
        tokens,
        ledger: Some(ledger),
    })
}

/// One isolated adapter dispatch, its receipt written before the answer is
/// read, so a refused answer still has a record of what was spent.
pub(super) fn shell(
    stage: &str,
    adapter: &vision::Adapter,
    envelope: &Value,
    uncalibrated: &str,
) -> Result<(Value, PathBuf), String> {
    use std::io::Write;
    use std::process::{Command, Stdio};
    if adapter.timeout_seconds == 0
        || adapter.timeout_seconds > 600
        || adapter.model.is_empty()
        || adapter.effort.is_empty()
    {
        return Err(format!("invalid {stage} adapter"));
    }
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
        .write_all(&serde_json::to_vec(envelope).unwrap())
        .map_err(|e| e.to_string())?;
    let out = child.wait_with_output().map_err(|e| e.to_string())?;
    std::fs::create_dir_all(&adapter.ledger).map_err(|e| e.to_string())?;
    let path = adapter
        .ledger
        .join(format!("{}.json", crate::ledger::new_entry_id()));
    let record = json!({"stage":stage,"request":envelope["request"],"model":adapter.model,
        "effort":adapter.effort,"exit":out.status.code(),
        "stdout":String::from_utf8_lossy(&out.stdout),
        "stderr":String::from_utf8_lossy(&out.stderr),"uncalibrated":uncalibrated});
    std::fs::OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(&path)
        .map_err(|e| e.to_string())?
        .write_all(&serde_json::to_vec_pretty(&record).unwrap())
        .map_err(|e| e.to_string())?;
    if !out.status.success() {
        return Err(format!("{stage} adapter failed; reservation retained"));
    }
    Ok((
        serde_json::from_slice(&out.stdout).map_err(|e| e.to_string())?,
        path,
    ))
}
