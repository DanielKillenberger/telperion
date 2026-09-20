//! Reference-first protocol. An attributed inventory is evidence, not ground truth.
use super::{evaluation::Image, state::CellStatus, vision};
use crate::sha256_hex;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

pub const VERSION: &str = "reference-first-v1";
pub const INVENTORY_PROMPT: &str = "Inspect only the supplied reference images of the factual target species. Inventory the visible morphology that most defines recognition and believable reference character, ranked by importance. Use stable trait IDs; distinguish core recognition traits, secondary traits and acceptable variation. Cite reference IDs for each observed trait, mark uncertainty, and separate observations from possible causes. Do not infer that photographs show the same specimen or a causal change; their relationship is unknown. Do not assume a candidate, its defects, or any owner's assessment. Do not require photorealism or exact pixel matching. Inventory visible evidence rather than proposing a generator mechanism. Missing or ambiguous evidence must remain uncertain.";
pub const COMPARISON_PROMPT: &str = "Compare the candidate jointly against the frozen reference-only inventory and supplied photographs at the catalogue finish floor. Account explicitly for every core trait using its exact trait ID, citing render and reference evidence. Pass requires a supported match, fail a defining mismatch, unknown missing/clipped/ambiguous evidence. Do not silently dismiss a core reference trait as non-photorealism or downgrade an uncertain trait. Distinguish optional refinement and acceptable variation from defining reference character. Inventory statements are attributed interpretations, not infallible facts: if contradicted or unassessable, report unknown with grounds. Keep observations separate from causal hypotheses; no particular mechanism is required. Relative improvement is not absolute readiness. A clean candidate can pass with supported evidence for every core trait and no blockers.";

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReferenceImage {
    pub id: String,
    pub image: Image,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReferenceRequest {
    pub protocol: String,
    pub target_species: String,
    pub references: Vec<ReferenceImage>,
    pub specimen_relationship: String,
}
impl ReferenceRequest {
    pub fn from_comparison(request: &vision::Request) -> Self {
        Self {
            protocol: VERSION.into(),
            target_species: request.target_species.clone(),
            references: request
                .references
                .iter()
                .enumerate()
                .map(|(i, image)| ReferenceImage {
                    id: format!("reference-{i}"),
                    image: image.clone(),
                })
                .collect(),
            specimen_relationship: "unknown".into(),
        }
    }
    pub fn hash(&self) -> String {
        sha256_hex(&serde_json::to_vec(self).unwrap())
    }
    pub fn prompt_hash(&self) -> String {
        sha256_hex(INVENTORY_PROMPT.as_bytes())
    }
    pub fn verify(&self) -> Result<(), String> {
        if self.protocol != VERSION
            || self.target_species.trim().is_empty()
            || self.specimen_relationship != "unknown"
            || self.references.is_empty()
            || self.references.len() > 8
        {
            return Err("invalid reference-only request".into());
        }
        for (i, r) in self.references.iter().enumerate() {
            if r.id != format!("reference-{i}") {
                return Err("invalid reference ID".into());
            }
            r.image.verify()?;
        }
        Ok(())
    }
}
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Priority {
    Core,
    Secondary,
    Variation,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Trait {
    pub id: String,
    pub priority: Priority,
    pub observation: String,
    pub reference_ids: Vec<String>,
    pub uncertain: bool,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Inventory {
    pub request: ReferenceRequest,
    pub request_sha256: String,
    pub prompt_sha256: String,
    pub model: String,
    pub effort: String,
    pub ledger: String,
    pub traits: Vec<Trait>,
    pub observations: Vec<String>,
}
fn text(s: &str) -> bool {
    !s.trim().is_empty() && s.len() <= 4096
}
fn ids(values: &[String], allowed: &HashSet<String>) -> bool {
    !values.is_empty()
        && values.len() <= 12
        && values.iter().collect::<HashSet<_>>().len() == values.len()
        && values.iter().all(|id| allowed.contains(id))
}
impl Inventory {
    pub fn hash(&self) -> String {
        sha256_hex(&serde_json::to_vec(self).unwrap())
    }
    pub fn verify(&self) -> Result<(), String> {
        self.request.verify()?;
        if self.request_sha256 != self.request.hash()
            || self.prompt_sha256 != self.request.prompt_hash()
            || !text(&self.model)
            || !text(&self.effort)
            || !text(&self.ledger)
            || self.traits.is_empty()
            || self.traits.len() > 16
            || !self.traits.iter().any(|t| t.priority == Priority::Core)
            || self.observations.len() > 16
            || self.observations.iter().any(|s| !text(s))
        {
            return Err("invalid or stale inventory receipt".into());
        }
        let allowed = self
            .request
            .references
            .iter()
            .map(|r| r.id.clone())
            .collect();
        let mut seen = HashSet::new();
        for t in &self.traits {
            if !text(&t.id)
                || t.id.len() > 64
                || !seen.insert(&t.id)
                || !text(&t.observation)
                || !ids(&t.reference_ids, &allowed)
            {
                return Err("invalid inventory trait or source".into());
            }
        }
        Ok(())
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ComparisonRequest {
    pub protocol: String,
    pub prompt_sha256: String,
    pub comparison: vision::Request,
    pub inventory: Inventory,
}
impl ComparisonRequest {
    pub fn new(comparison: &vision::Request, inventory: Inventory) -> Self {
        Self {
            protocol: VERSION.into(),
            prompt_sha256: sha256_hex(COMPARISON_PROMPT.as_bytes()),
            comparison: comparison.blind(),
            inventory,
        }
    }
    pub fn hash(&self) -> String {
        sha256_hex(&serde_json::to_vec(self).unwrap())
    }
    pub fn verify(&self) -> Result<(), String> {
        self.comparison.verify()?;
        self.inventory.verify()?;
        if self.protocol != VERSION
            || self.prompt_sha256 != sha256_hex(COMPARISON_PROMPT.as_bytes())
            || self.comparison.hash() != self.comparison.blind().hash()
        {
            return Err("nonblind or stale comparison protocol".into());
        }
        if self.inventory.request.hash()
            != ReferenceRequest::from_comparison(&self.comparison).hash()
        {
            return Err("inventory reference/species mismatch".into());
        }
        Ok(())
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Coverage {
    pub trait_id: String,
    pub status: CellStatus,
    pub evidence_ids: Vec<String>,
    pub explanation: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ComparisonResult {
    pub request_sha256: String,
    pub visual: vision::Result,
    pub coverage: Vec<Coverage>,
}
impl ComparisonResult {
    pub fn bind(&mut self, request: &ComparisonRequest) -> Result<(), String> {
        request.verify()?;
        if self.request_sha256 != request.hash() {
            return Err("stale reference-first comparison".into());
        }
        if self.visual.assessment.cells.len() != request.comparison.required.len()
            || !self
                .visual
                .assessment
                .cells
                .iter()
                .zip(&request.comparison.required)
                .all(|((cell, _), required)| cell == required)
        {
            return Err("wrong required-cell cardinality or order".into());
        }
        self.visual.bind(&request.comparison)?;
        let packet = request
            .comparison
            .joint
            .as_ref()
            .ok_or("missing joint comparison")?;
        let allowed = packet.inputs.iter().map(|i| i.id.clone()).collect();
        let mut seen = HashSet::new();
        if self.coverage.len() > 16 {
            return Err("too many trait dispositions".into());
        }
        for c in &self.coverage {
            if !seen.insert(&c.trait_id)
                || !request.inventory.traits.iter().any(|t| t.id == c.trait_id)
                || !text(&c.explanation)
                || !ids(&c.evidence_ids, &allowed)
            {
                return Err("invalid trait coverage".into());
            }
            if !c.evidence_ids.iter().any(|id| {
                packet
                    .inputs
                    .iter()
                    .any(|i| &i.id == id && i.role == "render")
            }) || !c.evidence_ids.iter().any(|id| {
                packet
                    .inputs
                    .iter()
                    .any(|i| &i.id == id && i.role == "reference")
            }) {
                return Err("trait disposition lacks render/reference evidence".into());
            }
            let trait_source = request
                .inventory
                .traits
                .iter()
                .find(|t| t.id == c.trait_id)
                .unwrap();
            if c.status != CellStatus::Unknown
                && !c
                    .evidence_ids
                    .iter()
                    .any(|id| trait_source.reference_ids.contains(id))
            {
                return Err("trait disposition cites a different reference".into());
            }
        }
        let mut status = CellStatus::Pass;
        for t in request
            .inventory
            .traits
            .iter()
            .filter(|t| t.priority == Priority::Core)
        {
            match self.coverage.iter().find(|c| c.trait_id == t.id) {
                Some(c) if c.status == CellStatus::Fail => {
                    status = CellStatus::Fail;
                    break;
                }
                Some(c) if c.status == CellStatus::Pass && !t.uncertain => {}
                _ => status = CellStatus::Unknown,
            }
        }
        if status != CellStatus::Pass {
            for (_, s) in &mut self.visual.assessment.cells {
                if *s != CellStatus::Fail {
                    *s = status;
                }
            }
        }
        let evidence = serde_json::json!({"protocol":VERSION,"inventory_sha256":request.inventory.hash(),"inventory":request.inventory,"coverage":self.coverage});
        let observation = format!("Attributed reference-first evidence: {evidence}");
        if !self.visual.observations.contains(&observation) {
            self.visual.observations.push(observation.clone());
        }
        if !self.visual.assessment.observations.contains(&observation) {
            self.visual.assessment.observations.push(observation);
        }
        Ok(())
    }
}
