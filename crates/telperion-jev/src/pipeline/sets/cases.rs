//! Scoring the four pipeline question sets against their labelled cases.
//!
//! Every judged question is scored twice: once over the labelled cases that
//! tuned its wording, once over the held-out cases. R7's bound is 0.9 accuracy
//! for the sufficiency level, the dominant gap, the mature size and its gap
//! (fn-127), the growth rate and its gap (fn-132), the described level and each obligation (the appearance support
//! among them, fn-128) and the rights class (fn-129), and 0.8 top-one agreement with the
//! person's admitted source for ranking. `format_scores` prints the confidence spread of each.

use std::path::Path;

use super::{
    appearance_state, chosen_level, described_questions, described_state, inspected_image_state,
    mature_questions, mature_state, measurement_state, obligation_questions, ranking_questions,
    ranking_state, rate_questions, sufficiency_questions, sufficiency_state, DESCRIBED_UNSTATED,
    RANKING_NONE, SUFFICIENCY_LEVELS,
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
    let stated = [
        (super::mature_cases(), mature_questions(), MATURE),
        (super::rate_cases(), rate_questions(), RATE),
    ];
    for (cases, questions, names) in stated {
        let (levels, gaps) = run_stated(transport, key, ledger_dir, cases, &questions, &names)?;
        out.extend(split(names.level_set, levels, thresholds().accuracy_bar));
        out.extend(split(names.gap_set, gaps, thresholds().accuracy_bar));
    }
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
    let o = super::obligation_cases();
    let nouls: [(&str, Vec<Noul>); 3] = [
        (
            "inspected_image",
            o.inspected_image
                .into_iter()
                .map(|c| {
                    (
                        c.id,
                        inspected_image_state(&c.observation),
                        c.expect,
                        c.holdout,
                    )
                })
                .collect(),
        ),
        (
            "measurement_not_invention",
            o.measurement_not_invention
                .into_iter()
                .map(|c| {
                    (
                        c.id,
                        measurement_state(None, &c.value_statement, &c.source_excerpt),
                        c.expect,
                        c.holdout,
                    )
                })
                .collect(),
        ),
        (
            "appearance_supported",
            o.appearance_supported
                .into_iter()
                .map(|c| {
                    (
                        c.id,
                        appearance_state(&c.trait_name, &c.level, &c.sentence),
                        c.expect,
                        c.holdout,
                    )
                })
                .collect(),
        ),
    ];
    for (name, cases) in nouls {
        out.extend(split(
            &format!("obligation {name}"),
            run_noul(transport, key, ledger_dir, name, cases)?,
            thresholds().accuracy_bar,
        ));
    }
    let rights = crate::pipeline::rights::run_cases(transport, key, ledger_dir)?;
    out.extend(crate::pipeline::rights::scored(rights));
    Ok(out)
}

/// One levelled case: its id, the state asked, the admitted level and gap,
/// and whether it is held out.
struct Levelled {
    id: String,
    state: serde_json::Value,
    level: String,
    gap: String,
    holdout: bool,
}

/// The names one levelled set is asked and scored under.
struct LevelledSet {
    tool: &'static str,
    score: &'static str,
    gap: &'static str,
    level_set: &'static str,
    gap_set: &'static str,
}

fn run_sufficiency(
    transport: &dyn Transport,
    key: &str,
    ledger_dir: &Path,
) -> Result<(Rows, Rows), CallerError> {
    let cases = super::sufficiency_cases().into_iter().map(|case| Levelled {
        state: sufficiency_state(&case),
        id: case.id,
        level: case.expect_level,
        gap: case.expect_gap,
        holdout: case.holdout,
    });
    let names = LevelledSet {
        tool: "sufficiency",
        score: "sufficiency",
        gap: "dominant_gap",
        level_set: "sufficiency level",
        gap_set: "sufficiency gap",
    };
    run_levelled(
        transport,
        key,
        ledger_dir,
        &sufficiency_questions(),
        &names,
        cases,
    )
}

/// A set judged on a stated value with no age: the mature size (fn-127)
/// and the growth rate (fn-132) lay out the same state.
fn run_stated(
    transport: &dyn Transport,
    key: &str,
    ledger_dir: &Path,
    cases: Vec<super::MatureCase>,
    questions: &serde_json::Value,
    names: &LevelledSet,
) -> Result<(Rows, Rows), CallerError> {
    let cases = cases.into_iter().map(|case| Levelled {
        state: mature_state(&case),
        id: case.id,
        level: case.expect_level,
        gap: case.expect_gap,
        holdout: case.holdout,
    });
    run_levelled(transport, key, ledger_dir, questions, names, cases)
}

const MATURE: LevelledSet = LevelledSet {
    tool: "mature_size",
    score: "mature_size",
    gap: "mature_gap",
    level_set: "mature size level",
    gap_set: "mature size gap",
};

const RATE: LevelledSet = LevelledSet {
    tool: "growth_rate",
    score: "growth_rate",
    gap: "rate_gap",
    level_set: "growth rate level",
    gap_set: "growth rate gap",
};

/// Asks every case of a four-level Score with its gap Choice and scores the
/// level and the gap as two sets.
fn run_levelled(
    transport: &dyn Transport,
    key: &str,
    ledger_dir: &Path,
    questions: &serde_json::Value,
    names: &LevelledSet,
    cases: impl Iterator<Item = Levelled>,
) -> Result<(Rows, Rows), CallerError> {
    let mut levels = Rows::new();
    let mut gaps = Rows::new();
    for case in cases {
        let entry = evaluate(
            transport,
            key,
            EvaluateRequest {
                tool: names.tool,
                source: None,
                state: &case.state,
                questions,
                ledger_dir,
            },
        )?;
        // The most probable level, as the gate itself reads it.
        let index = chosen_level(
            entry.probabilities(names.score),
            SUFFICIENCY_LEVELS.len(),
            0,
            0.0,
        );
        let level = SUFFICIENCY_LEVELS[index];
        levels.push((
            CaseRow {
                set: names.level_set.into(),
                id: case.id.clone(),
                expected: case.level.clone(),
                hit: level == case.level,
                answered: level.into(),
                top_probability: entry.top_probability(names.score),
                confidence: entry.confidence(names.score).unwrap_or(0.0),
                ledger: entry.reference(),
            },
            case.holdout,
        ));
        let gap = entry.choice(names.gap).unwrap_or_else(|| "none".into());
        gaps.push((
            CaseRow {
                set: names.gap_set.into(),
                id: case.id,
                expected: case.gap.clone(),
                hit: gap == case.gap,
                answered: gap,
                top_probability: entry.top_probability(names.gap),
                confidence: entry.confidence(names.gap).unwrap_or(0.0),
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

/// One obligation case: its id, the state asked, the side admitted, and
/// whether it is held out.
type Noul = (String, serde_json::Value, bool, bool);

fn run_noul(
    transport: &dyn Transport,
    key: &str,
    ledger_dir: &Path,
    name: &str,
    cases: Vec<Noul>,
) -> Result<Rows, CallerError> {
    let questions = obligation_questions(name);
    let tool = format!("obligation:{name}");
    let set = format!("obligation {name}");
    let mut rows = Rows::new();
    for (id, state, expect, holdout) in cases {
        let entry = evaluate(
            transport,
            key,
            EvaluateRequest {
                tool: &tool,
                source: None,
                state: &state,
                questions: &questions,
                ledger_dir,
            },
        )?;
        rows.push((noul_row(&set, &id, expect, &entry, name), holdout));
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
