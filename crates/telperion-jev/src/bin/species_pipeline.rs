//! The species template pipeline driver: one command per stage, the manifest
//! and every earlier artifact at fixed paths under `--dir`, the run's command
//! log appended on every invocation. The runbook is `docs/species-pipeline.md`.

use std::env;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use telperion_jev::caller::{load_key, UreqTransport};
use telperion_jev::pipeline::adapter::{FetchAdapter, FirecrawlCli, FixtureAdapter, RawSource};
use telperion_jev::pipeline::judge::Judge;
use telperion_jev::pipeline::known::{flow_root, KnownSources};
use telperion_jev::pipeline::render::{Measurer, SpeciesExample};
use telperion_jev::pipeline::stage::{log_command, Paths, STAGES};
use telperion_jev::pipeline::stages::{
    discover, extract, fetch, fit, gate, generate, quality, report, screen, select, verify,
};
use telperion_jev::pipeline::swap;

const USAGE: &str = "usage: species-pipeline <stage> --dir DIR [--adapter firecrawl|fixture:DIR] [--example] [--profiles FILE]\n       species-pipeline swap --left DIR --right DIR\n  stages: discover fetch extract screen quality select verify fit gate generate report";

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
    let result = run(&command, &dir, &args);
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
    if let Err(err) = log_command(&Paths::new(&dir), &args, exit) {
        eprintln!("command log: {err}");
    }
    ExitCode::from(exit as u8)
}

fn run(stage: &str, dir: &Path, args: &[String]) -> Result<String, String> {
    if !STAGES.contains(&stage) {
        return Err(format!("unknown stage {stage}\n{USAGE}"));
    }
    let ledger_dir = Paths::new(dir).ledger().join("entries");
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
    let adapter = adapter_from(args, dir);
    let example = example_from(args, dir);
    let outcome = match stage {
        "discover" => {
            let flow = flow_root(dir);
            let known = KnownSources::scan(&flow, &Paths::new(dir).manifest());
            if known.sources.is_empty() {
                eprintln!(
                    "discover: no admitted manifests or research URLs found under {}",
                    flow.display()
                );
            }
            describe(discover::run(dir, adapter.as_ref(), &judge, &known))
        }
        "fetch" => describe(fetch::run(dir, adapter.as_ref())),
        "extract" => describe(extract::run(dir)),
        "screen" => describe(screen::run(dir, &judge)),
        "quality" => describe(quality::run(dir, &judge)),
        "select" => describe(select::run(dir, &judge)),
        "verify" => describe(verify::run(dir, &judge)),
        "fit" => describe(fit::run(dir)),
        "gate" => describe(gate::run(dir, &gate_checks(&example))),
        "generate" => describe(generate::run(
            dir,
            &judge,
            measurer(&example).as_ref(),
            example.as_ref(),
        )),
        "report" => describe(report::run(dir)),
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

fn adapter_from(args: &[String], dir: &Path) -> Box<dyn FetchAdapter> {
    match flag(args, "--adapter").as_deref() {
        Some(spec) if spec.starts_with("fixture:") => {
            Box::new(FixtureAdapter::new(spec.trim_start_matches("fixture:")))
        }
        _ => {
            let mut cli = FirecrawlCli::new();
            cli.cache_dir = dir.join("cache").join("firecrawl");
            cli.raw_from = RawSource::Direct;
            Box::new(cli)
        }
    }
}

/// The measurement example and the headless renderer, when `--example` asks
/// for them; the binaries are the release examples under `target/`.
fn example_from(args: &[String], dir: &Path) -> Option<SpeciesExample> {
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
        work_dir: dir.join("cache").join("measure"),
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
