//! The species template pipeline driver: one command per stage, the manifest
//! and every earlier artifact at fixed paths under `--dir`, which is the
//! species' catalogue folder. `--run-dir` takes the scratch a run leaves - the
//! fetch cache, the ledger, the command log and rendered stills - out of the
//! catalogue and into the evidence tree; without it the two are one directory.
//! The runbook is `docs/species-pipeline.md`.

use std::env;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use telperion_jev::caller::{load_key, UreqTransport};
use telperion_jev::pipeline::adapter::{FetchAdapter, FirecrawlCli, FixtureAdapter, RawSource};
use telperion_jev::pipeline::gap::cli as gap_cli;
use telperion_jev::pipeline::judge::Judge;
use telperion_jev::pipeline::known::{flow_root, KnownSources};
use telperion_jev::pipeline::render::{Measurer, SpeciesExample};
use telperion_jev::pipeline::stage::{log_command, Paths, STAGES};
use telperion_jev::pipeline::stages::{
    discover, extract, fetch, fit, gate, generate, quality, report, screen, select, verify,
};
use telperion_jev::pipeline::swap;

const USAGE: &str = "usage: species-pipeline <stage> --dir DIR [--run-dir DIR] [--catalogue DIR] [--adapter firecrawl|fixture:DIR] [--example] [--profiles FILE]\n       species-pipeline swap --left DIR --right DIR\n       species-pipeline gap <command> --dir DIR  (see `gap` for its own usage)\n  stages: discover fetch extract screen quality select verify fit gate generate report";

fn main() -> ExitCode {
    let args: Vec<String> = env::args().skip(1).collect();
    let Some(command) = args.first().cloned() else {
        eprintln!("{USAGE}");
        return ExitCode::from(2);
    };
    if command == "swap" {
        return match (flag(&args, "--left"), flag(&args, "--right")) {
            (Some(left), Some(right)) => {
                let comparison = swap::compare(Path::new(&left), Path::new(&right));
                print!("{}", swap::format_comparison(&comparison));
                if comparison.passed() {
                    ExitCode::SUCCESS
                } else {
                    ExitCode::from(1)
                }
            }
            _ => {
                eprintln!("{USAGE}");
                ExitCode::from(2)
            }
        };
    }
    let Some(dir) = flag(&args, "--dir").map(PathBuf::from) else {
        eprintln!("{USAGE}");
        return ExitCode::from(2);
    };
    let run_dir = flag(&args, "--run-dir")
        .map(PathBuf::from)
        .unwrap_or(dir.clone());
    let paths = Paths::with_run(&dir, &run_dir);
    if command == gap_cli::COMMAND {
        return finish(gap_cli::run(&paths, &args), &paths, &args);
    }
    finish(run(&command, &paths, &args), &paths, &args)
}

/// Prints the outcome, appends the command to the run's log, and exits.
fn finish(result: Result<String, String>, paths: &Paths, args: &[String]) -> ExitCode {
    let exit = match &result {
        Ok(message) => {
            println!("{message}");
            0
        }
        Err(message) => {
            eprintln!("{message}");
            1
        }
    };
    if let Err(err) = log_command(paths, args, exit) {
        eprintln!("command log: {err}");
    }
    ExitCode::from(exit as u8)
}

fn run(stage: &str, paths: &Paths, args: &[String]) -> Result<String, String> {
    if !STAGES.contains(&stage) {
        return Err(format!("unknown stage {stage}\n{USAGE}"));
    }
    let ledger_dir = paths.ledger().join("entries");
    let transport = UreqTransport;
    let needs_jev = matches!(
        stage,
        "discover" | "screen" | "quality" | "select" | "verify" | "generate"
    );
    let key = if needs_jev {
        load_key().map_err(|err| err.to_string())?
    } else {
        String::new()
    };
    let judge = Judge {
        transport: &transport,
        key: &key,
        ledger_dir,
    };
    let adapter = adapter_from(args, paths);
    let example = example_from(args, paths);
    let catalogue = flag(args, "--catalogue")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("catalogue"));
    let outcome = match stage {
        "discover" => {
            let flow = flow_root(&paths.run);
            let known = KnownSources::scan(&catalogue, &flow, &paths.manifest());
            if known.sources.is_empty() {
                eprintln!(
                    "discover: no catalogued sources, admitted manifests or research URLs found under {} or {}",
                    catalogue.display(),
                    flow.display()
                );
            }
            describe(discover::run(paths, adapter.as_ref(), &judge, &known))
        }
        "fetch" => describe(fetch::run(paths, adapter.as_ref())),
        "extract" => describe(extract::run(paths)),
        "screen" => describe(screen::run(paths, &judge)),
        "quality" => describe(quality::run(paths, &judge)),
        "select" => describe(select::run(paths, &judge)),
        "verify" => describe(verify::run(paths, &judge)),
        "fit" => describe(fit::run(paths)),
        "gate" => describe(gate::run(paths, &gate_checks(&example))),
        "generate" => describe(generate::run(
            paths,
            &judge,
            measurer(&example).as_ref(),
            example.as_ref(),
        )),
        "report" => describe(report::run(paths)),
        _ => unreachable!("stage list checked above"),
    };
    outcome.map(|word| format!("{stage}: {word}"))
}

fn describe<T: Outcome, E: std::fmt::Display>(result: Result<T, E>) -> Result<String, String> {
    result.map(|o| o.word()).map_err(|e| e.to_string())
}

/// What a stage reports on stdout: `current` when its key matched, else `ran`.
trait Outcome {
    fn word(&self) -> String;
}

macro_rules! outcome {
    ($($module:ident),*) => {$(
        impl Outcome for $module::Outcome {
            fn word(&self) -> String {
                match self {
                    $module::Outcome::Current => "current (idempotence key matched)".into(),
                    _ => "ran".into(),
                }
            }
        }
    )*};
}
outcome!(discover, fetch, extract, screen, quality, select, verify, fit, gate, generate, report);

fn adapter_from(args: &[String], paths: &Paths) -> Box<dyn FetchAdapter> {
    match flag(args, "--adapter").as_deref() {
        Some(spec) if spec.starts_with("fixture:") => {
            Box::new(FixtureAdapter::new(spec.trim_start_matches("fixture:")))
        }
        _ => {
            let mut cli = FirecrawlCli::new();
            cli.cache_dir = paths.cache().join("firecrawl");
            cli.raw_from = RawSource::Direct;
            Box::new(cli)
        }
    }
}

/// The measurement example and the headless renderer, when `--example` asks
/// for them; the binaries are the release examples under `target/`.
fn example_from(args: &[String], paths: &Paths) -> Option<SpeciesExample> {
    if !args.iter().any(|a| a == "--example") {
        return None;
    }
    let profiles = flag(args, "--profiles")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(".flow/evidence/fn9/profiles.json"));
    let headless = PathBuf::from("target/release/examples/headless");
    Some(SpeciesExample {
        measure_binary: PathBuf::from("target/release/examples/species_measure"),
        headless_binary: headless.exists().then_some(headless),
        profiles,
        profile_id: flag(args, "--profile-id").unwrap_or_default(),
        work_dir: paths.cache().join("measure"),
    })
}

fn measurer(example: &Option<SpeciesExample>) -> Box<dyn Measurer> {
    match example {
        Some(example) => Box::new(example.clone()),
        None => Box::new(NoMeasurer),
    }
}

/// Without `--example` nothing is rendered or measured; every candidate is
/// recorded unmeasured and the routes record their values as unavailable.
struct NoMeasurer;

impl Measurer for NoMeasurer {
    fn measure(
        &self,
        _preset: &str,
        _seed: u32,
        _family: &serde_json::Value,
    ) -> Result<
        telperion_jev::pipeline::render::Measured,
        telperion_jev::pipeline::render::RenderError,
    > {
        Err(telperion_jev::pipeline::render::RenderError::Measure(
            "no measurement example configured; pass --example".into(),
        ))
    }
}

fn gate_checks(example: &Option<SpeciesExample>) -> gate::ExampleChecks {
    gate::ExampleChecks {
        geometry_benchmark: PathBuf::from("target/release/examples/geometry_benchmark"),
        species_measure: example
            .as_ref()
            .map(|e| e.measure_binary.clone())
            .unwrap_or_else(|| PathBuf::from("target/release/examples/species_measure")),
    }
}

fn flag(args: &[String], name: &str) -> Option<String> {
    args.windows(2)
        .find(|pair| pair[0] == name)
        .map(|pair| pair[1].clone())
}
