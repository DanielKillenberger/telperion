//! One explicit policy maps Jev's judgments to a route (R5). The table is
//! `data/conductor-policy.json`: its rows are tried in order over the
//! recorded signals and the first whose conditions all hold names the route;
//! the human rows sit first, so an unjustified attempt is never bought by a
//! later row. A signal the table names that the record lacks is the human's
//! by rule. Nothing here calls Jev; the signals are read from ledger entries
//! and from the run's own counters, so a changed table re-routes a recorded
//! run with no new call.
use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_json::{json, Map, Value};

use super::dispatch::Role;

pub const POLICY_JSON: &str = include_str!("../../data/conductor-policy.json");

/// A judgment under the confidence floor.
pub const INSUFFICIENT: &str = "insufficient_evidence";
/// A judgment still under the floor after one verified investigation of
/// the same revision: the table sends it to the human.
pub const INVESTIGATED: &str = "insufficient_after_investigation";

/// The tier and effort a route dispatches to. The tier is a name the
/// instruction file's routing block resolves to a model; the conductor
/// records the name and the result records the model that actually ran.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Allocation {
    pub role: Role,
    pub tier: String,
    pub effort: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Row {
    pub when: Map<String, Value>,
    pub route: String,
    pub why: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Decisions {
    pub note: String,
    pub routine: BTreeMap<String, Vec<String>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Table {
    pub version: u32,
    pub set_from: String,
    pub tiers: BTreeMap<String, String>,
    pub routes: BTreeMap<String, Allocation>,
    pub signals: Vec<String>,
    /// Equivalent failed attempts without progress before the human has it.
    pub no_progress_attempts: usize,
    /// A judgment below this confidence reads as insufficient evidence.
    pub min_confidence: f64,
    pub rows: Vec<Row>,
    pub decisions: Decisions,
}

pub fn load() -> Table {
    parse(POLICY_JSON).expect("conductor-policy.json parses")
}

pub fn parse(raw: &str) -> Result<Table, String> {
    let table: Table = serde_json::from_str(raw).map_err(|err| err.to_string())?;
    if table.rows.is_empty() {
        return Err("the policy has no rows".into());
    }
    if !table.rows.last().is_some_and(|row| row.when.is_empty()) {
        return Err("the policy's last row must match everything".into());
    }
    if table.rows.last().is_some_and(|row| row.route != "human") {
        return Err("the policy's catch-all row must be the human's".into());
    }
    for row in &table.rows {
        if !table.routes.contains_key(&row.route) {
            return Err(format!("row routes to an unlisted route {}", row.route));
        }
        for key in row.when.keys() {
            if !table.signals.contains(key) {
                return Err(format!("row condition {key} names no listed signal"));
            }
        }
    }
    for (name, allocation) in &table.routes {
        if name != "human" && !table.tiers.contains_key(&allocation.tier) {
            return Err(format!(
                "route {name} names an unlisted tier {}",
                allocation.tier
            ));
        }
    }
    Ok(table)
}

/// What the table decided: the route, the row that matched and its reason.
#[derive(Debug, Clone, PartialEq)]
pub struct Decided {
    pub route: String,
    /// `None` when a signal was missing and the human took it by rule.
    pub row: Option<usize>,
    pub why: String,
}

impl Decided {
    pub fn human(&self) -> bool {
        self.route == "human"
    }
}

impl Table {
    /// Looks the route up over `signals`. A missing signal is the human's.
    pub fn route(&self, signals: &Value) -> Decided {
        let missing: Vec<&str> = self
            .signals
            .iter()
            .map(String::as_str)
            .filter(|name| signals.get(name).is_none_or(Value::is_null))
            .collect();
        if !missing.is_empty() {
            return Decided {
                route: "human".into(),
                row: None,
                why: format!("signal missing from the record: {}", missing.join(", ")),
            };
        }
        for (index, row) in self.rows.iter().enumerate() {
            if row
                .when
                .iter()
                .all(|(key, expected)| signals.get(key) == Some(expected))
            {
                return Decided {
                    route: row.route.clone(),
                    row: Some(index),
                    why: row.why.clone(),
                };
            }
        }
        unreachable!("the last row matches everything")
    }

    pub fn allocation(&self, route: &str) -> Option<&Allocation> {
        self.routes.get(route)
    }

    /// Whether a cheap agent may resolve `kind` with `option` under policy.
    pub fn routine_decision(&self, kind: &str, option: &str) -> bool {
        self.decisions
            .routine
            .get(kind)
            .is_some_and(|options| options.iter().any(|o| o == option))
    }
}

/// The signals a route record carries, built by code from judgments and
/// counters. Every field is written; `not_asked` and `none` are values, not
/// absences, so a missing signal is always a record defect.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Signals {
    pub hard_limit: bool,
    /// `justified`, `unjustified` or `unavailable`.
    pub continuation: String,
    pub no_progress: bool,
    pub design_present: bool,
    /// `routine`, `complex`, `insufficient_evidence`,
    /// `insufficient_after_investigation` or `not_asked`.
    pub design_complexity: String,
    /// `straightforward`, `complex`, `needs_design`, `insufficient_evidence`,
    /// `insufficient_after_investigation` or `not_asked`.
    pub implementation_complexity: String,
    /// `none`, `verified` or `failed`: the last dispatch on this dependency.
    pub verification: String,
    /// The last route dispatched on this dependency, or `none`.
    pub last_route: String,
}

impl Signals {
    pub fn value(&self) -> Value {
        serde_json::to_value(self).expect("signals serialize")
    }
}

/// A judged choice reads as insufficient evidence when its confidence falls
/// under the policy's floor: a confident answer over weak evidence is still
/// only a distribution, and the floor is what the labelled set found.
/// Decides on the mass a side of the answer carries rather than on how
/// concentrated the distribution is. Three implementation judgments over the
/// same design put 0.61, 0.68 and 0.64 on `complex` and never cleared the
/// concentration floor, so the run bought investigations it did not need.
/// When the easy side or the hard side holds at least `floor` of the mass,
/// the answer is that side's argmax; otherwise nothing is decided.
pub fn on_mass(
    probabilities: Option<&Value>,
    easy: &str,
    hard: &[&str],
    floor: f64,
) -> Option<String> {
    let map = probabilities?.as_object()?;
    let mass = |name: &str| map.get(name).and_then(Value::as_f64).unwrap_or(0.0);
    if mass(easy) >= floor {
        return Some(easy.into());
    }
    let hard_mass: f64 = hard.iter().map(|h| mass(h)).sum();
    if hard_mass >= floor {
        return hard
            .iter()
            .max_by(|a, b| mass(a).total_cmp(&mass(b)))
            .map(|h| h.to_string());
    }
    None
}

pub fn thresholded(choice: Option<String>, confidence: Option<f64>, floor: f64) -> String {
    match (choice, confidence) {
        (Some(choice), Some(confidence)) if confidence >= floor => choice,
        (Some(_), _) => "insufficient_evidence".into(),
        (None, _) => "insufficient_evidence".into(),
    }
}

pub fn record(decided: &Decided, signals: &Signals, table: &Table) -> Value {
    json!({
        "signals": signals.value(),
        "route": decided.route,
        "row": decided.row,
        "why": decided.why,
        "policy_version": table.version,
        "allocation": table.allocation(&decided.route),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn quiet() -> Signals {
        Signals {
            hard_limit: false,
            continuation: "justified".into(),
            no_progress: false,
            design_present: false,
            design_complexity: "routine".into(),
            implementation_complexity: "not_asked".into(),
            verification: "none".into(),
            last_route: "none".into(),
        }
    }

    #[test]
    fn the_shipped_policy_parses_and_ends_in_the_humans_row() {
        let table = load();
        assert_eq!(table.version, 1);
        assert_eq!(table.rows.last().unwrap().route, "human");
        assert_eq!(table.allocation("design").unwrap().effort, "medium");
        assert_eq!(
            table.allocation("implement_strong_low").unwrap().effort,
            "low"
        );
        assert!(parse(r#"{"version":1,"set_from":"","tiers":{},"routes":{"x":{"role":"routine","tier":"cheap","effort":"d"}},"signals":[],"no_progress_attempts":2,"min_confidence":0.6,"rows":[{"when":{},"route":"x","why":""}],"decisions":{"note":"","routine":{}}}"#)
            .unwrap_err()
            .contains("human"));
    }

    #[test]
    fn every_human_row_overrides_the_allocation_rows() {
        let table = load();
        assert_eq!(table.route(&quiet().value()).route, "routine");
        let mut s = quiet();
        s.hard_limit = true;
        assert_eq!(table.route(&s.value()).route, "human");
        let mut s = quiet();
        s.continuation = "unavailable".into();
        assert_eq!(table.route(&s.value()).route, "human");
        let mut s = quiet();
        s.no_progress = true;
        assert!(table.route(&s.value()).why.contains("budget remaining"));
    }

    #[test]
    fn a_missing_signal_is_the_humans_by_rule_and_a_low_confidence_reads_insufficient() {
        let table = load();
        let mut v = quiet().value();
        v.as_object_mut().unwrap().remove("verification");
        let decided = table.route(&v);
        assert!(decided.human());
        assert_eq!(decided.row, None);
        assert_eq!(
            thresholded(Some("complex".into()), Some(0.3), 0.6),
            "insufficient_evidence"
        );
        assert_eq!(
            thresholded(Some("complex".into()), Some(0.9), 0.6),
            "complex"
        );
        assert_eq!(thresholded(None, Some(0.9), 0.6), "insufficient_evidence");
    }

    #[test]
    fn routine_decisions_never_lower_a_bar() {
        let table = load();
        assert!(table.routine_decision("unavailable-source", "retry"));
        assert!(!table.routine_decision("data-insufficient", "lower-bar"));
        assert!(!table.routine_decision("manifest-proposed", "admit"));
        for option in ["add-sources", "lower-bar"] {
            assert!(!table.routine_decision("requirements-unmet", option));
        }
    }
}
