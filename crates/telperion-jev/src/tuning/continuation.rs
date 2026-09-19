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
    let next = basis
        .next_tokens
        .filter(|n| *n > 0)
        .ok_or("unknown next token allowance")?;
    if budget
        .tokens
        .checked_add(next)
        .is_none_or(|n| n > budget.max_tokens)
        || budget.evaluations >= budget.max_evaluations
        || budget.images >= budget.max_images
        || budget.rounds >= budget.max_rounds
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
