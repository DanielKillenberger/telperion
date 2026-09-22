//! The conductor's questions to Jev, each over bounded explicit evidence and
//! each with a no-match answer. Code reads the answers into signals; nothing
//! here decides a route. Design complexity and remaining implementation
//! complexity are distinct questions: the second consumes the completed
//! design and never inherits the first's difficulty (R3).
use serde_json::{json, Value};

use super::state::{Budget, Run};
use super::{Config, Result};
use crate::ledger::LedgerEntry;
use crate::pipeline::judge::{Judge, Judgment};
use crate::tuning::continuation::{self, Assessment, Basis};

pub const VERSION: u32 = 1;
pub const NONE: &str = "none";

/// Is the work a gap spec asks for routine, or does it need a design first?
pub fn design_questions() -> Value {
    json!({
        "design_complexity": {"type": "choice",
            "instructions": "From the spec's text, the gap evidence and the capability definitions alone: is the change routine (a value table, a preset row, a bounded mechanical edit a cheap agent completes from the spec as written), or complex (new generator behaviour, shared code paths, interfaces or invariants nobody has written down yet, so a design has to exist before anyone implements)? Judge the evidence, not the confidence of the prose.",
            "criteria": {
                "routine": "The spec states what to change and how to verify it; no design decision is open.",
                "complex": "Interfaces, invariants, difficult cases or verification are undecided; a design is needed first.",
                "insufficient_evidence": "The evidence does not show what the change touches, or it contradicts itself."}}
    })
}

/// With the design in hand, how much is left for the implementer?
pub fn implementation_questions() -> Value {
    json!({
        "implementation_complexity": {"type": "choice",
            "instructions": "From the completed design handoff and the code evidence it cites: is the remaining implementation straightforward (the design resolved interfaces, invariants, difficult cases and verification, and the code to write follows from it), still complex (the design is sound but the implementation touches shared behaviour or needs judgment the design did not remove), or does it need more design (the handoff names unknowns that decide the implementation)? The initial gap's difficulty is not evidence here.",
            "criteria": {
                "straightforward": "Every decision is made in the design; a cheap model can implement it from the handoff.",
                "complex": "The design holds, but the implementation still needs a strong model.",
                "needs_design": "The handoff leaves unknowns that must be designed before implementing.",
                "insufficient_evidence": "The handoff is missing, incomplete or contradicts the spec."}}
    })
}

/// Which untried dial, if any, could reach the trait the reviewer named?
pub fn reach_questions(dials: &[Value]) -> Value {
    let mut criteria = serde_json::Map::new();
    for dial in dials {
        let id = dial["id"].as_str().unwrap_or_default().to_string();
        criteria.insert(id, json!(dial["meaning"].as_str().unwrap_or_default()));
    }
    criteria.insert(NONE.into(), json!("No untried dial plausibly moves this trait; the dials tried already cover what the table can express."));
    json!({
        "reachable_with": {"type": "choice",
            "instructions": "The reviewer's words name a trait the tuning run did not reach. The options are the authored dials the run has not yet tried on it, each with the meaning its author wrote. Choose the one dial whose meaning plausibly moves that trait, or none.",
            "criteria": criteria}
    })
}

/// Which open spec, if any, already covers the gap?
pub fn cover_questions(specs: &[Value]) -> Value {
    let mut criteria = serde_json::Map::new();
    for spec in specs {
        let id = spec["id"].as_str().unwrap_or_default().to_string();
        criteria.insert(id, json!(spec["title"].as_str().unwrap_or_default()));
    }
    criteria.insert(
        NONE.into(),
        json!("No open spec covers this gap; it is new."),
    );
    json!({
        "covered_by": {"type": "choice",
            "instructions": "The gap is a trait the reviewer says the tree lacks. The options are the open generator specs by id and title. Choose the spec whose work would close this gap, or none.",
            "criteria": criteria}
    })
}

/// The conductor's caller: one Jev request through the shared caller, its
/// usage counted against the run's budget.
pub struct Asker<'a> {
    pub judge: Judge<'a>,
    pub config: &'a Config,
}

impl Asker<'_> {
    pub fn ask(
        &self,
        run: &mut Run,
        tool: &str,
        state: &Value,
        questions: &Value,
    ) -> Result<Judgment> {
        if self.judge.key.is_empty() {
            return Err(crate::caller::KeyError::Missing.to_string().into());
        }
        let judgment = self.judge.ask(tool, None, state, questions)?;
        run.budget.jev_calls += 1;
        match &judgment.entry.usage {
            Some(usage) => {
                run.budget.tokens = run
                    .budget
                    .tokens
                    .saturating_add(usage.input_tokens.saturating_add(usage.output_tokens));
            }
            None => run.budget.usage_known = false,
        }
        Ok(judgment)
    }
}

/// The answer to a choice question, or none where Jev chose the no-match.
pub fn chosen(entry: &LedgerEntry, question: &str) -> Option<String> {
    entry
        .choice(question)
        .filter(|choice| choice != NONE && !choice.is_empty())
}

/// The three continuation answers, read into the shared contract's shape.
pub fn assessment(entry: &LedgerEntry, identity: &str) -> Assessment {
    let read = |q: &str| {
        entry
            .choice(q)
            .unwrap_or_else(|| "insufficient_evidence".into())
    };
    Assessment {
        identity: identity.into(),
        ledger: entry.identity.clone(),
        tractability: read("tractability"),
        progress: read("progress"),
        risk: read("risk"),
    }
}

/// The conductor's hard limits in the shared contract's budget shape: token
/// allowance and dispatch count; the tuning loop's own image and evaluation
/// caps live in its run and are not restated here.
pub fn shared_budget(budget: &Budget, run: &Run) -> crate::tuning::state::Budget {
    crate::tuning::state::Budget {
        evaluations: 0,
        images: 0,
        tokens: budget.tokens,
        rounds: run.dispatches.len() as u64,
        max_evaluations: u64::MAX,
        max_images: u64::MAX,
        max_tokens: budget.max_tokens,
        max_rounds: budget.max_dispatches,
        visual_passes: None,
        max_visual_passes: None,
    }
}

/// The continuation state Jev sees: the basis word for word, so the ledger
/// entry names the same attempt the code combined.
pub fn continuation_state(basis: &Basis, budget: &Budget, risks: &[String]) -> Value {
    json!({
        "identity": basis.identity,
        "proposed_action": basis.proposed_action,
        "evidence": basis.evidence,
        "recent_outcomes": basis.recent_outcomes,
        "next_tokens": basis.next_tokens,
        "estimate_basis": basis.estimate_basis,
        "usage_known": basis.usage_known,
        "tokens_spent": budget.tokens,
        "tokens_remaining": budget.remaining(),
        "attempt_bound": budget.attempt_max_tokens,
        "risks": risks,
    })
}

/// Asks the continuation trio and combines them under the shared contract.
/// `Ok(assessment)` justifies the attempt; `Err(reason)` is the pause reason.
pub fn continuation(
    asker: &Asker<'_>,
    run: &mut Run,
    basis: &Basis,
    risks: &[String],
) -> Result<std::result::Result<Assessment, (String, Option<Assessment>)>> {
    let shared = shared_budget(&run.budget, run);
    // The hard limits are checked before any call is spent on them.
    if let Err(reason) =
        continuation::assess(basis, &shared, None, asker.config.continuation_validated)
    {
        if reason != "missing continuation assessment" {
            return Ok(Err((reason, None)));
        }
    }
    let state = continuation_state(basis, &run.budget, risks);
    let judgment = asker.ask(
        run,
        "conductor-continuation",
        &state,
        &continuation::questions(),
    )?;
    let assessment = assessment(&judgment.entry, &basis.identity);
    Ok(
        match continuation::assess(
            basis,
            &shared,
            Some(&assessment),
            asker.config.continuation_validated,
        ) {
            Ok(()) => Ok(assessment),
            Err(reason) => Err((reason, Some(assessment))),
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_question_offers_its_no_match_answer() {
        let d = design_questions();
        assert!(d["design_complexity"]["criteria"]["insufficient_evidence"].is_string());
        let i = implementation_questions();
        assert!(i["implementation_complexity"]["criteria"]["needs_design"].is_string());
        let r = reach_questions(&[json!({"id": "crown_width", "meaning": "how wide"})]);
        assert!(r["reachable_with"]["criteria"]["crown_width"].is_string());
        assert!(r["reachable_with"]["criteria"][NONE].is_string());
        let c = cover_questions(&[json!({"id": "fn-103", "title": "Leaders keep their girth"})]);
        assert!(c["covered_by"]["criteria"]["fn-103"].is_string());
        assert!(c["covered_by"]["criteria"][NONE].is_string());
    }
}
