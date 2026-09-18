//! Pre-parsed selection: code extracts spans, Jev chooses one, code copies it.

use std::path::Path;

use serde_json::json;

use crate::caller::{evaluate, CallerError, EvaluateRequest, Transport};
use crate::extract::candidate_spans;
use crate::ledger::SourceRef;
use crate::questions::{selection_questions, thresholds};
use crate::screen::accumulate;

#[derive(Debug, Clone)]
pub struct SelectReport {
    pub chosen: String,
    pub contract_failure: bool,
    pub confidence: f64,
    pub spread: bool,
    pub candidates: Vec<String>,
    pub sentence: String,
    pub ledger: String,
    pub identity: String,
    pub probabilities: serde_json::Value,
    pub input_tokens: u64,
    pub output_tokens: u64,
    pub elapsed_ms: u64,
}

pub fn select(
    transport: &dyn Transport,
    key: &str,
    ledger_dir: &Path,
    document: &str,
    question: &str,
    source: Option<&SourceRef>,
) -> Result<SelectReport, CallerError> {
    let spans = candidate_spans(document);
    let questions = selection_questions(question, &spans);
    let state = json!({
        "document": document,
        "question": question,
        "candidates": spans,
    });
    let entry = evaluate(
        transport,
        key,
        EvaluateRequest {
            tool: "select",
            source,
            state: &state,
            questions: &questions,
            ledger_dir,
        },
    )?;
    let raw = entry.choice("span").unwrap_or_else(|| "none".into());
    let allowed = raw == "none" || spans.iter().any(|span| span == &raw);
    let contract_failure = !allowed;
    let chosen = if allowed { raw } else { String::new() };
    let confidence = entry.confidence("span").unwrap_or(0.0);
    let spread = chosen == "none" && confidence < thresholds().selection_spread_confidence;
    let mut input_tokens = 0;
    let mut output_tokens = 0;
    let mut elapsed_ms = 0;
    accumulate(
        &entry,
        &mut input_tokens,
        &mut output_tokens,
        &mut elapsed_ms,
    );
    Ok(SelectReport {
        chosen,
        contract_failure,
        confidence,
        spread,
        candidates: spans,
        sentence: document.chars().take(240).collect(),
        ledger: entry.reference(),
        identity: entry.identity.clone(),
        probabilities: entry
            .probabilities("span")
            .cloned()
            .unwrap_or(serde_json::Value::Null),
        input_tokens,
        output_tokens,
        elapsed_ms,
    })
}

pub fn format_report(report: &SelectReport) -> String {
    let chosen = if report.contract_failure {
        "<rejected>"
    } else {
        report.chosen.as_str()
    };
    let mut out = format!(
        "chosen={chosen} confidence={conf:.2} ledger={ledger}\nprobabilities={probs}\n",
        conf = report.confidence,
        ledger = report.ledger,
        probs = report.probabilities
    );
    if report.contract_failure {
        out.push_str(&format!(
            "contract failure: answer is not a candidate span or none\ncandidates={:?} beside {}\n",
            report.candidates, report.sentence
        ));
    }
    if report.spread {
        out.push_str(&format!(
            "spread none: candidates={:?} beside {}\n",
            report.candidates, report.sentence
        ));
    }
    out.push_str(&format!(
        "tokens in={} out={} wall_ms={}\n",
        report.input_tokens, report.output_tokens, report.elapsed_ms
    ));
    out
}
