//! The one dispatch the progress review makes, and the receipt it leaves.
use super::{bind, Adapter, Answer, Request, Verdict, PROMPT, UNCALIBRATED};
use crate::tuning::engine::Answer as Reply;
use crate::tuning::vision;
use serde_json::{json, Value};
use std::path::PathBuf;

/// One isolated adapter dispatch, its receipt written before the answer is
/// read, so a refused answer still has a record of what was spent.
fn shell(adapter: &vision::Adapter, envelope: &Value) -> Result<(Value, PathBuf), String> {
    use std::io::Write;
    use std::process::{Command, Stdio};
    if adapter.timeout_seconds == 0
        || adapter.timeout_seconds > 600
        || adapter.model.is_empty()
        || adapter.effort.is_empty()
    {
        return Err("invalid progress adapter".into());
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
    let record = json!({"stage":"progress","request":envelope["request"],"model":adapter.model,
        "effort":adapter.effort,"exit":out.status.code(),
        "stdout":String::from_utf8_lossy(&out.stdout),
        "stderr":String::from_utf8_lossy(&out.stderr),"uncalibrated":UNCALIBRATED});
    std::fs::OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(&path)
        .map_err(|e| e.to_string())?
        .write_all(&serde_json::to_vec_pretty(&record).unwrap())
        .map_err(|e| e.to_string())?;
    if !out.status.success() {
        return Err("progress adapter failed; reservation retained".into());
    }
    Ok((
        serde_json::from_slice(&out.stdout).map_err(|e| e.to_string())?,
        path,
    ))
}

/// The adapter call itself. One isolated dispatch, one receipt, no retry.
pub fn dispatch(
    adapter: &Adapter,
    request: &Request,
    side: &str,
) -> Result<Reply<Verdict>, String> {
    let envelope = json!({"stage":"progress","request":request,"request_sha256":request.hash(),
        "prompt":PROMPT,"prompt_sha256":Request::prompt_hash()});
    let (raw, path) = shell(&adapter.adapter, &envelope)?;
    if raw["status"] != "ok"
        || raw["request_sha256"] != request.hash()
        || raw["prompt_sha256"] != Request::prompt_hash()
        || raw["model"] != adapter.adapter.model
        || raw["effort"] != adapter.adapter.effort
    {
        return Err("stale or failed progress response; reservation retained".into());
    }
    let answer: Answer =
        serde_json::from_value(raw["answer"].clone()).map_err(|e| e.to_string())?;
    let tokens = raw["usage"]["input_tokens"]
        .as_u64()
        .zip(raw["usage"]["output_tokens"].as_u64())
        .map(|(a, b)| a + b);
    let ledger = path.display().to_string();
    let verdict = bind(
        request,
        &answer,
        side,
        ledger.clone(),
        adapter.adapter.model.clone(),
    )?;
    Ok(Reply {
        value: verdict,
        tokens,
        ledger: Some(ledger),
    })
}
