//! `species <id>`: the species runner (fn-149). Runs the eight stages from
//! the seeded manifest to the owner's look, rerunning only what changed, and
//! prints why it stopped. `--until <stage>` stops after a stage, `--stage
//! <stage>` runs one alone, `--status` says what each would do and runs
//! nothing. The runbook is `docs/species-runner.md`.
use std::env;
use std::path::PathBuf;
use std::process::ExitCode;

use telperion_jev::pipeline::stage::Paths;
use telperion_jev::runner::record::State;
use telperion_jev::runner::{self, preflight, Run, Scope, STAGES};
use telperion_jev::tape;

const USAGE: &str = "usage: species <id> [--until STAGE | --stage STAGE | --status] [--record DIR | --replay DIR] [--accept] [--settle-claims] [--tuning FILE] [--dir DIR] [--run-dir DIR] [--catalogue DIR] [--adapter firecrawl|fixture:DIR]";

fn main() -> ExitCode {
    let args: Vec<String> = env::args().skip(1).collect();
    let Some(species) = args.first().filter(|a| !a.starts_with("--")) else {
        eprintln!("{USAGE}");
        return ExitCode::from(2);
    };
    let value = |name: &str| {
        args.windows(2)
            .find(|pair| pair[0] == name)
            .map(|pair| pair[1].clone())
    };
    let has = |name: &str| args.iter().any(|a| a == name);
    // The species' catalogue folder holds the manifest and every stage
    // artifact, which the catalogue scripts read in place; the run's scratch
    // stays in the evidence tree.
    let evidence = PathBuf::from(".flow/evidence").join(species);
    let path = |name: &str| value(name).map(PathBuf::from);
    let catalogue = path("--catalogue").unwrap_or_else(|| PathBuf::from("catalogue"));
    let dir = path("--dir").unwrap_or_else(|| catalogue.join(species));
    let run_dir = path("--run-dir").unwrap_or_else(|| evidence.join("run"));
    let tuning = path("--tuning").unwrap_or_else(|| evidence.join("tuning.json"));
    let mut run = Run::new(species, Paths::with_run(&dir, &run_dir), catalogue, tuning);
    run.adapter = value("--adapter").unwrap_or(run.adapter);
    run.accept = has("--accept");
    run.settle_claims = has("--settle-claims");
    // One tape for the process and every adapter program it starts.
    match (value("--record"), value("--replay")) {
        (Some(_), Some(_)) => {
            eprintln!("--record and --replay are one or the other\n{USAGE}");
            return ExitCode::from(2);
        }
        (Some(dir), None) => env::set_var(tape::VAR, format!("record:{}", absolute(&dir))),
        (None, Some(dir)) => env::set_var(tape::VAR, format!("replay:{}", absolute(&dir))),
        (None, None) => env::remove_var(tape::VAR),
    }
    if has("--status") {
        run.build = false;
        return status(&run);
    }
    let scope = match (value("--until"), value("--stage")) {
        (Some(_), Some(_)) => {
            eprintln!("--until and --stage are one or the other\n{USAGE}");
            return ExitCode::from(2);
        }
        (Some(name), None) => Scope::Until(name),
        (None, Some(name)) => Scope::Only(name),
        (None, None) => Scope::All,
    };
    match runner::run(&run, &STAGES, &scope) {
        Ok(None) => {
            match scope {
                Scope::All => println!("accepted"),
                Scope::Until(name) | Scope::Only(name) => println!("reached {name}"),
            }
            ExitCode::SUCCESS
        }
        Ok(Some(stop)) => {
            println!("STOPPED: {stop}");
            ExitCode::from(3)
        }
        Err(err) => {
            eprintln!("{err}");
            ExitCode::from(1)
        }
    }
}

/// `dir` from the repository root, so an adapter program finds it too.
fn absolute(dir: &str) -> String {
    let _ = std::fs::create_dir_all(dir);
    std::fs::canonicalize(dir).map_or(dir.into(), |p| p.display().to_string())
}

/// Each stage's state, the preflight, and what the stages that would run
/// are expected to spend. No stage runs.
fn status(run: &Run) -> ExitCode {
    let states = match runner::status(run, &STAGES) {
        Ok(states) => states,
        Err(err) => {
            eprintln!("{err}");
            return ExitCode::from(1);
        }
    };
    for (name, state) in &states {
        println!("{name}: {state}");
    }
    let mut failed = false;
    for (name, found) in preflight::checks(run, tape::Tape::from_env().as_ref()) {
        match found {
            Ok(said) => println!("preflight {name}: ok, {said}"),
            Err(why) => {
                failed = true;
                println!("preflight {name}: FAILED, {why}");
            }
        }
    }
    for (name, state) in &states {
        if *state != State::Current {
            println!("expected {name}: {}", preflight::expected(run, name));
        }
    }
    match failed {
        true => ExitCode::from(1),
        false => ExitCode::SUCCESS,
    }
}
