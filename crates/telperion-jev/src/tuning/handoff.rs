//! Grounded gap handoffs for fn-89. Every field is copied or derived from run
//! state; this module invents no diagnosis and dispatches no work.
use super::{
    priority::{Checkpoint, Gap},
    state::{CellStatus, Visual},
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub const SPENDING_NOTE: &str = "host owns repair estimate";
pub const DIALS_NOTE: &str =
    "Dial ids present in the router's state. Not evidence that any was tried.";
pub const HYPOTHESES_NOTE: &str =
    "Unproven causal hypotheses restated from the reviewer's findings, never established causes.";
pub const UNCERTAIN_INVESTIGATION: &str = "uncertainty handoff: no supported diagnosis";

/// One routing answer. `route` is thresholded; `raw_choice` is kept only as
/// diagnostic and never authorizes dispatch on its own.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct PriorityRoute {
    pub gap_id: Option<String>,
    pub rank: usize,
    pub route: String,
    pub raw_choice: Option<String>,
    pub confidence: Option<f64>,
    pub threshold: f64,
    pub ledger: String,
}

impl PriorityRoute {
    pub fn grounded(&self) -> bool {
        self.route != "insufficient_evidence"
    }
    pub fn existing_spec(&self) -> Option<String> {
        self.route
            .strip_prefix("existing:")
            .map(|id| id.to_string())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct DefectEvidence {
    pub id: String,
    pub role: String,
    pub view: String,
    pub seed: u32,
    pub sha256: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ObservedDefect {
    pub observation: String,
    pub reviewer_model: String,
    pub visual_ledger: String,
    pub evidence: Vec<DefectEvidence>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct CellOutcome {
    pub item: String,
    pub view: String,
    pub seed: u32,
    pub status: CellStatus,
}

/// One real evaluated candidate whose label names a dial. `score_before_round`
/// is the score of the candidate that was current when that round began.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Attempt {
    pub dial: String,
    pub round: u64,
    pub action_ledger: Option<String>,
    pub score_before_round: Option<f64>,
    pub score_after: Option<f64>,
    pub feasible: bool,
    pub reason: Option<String>,
    pub visual_outcome: Option<Vec<CellOutcome>>,
    /// The reviewer's comparative words on this attempt, where one was asked.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub review: Option<serde_json::Value>,
    /// What the attempt moved: its bundle's moves, or its single dial's step.
    /// Empty on a record written before moves were kept.
    #[serde(default)]
    pub moves: Vec<super::bundle::Move>,
    /// True when a round made this attempt the tree the loop stands on.
    #[serde(default)]
    pub adopted: bool,
    /// Whether the adoption stood the closing review: false when it was rolled
    /// back, absent when the attempt was never adopted.
    #[serde(default)]
    pub stood: Option<bool>,
    /// Why the closing review rolled the adoption back.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rolled_back: Option<String>,
}

/// Most moves one summary line names; a bundle beyond it is counted.
const MOVES_NAMED: usize = 3;

impl Attempt {
    /// One line a person reads: the round, what moved, what became of it.
    pub fn summary(&self) -> String {
        let mut s = format!("round {} {}", self.round, self.dial);
        match self.moves.as_slice() {
            [] => {}
            moves if moves.len() > MOVES_NAMED => {
                s.push_str(&format!(" ({} dials moved)", moves.len()))
            }
            moves => {
                let named: Vec<String> = moves
                    .iter()
                    .map(|m| format!("{} {} to {}", m.dial, number(m.from), number(m.to)))
                    .collect();
                s.push_str(&format!(" ({})", named.join(", ")));
            }
        }
        match (self.adopted, self.stood, &self.rolled_back) {
            (false, _, _) => {}
            (true, Some(false), Some(why)) => {
                s.push_str(&format!(", adopted and rolled back: {why}"))
            }
            (true, Some(false), None) => s.push_str(", adopted and rolled back"),
            (true, _, _) => s.push_str(", adopted and stood"),
        }
        if let Some(reason) = &self.reason {
            s.push_str(&format!(": {reason}"));
        }
        s
    }
}

fn number(v: f64) -> String {
    let s = format!("{v:.4}");
    s.trim_end_matches('0').trim_end_matches('.').to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Handoff {
    pub run_identity: String,
    pub candidate_key: String,
    pub checkpoint_sha256: String,
    pub gap_id: Option<String>,
    pub rank: usize,
    pub priority: String,
    pub observed_defect: ObservedDefect,
    pub route: String,
    pub existing_spec: Option<String>,
    pub judgment_ledger: String,
    pub raw_choice: Option<String>,
    pub confidence: Option<f64>,
    pub threshold: f64,
    pub dispatch_authorized: bool,
    pub dials_in_router_state: Vec<String>,
    pub dials_note: String,
    pub attempts: Vec<Attempt>,
    pub observations: Vec<String>,
    pub hypotheses: Vec<String>,
    pub hypotheses_note: String,
    pub unknowns: Vec<String>,
    pub proposed_investigation: String,
    pub proposed_spending: Option<Value>,
    pub proposed_spending_note: String,
    /// Raised in a bootstrap run, where no readiness was ever available.
    #[serde(default)]
    pub bootstrap: bool,
}

/// Findings the checkpoint used for this gap. A reviewer gap names its finding
/// by index; an owner addition matches on cited evidence.
pub fn findings_for<'a>(checkpoint: &'a Checkpoint, gap: &Gap) -> Vec<&'a super::joint::Finding> {
    if let Some(index) = gap
        .id
        .strip_prefix("finding-")
        .and_then(|i| i.parse::<usize>().ok())
    {
        return checkpoint.visual.findings.get(index).into_iter().collect();
    }
    checkpoint
        .visual
        .findings
        .iter()
        .filter(|f| {
            f.impact != super::joint::Impact::Supported
                && f.evidence_ids
                    .iter()
                    .any(|id| gap.evidence_ids.contains(id))
        })
        .collect()
}

pub fn observed_defect(checkpoint: &Checkpoint, gap: &Gap) -> ObservedDefect {
    ObservedDefect {
        observation: gap.observation.clone(),
        reviewer_model: checkpoint.visual.model.clone(),
        visual_ledger: checkpoint.visual.ledger.clone(),
        evidence: gap
            .evidence_ids
            .iter()
            .filter_map(|id| checkpoint.evidence.iter().find(|e| &e.id == id))
            .map(|e| DefectEvidence {
                id: e.id.clone(),
                role: e.role.clone(),
                view: e.image.view.clone(),
                seed: e.image.seed,
                sha256: e.image.sha256.clone(),
            })
            .collect(),
    }
}

/// Uncertain or unassessable evidence behind this gap: the reviewer's own
/// uncertain findings, plus any unknown cell on one of the gap's views.
pub fn unknowns(checkpoint: &Checkpoint, gap: &Gap, visual: Option<&Visual>) -> Vec<String> {
    let mut out = Vec::new();
    for f in findings_for(checkpoint, gap) {
        if f.uncertain || f.impact == super::joint::Impact::RequiredUnknown {
            out.push(f.observation.clone());
        }
    }
    for (cell, status) in visual.map(|v| &v.cells).unwrap_or(&checkpoint.visual.cells) {
        if *status == CellStatus::Unknown && gap.views.contains(&cell.view) {
            out.push(format!(
                "unknown cell: {} view {} seed {}",
                cell.item, cell.view, cell.seed
            ));
        }
    }
    out.dedup();
    out
}
