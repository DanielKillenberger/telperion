//! What the four literature stages (Sources, Profile, Capability,
//! Catalogue) share: Jev, the fetch adapter, the measurement example, the
//! article claims a run may leave to a person, and the dropping of an inner
//! stage's record when an output it writes is gone. The pipeline keeps its own
//! per-stage idempotence records; the runner adds nothing to them.
use std::path::PathBuf;

use crate::caller::{load_key, UreqTransport};
use crate::pipeline::adapter::{FetchAdapter, FirecrawlCli, FixtureAdapter, RawSource, Retrying};
use crate::pipeline::decision::{
    apply_resolutions, read_decisions, read_resolutions, Decision, Status,
};
use crate::pipeline::judge::Judge;
use crate::pipeline::render::SpeciesExample;
use crate::pipeline::stage::StageError;
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

/// The decision kinds that are claims: an article sentence the citation
/// check flagged.
pub const CLAIMS: [&str; 2] = ["article-claim-contradicted", "article-claim-unsupported"];

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
