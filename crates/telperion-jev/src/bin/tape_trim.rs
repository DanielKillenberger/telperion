//! `tape_trim <tape> [--check] [--open HOST]...`: trims a recording's pages
//! that are not openly licensed to the passages the run quoted
//! (`telperion_jev::tape::trim`), or with `--check` lists any recorded page
//! that keeps more. Run it over a `species --record` tape before committing
//! it as a fixture.
use std::path::PathBuf;
use std::process::ExitCode;

use telperion_jev::tape::trim;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let Some(tape) = args
        .first()
        .filter(|a| !a.starts_with("--"))
        .map(PathBuf::from)
    else {
        eprintln!("usage: tape_trim <tape> [--check] [--open HOST]...");
        return ExitCode::from(2);
    };
    let open: Vec<String> = args
        .windows(2)
        .filter(|pair| pair[0] == "--open")
        .map(|pair| pair[1].clone())
        .collect();
    let result = match args.iter().any(|a| a == "--check") {
        true => trim::only_quoted(&tape, &open),
        false => trim::tape(&tape, &open).and_then(|mut words| {
            words.extend(trim::rekey(&tape)?);
            Ok(words)
        }),
    };
    match result {
        Ok(words) => {
            words.iter().for_each(|w| println!("{w}"));
            ExitCode::SUCCESS
        }
        Err(err) => {
            eprintln!("{err}");
            ExitCode::from(1)
        }
    }
}
