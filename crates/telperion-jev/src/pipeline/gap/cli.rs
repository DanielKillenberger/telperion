//! `species-pipeline gap <command> --dir DIR`: the loop's surface.
//!
//! One command per step of the loop, each reading and writing the run's own
//! artifacts under `DIR` and printing one line. The driver passes nothing but
//! paths and ids: it writes no option, resolves no decision and judges no
//! still. Only `route` reaches Jev, and only through the shared caller.

use serde_json::Value;

use super::option::{parse_options, Author, Written};
use super::table::Route;
use super::{metrics, option, resume, rounds, route as routing};
use crate::caller::{load_key, UreqTransport};
use crate::pipeline::judge::Judge;
use crate::pipeline::stage::Paths;

pub const COMMAND: &str = "gap";
pub const USAGE: &str = "usage: species-pipeline gap <command> --dir DIR\n  \
    open    --decision ID                          the gap record for a halting decision\n  \
    options --decision ID --author agent|stronger --model NAME --options FILE\n  \
    route   --decision ID [--verdicts FILE]        ask Jev over the set and route it\n  \
    reroute --decision ID                          re-read the recorded signals, no call\n  \
    spec    --decision ID --spec SPEC              record the spec minted for the fix\n  \
    review  --decision ID --verdict ship|needs-work\n  \
    resume  --decision ID --commit SHA [--pin-note NOTE]\n  \
    round   --species S --verdict V [--note N]     open one value round\n  \
    accept  --species S --verdict V                the verdict accepts\n  \
    metrics --species S                            write metrics.json";

/// Runs one gap command. `args` is the driver's argv from `gap` onward, and
/// `paths` carries the species folder the driver resolved from `--dir` beside
/// the run directory the ledger is written to.
pub fn run(paths: &Paths, args: &[String]) -> Result<String, String> {
    let command = args.get(1).cloned().unwrap_or_default();
    // The driver resolved `--dir` into `paths` already; the flag is still
    // required here, so a gap command that lacks it names its own usage.
    flag(args, "--dir").ok_or_else(|| USAGE.to_string())?;
    match command.as_str() {
        "open" => {
            let id = required(args, "--decision")?;
            let record = super::open(paths, &id).map_err(show)?;
            super::write(paths, &record).map_err(show)?;
            Ok(format!(
                "gap open: {id} ({} at {})",
                record["halt"]["kind"].as_str().unwrap_or_default(),
                record["halt"]["stage"].as_str().unwrap_or_default()
            ))
        }
        "options" => {
            let id = required(args, "--decision")?;
            let author = Author::parse(&required(args, "--author")?)
                .ok_or_else(|| "--author is agent or stronger".to_string())?;
            let model = required(args, "--model")?;
            let file = required(args, "--options")?;
            let options = parse_options(&read_json_file(&file)?).map_err(show)?;
            let count = options.len();
            let mut record = super::open(paths, &id).map_err(show)?;
            let written = option::add_set(&mut record, author, &model, options).map_err(show)?;
            super::write(paths, &record).map_err(show)?;
            Ok(match written {
                Written::Ready => format!("gap options: {count} from the {}", author.key()),
                Written::EmptyToStronger => {
                    "gap options: empty from the agent; the stronger model writes the set".into()
                }
                Written::EmptyToOwner => {
                    "gap options: empty from both; the gap is the owner's".into()
                }
            })
        }
        "route" => {
            let id = required(args, "--decision")?;
            let verdicts = match flag(args, "--verdicts") {
                Some(path) => read_strings(&path)?,
                None => Vec::new(),
            };
            let key = load_key().map_err(|err| err.to_string())?;
            let transport = UreqTransport;
            let judge = Judge {
                transport: &transport,
                key: &key,
                ledger_dir: paths.ledger().join("entries"),
            };
            let routed = routing::route(paths, &judge, &id, &verdicts).map_err(show)?;
            Ok(describe_route(&routed))
        }
        "reroute" => {
            let id = required(args, "--decision")?;
            let routed = routing::reroute(paths, &id).map_err(show)?;
            Ok(describe_route(&routed))
        }
        "spec" => {
            let id = required(args, "--decision")?;
            let spec = required(args, "--spec")?;
            resume::record_spec(paths, &id, &spec).map_err(show)?;
            Ok(format!("gap spec: {spec} minted for {id}"))
        }
        "review" => {
            let id = required(args, "--decision")?;
            let verdict = required(args, "--verdict")?;
            Ok(
                match resume::record_review(paths, &id, &verdict).map_err(show)? {
                    resume::Reviewed::Recorded { needs_work } => {
                        format!("gap review: {verdict} ({needs_work} needs-work so far)")
                    }
                    resume::Reviewed::Owner { decision } => {
                        format!("gap review: needs-work twice; filed {decision} for the owner")
                    }
                },
            )
        }
        "resume" => {
            let id = required(args, "--decision")?;
            let commit = required(args, "--commit")?;
            let pin = flag(args, "--pin-note");
            let resumed = resume::resume(paths, &id, &commit, pin.as_deref()).map_err(show)?;
            Ok(format!(
                "gap resume: {} landed at {commit}; rerun {}",
                resumed.spec,
                resumed.reruns.join(" ")
            ))
        }
        "round" => {
            let species = required(args, "--species")?;
            let verdict = required(args, "--verdict")?;
            let note = flag(args, "--note").unwrap_or_default();
            Ok(
                match rounds::open_round(paths, &species, &verdict, &note).map_err(show)? {
                    rounds::Opened::Round(round) => format!("gap round: {verdict} round {round}"),
                    rounds::Opened::Spent { round, decision } => format!(
                        "gap round: {verdict} round {round} is past the bound; filed {decision} for the owner"
                    ),
                },
            )
        }
        "accept" => {
            let verdict = required(args, "--verdict")?;
            let round = rounds::accept(paths, &verdict).map_err(show)?;
            Ok(format!("gap accept: {verdict} accepts at round {round}"))
        }
        "metrics" => {
            let species = required(args, "--species")?;
            let record = metrics::write(paths, &species).map_err(show)?;
            Ok(format!(
                "gap metrics: {} gaps ({} routed), {:.2} taken by the loop, {} reversals, {} captures",
                record["autonomy"]["gaps"],
                record["autonomy"]["routed"],
                record["autonomy"]["share_taken"].as_f64().unwrap_or(0.0),
                record["quality"]["reversals"]
                    .as_array()
                    .map_or(0, Vec::len),
                record["efficiency"]["captures"],
            ))
        }
        other => Err(format!("unknown gap command {other}\n{USAGE}")),
    }
}

fn describe_route(routed: &routing::Routed) -> String {
    let chosen = routed.chosen.as_deref().unwrap_or("none");
    let filed = routed
        .decision
        .as_deref()
        .map(|id| format!("; filed {id}"))
        .unwrap_or_default();
    let next = match routed.route {
        Route::Proceed => "mint the spec",
        Route::Stronger => "the stronger model writes the set",
        Route::Owner => "the owner resolves it",
    };
    format!(
        "gap route: {} ({chosen}) - {}: {next}{filed}",
        routed.route.key(),
        routed.why
    )
}

fn show<E: std::fmt::Display>(err: E) -> String {
    err.to_string()
}

fn flag(args: &[String], name: &str) -> Option<String> {
    args.windows(2)
        .find(|pair| pair[0] == name)
        .map(|pair| pair[1].clone())
}

fn required(args: &[String], name: &str) -> Result<String, String> {
    flag(args, name).ok_or_else(|| format!("missing {name}\n{USAGE}"))
}

fn read_json_file(path: &str) -> Result<Value, String> {
    let text = std::fs::read_to_string(path).map_err(|err| format!("{path}: {err}"))?;
    serde_json::from_str(&text).map_err(|err| format!("{path}: {err}"))
}

/// The owner's written verdicts, one string per entry.
fn read_strings(path: &str) -> Result<Vec<String>, String> {
    let value = read_json_file(path)?;
    let list = value
        .get("verdicts")
        .cloned()
        .unwrap_or(value)
        .as_array()
        .cloned()
        .ok_or_else(|| format!("{path}: expected a JSON array of verdicts"))?;
    list.iter()
        .map(|item| {
            item.as_str()
                .map(str::to_string)
                .ok_or_else(|| format!("{path}: expected string entries"))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    fn argv(words: &[&str]) -> Vec<String> {
        words.iter().map(|w| w.to_string()).collect()
    }

    fn nowhere() -> Paths {
        Paths::new(Path::new("/nowhere"))
    }

    #[test]
    fn every_command_names_the_flag_it_lacks_before_it_reads_anything() {
        assert!(run(&nowhere(), &argv(&["gap", "open"]))
            .unwrap_err()
            .contains("usage"));
        let err = run(&nowhere(), &argv(&["gap", "open", "--dir", "/nowhere"])).unwrap_err();
        assert!(err.contains("missing --decision"), "{err}");
        let err = run(
            &nowhere(),
            &argv(&["gap", "round", "--dir", "/nowhere", "--species", "s"]),
        )
        .unwrap_err();
        assert!(err.contains("missing --verdict"), "{err}");
        let err = run(&nowhere(), &argv(&["gap", "fly", "--dir", "/nowhere"])).unwrap_err();
        assert!(err.contains("unknown gap command fly"), "{err}");
    }

    #[test]
    fn the_verdict_file_reads_a_bare_list_or_one_under_verdicts() {
        let dir =
            std::env::temp_dir().join(format!("jev-gap-cli-{}", crate::ledger::new_entry_id()));
        std::fs::create_dir_all(&dir).unwrap();
        let bare = dir.join("bare.json");
        std::fs::write(&bare, r#"["the curtain reaches the ground"]"#).unwrap();
        assert_eq!(
            read_strings(bare.to_str().unwrap()).unwrap(),
            vec!["the curtain reaches the ground"]
        );
        let wrapped = dir.join("wrapped.json");
        std::fs::write(&wrapped, r#"{"verdicts": ["one", "two"]}"#).unwrap();
        assert_eq!(read_strings(wrapped.to_str().unwrap()).unwrap().len(), 2);
        let bad = dir.join("bad.json");
        std::fs::write(&bad, r#"{"verdicts": [1]}"#).unwrap();
        assert!(read_strings(bad.to_str().unwrap())
            .unwrap_err()
            .contains("string entries"));
    }
}
