//! A dispatch: the work handed to an agent, with the input identity and
//! design revision it was cut against, the tier and effort the policy chose,
//! and the result that came back with the model and effort that actually
//! ran. A result for another identity is obsolete; one without verification
//! or attribution is interrupted; neither advances the run.
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use super::state::Run;
use super::{now, ConductorError, Result};
use crate::ledger::Usage;
use crate::pipeline::canon::read_json;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Role {
    Routine,
    Investigate,
    Design,
    Implement,
    Visual,
    Review,
}

impl Role {
    pub fn key(self) -> &'static str {
        match self {
            Self::Routine => "routine",
            Self::Investigate => "investigate",
            Self::Design => "design",
            Self::Implement => "implement",
            Self::Visual => "visual",
            Self::Review => "review",
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Outcome {
    Verified,
    Failed,
    /// Named a different input identity or design revision than the dispatch.
    Obsolete,
    /// Incomplete: no verification or no attribution. Retained, never advancing.
    Interrupted,
}

/// What came back from a dispatched agent. The conductor copies it; it
/// verifies nothing itself beyond identity, attribution and completeness.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct DispatchResult {
    pub input_identity: String,
    #[serde(default)]
    pub design_revision: Option<String>,
    pub actual_model: String,
    pub actual_effort: String,
    #[serde(default)]
    pub usage: Option<Usage>,
    #[serde(default)]
    pub cost_usd: Option<f64>,
    #[serde(default)]
    pub wall_ms: Option<u64>,
    /// `verified`, `failed`, or empty for an interrupted attempt.
    #[serde(default)]
    pub verification: String,
    #[serde(default)]
    pub observed: String,
    /// The design handoff a design dispatch produced, by path and revision.
    #[serde(default)]
    pub handoff: Option<PathBuf>,
    #[serde(default)]
    pub failure: Option<String>,
    /// True when the result reported no usage and the dispatch's reservation
    /// was charged in its place: the spend stays bounded and known, and the
    /// report still lists the cost as unknown.
    #[serde(default)]
    pub usage_is_reservation: bool,
    #[serde(default)]
    pub outcome: Option<Outcome>,
    #[serde(default)]
    pub finished_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Dispatch {
    pub id: String,
    pub role: Role,
    pub route: String,
    pub tier: String,
    pub effort: String,
    /// The gap spec this dispatch serves, when it serves one.
    #[serde(default)]
    pub dependency: Option<String>,
    pub input_identity: String,
    #[serde(default)]
    pub design_revision: Option<String>,
    pub scope: String,
    /// Ledger identities of the judgments that chose this route.
    pub judgments: Vec<String>,
    pub reserved_tokens: u64,
    pub opened_at: String,
    #[serde(default)]
    pub result: Option<DispatchResult>,
}

impl Dispatch {
    pub fn open(&self) -> bool {
        self.result.is_none()
    }
    pub fn outcome(&self) -> Option<Outcome> {
        self.result.as_ref().and_then(|r| r.outcome)
    }
}

impl Run {
    /// Records a result against its dispatch. A result for another identity
    /// or design revision is obsolete and advances nothing; one without
    /// verification or attribution is interrupted and advances nothing.
    pub fn ingest(&mut self, id: &str, mut result: DispatchResult) -> Result<Outcome> {
        let dispatch = self
            .dispatches
            .iter_mut()
            .find(|d| d.id == id)
            .ok_or_else(|| ConductorError::Invalid(format!("no dispatch {id}")))?;
        if dispatch.result.is_some() {
            return Err(format!("dispatch {id} already carries a result").into());
        }
        let stale = result.input_identity != dispatch.input_identity
            || result.design_revision != dispatch.design_revision;
        let unattributed =
            result.actual_model.trim().is_empty() || result.actual_effort.trim().is_empty();
        let outcome = if stale {
            Outcome::Obsolete
        } else if unattributed || result.verification.is_empty() {
            Outcome::Interrupted
        } else if result.verification == "verified" && result.failure.is_none() {
            Outcome::Verified
        } else {
            Outcome::Failed
        };
        result.outcome = Some(outcome);
        if result.finished_at.is_empty() {
            result.finished_at = now();
        }
        match &result.usage {
            Some(usage) => {
                self.budget.tokens = self
                    .budget
                    .tokens
                    .saturating_add(usage.input_tokens.saturating_add(usage.output_tokens));
            }
            None => {
                // A finished dispatch that could not count charges what it
                // reserved. The first live run's host-written option set had
                // no count, and an unknown usage had made every later
                // continuation judgment unavailable.
                self.budget.tokens = self.budget.tokens.saturating_add(dispatch.reserved_tokens);
                result.usage_is_reservation = true;
            }
        }
        dispatch.result = Some(result);
        Ok(outcome)
    }
}

/// Reads a dispatch result file a dispatched agent wrote.
pub fn read_result(path: &Path) -> Result<DispatchResult> {
    serde_json::from_value(read_json(path)?)
        .map_err(|err| ConductorError::Invalid(format!("{}: {err}", path.display())))
}
