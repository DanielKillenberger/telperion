//! The one uncalibrated question an adoption survives, and what it is shown.
use crate::tuning::{
    engine::{Run, Services},
    state::Visual,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

pub const VERSION: &str = "side-effects-v1";
pub const QUESTION: &str = "side_effects";
/// The label this question carries wherever it is recorded.
pub const UNCALIBRATED: &str = "uncalibrated side-effect question";

pub fn questions() -> Value {
    json!({QUESTION:{"type":"choice","version":VERSION,
        "instructions":"Does the new review report a defect that the previous review did not, other than the priorities this move targeted? This question is uncalibrated: no labelled set stands behind it.",
        "criteria":{
            "new_defect":"The new review reports a defect the previous review did not, other than the priorities this move targeted.",
            "no_new_defect":"The new review reports nothing the previous one did not, beyond what this move was aiming at.",
            "insufficient_evidence":"The evidence cannot support the comparison."}}})
}

/// One thresholded answer, with the raw choice kept as diagnostic. A choice
/// below the frozen threshold never vetoes and is still recorded.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Judged {
    pub choice: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub raw_choice: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f64>,
    pub threshold: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ledger: Option<String>,
}

impl Judged {
    pub fn vetoes(&self) -> bool {
        self.choice == "new_defect"
    }
}

/// What the previous look reported, what this one reports, and what moved
/// between them. No numbers and no verdict: the question is about text.
fn state_for(before: &Visual, after: &Visual, trial: &crate::tuning::evaluation::Trial) -> Value {
    let said = |v: &Visual| {
        json!({"defects":v.defects,
            "observations":v.findings.iter().map(|f| f.observation.clone())
                .chain(v.observations.iter().cloned()).collect::<Vec<_>>()})
    };
    json!({
        "question_is_uncalibrated":UNCALIBRATED,
        "previous_review":said(before),
        "new_review":said(after),
        "the_move":{"label":trial.label,
            "moves":trial.bundle.as_ref().map(|b| b.moves.iter()
                .map(|m| json!({"dial":m.dial,"direction":m.direction}))
                .collect::<Vec<_>>()),
            "dial":trial.label,"action":trial.action},
        "targeted":crate::tuning::progress::words(trial),
        "meaning":"A move that fixed what it aimed at and broke something else is not an improvement."})
}

pub(super) fn ask(
    state: &mut Run,
    services: &mut dyn Services,
    save: &mut dyn FnMut(&Run) -> Result<(), String>,
    before: &Visual,
    after: &Visual,
    trial: usize,
) -> Result<Judged, String> {
    let shown = state_for(before, after, &state.trials[trial]);
    let allowance = services.side_effect_tokens(&shown);
    state.push_judgment_input(UNCALIBRATED, shown.clone());
    state.reserve(0, 0, allowance, 0, UNCALIBRATED, save)?;
    let answer = services.side_effects(&shown)?;
    state.settle(answer, allowance)
}

/// What the router is told when the question took the adoption back.
pub(super) fn reason(judged: &Judged, after: &Visual) -> String {
    format!(
        "{UNCALIBRATED}: the closing review reports a defect the previous one did not ({:?} at {:?} against threshold {}); it says {}",
        judged.raw_choice, judged.confidence, judged.threshold,
        serde_json::to_string(&after.defects).unwrap_or_default()
    )
}

/// What the router is told when it did not.
pub(super) fn kept_note(judged: &Judged) -> String {
    format!(
        "adoption kept; {UNCALIBRATED} answered {:?} at {:?} against threshold {}",
        judged.raw_choice, judged.confidence, judged.threshold
    )
}
