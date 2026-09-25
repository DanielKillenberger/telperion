//! Reference-first protocol. An attributed inventory is evidence, not ground truth.
use super::tidy::{self, Untidy};
use super::unexpressed::{set_aside, Defect, Unexpressed};
use super::{evaluation::Image, state::CellStatus, vision};
use crate::sha256_hex;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

pub const VERSION: &str = "reference-first-v1";
/// The comparison's protocol. v2 (fn-136) ties each finding and defect to a
/// trait and names the known gaps; a v1 request or qualification is stale.
pub const COMPARISON_VERSION: &str = "reference-first-comparison-v2";
pub const INVENTORY_PROMPT: &str = "Inspect only the supplied reference images of the factual target species. Inventory the visible morphology that most defines recognition and believable reference character, ranked by importance. Use stable trait IDs; distinguish core recognition traits, secondary traits and acceptable variation. Cite reference IDs for each observed trait, mark uncertainty, and separate observations from possible causes. Do not infer that photographs show the same specimen or a causal change; their relationship is unknown. Do not assume a candidate, its defects, or any owner's assessment. Do not require photorealism or exact pixel matching. Inventory visible evidence rather than proposing a generator mechanism. Missing or ambiguous evidence must remain uncertain.";
pub const COMPARISON_PROMPT: &str = "Compare the candidate jointly against the frozen reference-only inventory and supplied photographs at the catalogue finish floor. Account explicitly for every core trait using its exact trait ID, citing render and reference evidence. Pass requires a supported match, fail a defining mismatch, unknown missing/clipped/ambiguous evidence. Do not silently dismiss a core reference trait as non-photorealism or downgrade an uncertain trait. Distinguish optional refinement and acceptable variation from defining reference character. Inventory statements are attributed interpretations, not infallible facts: if contradicted or unassessable, report unknown with grounds. Keep observations separate from causal hypotheses; no particular mechanism is required. Relative improvement is not absolute readiness. A clean candidate can pass with supported evidence for every core trait and no blockers. Give every finding and defect a trait_id: the exact inventory trait ID it concerns, or null when it concerns none. The traits listed in known_gaps are known gaps the generator cannot draw yet: do not assess them, and give any finding or defect about one its trait_id.";

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
    #[serde(default, skip_serializing_if = "is_false")]
    pub production_requirements: bool,
    pub comparison: vision::Request,
    pub inventory: Inventory,
    /// Traits the generator cannot draw yet, each with the spec that captures
    /// it; the reviewer does not assess them (fn-136).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub known_gaps: Vec<Unexpressed>,
}
impl ComparisonRequest {
    pub fn new(comparison: &vision::Request, inventory: Inventory) -> Self {
        Self {
            protocol: COMPARISON_VERSION.into(),
            prompt_sha256: sha256_hex(COMPARISON_PROMPT.as_bytes()),
            production_requirements: false,
            comparison: comparison.blind(),
            inventory,
            known_gaps: vec![],
        }
    }
    pub fn production(comparison: &vision::Request, inventory: Inventory) -> Self {
        let mut request = Self::new(comparison, inventory);
        request.comparison = comparison.clone();
        request.production_requirements = true;
        request
    }
    pub fn hash(&self) -> String {
        sha256_hex(&serde_json::to_vec(self).unwrap())
    }
    pub fn verify(&self) -> Result<(), String> {
        self.comparison.verify()?;
        self.inventory.verify()?;
        if self.protocol != COMPARISON_VERSION
            || self.prompt_sha256 != sha256_hex(COMPARISON_PROMPT.as_bytes())
            || (!self.production_requirements
                && self.comparison.hash() != self.comparison.blind().hash())
        {
            return Err("nonblind or stale comparison protocol".into());
        }
        if self.inventory.request.hash()
            != ReferenceRequest::from_comparison(&self.comparison).hash()
        {
            return Err("inventory reference/species mismatch".into());
        }
        if self.known_gaps.iter().any(|g| {
            g.spec.trim().is_empty() || !self.inventory.traits.iter().any(|t| t.id == g.trait_id)
        }) {
            return Err("a known gap names no spec or no inventory trait".into());
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
fn is_false(value: &bool) -> bool {
    !value
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FilePin {
    pub path: std::path::PathBuf,
    pub sha256: String,
}
impl FilePin {
    pub fn bytes(&self) -> Result<Vec<u8>, String> {
        let meta = std::fs::metadata(&self.path).map_err(|e| e.to_string())?;
        if !meta.is_file() || meta.len() > 1_048_576 {
            return Err("invalid pinned preparation file".into());
        }
        let bytes = std::fs::read(&self.path).map_err(|e| e.to_string())?;
        if sha256_hex(&bytes) != self.sha256 {
            return Err("changed preparation/inventory file".into());
        }
        Ok(bytes)
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RuntimeConfig {
    pub inventory: FilePin,
    pub preparation: FilePin,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct PreparationCharge {
    pub inventory_sha256: String,
    pub preparation_sha256: String,
    pub tokens: u64,
    pub visual_attempts: u64,
}
impl RuntimeConfig {
    pub fn load(
        &self,
        model: &str,
        effort: &str,
    ) -> Result<(Inventory, PreparationCharge), String> {
        let inventory: Inventory =
            serde_json::from_slice(&self.inventory.bytes()?).map_err(|e| e.to_string())?;
        inventory.verify()?;
        let prep: serde_json::Value =
            serde_json::from_slice(&self.preparation.bytes()?).map_err(|e| e.to_string())?;
        if prep["status"] != "ok"
            || prep["model"] != model
            || prep["effort"] != effort
            || inventory.model != model
            || inventory.effort != effort
            || prep["request_sha256"] != inventory.request_sha256
            || prep["prompt_sha256"] != inventory.prompt_sha256
            || prep["answer"]["traits"] != serde_json::to_value(&inventory.traits).unwrap()
            || prep["answer"]["observations"]
                != serde_json::to_value(&inventory.observations).unwrap()
        {
            return Err("unattributed preparation receipt".into());
        }
        let input = prep["usage"]["input_tokens"]
            .as_u64()
            .ok_or("unknown preparation usage")?;
        let output = prep["usage"]["output_tokens"]
            .as_u64()
            .ok_or("unknown preparation usage")?;
        let tokens = input
            .checked_add(output)
            .ok_or("preparation usage overflow")?;
        Ok((
            inventory,
            PreparationCharge {
                inventory_sha256: self.inventory.sha256.clone(),
                preparation_sha256: self.preparation.sha256.clone(),
                tokens,
                visual_attempts: 1,
            },
        ))
    }
}
pub fn verify_preparation(
    expected: Option<&PreparationCharge>,
    record: Option<&PreparationCharge>,
) -> Result<(), String> {
    if expected.is_some() && expected != record {
        return Err("preparation charge proof missing or changed".into());
    }
    Ok(())
}
pub fn charge_preparation(
    budget: &mut super::state::Budget,
    record: &mut Option<PreparationCharge>,
    charge: &PreparationCharge,
) -> Result<(), String> {
    if record.is_some() {
        return verify_preparation(Some(charge), record.as_ref());
    }
    if charge.visual_attempts != 1 {
        return Err("invalid preparation attempt count".into());
    }
    budget.reserve_visual();
    budget.reserve(0, 0, charge.tokens, 0);
    *record = Some(charge.clone());
    Ok(())
}

/// Executes only Stage B. Stage A is separately prepared, pinned and charged
/// once. An answer with tidiness violations goes back to the reviewer for one
/// repair (`repair.rs`).
pub fn assess(
    adapter: &vision::Adapter,
    request: &ComparisonRequest,
) -> Result<ComparisonResult, String> {
    super::repair::assess(adapter, request)
}

/// Binds one adapter answer and returns its tidiness violations, already
/// trimmed and recorded. `repair` names the violations a repaired answer was
/// asked to fix; it answers the repair prompt and says so in its notes.
pub(super) fn bind_response(
    adapter: &vision::Adapter,
    request: &ComparisonRequest,
    raw: &serde_json::Value,
    path: &std::path::Path,
    repair: Option<&[String]>,
) -> Result<(ComparisonResult, Vec<Untidy>), String> {
    let prompt = match repair {
        Some(_) => sha256_hex(super::repair::REPAIR_PROMPT.as_bytes()),
        None => request.prompt_sha256.clone(),
    };
    if raw["status"] != "ok"
        || raw["request_sha256"] != request.hash()
        || raw["prompt_sha256"] != prompt
        || raw["model"] != adapter.model
        || raw["effort"] != adapter.effort
    {
        let failed = "stale or failed reference-first response";
        return Err(super::vision::refused(failed, raw, ""));
    }
    let answer = &raw["answer"];
    let passes = answer["passes"].as_array().ok_or("missing passes")?;
    if passes.len() != request.comparison.required.len() {
        return Err("wrong required-cell cardinality".into());
    }
    let cells: Vec<_> = request
        .comparison
        .required
        .iter()
        .zip(passes)
        .map(|(cell, status)| serde_json::json!([cell, status]))
        .collect();
    let usage = crate::ledger::Usage {
        input_tokens: raw["usage"]["input_tokens"]
            .as_u64()
            .ok_or("unknown usage")?,
        output_tokens: raw["usage"]["output_tokens"]
            .as_u64()
            .ok_or("unknown usage")?,
    };
    let visual:vision::Result=serde_json::from_value(serde_json::json!({"request_sha256":request.comparison.hash(),"assessment":{"identity":request.comparison.identity,"model":adapter.model,"ledger":path,"cells":cells,"defects":[],"findings":answer["findings"]},"effort":adapter.effort,"usage":usage,"observations":answer["observations"]})).map_err(|e|e.to_string())?;
    let mut result = ComparisonResult {
        request_sha256: request.hash(),
        visual,
        coverage: serde_json::from_value(answer["coverage"].clone()).map_err(|e| e.to_string())?,
    };
    let defects: Vec<Defect> =
        serde_json::from_value(answer["defects"].clone()).map_err(|e| e.to_string())?;
    set_aside(&mut result.visual.assessment, defects, &request.known_gaps);
    request.verify()?;
    let untidy = result.bind_tidy(request)?;
    if let Some(violations) = repair {
        let note = format!(
            "reviewer repaired its answer once for {} tidiness violation(s): {}",
            violations.len(),
            violations.join("; ")
        );
        tidy::record(&mut result.visual, [&note]);
    }
    Ok((result, untidy))
}

impl ComparisonResult {
    pub fn bind(&mut self, request: &ComparisonRequest) -> Result<(), String> {
        self.bind_tidy(request).map(|_| ())
    }
    /// Binds the result and returns every tidiness rule its answer broke,
    /// each already trimmed or dropped and recorded (`tidy.rs`). A trust
    /// violation refuses it: a stale answer, a missing or mis-ordered required
    /// cell, an evidence id the request never supplied.
    pub fn bind_tidy(&mut self, request: &ComparisonRequest) -> Result<Vec<Untidy>, String> {
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
        let packet = request
            .comparison
            .joint
            .as_ref()
            .ok_or("missing joint comparison")?;
        tidy::invented_coverage(packet, &self.coverage)?;
        let mut untidy = self.visual.bind_tidy(&request.comparison)?;
        // A row naming no inventory trait, citing no render or no reference,
        // repeating a trait or carrying unusable text or evidence leaves
        // `coverage` before anything reads it, so it never counts toward a
        // trait's disposition, the core gate or readiness.
        let rows = tidy::coverage(request, &mut self.coverage);
        tidy::record(&mut self.visual, rows.iter().map(|u| &u.note));
        untidy.extend(rows);
        // A disposition that cites none of the references its trait was
        // inventoried from can compare nothing for that trait: it becomes
        // unknown and is recorded, rather than costing the whole paid pass.
        let mut recited = vec![];
        for c in &mut self.coverage {
            let Some(t) = request.inventory.traits.iter().find(|t| t.id == c.trait_id) else {
                continue;
            };
            if c.status != CellStatus::Unknown
                && !c.evidence_ids.iter().any(|id| t.reference_ids.contains(id))
            {
                recited.push(format!(
                    "downgraded coverage row citing a different reference {}: {:?} \u{2014} {}",
                    c.trait_id, c.status, c.explanation
                ));
                c.status = CellStatus::Unknown;
            }
        }
        tidy::record(&mut self.visual, &recited);
        // The dispositions travel on with the assessment, so a later one can
        // be compared with this one without reopening the receipt.
        self.visual.assessment.coverage = self
            .coverage
            .iter()
            .map(|c| super::state::TraitStatus {
                trait_id: c.trait_id.clone(),
                status: c.status,
            })
            .collect();
        let (status, _) = super::unexpressed::core_coverage(
            &request.inventory,
            &self.visual.assessment.coverage,
            &request.known_gaps,
        );
        if status != CellStatus::Pass {
            let finding = super::joint::Finding {
                observation: format!("Code-derived reference-first coverage gate: core trait coverage is {status:?}; this is a joint readiness constraint, not a new per-view model verdict. Inventory {}", request.inventory.hash()),
                evidence_ids: packet.inputs.iter().filter(|i| i.role == "reference" || i.role == "render").map(|i| i.id.clone()).collect(),
                impact: if status == CellStatus::Fail { super::joint::Impact::Blocker } else { super::joint::Impact::RequiredUnknown },
                uncertain: status == CellStatus::Unknown,
                causal_hypothesis: None,
                trait_id: None,
            };
            if !self
                .visual
                .assessment
                .findings
                .iter()
                .any(|f| f.observation == finding.observation)
            {
                // The gate's own finding never counts against the reviewer:
                // the palm's live run stopped on sixteen reviewer findings
                // plus this one (fn-80, 2026-09-24). Room is made by keeping
                // the reviewer's first fifteen.
                let room = tidy::findings(
                    &mut self.visual.assessment.findings,
                    tidy::MAX_FINDINGS - 1,
                    " when the code-derived coverage gate adds its own",
                );
                tidy::record(&mut self.visual, room.iter().map(|u| &u.note));
                untidy.extend(room);
                self.visual.assessment.findings.push(finding);
            }
            packet.verify_findings(&self.visual.assessment.findings)?;
        }
        let evidence = serde_json::json!({"protocol":VERSION,"inventory_sha256":request.inventory.hash(),"inventory":request.inventory,"coverage":self.coverage});
        let observation = format!("Attributed reference-first evidence: {evidence}");
        if !self.visual.observations.contains(&observation) {
            self.visual.observations.push(observation.clone());
        }
        if !self.visual.assessment.observations.contains(&observation) {
            self.visual.assessment.observations.push(observation);
        }
        Ok(untidy)
    }
}
