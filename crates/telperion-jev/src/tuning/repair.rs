//! One repair per answer (fn-80, owner 2026-09-24).
//!
//! The palm's live run stopped four times because the receipt refused a
//! reviewer's whole paid answer for a tidiness rule. Now an answer whose only
//! violations are tidiness ones (`tidy.rs`) goes back to the same reviewer
//! once, as a short text-only request carrying its own answer and the exact
//! broken rules; the adapter runs with no session persistence, so there is no
//! session to resume. The repaired answer passes the receipt again and code
//! trims whatever is still untidy. A trust violation refuses at any point.
use super::reference_first::{
    bind_response, ComparisonRequest, ComparisonResult, COMPARISON_PROMPT,
};
use super::vision::Adapter;
use crate::{ledger::Usage, sha256_hex};
use serde_json::{json, Value};
use std::{
    io::Write,
    path::PathBuf,
    process::{Command, Stdio},
};

pub const REPAIR_PROMPT: &str = "Your previous answer to this reference-first comparison broke the receipt's tidiness rules listed below. Return the corrected answer in the same schema. Fix only the listed violations: remove items to meet a cap, drop a repeated row, remove a repeated evidence id, shorten text over its limit, remove an item that cites no evidence. Cite only evidence ids your previous answer already cited. Change nothing else: keep every pass, every coverage status and all remaining text exactly as it was.";

/// One adapter call, its ledger record written before anything reads it.
struct Call {
    ok: bool,
    stdout: Vec<u8>,
    path: PathBuf,
}

fn dispatch(
    adapter: &Adapter,
    request: &ComparisonRequest,
    envelope: &Value,
    repair: Option<Value>,
) -> Result<Call, String> {
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
    let mut record = json!({"request":request,"model":adapter.model,"effort":adapter.effort,
        "exit":out.status.code(),"stdout":String::from_utf8_lossy(&out.stdout),
        "stderr":String::from_utf8_lossy(&out.stderr)});
    if let Some(repair) = repair {
        record["label"] = json!("repair");
        record["repair"] = repair;
    }
    std::fs::OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(&path)
        .map_err(|e| e.to_string())?
        .write_all(&serde_json::to_vec_pretty(&record).unwrap())
        .map_err(|e| e.to_string())?;
    Ok(Call {
        ok: out.status.success(),
        stdout: out.stdout,
        path,
    })
}

/// Stage B: one comparison call, and at most one repair of its answer.
pub fn assess(adapter: &Adapter, request: &ComparisonRequest) -> Result<ComparisonResult, String> {
    request.verify()?;
    if adapter.timeout_seconds == 0
        || adapter.timeout_seconds > 600
        || adapter.model.is_empty()
        || adapter.effort.is_empty()
    {
        return Err("invalid reference-first adapter".into());
    }
    let envelope = json!({"stage":"comparison","request":request,"request_sha256":request.hash(),
        "prompt":COMPARISON_PROMPT,"prompt_sha256":sha256_hex(COMPARISON_PROMPT.as_bytes())});
    let first = dispatch(adapter, request, &envelope, None)?;
    if !first.ok {
        return Err("reference-first adapter failed; reservation retained".into());
    }
    let raw: Value = serde_json::from_slice(&first.stdout).map_err(|e| e.to_string())?;
    let (mut trimmed, untidy) = bind_response(adapter, request, &raw, &first.path, None)?;
    if untidy.is_empty() {
        return Ok(trimmed);
    }
    let violations: Vec<String> = untidy.into_iter().map(|u| u.violation).collect();
    let envelope = json!({"stage":"repair","request":request,"request_sha256":request.hash(),
        "prompt":REPAIR_PROMPT,"prompt_sha256":sha256_hex(REPAIR_PROMPT.as_bytes()),
        "answer":raw["answer"],"violations":violations});
    let repair = json!({"of":first.path,"violations":violations});
    let call = dispatch(adapter, request, &envelope, Some(repair))?;
    let fixed: Value = match serde_json::from_slice(&call.stdout) {
        Ok(fixed) if call.ok => fixed,
        // A repair that failed to answer leaves the first answer, trimmed.
        _ => return Ok(trimmed),
    };
    let spent = fixed["usage"].clone();
    if fixed["status"] != "ok" || changed_verdicts(&raw["answer"], &fixed["answer"]) {
        // A repair that did not answer, or changed a verdict it was only asked
        // to tidy, is set aside: the first answer stands, trimmed and charged.
        add_usage(&mut trimmed.visual.usage, &spent);
        return Ok(trimmed);
    }
    let (mut repaired, _) = bind_response(adapter, request, &fixed, &call.path, Some(&violations))?;
    repaired.visual.usage = trimmed.visual.usage;
    add_usage(&mut repaired.visual.usage, &spent);
    Ok(repaired)
}

/// Whether a repair moved a pass or a coverage status: a trait's status is
/// the one its first row gave in the original answer.
fn changed_verdicts(before: &Value, after: &Value) -> bool {
    let status = |answer: &Value, id: &Value| {
        answer["coverage"]
            .as_array()
            .and_then(|rows| rows.iter().find(|r| &r["trait_id"] == id))
            .map(|r| r["status"].clone())
    };
    before["passes"] != after["passes"]
        || after["coverage"].as_array().is_none_or(|rows| {
            rows.iter()
                .any(|r| status(before, &r["trait_id"]) != Some(r["status"].clone()))
        })
}

/// Adds a call's reported usage; one that reported none adds nothing.
fn add_usage(total: &mut Option<Usage>, spent: &Value) {
    let (Some(input), Some(output)) = (
        spent["input_tokens"].as_u64(),
        spent["output_tokens"].as_u64(),
    ) else {
        return;
    };
    let base = total.clone().unwrap_or(Usage {
        input_tokens: 0,
        output_tokens: 0,
    });
    *total = Some(Usage {
        input_tokens: base.input_tokens.saturating_add(input),
        output_tokens: base.output_tokens.saturating_add(output),
    });
}
