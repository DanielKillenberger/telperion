//! Literature screen: one row per candidate sentence.

use std::path::Path;

use serde_json::json;

use crate::caller::{evaluate, CallerError, EvaluateRequest, Transport};
use crate::extract::{candidate_sentences, split_for_state, visible_text};
use crate::ledger::{LedgerEntry, SourceRef};
use crate::questions::{screen_questions, thresholds};

#[derive(Debug, Clone)]
pub struct ScreenRow {
    pub sentence: String,
    pub kind: String,
    pub condition: String,
    pub anchor_probability: f64,
    pub anchor_usable: bool,
    pub ledger: String,
    pub kind_confidence: f64,
}

#[derive(Debug, Clone)]
pub struct ScreenReport {
    pub source_sha256: String,
    pub species: String,
    pub rows: Vec<ScreenRow>,
    pub input_tokens: u64,
    pub output_tokens: u64,
    pub elapsed_ms: u64,
}

/// Screen a fetched source. An empty candidate list is a successful empty table.
pub fn screen(
    transport: &dyn Transport,
    key: &str,
    ledger_dir: &Path,
    source: &SourceRef,
    bytes: &[u8],
    species: &str,
) -> Result<ScreenReport, CallerError> {
    let text = visible_text(bytes);
    let questions = screen_questions();
    let mut rows = Vec::new();
    let mut input_tokens = 0u64;
    let mut output_tokens = 0u64;
    let mut elapsed_ms = 0u64;

    for candidate in candidate_sentences(&text) {
        for (part, whole) in split_for_state(&candidate.sentence) {
            let state = json!({
                "species": species,
                "source": {
                    "id": source.id,
                    "url": source.url,
                    "sha256": source.sha256,
                    "bytes": source.bytes,
                },
                "candidate": {
                    "sentence": part,
                    "context": candidate.context,
                    "whole": whole,
                }
            });
            let entry = evaluate(
                transport,
                key,
                EvaluateRequest {
                    tool: "screen",
                    source: Some(source),
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
            rows.push(ScreenRow {
                sentence: part,
                kind: entry.choice("kind").unwrap_or_else(|| "none".into()),
                condition: entry
                    .choice("condition")
                    .unwrap_or_else(|| "unstated".into()),
                anchor_probability: entry.noul("anchor_usable").unwrap_or(0.0),
                anchor_usable: entry.noul("anchor_usable").unwrap_or(0.0)
                    >= thresholds().anchor_usable,
                ledger: entry.reference(),
                kind_confidence: entry.confidence("kind").unwrap_or(0.0),
            });
        }
    }

    Ok(ScreenReport {
        source_sha256: source.sha256.clone(),
        species: species.to_string(),
        rows,
        input_tokens,
        output_tokens,
        elapsed_ms,
    })
}

pub fn format_report(report: &ScreenReport) -> String {
    let mut out = String::new();
    out.push_str(&format!(
        "source_sha256={} species={} sentences={}\n",
        report.source_sha256,
        report.species,
        report.rows.len()
    ));
    if report.rows.is_empty() {
        out.push_str("(no candidate sentence)\n");
    }
    for row in &report.rows {
        out.push_str(&format!(
            "{kind}\t{condition}\tanchor={anchor:.2}\tusable={usable}\t{ledger}\t{sentence}\n",
            kind = row.kind,
            condition = row.condition,
            anchor = row.anchor_probability,
            usable = row.anchor_usable,
            ledger = row.ledger,
            sentence = row.sentence
        ));
    }
    out.push_str(&format!(
        "tokens in={} out={} wall_ms={}\n",
        report.input_tokens, report.output_tokens, report.elapsed_ms
    ));
    out
}

pub(crate) fn accumulate(
    entry: &LedgerEntry,
    input: &mut u64,
    output: &mut u64,
    elapsed: &mut u64,
) {
    if let Some(usage) = &entry.usage {
        *input += usage.input_tokens;
        *output += usage.output_tokens;
    }
    *elapsed += entry.elapsed_ms;
}
