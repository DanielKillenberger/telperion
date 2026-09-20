//! A command adapter sees real images. Jev receives only its attributed observations.
use super::{
    evaluation::Image,
    state::{ready, Cell, Visual},
};
use crate::{ledger::Usage, sha256_hex};
use serde::{Deserialize, Serialize};
use std::{
    fs,
    io::Write,
    path::PathBuf,
    process::{Command, Stdio},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Request {
    pub schema: String,
    pub identity: String,
    pub required: Vec<Cell>,
    pub images: Vec<Image>,
    pub references: Vec<Image>,
    pub checklist: String,
    #[serde(default)]
    pub quality_anchors: Vec<QualityAnchor>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub joint: Option<super::joint::Packet>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct QualityAnchor {
    pub image: Image,
    pub provenance: String,
    pub scope: String,
}

impl Request {
    /// Blind development/evaluation projection: assessment labels are not inputs.
    pub fn blind(&self) -> Self {
        let mut r = self.clone();
        r.schema = "tuning-vision-v3".into();
        r.checklist = super::joint::BLIND_CHECKLIST.into();
        for c in &mut r.required {
            c.item = "reference_character".into();
        }
        for a in &mut r.quality_anchors {
            a.provenance = format!(
                "established catalogue anchor image sha256:{}",
                a.image.sha256
            );
            a.scope = "finish/style only; not species morphology".into();
        }
        r.joint = self
            .joint
            .clone()
            .or_else(|| Some(super::joint::Packet::from_request(&r)));
        if let Some(packet) = &mut r.joint {
            packet.reference_relation = "unknown".into();
            packet.relation_source = None;
        }
        r
    }
    pub fn verify(&self) -> std::result::Result<(), String> {
        if !["tuning-vision-v2", "tuning-vision-v3"].contains(&self.schema.as_str())
            || self.identity.is_empty()
            || self.required.is_empty()
            || self.references.is_empty()
            || self.checklist.is_empty()
        {
            return Err("missing visual evidence".into());
        }
        if self.schema == "tuning-vision-v3" && self.joint.is_none() {
            return Err("missing joint packet".into());
        }
        if let Some(packet) = &self.joint {
            packet.verify(self)?;
        }
        if self.quality_anchors.is_empty() {
            return Err("missing accepted catalogue quality anchors".into());
        }
        for anchor in &self.quality_anchors {
            if anchor.provenance.is_empty() || anchor.scope.is_empty() {
                return Err("unattributed quality anchor".into());
            }
            anchor.image.verify()?;
        }
        for image in self.images.iter().chain(&self.references) {
            image.verify()?;
        }
        for cell in &self.required {
            if !self
                .images
                .iter()
                .any(|i| i.view == cell.view && i.seed == cell.seed)
                || !self.references.iter().any(|i| i.view == cell.view)
            {
                return Err(format!(
                    "missing required image/reference for {} seed {}",
                    cell.view, cell.seed
                ));
            }
        }
        Ok(())
    }
    pub fn hash(&self) -> String {
        sha256_hex(&serde_json::to_vec(self).unwrap())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Result {
    pub request_sha256: String,
    pub assessment: Visual,
    pub effort: String,
    pub usage: Option<Usage>,
    pub observations: Vec<String>,
}

impl Result {
    pub fn bind(&mut self, request: &Request) -> std::result::Result<(), String> {
        if self.request_sha256 != request.hash() || self.assessment.identity != request.identity {
            return Err("stale joint result".into());
        }
        self.assessment.observations = self.observations.clone();
        self.assessment.joint = request.joint.clone();
        if let Some(packet) = &request.joint {
            packet.verify_findings(&self.assessment.findings)?;
            for (cell, status) in &mut self.assessment.cells {
                if packet.inputs.iter().any(|i| {
                    i.role == "render"
                        && i.view == cell.view
                        && i.seed == cell.seed
                        && i.framing == super::joint::Framing::Clipped
                }) || !self.assessment.findings.iter().any(|f| {
                    f.evidence_ids.iter().any(|id| {
                        packet.inputs.iter().any(|i| {
                            &i.id == id
                                && i.role == "render"
                                && i.view == cell.view
                                && i.seed == cell.seed
                        })
                    }) && f.evidence_ids.iter().any(|id| {
                        packet
                            .inputs
                            .iter()
                            .any(|i| &i.id == id && i.role == "reference")
                    })
                }) {
                    *status = super::state::CellStatus::Unknown;
                }
            }
            if self.assessment.findings.is_empty() {
                for (_, status) in &mut self.assessment.cells {
                    *status = super::state::CellStatus::Unknown;
                }
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Adapter {
    pub program: PathBuf,
    pub args: Vec<String>,
    pub model: String,
    pub effort: String,
    pub timeout_seconds: u64,
    pub ledger: PathBuf,
}

impl Adapter {
    pub fn assess(&self, request: &Request) -> std::result::Result<Result, String> {
        request.verify()?;
        if self.timeout_seconds == 0
            || self.timeout_seconds > 600
            || self.model.is_empty()
            || self.effort.is_empty()
        {
            return Err("invalid vision adapter".into());
        }
        let mut child = Command::new("timeout")
            .arg(self.timeout_seconds.to_string())
            .arg(&self.program)
            .args(&self.args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| e.to_string())?;
        let envelope = serde_json::json!({"request":request,"request_sha256":request.hash()});
        child
            .stdin
            .take()
            .ok_or("vision stdin unavailable")?
            .write_all(&serde_json::to_vec(&envelope).unwrap())
            .map_err(|e| e.to_string())?;
        let output = child.wait_with_output().map_err(|e| e.to_string())?;
        fs::create_dir_all(&self.ledger).map_err(|e| e.to_string())?;
        let record = self
            .ledger
            .join(format!("{}.json", crate::ledger::new_entry_id()));
        let receipt = serde_json::json!({"request":request,"model":self.model,"effort":self.effort,
            "status":output.status.code(),"stdout":String::from_utf8_lossy(&output.stdout),
            "stderr":String::from_utf8_lossy(&output.stderr)});
        let mut file = fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&record)
            .map_err(|e| e.to_string())?;
        file.write_all(&serde_json::to_vec_pretty(&receipt).unwrap())
            .map_err(|e| e.to_string())?;
        if !output.status.success() {
            return Err(format!("vision failed; receipt {}", record.display()));
        }
        let mut result: Result =
            serde_json::from_slice(&output.stdout).map_err(|e| e.to_string())?;
        if result.request_sha256 != request.hash()
            || result.assessment.identity != request.identity
            || result.assessment.model != self.model
            || result.effort != self.effort
        {
            return Err("stale or wrong-model visual assessment".into());
        }
        request.verify()?;
        result.bind(request)?;
        result.assessment.ledger = record.display().to_string();
        Ok(result)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReplayCase {
    pub id: String,
    pub provenance: String,
    pub expected_ready: bool,
    pub request: Request,
}

impl ReplayCase {
    pub fn blind_request(&self) -> Request {
        self.request.blind()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Replay {
    pub schema: String,
    pub model: String,
    pub effort: String,
    pub protocol_sha256: String,
    pub cases: Vec<ReplayCase>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReplayResult {
    pub manifest_sha256: String,
    pub results: Vec<Result>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplayScore {
    pub positives: usize,
    pub negatives: usize,
    pub false_ready: usize,
    pub false_rejections: usize,
    pub abstentions: usize,
}

pub fn replay_score(
    manifest: &[u8],
    result: &ReplayResult,
) -> std::result::Result<ReplayScore, String> {
    let replay: Replay = serde_json::from_slice(manifest).map_err(|e| e.to_string())?;
    if result.manifest_sha256 != sha256_hex(manifest)
        || result.results.len() != replay.cases.len()
        || replay.schema != "tuning-visual-replay-v2"
    {
        return Err("changed or incomplete visual replay".into());
    }
    let mut score = ReplayScore {
        positives: 0,
        negatives: 0,
        false_ready: 0,
        false_rejections: 0,
        abstentions: 0,
    };
    for (case, observed) in replay.cases.iter().zip(&result.results) {
        case.request.verify()?;
        if case.provenance.is_empty()
            || observed.request_sha256 != case.request.hash()
            || observed.assessment.identity != case.request.identity
            || observed.assessment.model != replay.model
            || observed.effort != replay.effort
            || observed.usage.is_none()
        {
            return Err("unattributed or stale replay result".into());
        }
        let mut bound = observed.clone();
        bound.bind(&case.request)?;
        let got = ready(
            &case.request.required,
            &case.request.identity,
            &bound.assessment,
        );
        if case.expected_ready {
            score.positives += 1;
            score.false_rejections += usize::from(!got);
        } else {
            score.negatives += 1;
            score.false_ready += usize::from(got);
        }
        if observed.assessment.cells.len() != case.request.required.len()
            || observed
                .assessment
                .cells
                .iter()
                .any(|(_, status)| *status == super::state::CellStatus::Unknown)
        {
            score.abstentions += 1;
        }
    }
    Ok(score)
}
