//! The reviewer's side-by-side progress verdict: which of two renders of the
//! same tree better answers each approved priority, against the references.
//!
//! It is a new, uncalibrated question. No replay qualifies it, so it runs only
//! under the scoped experimental authority, it is labelled uncalibrated
//! wherever it is recorded, and it spends a visual pass like any other look.
//! The reviewer is never told which render is the candidate, which is newer,
//! or what any number says: code assigns the two sides from the trial keys,
//! records the assignment, and maps the answer back.
mod adapter;
mod request;
mod verdict;
pub(in crate::tuning) use adapter::shell;
pub use adapter::{dispatch, envelope};
pub use request::{candidate_side, inert, prompt_request, request, Look};
pub(in crate::tuning) use request::{redacted, shared_views, still};
pub use verdict::{bind, Answer, Choice, Judgment, Movement, Note, On, Regression, Verdict};

use super::{
    engine::{Run, Services},
    evaluation::{Image, Trial},
    priority::Gap,
    vision,
};
use crate::sha256_hex;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::path::PathBuf;

pub const VERSION: &str = "tuning-progress-v2";
/// The label this question carries wherever it is recorded.
pub const UNCALIBRATED: &str =
    "uncalibrated progress review: a comparative verdict, never a score or a readiness claim";
pub const PENDING: &str = "progress review";
/// What a candidate or variant records when the paid review it was sent to
/// failed or would not bind. The pass and the tokens stay charged, the attempt
/// is never adopted, and the same move is not bought a second time.
pub const REVIEW_FAILED: &str = "review failed; attempt charged";
pub const PROMPT: &str = "You are shown reference photographs of a tree species, then two renders, A and B, of the same generated tree at the same view and seed. One or the other may be the newer attempt; nothing here says which, and neither is a photograph.\n\nFor each listed priority, say which render better satisfies it relative to the references: a_better, b_better, same when neither is closer, or unknown when this view cannot show it. Judge only what the images show.\n\nThen say in one or two sentences what differs for the better between them, what is still missing in both against the references, and list anything one render breaks that the other does not; for each, say which render has the problem.\n\nGive no numbers, no scores and no overall winner.";

/// How a candidate is chosen against the tree it came from.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Selection {
    /// The five still-side numbers decide, and the reviewer never sees a
    /// rejected candidate. Every run before 2026-09-21 ran this way.
    #[default]
    Score,
    /// The reviewer's comparative verdict decides; the numbers are telemetry.
    Visual,
    /// One bundle of every dial Jev supported, rendered at several strengths
    /// and judged on one contact sheet.
    Bundle,
}

impl Selection {
    pub fn is_score(&self) -> bool {
        *self == Selection::Score
    }
}

/// The adapter this question is asked through, and the protocol it was written
/// against. The protocol's bytes join the run's identity.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Adapter {
    pub adapter: vision::Adapter,
    pub protocol: PathBuf,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Priority {
    pub id: String,
    pub observation: String,
}

/// What the reviewer is sent. It names no candidate and no round.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Request {
    pub schema: String,
    pub target_species: String,
    pub view: String,
    pub seed: u32,
    pub references: Vec<Image>,
    pub a: Image,
    pub b: Image,
    pub priorities: Vec<Priority>,
    pub owner_notes: String,
}

impl Request {
    pub fn hash(&self) -> String {
        sha256_hex(&serde_json::to_vec(self).unwrap())
    }
    pub fn prompt_hash() -> String {
        sha256_hex(PROMPT.as_bytes())
    }
    pub fn verify(&self) -> Result<(), String> {
        if self.schema != VERSION
            || self.priorities.is_empty()
            || self.priorities.len() > 8
            || self.references.is_empty()
            || self.a.sha256 == self.b.sha256
            || self.a.view != self.view
            || self.b.view != self.view
        {
            return Err("invalid progress request".into());
        }
        for image in self.references.iter().chain([&self.a, &self.b]) {
            image.verify()?;
        }
        Ok(())
    }
}

/// The approved priorities the router last sent to tuning. A priority routed
/// elsewhere is fn-89's, and an unrouted one has no verdict to give yet.
pub fn tuning_priorities(state: &Run) -> Vec<Gap> {
    let Some(approval) = state.approved_priorities() else {
        return vec![];
    };
    approval
        .ordered
        .iter()
        .filter(|gap| {
            state
                .routes
                .iter()
                .rev()
                .find_map(|r| r.strip_prefix(&format!("{}=", gap.id)))
                .is_some_and(|route| route == "tuning")
        })
        .cloned()
        .collect()
}

/// Which reviewed candidate the round adopts: the one judged better on most
/// priorities, among those judged better somewhere, worse nowhere and breaking
/// nothing. Ties keep the order the proposals arrived in, which is their
/// direction-mass order.
pub fn adopt(reviewed: &[(usize, Value)], trials: &[Trial]) -> Option<usize> {
    reviewed
        .iter()
        .filter_map(|(index, _)| {
            let verdict = trials.get(*index)?.progress.as_ref()?;
            (trials[*index].feasible && verdict.adoptable()).then(|| (*index, verdict.better()))
        })
        .reduce(|best, next| if next.1 > best.1 { next } else { best })
        .map(|(index, _)| index)
}

/// Which candidate the round adopts and what it makes effective. Score mode
/// is the comparison the engine already made; visual mode asks the verdicts.
pub(super) fn chosen(
    state: &mut Run,
    selection: Selection,
    score: (usize, usize, Value),
    reviewed: Vec<(usize, Value)>,
) -> Option<(usize, Value)> {
    let (old, best, best_effective) = score;
    if selection.is_score() {
        return (best != old).then_some((best, best_effective));
    }
    let index = adopt(&reviewed, &state.trials)?;
    // Owner-facing only: what else this round could have kept.
    let others = reviewed
        .iter()
        .filter(|(i, _)| {
            *i != index
                && state.trials[*i]
                    .progress
                    .as_ref()
                    .is_some_and(Verdict::adoptable)
        })
        .map(|(i, _)| state.trials[*i].key.clone())
        .collect::<Vec<_>>();
    state.trials[index].adopted_over = others;
    reviewed.into_iter().find(|(i, _)| *i == index)
}

/// Why a round adopted nothing, in the words of the rule that decided.
pub(super) fn stall(selection: Selection) -> String {
    match selection {
        Selection::Score => {
            "numeric stall; reassess remaining defect and recent failed attempts".into()
        }
        Selection::Visual => "visual stall; no candidate judged better".into(),
        Selection::Bundle => "bundle stall; no strength judged better".into(),
    }
}

/// The reviewer's words about one attempt, for whoever is asked next. A
/// bundle attempt answers with what it moved as well as how it was judged.
pub fn words(trial: &Trial) -> Option<Value> {
    let mut out = trial
        .progress
        .as_ref()
        .map(|p| {
            json!({"per_priority":p.per_priority,"improved":p.improved,"missing":p.missing,
            "regressions":p.regressions,"inert":p.inert,"note":p.note,
            "uncalibrated":UNCALIBRATED})
        })
        .or_else(|| super::bundle::words(trial))?;
    // What the move broke, for whoever proposes the next one.
    if let Some(veto) = &trial.vetoed {
        out["rolled_back"] = json!({"reasons":veto.reasons,"ledger":veto.ledger,
            "meaning":"this move was adopted and the all-view review that followed took it back"});
    }
    Some(out)
}

/// What a round would send, for pricing before any candidate exists.
pub fn skeleton(state: &Run, references: &[Image]) -> Value {
    json!({"schema":VERSION,"prompt_sha256":Request::prompt_hash(),"seed":state.seed,
        "references":references,"owner_notes":state.owner_notes,
        "priorities":tuning_priorities(state).iter().map(|g| Priority{id:g.id.clone(),
            observation:g.observation.clone()}).collect::<Vec<_>>()})
}

/// One reviewed candidate: the pass and the tokens are reserved and saved
/// before dispatch, so a failed review leaves an attempt to recover rather
/// than a silent retry.
pub(super) fn review(
    state: &mut Run,
    services: &mut dyn Services,
    save: &mut dyn FnMut(&Run) -> Result<(), String>,
    current: usize,
    candidate: usize,
) -> Result<(), String> {
    let priorities = tuning_priorities(state);
    // A request that cannot be built is this candidate's problem, not the
    // round's: the attempt is recorded and the next candidate is evaluated.
    let look = match services.progress_request(state, current, candidate, &priorities) {
        Ok(look) => look,
        Err(reason) => {
            let label = state.trials[candidate].label.clone();
            state.trials[candidate].reason = Some(format!("progress review not asked: {reason}"));
            state
                .routes
                .push(format!("progress review not asked for {label}: {reason}"));
            return save(state);
        }
    };
    let (request, side) = match look {
        Look::Inert => {
            let trial = &mut state.trials[candidate];
            trial.progress = Some(inert());
            let note = format!(
                "inert: {} {}",
                trial.label,
                trial
                    .action
                    .and_then(|a| serde_json::to_value(a).ok())
                    .and_then(|v| v.as_str().map(str::to_owned))
                    .unwrap_or_default()
            );
            state.routes.push(note);
            return save(state);
        }
        Look::Ask(request, side, from_priority) => {
            if !from_priority {
                state.routes.push(format!(
                    "progress review on {}: no tuning-priority view differs",
                    request.view
                ));
            }
            (request, side)
        }
    };
    let allowance = services.progress_tokens(&request);
    let mut budget = state.budget.clone();
    budget.reserve_visual()?;
    budget.reserve(0, 0, allowance, 0)?;
    state.budget = budget;
    state.push_judgment_input(UNCALIBRATED, serde_json::to_value(&request).unwrap());
    state.pending = Some(PENDING.into());
    save(state)?;
    let asked = services.progress(&request, &side);
    let verdict = match asked.and_then(|answer| state.settle(answer, allowance)) {
        Ok(verdict) => verdict,
        // The attempt is charged either way, so it is recorded either way.
        Err(reason) => {
            state.trials[candidate].reason = Some(REVIEW_FAILED.into());
            save(state)?;
            return Err(reason);
        }
    };
    state.trials[candidate].progress = Some(verdict);
    state.pending = None;
    save(state)
}
