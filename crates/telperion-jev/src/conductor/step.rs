//! One hop of the run: the plan names the action, the step executes what
//! code can execute (a stage, a tuning revision, a gap check, a dispatch
//! opening, the packet) and returns what the run waits on. A repeated step
//! on the same state observes the same artifacts and creates nothing twice.
use std::path::PathBuf;
use std::process::Command;

use serde_json::json;

use super::dispatch::{Dispatch, Role};
use super::gapcheck::{self, Verdict};
use super::plan::{self, Next};
use super::questions::Asker;
use super::state::{Run, TuningRevision};
use super::{dependency, handoff, now, packet, Config, Result};
use crate::pipeline::canon::{canonical_sha256, read_json};
use crate::pipeline::stage::STAGES;
use crate::tuning::continuation::Basis;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StageOutcome {
    Current,
    Ran,
}

/// How the runbook's commands are run. The live executor spawns the
/// binaries the config names; a test scripts the outcomes.
pub trait Executor {
    /// `Err` is the stage's own stop message: a decision, a missing input, a
    /// tool failure. The step reads the decisions after it.
    fn stage(&self, config: &Config, stage: &str) -> std::result::Result<StageOutcome, String>;
    /// Runs one tuning revision into `out` and returns once `result.json` exists.
    fn tune(
        &self,
        config: &Config,
        revision: u64,
        focus: &[String],
        out: &PathBuf,
    ) -> std::result::Result<(), String>;
}

pub struct LiveExecutor;

impl Executor for LiveExecutor {
    fn stage(&self, config: &Config, stage: &str) -> std::result::Result<StageOutcome, String> {
        let output = Command::new(&config.species_pipeline)
            .arg(stage)
            .args(["--dir", &config.dir.display().to_string()])
            .args(["--run-dir", &config.run_dir.display().to_string()])
            .args(&config.stage_args)
            .output()
            .map_err(|err| format!("{}: {err}", config.species_pipeline.display()))?;
        let stdout = String::from_utf8_lossy(&output.stdout);
        if !output.status.success() {
            return Err(String::from_utf8_lossy(&output.stderr).trim().to_string());
        }
        Ok(if stdout.contains("current") {
            StageOutcome::Current
        } else {
            StageOutcome::Ran
        })
    }

    fn tune(
        &self,
        config: &Config,
        _revision: u64,
        _focus: &[String],
        out: &PathBuf,
    ) -> std::result::Result<(), String> {
        let status = Command::new(&config.tuning_loop)
            .arg("run")
            .args(["--config", &config.tuning_config.display().to_string()])
            .args(["--out", &out.display().to_string()])
            .status()
            .map_err(|err| format!("{}: {err}", config.tuning_loop.display()))?;
        if !status.success() {
            return Err(format!("tuning-loop run exited {status}"));
        }
        Ok(())
    }
}

/// Runs the stages from `from` in order; stops at the first that stops.
fn stages(config: &Config, run: &mut Run, executor: &dyn Executor, from: &str) -> Result<String> {
    let start = STAGES.iter().position(|s| *s == from).unwrap_or(0);
    let mut words = Vec::new();
    for commit in run
        .dependencies
        .iter()
        .filter_map(|d| d.landed_commit.clone())
        .collect::<Vec<_>>()
    {
        if !run.stages_rerun_for.contains(&commit) {
            run.stages_rerun_for.push(commit);
        }
    }
    for stage in &STAGES[start..] {
        match executor.stage(config, stage) {
            Ok(StageOutcome::Current) => words.push(format!("{stage}: current")),
            Ok(StageOutcome::Ran) => words.push(format!("{stage}: ran")),
            Err(message) => {
                words.push(format!("{stage}: stopped: {message}"));
                return Ok(words.join("; "));
            }
        }
    }
    run.stage_fingerprint = Some(plan::stage_fingerprint(config, run));
    Ok(words.join("; "))
}

/// Opens the routine dispatch a cheap agent resolves under policy: a
/// decision with the options it may choose, or the runbook's gap loop.
fn routine(run: &mut Run, scope: String, input: &serde_json::Value) -> String {
    let id = format!("dispatch-{}", run.dispatches.len() + 1);
    run.resumed_from = None;
    run.dispatches.push(Dispatch {
        id: id.clone(),
        role: Role::Routine,
        route: "routine".into(),
        tier: "cheap".into(),
        effort: "default".into(),
        dependency: None,
        input_identity: canonical_sha256(input),
        design_revision: None,
        scope,
        judgments: Vec::new(),
        reserved_tokens: run.budget.attempt_max_tokens,
        opened_at: now(),
        result: None,
    });
    format!("dispatch {id}: routine on cheap")
}

fn tune(
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
        let basis = tuning_basis(run, revision, focus);
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
    if revision > 1 {
        let basis = tuning_basis(run, revision, focus);
        match super::questions::continuation(
            asker,
            run,
            &basis,
            &[format!("tuning revision {revision}")],
        )? {
            Ok(_) => run.route(
                &format!("tuning:{revision}"),
                "tune",
                "continuation justified",
            ),
            Err((reason, assessment)) => {
                let signals = json!({"continuation": if assessment.is_some() { "unjustified" } else { "unavailable" }, "reason": reason});
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
    if !out.join("result.json").exists() {
        executor.tune(config, revision, focus, &out)?;
    }
    let result = gapcheck::read_result(&out)?;
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
    });
    Ok(format!(
        "tuning revision {revision}: {} ({} gaps listed)",
        result.outcome.stopped,
        result.gaps.len()
    ))
}

fn tuning_basis(run: &Run, revision: u64, focus: &[String]) -> Basis {
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

fn gap_check(
    asker: &Asker<'_>,
    config: &Config,
    run: &mut Run,
    revision: u64,
    gaps: &[String],
) -> Result<String> {
    let out = config.tuning_dir(revision);
    let result = gapcheck::read_result(&out)?;
    let mut words = Vec::new();
    let mut new_gaps = Vec::new();
    for entry in result.gaps.iter().filter(|g| gaps.contains(&g.id)) {
        let Some(checked) = gapcheck::check(asker, run, &result, entry, revision)? else {
            continue;
        };
        match &checked.verdict {
            Verdict::Reachable { dials } => {
                words.push(format!("{}: reachable with {}", entry.id, dials.join(",")))
            }
            Verdict::Covered { spec } => {
                handoff::attach(run, spec, &entry.id, "existing");
                words.push(format!("{}: covered by {spec}", entry.id));
            }
            Verdict::New { package } => {
                new_gaps.push(package.display().to_string());
                words.push(format!(
                    "{}: new, packaged at {}",
                    entry.id,
                    package.display()
                ));
            }
        }
    }
    if !new_gaps.is_empty() {
        let (next_tokens, estimate_basis) = dependency::estimate(run);
        let basis = Basis {
            identity: canonical_sha256(&json!({"new_gaps": new_gaps, "revision": revision})),
            proposed_action: handoff::REQUESTED_ON_NEW_GAP.into(),
            evidence: new_gaps
                .iter()
                .map(|p| format!("gap package {p}"))
                .collect(),
            recent_outcomes: words.clone(),
            next_tokens,
            estimate_basis,
            usage_known: run.budget.usage_known,
        };
        run.route(
            &format!("gaps:{revision}"),
            "human",
            "new gap escalates to the host",
        );
        handoff::pause(
            config,
            run,
            &format!("pause-gaps-{revision}"),
            "new gap: its cause and the shape of its spec are the host's",
            basis,
            &json!({"new_gaps": new_gaps}),
            handoff::REQUESTED_ON_NEW_GAP,
        )?;
    }
    Ok(words.join("; "))
}

/// Executes one hop and returns the action the run now waits on.
pub fn drive(
    asker: &Asker<'_>,
    config: &Config,
    run: &mut Run,
    executor: &dyn Executor,
) -> Result<(String, Next)> {
    let next = plan::next(config, run)?;
    let word = match &next {
        Next::Paused { id, reason } => format!("paused {id}: {reason}"),
        Next::AwaitDispatch {
            id,
            role,
            tier,
            effort,
        } => {
            run.wait(&format!("dispatch {id}"));
            format!("awaiting {id} ({role} on {tier} at {effort})")
        }
        Next::AwaitOwner { decision, kind } => {
            run.wait(&format!("owner: {decision}"));
            format!("awaiting the owner: {decision} ({kind})")
        }
        Next::AwaitLanding { spec } => {
            run.wait(&format!("landing: {spec}"));
            format!(
                "awaiting the landing of {spec}: the host's authority, recorded as an exception"
            )
        }
        Next::Routine {
            decision,
            kind,
            options,
        } => {
            let input = json!({"decision": decision, "options": options});
            routine(
                run,
                format!(
                    "resolve {decision} ({kind}) with one of {}",
                    options.join("|")
                ),
                &input,
            )
        }
        Next::GapLoop { decision } => {
            let input =
                read_json(&config.paths().decisions()).unwrap_or(json!({"decision": decision}));
            routine(run, format!("gap-loop:{decision}: open, write the options, route; report the spec it minted in observed, or failure with the route"), &input)
        }
        Next::Dependency { spec, .. } => dependency::advance(asker, config, run, spec)?,
        Next::Stages { from } => stages(config, run, executor, from)?,
        Next::Tune { revision, focus } => tune(asker, config, run, executor, *revision, focus)?,
        Next::GapCheck { revision, gaps } => gap_check(asker, config, run, *revision, gaps)?,
        Next::Packet => packet::assemble(config, run)?,
        Next::Ready => "ready for the owner's review; only their verdict accepts".into(),
    };
    if !matches!(
        next,
        Next::Paused { .. }
            | Next::AwaitDispatch { .. }
            | Next::AwaitOwner { .. }
            | Next::AwaitLanding { .. }
    ) {
        run.end_wait();
    }
    run.save(config)?;
    Ok((word, plan::next(config, run)?))
}
