//! The literature stages as the runner runs them: Sources, Profile,
//! Capability and Catalogue, each a fixed run of the pipeline's own stages,
//! in process. The pipeline keeps its per-stage idempotence records; the
//! runner adds nothing to them.
use std::path::PathBuf;

use crate::caller::{load_key, UreqTransport};
use crate::pipeline::adapter::{FetchAdapter, FirecrawlCli, FixtureAdapter, RawSource};
use crate::pipeline::decision::{read_decisions, Decision, Status};
use crate::pipeline::judge::Judge;
use crate::pipeline::known::{flow_root, KnownSources};
use crate::pipeline::render::SpeciesExample;
use crate::pipeline::search;
use crate::pipeline::stages::{
    discover, document, extract, fetch, fit, gate, generate, quality, screen, select, verify,
};

use super::{tools::Tools, Run, Stage};

/// The decision kinds that stop a run: a sourced claim a person settles.
pub const CLAIMS: [&str; 4] = [
    "claim-contradicted",
    "claim-unsupported",
    "article-claim-contradicted",
    "article-claim-unsupported",
];

/// What the literature stages need beside the paths: Jev, the fetch
/// adapter, and the measurement example the tuning config names.
pub struct Literature<'a> {
    pub run: &'a Run,
    pub tools: &'a Tools,
    pub profiles: PathBuf,
    pub profile_id: String,
}

impl Literature<'_> {
    fn adapter(&self) -> Box<dyn FetchAdapter> {
        match self.run.adapter.strip_prefix("fixture:") {
            Some(dir) => Box::new(FixtureAdapter::new(dir)),
            None => {
                let mut cli = FirecrawlCli::new();
                cli.cache_dir = self.run.paths.cache().join("firecrawl");
                cli.raw_from = RawSource::Direct;
                Box::new(cli)
            }
        }
    }

    fn example(&self) -> SpeciesExample {
        SpeciesExample {
            measure_binary: self.tools.species_measure.clone(),
            headless_binary: Some(self.tools.headless.clone()),
            profiles: self.profiles.clone(),
            profile_id: self.profile_id.clone(),
            work_dir: self.run.paths.cache().join("measure"),
        }
    }

    fn checks(&self) -> gate::ExampleChecks {
        gate::ExampleChecks {
            geometry_benchmark: self.tools.geometry_benchmark.clone(),
            species_measure: self.tools.species_measure.clone(),
        }
    }

    /// Runs `stage`'s pipeline stages in order; the words say which ran.
    pub fn run(&self, stage: Stage) -> Result<String, String> {
        let key = load_key().map_err(|e| e.to_string())?;
        let transport = UreqTransport;
        let judge = Judge {
            transport: &transport,
            key: &key,
            ledger_dir: self.run.paths.ledger().join("entries"),
        };
        let paths = &self.run.paths;
        let mut words = Vec::new();
        let mut note = |name: &str, ran: bool| {
            words.push(format!("{name} {}", if ran { "ran" } else { "current" }))
        };
        let e = |err: crate::pipeline::stage::StageError| err.to_string();
        match stage {
            Stage::Sources => {
                let adapter = self.adapter();
                let known = KnownSources::scan(
                    &self.run.catalogue,
                    &flow_root(&paths.run),
                    &paths.manifest(),
                );
                let ran = discover::run(paths, adapter.as_ref(), &judge, &known).map_err(e)?;
                note("discover", !matches!(ran, discover::Outcome::Current));
                let ran = fetch::run(paths, adapter.as_ref()).map_err(e)?;
                note("fetch", !matches!(ran, fetch::Outcome::Current));
            }
            Stage::Profile => {
                // A requirement below its bar is searched for again and the
                // profile rerun, until the search has no round left.
                loop {
                    let ran = extract::run(paths).map_err(e)?;
                    note("extract", !matches!(ran, extract::Outcome::Current));
                    let ran = screen::run(paths, &judge).map_err(e)?;
                    note("screen", !matches!(ran, screen::Outcome::Current));
                    let ran = quality::run(paths, &judge).map_err(e)?;
                    note("quality", !matches!(ran, quality::Outcome::Current));
                    let ran = select::run(paths, &judge).map_err(e)?;
                    note("select", !matches!(ran, select::Outcome::Current));
                    let ran = verify::run(paths, &judge).map_err(e)?;
                    note("verify", !matches!(ran, verify::Outcome::Current));
                    let ran = fit::run(paths).map_err(e)?;
                    note("fit", !matches!(ran, fit::Outcome::Current));
                    let adapter = self.adapter();
                    match search::run(paths, adapter.as_ref(), &judge).map_err(e)? {
                        search::Outcome::Nothing => break,
                        search::Outcome::Ran { .. } => note("search-again", true),
                    }
                }
            }
            Stage::Capability => {
                let ran = gate::run(paths, &self.checks()).map_err(e)?;
                note("gate", !matches!(ran, gate::Outcome::Current));
            }
            Stage::Catalogue => {
                let example = self.example();
                let ran = generate::run(paths, &judge, &example, Some(&example)).map_err(e)?;
                note("generate", !matches!(ran, generate::Outcome::Current));
                // The gate audits the specimen seeds generate just wrote.
                let ran = gate::run(paths, &self.checks()).map_err(e)?;
                note("gate", !matches!(ran, gate::Outcome::Current));
                let ran = document::run(paths, &judge).map_err(e)?;
                note("document", !matches!(ran, document::Outcome::Current));
            }
            _ => unreachable!("not a literature stage"),
        }
        Ok(words.join(", "))
    }
}

/// The open decisions the pipeline filed, split into the claims that stop
/// the run and everything else, which is logged and never waited on.
pub fn open(run: &Run) -> Result<(Vec<String>, Vec<Decision>), String> {
    let list = read_decisions(&run.paths.decisions()).map_err(|e| e.to_string())?;
    let open: Vec<Decision> = list
        .into_iter()
        .filter(|d| d.status == Status::Open)
        .collect();
    let claims = open
        .iter()
        .filter(|d| CLAIMS.contains(&d.kind.as_str()))
        .map(|d| d.id.clone())
        .collect();
    let logged = open
        .into_iter()
        .filter(|d| !CLAIMS.contains(&d.kind.as_str()))
        .collect();
    Ok((claims, logged))
}

/// Each stage's files: what it reads and what it writes.
pub fn files(run: &Run, stage: Stage) -> (Vec<PathBuf>, Vec<PathBuf>) {
    let p = &run.paths;
    let a = |name: &str| p.artifact(name);
    match stage {
        Stage::Sources => (
            vec![p.manifest(), p.resolutions()],
            vec![a("discover"), a("fetch")],
        ),
        Stage::Profile => (
            vec![p.manifest(), p.resolutions(), a("fetch")],
            ["extract", "screen", "quality", "select", "verify", "fit"]
                .into_iter()
                .map(a)
                .chain([p.packet("profile"), p.packet("references")])
                .collect(),
        ),
        Stage::Capability => (vec![a("select"), p.packet("capability")], vec![a("gate")]),
        Stage::Catalogue => (
            vec![
                p.manifest(),
                p.resolutions(),
                a("select"),
                a("fit"),
                a("gate"),
            ],
            vec![
                a("generate"),
                a("document"),
                p.packet("species"),
                p.packet("specimens"),
                run.folder().join("ARTICLE.md"),
            ],
        ),
        _ => unreachable!("not a literature stage"),
    }
}
