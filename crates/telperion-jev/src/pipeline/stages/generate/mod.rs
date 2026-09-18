//! Generation: the two value routes, the packet records and the stills (R3).
//!
//! The stage measures the shipped preset once, runs the described route for
//! every trait the select stage scored above `unstated`, runs the reference
//! transfer for every dial the manifest lists, and writes the packet's
//! `species.json` and `specimens.json`. Every value that ships is one code
//! proposed and a render measured; Jev only names the nearest template and
//! scores the relation to it, and no output carries a probability.
//!
//! The reference templates of a dial come from the manifest, never from this
//! module: the engineering entry keyed `transfer:<dial>` holds a JSON array of
//! `{"template", "shipped_value", "growth_form"}`, an ordinary engineering
//! entry with its rationale that a person admits. Stills are never judged
//! here: they file one `visual-unassessed` decision that blocks nothing, so
//! the report may describe them and the owner's eye has the last word.

use std::path::Path;

use serde_json::{json, Map, Value};

use crate::pipeline::decision::{append_decisions, Decision, DecisionParts};
use crate::pipeline::judge::Judge;
use crate::pipeline::manifest::{Manifest, Transfer};
use crate::pipeline::render::{family_for, Measurer, SpeciesExample};
use crate::pipeline::routes::{
    described, transfer, DescribedInput, DescribedOutcome, Reference, RelationLevel, TransferInput,
    TransferOutcome,
};
use crate::pipeline::sets::level_from_score;
use crate::pipeline::stage::{Context, StageError};

mod packet;

use packet::{overlaid, pointer_for, run_parameters, write_packet, write_sidecar};

use super::{body, inputs};

pub const STAGE: &str = "generate";
/// The no-match answer of the nearest-reference Choice.
const NO_REFERENCE: &str = "No tuned template is a usable reference for this dial";
const NEAREST: &str = "Name the onboarded template whose shipped value for this dial is the \
     closest starting point for the species the sentences describe, or answer none when no \
     template is a usable reference.";
const RELATION: &str = "Score the new species' standing on this dial against the nearest \
     template, over the botanical situations below in order.";
/// The view and size every still is drawn at, so two runs draw the same frame.
const STILL: (&str, &str) = ("whole", "1024x1024");
/// Each filed kind: its name, the options a person picks from, and its note.
const MISS: Kind = (
    "level-miss",
    &["widen-range", "add-candidates", "reject"],
    "No candidate this run rendered measured inside the level's range; the pipeline ships no \
     nearest miss.",
);
const NONE: Kind = (
    "no-reference",
    &["add-reference", "tune-by-hand", "accept-unfilled"],
    "No tuned template is a usable reference for this dial; nothing is transferred in its place.",
);
const UNSEEN: Kind = (
    "visual-unassessed",
    &["accept", "reject"],
    "These stills were rendered and never assessed; the pipeline judges no image.",
);

type Kind = (&'static str, &'static [&'static str], &'static str);
/// The described route's two body maps: the traits it ran and the ones it did not.
type Described = (Map<String, Value>, Map<String, Value>);

#[derive(Debug)]
pub enum Outcome {
    Current,
    Ran { decisions: Vec<String> },
}

/// What the routes ship, gathered for the sidecar, the packet and the stills.
#[derive(Default)]
pub(super) struct Shipped {
    /// Sidecar pointer -> entry.
    entries: Map<String, Value>,
    /// Dial -> shipped value, in the order the routes shipped them.
    values: Vec<(String, f64)>,
    ledger: Vec<String>,
    decisions: Vec<Decision>,
}

impl Shipped {
    /// Files one decision: its kind, the field it scopes to, the stages it
    /// blocks, the payload code built, and the checksum of the select
    /// artifact a resolution binds to.
    fn file(
        &mut self,
        of: &Manifest,
        k: Kind,
        field: Option<&str>,
        blocks: &[&str],
        p: Value,
        sha: &str,
    ) {
        let parts = DecisionParts {
            species: &of.species,
            stage: STAGE,
            kind: k.0,
            field,
            age_years: None,
        };
        let inputs = [("select.json".to_string(), sha.to_string())];
        let decision = Decision::new(parts, blocks, inputs.into(), vec![], p, k.1, k.2);
        self.decisions.push(decision);
    }
}

pub fn run(
    dir: &Path,
    judge: &Judge<'_>,
    measurer: &dyn Measurer,
    example: Option<&SpeciesExample>,
) -> Result<Outcome, StageError> {
    let (ctx, blocked) = Context::open(dir, STAGE)?;
    // The capability and registry gates halt generation outright; the seeds
    // gate is answered by the specimens record this stage writes.
    let halting: Vec<String> = ctx
        .decisions
        .iter()
        .filter(|d| d.kind == "onboarding-gate" && d.blocks_stage(STAGE))
        .filter(|d| d.field.as_deref() != Some("seeds"))
        .map(|d| d.id.clone())
        .collect();
    if !halting.is_empty() {
        return Err(StageError::OpenDecision {
            stage: STAGE.into(),
            decisions: halting,
        });
    }
    let (select, sha) = body(&ctx, STAGE, "select")?;
    let (_, gate_sha) = body(&ctx, STAGE, "gate")?;
    let fit_sha = ctx
        .paths
        .artifact("fit")
        .exists()
        .then(|| body(&ctx, STAGE, "fit").map(|(_, sha)| sha))
        .transpose()?;
    let mut pairs = vec![
        ("select.json", sha.as_str()),
        ("gate.json", gate_sha.as_str()),
    ];
    if let Some(fit) = &fit_sha {
        pairs.push(("fit.json", fit.as_str()));
    }
    let mut header = ctx.header(STAGE, "generate", inputs(&pairs), vec![]);
    if ctx.is_current(STAGE, &header.idempotence_key) {
        return Ok(Outcome::Current);
    }
    let manifest = &ctx.admitted.manifest;

    let base = measurer.measure(&manifest.preset, manifest.seed, &json!({}));
    let printed = example.and_then(|e| e.print_family(&manifest.preset).ok());
    let (metrics, parameters, note) = match &base {
        Ok(done) => (
            done.metrics.clone(),
            printed
                .or_else(|| run_parameters(&done.receipt_path))
                .unwrap_or_else(|| json!({})),
            None,
        ),
        Err(err) => (
            json!({"status": "unavailable", "reason": err.to_string()}),
            json!({}),
            Some(format!("parameters unavailable: no example binary ({err})")),
        ),
    };

    let mut shipped = Shipped::default();
    let (described, unavailable) =
        run_described(manifest, &select, &blocked, measurer, &mut shipped, &sha)?;
    let mut transfers = Map::new();
    for spec in &manifest.transfers {
        let entry = run_transfer(judge, manifest, spec, &select, measurer, &mut shipped, &sha)?;
        transfers.insert(spec.dial.clone(), entry);
    }
    header.ledger.extend(shipped.ledger.iter().cloned());

    write_sidecar(&ctx, &shipped)?;
    write_packet(&ctx, manifest, overlaid(parameters, &shipped))?;
    let stills = draw_stills(&ctx, manifest, example, &shipped);
    let drawn: Vec<&Value> = stills.iter().filter(|s| s.get("path").is_some()).collect();
    if !drawn.is_empty() {
        let payload = json!({"stills": drawn});
        shipped.file(manifest, UNSEEN, None, &[], payload, &sha);
    }

    let out = json!({
        "metrics": metrics, "described": described, "transfers": transfers,
        "unavailable": unavailable, "stills": stills, "note": note,
    });
    let ids: Vec<String> = shipped.decisions.iter().map(|d| d.id.clone()).collect();
    if !shipped.decisions.is_empty() {
        append_decisions(&ctx.paths.decisions(), shipped.decisions)?;
    }
    ctx.write(&header, out)?;
    Ok(Outcome::Ran { decisions: ids })
}

/// The described route over every trait the select stage scored above the
/// no-match level and no open decision blocks. Returns the body's `described`
/// and `unavailable` maps.
fn run_described(
    manifest: &Manifest,
    select: &Value,
    blocked: &[String],
    measurer: &dyn Measurer,
    shipped: &mut Shipped,
    sha: &str,
) -> Result<Described, StageError> {
    let (mut body, mut unavailable) = (Map::new(), Map::new());
    for trait_ in &manifest.described {
        let name = trait_.trait_name.as_str();
        let scored = &select["described"][name];
        let level = scored["level"].as_str().unwrap_or("unstated");
        if level == "unstated" || blocked.contains(&name.to_string()) {
            unavailable.insert(name.into(), json!("the trait is unstated or blocked"));
            continue;
        }
        let input = DescribedInput {
            table: trait_.table.clone(),
            level_key: level.to_string(),
            sentence: scored["sentence"].as_str().unwrap_or_default().to_string(),
            ledger: scored["ledger"].as_str().unwrap_or_default().to_string(),
            preset: manifest.preset.clone(),
            seed: manifest.seed,
        };
        match described(&input, measurer).map_err(failed)? {
            DescribedOutcome::Shipped(entry) => {
                let value = serde_json::to_value(&entry).expect("a described entry serializes");
                shipped
                    .entries
                    .insert(pointer_for(&entry.dial), value.clone());
                shipped.values.push((entry.dial.clone(), entry.shipped));
                shipped.ledger.extend(entry.ledger.iter().cloned());
                body.insert(name.into(), value);
            }
            DescribedOutcome::LevelMiss(miss) => {
                let payload = serde_json::to_value(&miss).expect("a level miss serializes");
                body.insert(name.into(), payload.clone());
                shipped.file(manifest, MISS, Some(name), &["report"], payload, sha);
            }
            DescribedOutcome::Unavailable { reason } => {
                unavailable.insert(name.into(), json!(reason));
            }
        }
    }
    Ok((body, unavailable))
}

/// One dial's reference transfer: two questions in one request, then the same
/// render-and-measure step the described route takes.
fn run_transfer(
    judge: &Judge<'_>,
    manifest: &Manifest,
    spec: &Transfer,
    select: &Value,
    measurer: &dyn Measurer,
    shipped: &mut Shipped,
    sha: &str,
) -> Result<Value, StageError> {
    let references = references_for(manifest, &spec.dial);
    let templates: Vec<String> = references.iter().map(|r| r.template.clone()).collect();
    let spoken: Vec<&str> = select["described"]
        .as_object()
        .into_iter()
        .flatten()
        .filter_map(|(_, entry)| entry["sentence"].as_str())
        .collect();
    let state = json!({
        "dial": spec.dial,
        "species_sentences": spoken.join("\n"),
        "references": references.iter().map(|r| {
            json!({"template": r.template, "shipped_value": r.shipped_value, "sentences": ""})
        }).collect::<Vec<_>>(),
    });
    let questions = transfer_questions(&templates, &spec.levels);
    let judgment = judge
        .ask("transfer", None, &state, &questions)
        .map_err(failed)?;
    let nearest = judgment.entry.choice("nearest").unwrap_or("none".into());
    let scored = judgment.entry.score("relation").unwrap_or(f64::NAN);
    let index = level_from_score(scored, spec.levels.len());
    // No relation score is no reference: the dial files no-reference, never the first level.
    let nearest = if index.is_none() {
        NO_REFERENCE.to_string()
    } else {
        nearest
    };
    shipped.ledger.push(judgment.reference.clone());
    let input = TransferInput {
        dial: spec.dial.clone(),
        references,
        nearest: if nearest == NO_REFERENCE {
            "none".into()
        } else {
            nearest
        },
        second: None,
        relation: index
            .and_then(|i| spec.levels.get(i))
            .map_or_else(String::new, |l| l.key.clone()),
        levels: spec.levels.clone(),
        target_range: spec.target_range,
        measured_metric: spec.measured_metric.clone(),
        forbid_other_growth_form: spec.forbid_other_growth_form,
        growth_form: manifest.growth_form.clone(),
        ledger: vec![judgment.reference],
        preset: manifest.preset.clone(),
        seed: manifest.seed,
    };
    let dial = Some(spec.dial.as_str());
    Ok(match transfer(&input, measurer).map_err(failed)? {
        TransferOutcome::Shipped(entry) => {
            let value = serde_json::to_value(&entry).expect("a transfer entry serializes");
            shipped
                .entries
                .insert(pointer_for(&entry.dial), value.clone());
            shipped.values.push((entry.dial.clone(), entry.candidate));
            value
        }
        TransferOutcome::NoReference { reason, .. } => {
            let payload = json!({"dial": spec.dial, "reason": reason});
            shipped.file(manifest, NONE, dial, &["report"], payload.clone(), sha);
            payload
        }
        TransferOutcome::Unavailable { reason } => {
            let payload = json!({"dial": spec.dial, "reason": reason});
            shipped.file(manifest, MISS, dial, &["report"], payload.clone(), sha);
            payload
        }
    })
}

pub(super) fn failed(err: impl std::fmt::Display) -> StageError {
    StageError::Failed {
        stage: STAGE.into(),
        reason: err.to_string(),
    }
}

/// The tuned templates a dial transfers from, as the manifest's engineering
/// entry `transfer:<dial>` states them.
fn references_for(manifest: &Manifest, dial: &str) -> Vec<Reference> {
    manifest
        .engineering
        .get(&format!("transfer:{dial}"))
        .and_then(|entry| serde_json::from_value(entry.value.clone()).ok())
        .unwrap_or_default()
}

/// The nearest-reference Choice and the relation Score, in one request.
pub fn transfer_questions(templates: &[String], levels: &[RelationLevel]) -> Value {
    let mut criteria = Map::new();
    for template in templates {
        criteria.insert(template.clone(), Value::Null);
    }
    criteria.insert("none".into(), json!(NO_REFERENCE));
    let relation: Vec<Value> = levels
        .iter()
        .map(|level| json!({"key": level.key, "summary": level.summary}))
        .collect();
    json!({
        "nearest": {"type": "choice", "instructions": NEAREST, "criteria": criteria},
        "relation": {"type": "score", "instructions": RELATION, "criteria": relation},
    })
}

/// The sidecar pointer of a dial: `skeleton.envelope.spread` is laid at
/// `/parameters/skeleton/envelope/spread`, where that value sits in the
/// packet's parameters.
/// One still per shipped candidate and one of the preset as shipped. A render
/// error is recorded beside the stills and never stops the stage.
fn draw_stills(
    ctx: &Context,
    manifest: &Manifest,
    example: Option<&SpeciesExample>,
    shipped: &Shipped,
) -> Vec<Value> {
    let Some(example) = example.filter(|e| e.headless_binary.is_some()) else {
        return Vec::new();
    };
    let dir = ctx.paths.dir.join("stills");
    let mut cases = vec![("preset".to_string(), json!({}))];
    for (dial, value) in &shipped.values {
        cases.push((dial.replace('.', "-"), family_for(dial, *value)));
    }
    let (preset, seed) = (manifest.preset.as_str(), manifest.seed);
    cases
        .into_iter()
        .map(|(name, family)| {
            let out = dir.join(format!("{name}.png"));
            match example.still(preset, seed, &family, &out, STILL.0, STILL.1) {
                Ok(path) => json!({"still": name, "path": path.display().to_string()}),
                Err(err) => json!({"still": name, "error": err.to_string()}),
            }
        })
        .collect()
}
