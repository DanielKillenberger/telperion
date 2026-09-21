//! One sheet, reserved and persisted before it is asked.
//!
//! The pass and the tokens are charged and saved before dispatch, so a refused
//! answer leaves an attempt to recover rather than a silent retry.
use super::{Plan, Verdict, INPUT_LABEL, PENDING};
use crate::tuning::engine::{Run, Services};

pub(in crate::tuning) fn review(
    state: &mut Run,
    services: &mut dyn Services,
    save: &mut dyn FnMut(&Run) -> Result<(), String>,
    plan: &Plan,
) -> Result<Verdict, String> {
    let allowance = services.sheet_tokens(&plan.request);
    let mut budget = state.budget.clone();
    budget.reserve_visual()?;
    budget.reserve(0, 0, allowance, 0)?;
    state.budget = budget;
    // The request only: the order and the current tree's label are code's, and
    // recorded with the verdict rather than with what was sent.
    state.push_judgment_input(INPUT_LABEL, serde_json::to_value(&plan.request).unwrap());
    state.pending = Some(PENDING.into());
    save(state)?;
    let answer = services.sheet(plan)?;
    let verdict = state.settle(answer, allowance)?;
    state.pending = None;
    save(state)?;
    Ok(verdict)
}
