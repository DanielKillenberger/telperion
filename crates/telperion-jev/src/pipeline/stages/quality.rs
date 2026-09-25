//! The data-quality gate, after the screen and before select, fit and
//! generation. For every required evidence field, code lays out the screened
//! candidates and the admitted table rows beside the requirement, counts the
//! measured points and the required ages they cover, and Jev scores the
//! sufficiency level and names the dominant gap. A field below the manifest's
//! bar files a data-insufficient decision; the stop is code on the level. A
//! field the requirements table requires, below the table's bar, files a
//! requirements-unmet decision instead, with no bar to lower: the pipeline
//! searches again for it twice (`pipeline::search`), then it is the owner's.
//! A field the table asks as `mature` (fn-127) or as a `rate` (fn-132)
//! asks no age: code lays out the sentences that name it and the
//! mature-size or growth-rate set scores a stated mature value or range, or
//! a stated yearly rate, on the same four levels. The gap follows the
//! points: a field with any point never carries the gap that says no value
//! is stated, whatever its level (fn-133).

use serde_json::{json, Map, Value};

use crate::pipeline::consume::{sources_sha256, REQUIREMENTS_UNMET};
use crate::pipeline::decision::{append_decisions, retire_unfiled, Decision, DecisionParts};
use crate::pipeline::judge::Judge;
use crate::pipeline::manifest::{Field, Manifest, Sufficiency};
use crate::pipeline::requirements::{asked, required_bar, Asked};
use crate::pipeline::sets::{
    chosen_level, mature_questions, rate_questions, sufficiency_questions, SUFFICIENCY_LEVELS,
};
use crate::pipeline::stage::{Context, Paths, StageError};

use super::points::{coverage, measured_points};
use super::rows::{for_field, stated_at};
use super::{body, inputs};

pub const STAGE: &str = "quality";
/// The gaps that say no value is stated at all: a field asked at no age
/// fails on one (fn-131), and a field with a point never carries one
/// (fn-133).
const UNSTATED: [&str; 2] = ["no_mature_size", "no_growth_rate"];

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
        let way = asked(manifest, &field.field);
        let question = match way {
            Asked::Age => at_age(manifest, field, &screen, &fetch),
            Asked::Mature => stated(manifest, field, &screen, mature_questions(), MATURE_KEYS),
            Asked::Rate => stated(manifest, field, &screen, rate_questions(), RATE_KEYS),
        };
        let (points, covered, uncovered) = (question.points, question.covered, question.uncovered);
        let (score_key, gap_key) = question.keys;
        let judgment = judge
            .ask(score_key, None, &question.state, &question.questions)
            .map_err(|err| StageError::Failed {
                stage: STAGE.into(),
                reason: err.to_string(),
            })?;
        // The most probable level; none at all is the lowest: the gate fails
        // closed. No labelled set of live sufficiency answers exists yet, so
        // no floor is trusted here (fn-131): the level floor is the
        // appearance levels'.
        let level = Sufficiency::from_index(chosen_level(
            judgment.entry.probabilities(score_key),
            SUFFICIENCY_LEVELS.len(),
            0,
            0.0,
        ));
        let gap = follow_points(
            level,
            !points.is_empty(),
            judgment
                .entry
                .choice(gap_key)
                .unwrap_or_else(|| "none".into()),
            judgment.entry.probabilities(gap_key),
        );
        // A field asked at no age with no value stated has nothing select can copy.
        let unstated = way != Asked::Age && UNSTATED.contains(&gap.as_str());
        let passed = level >= field.bar && !unstated;
        let required = required_bar(manifest, &field.field).filter(|bar| level < *bar || unstated);
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
        &crate::pipeline::stage::now(),
    )?;
    ctx.write(&header, json!({"fields": fields}))?;
    Ok(Outcome::Ran { decisions: ids })
}

/// One field's question: the state code laid out, the set and its answer
/// keys, and the points and age coverage the stage records.
struct Question {
    state: Value,
    questions: Value,
    /// The Score's key, which also names the tool in the ledger, and the gap Choice's.
    keys: (&'static str, &'static str),
    points: Vec<Value>,
    covered: Vec<f64>,
    uncovered: Vec<f64>,
}

/// An age-indexed field: the measured points at each required age.
fn at_age(manifest: &Manifest, field: &Field, screen: &Value, fetch: &Value) -> Question {
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
    Question {
        state,
        questions: sufficiency_questions(),
        keys: ("sufficiency", "dominant_gap"),
        points,
        covered,
        uncovered,
    }
}

/// The score and gap keys of the mature-size and growth-rate sets.
const MATURE_KEYS: (&str, &str) = ("mature_size", "mature_gap");
const RATE_KEYS: (&str, &str) = ("growth_rate", "rate_gap");

/// A gap that says no value is stated never stands beside a point (fn-133):
/// `sufficient` is met, `partial` is one source (fn-132), and a lower level
/// takes the most probable gap that names a shortfall of a stated value.
/// The live palm's leaflets were `sufficient` on 7 points and its trunk
/// diameter `proxy_only` on 3, each failed on `no_mature_size`.
fn follow_points(
    level: Sufficiency,
    has_points: bool,
    gap: String,
    probabilities: Option<&Value>,
) -> String {
    if !has_points || !UNSTATED.contains(&gap.as_str()) {
        return gap;
    }
    match level {
        Sufficiency::Sufficient => "none".into(),
        Sufficiency::Partial => "single_source".into(),
        _ => stated_gap(probabilities),
    }
}

/// The most probable gap that is neither unstated nor `none`; `wrong_taxon`,
/// which both sets offer, when Jev gave no other.
fn stated_gap(probabilities: Option<&Value>) -> String {
    probabilities
        .and_then(Value::as_object)
        .into_iter()
        .flatten()
        .filter(|(key, _)| key.as_str() != "none" && !UNSTATED.contains(&key.as_str()))
        .filter_map(|(key, p)| Some((key, p.as_f64()?)))
        .max_by(|a, b| a.1.total_cmp(&b.1))
        .map_or_else(|| "wrong_taxon".into(), |(key, _)| key.clone())
}

/// A field asked at no age: the screened rows select can use for it
/// (fn-131), an organ size by its organ class, a cultivar's size or a
/// growth rate among them. The points are those sentences; no age is asked
/// or covered.
fn stated(
    manifest: &Manifest,
    field: &Field,
    screen: &Value,
    questions: Value,
    keys: (&'static str, &'static str),
) -> Question {
    let evidence: Vec<Value> = for_field(&field.field, screen)
        .into_iter()
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
    Question {
        state,
        questions,
        keys,
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
            let mut item = json!({
                "source": row["source"],
                "sentence": row["sentence"],
                "kind": row["kind"],
                "condition": row["condition"],
                "taxon": taxon_of(manifest, field, row),
            });
            let ages = stated_at(row);
            if !ages.is_empty() {
                item["ages_years"] = json!(ages);
            }
            item
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
