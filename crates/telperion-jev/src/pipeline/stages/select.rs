//! Selection fills the packet's closed records from the screened evidence.
//!
//! A measured field is filled by fn-57's selection tool over the rows
//! quality counted for it (`pick`, fn-131): Jev picks a span keyed to its
//! sentence and source, code parses the numbers and the unit from that span
//! and copies them into the profile metric. A required field left unfilled,
//! or whose flagged value a resolution dropped (`flagged`), files
//! `requirements-unmet`. A described trait is
//! scored over the levels a person wrote; its value ships only after the
//! generate stage rendered and measured the candidates. An appearance trait
//! is scored over the requirements table's levels and its ranges are copied
//! (`appearance`), never rendered. Every filled value
//! has a sidecar entry keyed by JSON Pointer; a field with no admissible
//! candidate is recorded as unavailable with the reason, never estimated.

use serde_json::{json, Map, Value};

use crate::pipeline::canon::{read_json, write_canonical};
use crate::pipeline::decision::{append_decisions, retire_unfiled};
use crate::pipeline::judge::Judge;
use crate::pipeline::manifest::{Described, Manifest};
use crate::pipeline::requirements::{asked, required_bar, Asked};
use crate::pipeline::sets::DescribedLevel;
use crate::pipeline::stage::{Context, Paths, StageError};

use super::appearance::{self, score_levels};
use super::flagged::{flags, flags_sha256, unmet, Flag, DROP_VALUE, KEEP_RANGE, REPLACE_SOURCE};
use super::pick::{pick, Pick, Spread};
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
    // A value a resolution dropped reruns select: its key covers the drops.
    let flags = flags(&ctx);
    let mut keyed = inputs(&[
        ("fetch.json", &fetch_sha),
        ("screen.json", &screen_sha),
        ("quality.json", &quality_sha),
    ]);
    if !flags.is_empty() {
        keyed.insert("resolutions:verify".into(), flags_sha256(&flags));
    }
    let mut header = ctx.header(STAGE, "select", keyed, vec![]);
    if ctx.is_current(STAGE, &header.idempotence_key) {
        return Ok(Outcome::Current);
    }
    let manifest = &ctx.admitted.manifest;
    let mut filled = Map::new();
    let mut sidecar = Map::new();
    let mut picked_at = Map::new();
    let mut unavailable = Map::new();
    let mut decisions = Vec::new();
    for field in &manifest.fields {
        let pointer = format!("/profiles/0/metrics/{}", field.field);
        let judged = &quality["fields"][&field.field];
        if blocked.contains(&field.field) || judged["passed"] != json!(true) {
            unavailable.insert(field.field.clone(), json!("below the data-quality bar"));
            continue;
        }
        let unit = match asked(manifest, &field.field) {
            Asked::Rate => "m/yr",
            _ => "m",
        };
        let (picked, identity) = pick(judge, field, unit, &screen)?;
        header.ledger.extend(identity);
        let (reason, search) = match picked {
            Pick::Filled(metric, entry, p, spread) => {
                let kept = match flags.get(&pointer).filter(|f| f.names(&entry)) {
                    None => Ok(metric),
                    Some(f) if f.option == KEEP_RANGE => kept_range(metric, spread, f),
                    Some(f) => Err((f.reason(), f.option == REPLACE_SOURCE)),
                };
                match kept {
                    Ok(metric) => {
                        picked_at.insert(field.field.clone(), json!(p));
                        filled.insert(pointer.clone(), metric);
                        sidecar.insert(pointer, entry);
                        continue;
                    }
                    Err(left) => left,
                }
            }
            Pick::Unfilled(reason) => (reason.to_string(), false),
        };
        if search || required_bar(manifest, &field.field).is_some() {
            decisions.push(unmet(&ctx, &field.field, &reason, judged, &header.inputs)?);
        }
        unavailable.insert(field.field.clone(), json!(reason));
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
    let mut copied = appearance::run(judge, &ctx, &fetch)?;
    appearance::drop_flagged(&ctx, &mut copied, &flags)?;
    header.ledger.extend(copied.ledger.iter().cloned());
    sidecar.extend(copied.sidecar.clone());
    write_packet(&ctx, manifest, &filled, &unavailable, &copied.profile)?;
    let mut provenance = json!({"schema": "provenance", "schema_version": 1, "entries": sidecar, "unavailable": unavailable});
    if !copied.defaults.is_empty() {
        provenance["defaults"] = json!(copied.defaults);
    }
    write_canonical(&ctx.paths.sidecar(), &provenance)?;
    let counts = (filled.len(), unavailable.len());
    let mut out = json!({"filled": filled, "unavailable": unavailable, "described": described,
                         "pick_probability": picked_at});
    if !manifest.appearance.is_empty() {
        out["appearance"] = json!(copied.body);
    }
    decisions.extend(copied.decisions);
    let ids: Vec<String> = decisions.iter().map(|d| d.id.clone()).collect();
    if !decisions.is_empty() {
        append_decisions(&ctx.paths.decisions(), decisions)?;
    }
    // A field or trait this rerun filled files nothing: its earlier decision is stale.
    retire_unfiled(
        &ctx.paths.decisions(),
        STAGE,
        &ids,
        &header.inputs,
        &crate::pipeline::stage::now(),
    )?;
    ctx.write(&header, out)?;
    Ok(Outcome::Ran {
        filled: counts.0,
        unavailable: counts.1,
    })
}

/// A contradicted value kept as the range its sources span, every source
/// cited (fn-149); with no spread, the value leaves as flagged.
fn kept_range(
    mut metric: Value,
    spread: Option<Spread>,
    flag: &Flag,
) -> Result<Value, (String, bool)> {
    let (range, sources) = spread.ok_or((flag.reason(), false))?;
    metric["range"] = json!(range);
    metric["source"] = json!(sources);
    metric["confidence"] = json!("spanned");
    metric["note"] = json!(format!("{}: the range the sources span", flag.reason()));
    Ok(metric)
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
        // A dropped value leaves its field unsourced: the generator's
        // default stands and Tune sets it from the photographs (fn-149).
        let dropped = reason.as_str().is_some_and(|r| r.starts_with(DROP_VALUE));
        let class = if dropped { "unsourced" } else { "unavailable" };
        metrics.insert(field.clone(), json!({"unit": "m", "range": null, "classification": class, "source": [], "confidence": class, "note": reason}));
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
    // Select owns only `sources`; a recorded reference photograph and its
    // matched shot are kept byte for byte across a rerun (fn-142).
    let references_path = ctx.paths.packet("references");
    let references = read_json(&references_path)
        .ok()
        .and_then(|v| v["references"].as_array().cloned())
        .unwrap_or_default();
    write_canonical(
        &references_path,
        &json!({"reference_version": "fn19-references-v1", "sources": sources, "references": references}),
    )?;
    Ok(())
}
