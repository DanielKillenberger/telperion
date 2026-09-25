//! The profile-to-preset derivation (fn-135): the sourced profile's
//! appearance and sizes become the wire values a species' first tuned tree
//! starts from. `data/profile-to-preset.json` says which metric reaches which
//! path and by which formula; code owns every calculation, clamps each value
//! to its dial in `data/dials.json` and records where it came from. The table
//! names no species: a row's condition reads the family's own values.
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::tuning::actions::Dial;

pub const TABLE_JSON: &str = include_str!("../../data/profile-to-preset.json");
const DIALS_JSON: &str = include_str!("../../data/dials.json");

/// The metrics `species_measure` writes a reading a gate can pass on: a
/// `measured` or `measured_proxy` status at least for some families. Its
/// other keys are always `estimated` (branch counts, lengths, orders and
/// insertions) or not metrics at all, and every other key is never written.
pub const MEASURED: &[&str] = &[
    "nodes",
    "wood_height_m",
    "height_m",
    "crown_width_m",
    "crown_span_x_m",
    "crown_span_z_m",
    "crown_base_m",
    "crown_width_height_ratio",
    "dbh_m",
    "stored_branch_runs",
    "twig_count",
    "units_per_instance",
    "pre_cull_instances",
    "retained_instances",
    "foliage_units",
    "discarded_units",
    "leaf_area_m2",
    "foliage_length_m",
    "foliage_width_m",
    "needle_surface_area_m2",
    "projected_area_m2",
];

/// One derived wire value with the profile entry, citations and formula
/// behind it. `clamped_from` holds the value before its dial's range held it.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DerivedRow {
    pub path: String,
    pub value: f64,
    pub source: String,
    pub sources: Vec<String>,
    pub formula: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub clamped_from: Option<f64>,
}

/// A profile entry the derivation left out, and why.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Skipped {
    pub source: String,
    pub reason: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct Derived {
    pub rows: Vec<DerivedRow>,
    pub skipped: Vec<Skipped>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Table {
    #[allow(dead_code)]
    schema_version: u32,
    #[allow(dead_code)]
    meaning: String,
    rows: Vec<Row>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Row {
    metric: String,
    path: String,
    formula: Formula,
    #[serde(default = "one")]
    factor: f64,
    #[serde(default)]
    over: Option<Over>,
    #[serde(default)]
    when: Option<When>,
    #[allow(dead_code)]
    basis: String,
}

fn one() -> f64 {
    1.0
}

#[derive(Deserialize, Clone, Copy)]
#[serde(rename_all = "snake_case")]
enum Formula {
    Midpoint,
    Identity,
    Ratio,
}

impl Formula {
    fn name(self) -> &'static str {
        match self {
            Self::Midpoint => "midpoint",
            Self::Identity => "identity",
            Self::Ratio => "ratio",
        }
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
enum Over {
    Metric(String),
    Family(String),
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct When {
    path: String,
    #[serde(default)]
    above: Option<f64>,
    #[serde(default)]
    at_most: Option<f64>,
}

impl When {
    fn holds(&self, family: &Value) -> bool {
        family
            .pointer(&self.path)
            .and_then(Value::as_f64)
            .is_some_and(|v| {
                self.above.is_none_or(|a| v > a) && self.at_most.is_none_or(|b| v <= b)
            })
    }
}

fn parse<T: for<'a> Deserialize<'a>>(json: &str, what: &str) -> Result<T, String> {
    serde_json::from_str(json).map_err(|err| format!("{what}: {err}"))
}

fn range(value: &Value) -> Option<(f64, f64)> {
    let pair = value.as_array().filter(|a| a.len() == 2)?;
    Some((pair[0].as_f64()?, pair[1].as_f64()?)).filter(|(lo, hi)| lo <= hi)
}

fn midpoint(value: &Value) -> Option<f64> {
    range(value).map(|(lo, hi)| (lo + hi) / 2.0)
}

fn strings(value: &Value) -> Vec<String> {
    value
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(|s| s.as_str().map(str::to_string))
        .collect()
}

/// `bark_red` -> `barkRed`: a `MaterialParams` field to its wire key.
fn camel(field: &str) -> String {
    let mut parts = field.split('_');
    let head = parts.next().unwrap_or_default().to_string();
    parts.fold(head, |mut out, part| {
        let mut chars = part.chars();
        if let Some(first) = chars.next() {
            out.extend(first.to_uppercase());
            out.push_str(chars.as_str());
        }
        out
    })
}

struct Builder {
    dials: Vec<Dial>,
    out: Derived,
}

impl Builder {
    fn skip(&mut self, source: &str, reason: String) {
        self.out.skipped.push(Skipped {
            source: source.into(),
            reason,
        });
    }

    /// Clamps to the dial's range and records the value; a path with no
    /// dial or one already derived is left out and recorded.
    fn push(&mut self, path: &str, value: f64, source: &str, sources: Vec<String>, formula: &str) {
        if let Some(prior) = self.out.rows.iter().find(|r| r.path == path) {
            let reason = format!("{path} is already derived from {}", prior.source);
            return self.skip(source, reason);
        }
        let Some(dial) = self.dials.iter().find(|d| d.path == path) else {
            return self.skip(source, format!("no dial bounds {path}"));
        };
        let mut held = value.clamp(dial.min, dial.max);
        if dial.integer {
            held = held.round();
        }
        self.out.rows.push(DerivedRow {
            path: path.into(),
            value: held,
            source: source.into(),
            sources,
            formula: formula.into(),
            clamped_from: (held != value).then_some(value),
        });
    }
}

/// Derives the wire values `profile` (one entry of a profile manifest) sets
/// on `family` (its wire metadata). Appearance first: every range keyed by a
/// material field sets that field to its middle. Then each metric through
/// the table rows whose condition the family meets.
pub fn derive(profile: &Value, family: &Value) -> Result<Derived, String> {
    let table: Table = parse(TABLE_JSON, "profile-to-preset table")?;
    let mut b = Builder {
        dials: parse(DIALS_JSON, "dial table")?,
        out: Derived::default(),
    };
    for (entry, a) in profile["appearance"].as_object().into_iter().flatten() {
        for (field, span) in a["ranges"].as_object().into_iter().flatten() {
            let source = format!("appearance.{entry}.{field}");
            let path = format!("/material/{}", camel(field));
            match midpoint(span) {
                Some(v) => b.push(&path, v, &source, strings(&a["sources"]), "midpoint"),
                None => b.skip(&source, "no range".into()),
            }
        }
    }
    let metrics = &profile["metrics"];
    for (metric, m) in metrics.as_object().into_iter().flatten() {
        let source = format!("metrics.{metric}");
        let rows: Vec<&Row> = table.rows.iter().filter(|r| r.metric == *metric).collect();
        if rows.is_empty() {
            b.skip(&source, "no row in the table".into());
            continue;
        }
        let mut applied = false;
        for row in rows
            .iter()
            .filter(|r| r.when.as_ref().is_none_or(|w| w.holds(family)))
        {
            applied = true;
            match value(row, m, metrics, family) {
                Ok(v) => b.push(
                    &row.path,
                    v,
                    &source,
                    strings(&m["source"]),
                    row.formula.name(),
                ),
                Err(reason) => b.skip(&source, reason),
            }
        }
        if !applied {
            let paths: Vec<&str> = rows.iter().map(|r| r.path.as_str()).collect();
            let reason = format!("the family meets no condition of {}", paths.join(", "));
            b.skip(&source, reason);
        }
    }
    Ok(b.out)
}

fn value(row: &Row, metric: &Value, metrics: &Value, family: &Value) -> Result<f64, String> {
    let middle = || midpoint(&metric["range"]).ok_or_else(|| "no range".to_string());
    match row.formula {
        Formula::Midpoint => middle(),
        Formula::Identity => match range(&metric["range"]) {
            Some((lo, hi)) if lo == hi => Ok(lo),
            _ => Err("identity needs one stated value".into()),
        },
        Formula::Ratio => {
            let (over, denominator) = match &row.over {
                Some(Over::Metric(key)) => (key, midpoint(&metrics[key]["range"])),
                Some(Over::Family(path)) => (path, family.pointer(path).and_then(Value::as_f64)),
                None => return Err("a ratio row names nothing to divide by".into()),
            };
            match denominator.filter(|d| *d > 0.0) {
                Some(d) => Ok(middle()? * row.factor / d),
                None => Err(format!("no positive {over} to divide by")),
            }
        }
    }
}

/// Classifies every gating metric the measurer cannot read as contextual,
/// so it reports and gates nothing. Returns the metrics it changed.
pub fn measurable(profile: &mut Value) -> Vec<String> {
    let mut changed = Vec::new();
    let Some(metrics) = profile["metrics"].as_object_mut() else {
        return changed;
    };
    for (key, metric) in metrics.iter_mut() {
        if metric["classification"] == "gating" && !MEASURED.contains(&key.as_str()) {
            metric["classification"] = "contextual".into();
            changed.push(key.clone());
        }
    }
    changed
}
