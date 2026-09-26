//! The search rounds a requirement gets, and the query each one asks.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::pipeline::admission::Checked;
use crate::pipeline::canon::{read_json, write_canonical, CanonError};
use crate::pipeline::consume::REQUIREMENTS_UNMET;
use crate::pipeline::decision::{Decision, Status};
use crate::pipeline::stage::Paths;
use crate::pipeline::stages::discover::{plain_query, stem};

/// Rounds a field or trait gets before the owner has it.
pub const MAX_ROUNDS: usize = 2;

/// One round for one field: the query, what it found, what was tried and
/// what was admitted, or the error that stopped it.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Round {
    pub round: usize,
    pub decision: String,
    pub gap: String,
    pub query: String,
    pub hits: Vec<Value>,
    pub tried: Vec<Checked>,
    pub admitted: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    pub ledger: Vec<String>,
    pub at: String,
}

pub type Rounds = BTreeMap<String, Vec<Round>>;

pub fn read_rounds(paths: &Paths) -> Result<Rounds, CanonError> {
    let path = paths.search_rounds();
    if !path.exists() {
        return Ok(Rounds::new());
    }
    let value = read_json(&path)?;
    serde_json::from_value(value["fields"].clone()).map_err(|err| CanonError::Json {
        path,
        error: err.to_string(),
    })
}

pub(super) fn write_rounds(paths: &Paths, rounds: &Rounds) -> Result<(), CanonError> {
    let value = json!({"schema": "search-rounds", "schema_version": 1, "fields": rounds});
    write_canonical(&paths.search_rounds(), &value).map(|_| ())
}

/// The open requirements-unmet decisions the pipeline still searches for:
/// filed by `quality` for a field, or by `select` for an appearance trait or
/// a required field it could not fill (fn-131),
/// with a round left. An unreadable rounds file leaves nothing to search, so
/// the owner has every decision.
pub fn searchable(paths: &Paths, decisions: &[Decision]) -> Vec<String> {
    let Ok(rounds) = read_rounds(paths) else {
        return Vec::new();
    };
    decisions
        .iter()
        .filter(|d| d.kind == REQUIREMENTS_UNMET && is_searched(&d.stage))
        .filter(|d| d.status == Status::Open)
        .filter(|d| {
            let field = d.field.as_deref().unwrap_or_default();
            rounds.get(field).map_or(0, Vec::len) < MAX_ROUNDS
        })
        .map(|d| d.id.clone())
        .collect()
}

/// The stages whose requirements-unmet decisions are searched again: a
/// field's from `quality`, an appearance trait's or an unfilled field's
/// from `select`.
fn is_searched(stage: &str) -> bool {
    stage == "quality" || stage == "select"
}

/// Every URL the rounds for `field` tried, in order: what the owner is
/// handed when the rounds run out.
pub fn tried(rounds: &Rounds, field: &str) -> Vec<String> {
    rounds
        .get(field)
        .into_iter()
        .flatten()
        .flat_map(|round| round.tried.iter().map(|c| c.url.clone()))
        .collect()
}

/// The query aimed at the field's dominant gap, in plain words; an
/// appearance trait's gap is `unstated`, and its query is the trait.
pub fn gap_query(taxon: &str, field: &str, condition: &str, gap: &str, ages: &[f64]) -> String {
    let (what, grown) = (stem(field), condition.replace('_', " "));
    match gap {
        "no_age_indexed_points" => format!("{taxon} {what} at stated ages in years, {grown}"),
        "age_range_uncovered" => {
            let ages: Vec<String> = ages.iter().map(f64::to_string).collect();
            format!("{taxon} {what} at {} years, {grown}", ages.join(", "))
        }
        "wrong_condition" => format!("{taxon} {what} by age of {grown} trees"),
        "wrong_taxon" => format!("\"{taxon}\" {what} at age, {grown}"),
        "no_mature_size" | "single_source" => format!("{taxon} mature {what}, typical range"),
        "bound_only" => format!("{taxon} typical mature {what}, not the maximum"),
        "no_growth_rate" => format!("{taxon} {what}, cm or ft"),
        "unstated" => format!("{taxon} {what}"),
        _ => plain_query(taxon, field, condition),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_query_aims_at_the_dominant_gap() {
        let q = |gap: &str, ages: &[f64]| {
            gap_query("Phoenix dactylifera", "height_m", "open_grown", gap, ages)
        };
        let table = [
            (
                "no_age_indexed_points",
                "Phoenix dactylifera height at stated ages in years, open grown",
            ),
            (
                "wrong_condition",
                "Phoenix dactylifera height by age of open grown trees",
            ),
            (
                "wrong_taxon",
                "\"Phoenix dactylifera\" height at age, open grown",
            ),
            ("none", "Phoenix dactylifera height at age, open grown"),
        ];
        for (gap, expect) in table {
            assert_eq!(q(gap, &[]), expect, "{gap}");
        }
        assert_eq!(
            q("age_range_uncovered", &[10.0, 25.0]),
            "Phoenix dactylifera height at 10, 25 years, open grown"
        );
        assert_eq!(
            gap_query(
                "Phoenix dactylifera",
                "frond_length_m",
                "open_grown",
                "bound_only",
                &[]
            ),
            "Phoenix dactylifera typical mature frond length, not the maximum"
        );
        assert_eq!(
            gap_query(
                "Phoenix dactylifera",
                "height_growth_m_per_year",
                "open_grown",
                "no_growth_rate",
                &[]
            ),
            "Phoenix dactylifera height growth rate per year, cm or ft"
        );
        assert_eq!(
            gap_query("Phoenix dactylifera", "bark_colour", "", "unstated", &[]),
            "Phoenix dactylifera bark colour"
        );
    }
}
