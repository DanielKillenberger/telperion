//! Aggregate (fn-157): each field's value is the confident aggregate of
//! everything the documents say about it (`pipeline::agree`), composed in
//! code from the spans the read stage labelled. A span counts toward a field
//! when Jev labelled it that field, of a grown tree, under the field's
//! condition or an unstated one; typical spans make the value, and a record
//! or a single specimen the field's maximum. A span whose neighbouring words
//! name another dimension is set aside. A field no source states typically
//! is `unsourced`: the generator's default stands and Tune sets it from the
//! photographs. Appearance and described traits read every fetched document
//! the trait does not narrow. The profile keeps its shape, so Start, Tune
//! and Gaps read it unchanged.

use serde_json::{json, Map, Value};

use crate::pipeline::agree::{aggregate, round, site, Aggregate, Basis, Reading};
use crate::pipeline::canon::{read_json, write_canonical};
use crate::pipeline::judge::Judge;
use crate::pipeline::manifest::{Described, Field, Manifest};
use crate::pipeline::requirements::{asked, Asked};
use crate::pipeline::sets::DescribedLevel;
use crate::pipeline::stage::{Context, Paths, StageError};
use crate::quantity::first_length;

use super::appearance::{self, score_levels};
use super::read::{beside, dimension};
use super::{body, inputs};

pub const STAGE: &str = "aggregate";
/// Why a field has no value.
pub const NO_TYPICAL: &str = "no document states a typical value of a grown tree";

#[derive(Debug)]
pub enum Outcome {
    Current,
    Ran { filled: usize, unsourced: usize },
}

pub fn run(paths: &Paths, judge: &Judge<'_>) -> Result<Outcome, StageError> {
    let (ctx, _) = Context::open(paths, STAGE)?;
    let (fetch, fetch_sha) = body(&ctx, STAGE, "fetch")?;
    let (read, read_sha) = body(&ctx, STAGE, "read")?;
    let mut header = ctx.header(
        STAGE,
        "aggregate",
        inputs(&[("fetch.json", &fetch_sha), ("read.json", &read_sha)]),
        vec![],
    );
    if ctx.is_current(STAGE, &header.idempotence_key) {
        return Ok(Outcome::Current);
    }
    let manifest = &ctx.admitted.manifest;
    let spans = read["spans"].as_array().cloned().unwrap_or_default();
    let (mut filled, mut sidecar, mut unsourced) = (Map::new(), Map::new(), Map::new());
    let mut metrics = Map::new();
    for field in &manifest.fields {
        let pointer = format!("/profiles/0/metrics/{}", field.field);
        let (readings, beside_aside) = readings(manifest, field, &spans);
        let mut agg = aggregate(&readings);
        agg.set_aside.extend(beside_aside);
        let metric = metric(&agg, unit(manifest, field));
        if agg.value.is_some() {
            filled.insert(pointer.clone(), metric.clone());
            sidecar.insert(pointer, provenance(&agg));
        } else {
            unsourced.insert(field.field.clone(), json!(NO_TYPICAL));
        }
        metrics.insert(field.field.clone(), metric);
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
    write_profile(&ctx, manifest, metrics, &copied.profile)?;
    write_sources(&ctx, manifest)?;
    let mut provenance = json!({"schema": "provenance", "schema_version": 1,
        "entries": sidecar, "unavailable": unsourced});
    if !copied.defaults.is_empty() {
        provenance["defaults"] = json!(copied.defaults);
    }
    write_canonical(&ctx.paths.sidecar(), &provenance)?;
    let counts = (filled.len(), unsourced.len());
    let mut out = json!({"filled": filled, "unavailable": unsourced, "described": described});
    if !manifest.appearance.is_empty() {
        out["appearance"] = json!(copied.body);
    }
    ctx.write(&header, out)?;
    Ok(Outcome::Ran {
        filled: counts.0,
        unsourced: counts.1,
    })
}

/// The field's unit: metres, or metres a year for a growth rate (fn-132).
fn unit(manifest: &Manifest, field: &Field) -> &'static str {
    match asked(manifest, &field.field) {
        Asked::Rate => "m/yr",
        _ => "m",
    }
}

/// The spans labelled `field`, of a grown tree, under its condition or an
/// unstated one, parsed by code; and those whose neighbouring words name
/// another dimension, set aside.
pub fn readings(
    manifest: &Manifest,
    field: &Field,
    spans: &[Value],
) -> (Vec<Reading>, Vec<crate::pipeline::agree::SetAside>) {
    let (mut kept, mut aside) = (Vec::new(), Vec::new());
    for span in spans {
        let text = |k: &str| span[k].as_str().unwrap_or_default();
        let condition = text("condition");
        let counted = text("field") == field.field
            && text("age") == "mature"
            && (condition == field.condition || condition == "unstated");
        let basis = match text("basis") {
            "typical" => Basis::Typical,
            "record" | "specimen" => Basis::Record,
            _ => continue,
        };
        let Some((range, _)) = first_length(text("span")).filter(|_| counted) else {
            continue;
        };
        let source = text("source");
        let url = manifest.source(source).map_or("", |s| s.url.as_str());
        let reading = Reading {
            source: source.into(),
            site: site(url),
            range,
            span: text("span").into(),
            sentence: text("sentence").into(),
            basis,
            ledger: text("ledger").into(),
        };
        let at = span["at"].as_u64().unwrap_or_default() as usize;
        match (
            beside(&reading.sentence, at, &reading.span),
            dimension(&field.field),
        ) {
            (Some(said), Some(wanted)) if said != wanted => {
                aside.push(crate::pipeline::agree::SetAside {
                    reading,
                    reason: format!("the words beside the span give its {said}, not its {wanted}"),
                })
            }
            _ => kept.push(reading),
        }
    }
    (kept, aside)
}

/// The profile metric: the value's range and sources, with the median, the
/// agreement, the maximum and what was set aside beside them.
pub fn metric(agg: &Aggregate, unit: &str) -> Value {
    let mut metric = match (agg.value, agg.range) {
        (Some(value), Some(range)) => json!({
            "unit": unit, "range": range, "classification": "gating",
            "source": agg.sources, "confidence": agg.confidence, "value": value,
            "sources_agreeing": points(agg), "spread_ratio": agg.spread_ratio,
            "note": format!("the median of {} independent sources; the range is their extent",
                            points(agg)),
        }),
        _ => json!({
            "unit": unit, "range": null, "classification": "unsourced", "source": [],
            "confidence": "unsourced", "note": NO_TYPICAL,
        }),
    };
    if let Some(max) = &agg.maximum {
        metric["maximum"] = json!({"value": max.range[1], "source": max.source, "span": max.span});
    }
    if !agg.set_aside.is_empty() {
        let aside: Vec<Value> = agg
            .set_aside
            .iter()
            .map(
                |s| json!({"source": s.reading.source, "span": s.reading.span, "reason": s.reason}),
            )
            .collect();
        metric["set_aside"] = json!(aside);
    }
    metric
}

/// The independent sources behind a value: its sites.
fn points(agg: &Aggregate) -> usize {
    let mut sites: Vec<&str> = agg.contributions.iter().map(|r| r.site.as_str()).collect();
    sites.sort();
    sites.dedup();
    sites.len()
}

/// The provenance entry of an aggregated value: every reading behind it,
/// the maximum and what was set aside, each with its sentence and ledger.
fn provenance(agg: &Aggregate) -> Value {
    let row = |r: &Reading| json!({"source": r.source, "sentence": r.sentence, "span": r.span, "ledger": r.ledger});
    let mut entry = json!({
        "route": "aggregated",
        "value": agg.value.map(round),
        "contributions": agg.contributions.iter().map(row).collect::<Vec<_>>(),
    });
    if let Some(max) = &agg.maximum {
        entry["maximum"] = row(max);
    }
    entry
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
    let sources = appearance::read_from(&ctx.admitted.manifest, &trait_.sources);
    score_levels(judge, ctx, &trait_.trait_name, &sources, &levels, fetch)
}

/// The profile record, a closed shape with no added key at its top.
fn write_profile(
    ctx: &Context,
    manifest: &Manifest,
    metrics: Map<String, Value>,
    appearance: &Map<String, Value>,
) -> Result<(), StageError> {
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
    Ok(())
}

/// The references record's `sources`. A recorded reference photograph and
/// its matched shot are kept byte for byte across a rerun (fn-142).
fn write_sources(ctx: &Context, manifest: &Manifest) -> Result<(), StageError> {
    let sources: Vec<Value> = manifest
        .sources
        .iter()
        .map(|s| json!({"id": s.id, "url": s.url, "attribution": s.title, "verified": "pipeline", "use": s.rights}))
        .collect();
    let path = ctx.paths.packet("references");
    let references = read_json(&path)
        .ok()
        .and_then(|v| v["references"].as_array().cloned())
        .unwrap_or_default();
    write_canonical(
        &path,
        &json!({"reference_version": "fn19-references-v1", "sources": sources, "references": references}),
    )?;
    Ok(())
}
