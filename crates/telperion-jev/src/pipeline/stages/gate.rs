//! The onboarding gate, in code, before anything is generated.
//!
//! Three gates stand between the selected evidence and generation: every
//! capability the packet requires is one the generator declares it can
//! express, the preset is registered and bound to a profile, and the specimen
//! seeds are audited. Each unresolved gate files one `onboarding-gate`
//! decision that blocks generate, and the stage claims no measurement window
//! of its own. Jev is not called here.
//!
//! The capability gate asks the generator, not the preset: the vocabulary is
//! `telperion_core::capability`, a declared list no preset owns, so a species
//! with no preset yet is read on its needs rather than on its absence. A
//! registered preset is asked a second and different question, through the
//! benchmark binary: whether its own value table produces the capabilities
//! the species requires of it. That answer never reaches `missing`.

use std::path::PathBuf;
use std::process::Command;

use serde_json::{json, Map, Value};
use telperion_core::capability::{self, Support, DERIVABLE};

use crate::pipeline::canon::read_json;
use crate::pipeline::decision::{append_decisions, Decision, DecisionParts};
use crate::pipeline::manifest::Manifest;
use crate::pipeline::stage::{Context, Paths, StageError};

use super::{body, inputs};

pub const STAGE: &str = "gate";
/// The onboarding protocol's seed counts: three fixed and three holdout.
const SEEDS_PER_ROLE: usize = 3;

#[derive(Debug)]
pub enum Outcome {
    Current,
    Ran { decisions: Vec<String> },
}

/// What the registry and preset-table checks ask of the built repository. The
/// pipeline holds them behind a trait so a test answers without a built
/// binary. The capability vocabulary is not here: it is compiled in.
pub trait GateChecks {
    /// True when the preset is registered and bound to a profile.
    fn registry(&self, preset: &str) -> Result<bool, String>;
    /// The capability names this preset's own value table produces.
    fn capabilities(&self, preset: &str) -> Result<Vec<String>, String>;
}

/// The two example binaries as the pipeline runs them:
/// `species_measure --print-family <preset>` exits zero for a registered
/// preset, and `geometry_benchmark --support <preset>` prints
/// `{"implemented": bool, "profile_id": ..., "capabilities": [...]}`.
#[derive(Debug, Clone)]
pub struct ExampleChecks {
    pub geometry_benchmark: PathBuf,
    pub species_measure: PathBuf,
}

impl GateChecks for ExampleChecks {
    fn registry(&self, preset: &str) -> Result<bool, String> {
        let output = Command::new(&self.species_measure)
            .args(["--print-family", preset])
            .output()
            .map_err(|err| format!("{}: {err}", self.species_measure.display()))?;
        Ok(output.status.success())
    }

    fn capabilities(&self, preset: &str) -> Result<Vec<String>, String> {
        let output = Command::new(&self.geometry_benchmark)
            .args(["--support", preset])
            .output()
            .map_err(|err| format!("{}: {err}", self.geometry_benchmark.display()))?;
        if !output.status.success() {
            let printed = String::from_utf8_lossy(&output.stderr);
            let tail = printed.trim().lines().next_back().unwrap_or("no output");
            return Err(format!("--support {preset}: {tail}"));
        }
        let value: Value = serde_json::from_slice(&output.stdout)
            .map_err(|err| format!("--support {preset}: {err}"))?;
        Ok(names(&value["capabilities"]))
    }
}

pub fn run(paths: &Paths, checks: &dyn GateChecks) -> Result<Outcome, StageError> {
    let (ctx, _) = Context::open(paths, STAGE)?;
    let (_, select_sha) = body(&ctx, STAGE, "select")?;
    let header = ctx.header(
        STAGE,
        "gate",
        inputs(&[("select.json", &select_sha)]),
        vec![],
    );
    if ctx.is_current(STAGE, &header.idempotence_key) {
        return Ok(Outcome::Current);
    }
    let manifest = &ctx.admitted.manifest;
    let preset = manifest.preset.as_str();
    let mut unresolved: Vec<Value> = Vec::new();

    // The preset check reads the registry answer, so it is asked first; its
    // own detail is still filed after the capability gate's.
    let registered = checks.registry(preset);
    let required = required_capabilities(&ctx, manifest);
    let (capability, capability_details) =
        capability_gate(&required, preset, matches!(registered, Ok(true)), checks);
    unresolved.extend(capability_details);

    let registry = match registered {
        Ok(true) => json!(true),
        Ok(false) => {
            unresolved.push(gate_detail(
                "registry",
                &format!("preset {preset} is not registered with a bound profile"),
            ));
            json!(false)
        }
        Err(error) => {
            unresolved.push(gate_detail("registry", &error));
            json!(false)
        }
    };

    let seeds = match audited_seeds(&ctx) {
        Ok(counts) => counts,
        Err(detail) => {
            unresolved.push(gate_detail("seeds", &detail));
            json!({"status": "unresolved", "detail": detail})
        }
    };

    let decisions: Vec<Decision> = unresolved
        .iter()
        .map(|detail| onboarding_gate(manifest, detail, &select_sha))
        .collect();
    let ids: Vec<String> = decisions.iter().map(|d| d.id.clone()).collect();
    if !decisions.is_empty() {
        append_decisions(&ctx.paths.decisions(), decisions)?;
    }
    ctx.write(
        &header,
        json!({
            "capability": capability,
            "registry": registry,
            "seeds": seeds,
            "unresolved": unresolved,
        }),
    )?;
    Ok(Outcome::Ran { decisions: ids })
}

/// The capability gate: what the species requires, read against the
/// generator's declared vocabulary and against the preset's own table.
///
/// A required name the vocabulary expresses is met; one it carries as
/// unexpressed is missing; one it does not carry at all is unrecognised,
/// which is a different report, because nobody has said what the name means.
/// The gate fails closed: a species with no recorded requirement has had no
/// capability assessment, and that is unresolved rather than met.
fn capability_gate(
    required: &[String],
    preset: &str,
    registered: bool,
    checks: &dyn GateChecks,
) -> (Value, Vec<Value>) {
    let mut details = Vec::new();
    let (mut expressed, mut missing, mut unrecognised) = (Vec::new(), Vec::new(), Vec::new());
    for name in required {
        match capability::support(name) {
            Support::Expressed => expressed.push(name.clone()),
            Support::Absent => missing.push(name.clone()),
            Support::Unrecognised => unrecognised.push(name.clone()),
        }
    }
    if required.is_empty() {
        details.push(gate_detail(
            "capability",
            "no capability assessment is recorded: neither the packet's species.json \
             nor the manifest's engineering row names a required capability",
        ));
    }
    if !missing.is_empty() {
        details.push(gate_detail(
            "capability",
            &format!("the generator does not express {}", missing.join(", ")),
        ));
    }
    if !unrecognised.is_empty() {
        details.push(gate_detail(
            "capability",
            &format!(
                "{} is not a name the capability vocabulary carries",
                unrecognised.join(", ")
            ),
        ));
    }
    let (table, table_details) = preset_table(preset, registered, &expressed, checks);
    details.extend(table_details);
    (
        json!({"vocabulary_version": capability::version(), "required": required,
               "expressed": expressed, "missing": missing, "unrecognised": unrecognised,
               "preset": table}),
        details,
    )
}

/// The preset's own question: does this registered preset's value table
/// produce the capabilities the species requires of it? Only the names the
/// derivation reads a threshold for are asked, because a name it has no
/// threshold for says nothing either way. An unregistered preset has no table
/// to ask, so it is not asked, and its empty answer never reaches `missing`.
fn preset_table(
    preset: &str,
    registered: bool,
    expressed: &[String],
    checks: &dyn GateChecks,
) -> (Value, Vec<Value>) {
    if !registered {
        return (json!({"registered": false}), Vec::new());
    }
    match checks.capabilities(preset) {
        Ok(derived) => {
            let unproduced: Vec<String> = expressed
                .iter()
                .filter(|name| DERIVABLE.contains(&name.as_str()) && !derived.contains(name))
                .cloned()
                .collect();
            let details = if unproduced.is_empty() {
                Vec::new()
            } else {
                vec![gate_detail(
                    "preset-capability",
                    &format!(
                        "the {preset} value table does not produce {}",
                        unproduced.join(", ")
                    ),
                )]
            };
            (
                json!({"registered": true, "derived": derived, "unproduced": unproduced}),
                details,
            )
        }
        Err(error) => (
            json!({"registered": true, "error": error}),
            vec![gate_detail("preset-capability", &error)],
        ),
    }
}

/// The capabilities the packet requires: the generate stage's `species.json`
/// when it exists, else the manifest's engineering entry, else none.
fn required_capabilities(ctx: &Context, manifest: &Manifest) -> Vec<String> {
    let species = ctx.paths.packet("species");
    if species.exists() {
        if let Ok(value) = read_json(&species) {
            return names(&value["required_capabilities"]);
        }
    }
    manifest
        .engineering
        .get("required_capabilities")
        .map(|entry| names(&entry.value))
        .unwrap_or_default()
}

/// The audited seed counts, or why the gate is unresolved. The generate stage
/// writes `packet/specimens.json`, so before it runs the gate is unresolved.
fn audited_seeds(ctx: &Context) -> Result<Value, String> {
    let path = ctx.paths.packet("specimens");
    if !path.exists() {
        return Err("packet/specimens.json does not exist yet".into());
    }
    let value = read_json(&path).map_err(|err| err.to_string())?;
    let role = |name: &str| {
        value["cases"]
            .as_array()
            .into_iter()
            .flatten()
            .filter(|case| case["seed_role"].as_str().is_some_and(|r| r.contains(name)))
            .count()
    };
    let (fixed, holdout) = (role("regression"), role("holdout"));
    if fixed < SEEDS_PER_ROLE || holdout < SEEDS_PER_ROLE {
        return Err(format!(
            "{fixed} fixed and {holdout} holdout seeds, fewer than {SEEDS_PER_ROLE} each"
        ));
    }
    Ok(json!({"status": "resolved", "fixed": fixed, "holdout": holdout}))
}

fn gate_detail(gate: &str, detail: &str) -> Value {
    json!({"gate": gate, "detail": detail})
}

/// The string entries of a JSON array, in order; anything else is empty.
fn names(value: &Value) -> Vec<String> {
    value
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(|name| name.as_str().map(str::to_string))
        .collect()
}

fn onboarding_gate(manifest: &Manifest, detail: &Value, select_sha: &str) -> Decision {
    let gate = detail["gate"].as_str().unwrap_or_default();
    let mut payload = Map::new();
    payload.insert("gate".into(), detail["gate"].clone());
    payload.insert("detail".into(), detail["detail"].clone());
    Decision::new(
        DecisionParts {
            species: &manifest.species,
            stage: STAGE,
            kind: "onboarding-gate",
            field: Some(gate),
            age_years: None,
        },
        &["generate"],
        [("select.json".to_string(), select_sha.to_string())]
            .into_iter()
            .collect(),
        vec![],
        Value::Object(payload),
        &["resolve", "waive"],
        "An onboarding gate is unresolved; nothing is generated and no measurement window is claimed until a person resolves or waives it.",
    )
}
