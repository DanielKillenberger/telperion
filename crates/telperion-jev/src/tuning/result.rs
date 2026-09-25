//! The end result of a revision: the outcome and the gap list, assembled from
//! the record by code so nothing the run learned stays buried in run.json.
//! The runner's Gaps stage classes each gap; this module decides nothing and
//! dispatches nothing. It is the only result a revision writes.
use super::{engine::Run, evaluation::Image, state::CellStatus};
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub mod attempt;
pub use attempt::{Attempt, CellOutcome};

pub const CHECK_PENDING: &str =
    "pending: the runner's Gaps stage classes it reachable, identity or global";
pub const NEW_GAP_NOTE: &str =
    "a gap the runner classes identity or global is the host's to spec, never this run's";
/// The status of an objective the current tree passes: no gap.
pub const PASSING: &str = "passing on the current tree";
pub const MEANING: &str =
    "outcome plus gap list; a listed gap is never readiness and this file mints nothing";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Still {
    pub view: String,
    pub seed: u32,
    pub sha256: String,
    pub path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct CurrentTree {
    pub key: String,
    pub round: u64,
    pub label: String,
    pub score_telemetry: Option<f64>,
    pub overrides: Value,
    pub stills: Vec<Still>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Outcome {
    pub run_identity: String,
    pub preset: String,
    pub seed: u32,
    pub bootstrap: bool,
    pub machine_ready: bool,
    pub reviewer_passed_unqualified: bool,
    pub owner_acceptance: String,
    pub stopped: String,
    pub adoptions_kept: usize,
    pub adoptions_rolled_back: usize,
    pub budget: Value,
    pub current: Option<CurrentTree>,
    pub finalists: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct GapEntry {
    pub id: String,
    pub rank: usize,
    pub priority: String,
    pub status: String,
    pub attempts: Vec<Attempt>,
    pub reviewer_words: Vec<String>,
    pub stills: Vec<Still>,
    pub check: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct EndResult {
    pub meaning: String,
    pub outcome: Outcome,
    pub gaps: Vec<GapEntry>,
    pub gaps_note: String,
    /// The traits the generator cannot draw yet, each with the spec that
    /// captures it: left out of readiness and never a gap of this run.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub known_gaps: Vec<super::unexpressed::Unexpressed>,
}

fn still(image: &Image) -> Still {
    Still {
        view: image.view.clone(),
        seed: image.seed,
        sha256: image.sha256.clone(),
        path: image.path.display().to_string(),
    }
}

impl Run {
    fn current_tree(&self) -> Option<CurrentTree> {
        let trial = self.trials.get(self.current?)?;
        Some(CurrentTree {
            key: trial.key.clone(),
            round: trial.round,
            label: trial.label.clone(),
            score_telemetry: trial.score,
            overrides: trial.overrides.clone(),
            stills: trial
                .comparisons
                .iter()
                .flat_map(|c| c.images.iter().map(still))
                .collect(),
        })
    }

    fn stopped(&self) -> String {
        match (&self.stopped, &self.pending) {
            (Some(reason), _) => format!("stopped: {reason}"),
            (None, Some(step)) => format!("interrupted during {step}"),
            (None, None) => "ended".into(),
        }
    }

    fn owner_cell_passes(&self, gap_id: &str) -> Option<bool> {
        let visual = self.visual.as_ref()?;
        let tag = format!("owner-priority:{gap_id}: ");
        let cells: Vec<_> = visual
            .cells
            .iter()
            .filter(|(c, _)| c.item.starts_with(&tag))
            .collect();
        (!cells.is_empty()).then(|| cells.iter().all(|(_, s)| *s == CellStatus::Pass))
    }

    /// Every tuning objective, whatever became of it. One the revision could
    /// not bring to pass is a gap here, not a vanished stall.
    pub fn end_result(&self) -> EndResult {
        let current = self.current_tree();
        let gaps = self
            .approved_priorities()
            .map(|a| a.ordered.clone())
            .unwrap_or_default()
            .iter()
            .enumerate()
            .map(|(i, gap)| {
                let status = match self.owner_cell_passes(&gap.id) {
                    Some(true) => PASSING,
                    Some(false) => "failing on the current tree",
                    None => "not assessed",
                }
                .to_string();
                let attempts = judged_on(self.attempts_for(gap), &gap.id);
                GapEntry {
                    id: gap.id.clone(),
                    rank: i + 1,
                    priority: gap.observation.clone(),
                    status,
                    reviewer_words: reviewer_words(&attempts),
                    attempts,
                    stills: current
                        .as_ref()
                        .map(|c| c.stills.clone())
                        .unwrap_or_default(),
                    check: CHECK_PENDING.into(),
                }
            })
            .collect();
        EndResult {
            meaning: MEANING.into(),
            outcome: Outcome {
                run_identity: self.identity.clone(),
                preset: self.preset.clone(),
                seed: self.seed,
                bootstrap: self.visual_bootstrap,
                machine_ready: self.machine_ready,
                reviewer_passed_unqualified: self.reviewer_passed_unqualified,
                owner_acceptance: "pending".into(),
                stopped: self.stopped(),
                adoptions_kept: self
                    .routes
                    .iter()
                    .filter(|r| r.starts_with("adoption kept"))
                    .count(),
                adoptions_rolled_back: self
                    .routes
                    .iter()
                    .filter(|r| r.starts_with("adoption rolled back"))
                    .count(),
                budget: serde_json::to_value(&self.budget).unwrap_or(Value::Null),
                current,
                finalists: self.finalists().iter().map(|t| t.key.clone()).collect(),
            },
            gaps,
            gaps_note: NEW_GAP_NOTE.into(),
            known_gaps: Vec::new(),
        }
    }
}

/// The attempts on which the reviewer saw this priority move, better or
/// worse, or every attempt when no review named priorities at all. A sheet
/// grades every priority on every render, so "none" is not relevance.
fn judged_on(attempts: Vec<Attempt>, gap_id: &str) -> Vec<Attempt> {
    let named = attempts.iter().any(|a| {
        a.review
            .as_ref()
            .is_some_and(|r| r["per_priority"].is_object())
    });
    if !named {
        return attempts;
    }
    attempts
        .into_iter()
        .filter(|a| {
            a.review.as_ref().is_some_and(|r| {
                r["per_priority"]
                    .get(gap_id)
                    .and_then(Value::as_str)
                    .is_some_and(|g| !NO_MOVE.contains(&g.to_ascii_lowercase().as_str()))
            })
        })
        .collect()
}

pub const WORDS_KEPT: usize = 12;
/// The sheet grades "none"; the side-by-side review says "same" or "unknown".
const NO_MOVE: [&str; 3] = ["none", "same", "unknown"];

/// What the reviewer said was wrong or missing on these attempts, the most
/// recent first, once each, at most `WORDS_KEPT`.
fn reviewer_words(attempts: &[Attempt]) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    for review in attempts.iter().rev().filter_map(|a| a.review.as_ref()) {
        for key in ["looks_wrong", "regressions", "breaks", "missing"] {
            let words: Vec<String> = match &review[key] {
                Value::String(s) => vec![s.clone()],
                Value::Array(items) => items
                    .iter()
                    .map(|v| {
                        v.as_str()
                            .map(str::to_string)
                            .unwrap_or_else(|| v.to_string())
                    })
                    .collect(),
                _ => vec![],
            };
            for w in words.into_iter().filter(|w| !w.trim().is_empty()) {
                if !out.contains(&w) && out.len() < WORDS_KEPT {
                    out.push(w);
                }
            }
        }
    }
    out
}
