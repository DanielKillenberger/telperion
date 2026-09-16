//! Evidence-tooling entry point. Never imported by generation or the renderer.

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use serde_json::{Map, Value};
use telperion_jev::caller::{load_key, CallerError, UreqTransport};
use telperion_jev::cite::{cite, format_report as format_cite, load_source, parse_research};
use telperion_jev::ledger::SourceRef;
use telperion_jev::screen::{format_report as format_screen, screen};
use telperion_jev::select::{format_report as format_select, select};
use telperion_jev::sha256_hex;
use telperion_jev::triage::{format_proposal, triage};

fn main() -> ExitCode {
    let mut args = env::args().skip(1).collect::<Vec<_>>();
    if args.is_empty() {
        eprintln!(
            "usage: jev <screen|select|cite|triage> [options]\n  key: {path} via bash -ic",
            path = telperion_jev::INTERACTIVE_SHELL
        );
        return ExitCode::from(2);
    }
    let cmd = args.remove(0);
    match run(&cmd, &args) {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("{err}");
            ExitCode::from(1)
        }
    }
}

fn run(cmd: &str, args: &[String]) -> Result<(), String> {
    let key = load_key().map_err(|err| err.to_string())?;
    let ledger = flag(args, "--ledger")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(".flow/ledger/jev"));
    let transport = UreqTransport;
    match cmd {
        "screen" => {
            let path = required(args, "--source")?;
            let species = required(args, "--species")?;
            let bytes = fs::read(&path).map_err(|err| err.to_string())?;
            let source = SourceRef {
                id: flag(args, "--id").unwrap_or_else(|| path.clone()),
                url: flag(args, "--url").unwrap_or_default(),
                sha256: sha256_hex(&bytes),
                bytes: bytes.len() as u64,
            };
            let report =
                screen(&transport, &key, &ledger, &source, &bytes, &species).map_err(show_err)?;
            print!("{}", format_screen(&report));
        }
        "select" => {
            let path = required(args, "--document")?;
            let question = required(args, "--question")?;
            let document = fs::read_to_string(&path).map_err(|err| err.to_string())?;
            let report =
                select(&transport, &key, &ledger, &document, &question, None).map_err(show_err)?;
            print!("{}", format_select(&report));
        }
        "cite" => {
            let path = required(args, "--research")?;
            let markdown = fs::read_to_string(&path).map_err(|err| err.to_string())?;
            let claims = parse_research(&markdown);
            let loads: Vec<_> = claims
                .iter()
                .map(|claim| load_source(&transport, &claim.url))
                .collect();
            let report = cite(&transport, &key, &ledger, &claims, &loads).map_err(show_err)?;
            print!("{}", format_cite(&report));
        }
        "triage" => {
            let observation = required(args, "--observation")?;
            let specs_path = required(args, "--specs")?;
            let specs = read_map(&specs_path)?;
            let findings = flag(args, "--findings")
                .map(|path| read_strings(&path))
                .transpose()?
                .unwrap_or_default();
            let standard = flag(args, "--standard").unwrap_or_default();
            let proposal = triage(
                &transport,
                &key,
                &ledger,
                &observation,
                &specs,
                &findings,
                &standard,
            )
            .map_err(show_err)?;
            print!("{}", format_proposal(&proposal));
        }
        other => return Err(format!("unknown command {other}")),
    }
    Ok(())
}

fn show_err(err: CallerError) -> String {
    err.to_string()
}

fn flag(args: &[String], name: &str) -> Option<String> {
    args.windows(2)
        .find(|pair| pair[0] == name)
        .map(|pair| pair[1].clone())
}

fn required(args: &[String], name: &str) -> Result<String, String> {
    flag(args, name).ok_or_else(|| format!("missing {name}"))
}

fn read_map(path: &str) -> Result<Map<String, Value>, String> {
    let text = fs::read_to_string(path).map_err(|err| err.to_string())?;
    let value: Value = serde_json::from_str(&text).map_err(|err| err.to_string())?;
    value
        .as_object()
        .cloned()
        .ok_or_else(|| format!("{}: expected a JSON object", Path::new(path).display()))
}

fn read_strings(path: &str) -> Result<Vec<String>, String> {
    let text = fs::read_to_string(path).map_err(|err| err.to_string())?;
    let value: Value = serde_json::from_str(&text).map_err(|err| err.to_string())?;
    value
        .as_array()
        .ok_or_else(|| format!("{path}: expected a JSON array"))?
        .iter()
        .map(|item| {
            item.as_str()
                .map(str::to_string)
                .ok_or_else(|| format!("{path}: expected string entries"))
        })
        .collect()
}
