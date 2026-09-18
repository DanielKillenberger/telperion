//! Scoring the four pipeline question sets against their labelled cases.
//!
//! Every judged question is scored twice: once over the labelled cases that
//! tuned its wording, once over the held-out cases. R7's bound is 0.9 accuracy
//! for the sufficiency level, the dominant gap, the described level and each
//! obligation, and 0.8 top-one agreement with the person's admitted source for
//! ranking. `format_scores` prints the confidence spread of each.

use std::path::Path;

use super::{
    described_questions, described_state, inspected_image_state, level_from_score,
    measurement_state, obligation_questions, ranking_questions, ranking_state,
    sufficiency_questions, sufficiency_state, DESCRIBED_UNSTATED, RANKING_NONE, SUFFICIENCY_LEVELS,
};
use crate::caller::{evaluate, CallerError, EvaluateRequest, Transport};
use crate::cases::{CaseRow, SetScore};
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
    let (levels, gaps) = run_sufficiency(transport, key, ledger_dir)?;
    let mut out = split("sufficiency level", levels, thresholds().accuracy_bar);
    out.extend(split("sufficiency gap", gaps, thresholds().accuracy_bar));
    out.extend(split(
        "ranking source",
        run_ranking(transport, key, ledger_dir)?,
        thresholds().ranking_bar,
    ));
    out.extend(split(
        "described level",
        run_described(transport, key, ledger_dir)?,
        thresholds().accuracy_bar,
    ));
    out.extend(split(
        "obligation inspected_image",
        run_inspected_image(transport, key, ledger_dir)?,
        thresholds().accuracy_bar,
    ));
    out.extend(split(
        "obligation measurement_not_invention",
        run_measurement(transport, key, ledger_dir)?,
        thresholds().accuracy_bar,
    ));
    Ok(out)
}

fn run_sufficiency(
    transport: &dyn Transport,
    key: &str,
    ledger_dir: &Path,
) -> Result<(Rows, Rows), CallerError> {
    let questions = sufficiency_questions();
    let mut levels = Rows::new();
    let mut gaps = Rows::new();
    for case in super::sufficiency_cases() {
        let state = sufficiency_state(&case);
        let entry = evaluate(
            transport,
            key,
            EvaluateRequest {
                tool: "sufficiency",
                source: None,
                state: &state,
                questions: &questions,
                ledger_dir,
            },
        )?;
        // A missing score is the lowest level, as the gate itself reads it.
        let index = level_from_score(
            entry.score("sufficiency").unwrap_or(f64::NAN),
            SUFFICIENCY_LEVELS.len(),
        )
        .unwrap_or(0);
        let level = SUFFICIENCY_LEVELS[index];
        levels.push((
            CaseRow {
                set: "sufficiency level".into(),
                id: case.id.clone(),
                expected: case.expect_level.clone(),
                hit: level == case.expect_level,
                answered: level.into(),
                top_probability: entry.top_probability("sufficiency"),
                confidence: entry.confidence("sufficiency").unwrap_or(0.0),
                ledger: entry.reference(),
            },
            case.holdout,
        ));
        let gap = entry
            .choice("dominant_gap")
            .unwrap_or_else(|| "none".into());
        gaps.push((
            CaseRow {
                set: "sufficiency gap".into(),
                id: case.id.clone(),
                expected: case.expect_gap.clone(),
                hit: gap == case.expect_gap,
                answered: gap,
                top_probability: entry.top_probability("dominant_gap"),
                confidence: entry.confidence("dominant_gap").unwrap_or(0.0),
                ledger: entry.reference(),
            },
            case.holdout,
        ));
    }
    Ok((levels, gaps))
}

fn run_ranking(
    transport: &dyn Transport,
    key: &str,
    ledger_dir: &Path,
) -> Result<Rows, CallerError> {
    let mut rows = Rows::new();
    for case in super::ranking_cases() {
        let ids: Vec<String> = case.candidates.iter().map(|c| c.id.clone()).collect();
        let questions = ranking_questions(&ids);
        let state = ranking_state(&case);
        let entry = evaluate(
            transport,
            key,
            EvaluateRequest {
                tool: "ranking",
                source: None,
                state: &state,
                questions: &questions,
                ledger_dir,
            },
        )?;
        let answered = entry
            .choice("source")
            .unwrap_or_else(|| RANKING_NONE.into());
        rows.push((
            CaseRow {
                set: "ranking source".into(),
                id: case.id.clone(),
                expected: case.expect_source.clone(),
                hit: answered == case.expect_source,
                answered,
                top_probability: entry.top_probability("source"),
                confidence: entry.confidence("source").unwrap_or(0.0),
                ledger: entry.reference(),
            },
            case.holdout,
        ));
    }
    Ok(rows)
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
        let index = level_from_score(
            entry.score("level").unwrap_or(f64::NAN),
            case.levels.len() + 1,
        )
        .unwrap_or(case.levels.len());
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

fn run_inspected_image(
    transport: &dyn Transport,
    key: &str,
    ledger_dir: &Path,
) -> Result<Rows, CallerError> {
    let questions = obligation_questions("inspected_image");
    let mut rows = Rows::new();
    for case in super::obligation_cases().inspected_image {
        let state = inspected_image_state(&case.observation);
        let entry = evaluate(
            transport,
            key,
            EvaluateRequest {
                tool: "obligation:inspected_image",
                source: None,
                state: &state,
                questions: &questions,
                ledger_dir,
            },
        )?;
        rows.push((
            noul_row(
                "obligation inspected_image",
                &case.id,
                case.expect,
                &entry,
                "inspected_image",
            ),
            case.holdout,
        ));
    }
    Ok(rows)
}

fn run_measurement(
    transport: &dyn Transport,
    key: &str,
    ledger_dir: &Path,
) -> Result<Rows, CallerError> {
    let questions = obligation_questions("measurement_not_invention");
    let mut rows = Rows::new();
    for case in super::obligation_cases().measurement_not_invention {
        let state = measurement_state(&case.value_statement, &case.source_excerpt);
        let entry = evaluate(
            transport,
            key,
            EvaluateRequest {
                tool: "obligation:measurement_not_invention",
                source: None,
                state: &state,
                questions: &questions,
                ledger_dir,
            },
        )?;
        rows.push((
            noul_row(
                "obligation measurement_not_invention",
                &case.id,
                case.expect,
                &entry,
                "measurement_not_invention",
            ),
            case.holdout,
        ));
    }
    Ok(rows)
}

fn noul_row(
    set: &str,
    id: &str,
    expect: bool,
    entry: &crate::ledger::LedgerEntry,
    question: &str,
) -> CaseRow {
    let probability = entry.noul(question).unwrap_or(0.0);
    let answered = probability >= 0.5;
    CaseRow {
        set: set.into(),
        id: id.into(),
        expected: expect.to_string(),
        answered: answered.to_string(),
        top_probability: entry.top_probability(question),
        confidence: entry.top_probability(question),
        ledger: entry.reference(),
        hit: answered == expect,
    }
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
