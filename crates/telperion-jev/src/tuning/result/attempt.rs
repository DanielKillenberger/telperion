//! What each evaluated candidate did, as the result names it: the moves, the
//! reviewer's words, and whether an adoption stood.
use super::{still, Still};
use crate::tuning::{engine::Run, evaluation::Trial, priority::Gap, state::CellStatus};
use serde::{Deserialize, Serialize};

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
    pub moves: Vec<crate::tuning::bundle::Move>,
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
    /// The renders of the tree the attempt moved from, and of the attempt
    /// itself: the two sides of the comparison the reviewer judged.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub before: Vec<Still>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub after: Vec<Still>,
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

/// Every still a trial was drawn at.
fn renders(trial: &Trial) -> Vec<Still> {
    trial
        .comparisons
        .iter()
        .flat_map(|c| c.images.iter().map(still))
        .collect()
}

fn number(v: f64) -> String {
    let s = format!("{v:.4}");
    s.trim_end_matches('0').trim_end_matches('.').to_string()
}

impl Run {
    /// Only candidates this run actually evaluated, named by the dial they
    /// moved. Dials never tried simply do not appear.
    pub fn attempts_for(&self, gap: &Gap) -> Vec<Attempt> {
        self.trials
            .iter()
            .enumerate()
            .filter(|(_, t)| {
                self.measured_here(&t.identity)
                    && (t.bundle.is_some() || self.dials.iter().any(|d| d.id == t.label))
            })
            .map(|(i, t)| (i, t, self.adopted(i)))
            .map(|(i, t, adopted)| Attempt {
                moves: t
                    .bundle
                    .as_ref()
                    .map(|b| b.moves.clone())
                    .or_else(|| t.step.clone().map(|m| vec![m]))
                    .unwrap_or_default(),
                adopted,
                stood: adopted.then_some(t.vetoed.is_none()),
                rolled_back: t.vetoed.as_ref().map(|v| v.reasons.join("; ")),
                before: t
                    .base
                    .as_deref()
                    .and_then(|key| self.trials.iter().find(|b| b.key == key))
                    .map(renders)
                    .unwrap_or_default(),
                after: renders(t),
                review: crate::tuning::look::words(t),
                dial: t.label.clone(),
                round: t.round,
                action_ledger: t.ledger.clone(),
                score_before_round: self
                    .trials
                    .iter()
                    .filter(|p| self.measured_here(&p.identity) && p.round < t.round && p.feasible)
                    .filter_map(|p| p.score)
                    .min_by(f64::total_cmp),
                score_after: t.score,
                feasible: t.feasible,
                reason: t.reason.clone(),
                visual_outcome: self
                    .visual
                    .as_ref()
                    .filter(|v| self.current == Some(i) && v.identity == t.key)
                    .map(|v| {
                        v.cells
                            .iter()
                            .filter(|(c, _)| gap.views.contains(&c.view))
                            .map(|(c, status)| CellOutcome {
                                item: c.item.clone(),
                                view: c.view.clone(),
                                seed: c.seed,
                                status: *status,
                            })
                            .collect::<Vec<_>>()
                    })
                    .filter(|cells: &Vec<CellOutcome>| !cells.is_empty()),
            })
            .collect()
    }

    /// Whether a round adopted this trial. A record from before `adopted` was
    /// kept still shows its adoptions: a veto, a passed-over round, the tree
    /// the run stands on.
    fn adopted(&self, index: usize) -> bool {
        let t = &self.trials[index];
        t.adopted || t.vetoed.is_some() || !t.adopted_over.is_empty() || self.current == Some(index)
    }
}
