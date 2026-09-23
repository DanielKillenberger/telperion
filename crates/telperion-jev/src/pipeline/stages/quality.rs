//! The data-quality gate, after the screen and before select, fit and
//! generation. For every required evidence field, code lays out the screened
//! candidates and the admitted table rows beside the requirement, counts the
//! measured points and the required ages they cover, and Jev scores the
//! sufficiency level and names the dominant gap. A field below the manifest's
//! bar files a data-insufficient decision; the stop is code on the level. A
//! field the requirements table requires, below the table's bar, files a
//! requirements-unmet decision instead, with no bar to lower: the pipeline
//! searches again for it twice (`pipeline::search`), then it is the owner's.
//! A field the table marks `mature` (fn-127) asks no age: code lays out the
//! sentences that name it and the mature-size set scores a stated mature
//! value or range on the same four levels.

use serde_json::{json, Map, Value};

use crate::pipeline::consume::{sources_sha256, REQUIREMENTS_UNMET};
use crate::pipeline::decision::{append_decisions, retire_unfiled, Decision, DecisionParts};
use crate::pipeline::judge::Judge;
use crate::pipeline::manifest::{Field, Manifest, Sufficiency};
use crate::pipeline::requirements::{is_mature, required_bar, terms};
use crate::pipeline::sets::{
    level_from_score, mature_questions, sufficiency_questions, SUFFICIENCY_LEVELS,
};
use crate::pipeline::stage::{Context, Paths, StageError};

use super::{body, inputs};

pub const STAGE: &str = "quality";
/// A required age is covered when a matching point lies within this fraction
/// of it, or two matching points bracket it.
const AGE_WINDOW: f64 = 0.25;

#[derive(Debug)]
pub enum Outcome {
    Current,
    Ran { decisions: Vec<String> },
}

pub fn run(paths: &Paths, judge: &Judge<'_>) -> Result<Outcome, StageError> {
    let (ctx, _) = Context::open(paths, STAGE)?;
    let (fetch, fetch_sha) = body(&ctx, STAGE, "fetch")?;
    let (screen, screen_sha) = body(&ctx, STAGE, "screen")?;
    let mut header = ctx.header(
        STAGE,
        "quality",
        inputs(&[("fetch.json", &fetch_sha), ("screen.json", &screen_sha)]),
        vec![],
    );
    if ctx.is_current(STAGE, &header.idempotence_key) {
        return Ok(Outcome::Current);
    }
    let manifest = &ctx.admitted.manifest;
    let mut fields = Map::new();
    let mut decisions = Vec::new();
    for field in &manifest.fields {
        let asked = if is_mature(manifest, &field.field) {
            mature(manifest, field, &screen)
        } else {
            at_age(manifest, field, &screen, &fetch)
        };
        let (points, covered, uncovered) = (asked.points, asked.covered, asked.uncovered);
        let (score_key, gap_key) = asked.keys;
        let judgment = judge
            .ask(score_key, None, &asked.state, &asked.questions)
            .map_err(|err| StageError::Failed {
                stage: STAGE.into(),
                reason: err.to_string(),
            })?;
        let score = judgment.entry.score(score_key).unwrap_or(0.0);
        // No score is the lowest level: the gate fails closed.
        let level =
            Sufficiency::from_index(level_from_score(score, SUFFICIENCY_LEVELS.len()).unwrap_or(0));
        let gap = judgment
            .entry
            .choice(gap_key)
            .unwrap_or_else(|| "none".into());
        let passed = level >= field.bar;
        let required = required_bar(manifest, &field.field).filter(|bar| level < *bar);
        header.ledger.push(judgment.reference.clone());
        fields.insert(
            field.field.clone(),
            json!({
                "level": level.key(),
                "dominant_gap": gap,
                "points": points,
                "required_ages_covered": covered,
                "required_ages_uncovered": uncovered,
                "bar": field.bar.key(),
                "passed": passed,
                "ledger": judgment.reference,
            }),
        );
        let shortfall = Shortfall {
            field,
            level,
            gap: &gap,
            points: &points,
            uncovered: &uncovered,
            ledger: &judgment.reference,
            fetch_sha: &fetch_sha,
        };
        if let Some(bar) = required {
            let sources = sources_sha256(&ctx.paths.manifest())?;
            decisions.push(unmet(manifest, &shortfall, bar, &sources));
        } else if !passed {
            decisions.push(insufficient(manifest, &shortfall));
        }
    }
    let ids: Vec<String> = decisions.iter().map(|d| d.id.clone()).collect();
    if !decisions.is_empty() {
        append_decisions(&ctx.paths.decisions(), decisions)?;
    }
    // A field this rerun passed files nothing: its earlier decision is stale.
    retire_unfiled(
        &ctx.paths.decisions(),
        STAGE,
        &ids,
        &header.inputs,
        &crate::pipeline::gap::now(),
    )?;
    ctx.write(&header, json!({"fields": fields}))?;
    Ok(Outcome::Ran { decisions: ids })
}

/// One field's question: the state code laid out, the set and its answer
/// keys, and the points and age coverage the stage records.
struct Asked {
    state: Value,
    questions: Value,
    /// The Score's key, which also names the tool in the ledger, and the gap Choice's.
    keys: (&'static str, &'static str),
    points: Vec<Value>,
    covered: Vec<f64>,
    uncovered: Vec<f64>,
}

/// An age-indexed field: the measured points at each required age.
fn at_age(manifest: &Manifest, field: &Field, screen: &Value, fetch: &Value) -> Asked {
    let evidence = evidence_for(manifest, field, screen, fetch);
    let points = measured_points(manifest, field, &evidence);
    let (covered, uncovered) = coverage(field, &points);
    let state = json!({
        "requirement": {
            "taxon": manifest.taxon.scientific_name,
            "field": field.field,
            "condition": field.condition,
            "required_ages_years": field.required_ages_years,
        },
        "evidence": evidence,
        "counts": {
            "measured_points": points.len(),
            "required_ages_covered": covered,
            "required_ages_uncovered": uncovered,
        },
    });
    Asked {
        state,
        questions: sufficiency_questions(),
        keys: ("sufficiency", "dominant_gap"),
        points,
        covered,
        uncovered,
    }
}

/// A mature field: every screened sentence that names it, whatever the
/// screen called its kind, since an organ size is no tree size at an age.
/// The points are those sentences; no age is asked or covered.
fn mature(manifest: &Manifest, field: &Field, screen: &Value) -> Asked {
    let evidence: Vec<Value> = screen["rows"]
        .as_array()
        .into_iter()
        .flatten()
        .filter(|row| names_field(field, row["sentence"].as_str().unwrap_or_default()))
        .map(|row| {
            json!({
                "source": row["source"],
                "sentence": row["sentence"],
                "condition": row["condition"],
                "taxon": taxon_of(manifest, field, row),
            })
        })
        .collect();
    let sources: std::collections::BTreeSet<&str> = evidence
        .iter()
        .filter_map(|e| e["source"].as_str())
        .collect();
    let state = json!({
        "requirement": {
            "taxon": manifest.taxon.scientific_name,
            "field": field.field,
            "condition": field.condition,
        },
        "evidence": evidence,
        "counts": {"sentences": evidence.len(), "sources": sources.len()},
    });
    Asked {
        state,
        questions: mature_questions(),
        keys: ("mature_size", "mature_gap"),
        points: evidence,
        covered: Vec::new(),
        uncovered: Vec::new(),
    }
}

/// A sentence from the field's proxy source describes the proxy taxon.
fn taxon_of<'a>(manifest: &'a Manifest, field: &'a Field, row: &Value) -> &'a str {
    field
        .proxy
        .as_ref()
        .filter(|p| row["source"] == p.source)
        .map_or(manifest.taxon.scientific_name.as_str(), |p| {
            p.taxon.as_str()
        })
}

/// Screened sentences and admitted table rows, each with its kind, condition
/// and taxon beside the requirement.
fn evidence_for(manifest: &Manifest, field: &Field, screen: &Value, fetch: &Value) -> Vec<Value> {
    let mut evidence: Vec<Value> = screen["rows"]
        .as_array()
        .into_iter()
        .flatten()
        .map(|row| {
            json!({
                "source": row["source"],
                "sentence": row["sentence"],
                "kind": row["kind"],
                "condition": row["condition"],
                "taxon": taxon_of(manifest, field, row),
            })
        })
        .collect();
    for (id, table) in fetch["tables"].as_object().into_iter().flatten() {
        if table["dimension"] != field.field {
            continue;
        }
        let rows = table["rows"].as_array().cloned().unwrap_or_default();
        let ages: Vec<f64> = rows
            .iter()
            .filter_map(|r| r["age_years"].as_f64())
            .collect();
        evidence.push(json!({
            "source": table["source"],
            "sentence": format!(
                "{} {} by age in {} rows from {} to {} years ({id})",
                table["taxon"].as_str().unwrap_or(""),
                field.field,
                rows.len(),
                ages.first().copied().unwrap_or(0.0),
                ages.last().copied().unwrap_or(0.0)
            ),
            "kind": if rows.len() >= 2 { "measured_size_at_age" } else { "not_about_tree_size" },
            "condition": table["condition"],
            "taxon": table["taxon"],
            "ages_years": ages,
        }));
    }
    evidence
}

/// Evidence items that are measured sizes at an age for this taxon under the
/// required condition, with the ages they state.
fn measured_points(manifest: &Manifest, field: &Field, evidence: &[Value]) -> Vec<Value> {
    evidence
        .iter()
        .filter(|item| {
            item["kind"] == "measured_size_at_age"
                && item["condition"] == field.condition
                && item["taxon"] == manifest.taxon.scientific_name
                && names_field(field, item["sentence"].as_str().unwrap_or_default())
        })
        .cloned()
        .collect()
}

/// Whether a sentence is about this field's dimension: a height sentence is
/// not a diameter point. The words are the requirements table's; a field
/// with no word list passes every sentence.
fn names_field(field: &Field, sentence: &str) -> bool {
    let Some(words) = terms(&field.field) else {
        return true;
    };
    let lower = sentence.to_ascii_lowercase();
    words.iter().any(|w| lower.contains(w.as_str()))
}

fn coverage(field: &Field, points: &[Value]) -> (Vec<f64>, Vec<f64>) {
    let ages: Vec<f64> = points
        .iter()
        .flat_map(|p| p["ages_years"].as_array().cloned().unwrap_or_default())
        .filter_map(|a| a.as_f64())
        .collect();
    field
        .required_ages_years
        .iter()
        .copied()
        .partition(|&required| {
            let near = ages
                .iter()
                .any(|&a| (a - required).abs() <= AGE_WINDOW * required);
            let below = ages.iter().any(|&a| a < required);
            let above = ages.iter().any(|&a| a > required);
            near || (below && above)
        })
}

/// One field's shortfall at the gate, as both decisions carry it.
struct Shortfall<'a> {
    field: &'a Field,
    level: Sufficiency,
    gap: &'a str,
    points: &'a [Value],
    uncovered: &'a [f64],
    ledger: &'a str,
    fetch_sha: &'a str,
}

impl Shortfall<'_> {
    fn decision(
        &self,
        manifest: &Manifest,
        kind: &str,
        payload: Value,
        options: &[&str],
        note: &str,
    ) -> Decision {
        let mut payload = payload;
        payload["field"] = json!(self.field.field);
        payload["level"] = json!(self.level.key());
        payload["dominant_gap"] = json!(self.gap);
        payload["points"] = json!(self.points);
        payload["required_ages_uncovered"] = json!(self.uncovered);
        payload["sources_tried"] = json!(manifest
            .sources
            .iter()
            .map(|s| s.id.clone())
            .collect::<Vec<_>>());
        Decision::new(
            DecisionParts {
                species: &manifest.species,
                stage: STAGE,
                kind,
                field: Some(&self.field.field),
                age_years: None,
            },
            &["select", "fit", "generate"],
            [("fetch.json".to_string(), self.fetch_sha.to_string())]
                .into_iter()
                .collect(),
            vec![self.ledger.to_string()],
            payload,
            options,
            note,
        )
    }
}

fn insufficient(manifest: &Manifest, s: &Shortfall<'_>) -> Decision {
    s.decision(
        manifest,
        "data-insufficient",
        json!({"bar": s.field.bar.key()}),
        &["admit-proxy", "add-sources", "lower-bar"],
        "The evidence for this field is below the manifest's bar; no select, fit or render runs for it until a person admits a proxy, adds sources or lowers the bar.",
    )
}

/// A required field below the requirements table's bar. The pipeline adds
/// sources for two rounds, then the owner does (NEEDS_HUMAN); the bar is
/// the table's and no option lowers it.
fn unmet(manifest: &Manifest, s: &Shortfall<'_>, bar: Sufficiency, sources: &str) -> Decision {
    s.decision(
        manifest,
        REQUIREMENTS_UNMET,
        json!({"bar": bar.key(), "sources_sha256": sources}),
        &["add-sources"],
        "The requirements table asks this field at its bar and the literature falls short. The pipeline searches again for sources aimed at the dominant gap, two rounds at most; after them it is NEEDS_HUMAN and the owner adds sources. A resolution that adds none stays open, and the table's bar is never lowered.",
    )
}
