//! One tuning revision as the conductor carries it. The revision runs, and
//! what it leaves in its own `run.json` decides the rest: a pause there
//! becomes the conductor's pause under the same id, identity and action, so
//! one decision file resumes both; a revision that ended is recorded and its
//! gaps wait for their check. The tuning loop's own pauses and decisions are
//! untouched: the conductor only reads its record and passes the decision on.
use std::path::{Path, PathBuf};

use serde_json::json;

use super::questions::Asker;
use super::state::{Run, TuningRevision};
use super::step::Executor;
use super::{dependency, finish, gapcheck, handoff, now, ConductorError, Config, Result};
use crate::pipeline::canon::{canonical_sha256, read_json};
use crate::tuning::continuation::{Basis, Pause};
use crate::tuning::result::EndResult;

/// What the tuning run's own record says about where it stands.
struct Record {
    pause: Option<Pause>,
    pending: bool,
}

fn record(out: &Path) -> Result<Record> {
    let path = out.join("run.json");
    if !path.exists() {
        return Ok(Record {
            pause: None,
            pending: false,
        });
    }
    let raw = read_json(&path)?;
    let pause = serde_json::from_value(raw["pause"].clone())
        .map_err(|err| ConductorError::Invalid(format!("{}: pause: {err}", path.display())))?;
    Ok(Record {
        pause,
        pending: !raw["pending"].is_null(),
    })
}

/// A revision has ended when its result is written and its record holds
/// neither a pause nor an attempt in flight; `result.json` alone is not
/// enough, because the tuning loop rewrites it on every save.
fn ended(out: &Path) -> Result<bool> {
    let record = record(out)?;
    Ok(out.join("result.json").exists() && record.pause.is_none() && !record.pending)
}

pub fn tune(
    asker: &Asker<'_>,
    config: &Config,
    run: &mut Run,
    executor: &dyn Executor,
    revision: u64,
    focus: &[String],
) -> Result<String> {
    if run
        .budget
        .max_tuning_revisions
        .is_some_and(|cap| run.tuning.len() as u64 >= cap)
    {
        let basis = basis(run, revision, focus);
        handoff::pause(
            config,
            run,
            &format!("pause-tuning-{revision}"),
            "tuning revision cap reached",
            basis,
            &json!({"hard_limit": true}),
            "extend the tuning revision cap or park the species",
        )?;
        return Ok("paused: tuning revision cap".into());
    }
    // A scoped human resume of this revision's pause authorizes it; the trio
    // does not second-guess it, as on a dependency's attempt.
    let pause_id = format!("pause-tuning-{revision}");
    if revision > 1 && run.resumed_from.as_deref() == Some(pause_id.as_str()) {
        run.route(
            &format!("tuning:{revision}"),
            "tune",
            &format!("authorized by the scoped resume of {pause_id}"),
        );
        run.resumed_from = None;
    } else if revision > 1 {
        let basis = basis(run, revision, focus);
        let context = [format!("tuning revision {revision}")];
        match super::questions::continuation(asker, run, &basis, &context)? {
            Ok(_) => run.route(
                &format!("tuning:{revision}"),
                "tune",
                "continuation justified",
            ),
            Err((reason, assessment)) => {
                let judged = if assessment.is_some() {
                    "unjustified"
                } else {
                    "unavailable"
                };
                let signals = json!({"continuation": judged, "reason": reason});
                run.route(&format!("tuning:{revision}"), "human", &reason);
                handoff::pause(
                    config,
                    run,
                    &format!("pause-tuning-{revision}"),
                    &reason,
                    basis,
                    &signals,
                    "authorize another tuning revision or park the species",
                )?;
                return Ok(format!("paused: {reason}"));
            }
        }
    }
    let out = config.tuning_dir(revision);
    // A revision starts from the sourced profile's values; one already
    // under way keeps the overlay it started from, as its resume requires.
    let derived = match out.join("run.json").exists() || ended(&out)? {
        true => None,
        false => super::overlay::refresh(config)?,
    };
    let ran = if ended(&out)? {
        Ok(())
    } else {
        executor.tune(config, revision, focus, &out, None)
    };
    let settled = settle(config, run, revision, out, ran)?;
    Ok(match derived {
        Some(words) => format!("{words}\n{settled}"),
        None => settled,
    })
}

/// Resumes from the decision file. On a carried tuning pause the conductor's
/// pause is resolved by the same decision, the tuning run resumes with it in
/// its own directory, and what the run leaves is settled as a first run's
/// would be. A decision either side refuses leaves the run as it was.
pub fn resume(
    config: &Config,
    run: &mut Run,
    executor: &dyn Executor,
    decision: &Path,
) -> Result<String> {
    let Some(revision) = run.tuning_pause else {
        return handoff::resume(run, decision);
    };
    let before = run.clone();
    let resumed = resume_tuning(config, run, executor, decision, revision);
    if resumed.is_err() {
        *run = before;
    }
    resumed
}

fn resume_tuning(
    config: &Config,
    run: &mut Run,
    executor: &dyn Executor,
    decision: &Path,
    revision: u64,
) -> Result<String> {
    let id = run.pause.as_ref().map(|p| p.id.clone()).unwrap_or_default();
    let word = handoff::resume(run, decision)?;
    run.tuning_pause = None;
    let out = config.tuning_dir(revision);
    let ran = executor.tune(config, revision, &[], &out, Some(decision));
    if let Err(err) = &ran {
        if record(&out)?.pause.is_some_and(|p| p.id == id) {
            return Err(format!("tuning-loop kept pause {id}: {err}").into());
        }
    }
    let settled = settle(config, run, revision, out, ran)?;
    Ok(format!("{word}\n{settled}"))
}

/// Reads the tuning run's record after it exits: a converged stop is
/// recorded as a finish, any other pause is carried, an exit that left none
/// is the tool's failure, and an ended revision is recorded for its gap
/// check.
fn settle(
    config: &Config,
    run: &mut Run,
    revision: u64,
    out: PathBuf,
    ran: std::result::Result<(), String>,
) -> Result<String> {
    if let Some(pause) = record(&out)?.pause {
        // A stop that converged is a finish, not the host's (fn-136).
        let result = gapcheck::read_result(&out).ok();
        let why = result
            .as_ref()
            .and_then(|r| finish::converged(Some(&pause), r));
        return match (result, why) {
            (Some(result), Some(why)) => {
                run.route(&format!("tuning:{revision}"), "packet", &why);
                Ok(revised(run, revision, out, &result, Some(why)))
            }
            _ => carry(config, run, revision, &out, pause, ran.err()),
        };
    }
    ran.map_err(|err| ConductorError::Invalid(format!("tuning revision {revision}: {err}")))?;
    let result = gapcheck::read_result(&out)?;
    let why = finish::converged(None, &result);
    Ok(revised(run, revision, out, &result, why))
}

/// Records an ended revision for its gap check, or a converged one for the
/// packet, and says how it stopped.
fn revised(
    run: &mut Run,
    revision: u64,
    out: PathBuf,
    result: &EndResult,
    converged: Option<String>,
) -> String {
    let tokens = result.outcome.budget["tokens"].as_u64().unwrap_or(0);
    run.budget.tokens = run.budget.tokens.saturating_add(tokens);
    let landed_count = run
        .dependencies
        .iter()
        .filter(|d| d.landed_commit.is_some())
        .count();
    run.tuning.push(TuningRevision {
        revision,
        out,
        run_identity: result.outcome.run_identity.clone(),
        machine_ready: result.outcome.machine_ready,
        bootstrap: result.outcome.bootstrap,
        tokens,
        gaps: result.gaps.len(),
        landed_count,
        at: now(),
        converged: converged.clone(),
    });
    let word = format!(
        "tuning revision {revision}: {} ({} gaps listed)",
        result.outcome.stopped,
        result.gaps.len()
    );
    match converged {
        Some(why) => format!("{word}; {why}"),
        None => word,
    }
}

/// The tuning pause becomes the conductor's: its id, its identity, its
/// proposed action, its reason and its request, with a handoff.
fn carry(
    config: &Config,
    run: &mut Run,
    revision: u64,
    out: &Path,
    pause: Pause,
    exit: Option<String>,
) -> Result<String> {
    let basis = Basis {
        identity: pause.identity.clone(),
        ..pause.basis
    };
    let signals = json!({
        "tuning_pause": pause.id, "tuning_revision": revision,
        "tuning_run": out.join("run.json"), "tuning_exit": exit,
        "resume": "species-conductor resume --decision FILE runs tuning-loop run --resume FILE in the tuning_run directory",
    });
    run.route(&format!("tuning:{revision}"), "human", &pause.reason);
    handoff::pause(
        config,
        run,
        &pause.id,
        &pause.reason,
        basis,
        &signals,
        &pause.decision_requested,
    )?;
    run.tuning_pause = Some(revision);
    Ok(format!(
        "tuning revision {revision} paused {}: {}",
        pause.id, pause.reason
    ))
}

fn basis(run: &Run, revision: u64, focus: &[String]) -> Basis {
    let (next_tokens, estimate_basis) = dependency::estimate(run);
    Basis {
        identity: canonical_sha256(
            &json!({"tuning": revision, "focus": focus, "landed": run.dependencies.len()}),
        ),
        proposed_action: format!("tuning revision {revision}"),
        evidence: if focus.is_empty() {
            vec!["a landed dependency changed the generator; rebuild, render and reassess".into()]
        } else {
            focus
                .iter()
                .map(|d| format!("untried dial {d} judged reachable"))
                .collect()
        },
        recent_outcomes: run
            .tuning
            .iter()
            .map(|t| {
                format!(
                    "revision {}: machine_ready {}, {} gaps",
                    t.revision, t.machine_ready, t.gaps
                )
            })
            .collect(),
        next_tokens,
        estimate_basis,
        usage_known: run.budget.usage_known,
    }
}
