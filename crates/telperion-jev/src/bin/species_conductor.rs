//! The species conductor's driver (fn-89): one command per hop, every record
//! under the run directory's `conductor/` folder. The surface is
//! `telperion_jev::conductor::cli`; the doc is `docs/species-conductor.md`.
use std::env;
use std::process::ExitCode;

use telperion_jev::conductor::cli;

fn main() -> ExitCode {
    let args: Vec<String> = env::args().skip(1).collect();
    match cli::run(&args) {
        Ok(message) => {
            println!("{message}");
            ExitCode::SUCCESS
        }
        Err(message) => {
            eprintln!("{message}");
            ExitCode::from(1)
        }
    }
}
