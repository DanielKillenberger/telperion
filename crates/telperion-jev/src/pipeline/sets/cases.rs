//! Scoring the pipeline question sets against their labelled cases.
//!
//! Every judged question is scored twice: once over the labelled cases that
//! tuned its wording, once over the held-out cases. R7's bound is 0.9
//! accuracy for the described level, each question of the read stage's
//! label (fn-157) and the rights class (fn-129). `format_scores` prints the
//! confidence spread of each.

use std::path::Path;

use serde_json::json;

use super::{
    chosen_level, described_questions, described_state, label_cases, label_questions, LabelCase,
    DESCRIBED_UNSTATED, LABEL_QUESTIONS,
};
use crate::caller::{evaluate, CallerError, EvaluateRequest, Transport};
use crate::cases::{CaseRow, SetScore};
use crate::pipeline::requirements::table;
use crate::pipeline::stages::read::{label_state, Occurrence};
use crate::questions::thresholds;

/// One row and whether its case is held out.
pub(crate) type Rows = Vec<(CaseRow, bool)>;

/// Ask every case of every pipeline set and score each judged question over
/// the labelled cases and over the held-out cases.
pub fn run_pipeline_cases(
    transport: &dyn Transport,
    key: &str,
    ledger_dir: &Path,
) -> Result<Vec<SetScore>, CallerError> {
    let mut out = split(
        "described level",
        run_described(transport, key, ledger_dir)?,
        thresholds().accuracy_bar,
    );
    for (name, rows) in run_label(transport, key, ledger_dir)? {
        out.extend(split(&name, rows, thresholds().accuracy_bar));
    }
    let rights = crate::pipeline::rights::run_cases(transport, key, ledger_dir)?;
    out.extend(crate::pipeline::rights::scored(rights));
    Ok(out)
}

fn run_described(
    transport: &dyn Transport,
    key: &str,
    ledger_dir: &Path,
) -> Result<Rows, CallerError> {
    let mut rows = Rows::new();
    for case in super::described_cases() {
        let questions = described_questions(&case.levels);
        let state = described_state(&case);
        let entry = evaluate(
            transport,
            key,
            EvaluateRequest {
                tool: "described",
                source: None,
                state: &state,
                questions: &questions,
                ledger_dir,
            },
        )?;
        let index = chosen_level(
            entry.probabilities("level"),
            case.levels.len() + 1,
            case.levels.len(),
            thresholds().level_floor,
        );
        let answered = case
            .levels
            .get(index)
            .map(|level| level.key.clone())
            .unwrap_or_else(|| DESCRIBED_UNSTATED.into());
        rows.push((
            CaseRow {
                set: "described level".into(),
                id: case.id.clone(),
                expected: case.expect_level.clone(),
                hit: answered == case.expect_level,
                answered,
                top_probability: entry.top_probability("level"),
                confidence: entry.confidence("level").unwrap_or(0.0),
                ledger: entry.reference(),
            },
            case.holdout,
        ));
    }
    Ok(rows)
}

/// The occurrence a label case marks: the `occurrence`-th time its span
/// stands in its sentence.
pub fn marked(case: &LabelCase) -> Option<Occurrence> {
    let at = case
        .sentence
        .match_indices(&case.span)
        .nth(case.occurrence)?
        .0;
    Some(Occurrence {
        sentence: case.sentence.clone(),
        context: case.context.clone(),
        at,
        span: case.span.clone(),
    })
}

/// Asks every label case once, as the read stage lays it out, and scores
/// each of its four questions as a set of its own.
fn run_label(
    transport: &dyn Transport,
    key: &str,
    ledger_dir: &Path,
) -> Result<Vec<(String, Rows)>, CallerError> {
    let names: Vec<&str> = std::iter::once("field").chain(LABEL_QUESTIONS).collect();
    let mut sets: Vec<(String, Rows)> = names
        .iter()
        .map(|n| (format!("label {n}"), Rows::new()))
        .collect();
    for case in label_cases() {
        let Some(found) = marked(&case) else {
            continue;
        };
        let fields: Vec<(String, String)> = case
            .fields
            .iter()
            .map(|f| {
                (
                    f.clone(),
                    table()
                        .fields
                        .get(f)
                        .map_or_else(String::new, |t| t.what.clone()),
                )
            })
            .collect();
        let source = json!({"id": "case", "url": case.url});
        let state = label_state(&case.species, source, &found);
        let entry = evaluate(
            transport,
            key,
            EvaluateRequest {
                tool: "label",
                source: None,
                state: &state,
                questions: &label_questions(&fields),
                ledger_dir,
            },
        )?;
        let expected = [
            &case.expect_field,
            &case.expect_basis,
            &case.expect_age,
            &case.expect_condition,
        ];
        for ((name, rows), (question, want)) in sets.iter_mut().zip(names.iter().zip(expected)) {
            let answered = entry.choice(question).unwrap_or_default();
            rows.push((
                CaseRow {
                    set: name.clone(),
                    id: case.id.clone(),
                    expected: want.clone(),
                    hit: &answered == want,
                    answered,
                    top_probability: entry.top_probability(question),
                    confidence: entry.confidence(question).unwrap_or(0.0),
                    ledger: entry.reference(),
                },
                case.holdout,
            ));
        }
    }
    Ok(sets)
}

/// One SetScore over the labelled cases and one over the held-out cases.
pub(crate) fn split(name: &str, rows: Rows, bar: f64) -> Vec<SetScore> {
    let labelled = rows
        .iter()
        .filter(|(_, holdout)| !holdout)
        .map(|(row, _)| row.clone())
        .collect();
    let held_out = rows
        .iter()
        .filter(|(_, holdout)| *holdout)
        .map(|(row, _)| row.clone())
        .collect();
    vec![
        score(name.to_string(), labelled, bar),
        score(format!("{name} (held-out)"), held_out, bar),
    ]
}

fn score(name: String, rows: Vec<CaseRow>, bar: f64) -> SetScore {
    let hits = rows.iter().filter(|row| row.hit).count();
    let total = rows.len();
    let confidences = rows.iter().map(|row| row.confidence).collect();
    SetScore {
        name,
        hits,
        total,
        required: (bar * total as f64).ceil() as usize,
        ranking_ok: None,
        confidences,
        rows,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Candidate coverage before a label is trusted: every case's span is one
    /// the read stage itself finds in its sentence, at the occurrence named.
    #[test]
    fn every_label_case_marks_a_span_the_read_stage_finds() {
        for case in label_cases() {
            let want = marked(&case).unwrap_or_else(|| panic!("{}: no such occurrence", case.id));
            let found = crate::pipeline::stages::read::occurrences(&case.sentence, &case.fields);
            assert!(
                found.iter().any(|o| o.at == want.at && o.span == want.span),
                "{}: the read stage never marks {:?} at {}",
                case.id,
                case.span,
                want.at
            );
            assert!(case.fields.contains(&case.expect_field) || case.expect_field == "none");
        }
    }
}
