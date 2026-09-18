//! The route is a table, not a judgment (R3).
//!
//! `data/gap-routes.json` is one versioned table a person reads and changes.
//! Its rows are tried in order over the recorded signals; the first whose
//! conditions all hold names the route. A condition is an equality on a
//! signal, or `spread_below`, a bound the spread must fall under. The five
//! owner signals sit first, so they override the spread. A signal the table
//! names that the record lacks routes to the owner, never to a default row.
//! Moving a threshold re-routes recorded runs without a new call.

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

pub const ROUTES_JSON: &str = include_str!("../../../data/gap-routes.json");

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Route {
    Proceed,
    Stronger,
    Owner,
}

impl Route {
    pub fn key(self) -> &'static str {
        match self {
            Self::Proceed => "proceed",
            Self::Stronger => "stronger",
            Self::Owner => "owner",
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Row {
    pub when: Map<String, Value>,
    pub route: Route,
    pub why: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Table {
    pub version: u32,
    pub set_from: String,
    /// The value rounds a verdict allows before a gap or a hand-off (fn-62).
    pub rounds_per_verdict: usize,
    /// Every signal a route record must carry.
    pub signals: Vec<String>,
    pub rows: Vec<Row>,
}

/// The table shipped with the crate.
pub fn load() -> Table {
    parse(ROUTES_JSON).expect("gap-routes.json parses")
}

pub fn parse(raw: &str) -> Result<Table, String> {
    let table: Table = serde_json::from_str(raw).map_err(|err| err.to_string())?;
    if table.rows.is_empty() {
        return Err("the route table has no rows".into());
    }
    if !table.rows.last().is_some_and(|row| row.when.is_empty()) {
        return Err("the route table's last row must match everything".into());
    }
    for row in &table.rows {
        for key in row.when.keys() {
            let name = key
                .strip_prefix("spread_below")
                .map_or(key.as_str(), |_| "spread");
            if !table.signals.contains(&name.to_string()) {
                return Err(format!("row condition {key} names no listed signal"));
            }
        }
    }
    Ok(table)
}

/// What the table decided: the route, the row that matched and its reason.
#[derive(Debug, Clone, PartialEq)]
pub struct Decided {
    pub route: Route,
    /// `None` when a signal was missing and the owner took it by rule.
    pub row: Option<usize>,
    pub why: String,
}

impl Table {
    /// Looks the route up over `signals`. A missing signal is the owner's.
    pub fn route(&self, signals: &Value) -> Decided {
        let missing: Vec<&str> = self
            .signals
            .iter()
            .map(String::as_str)
            .filter(|name| signals.get(name).is_none_or(Value::is_null))
            .collect();
        if !missing.is_empty() {
            return Decided {
                route: Route::Owner,
                row: None,
                why: format!("signal missing from the record: {}", missing.join(", ")),
            };
        }
        for (index, row) in self.rows.iter().enumerate() {
            if row
                .when
                .iter()
                .all(|(key, expected)| holds(signals, key, expected))
            {
                return Decided {
                    route: row.route,
                    row: Some(index),
                    why: row.why.clone(),
                };
            }
        }
        unreachable!("the last row matches everything")
    }
}

fn holds(signals: &Value, key: &str, expected: &Value) -> bool {
    if key == "spread_below" {
        return match (signals["spread"].as_f64(), expected.as_f64()) {
            (Some(spread), Some(bound)) => spread < bound,
            _ => false,
        };
    }
    signals.get(key) == Some(expected)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn signals() -> Value {
        json!({
            "spread": 0.6, "best_match": "a", "generator_touch": true, "moves_pin": false,
            "reversible": true, "prior_verdict": "none", "changes_preset_output": false,
            "spends_captures": false, "lowers_bar": false, "changes_boundary": false,
        })
    }

    #[test]
    fn the_shipped_table_parses_and_ends_in_a_catch_all_row() {
        let table = load();
        assert_eq!(table.version, 1);
        assert_eq!(table.rounds_per_verdict, 2);
        assert!(table.rows.last().unwrap().when.is_empty());
        assert!(parse(r#"{"version":1,"set_from":"","rounds_per_verdict":2,"signals":["spread"],"rows":[{"when":{"spread_below":0.1},"route":"owner","why":""}]}"#).is_err());
        assert!(parse(r#"{"version":1,"set_from":"","rounds_per_verdict":2,"signals":[],"rows":[{"when":{"x":1},"route":"owner","why":""},{"when":{},"route":"proceed","why":""}]}"#)
            .unwrap_err()
            .contains("names no listed signal"));
    }

    #[test]
    fn a_clear_winner_proceeds_and_each_owner_signal_overrides_it() {
        let table = load();
        assert_eq!(table.route(&signals()).route, Route::Proceed);
        for flag in [
            "moves_pin",
            "changes_preset_output",
            "spends_captures",
            "lowers_bar",
            "changes_boundary",
        ] {
            let mut s = signals();
            s[flag] = json!(true);
            assert_eq!(table.route(&s).route, Route::Owner, "{flag}");
        }
        let mut s = signals();
        s["prior_verdict"] = json!("against");
        assert_eq!(table.route(&s).route, Route::Owner);
    }

    #[test]
    fn a_close_spread_or_no_match_goes_to_the_stronger_model() {
        let table = load();
        let mut s = signals();
        s["spread"] = json!(0.05);
        let decided = table.route(&s);
        assert_eq!(decided.route, Route::Stronger);
        assert!(decided.why.contains("close"), "{}", decided.why);
        let mut s = signals();
        s["best_match"] = json!("none");
        assert_eq!(table.route(&s).route, Route::Stronger);
        // A verdict already for it proceeds even on a close spread.
        let mut s = signals();
        s["spread"] = json!(0.05);
        s["prior_verdict"] = json!("for");
        assert_eq!(table.route(&s).route, Route::Proceed);
    }

    #[test]
    fn a_missing_signal_routes_to_the_owner_by_rule() {
        let table = load();
        let mut s = signals();
        s.as_object_mut().unwrap().remove("reversible");
        let decided = table.route(&s);
        assert_eq!(decided.route, Route::Owner);
        assert_eq!(decided.row, None);
        assert!(decided.why.contains("reversible"));
    }
}
