//! Aggregate (fn-157): each field's value is the confident aggregate of
//! everything the documents say about it (`pipeline::agree`), composed in
//! code from the spans the read stage labelled. A span counts toward a field
//! when Jev labelled it that field, of a grown tree, under the field's
//! condition or an unstated one; typical spans make the value, and a record
//! or a single specimen the field's maximum. A span whose neighbouring words
//! name another dimension is set aside. Each document ranks by its kind, and
//! the best tier that agrees decides a value (`pipeline::agree`); the metric
//! names that tier and what every tier held. A field no source states
//! typically is `unsourced`: the generator's default stands and Tune sets it
//! from the photographs. Appearance traits read the documents of the best
//! tier that speaks of them. The profile keeps its shape, so Start, Tune
//! and Gaps read it unchanged.

use std::collections::BTreeMap;

use serde_json::{json, Map, Value};

use crate::pipeline::agree::{aggregate, round, site, Aggregate, Basis, Reading};
use crate::pipeline::canon::{read_json, write_canonical};
use crate::pipeline::judge::Judge;
use crate::pipeline::manifest::{Described, Field, Manifest};
use crate::pipeline::requirements::{asked, Asked};
use crate::pipeline::sets::{tier, DescribedLevel, KINDS};
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
    let kinds = kinds(&read);
    let sites = sites(&ctx.admitted.manifest, &fetch);
    let (mut filled, mut sidecar, mut unsourced) = (Map::new(), Map::new(), Map::new());
    let mut metrics = Map::new();
    for field in &manifest.fields {
        let pointer = format!("/profiles/0/metrics/{}", field.field);
        let (readings, beside_aside) = readings(field, &spans, &kinds, &sites);
        let mut agg = aggregate(&readings);
        agg.set_aside.extend(beside_aside);
        let metric = metric(&agg, unit(manifest, field));
        // A record-only field keeps its maximum's sentence and ledger too.
        if agg.value.is_some() || agg.maximum.is_some() {
            sidecar.insert(pointer.clone(), provenance(&agg));
        }
        match agg.value {
            Some(_) => {
                filled.insert(pointer, metric.clone());
            }
            None => {
                unsourced.insert(field.field.clone(), json!(NO_TYPICAL));
            }
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
    let copied = appearance::run(judge, &ctx, &fetch, &kinds)?;
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

/// Each read document's tier, by its kind (`sets::tier`); a document read
/// never classed ranks last.
pub fn kinds(read: &Value) -> BTreeMap<String, usize> {
    read["documents"]
        .as_object()
        .into_iter()
        .flatten()
        .map(|(id, doc)| (id.clone(), tier(doc["kind"].as_str().unwrap_or_default())))
        .collect()
}

/// Each source's site, from the address its fetch ended at (a DOI resolves
/// to its publisher), else the manifest's.
pub fn sites(manifest: &Manifest, fetch: &Value) -> BTreeMap<String, String> {
    manifest
        .sources
        .iter()
        .map(|s| {
            let url = fetch["sources"][&s.id]["final_url"]
                .as_str()
                .filter(|u| !u.is_empty())
                .unwrap_or(&s.url);
            (s.id.clone(), site(url))
        })
        .collect()
}

/// The spans labelled `field`, of a grown tree, under its condition or an
/// unstated one, parsed by code and ranked by their document's tier; and
/// those whose neighbouring words name another dimension, set aside.
pub fn readings(
    field: &Field,
    spans: &[Value],
    kinds: &BTreeMap<String, usize>,
    sites: &BTreeMap<String, String>,
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
        let reading = Reading {
            source: source.into(),
            site: sites.get(source).cloned().unwrap_or_default(),
            tier: kinds.get(source).copied().unwrap_or(KINDS.len() - 1),
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
            "unit": unit, "range": range, "classification": classification(agg),
            "classified": classified(agg),
            "source": agg.sources, "confidence": agg.confidence, "value": value,
            "sources_agreeing": points(agg), "spread_ratio": agg.spread_ratio,
            "tier": agg.tier.map(|t| KINDS[t]),
            "note": note(points(agg), agg.tier.map_or("", |t| KINDS[t])),
        }),
        _ => json!({
            "unit": unit, "range": null, "classification": "unsourced", "source": [],
            "confidence": "unsourced", "note": NO_TYPICAL,
        }),
    };
    if !agg.held.is_empty() {
        let held: Map<String, Value> = agg
            .held
            .iter()
            .map(|(t, n)| (KINDS[*t].to_string(), json!(n)))
            .collect();
        metric["tiers"] = json!(held);
    }
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

/// A value gates only when its deciding tier agrees (host, 2026-09-26): one
/// source, or sources that disagree, only inform, so Start derives from it,
/// Tune may move it and it never makes a baseline infeasible.
fn classification(agg: &Aggregate) -> &'static str {
    match agg.confidence {
        "agreed" => "gating",
        _ => "contextual",
    }
}

/// Why the value gates or informs, in words.
fn classified(agg: &Aggregate) -> String {
    let (n, kind) = (points(agg), agg.tier.map_or("", |t| KINDS[t]));
    match (agg.confidence, n) {
        ("agreed", _) => format!("gating: {n} independent {kind} sources agree"),
        (_, 1) => format!("contextual: one {kind} source, nothing to agree with"),
        _ => format!("contextual: {n} {kind} sources that do not agree"),
    }
}

/// How the value was composed, in words.
fn note(sources: usize, kind: &str) -> String {
    match sources {
        1 => format!("one {kind} source's value; the range is its extent"),
        n => format!("the median of {n} independent {kind} sources; the range is their extent"),
    }
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

#[cfg(test)]
mod tests {
    use super::sites;
    use serde_json::json;

    /// A paper reached through doi.org counts as its publisher's site, so two
    /// DOIs are two sources and a DOI and its publisher's page are one.
    #[test]
    fn a_source_is_the_site_its_fetch_ended_at() {
        let manifest = serde_json::from_value(json!({
            "schema": "manifest", "schema_version": 1, "species": "s",
            "taxon": {"scientific_name": "T", "common_name": "t", "rank": "species"},
            "context": "c", "growth_form": "broadleaf", "preset": "s", "profile_id": "s", "seed": 1,
            "sources": [
                {"id": "P1", "url": "https://doi.org/10.1/a", "title": "a", "rights": "r"},
                {"id": "P2", "url": "https://doi.org/10.2/b", "title": "b", "rights": "r"},
                {"id": "P3", "url": "https://example.org/x", "title": "x", "rights": "r"}
            ],
            "fields": [], "versions": {"question_sets": {}, "tools": {}}, "model": "m"
        }))
        .unwrap();
        let fetch = json!({"sources": {
            "P1": {"final_url": "https://www.tandfonline.com/doi/full/10.1/a"},
            "P2": {"final_url": "https://link.springer.com/article/10.2/b"}
        }});
        let got = sites(&manifest, &fetch);
        assert_eq!(got["P1"], "tandfonline.com");
        assert_eq!(got["P2"], "springer.com");
        assert_eq!(got["P3"], "example.org");
    }
}
