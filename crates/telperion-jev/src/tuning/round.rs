//! The round boundary, decided in code. A first attempt against a candidate is
//! bounded by construction, so it needs no judgment; only a repeat after a
//! stall does, and then only to ask whether the evidence actually changed.
use super::{
    engine::{Proposal, Run, Services},
    evaluation::Trial,
};
use serde_json::{json, Value};

/// The evidence-difference question is new and has no labelled set behind it.
/// Every record it touches says so.
pub const EVIDENCE_VERSION: &str = "continuation-evidence-v1";
pub const EVIDENCE_QUESTION: &str = "evidence_difference";
pub const UNCALIBRATED: &str = "uncalibrated evidence-difference question";

#[derive(Debug, Clone, PartialEq)]
pub enum Decision {
    /// No round has been spent on this candidate yet.
    Proceed,
    /// A round stalled here and the evidence has moved; ask whether it moved
    /// materially enough to justify another attempt.
    AskEvidence,
    Pause(String),
}

pub fn questions() -> Value {
    json!({EVIDENCE_QUESTION:{"type":"choice","version":EVIDENCE_VERSION,
        "instructions":"Does the new evidence differ materially from what the last failed attempt acted on? This question is uncalibrated: no labelled set stands behind it.",
        "criteria":{
            "different":"The new evidence differs materially from what the last failed attempt acted on.",
            "same":"The new evidence does not differ materially from what the last failed attempt acted on.",
            "insufficient_evidence":"The evidence cannot support the comparison."}}})
}

impl Run {
    fn current_candidate_key(&self) -> Option<String> {
        self.current
            .and_then(|i| self.trials.get(i))
            .map(|t| t.key.clone())
    }

    /// Rounds already spent against the candidate that is current now.
    fn rounds_here(&self) -> Vec<&Trial> {
        let Some(key) = self.current_candidate_key() else {
            return vec![];
        };
        self.trials
            .iter()
            .filter(|t| {
                self.measured_here(&t.identity) && t.round > 0 && t.base.as_deref() == Some(&key)
            })
            .collect()
    }

    /// The reservations `continuation::assess` used to guard before the engine
    /// stopped asking it every round.
    fn exhausted(&self) -> Option<String> {
        let b = &self.budget;
        (b.evaluations >= b.max_evaluations || b.images >= b.max_images || b.rounds >= b.max_rounds)
            .then(|| "hard budget exhausted".to_string())
    }

    /// Code decides the round boundary. The hard budget and preflight checks in
    /// the engine run before this and are unchanged.
    pub fn round_decision(&self) -> Decision {
        if let Some(reason) = self.exhausted() {
            return Decision::Pause(reason);
        }
        let spent = self.rounds_here();
        if spent.is_empty() {
            // A first attempt is bounded by construction: authored dials only,
            // at most `max_candidates` of them, candidate overlays only.
            return Decision::Proceed;
        }
        let acted_on = spent
            .iter()
            .filter_map(|t| t.evidence.clone())
            .collect::<Vec<_>>();
        let now = self.visual.as_ref().map(|v| v.ledger.clone());
        match now {
            // New evidence: ask whether it differs enough to justify a repeat.
            Some(ledger) if !acted_on.contains(&ledger) => Decision::AskEvidence,
            // Same evidence: an untried move is still worth trying, and it
            // costs no judgment to find out. The proposal call and the repeat
            // filter decide; when nothing untried is left, the round stops
            // there instead of here.
            _ => Decision::Proceed,
        }
    }

    /// Applies the round decision. A first attempt proceeds silently; a repeat
    /// on moved evidence buys exactly one uncalibrated question.
    pub(super) fn settle_round_boundary(
        &mut self,
        services: &mut dyn Services,
        save: &mut dyn FnMut(&Self) -> Result<(), String>,
    ) -> Result<(), String> {
        match self.round_decision() {
            Decision::Proceed => Ok(()),
            Decision::Pause(reason) => Err(reason),
            Decision::AskEvidence => {
                let state = self.evidence_state();
                let allowance = services.evidence_tokens(&state);
                self.push_judgment_input(UNCALIBRATED, state.clone());
                self.reserve(0, 0, allowance, 0, UNCALIBRATED, save)?;
                let answer = services.evidence(&state)?;
                let differs = self.settle(answer, allowance)?;
                if differs != "different" {
                    return Err(format!(
                        "repeat attempt unjustified ({differs}); {UNCALIBRATED}"
                    ));
                }
                Ok(())
            }
        }
    }

    /// The moves already made from the current candidate, for whoever is
    /// about to propose the next one.
    pub fn attempts_here(&self) -> Vec<Value> {
        let before = self
            .current
            .and_then(|i| self.trials.get(i))
            .and_then(|t| t.score);
        self.rounds_here()
            .iter()
            .map(|t| {
                json!({"dial":t.label,"action":t.action,"feasible":t.feasible,
                    "score_before":before,"score_after":t.score,
                    "review":super::progress::words(t)})
            })
            .collect()
    }

    /// What the evidence question is shown: the attempts that failed here, and
    /// the assessment then against the assessment now.
    pub fn evidence_state(&self) -> Value {
        let attempts = self
            .rounds_here()
            .iter()
            .map(|t| {
                json!({"dial":t.label,"action":t.action,"feasible":t.feasible,
                    "reason":t.reason,"score_after":t.score,
                    "score_before":self.current.and_then(|i| self.trials.get(i)).and_then(|c| c.score),
                    "acted_on_assessment":t.evidence})
            })
            .collect::<Vec<_>>();
        json!({
            "question_is_uncalibrated":UNCALIBRATED,
            "previous_failed_attempts":attempts,
            "assessment_now":self.visual.as_ref().map(|v| json!({
                "ledger":v.ledger,"identity":v.identity,
                "defects":v.defects,"findings":v.findings})),
            "meaning":"A repeat of an attempt that already failed on the same evidence is not justified."})
    }

    /// Drops a proposal whose exact move already failed against this candidate.
    /// An untried move is never dropped, and nothing here implies a dial was
    /// exhausted.
    /// Returns the moves worth trying and how many were refused as repeats.
    pub fn filter_repeats(&mut self, proposals: Vec<Proposal>) -> (Vec<Proposal>, usize) {
        let base = self.current.and_then(|i| self.trials.get(i)).cloned();
        let spent = self
            .rounds_here()
            .iter()
            .map(|t| {
                (
                    t.label.clone(),
                    t.action,
                    t.feasible,
                    t.score,
                    t.progress.is_some(),
                )
            })
            .collect::<Vec<_>>();
        let mut kept = vec![];
        let mut refused = vec![];
        for proposal in proposals {
            let repeat = spent
                .iter()
                .any(|(dial, action, feasible, score, reviewed)| {
                    dial == &proposal.dial
                    && action.as_ref() == Some(&proposal.action)
                    // A reviewed attempt still standing here was not adopted,
                    // whatever its numbers did.
                    && (!feasible
                        || *reviewed
                        || !score
                            .zip(base.as_ref().and_then(|b| b.score))
                            .is_some_and(|(s, b)| s < b))
                });
            if repeat {
                refused.push(format!(
                    "repeat refused: {} {}",
                    proposal.dial,
                    serde_json::to_value(proposal.action)
                        .ok()
                        .and_then(|v| v.as_str().map(str::to_string))
                        .unwrap_or_default()
                ));
            } else {
                kept.push(proposal);
            }
        }
        let count = refused.len();
        self.routes.extend(refused);
        (kept, count)
    }
}
