//! A command adapter sees real images. Jev receives only its attributed observations.
use super::{
    evaluation::Image,
    state::{Cell, Visual},
    tidy::{self, Untidy},
};
use crate::{ledger::Usage, sha256_hex};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Request {
    pub schema: String,
    pub identity: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub target_species: String,
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
        if self.schema == "tuning-vision-v3" && self.target_species.trim().is_empty() {
            return Err("missing target species".into());
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
        self.bind_tidy(request).map(|_| ())
    }
    /// Binds the result to its request and returns the tidiness rules its
    /// findings broke, each already trimmed or dropped and recorded; a trust
    /// violation refuses it (`tidy.rs`).
    pub fn bind_tidy(&mut self, request: &Request) -> std::result::Result<Vec<Untidy>, String> {
        if self.request_sha256 != request.hash() || self.assessment.identity != request.identity {
            return Err("stale joint result".into());
        }
        self.assessment.observations = self.observations.clone();
        self.assessment.joint = request.joint.clone();
        let mut untidy = vec![];
        if let Some(packet) = &request.joint {
            tidy::invented_findings(packet, &self.assessment.findings)?;
            untidy = tidy::findings(&mut self.assessment.findings, tidy::MAX_FINDINGS, "");
            tidy::record(self, untidy.iter().map(|u| &u.note));
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
        Ok(untidy)
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
