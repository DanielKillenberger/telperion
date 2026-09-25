//! One measured field's pick (fn-131). The document is the rows quality
//! counted for the field, each labelled by its source and its place there
//! ("F1.1"); every span code extracts from a row is a candidate keyed to
//! that row ("F1.1: 20 feet"), so a number two sentences share is two
//! candidates and the value is credited to the sentence it was chosen
//! from. A pick below the floor fills nothing once the floor is calibrated
//! (fn-133); until then the most probable span fills the field with its
//! probability recorded, and verify's field-aware check is the guard. Code
//! parses the numbers and the unit from the chosen span with the shared
//! grammar.

use std::collections::BTreeMap;

use serde_json::{json, Value};

use crate::extract::candidate_spans;
use crate::pipeline::floors::selection_floor;
use crate::pipeline::judge::Judge;
use crate::pipeline::manifest::Field;
use crate::pipeline::stage::StageError;
use crate::quantity::first_length;
use crate::select::select_among;

use super::rows::for_field;
use super::select::STAGE;

/// Why a field was left unfilled, as the select body records it.
pub const NO_CANDIDATE: &str = "no admissible candidate span";
pub const BELOW_FLOOR: &str = "pick below the selection floor";
pub const NO_LENGTH: &str = "the chosen span states no length";

/// The range the sources span and the sources behind it.
pub type Spread = ([f64; 2], Vec<String>);

pub enum Pick {
    /// The profile metric, its sidecar entry, the chosen span's
    /// probability, which the select body records and provenance never
    /// carries, and the sources' spread, which a contradicted value keeps
    /// when no source settles it (fn-149).
    Filled(Value, Value, Option<f64>, Option<Spread>),
    Unfilled(&'static str),
}

/// The pick for `field` and the ledger identity of the judgment, if one
/// ran. The metric carries `unit`: metres, or metres a year for a growth
/// rate (fn-132), whose span states a length gained per year.
pub fn pick(
    judge: &Judge<'_>,
    field: &Field,
    unit: &str,
    screen: &Value,
) -> Result<(Pick, Option<String>), StageError> {
    let rows = for_field(&field.field, screen);
    if rows.is_empty() {
        return Ok((Pick::Unfilled(NO_CANDIDATE), None));
    }
    let mut seen: BTreeMap<&str, usize> = BTreeMap::new();
    let mut lines = Vec::new();
    let mut keyed: Vec<(String, &Value, String)> = Vec::new();
    for row in rows {
        let source = row["source"].as_str().unwrap_or_default();
        let place = seen.entry(source).or_default();
        *place += 1;
        let label = format!("{source}.{place}");
        let sentence = row["sentence"].as_str().unwrap_or_default();
        lines.push(format!("{label}: {sentence}"));
        for span in candidate_spans(sentence) {
            keyed.push((format!("{label}: {span}"), row, span));
        }
    }
    let keys: Vec<String> = keyed.iter().map(|(k, _, _)| k.clone()).collect();
    let report = select_among(
        judge.transport,
        judge.key,
        &judge.ledger_dir,
        &lines.join("\n"),
        &field.question,
        keys,
        None,
    )
    .map_err(|err| StageError::Failed {
        stage: STAGE.into(),
        reason: err.to_string(),
    })?;
    let identity = Some(report.identity.clone());
    let Some((_, row, span)) = keyed.iter().find(|(k, _, _)| *k == report.chosen) else {
        return Ok((Pick::Unfilled(NO_CANDIDATE), identity));
    };
    if selection_floor().is_some_and(|floor| report.confidence < floor) {
        return Ok((Pick::Unfilled(BELOW_FLOOR), identity));
    }
    let probability = report.probabilities[&report.chosen].as_f64();
    let Some((range, stated)) = parse_span(span) else {
        return Ok((Pick::Unfilled(NO_LENGTH), identity));
    };
    let source = row["source"].as_str().unwrap_or_default();
    let metric = json!({
        "unit": unit, "range": range, "classification": "gating",
        "source": [source], "confidence": "pipeline", "note": span,
    });
    let entry = json!({
        "route": "copied", "source": source, "sentence": row["sentence"], "span": span,
        "unit": stated, "pick_confidence": report.confidence, "ledger": [report.identity],
    });
    let spread = spread(&keyed, &report.probabilities);
    Ok((Pick::Filled(metric, entry, probability, spread), identity))
}

/// Each source's most probable span, parsed by code, and the range they
/// span. A source counts when its best span has probability and reaches
/// the selection floor once that is calibrated; None when none counts.
fn spread(keyed: &[(String, &Value, String)], probabilities: &Value) -> Option<Spread> {
    let mut best: BTreeMap<&str, (f64, &str)> = BTreeMap::new();
    for (key, row, span) in keyed {
        let p = probabilities[key].as_f64().unwrap_or(0.0);
        let source = row["source"].as_str().unwrap_or_default();
        if p > best.get(source).map_or(0.0, |b| b.0) {
            best.insert(source, (p, span));
        }
    }
    let floor = selection_floor().unwrap_or(0.0);
    let parsed: Vec<(&str, [f64; 2])> = best
        .into_iter()
        .filter(|(_, (p, _))| *p >= floor)
        .filter_map(|(source, (_, span))| Some((source, parse_span(span)?.0)))
        .collect();
    let lo = parsed.iter().map(|(_, r)| r[0]).reduce(f64::min)?;
    let hi = parsed.iter().map(|(_, r)| r[1]).reduce(f64::max)?;
    Some((
        [lo, hi],
        parsed.into_iter().map(|(s, _)| s.to_string()).collect(),
    ))
}

/// The first length in a span, in metres: `50 to 90 ft` -> [15.24, 27.432];
/// `6 m at 20 years` -> [6, 6]; the glued `6–10m` -> [6, 10].
pub fn parse_span(span: &str) -> Option<([f64; 2], String)> {
    first_length(span)
}

#[cfg(test)]
mod tests {
    use super::{parse_span, spread};
    use serde_json::json;

    #[test]
    fn the_spread_takes_each_sources_most_probable_span() {
        let (f1, p5) = (json!({"source": "F1"}), json!({"source": "P5"}));
        let key = |k: &str, row, span: &str| (k.to_string(), row, span.to_string());
        let keyed = [
            key("F1.1: 20 feet", &f1, "20 feet"),
            key("F1.1: 1 to 2 feet", &f1, "1 to 2 feet"),
            key("P5.1: 5-7 m", &p5, "5-7 m"),
            key("P5.1: 0.3-0.6 m", &p5, "0.3-0.6 m"),
        ];
        let probabilities = json!({"F1.1: 20 feet": 0.6, "F1.1: 1 to 2 feet": 0.1,
                                   "P5.1: 5-7 m": 0.3, "P5.1: 0.3-0.6 m": 0.0});
        let (range, sources) = spread(&keyed, &probabilities).unwrap();
        assert_eq!(range, [5.0, 7.0]);
        assert_eq!(sources, ["F1", "P5"]);
        let (f1_only, _) = spread(&keyed[..2], &probabilities).unwrap();
        assert!((f1_only[0] - 6.096).abs() < 1e-9 && (f1_only[1] - 6.096).abs() < 1e-9);
        assert!(spread(&keyed, &json!({})).is_none());
    }

    #[test]
    fn a_span_yields_its_numbers_in_metres() {
        let cases = [
            ("50 to 90 ft tall", [15.24, 27.432], "ft"),
            ("reaches 6 m at 20 years", [6.0, 6.0], "m"),
            ("24 to 40 in. in DBH", [0.6096, 1.016], "in"),
            ("DG 7,2 cm", [0.072, 0.072], "cm"),
            ("15-27 m tall", [15.0, 27.0], "m"),
            ("6\u{2013}10m", [6.0, 10.0], "m"),
        ];
        for (span, range, unit) in cases {
            let (got, u) = parse_span(span).unwrap();
            assert!(
                (got[0] - range[0]).abs() < 1e-9 && (got[1] - range[1]).abs() < 1e-9,
                "{span}: {got:?}"
            );
            assert_eq!(u, unit);
        }
        assert!(parse_span("about 500 years").is_none());
    }
}
