//! The end result of a run: the outcome and the gap list, assembled from the
//! record by code so nothing the run learned stays buried in run.json. The
//! invoker, the owner or the add-species agent, reads it and answers the
//! check on each gap; this module decides nothing and dispatches nothing.
use super::{engine::Run, evaluation::Image, handoff::Attempt, state::CellStatus};
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub mod html;

pub const CHECK_PENDING: &str =
    "pending: the invoker answers reachable (dials not yet tried), covered (an open spec) or new";
pub const NEW_GAP_NOTE: &str = "a new gap escalates: its cause in generator terms and the shape of its spec are the host's, never this run's";
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
    pub latest_route: Option<String>,
    pub existing_spec: Option<String>,
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
        match (&self.pause, &self.pending) {
            (Some(p), _) => format!("paused: {}", p.reason),
            (None, Some(step)) => format!("interrupted during {step}"),
            (None, None) => "ended".into(),
        }
    }

    fn latest_route(&self, gap_id: &str) -> Option<String> {
        let tag = format!("{gap_id}=");
        self.routes
            .iter()
            .rev()
            .find_map(|r| r.strip_prefix(&tag).map(str::to_string))
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

    /// Every approved priority, whatever became of it. A priority that kept
    /// routing to tuning and stalled is a gap here, not a vanished stall.
    pub fn end_result(&self) -> EndResult {
        let handoffs = self.current_handoffs();
        let current = self.current_tree();
        let gaps = self
            .approved_priorities()
            .map(|a| a.ordered.clone())
            .unwrap_or_default()
            .iter()
            .enumerate()
            .map(|(i, gap)| {
                let handoff = handoffs
                    .iter()
                    .find(|h| h.gap_id.as_deref() == Some(&gap.id));
                let latest_route = handoff
                    .map(|h| h.route.clone())
                    .or_else(|| self.latest_route(&gap.id));
                let passes = self.owner_cell_passes(&gap.id);
                let status = match (handoff, passes, latest_route.as_deref()) {
                    (Some(_), _, _) => "handed off",
                    (None, Some(true), _) => "passing on the current tree",
                    (None, _, Some("tuning")) => "stalled in tuning",
                    (None, Some(false), _) => "open, not routed",
                    (None, None, _) => "not assessed",
                }
                .to_string();
                let attempts = judged_on(self.attempts_for(gap), &gap.id);
                GapEntry {
                    id: gap.id.clone(),
                    rank: i + 1,
                    priority: gap.observation.clone(),
                    status,
                    latest_route,
                    existing_spec: handoff.and_then(|h| h.existing_spec.clone()),
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

/// The same result as a page a person reads first.
pub fn markdown(r: &EndResult) -> String {
    let o = &r.outcome;
    let mut s = String::new();
    s.push_str(&format!(
        "# Run result: {} at seed {}\n\n",
        o.preset, o.seed
    ));
    s.push_str(&format!("{}\n\n", r.meaning));
    s.push_str("## Outcome\n\n| | |\n| --- | --- |\n");
    s.push_str(&format!("| Stopped | {} |\n", o.stopped));
    s.push_str(&format!("| Bootstrap | {} |\n", o.bootstrap));
    s.push_str(&format!("| Machine ready | {} |\n", o.machine_ready));
    s.push_str(&format!("| Owner acceptance | {} |\n", o.owner_acceptance));
    s.push_str(&format!(
        "| Adoptions kept / rolled back | {} / {} |\n",
        o.adoptions_kept, o.adoptions_rolled_back
    ));
    if let Some(b) = o.budget.as_object() {
        for key in ["rounds", "evaluations", "images", "visual_passes"] {
            let cap = b
                .get(&format!("max_{key}"))
                .map(|v| v.to_string())
                .unwrap_or_else(|| "?".into());
            let used = b
                .get(key)
                .map(|v| v.to_string())
                .unwrap_or_else(|| "?".into());
            s.push_str(&format!(
                "| {} | {} of {} |\n",
                key.replace('_', " "),
                used,
                cap
            ));
        }
    }
    match &o.current {
        Some(c) => {
            s.push_str(&format!(
                "\n## Current tree\n\nTrial `{}`, round {}, {}",
                c.key, c.round, c.label
            ));
            if let Some(score) = c.score_telemetry {
                s.push_str(&format!(", score telemetry {score:.4}"));
            }
            s.push_str(".\n\nStills:\n\n");
            for st in &c.stills {
                s.push_str(&format!(
                    "- {} seed {}: `{}` ({})\n",
                    st.view,
                    st.seed,
                    st.path,
                    &st.sha256[..12.min(st.sha256.len())]
                ));
            }
            s.push_str("\nOverlay:\n\n```json\n");
            s.push_str(&serde_json::to_string_pretty(&c.overrides).unwrap_or_default());
            s.push_str("\n```\n");
        }
        None => s.push_str("\n## Current tree\n\nNone: no candidate was ever current.\n"),
    }
    s.push_str(&format!(
        "\n## Gaps ({})\n\n{}\n",
        r.gaps.len(),
        r.gaps_note
    ));
    if r.gaps.is_empty() {
        s.push_str("\nNo approved priorities: nothing was asked of this run.\n");
    }
    for g in &r.gaps {
        s.push_str(&format!(
            "\n### {}. {} — {}\n\n{}\n\n",
            g.rank, g.id, g.status, g.priority
        ));
        s.push_str(&format!(
            "- Latest route: {}\n",
            g.latest_route.as_deref().unwrap_or("none")
        ));
        if let Some(spec) = &g.existing_spec {
            s.push_str(&format!("- Existing spec: {spec}\n"));
        }
        let feasible = g.attempts.iter().filter(|a| a.feasible).count();
        s.push_str(&format!(
            "- Attempts: {} evaluated, {} feasible\n",
            g.attempts.len(),
            feasible
        ));
        for a in g
            .attempts
            .iter()
            .rev()
            .take(6)
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
        {
            s.push_str(&format!(
                "  - round {} {}{}\n",
                a.round,
                a.dial,
                a.reason
                    .as_ref()
                    .map(|r| format!(": {r}"))
                    .unwrap_or_default()
            ));
        }
        if !g.reviewer_words.is_empty() {
            s.push_str("- Reviewer's words:\n");
            for w in &g.reviewer_words {
                s.push_str(&format!("  - {w}\n"));
            }
        }
        s.push_str(&format!("- Check: {}\n", g.check));
    }
    s
}
