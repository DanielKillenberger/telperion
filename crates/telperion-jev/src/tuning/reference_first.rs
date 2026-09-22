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
    #[serde(default, skip_serializing_if = "is_false")]
    pub production_requirements: bool,
    pub comparison: vision::Request,
    pub inventory: Inventory,
}
impl ComparisonRequest {
    pub fn new(comparison: &vision::Request, inventory: Inventory) -> Self {
        Self {
            protocol: VERSION.into(),
            prompt_sha256: sha256_hex(COMPARISON_PROMPT.as_bytes()),
            production_requirements: false,
            comparison: comparison.blind(),
            inventory,
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
        if self.protocol != VERSION
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
    let mut next = budget.clone();
    next.reserve_visual()?;
    next.reserve(0, 0, charge.tokens, 0)?;
    *budget = next;
    *record = Some(charge.clone());
    Ok(())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReplayCase {
    pub id: String,
    pub provenance: String,
    pub expected_ready: bool,
    pub request: ComparisonRequest,
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
    pub results: Vec<ComparisonResult>,
}
pub fn replay_score(
    manifest: &[u8],
    result: &ReplayResult,
) -> Result<(Replay, vision::ReplayScore), String> {
    let replay: Replay = serde_json::from_slice(manifest).map_err(|e| e.to_string())?;
    if replay.schema != "reference-first-replay-v1"
        || result.manifest_sha256 != sha256_hex(manifest)
        || result.results.len() != replay.cases.len()
    {
        return Err("reference-first requires fresh complete replay".into());
    }
    let mut score = vision::ReplayScore {
        positives: 0,
        negatives: 0,
        false_ready: 0,
        false_rejections: 0,
        abstentions: 0,
    };
    let mut ids = HashSet::new();
    for (case, observed) in replay.cases.iter().zip(&result.results) {
        if !text(&case.id)
            || !ids.insert(&case.id)
            || !text(&case.provenance)
            || observed.visual.assessment.model != replay.model
            || observed.visual.effort != replay.effort
            || observed.visual.usage.is_none()
            || case.request.inventory.model != replay.model
            || case.request.inventory.effort != replay.effort
        {
            return Err("unattributed reference-first replay".into());
        }
        let mut bound = observed.clone();
        bound.bind(&case.request)?;
        let r = &case.request.comparison;
        let got = super::state::ready(&r.required, &r.identity, &bound.visual.assessment);
        if case.expected_ready {
            score.positives += 1;
            score.false_rejections += usize::from(!got);
        } else {
            score.negatives += 1;
            score.false_ready += usize::from(got);
        }
        score.abstentions += usize::from(
            bound
                .visual
                .assessment
                .cells
                .iter()
                .any(|(_, s)| *s == CellStatus::Unknown),
        );
    }
    Ok((replay, score))
}

/// Executes only Stage B. Stage A is separately prepared, pinned and charged once.
pub fn assess(
    adapter: &vision::Adapter,
    request: &ComparisonRequest,
) -> Result<ComparisonResult, String> {
    use std::{
        io::Write,
        process::{Command, Stdio},
    };
    request.verify()?;
    if adapter.timeout_seconds == 0
        || adapter.timeout_seconds > 600
        || adapter.model.is_empty()
        || adapter.effort.is_empty()
    {
        return Err("invalid reference-first adapter".into());
    }
    let envelope = serde_json::json!({"stage":"comparison","request":request,"request_sha256":request.hash(),"prompt":COMPARISON_PROMPT,"prompt_sha256":sha256_hex(COMPARISON_PROMPT.as_bytes())});
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
    let out = child.wait_with_output().map_err(|e| e.to_string())?;
    std::fs::create_dir_all(&adapter.ledger).map_err(|e| e.to_string())?;
    let path = adapter
        .ledger
        .join(format!("{}.json", crate::ledger::new_entry_id()));
    let record = serde_json::json!({"request":request,"model":adapter.model,"effort":adapter.effort,"exit":out.status.code(),"stdout":String::from_utf8_lossy(&out.stdout),"stderr":String::from_utf8_lossy(&out.stderr)});
    std::fs::OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(&path)
        .map_err(|e| e.to_string())?
        .write_all(&serde_json::to_vec_pretty(&record).unwrap())
        .map_err(|e| e.to_string())?;
    if !out.status.success() {
        return Err("reference-first adapter failed; reservation retained".into());
    }
    let raw: serde_json::Value = serde_json::from_slice(&out.stdout).map_err(|e| e.to_string())?;
    bind_response(adapter, request, &raw, &path)
}

fn bind_response(
    adapter: &vision::Adapter,
    request: &ComparisonRequest,
    raw: &serde_json::Value,
    path: &std::path::Path,
) -> Result<ComparisonResult, String> {
    if raw["status"] != "ok"
        || raw["request_sha256"] != request.hash()
        || raw["prompt_sha256"] != request.prompt_sha256
        || raw["model"] != adapter.model
        || raw["effort"] != adapter.effort
    {
        return Err("stale or failed reference-first response".into());
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
    let visual:vision::Result=serde_json::from_value(serde_json::json!({"request_sha256":request.comparison.hash(),"assessment":{"identity":request.comparison.identity,"model":adapter.model,"ledger":path,"cells":cells,"defects":answer["defects"],"findings":answer["findings"]},"effort":adapter.effort,"usage":usage,"observations":answer["observations"]})).map_err(|e|e.to_string())?;
    let mut result = ComparisonResult {
        request_sha256: request.hash(),
        visual,
        coverage: serde_json::from_value(answer["coverage"].clone()).map_err(|e| e.to_string())?,
    };
    request.verify()?;
    result.bind(request)?;
    Ok(result)
}

/// Rebind the persisted Stage B receipt; a legacy ready visual is not convergence proof.
pub fn verify_convergence(
    adapter: &vision::Adapter,
    prepared: &RuntimeConfig,
    visual: &super::state::Visual,
) -> Result<(), String> {
    let path = std::path::Path::new(&visual.ledger);
    let metadata = std::fs::metadata(path).map_err(|e| e.to_string())?;
    if !metadata.is_file() || metadata.len() > 1_048_576 {
        return Err("invalid convergence receipt".into());
    }
    let record: serde_json::Value =
        serde_json::from_slice(&std::fs::read(path).map_err(|e| e.to_string())?)
            .map_err(|e| e.to_string())?;
    if record["exit"] != 0 || record["model"] != adapter.model || record["effort"] != adapter.effort
    {
        return Err("unattributed convergence receipt".into());
    }
    let request: ComparisonRequest =
        serde_json::from_value(record["request"].clone()).map_err(|e| e.to_string())?;
    let (inventory, _) = prepared.load(&adapter.model, &adapter.effort)?;
    if request.inventory.hash() != inventory.hash() {
        return Err("convergence inventory differs".into());
    }
    let raw: serde_json::Value = serde_json::from_str(
        record["stdout"]
            .as_str()
            .ok_or("missing convergence output")?,
    )
    .map_err(|e| e.to_string())?;
    let bound = bind_response(adapter, &request, &raw, path)?;
    if !super::state::ready(
        &request.comparison.required,
        &request.comparison.identity,
        &bound.visual.assessment,
    ) || serde_json::to_value(&bound.visual.assessment).unwrap()
        != serde_json::to_value(visual).unwrap()
    {
        return Err("convergence assessment differs from bound receipt".into());
    }
    Ok(())
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
        if self.coverage.len() > 16 {
            return Err("too many trait dispositions".into());
        }
        // A row naming something the inventory does not state - the live run
        // answered with a required cell's item name - is dropped and recorded
        // rather than refused, because refusing it costs the whole paid pass.
        // It leaves `coverage` before anything reads it, so it can never count
        // toward a trait's disposition, the core gate or readiness.
        // A row whose evidence is all real but names no render, or no
        // reference, is dropped the same way: it can compare nothing, and the
        // reviewer writes one for a trait that is about the references
        // themselves. An invented evidence id still refuses the pass below.
        let has_role = |c: &Coverage, role: &str| {
            c.evidence_ids
                .iter()
                .any(|id| packet.inputs.iter().any(|i| &i.id == id && i.role == role))
        };
        let (known, stray): (Vec<Coverage>, Vec<Coverage>) = std::mem::take(&mut self.coverage)
            .into_iter()
            .partition(|c| {
                request.inventory.traits.iter().any(|t| t.id == c.trait_id)
                    && (!ids(&c.evidence_ids, &allowed)
                        || (has_role(c, "render") && has_role(c, "reference")))
            });
        self.coverage = known;
        for c in &stray {
            let why = if request.inventory.traits.iter().any(|t| t.id == c.trait_id) {
                "without render and reference evidence"
            } else {
                "for unknown trait"
            };
            let note = format!(
                "dropped coverage row {why} {}: {:?} \u{2014} {}",
                c.trait_id, c.status, c.explanation
            );
            if !self.visual.observations.contains(&note) {
                self.visual.observations.push(note.clone());
            }
            if !self.visual.assessment.observations.contains(&note) {
                self.visual.assessment.observations.push(note);
            }
        }
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
        let mut seen = HashSet::new();
        for c in &self.coverage {
            if !seen.insert(&c.trait_id) || !text(&c.explanation) || !ids(&c.evidence_ids, &allowed)
            {
                return Err("invalid trait coverage".into());
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
            let finding = super::joint::Finding {
                observation: format!("Code-derived reference-first coverage gate: core trait coverage is {status:?}; this is a joint readiness constraint, not a new per-view model verdict. Inventory {}", request.inventory.hash()),
                evidence_ids: packet.inputs.iter().filter(|i| i.role == "reference" || i.role == "render").map(|i| i.id.clone()).collect(),
                impact: if status == CellStatus::Fail { super::joint::Impact::Blocker } else { super::joint::Impact::RequiredUnknown },
                uncertain: status == CellStatus::Unknown,
                causal_hypothesis: None,
            };
            if !self
                .visual
                .assessment
                .findings
                .iter()
                .any(|f| f.observation == finding.observation)
            {
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
        Ok(())
    }
}
