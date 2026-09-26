//! Curve composition and Chapman-Richards fit, entirely in code (R5).
//!
//! The stage substitutes the age-indexed rows the fetch stage parsed into the
//! composition each dimension's manifest entry names, runs `curve::fit` over
//! them, and files a decision for every gap and every miss. Jev is not called
//! here: the header's ledger is empty and no answer reaches the arithmetic.
//!
//! A dimension whose table the fetch stage did not yield, or which yields
//! fewer than two age-indexed rows, files `missing-curve` and skips that fit;
//! the other dimension still fits and its missing reference values are
//! recorded as unavailable. A field a `data-insufficient` decision blocks is
//! skipped the same way. Every `ToleranceMiss` files `tolerance-miss`, which
//! blocks generation; the stage never accepts one.

use serde_json::{json, Map, Value};

use crate::pipeline::curve::{
    self, BelowFirstRow, Composition, CurveError, FitInput, Row, Table, ToleranceMiss, Unit,
};
use crate::pipeline::decision::{append_decisions, Decision, DecisionParts};
use crate::pipeline::manifest::{CompositionSpec, Manifest};
use crate::pipeline::stage::{Context, Paths, StageError};

use super::{body, inputs};

pub const STAGE: &str = "fit";
/// Centimetres per metre, for the Gould anchor the composition reads in cm.
const CM_PER_M: f64 = 100.0;

#[derive(Debug)]
pub enum Outcome {
    Current,
    /// The manifest names no curve; the stage writes its record and nothing else.
    Skipped,
    Ran {
        decisions: Vec<String>,
    },
}

pub fn run(paths: &Paths) -> Result<Outcome, StageError> {
    let (ctx, blocked) = Context::open(paths, STAGE)?;
    let (fetch, fetch_sha) = body(&ctx, STAGE, "fetch")?;
    let (_, quality_sha) = body(&ctx, STAGE, "quality")?;
    let header = ctx.header(
        STAGE,
        "fit",
        inputs(&[("fetch.json", &fetch_sha), ("quality.json", &quality_sha)]),
        vec![],
    );
    if ctx.is_current(STAGE, &header.idempotence_key) {
        return Ok(Outcome::Current);
    }
    let manifest = &ctx.admitted.manifest;
    let Some(curves) = &manifest.curves else {
        // The record still lands, so the stage is current on the next pass:
        // the palm's conductor reran fit on every step until it did.
        ctx.write(&header, json!({"skipped": "the manifest names no curve"}))?;
        return Ok(Outcome::Skipped);
    };
    let tables = &fetch["tables"];
    let ages = curves.reference_ages_years.to_vec();
    let mut decisions = Vec::new();
    let mut compositions = Map::new();
    let mut points = Map::new();
    let mut skipped = Map::new();
    let mut composed: [Option<Composition>; 2] = [None, None];

    let dimensions = [
        ("height", "height_m", &curves.height),
        ("dbh", "dbh_m", &curves.dbh),
    ];
    for (slot, (key, dimension, spec)) in dimensions.into_iter().enumerate() {
        compositions.insert(
            key.to_string(),
            serde_json::to_value(spec).expect("a composition spec serializes"),
        );
        for id in table_ids(spec) {
            if let Some(table) = tables.get(&id) {
                points.insert(id.to_string(), table["rows"].clone());
            }
        }
        if blocked.contains(&dimension.to_string()) {
            skipped.insert(
                dimension.to_string(),
                json!("an open data-insufficient decision blocks this field"),
            );
            continue;
        }
        match compose(spec, tables) {
            Ok(composition) => composed[slot] = Some(composition),
            Err(reason) => {
                skipped.insert(dimension.to_string(), json!(reason));
                decisions.push(missing_curve(
                    manifest, dimension, &ages, &reason, &fetch_sha,
                ));
            }
        }
    }

    let mut report = None;
    if let Some(height) = composed[0].take() {
        let input = FitInput {
            envelope_height_m: curves.envelope_height_m,
            mature_dbh_m: curves.mature_dbh_m,
            reference_ages_years: curves.reference_ages_years,
            height,
            dbh: composed[1].take().unwrap_or_else(no_table),
            tolerance_percent: curves.tolerance_percent,
        };
        match curve::fit(&input) {
            Ok(fitted) => {
                for miss in &fitted.misses {
                    let sources = table_ids(dimension_spec(&dimensions, &miss.field));
                    decisions.push(tolerance_miss(manifest, miss, &sources, &fetch_sha));
                }
                report = Some(fitted);
            }
            Err(CurveError::TooFewPoints { points: found }) => {
                let reason = format!("the height reference answers {found} of the reference ages");
                skipped.insert("height_m".to_string(), json!(reason));
                decisions.push(missing_curve(
                    manifest, "height_m", &ages, &reason, &fetch_sha,
                ));
            }
            Err(other) => {
                return Err(StageError::Failed {
                    stage: STAGE.into(),
                    reason: other.to_string(),
                })
            }
        }
    }

    let mut out = match report {
        Some(fitted) => serde_json::to_value(&fitted).expect("a fit report serializes"),
        None => json!({}),
    };
    out["points"] = Value::Object(points);
    out["compositions"] = Value::Object(compositions);
    if !skipped.is_empty() {
        out["skipped"] = Value::Object(skipped);
    }
    let ids: Vec<String> = decisions.iter().map(|d| d.id.clone()).collect();
    if !decisions.is_empty() {
        append_decisions(&ctx.paths.decisions(), decisions)?;
    }
    ctx.write(&header, out)?;
    Ok(Outcome::Ran { decisions: ids })
}

/// The admitted table ids a composition reads, in the order it reads them.
fn table_ids(spec: &CompositionSpec) -> Vec<String> {
    match spec {
        CompositionSpec::Table { table, .. } | CompositionSpec::ScaledTable { table, .. } => {
            vec![table.clone()]
        }
        CompositionSpec::GouldIntegration { anchor_table, .. } => vec![anchor_table.clone()],
    }
}

/// The composition behind a dimension name, for the sources of a miss.
fn dimension_spec<'a>(
    dimensions: &[(&str, &str, &'a CompositionSpec); 2],
    field: &str,
) -> &'a CompositionSpec {
    dimensions
        .iter()
        .find(|(_, dimension, _)| *dimension == field)
        .map_or(dimensions[0].2, |(_, _, spec)| *spec)
}

/// The two-column length unit a table states, as the curve module reads it.
fn unit_from(name: &str) -> Option<Unit> {
    match name {
        "m" => Some(Unit::M),
        "cm" => Some(Unit::Cm),
        "ft" => Some(Unit::Ft),
        "in" => Some(Unit::In),
        _ => None,
    }
}

/// The rows the fetch stage parsed for one admitted table id.
fn table_from(tables: &Value, id: &str) -> Result<Table, String> {
    let Some(entry) = tables.get(id) else {
        return Err(format!("the fetch stage yielded no table {id}"));
    };
    let stated = entry["unit"].as_str().unwrap_or_default();
    let unit =
        unit_from(stated).ok_or_else(|| format!("table {id} states the unknown unit {stated}"))?;
    let rows: Vec<Row> = entry["rows"]
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(|row| {
            Some(Row {
                age_years: row["age_years"].as_f64()?,
                value: row["value"].as_f64()?,
            })
        })
        .collect();
    if rows.len() < 2 {
        return Err(format!(
            "table {id} yields {} age-indexed rows, fewer than two",
            rows.len()
        ));
    }
    Ok(Table {
        source: entry["source"].as_str().unwrap_or_default().to_string(),
        unit,
        rows,
    })
}

/// The manifest's composition choice with the fetched rows substituted. The
/// Gould anchor is the anchor table interpolated at the anchor age, in
/// centimetres, which is the unit that composition reads.
fn compose(spec: &CompositionSpec, tables: &Value) -> Result<Composition, String> {
    match spec {
        CompositionSpec::Table {
            table,
            below_first_row,
        } => Ok(Composition::Table {
            table: table_from(tables, table)?,
            below_first_row: *below_first_row,
        }),
        CompositionSpec::ScaledTable {
            table,
            factor,
            below_first_row,
        } => Ok(Composition::ScaledTable {
            table: table_from(tables, table)?,
            factor: *factor,
            below_first_row: *below_first_row,
        }),
        CompositionSpec::GouldIntegration {
            anchor_table,
            anchor_age_years,
            coefficients,
            site_index_m,
            max_age_years,
            below_anchor,
        } => {
            let table = table_from(tables, anchor_table)?;
            let metres = curve::interpolate(&table, *anchor_age_years, *below_anchor)
                .map_err(|err| format!("table {anchor_table} has no anchor value: {err}"))?;
            Ok(Composition::GouldIntegration {
                anchor_age_years: *anchor_age_years,
                anchor_dbh_cm: metres * CM_PER_M,
                coefficients: coefficients.clone(),
                site_index_m: *site_index_m,
                max_age_years: *max_age_years,
                below_anchor: *below_anchor,
            })
        }
    }
}

/// The stand-in for a dimension with no admissible table: every reference age
/// is unavailable, so the other dimension still fits and nothing is invented.
fn no_table() -> Composition {
    Composition::Table {
        table: Table {
            source: String::new(),
            unit: Unit::M,
            rows: Vec::new(),
        },
        below_first_row: BelowFirstRow::Unavailable,
    }
}

fn missing_curve(
    manifest: &Manifest,
    dimension: &str,
    ages: &[f64],
    reason: &str,
    fetch_sha: &str,
) -> Decision {
    Decision::new(
        DecisionParts {
            species: &manifest.species,
            stage: STAGE,
            kind: "missing-curve",
            field: Some(dimension),
            age_years: None,
        },
        &["generate"],
        [("fetch.json".to_string(), fetch_sha.to_string())]
            .into_iter()
            .collect(),
        vec![],
        json!({"dimension": dimension, "ages_with_no_point": ages}),
        &["add-table", "admit-proxy", "reject"],
        &format!("No age curve is composable for this dimension: {reason}."),
    )
}

fn tolerance_miss(
    manifest: &Manifest,
    miss: &ToleranceMiss,
    sources: &[String],
    fetch_sha: &str,
) -> Decision {
    Decision::new(
        DecisionParts {
            species: &manifest.species,
            stage: STAGE,
            kind: "tolerance-miss",
            field: Some(&miss.field),
            age_years: Some(miss.age_years),
        },
        &["generate"],
        [("fetch.json".to_string(), fetch_sha.to_string())]
            .into_iter()
            .collect(),
        vec![],
        json!({
            "field": miss.field,
            "age_years": miss.age_years,
            "measured": miss.measured,
            "reference": miss.reference,
            "error_percent": miss.error_percent,
            "sources": sources,
        }),
        &["accept-composed-reference", "anchor-elsewhere", "reject"],
        "The fitted model and the composed reference differ beyond the manifest's tolerance at this age; the pipeline never accepts a miss.",
    )
}
