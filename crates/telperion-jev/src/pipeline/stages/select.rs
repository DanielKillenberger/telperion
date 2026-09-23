//! Selection fills the packet's closed records from the screened evidence.
//!
//! A measured field is filled by fn-57's selection tool: Jev picks a span
//! among the ones code extracted, code parses the numbers and the unit from
//! that span and copies them into the profile metric. A described trait is
//! scored over the levels a person wrote; its value ships only after the
//! generate stage rendered and measured the candidates. An appearance trait
//! is scored over the requirements table's levels and its ranges are copied
//! (`appearance`), never rendered. Every filled value
//! has a sidecar entry keyed by JSON Pointer; a field with no admissible
//! candidate is recorded as unavailable with the reason, never estimated.

use serde_json::{json, Map, Value};

use crate::pipeline::canon::write_canonical;
use crate::pipeline::decision::append_decisions;
use crate::pipeline::judge::Judge;
use crate::pipeline::manifest::{Described, Field, Manifest};
use crate::pipeline::sets::DescribedLevel;
use crate::pipeline::stage::{Context, Paths, StageError};
use crate::select::select;

use super::appearance::{self, score_levels};
use super::{body, inputs};

pub const STAGE: &str = "select";

#[derive(Debug)]
pub enum Outcome {
    Current,
    Ran { filled: usize, unavailable: usize },
}

pub fn run(paths: &Paths, judge: &Judge<'_>) -> Result<Outcome, StageError> {
    let (ctx, blocked) = Context::open(paths, STAGE)?;
    let (fetch, fetch_sha) = body(&ctx, STAGE, "fetch")?;
    let (screen, screen_sha) = body(&ctx, STAGE, "screen")?;
    let (quality, quality_sha) = body(&ctx, STAGE, "quality")?;
    let mut header = ctx.header(
        STAGE,
        "select",
        inputs(&[
            ("fetch.json", &fetch_sha),
            ("screen.json", &screen_sha),
            ("quality.json", &quality_sha),
        ]),
        vec![],
    );
    if ctx.is_current(STAGE, &header.idempotence_key) {
        return Ok(Outcome::Current);
    }
    let manifest = &ctx.admitted.manifest;
    let mut filled = Map::new();
    let mut sidecar = Map::new();
    let mut unavailable = Map::new();
    for field in &manifest.fields {
        let pointer = format!("/profiles/0/metrics/{}", field.field);
        if blocked.contains(&field.field)
            || quality["fields"][&field.field]["passed"] != json!(true)
        {
            unavailable.insert(field.field.clone(), json!("below the data-quality bar"));
            continue;
        }
        match select_field(judge, field, &screen)? {
            Some((metric, entry)) => {
                header.ledger.extend(
                    entry["ledger"]
                        .as_array()
                        .into_iter()
                        .flatten()
                        .filter_map(|l| l.as_str().map(str::to_string)),
                );
                filled.insert(pointer.clone(), metric);
                sidecar.insert(pointer, entry);
            }
            None => {
                unavailable.insert(field.field.clone(), json!("no admissible candidate span"));
            }
        }
    }
    let mut described = Map::new();
    for trait_ in &manifest.described {
        let (level, entry) = score_described(judge, &ctx, trait_, &fetch)?;
        header
            .ledger
            .push(entry["ledger"].as_str().unwrap_or_default().to_string());
        described.insert(
            trait_.trait_name.clone(),
            json!({"level": level, "sentence": entry["sentence"], "ledger": entry["ledger"]}),
        );
    }
    let copied = appearance::run(judge, &ctx, &fetch)?;
    header.ledger.extend(copied.ledger.iter().cloned());
    sidecar.extend(copied.sidecar.clone());
    write_packet(&ctx, manifest, &filled, &unavailable, &copied.profile)?;
    write_canonical(
        &ctx.paths.sidecar(),
        &json!({"schema": "provenance", "schema_version": 1, "entries": sidecar, "unavailable": unavailable}),
    )?;
    let counts = (filled.len(), unavailable.len());
    let mut out = json!({"filled": filled, "unavailable": unavailable, "described": described});
    if !manifest.appearance.is_empty() {
        out["appearance"] = json!(copied.body);
    }
    if !copied.decisions.is_empty() {
        append_decisions(&ctx.paths.decisions(), copied.decisions)?;
    }
    ctx.write(&header, out)?;
    Ok(Outcome::Ran {
        filled: counts.0,
        unavailable: counts.1,
    })
}

/// The screened measured sentences as the selection document, Jev's chosen
/// span, and the numbers code parsed from it.
fn select_field(
    judge: &Judge<'_>,
    field: &Field,
    screen: &Value,
) -> Result<Option<(Value, Value)>, StageError> {
    let rows: Vec<&Value> = screen["rows"]
        .as_array()
        .into_iter()
        .flatten()
        .filter(|r| r["kind"] == "measured_size_at_age" || r["kind"] == "mature_size_range")
        .collect();
    if rows.is_empty() {
        return Ok(None);
    }
    let document = rows
        .iter()
        .map(|r| r["sentence"].as_str().unwrap_or_default())
        .collect::<Vec<_>>()
        .join("\n");
    let report = select(
        judge.transport,
        judge.key,
        &judge.ledger_dir,
        &document,
        &field.question,
        None,
    )
    .map_err(|err| StageError::Failed {
        stage: STAGE.into(),
        reason: err.to_string(),
    })?;
    if report.contract_failure || report.chosen == "none" || report.chosen.is_empty() {
        return Ok(None);
    }
    let Some((range, unit)) = parse_span(&report.chosen) else {
        return Ok(None);
    };
    let source = rows
        .iter()
        .find(|r| {
            r["sentence"]
                .as_str()
                .is_some_and(|s| s.contains(&report.chosen))
        })
        .and_then(|r| r["source"].as_str())
        .unwrap_or_default()
        .to_string();
    let metric = json!({
        "unit": "m",
        "range": range,
        "classification": "gating",
        "source": [source],
        "confidence": "pipeline",
        "note": report.chosen,
    });
    let entry = json!({
        "route": "copied",
        "source": source,
        "span": report.chosen,
        "unit": unit,
        "ledger": [report.identity],
    });
    Ok(Some((metric, entry)))
}

/// The numbers before a length unit in a span, converted to metres in code:
/// `50 to 90 ft` -> [15.24, 27.432]; `6 m at 20 years` -> [6, 6]. A range is
/// the two numbers joined by `to` or a dash right before the unit.
pub fn parse_span(span: &str) -> Option<([f64; 2], String)> {
    let lower = span.to_ascii_lowercase();
    let words: Vec<(usize, &str)> = lower
        .split(|c: char| !c.is_alphanumeric())
        .filter(|w| !w.is_empty())
        .map(|w| (w.as_ptr() as usize - lower.as_ptr() as usize, w))
        .collect();
    let (at, unit) = words
        .iter()
        .find(|(_, w)| ["m", "ft", "feet", "foot", "cm", "in", "inches"].contains(w))
        .copied()?;
    let factor = match unit {
        "m" => 1.0,
        "ft" | "feet" | "foot" => 0.3048,
        "cm" => 0.01,
        _ => 0.0254,
    };
    // A decimal comma is a decimal point: `7,2 cm` is 0.072 m, as the table parser reads it.
    let prefix = lower[..at].trim_end().replace(',', ".");
    let numbers: Vec<f64> = prefix
        .split(|c: char| !(c.is_ascii_digit() || c == '.'))
        .filter_map(|t| t.parse::<f64>().ok())
        .collect();
    let last = *numbers.last()?;
    let joined = prefix
        .rsplit_once(|c: char| c.is_ascii_digit() || c == '.')
        .map(|(head, _)| head.trim_end_matches(|c: char| c.is_ascii_digit() || c == '.'))
        .is_some_and(|head| {
            let tail = head.trim_end();
            tail.ends_with(" to") || tail.ends_with('-') || tail.ends_with('\u{2013}')
        });
    let first = if joined && numbers.len() >= 2 {
        numbers[numbers.len() - 2]
    } else {
        last
    };
    Some(([first * factor, last * factor], unit.to_string()))
}

/// Jev scores the trait over the manifest's levels on the source sections
/// that carry the trait's terms.
fn score_described(
    judge: &Judge<'_>,
    ctx: &Context,
    trait_: &Described,
    fetch: &Value,
) -> Result<(String, Value), StageError> {
    let levels: Vec<DescribedLevel> = trait_
        .table
        .levels
        .iter()
        .map(|l| DescribedLevel {
            key: l.key.clone(),
            summary: l.summary.clone(),
        })
        .collect();
    score_levels(
        judge,
        ctx,
        &trait_.trait_name,
        &trait_.sources,
        &levels,
        fetch,
    )
}

/// The profile and references records, closed shapes with no added key.
fn write_packet(
    ctx: &Context,
    manifest: &Manifest,
    filled: &Map<String, Value>,
    unavailable: &Map<String, Value>,
    appearance: &Map<String, Value>,
) -> Result<(), StageError> {
    let mut metrics = Map::new();
    for (pointer, metric) in filled {
        let name = pointer.rsplit('/').next().unwrap_or_default();
        metrics.insert(name.to_string(), metric.clone());
    }
    for (field, reason) in unavailable {
        metrics.insert(field.clone(), json!({"unit": "m", "range": null, "classification": "unavailable", "source": [], "confidence": "unavailable", "note": reason}));
    }
    let mut profile = json!({
        "schema_version": 1,
        "provenance": ctx.paths.sidecar().file_name().map(|n| n.to_string_lossy().into_owned()),
        "definitions": {},
        "profiles": [{
            "id": manifest.species,
            "scientific_name": manifest.taxon.scientific_name,
            "common_name": manifest.taxon.common_name,
            "readiness": "draft",
            "context": manifest.context,
            "sources": manifest.sources.iter().map(|s| s.id.clone()).collect::<Vec<_>>(),
            "metrics": metrics,
            "anatomy": {},
            "rubric": {},
        }],
    });
    if !appearance.is_empty() {
        profile["profiles"][0]["appearance"] = json!(appearance);
    }
    write_canonical(&ctx.paths.packet("profile"), &profile)?;
    let sources: Vec<Value> = manifest
        .sources
        .iter()
        .map(|s| json!({"id": s.id, "url": s.url, "attribution": s.title, "verified": "pipeline", "use": s.rights}))
        .collect();
    write_canonical(
        &ctx.paths.packet("references"),
        &json!({"reference_version": "fn19-references-v1", "sources": sources, "references": []}),
    )?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::parse_span;

    #[test]
    fn a_span_yields_its_numbers_in_metres() {
        let cases = [
            ("50 to 90 ft tall", [15.24, 27.432], "ft"),
            ("reaches 6 m at 20 years", [6.0, 6.0], "m"),
            ("24 to 40 in. in DBH", [0.6096, 1.016], "in"),
            ("DG 7,2 cm", [0.072, 0.072], "cm"),
            ("15-27 m tall", [15.0, 27.0], "m"),
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
