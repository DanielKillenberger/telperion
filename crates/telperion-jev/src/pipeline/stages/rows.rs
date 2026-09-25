//! The screened rows a field can use (fn-131). Quality counts them and
//! select reads them, so the gate never passes a field on a row select
//! drops: a row's kind is one the requirements table gives the field, and
//! its sentence names the field in whole words. A growth-rate sentence that
//! states a size reached at an age ("reaching 5 m in 15 to 20 years") is a
//! point at that age, whatever rate it also states.

use serde_json::Value;

use crate::pipeline::requirements::{kinds, names_field};
use crate::quantity::{first_length, stated_ages};

/// The ages a row states a size at, parsed by code: empty unless the
/// sentence carries both a length and a stated age.
pub fn stated_at(row: &Value) -> Vec<f64> {
    let sentence = row["sentence"].as_str().unwrap_or_default();
    if first_length(sentence).is_none() {
        return Vec::new();
    }
    stated_ages(sentence)
}

/// Whether `row` counts for `field`.
pub fn usable(field: &str, row: &Value) -> bool {
    let sentence = row["sentence"].as_str().unwrap_or_default();
    if !names_field(field, sentence) {
        return false;
    }
    let kind = row["kind"].as_str().unwrap_or_default();
    kinds(field).contains(&kind) || (kind == "typical_growth_rate" && !stated_at(row).is_empty())
}

/// The screen's rows that count for `field`, in screen order.
pub fn for_field<'a>(field: &str, screen: &'a Value) -> Vec<&'a Value> {
    screen["rows"]
        .as_array()
        .into_iter()
        .flatten()
        .filter(|row| usable(field, row))
        .collect()
}
