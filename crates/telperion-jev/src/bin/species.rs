//! `species <id>`: the species runner (fn-149). Runs the eight stages from
//! the seeded manifest to the owner's look, rerunning only what changed, and
//! prints why it stopped. The runbook is `docs/species-runner.md`.
use std::env;
use std::path::PathBuf;
use std::process::ExitCode;

use telperion_jev::pipeline::stage::Paths;
use telperion_jev::runner::{self, live::Live, tools::Tools, Run};

const USAGE: &str = "usage: species <id> [--accept] [--tuning FILE] [--dir DIR] [--run-dir DIR] [--catalogue DIR] [--adapter firecrawl|fixture:DIR]";

fn main() -> ExitCode {
    let args: Vec<String> = env::args().skip(1).collect();
    let Some(species) = args.first().filter(|a| !a.starts_with("--")).cloned() else {
        eprintln!("{USAGE}");
        return ExitCode::from(2);
    };
    let flag = |name: &str| {
        args.windows(2)
            .find(|pair| pair[0] == name)
            .map(|pair| PathBuf::from(&pair[1]))
    };
    let evidence = PathBuf::from(".flow/evidence").join(&species);
    let dir = flag("--dir").unwrap_or_else(|| evidence.join("pipeline"));
    let run_dir = flag("--run-dir").unwrap_or_else(|| dir.clone());
    let run = Run {
        paths: Paths::with_run(&dir, &run_dir),
        catalogue: flag("--catalogue").unwrap_or_else(|| PathBuf::from("catalogue")),
        tuning: flag("--tuning").unwrap_or_else(|| evidence.join("tuning.json")),
        adapter: flag("--adapter").map_or("firecrawl".into(), |p| p.display().to_string()),
        accept: args.iter().any(|a| a == "--accept"),
        species,
    };
    let outcome = Tools::build(&PathBuf::from("."))
        .and_then(|tools| Live::new(&run, tools))
        .and_then(|live| runner::run(&run, &live));
    match outcome {
        Ok(None) => {
            println!("accepted");
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
