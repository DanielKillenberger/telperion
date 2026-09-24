//! Freezing and running a `reference-first-replay-v1`. The expected label is
//! declared here and never reaches the dispatched request: every case is
//! blinded by `ComparisonRequest::new`, so relabelling one cannot change the
//! bytes the adapter sees.
use super::{
    calibration::Journal,
    joint::{Framing, Packet},
    reference_first::{
        assess, replay_score, ComparisonRequest, ComparisonResult, Inventory, Replay, ReplayCase,
        ReplayResult,
    },
    vision,
};
use crate::sha256_hex;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::{fs, io::Write, path::Path};

pub const JOB_SCHEMA: &str = "reference-first-replay-job-v1";
const PER_CASE_TOKENS: u64 = 35000;

/// One declared render, with the provenance label the packet will carry.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Render {
    pub image: super::evaluation::Image,
    pub condition: String,
    pub framing: Framing,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Case {
    pub id: String,
    pub provenance: String,
    pub expected_ready: bool,
    pub identity: String,
    pub target_species: String,
    pub inventory: std::path::PathBuf,
    pub required: Vec<super::state::Cell>,
    pub renders: Vec<Render>,
    pub references: Vec<super::evaluation::Image>,
    pub quality_anchors: Vec<vision::QualityAnchor>,
    #[serde(default)]
    pub shots: Option<std::path::PathBuf>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Job {
    pub schema: String,
    pub model: String,
    pub effort: String,
    pub protocol: std::path::PathBuf,
    pub cases: Vec<Case>,
}

fn write(path: &Path, value: &Value) -> Result<(), String> {
    fs::OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(path)
        .map_err(|e| format!("{}: {e}", path.display()))?
        .write_all(&serde_json::to_vec_pretty(value).unwrap())
        .map_err(|e| e.to_string())
}

fn case_request(case: &Case, job: &Job) -> Result<ComparisonRequest, String> {
    let inventory: Inventory =
        serde_json::from_slice(&fs::read(&case.inventory).map_err(|e| e.to_string())?)
            .map_err(|e| e.to_string())?;
    inventory.verify()?;
    if inventory.model != job.model || inventory.effort != job.effort {
        return Err(format!(
            "case {}: inventory role differs from the replay",
            case.id
        ));
    }
    let mut request = vision::Request {
        schema: "tuning-vision-v3".into(),
        identity: case.identity.clone(),
        target_species: case.target_species.clone(),
        required: case.required.clone(),
        images: case.renders.iter().map(|r| r.image.clone()).collect(),
        references: case.references.clone(),
        checklist: super::joint::BLIND_CHECKLIST.into(),
        quality_anchors: case.quality_anchors.clone(),
        joint: None,
    };
    let mut packet = Packet::from_request(&request);
    if let Some(shots) = &case.shots {
        packet = packet.with_shots(shots)?;
    }
    // Each render carries the provenance the host declared for it; nothing here
    // relabels a still to make a case pass.
    for (input, render) in packet
        .inputs
        .iter_mut()
        .filter(|i| i.role == "render")
        .zip(&case.renders)
    {
        input.condition = render.condition.clone();
        input.framing = render.framing;
    }
    request.joint = Some(packet);
    request.verify()?;
    let prepared = ComparisonRequest::new(&request, inventory);
    prepared.verify()?;
    Ok(prepared)
}

/// Freezes the manifest. Refuses to overwrite one that already exists.
pub fn freeze(job_path: &Path, out: &Path) -> Result<Replay, String> {
    let job: Job = serde_json::from_slice(&fs::read(job_path).map_err(|e| e.to_string())?)
        .map_err(|e| e.to_string())?;
    if job.schema != JOB_SCHEMA || job.cases.is_empty() || job.cases.len() > 8 {
        return Err("invalid reference-first replay job".into());
    }
    let mut cases = vec![];
    for case in &job.cases {
        cases.push(ReplayCase {
            id: case.id.clone(),
            provenance: case.provenance.clone(),
            expected_ready: case.expected_ready,
            request: case_request(case, &job)?,
        });
    }
    let replay = Replay {
        schema: "reference-first-replay-v1".into(),
        model: job.model.clone(),
        effort: job.effort.clone(),
        protocol_sha256: sha256_hex(&fs::read(&job.protocol).map_err(|e| e.to_string())?),
        cases,
    };
    write(out, &serde_json::to_value(&replay).unwrap())?;
    Ok(replay)
}

/// One dispatch per case, never retried. A case already in the journal is
/// reused, so a resumed run cannot re-buy an answer it has.
pub fn run(
    manifest: &[u8],
    adapter: &vision::Adapter,
    protocol: &Path,
    journal_path: &Path,
    result_path: &Path,
    max_tokens: u64,
) -> Result<vision::ReplayScore, String> {
    let replay: Replay = serde_json::from_slice(manifest).map_err(|e| e.to_string())?;
    if replay.model != adapter.model || replay.effort != adapter.effort {
        return Err("wrong replay model".into());
    }
    // The adapter source is pinned at dispatch as well as at admission.
    if replay.protocol_sha256 != sha256_hex(&fs::read(protocol).map_err(|e| e.to_string())?) {
        return Err("adapter differs from the frozen protocol".into());
    }
    let mut journal = Journal::open(journal_path, max_tokens)?;
    let mut results = vec![];
    for case in &replay.cases {
        if let Some(recorded) = journal.case(&case.id) {
            let done: ComparisonResult = serde_json::from_value(recorded["result"].clone())
                .map_err(|e| {
                    format!(
                        "case {} was dispatched; its record is unreadable: {e}",
                        case.id
                    )
                })?;
            results.push(done);
            continue;
        }
        journal.reserve(PER_CASE_TOKENS)?;
        let outcome = assess(adapter, &case.request, &[]);
        let Ok(result) = outcome else {
            journal.note(
                &case.id,
                json!({"status":"failed","usage":null,"result":null}),
            )?;
            let _ = journal.settle(0);
            return Err(format!(
                "case {} failed; the attempt is charged and is not retried",
                case.id
            ));
        };
        let usage = result
            .visual
            .usage
            .as_ref()
            .ok_or("unknown vision usage; reserved spend retained")?;
        let tokens = usage
            .input_tokens
            .checked_add(usage.output_tokens)
            .ok_or("usage overflow")?;
        journal.note(
            &case.id,
            json!({"status":"settled","usage":usage,"result":result}),
        )?;
        journal.settle(tokens)?;
        results.push(result);
    }
    let result = ReplayResult {
        manifest_sha256: sha256_hex(manifest),
        results,
    };
    write(result_path, &serde_json::to_value(&result).unwrap())?;
    Ok(replay_score(manifest, &result)?.1)
}
