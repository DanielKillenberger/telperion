//! Shared fn-89 continuation contract. This module dispatches no work.
use super::state::Budget;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

pub const VERSION: &str = "continuation-v1";

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Basis {
    pub identity: String,
    pub proposed_action: String,
    pub evidence: Vec<String>,
    pub recent_outcomes: Vec<String>,
    pub next_tokens: Option<u64>,
    pub estimate_basis: String,
    pub usage_known: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Assessment {
    pub identity: String,
    pub ledger: String,
    pub tractability: String,
    pub progress: String,
    pub risk: String,
}

pub fn questions() -> Value {
    json!({
        "tractability":{"type":"choice","instructions":"Does the proposed action have a supported bounded path to resolving the observed defect? A complex feature with a reviewed bounded design may qualify. Judge the evidence, not confidence as a success probability.","criteria":{
            "supported":"Evidence supports the diagnosis and bounded next attempt.",
            "unsupported":"Diagnosis is contradicted or the path is unsupported.",
            "insufficient_evidence":"Missing evidence or unknown effort prevents judgment."}},
        "progress":{"type":"choice","instructions":"Do observed progress and recent failed attempts justify this next action?","criteria":{
            "supported":"First evidenced attempt, measurable progress, or new evidence justifies a changed approach.",
            "repeated_failure":"Equivalent failed attempts without new evidence make another unjustified.",
            "insufficient_evidence":"Recent outcomes or evidence are missing."}},
        "risk":{"type":"choice","instructions":"Is the proposed action's implementation risk bounded by the stated scope and verification?","criteria":{
            "bounded":"Known limited effects and verification, including reviewed complex work.",
            "unusual":"Unexpected shared behavior, security, data loss or other unusual implementation risk.",
            "insufficient_evidence":"Effects or verification are unknown."}}
    })
}

/// Only the risk question, word for word from `questions()`. The pre-dispatch
/// judgment asks this alone; tractability and progress are no longer asked.
pub fn risk_only() -> Value {
    let all = questions();
    json!({ "risk": all["risk"] })
}

pub fn assess(
    basis: &Basis,
    budget: &Budget,
    assessment: Option<&Assessment>,
    validated: bool,
) -> Result<(), String> {
    if !validated {
        return Err("continuation calibration unavailable".into());
    }
    if basis.identity.is_empty()
        || basis.proposed_action.is_empty()
        || basis.evidence.is_empty()
        || basis.evidence.iter().any(|s| s.trim().is_empty())
        || basis.estimate_basis.is_empty()
        || !basis.usage_known
    {
        return Err("missing bounded attempt basis or unknown prior usage".into());
    }
    // The next attempt's allowance is only needed to test it against a token
    // cap; with none set there is nothing to fit it into.
    let next = match budget.max_tokens {
        Some(_) => basis
            .next_tokens
            .filter(|n| *n > 0)
            .ok_or("unknown next token allowance")?,
        None => basis.next_tokens.unwrap_or(0),
    };
    let at = |spent: u64, cap: Option<u64>| cap.is_some_and(|cap| spent >= cap);
    if budget
        .tokens
        .checked_add(next)
        .is_none_or(|n| super::state::over(n, budget.max_tokens))
        || at(budget.evaluations, budget.max_evaluations)
        || at(budget.images, budget.max_images)
        || at(budget.rounds, budget.max_rounds)
    {
        return Err("hard budget exhausted".into());
    }
    let a = assessment.ok_or("missing continuation assessment")?;
    if a.identity != basis.identity || a.ledger.is_empty() {
        return Err("stale or unattributed continuation assessment".into());
    }
    if a.tractability != "supported" || a.progress != "supported" || a.risk != "bounded" {
        return Err("next attempt unjustified: human decision required".into());
    }
    Ok(())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Pause {
    pub id: String,
    pub identity: String,
    pub reason: String,
    pub basis: Basis,
    pub decision_requested: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HumanDecision {
    pub pause_id: String,
    pub identity: String,
    pub action: String,
    pub by: String,
    pub rationale: String,
    #[serde(default)]
    pub next_identity: Option<String>,
    #[serde(default)]
    pub recover_interrupted: bool,
    #[serde(default)]
    pub preserve_evidence: bool,
    #[serde(default)]
    pub token_cap_extension: Option<TokenCapExtension>,
    #[serde(default)]
    pub round_cap_extension: Option<TokenCapExtension>,
    #[serde(default)]
    pub visual_cap_extension: Option<TokenCapExtension>,
    #[serde(default)]
    pub image_cap_extension: Option<TokenCapExtension>,
    #[serde(default)]
    pub evaluation_cap_extension: Option<TokenCapExtension>,
    #[serde(default)]
    pub visual_reconciliation: Option<VisualReconciliation>,
    #[serde(default)]
    pub baseline_amendment: Option<BaselineAmendment>,
    #[serde(default)]
    pub experimental_pilot: Option<PilotAuthority>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub diagnosis: Option<Diagnosis>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub external_usage: Option<ExternalUsage>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub priority_approval: Option<super::priority::Approval>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ExternalUsage {
    pub previous_tokens: u64,
    pub next_tokens: u64,
    pub reason: String,
    pub ledgers: Vec<ExternalLedger>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ExternalLedger {
    pub path: std::path::PathBuf,
    pub sha256: String,
    pub id: String,
    pub tool: String,
    pub model: String,
    pub identity: String,
}
impl ExternalUsage {
    pub fn verify(
        &self,
        current: u64,
        cap: Option<u64>,
        imported: &std::collections::HashSet<String>,
    ) -> Result<u64, String> {
        if self.previous_tokens != current
            || self.reason.trim().is_empty()
            || self.ledgers.is_empty()
            || self.ledgers.len() > 32
        {
            return Err("invalid external usage scope".into());
        }
        let mut seen = imported.clone();
        let mut sum = 0u64;
        for pin in &self.ledgers {
            if [&pin.id, &pin.tool, &pin.model, &pin.identity]
                .iter()
                .any(|s| s.trim().is_empty())
                || pin.tool == "tuning"
                || !seen.insert(pin.id.clone())
            {
                return Err("duplicate, native or unidentified external ledger".into());
            }
            let bytes = std::fs::read(&pin.path).map_err(|e| format!("external ledger: {e}"))?;
            if pin.sha256.len() != 64 || crate::sha256_hex(&bytes) != pin.sha256 {
                return Err("external ledger hash mismatch".into());
            }
            let e: crate::ledger::LedgerEntry =
                serde_json::from_slice(&bytes).map_err(|e| e.to_string())?;
            if e.id != pin.id
                || e.tool != pin.tool
                || e.model != pin.model
                || e.identity != pin.identity
                || e.error.is_some()
                || e.state_sha256.len() != 64
                || !e.state_sha256.bytes().all(|b| b.is_ascii_hexdigit())
                || e.identity
                    != crate::ledger::derived_identity(&e.state_sha256, &e.questions, &e.model)
                || e.questions.as_object().is_none_or(|q| q.is_empty())
                || e.answers.as_object().is_none_or(|a| {
                    a.is_empty()
                        || e.questions
                            .as_object()
                            .unwrap()
                            .keys()
                            .any(|k| !a.contains_key(k))
                })
            {
                return Err("external ledger identity or success mismatch".into());
            }
            let u = e.usage.ok_or("external usage unknown")?;
            sum = sum
                .checked_add(u.input_tokens)
                .and_then(|n| n.checked_add(u.output_tokens))
                .ok_or("external usage overflow")?;
        }
        let next = current.checked_add(sum).ok_or("external total overflow")?;
        if sum == 0 || next != self.next_tokens || super::state::over(next, cap) {
            return Err("external usage sum or cap mismatch".into());
        }
        Ok(next)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Diagnosis {
    pub target_identity: String,
    pub author: String,
    pub model: String,
    pub findings: Vec<Finding>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Finding {
    pub claim: String,
    pub source: std::path::PathBuf,
    pub sha256: String,
    pub excerpt: String,
}
impl Diagnosis {
    pub fn verify(&self, identity: &str) -> Result<(), String> {
        use std::io::Read;
        if self.target_identity != identity
            || [&self.author, &self.model]
                .iter()
                .any(|s| s.trim().is_empty() || s.len() > 200)
            || self.findings.is_empty()
            || self.findings.len() > 8
        {
            return Err("invalid diagnosis identity, attribution or finding count".into());
        }
        for f in &self.findings {
            if f.claim.trim().is_empty()
                || f.claim.len() > 1024
                || f.excerpt.trim().is_empty()
                || f.excerpt.len() > 2048
                || f.sha256.len() != 64
                || !f.sha256.bytes().all(|b| b.is_ascii_hexdigit())
            {
                return Err("invalid diagnosis finding or hash".into());
            }
            if !std::fs::metadata(&f.source)
                .map_err(|e| format!("diagnosis source: {e}"))?
                .is_file()
            {
                return Err("diagnosis source must be a regular file".into());
            }
            let file =
                std::fs::File::open(&f.source).map_err(|e| format!("diagnosis source: {e}"))?;
            if !file.metadata().map_err(|e| e.to_string())?.is_file() {
                return Err("diagnosis source must be a regular file".into());
            }
            let mut bytes = Vec::new();
            file.take(1_048_577)
                .read_to_end(&mut bytes)
                .map_err(|e| e.to_string())?;
            if bytes.len() > 1_048_576
                || crate::sha256_hex(&bytes) != f.sha256
                || !bytes
                    .windows(f.excerpt.len())
                    .any(|w| w == f.excerpt.as_bytes())
            {
                return Err("diagnosis source hash or excerpt mismatch".into());
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct VisualReconciliation {
    pub previous_cap: u64,
    pub paid_ledgers: Vec<std::path::PathBuf>,
    pub reason: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BaselineAmendment {
    pub previous: Value,
    pub next: Value,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PilotAuthority {
    /// The authority must name the bootstrap mode explicitly; a run cannot
    /// slip into it on a config flag alone.
    #[serde(default)]
    pub visual_bootstrap: bool,
    pub purpose: String,
    pub reason: String,
    pub next_identity: String,
    /// The caps the run carries, restated exactly; a run with none restates
    /// none.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_rounds: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_evaluations: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_images: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_visual_passes: Option<u64>,
}
impl PilotAuthority {
    pub fn verify(&self, identity: &str, budget: &Budget) -> Result<(), String> {
        if self.purpose.trim().is_empty()
            || self.reason.trim().is_empty()
            || self.next_identity != identity
            || self.max_tokens != budget.max_tokens
            || self.max_rounds != budget.max_rounds
            || self.max_evaluations != budget.max_evaluations
            || self.max_images != budget.max_images
            || self.max_visual_passes != budget.max_visual_passes
        {
            return Err("experimental pilot authority mismatch".into());
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TokenCapExtension {
    pub previous: u64,
    pub next: u64,
}

impl Pause {
    /// Only resolves the scoped pause; caller must obtain a fresh assessment.
    pub fn resume(&self, decision: &HumanDecision) -> Result<(), String> {
        if decision.pause_id != self.id
            || decision.identity != self.identity
            || decision.action != self.basis.proposed_action
            || decision.by.trim().is_empty()
            || decision.rationale.trim().is_empty()
        {
            return Err("resume requires the scoped human decision".into());
        }
        Ok(())
    }
}
