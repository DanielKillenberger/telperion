//! The option question set (R2): versioned data, labelled by the owner's
//! actual choices for fn-34's gaps.
//!
//! One request per gap carries the gap, the owner's verdicts and the option
//! set, with the declared change kind and pin movement left out so Jev reads
//! them from the summary and the touch. Per option: change kind, prior
//! verdict, whether it generalizes, whether it moves a pin. Over the set:
//! the best match, with `none` as its no-match answer; how close the top two
//! are is code's reading of that distribution. The labelled cases live in
//! `data/cases/gap.json`; `jev cases` scores them live, held out and not.

use std::collections::BTreeMap;
use std::path::Path;

use serde::Deserialize;
use serde_json::{json, Map, Value};

use super::option::GapOption;
use crate::caller::{evaluate, CallerError, EvaluateRequest, Transport};
use crate::cases::{CaseRow, SetScore};
use crate::pipeline::sets::cases::split;
use crate::questions::thresholds;

pub const GAP_JSON: &str = include_str!("../../../data/questions/gap.json");
pub const GAP_CASES: &str = include_str!("../../../data/cases/gap.json");
/// The no-match key of the best-match Choice.
pub const BEST_MATCH_NONE: &str = "none";
/// The four questions asked once per option, keyed `<question>:<option>`.
pub const PER_OPTION: [&str; 4] = ["change_kind", "prior_verdict", "generalizes", "moves_pin"];

fn raw() -> Value {
    serde_json::from_str(GAP_JSON).expect("gap.json parses")
}

pub fn version() -> u32 {
    raw()["version"].as_u64().unwrap_or(0) as u32
}

/// The request's questions over `options`, in the set's order.
pub fn gap_questions(options: &[String]) -> Value {
    let raw = raw();
    let mut out = Map::new();
    for id in options {
        for name in PER_OPTION {
            let mut question = raw[name].clone();
            let instructions = question["instructions"].as_str().unwrap_or_default();
            question["instructions"] = json!(format!(
                "{instructions} This question is about the option named `{id}`."
            ));
            out.insert(format!("{name}:{id}"), question);
        }
    }
    let mut criteria = Map::new();
    for id in options {
        criteria.insert(id.clone(), Value::Null);
    }
    criteria.insert(
        BEST_MATCH_NONE.into(),
        raw["best_match"][BEST_MATCH_NONE].clone(),
    );
    out.insert(
        "best_match".into(),
        json!({
            "type": "choice",
            "instructions": raw["best_match"]["instructions"],
            "criteria": criteria,
        }),
    );
    Value::Object(out)
}

/// The state: the gap as the halt stated it, the owner's verdicts, and each
/// option without its declared change kind or pin movement.
pub fn gap_state(gap: &Value, verdicts: &[String], options: &[GapOption]) -> Value {
    let shown: Vec<Value> = options
        .iter()
        .map(|o| {
            json!({
                "option": o.option,
                "touches": o.touches,
                "summary": o.summary,
                "serves_species": o.serves_species,
                "changes_preset_output": o.changes_preset_output,
                "reversible": o.reversible,
            })
        })
        .collect();
    json!({ "gap": gap, "verdicts": verdicts, "options": shown })
}

/// One labelled gap: the halt, the verdicts, the options the agent had, and
/// the owner's actual choice as the labels.
#[derive(Debug, Clone, Deserialize)]
pub struct GapCase {
    pub id: String,
    pub gap: Value,
    pub verdicts: Vec<String>,
    pub options: Vec<GapOption>,
    /// Option id -> `for` | `against` | `none`.
    pub expect_prior_verdict: BTreeMap<String, String>,
    /// The option the owner chose, or `none`.
    pub expect_best_match: String,
    pub holdout: bool,
    pub negative: bool,
}

pub fn gap_cases() -> Vec<GapCase> {
    serde_json::from_str(GAP_CASES).expect("gap cases")
}

pub fn case_state(case: &GapCase) -> Value {
    gap_state(&case.gap, &case.verdicts, &case.options)
}

/// Asks every labelled gap once and scores change kind, prior verdict and
/// best match, labelled and held out. Change kind and prior verdict are held
/// to the accuracy bar, best match to the ranking bar.
pub fn run_gap_cases(
    transport: &dyn Transport,
    key: &str,
    ledger_dir: &Path,
) -> Result<Vec<SetScore>, CallerError> {
    let mut kinds = Vec::new();
    let mut verdicts = Vec::new();
    let mut matches = Vec::new();
    for case in gap_cases() {
        let ids: Vec<String> = case.options.iter().map(|o| o.option.clone()).collect();
        let questions = gap_questions(&ids);
        let state = case_state(&case);
        let entry = evaluate(
            transport,
            key,
            EvaluateRequest {
                tool: "gap",
                source: None,
                state: &state,
                questions: &questions,
                ledger_dir,
            },
        )?;
        for option in &case.options {
            let kind = format!("change_kind:{}", option.option);
            let answered = entry.choice(&kind).unwrap_or_else(|| "unstated".into());
            kinds.push((
                row(
                    "gap change_kind",
                    &format!("{}/{}", case.id, option.option),
                    option.change_kind.key(),
                    answered,
                    &entry,
                    &kind,
                ),
                case.holdout,
            ));
            let verdict = format!("prior_verdict:{}", option.option);
            let expected = case
                .expect_prior_verdict
                .get(&option.option)
                .cloned()
                .unwrap_or_else(|| "none".into());
            let answered = entry.choice(&verdict).unwrap_or_else(|| "none".into());
            verdicts.push((
                row(
                    "gap prior_verdict",
                    &format!("{}/{}", case.id, option.option),
                    &expected,
                    answered,
                    &entry,
                    &verdict,
                ),
                case.holdout,
            ));
        }
        let answered = entry
            .choice("best_match")
            .unwrap_or_else(|| BEST_MATCH_NONE.into());
        matches.push((
            row(
                "gap best_match",
                &case.id,
                &case.expect_best_match,
                answered,
                &entry,
                "best_match",
            ),
            case.holdout,
        ));
    }
    let mut out = split("gap change_kind", kinds, thresholds().accuracy_bar);
    out.extend(split(
        "gap prior_verdict",
        verdicts,
        thresholds().accuracy_bar,
    ));
    out.extend(split("gap best_match", matches, thresholds().ranking_bar));
    Ok(out)
}

fn row(
    set: &str,
    id: &str,
    expected: &str,
    answered: String,
    entry: &crate::ledger::LedgerEntry,
    question: &str,
) -> CaseRow {
    CaseRow {
        set: set.into(),
        id: id.into(),
        expected: expected.into(),
        hit: answered == expected,
        answered,
        top_probability: entry.top_probability(question),
        confidence: entry.confidence(question).unwrap_or(0.0),
        ledger: entry.reference(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_questions_ask_four_per_option_and_one_best_match_with_a_no_match_key() {
        let questions = gap_questions(&["a".into(), "b".into()]);
        let map = questions.as_object().unwrap();
        assert_eq!(map.len(), 2 * PER_OPTION.len() + 1);
        assert!(map["change_kind:a"]["instructions"]
            .as_str()
            .unwrap()
            .contains("named `a`"));
        assert_eq!(map["generalizes:b"]["type"], "noul");
        let criteria = map["best_match"]["criteria"].as_object().unwrap();
        assert!(criteria.contains_key("a") && criteria.contains_key("b"));
        assert!(criteria[BEST_MATCH_NONE].is_string());
        assert_eq!(version(), 1);
    }

    #[test]
    fn every_case_is_answerable_and_the_set_carries_a_no_match_case() {
        let cases = gap_cases();
        assert!(
            cases.len() >= 15,
            "the fifteen gaps of fn-34: {}",
            cases.len()
        );
        let mut negatives = 0;
        let mut holdouts = 0;
        for case in &cases {
            assert!(
                (2..=super::super::option::MAX_OPTIONS).contains(&case.options.len()),
                "{}: {} options",
                case.id,
                case.options.len()
            );
            let ids: Vec<&str> = case.options.iter().map(|o| o.option.as_str()).collect();
            for option in &case.options {
                assert_eq!(option.gap, case.gap["id"], "{}", case.id);
                assert_eq!(
                    ids.iter().filter(|id| **id == option.option).count(),
                    1,
                    "{}: {} is listed twice",
                    case.id,
                    option.option
                );
                // Every option a question is asked about carries its label.
                assert!(
                    case.expect_prior_verdict.contains_key(&option.option),
                    "{}: {} has no prior-verdict label",
                    case.id,
                    option.option
                );
            }
            // The expected best match is an option of this case, or the
            // no-match answer, and only a negative case expects no match.
            if case.negative {
                negatives += 1;
                assert_eq!(case.expect_best_match, BEST_MATCH_NONE, "{}", case.id);
            } else {
                assert!(
                    ids.contains(&case.expect_best_match.as_str()),
                    "{}: expects {}, which is not an option",
                    case.id,
                    case.expect_best_match
                );
            }
            holdouts += usize::from(case.holdout);
        }
        assert!(negatives >= 1, "the set carries a no-match case");
        assert!(holdouts >= 3, "held out: {holdouts}");
        assert!(holdouts < cases.len(), "every case held out scores nothing");
    }

    #[test]
    fn the_state_leaves_the_declared_kind_and_pin_out() {
        let case = gap_cases().into_iter().next().unwrap();
        let state = case_state(&case);
        let shown = &state["options"][0];
        assert!(shown.get("change_kind").is_none());
        assert!(shown.get("moves_pin").is_none());
        assert!(shown["summary"].is_string());
        assert_eq!(state["gap"], case.gap);
    }
}
