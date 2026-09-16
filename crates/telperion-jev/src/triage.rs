//! QA triage: owning spec, nearest prior finding, severity. Writes no project state.

use std::path::Path;

use serde_json::{json, Map, Value};

use crate::caller::{evaluate, CallerError, EvaluateRequest, Transport};
use crate::questions::{
    same_defect_questions, severity_level, severity_questions, thresholds, triage_spec_questions,
};
use crate::screen::accumulate;

#[derive(Debug, Clone)]
pub struct TriageProposal {
    pub spec: String,
    pub spec_probabilities: Value,
    pub new_spec: bool,
    pub prior_finding: Option<String>,
    pub same_defect: Option<f64>,
    pub same_defect_match: Option<bool>,
    pub severity: Option<f64>,
    pub severity_level: Option<String>,
    pub ledger: Vec<String>,
    pub input_tokens: u64,
    pub output_tokens: u64,
    pub elapsed_ms: u64,
}

pub fn triage(
    transport: &dyn Transport,
    key: &str,
    ledger_dir: &Path,
    observation: &str,
    open_specs: &Map<String, Value>,
    prior_findings: &[String],
    standard: &str,
) -> Result<TriageProposal, CallerError> {
    let cuts = thresholds();
    let mut input_tokens = 0;
    let mut output_tokens = 0;
    let mut elapsed_ms = 0;
    let mut ledger = Vec::new();

    let spec_questions = triage_spec_questions(open_specs);
    let spec_state = json!({
        "owner_observation": observation,
        "open_specs": open_specs,
    });
    let spec_entry = evaluate(
        transport,
        key,
        EvaluateRequest {
            tool: "triage",
            source: None,
            state: &spec_state,
            questions: &spec_questions,
            ledger_dir,
        },
    )?;
    accumulate(
        &spec_entry,
        &mut input_tokens,
        &mut output_tokens,
        &mut elapsed_ms,
    );
    ledger.push(spec_entry.reference());

    let mut spec = spec_entry
        .choice("spec")
        .unwrap_or_else(|| "new_spec".into());
    let spec_probabilities = spec_entry
        .probabilities("spec")
        .cloned()
        .unwrap_or(Value::Null);
    let top_p = spec_probabilities
        .get(&spec)
        .and_then(Value::as_f64)
        .unwrap_or(0.0);
    let new_spec = spec == "new_spec" || top_p < cuts.route_min_probability;
    if new_spec {
        spec = "new_spec".into();
    }

    let mut best_finding = None;
    let mut best_p = None;
    for prior in prior_findings {
        let state = json!({
            "new_observation": observation,
            "prior_finding": prior,
        });
        let questions = same_defect_questions();
        let entry = evaluate(
            transport,
            key,
            EvaluateRequest {
                tool: "triage",
                source: None,
                state: &state,
                questions: &questions,
                ledger_dir,
            },
        )?;
        accumulate(
            &entry,
            &mut input_tokens,
            &mut output_tokens,
            &mut elapsed_ms,
        );
        ledger.push(entry.reference());
        let p = entry.noul("same_defect").unwrap_or(0.0);
        if best_p.map(|best| p > best).unwrap_or(true) {
            best_p = Some(p);
            best_finding = Some(prior.clone());
        }
    }

    let mut severity = None;
    let mut level = None;
    if !standard.is_empty() {
        let state = json!({
            "observation": observation,
            "standard": standard,
        });
        let questions = severity_questions();
        let entry = evaluate(
            transport,
            key,
            EvaluateRequest {
                tool: "triage",
                source: None,
                state: &state,
                questions: &questions,
                ledger_dir,
            },
        )?;
        accumulate(
            &entry,
            &mut input_tokens,
            &mut output_tokens,
            &mut elapsed_ms,
        );
        ledger.push(entry.reference());
        if let Some(score) = entry.score("severity") {
            severity = Some(score);
            level = Some(severity_level(score).to_string());
        }
    }

    Ok(TriageProposal {
        spec,
        spec_probabilities,
        new_spec,
        prior_finding: best_finding,
        same_defect: best_p,
        same_defect_match: best_p.map(|p| p >= cuts.duplicate_same_defect),
        severity,
        severity_level: level,
        ledger,
        input_tokens,
        output_tokens,
        elapsed_ms,
    })
}

pub fn format_proposal(proposal: &TriageProposal) -> String {
    let mut out = format!(
        "spec={spec} new_spec={new} distribution={dist}\n",
        spec = proposal.spec,
        new = proposal.new_spec,
        dist = proposal.spec_probabilities
    );
    if let (Some(finding), Some(p)) = (&proposal.prior_finding, proposal.same_defect) {
        let above = proposal.same_defect_match.unwrap_or(false);
        out.push_str(&format!(
            "nearest={finding}\nsame_defect={p:.2}\tabove_cut={above}\n"
        ));
    }
    if let (Some(score), Some(level)) = (proposal.severity, &proposal.severity_level) {
        out.push_str(&format!("severity={score:.2} ({level})\n"));
    }
    out.push_str(&format!("ledger={:?}\n", proposal.ledger));
    out.push_str(&format!(
        "tokens in={} out={} wall_ms={}\n",
        proposal.input_tokens, proposal.output_tokens, proposal.elapsed_ms
    ));
    out
}
