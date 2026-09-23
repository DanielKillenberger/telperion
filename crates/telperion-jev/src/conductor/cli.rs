//! `species-conductor <command> --config FILE`: the run's surface. One
//! command per hop or per record the host writes; each prints one line and
//! the action the run now waits on. Only `step` reaches Jev, and only
//! through the shared caller.
use std::path::PathBuf;

use serde_json::json;

use super::dispatch::read_result;
use super::questions::Asker;
use super::state::Run;
use super::step::{self, LiveExecutor};
use super::{cases, dependency, handoff, packet, plan, policy, report, tuning, Config};
use crate::caller::{load_key, UreqTransport};
use crate::pipeline::judge::Judge;

pub const USAGE: &str = "usage: species-conductor <command> --config FILE\n  \
    start                                     open or resume the run record\n  \
    status                                    the run's state and its next action\n  \
    step                                      one hop: execute what code can, print what it waits on\n  \
    dispatch --id ID --result FILE            record a dispatched agent's result\n  \
    attach --spec SPEC --gap GAP              attach the spec the owner minted for a packaged gap\n  \
    land --spec SPEC --commit SHA             record the host's landing of verified work\n  \
    resume --decision FILE                    resume a paused run; a tuning pause resumes the tuning run too\n  \
    packet                                    assemble the ready-for-review packet\n  \
    report                                    write the run's measurements\n  \
    cases                                     score the policy's labelled cases, no call";

pub fn run(args: &[String]) -> std::result::Result<String, String> {
    let command = args.first().cloned().unwrap_or_default();
    if command == "cases" {
        let scored = cases::score(&cases::load(), &policy::load());
        let table = cases::table(&scored);
        return if scored.iter().all(|r| r.agrees) {
            Ok(table)
        } else {
            Err(table)
        };
    }
    let config_path = flag(args, "--config").ok_or_else(|| USAGE.to_string())?;
    let config = Config::load(&PathBuf::from(config_path)).map_err(show)?;
    let existed = config.run_file().exists();
    let mut run = Run::open(&config).map_err(show)?;
    let word = match command.as_str() {
        "start" => {
            if existed {
                run.resumes += 1;
            }
            run.save(&config).map_err(show)?;
            format!(
                "run {}: {}",
                if existed { "resumed" } else { "started" },
                run.summary()
            )
        }
        "status" => {
            let next = plan::next(&config, &run).map_err(show)?;
            serde_json::to_string_pretty(&json!({"run": run.summary(), "next": next}))
                .unwrap_or_default()
        }
        "step" => {
            let key = load_key().unwrap_or_default();
            let transport = UreqTransport;
            let asker = Asker {
                judge: Judge {
                    transport: &transport,
                    key: &key,
                    ledger_dir: config.ledger_dir(),
                },
                config: &config,
            };
            let (word, next) =
                step::drive(&asker, &config, &mut run, &LiveExecutor).map_err(show)?;
            format!(
                "{word}\nnext: {}",
                serde_json::to_string(&next).unwrap_or_default()
            )
        }
        "dispatch" => {
            let id = required(args, "--id")?;
            let result = read_result(&PathBuf::from(required(args, "--result")?)).map_err(show)?;
            let outcome = run.ingest(&id, result).map_err(show)?;
            dependency::observe(&mut run, &id).map_err(show)?;
            minted_by_gap_loop(&mut run, &id);
            run.save(&config).map_err(show)?;
            format!("dispatch {id}: {outcome:?}")
        }
        "attach" => {
            let spec = required(args, "--spec")?;
            let gap = required(args, "--gap")?;
            let attached = handoff::attach(&mut run, &spec, &gap, "minted");
            run.save(&config).map_err(show)?;
            format!(
                "attach {spec} for {gap}: {}",
                if attached {
                    "attached"
                } else {
                    "already attached"
                }
            )
        }
        "land" => {
            let spec = required(args, "--spec")?;
            let commit = required(args, "--commit")?;
            dependency::land(&mut run, &spec, &commit).map_err(show)?;
            run.save(&config).map_err(show)?;
            format!(
                "land {spec} at {commit}: the affected stages and the next tuning revision rerun"
            )
        }
        "resume" => {
            let decision = PathBuf::from(required(args, "--decision")?);
            let word = tuning::resume(&config, &mut run, &LiveExecutor, &decision).map_err(show)?;
            run.save(&config).map_err(show)?;
            word
        }
        "packet" => {
            let word = packet::assemble(&config, &mut run).map_err(show)?;
            run.save(&config).map_err(show)?;
            word
        }
        "report" => {
            let report = report::write(&config, &run).map_err(show)?;
            format!(
                "report written to {}\n{}",
                config.report_file().display(),
                report::markdown(&report)
            )
        }
        _ => return Err(USAGE.into()),
    };
    Ok(word)
}

/// A verified gap-loop dispatch reports the spec the loop minted in
/// `observed`; the conductor attaches it as a minted dependency.
fn minted_by_gap_loop(run: &mut Run, id: &str) {
    let Some(dispatch) = run.dispatches.iter().find(|d| d.id == id).cloned() else {
        return;
    };
    let Some(rest) = dispatch.scope.strip_prefix("gap-loop:") else {
        return;
    };
    let decision = rest.split(':').next().unwrap_or_default().to_string();
    if let Some(result) = dispatch
        .result
        .filter(|r| r.outcome == Some(super::dispatch::Outcome::Verified))
    {
        let spec = result.observed.trim().to_string();
        if !spec.is_empty() {
            handoff::attach(run, &spec, &decision, "minted");
        }
    }
}

fn show<E: std::fmt::Display>(err: E) -> String {
    err.to_string()
}

fn flag(args: &[String], name: &str) -> Option<String> {
    args.windows(2)
        .find(|pair| pair[0] == name)
        .map(|pair| pair[1].clone())
}

fn required(args: &[String], name: &str) -> std::result::Result<String, String> {
    flag(args, name).ok_or_else(|| format!("missing {name}\n{USAGE}"))
}
