//! What the four literature stages (Sources, Profile, Capability,
//! Catalogue) share: Jev, the fetch adapter, the measurement example, the
//! decisions the runner settles itself, and the dropping of an inner stage's
//! record when an output it writes is gone. The pipeline keeps its own
//! per-stage idempotence records; the runner adds nothing to them.
use std::path::PathBuf;

use serde_json::Value;

use crate::caller::{load_key, UreqTransport};
use crate::pipeline::adapter::{FetchAdapter, FirecrawlCli, FixtureAdapter, RawSource, Retrying};
use crate::pipeline::admission::record_resolution;
use crate::pipeline::decision::{
    apply_resolutions, read_decisions, read_resolutions, Decision, Resolution, Status,
};
use crate::pipeline::judge::Judge;
use crate::pipeline::render::SpeciesExample;
use crate::pipeline::search;
use crate::pipeline::stage::StageError;
use crate::pipeline::stages::flagged::{DROP_VALUE, KEEP_RANGE, REPLACE_SOURCE};
use crate::pipeline::stages::gate;

use super::{start, Done, Run, Stop};
use crate::tape;

/// The key and transport a stage's Jev calls go through, recorded or
/// replayed when the run has a tape.
pub struct Jev {
    key: String,
    transport: tape::Jev<'static>,
}

impl Jev {
    pub fn load() -> Result<Self, String> {
        Ok(Self {
            key: tape::key(|| load_key().map_err(|e| e.to_string()))?,
            transport: tape::Jev::new(&UreqTransport),
        })
    }
    pub fn judge(&self, run: &Run) -> Judge<'_> {
        Judge {
            transport: &self.transport,
            key: &self.key,
            ledger_dir: run.paths.ledger().join("entries"),
        }
    }
}

/// A pipeline stage's error as the runner reports it.
pub fn e(err: StageError) -> String {
    err.to_string()
}

/// The fetch adapter, retrying what a rate limit refused.
pub fn adapter(run: &Run) -> Box<dyn FetchAdapter> {
    let inner: Box<dyn FetchAdapter> = match run.adapter.strip_prefix("fixture:") {
        Some(dir) => Box::new(FixtureAdapter::new(dir)),
        None => {
            let mut cli = FirecrawlCli::new();
            cli.cache_dir = run.paths.cache().join("firecrawl");
            cli.raw_from = RawSource::Direct;
            Box::new(cli)
        }
    };
    Box::new(Retrying::new(tape::fetch(inner)))
}

/// The measurement example, reading the tuning config's profile manifest.
pub fn example(run: &Run) -> Result<SpeciesExample, String> {
    let tuning = crate::pipeline::canon::read_json(&run.tuning).map_err(|e| e.to_string())?;
    let tools = run.tools()?;
    Ok(SpeciesExample {
        measure_binary: tools.species_measure.clone(),
        headless_binary: Some(tools.headless.clone()),
        profiles: PathBuf::from(start::field(&tuning, "profiles")?),
        profile_id: start::field(&tuning, "profile_id")?,
        work_dir: run.paths.cache().join("measure"),
    })
}

pub fn checks(run: &Run) -> Result<gate::ExampleChecks, String> {
    let tools = run.tools()?;
    Ok(gate::ExampleChecks {
        geometry_benchmark: tools.geometry_benchmark.clone(),
        species_measure: tools.species_measure.clone(),
    })
}

/// `name ran` or `name current`.
pub fn said(words: &mut Vec<String>, name: &str, ran: bool) {
    words.push(format!("{name} {}", if ran { "ran" } else { "current" }));
}

/// Drops the inner records of `names` when any of `outputs` is gone, so
/// their own keys cannot report a missing output current.
pub fn refresh(run: &Run, outputs: &[PathBuf], names: &[&str]) -> Result<(), String> {
    if outputs.iter().all(|p| p.exists()) {
        return Ok(());
    }
    for name in names {
        forget(run.paths.artifact(name))?;
    }
    Ok(())
}

pub fn forget(path: PathBuf) -> Result<(), String> {
    match std::fs::remove_file(&path) {
        Err(e) if e.kind() != std::io::ErrorKind::NotFound => {
            Err(format!("{}: {e}", path.display()))
        }
        _ => Ok(()),
    }
}

/// The decision kinds that are claims: a sourced value or an article
/// sentence the citation check flagged.
pub const CLAIMS: [&str; 4] = [
    "claim-contradicted",
    "claim-unsupported",
    "article-claim-contradicted",
    "article-claim-unsupported",
];

/// The option the runner settles `d` with, if it settles it: an unadmitted
/// proposal is skipped and an unreadable source dropped. A flagged value is
/// sent to the search while its field has a round left; after that a
/// contradicted measurement keeps the range its sources span and anything
/// else is dropped, leaving the field unsourced, unless the run leaves
/// claims to a person.
fn option(run: &Run, d: &Decision) -> Option<&'static str> {
    let pointer = d.field.as_deref().unwrap_or_default();
    match d.kind.as_str() {
        "manifest-proposed" => Some("skip"),
        "unavailable-source" => Some("drop-source"),
        "claim-contradicted" | "claim-unsupported" if !exhausted(run, claimed(pointer)) => {
            Some(REPLACE_SOURCE)
        }
        _ if run.settle_claims => None,
        "claim-contradicted" if pointer.contains("/metrics/") => Some(KEEP_RANGE),
        "claim-contradicted" | "claim-unsupported" => Some(DROP_VALUE),
        _ => None,
    }
}

/// The field a flagged claim's pointer names: the metric after `metrics`,
/// else its last key.
fn claimed(pointer: &str) -> &str {
    let keys: Vec<&str> = pointer.split('/').collect();
    keys.iter()
        .position(|k| *k == "metrics")
        .and_then(|at| keys.get(at + 1))
        .or(keys.last())
        .copied()
        .unwrap_or_default()
}

/// Whether `field` has spent its search rounds.
fn exhausted(run: &Run, field: &str) -> bool {
    let rounds = search::read_rounds(&run.paths).unwrap_or_default();
    rounds.get(field).map_or(0, Vec::len) >= search::MAX_ROUNDS
}

/// The decisions of the run no bound resolution settles yet.
fn open(run: &Run) -> Result<Vec<Decision>, String> {
    let p = &run.paths;
    let mut list = read_decisions(&p.decisions()).map_err(|e| e.to_string())?;
    let resolutions = read_resolutions(&p.resolutions()).map_err(|e| e.to_string())?;
    apply_resolutions(&mut list, &resolutions);
    Ok(list
        .into_iter()
        .filter(|d| d.status == Status::Open)
        .collect())
}

/// Resolves every open decision the runner settles itself, recording each
/// as the runner's resolution; the words name each option and id.
pub fn settle(run: &Run) -> Result<Vec<String>, String> {
    let mut settled = Vec::new();
    for d in open(run)? {
        let Some(option) = option(run, &d) else {
            continue;
        };
        let resolution = Resolution {
            id: d.id.clone(),
            inputs_sha256: d.inputs_sha256.clone(),
            option: option.into(),
            by: "species runner".into(),
            at: crate::pipeline::stage::now(),
            note: "settled by the runner, never waited on (fn-149)".into(),
            payload: Value::Null,
        };
        record_resolution(&run.paths.resolutions(), &resolution).map_err(|e| e.to_string())?;
        settled.push(format!("{option} {}", d.id));
    }
    Ok(settled)
}

/// The claims left open: what stops a run that leaves them to a person.
pub fn claims(run: &Run) -> Result<Option<Stop>, String> {
    let ids: Vec<String> = open(run)?
        .into_iter()
        .filter(|d| CLAIMS.contains(&d.kind.as_str()))
        .map(|d| d.id)
        .collect();
    Ok((run.settle_claims && !ids.is_empty()).then_some(Stop::Claims(ids)))
}

/// `word`, and the open decisions the run logged and did not wait on.
pub fn logged(run: &Run, word: String) -> Result<Done, String> {
    let ids: Vec<String> = open(run)?.into_iter().map(|d| d.id).collect();
    Ok(Done::Ran(match ids.is_empty() {
        true => word,
        false => format!("{word}; logged, not waited on: {}", ids.join(", ")),
    }))
}
