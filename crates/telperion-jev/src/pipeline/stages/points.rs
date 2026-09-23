//! The data-quality gate's points (fn-131): the evidence items that are a
//! size at an age for the field, each weighing 1 under the required
//! condition and less where the source leaves the condition unstated, and
//! the required ages they cover.

use serde_json::{json, Value};

use crate::pipeline::manifest::{Field, Manifest};
use crate::pipeline::requirements::names_field;

/// A required age is covered when a matching point lies within this fraction
/// of it, or two matching points bracket it.
const AGE_WINDOW: f64 = 0.25;
/// The weight of a point whose growing condition the source leaves
/// unstated, beside a point under the required condition's 1 (fn-131): an
/// age is covered by points weighing 1 together.
const UNSTATED_WEIGHT: f64 = 0.5;

/// Evidence items for this taxon that are sizes at an age: a measured size
/// at an age, or a sentence stating the age a size is reached at (fn-131),
/// that names the field. A point under the required condition weighs 1, one
/// whose condition is unstated less; any other condition is no point.
pub fn measured_points(manifest: &Manifest, field: &Field, evidence: &[Value]) -> Vec<Value> {
    evidence
        .iter()
        .filter(|item| {
            (item["kind"] == "measured_size_at_age" || item.get("ages_years").is_some())
                && item["taxon"] == manifest.taxon.scientific_name
                && names_field(&field.field, item["sentence"].as_str().unwrap_or_default())
        })
        .filter_map(|item| {
            let weight = if item["condition"] == field.condition {
                1.0
            } else if item["condition"] == "unstated" {
                UNSTATED_WEIGHT
            } else {
                return None;
            };
            let mut point = item.clone();
            point["weight"] = json!(weight);
            Some(point)
        })
        .collect()
}

/// A required age is covered by the points near it weighing 1 together, or
/// by points weighing 1 on each side of it.
pub fn coverage(field: &Field, points: &[Value]) -> (Vec<f64>, Vec<f64>) {
    let aged: Vec<(Vec<f64>, f64)> = points
        .iter()
        .map(|p| {
            let ages = p["ages_years"]
                .as_array()
                .into_iter()
                .flatten()
                .filter_map(Value::as_f64)
                .collect();
            (ages, p["weight"].as_f64().unwrap_or(1.0))
        })
        .collect();
    let weight = |test: &dyn Fn(f64) -> bool| -> f64 {
        aged.iter()
            .filter(|(ages, _)| ages.iter().any(|&a| test(a)))
            .map(|(_, w)| w)
            .sum()
    };
    field
        .required_ages_years
        .iter()
        .copied()
        .partition(|&required| {
            let near = weight(&|a| (a - required).abs() <= AGE_WINDOW * required);
            let below = weight(&|a| a < required);
            let above = weight(&|a| a > required);
            near >= 1.0 || (below >= 1.0 && above >= 1.0)
        })
}
