//! Jev answers narrow questions over the set; code reads them into signals;
//! the table names the route (R3).
//!
//! One request per option set. The signals are the top option's: the spread
//! between the top two best-match probabilities, whether it touches the
//! generator (declared or judged), whether it moves a pin (declared or
//! judged), its reversibility, the prior-verdict coverage, and the five owner
//! flags. Every routed gap records its judgments, its signals, its route and
//! the table version, so a changed table re-routes it without a new call.
//! The loop escalates once: a set the stronger model wrote that routes to the
//! stronger model is the owner's.

use std::collections::BTreeMap;

use serde_json::{json, Value};

use super::option::{latest_set, Author, ChangeKind, GapOption};
use super::questions::{gap_questions, gap_state, BEST_MATCH_NONE};
use super::table::{self, Decided, Route};
use super::{now, slug, GapError, STAGE};
use crate::ledger::LedgerEntry;
use crate::pipeline::canon::canonical_sha256;
use crate::pipeline::decision::{append_decisions, Decision, DecisionParts, Resolution, Status};
use crate::pipeline::judge::Judge;
use crate::pipeline::stage::Paths;
use crate::questions::thresholds;

/// What one request answered, read into the record's shape.
#[derive(Debug, Clone, PartialEq)]
pub struct Read {
    pub judgments: Value,
    pub signals: Value,
    /// The best match, or `None` when Jev answered none.
    pub chosen: Option<String>,
}

/// Reads the answers over `options` into judgments and signals. Pure, so a
/// recorded entry reads the same way twice.
pub fn read_answers(entry: &LedgerEntry, options: &[GapOption]) -> Read {
    let pin_cut = thresholds().obligation_cut;
    let mut judgments = serde_json::Map::new();
    for option in options {
        let id = &option.option;
        let kind = entry
            .choice(&format!("change_kind:{id}"))
            .unwrap_or_else(|| "unstated".into());
        let pin = entry.noul(&format!("moves_pin:{id}")).unwrap_or(0.0);
        judgments.insert(
            id.clone(),
            json!({
                "change_kind": {"declared": option.change_kind.key(), "judged": kind,
                                "agrees": kind == option.change_kind.key()},
                "prior_verdict": entry
                    .choice(&format!("prior_verdict:{id}"))
                    .unwrap_or_else(|| "none".into()),
                "generalizes": entry.noul(&format!("generalizes:{id}")).unwrap_or(0.0) >= pin_cut,
                "moves_pin": {"declared": option.moves_pin, "judged": pin >= pin_cut},
            }),
        );
    }
    let probabilities: BTreeMap<String, f64> = entry
        .probabilities("best_match")
        .and_then(Value::as_object)
        .map(|map| {
            map.iter()
                .filter_map(|(k, v)| v.as_f64().map(|p| (k.clone(), p)))
                .collect()
        })
        .unwrap_or_default();
    let mut ranked: Vec<(&String, &f64)> = probabilities.iter().collect();
    ranked.sort_by(|a, b| b.1.total_cmp(a.1).then(a.0.cmp(b.0)));
    let spread = match ranked.as_slice() {
        [first, second, ..] => first.1 - second.1,
        [first] => *first.1,
        [] => 0.0,
    };
    let choice = entry
        .choice("best_match")
        .unwrap_or_else(|| BEST_MATCH_NONE.into());
    let chosen = options
        .iter()
        .find(|o| o.option == choice)
        .map(|o| o.option.clone());
    // The option-level signals are the top option's; with no match they are
    // the runner-up's, so the record still carries every signal.
    let top = chosen
        .as_ref()
        .and_then(|id| options.iter().find(|o| &o.option == id))
        .or_else(|| {
            ranked
                .iter()
                .find_map(|(id, _)| options.iter().find(|o| &o.option == *id))
        })
        .or_else(|| options.first());
    let signals = match top {
        Some(top) => {
            let judged = &judgments[&top.option];
            json!({
                "spread": spread,
                "best_match": if chosen.is_some() { choice.clone() } else { BEST_MATCH_NONE.into() },
                "generator_touch": top.change_kind == ChangeKind::Generator
                    || judged["change_kind"]["judged"] == "generator",
                "moves_pin": top.moves_pin || judged["moves_pin"]["judged"] == true,
                "reversible": top.reversible,
                "prior_verdict": judged["prior_verdict"],
                "changes_preset_output": !top.changes_preset_output.is_empty(),
                "spends_captures": top.spends_captures,
                "lowers_bar": top.lowers_bar,
                "changes_boundary": top.changes_boundary,
            })
        }
        None => json!({}),
    };
    Read {
        judgments: json!({"options": judgments, "best_match": {"choice": choice,
                          "probabilities": probabilities, "spread": spread}}),
        signals,
        chosen,
    }
}

/// The route the table gives a set, with the one escalation the loop allows.
pub fn decide(author: Author, signals: &Value) -> Decided {
    let decided = table::load().route(signals);
    if decided.route == Route::Stronger && author == Author::Stronger {
        return Decided {
            route: Route::Owner,
            row: decided.row,
            why: format!(
                "{}; the stronger model's set routes no further, so the owner takes it",
                decided.why
            ),
        };
    }
    decided
}

#[derive(Debug, Clone, PartialEq)]
pub struct Routed {
    pub route: Route,
    pub chosen: Option<String>,
    pub why: String,
    /// The gap-fix decision filed, when the route files one.
    pub decision: Option<String>,
}

/// Asks Jev over the latest set, routes it, records it, and files the
/// gap-fix decision: resolved by the loop on proceed, open for the owner.
pub fn route(
    paths: &Paths,
    judge: &Judge<'_>,
    gap_id: &str,
    verdicts: &[String],
) -> Result<Routed, GapError> {
    let mut record = super::read(paths, gap_id)?;
    let (author, _, options) = latest_set(&record)?;
    if options.is_empty() {
        return Err(GapError::Invalid(
            "the latest set is empty; it routed by itself".into(),
        ));
    }
    let ids: Vec<String> = options.iter().map(|o| o.option.clone()).collect();
    let gap = json!({
        "id": gap_id,
        "species": record["species"],
        "capability": record["halt"]["field"],
        "detail": record["halt"]["payload"],
    });
    let state = gap_state(&gap, verdicts, &options);
    let questions = gap_questions(&ids);
    let judgment = judge.ask("gap", None, &state, &questions)?;
    let read = read_answers(&judgment.entry, &options);
    let decided = decide(author, &read.signals);
    let entry = json!({
        "author": author.key(),
        "at": now(),
        "options": ids,
        "verdicts": verdicts,
        "judgments": read.judgments,
        "signals": read.signals,
        "route": decided.route.key(),
        "row": decided.row,
        "why": decided.why,
        "table_version": table::load().version,
        "ledger": [judgment.reference],
        "chosen": read.chosen,
    });
    finish(paths, &mut record, entry, &options, &decided, read.chosen)
}

/// Re-reads the latest recorded signals against the current table, with no
/// call, and records the new route beside the old.
pub fn reroute(paths: &Paths, gap_id: &str) -> Result<Routed, GapError> {
    let mut record = super::read(paths, gap_id)?;
    let (author, _, options) = latest_set(&record)?;
    let Some(last) = record["routes"].as_array().and_then(|r| r.last()).cloned() else {
        return Err(GapError::Invalid("nothing is routed yet".into()));
    };
    if last["signals"].as_object().is_none_or(|s| s.is_empty()) {
        return Err(GapError::Invalid(
            "the latest route carries no signals; an empty set routes by itself".into(),
        ));
    }
    let decided = decide(author, &last["signals"]);
    let chosen = last["chosen"].as_str().map(str::to_string);
    let mut entry = last.clone();
    entry["at"] = json!(now());
    entry["route"] = json!(decided.route.key());
    entry["row"] = json!(decided.row);
    entry["why"] = json!(decided.why);
    entry["table_version"] = json!(table::load().version);
    entry["ledger"] = json!([]);
    entry["reroute_of"] = json!(record["routes"].as_array().map_or(0, |r| r.len() - 1));
    finish(paths, &mut record, entry, &options, &decided, chosen)
}

fn finish(
    paths: &Paths,
    record: &mut Value,
    entry: Value,
    options: &[GapOption],
    decided: &Decided,
    chosen: Option<String>,
) -> Result<Routed, GapError> {
    let ledger: Vec<String> = entry["ledger"]
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(|v| v.as_str().map(str::to_string))
        .collect();
    record["routes"]
        .as_array_mut()
        .expect("routes is a list")
        .push(entry);
    record["route"] = json!(decided.route.key());
    record["chosen"] = json!(chosen);
    let decision = match decided.route {
        Route::Stronger => None,
        Route::Proceed | Route::Owner => {
            let decision = gap_fix(record, options, decided, chosen.as_deref(), ledger);
            let id = decision.id.clone();
            append_decisions(&paths.decisions(), vec![decision])?;
            Some(id)
        }
    };
    super::write(paths, record)?;
    Ok(Routed {
        route: decided.route,
        chosen,
        why: decided.why.clone(),
        decision,
    })
}

/// The gap-fix decision: its options are the set's ids and `none`; its inputs
/// are the set and the table version, so a new set or a changed table
/// reissues it. On proceed it carries the loop's own resolution.
fn gap_fix(
    record: &Value,
    options: &[GapOption],
    decided: &Decided,
    chosen: Option<&str>,
    ledger: Vec<String>,
) -> Decision {
    let species = record["species"].as_str().unwrap_or_default();
    let gap_id = record["gap"].as_str().unwrap_or_default();
    let field = slug(gap_id);
    let mut ids: Vec<&str> = options.iter().map(|o| o.option.as_str()).collect();
    ids.push(BEST_MATCH_NONE);
    let inputs: BTreeMap<String, String> = [
        ("options".to_string(), canonical_sha256(&json!(options))),
        (
            "table_version".to_string(),
            table::load().version.to_string(),
        ),
    ]
    .into();
    let mut decision = Decision::new(
        DecisionParts {
            species,
            stage: STAGE,
            kind: "gap-fix",
            field: Some(&field),
            age_years: None,
        },
        &[],
        inputs.clone(),
        ledger,
        json!({"gap": gap_id, "route": decided.route.key(), "chosen": chosen,
               "why": decided.why, "row": decided.row, "table_version": table::load().version}),
        &ids,
        "The fix for this gap: the loop's own choice on proceed, the owner's otherwise. \
         A resolution by the owner that names another option is a reversal the next \
         threshold tuning reads.",
    );
    if decided.route == Route::Proceed {
        if let Some(chosen) = chosen {
            decision.status = Status::Resolved;
            decision.resolution = Some(Resolution {
                id: decision.id.clone(),
                inputs_sha256: inputs,
                option: chosen.into(),
                by: "loop".into(),
                at: now(),
                note: decided.why.clone(),
                payload: Value::Null,
            });
        }
    }
    decision
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pipeline::gap::option::tests::option;

    fn entry(answers: Value) -> LedgerEntry {
        LedgerEntry {
            id: "e".into(),
            tool: "gap".into(),
            state_sha256: "s".into(),
            source: None,
            model: "jev-latest".into(),
            questions: json!({}),
            answers,
            usage: None,
            elapsed_ms: 1,
            recorded_at: "2026-09-18T00:00:00Z".into(),
            error: None,
            identity: "i".into(),
        }
    }

    fn pair() -> Vec<GapOption> {
        let mut b = option("g", "b", ChangeKind::ValueTable);
        b.moves_pin = true;
        vec![option("g", "a", ChangeKind::Generator), b]
    }

    #[test]
    fn the_signals_are_the_top_options_with_the_spread_read_from_the_distribution() {
        let read = read_answers(
            &entry(json!({
                "change_kind:a": {"choice": "generator"},
                "change_kind:b": {"choice": "generator"},
                "prior_verdict:a": {"choice": "for"},
                "generalizes:a": {"noul": 0.9},
                "moves_pin:a": {"noul": 0.1},
                "best_match": {"choice": "a", "probabilities": {"a": 0.7, "b": 0.2, "none": 0.1}},
            })),
            &pair(),
        );
        assert_eq!(read.chosen.as_deref(), Some("a"));
        let s = &read.signals;
        assert!((s["spread"].as_f64().unwrap() - 0.5).abs() < 1e-9);
        assert_eq!(s["best_match"], "a");
        assert_eq!(s["generator_touch"], true);
        assert_eq!(s["moves_pin"], false);
        assert_eq!(s["prior_verdict"], "for");
        assert_eq!(
            read.judgments["options"]["b"]["change_kind"]["agrees"],
            false
        );
        assert_eq!(read.judgments["options"]["a"]["generalizes"], true);
        assert_eq!(decide(Author::Agent, s).route, Route::Proceed);
    }

    #[test]
    fn a_judged_pin_counts_like_a_declared_one_and_no_match_keeps_every_signal() {
        let read = read_answers(
            &entry(json!({
                "moves_pin:a": {"noul": 0.8},
                "best_match": {"choice": "a", "probabilities": {"a": 0.6, "b": 0.4}},
            })),
            &pair(),
        );
        assert_eq!(read.signals["moves_pin"], true);
        assert_eq!(decide(Author::Agent, &read.signals).route, Route::Owner);
        let none = read_answers(
            &entry(json!({
                "best_match": {"choice": "none", "probabilities": {"none": 0.5, "b": 0.3, "a": 0.2}},
            })),
            &pair(),
        );
        assert_eq!(none.chosen, None);
        assert_eq!(none.signals["best_match"], "none");
        // The runner-up b carries the option-level signals: it declares a pin.
        assert_eq!(none.signals["moves_pin"], true);
        assert_eq!(decide(Author::Agent, &none.signals).route, Route::Owner);
        let mut s = none.signals.clone();
        s["moves_pin"] = json!(false);
        assert_eq!(decide(Author::Agent, &s).route, Route::Stronger);
        assert_eq!(decide(Author::Stronger, &s).route, Route::Owner);
    }

    #[test]
    fn an_empty_answer_set_still_records_a_full_signal_row() {
        let read = read_answers(&entry(json!({})), &pair());
        assert_eq!(read.chosen, None);
        assert_eq!(read.signals["spread"], 0.0);
        assert_eq!(read.signals["best_match"], "none");
        assert!(read.signals["reversible"].is_boolean());
    }
}
