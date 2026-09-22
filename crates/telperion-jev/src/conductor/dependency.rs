//! A gap spec's path from attached to landed (R3, R4): Jev judges design
//! complexity over the spec and the gap evidence; a design dispatch produces
//! the handoff; Jev judges the remaining implementation complexity over that
//! handoff alone; an implementation dispatch produces verified work that
//! waits on the host's landing authority. Every dispatch passes the shared
//! continuation check first (R8), and every route the policy cannot justify
//! pauses for the human.
use serde_json::{json, Value};

use super::dispatch::{Dispatch, Outcome, Role};
use super::handoff;
use super::policy::{self, Signals};
use super::questions::{self, Asker};
use super::state::{Dependency, DependencyStatus, Run};
use super::{now, Config, Result};
use crate::pipeline::canon::{canonical_sha256, file_sha256, read_json};
use crate::tuning::continuation::Basis;

/// The spec's text and record, bounded to what a judgment needs.
pub fn spec_evidence(config: &Config, spec: &str) -> Result<Value> {
    let record = read_json(&config.flow.join("specs").join(format!("{spec}.json")))?;
    let body = std::fs::read_to_string(config.flow.join("specs").join(format!("{spec}.md")))
        .unwrap_or_default();
    Ok(json!({
        "id": spec,
        "title": record["title"],
        "status": record["status"],
        "body": body,
    }))
}

/// The gap evidence the run holds for this dependency: the package a new
/// gap left, or the check that tied the gap to an existing spec.
pub fn gap_evidence(config: &Config, run: &Run, dependency: &Dependency) -> Value {
    let package = config
        .gaps_dir()
        .join(format!("{}.json", dependency.gap_id));
    if package.exists() {
        return read_json(&package).unwrap_or(Value::Null);
    }
    run.gap_checks
        .get(&dependency.gap_id)
        .cloned()
        .unwrap_or(Value::Null)
}

/// The dispatches this dependency has taken, oldest first.
pub fn history<'a>(run: &'a Run, spec: &str) -> Vec<&'a Dispatch> {
    run.dispatches
        .iter()
        .filter(|d| d.dependency.as_deref() == Some(spec))
        .collect()
}

fn outcomes(history: &[&Dispatch]) -> Vec<String> {
    history
        .iter()
        .map(|d| {
            format!(
                "{} ({} at {}): {}",
                d.route,
                d.tier,
                d.effort,
                d.result
                    .as_ref()
                    .map(|r| {
                        let outcome = r
                            .outcome
                            .map(|o| format!("{o:?}").to_lowercase())
                            .unwrap_or_default();
                        match &r.failure {
                            Some(f) => format!("{outcome}: {f}"),
                            None => outcome,
                        }
                    })
                    .unwrap_or_else(|| "open".into())
            )
        })
        .collect()
}

/// Equivalent attempts without a verified result, counted from the end.
fn no_progress(history: &[&Dispatch], limit: usize) -> bool {
    let trailing = history
        .iter()
        .rev()
        .take_while(|d| d.outcome().is_some_and(|o| o != Outcome::Verified))
        .count();
    trailing >= limit
}

fn hard_limit(run: &Run) -> bool {
    run.dispatches.len() as u64 >= run.budget.max_dispatches
        || run
            .budget
            .tokens
            .saturating_add(run.budget.attempt_max_tokens)
            > run.budget.max_tokens
}

/// The bounded next-attempt estimate: the mean of the finished dispatches
/// that reported usage, capped by the attempt bound; the bound itself when
/// nothing has run. The basis names which.
pub fn estimate(run: &Run) -> (u64, String) {
    let known: Vec<u64> = run
        .dispatches
        .iter()
        .filter_map(|d| d.result.as_ref()?.usage.as_ref())
        .map(|u| u.input_tokens.saturating_add(u.output_tokens))
        // A zero is a driver that could not count, not a free attempt: the
        // first live run's one known usage was 0 and the mean priced the next
        // attempt at one token, which the continuation check rightly refused.
        .filter(|total| *total > 0)
        .collect();
    if known.is_empty() {
        return (
            run.budget.attempt_max_tokens,
            "the configured attempt bound; no dispatch has reported usage yet".into(),
        );
    }
    let mean = known.iter().sum::<u64>() / known.len() as u64;
    let bounded = mean.min(run.budget.attempt_max_tokens);
    (
        bounded.max(1),
        format!(
            "mean of {} dispatches with known usage, capped by the attempt bound",
            known.len()
        ),
    )
}

/// What the last finished dispatch on this dependency verified, as a signal.
fn verification(history: &[&Dispatch]) -> (String, String) {
    match history.iter().rev().find(|d| d.result.is_some()) {
        Some(d) => (
            match d.outcome() {
                Some(Outcome::Verified) => "verified",
                Some(Outcome::Failed) => "failed",
                _ => "none",
            }
            .into(),
            d.route.clone(),
        ),
        None => ("none".into(), "none".into()),
    }
}

/// One judgment call: design complexity before a design exists, remaining
/// implementation complexity once the handoff does. The confidence floor is
/// the policy's; a low-confidence choice reads as insufficient evidence.
fn judge(
    asker: &Asker<'_>,
    run: &mut Run,
    config: &Config,
    dependency: &Dependency,
    table: &policy::Table,
) -> Result<(String, String, Vec<String>)> {
    let designed = dependency.status == DependencyStatus::Designed;
    let spec = spec_evidence(config, &dependency.spec)?;
    let key = if designed {
        format!("implementation:{}", dependency.design_revision.clone().unwrap_or_default())
    } else {
        format!("design:{}", canonical_sha256(&spec))
    };
    if let Some(memo) = dependency.judged.get(&key) {
        let (choice, ledger) = memo.split_once('|').unwrap_or((memo, ""));
        return Ok(if designed {
            ("not_asked".into(), choice.into(), vec![ledger.into()])
        } else {
            (choice.into(), "not_asked".into(), vec![ledger.into()])
        });
    }
    let gap = gap_evidence(config, run, dependency);
    let investigations: Vec<String> = history(run, &dependency.spec)
        .iter()
        .filter(|d| d.role == Role::Investigate)
        .filter_map(|d| d.result.as_ref().map(|r| r.observed.clone()))
        .filter(|o| !o.trim().is_empty())
        .collect();
    if designed {
        let path = dependency.handoff.clone().unwrap_or_default();
        let text = std::fs::read_to_string(&path).unwrap_or_default();
        let state = json!({"spec": spec, "gap": gap, "design_handoff": text,
                           "design_revision": dependency.design_revision, "investigations": investigations});
        let judgment = asker.ask(
            run,
            "conductor-implementation",
            &state,
            &questions::implementation_questions(),
        )?;
        let choice = policy::thresholded(
            judgment.entry.choice("implementation_complexity"),
            judgment.entry.confidence("implementation_complexity"),
            table.min_confidence,
        );
        remember(run, &dependency.spec, &key, &choice, &judgment.reference);
        return Ok(("not_asked".into(), choice, vec![judgment.reference]));
    }
    let state = json!({"spec": spec, "gap": gap, "investigations": investigations});
    let judgment = asker.ask(
        run,
        "conductor-design",
        &state,
        &questions::design_questions(),
    )?;
    let choice = policy::thresholded(
        judgment.entry.choice("design_complexity"),
        judgment.entry.confidence("design_complexity"),
        table.min_confidence,
    );
    remember(run, &dependency.spec, &key, &choice, &judgment.reference);
    Ok((choice, "not_asked".into(), vec![judgment.reference]))
}

/// Keeps a judgment with the evidence it read, so the same evidence is never
/// judged twice. An `insufficient_evidence` answer is not kept: new evidence
/// may settle it, and the next step asks again.
fn remember(run: &mut Run, spec: &str, key: &str, choice: &str, ledger: &str) {
    if choice == "insufficient_evidence" {
        return;
    }
    if let Some(d) = run.dependency_mut(spec) {
        d.judged.insert(key.into(), format!("{choice}|{ledger}"));
    }
}

/// The next hop on a dependency: judge, route, check continuation, and open
/// the dispatch or pause. Returns what the run waits on now.
pub fn advance(asker: &Asker<'_>, config: &Config, run: &mut Run, spec: &str) -> Result<String> {
    let table = policy::load();
    let dependency = run
        .dependencies
        .iter()
        .find(|d| d.spec == spec)
        .cloned()
        .ok_or_else(|| format!("no dependency {spec}"))?;
    let past = history(run, spec);
    let (verification, last_route) = verification(&past);
    let no_progress = no_progress(&past, table.no_progress_attempts);
    let hard = hard_limit(run);
    let recent = outcomes(&past);
    let first_attempt = past.is_empty();
    drop(past);
    let (design_complexity, implementation_complexity, mut judgments) = if hard || no_progress {
        ("not_asked".into(), "not_asked".into(), Vec::new())
    } else {
        judge(asker, run, config, &dependency, &table)?
    };
    let mut signals = Signals {
        hard_limit: hard,
        continuation: "justified".into(),
        no_progress,
        design_present: dependency.status == DependencyStatus::Designed,
        design_complexity,
        implementation_complexity,
        verification,
        last_route,
    };
    let mut decided = table.route(&signals.value());
    let context = format!("dependency:{spec}");
    let (next_tokens, estimate_basis) = estimate(run);
    let basis = Basis {
        identity: canonical_sha256(&json!({"spec": spec, "status": dependency.status,
            "dispatches": run.dispatches.len(), "revision": dependency.design_revision})),
        proposed_action: format!("{}: {}", decided.route, decided.why),
        evidence: vec![
            format!("spec {spec} ({:?})", dependency.status),
            format!("gap {}", dependency.gap_id),
            format!("judgments {}", judgments.join(",")),
        ],
        recent_outcomes: recent,
        next_tokens: Some(next_tokens),
        estimate_basis,
        usage_known: run.budget.usage_known,
    };
    // A first attempt on a dependency is bounded by construction: one
    // dispatch, the attempt bound, a route the table justified. Nothing has
    // been tried, so the continuation trio has no progress, risk or
    // tractability to read and can only answer insufficient evidence; the
    // first live run paused on exactly that. As fn-68's R11 settled for the
    // tuning loop, only a repeat asks.
    if !decided.human() && first_attempt {
        decided.why = format!("{} (first attempt within the bound; no continuation question)", decided.why);
    }
    // A scoped human decision that resumed the run authorizes the attempt
    // it named; the trio does not second-guess it. Opening the attempt
    // clears the authorization, so the one after asks again.
    let authorized = run.resumed_from.clone();
    if !decided.human() && !first_attempt && authorized.is_some() {
        decided.why = format!(
            "{} (attempt authorized by the scoped resume of {}; no continuation question)",
            decided.why,
            authorized.clone().unwrap_or_default()
        );
    }
    if !decided.human() && !first_attempt && authorized.is_none() {
        let risks = vec![format!(
            "route {} on tier {}",
            decided.route,
            table
                .allocation(&decided.route)
                .map(|a| a.tier.as_str())
                .unwrap_or("none")
        )];
        match questions::continuation(asker, run, &basis, &risks)? {
            Ok(assessment) => judgments.push(assessment.ledger),
            Err((reason, assessment)) => {
                signals.continuation = if assessment.is_none() {
                    "unavailable"
                } else {
                    "unjustified"
                }
                .into();
                if let Some(a) = assessment {
                    judgments.push(a.ledger);
                }
                decided = table.route(&signals.value());
                decided.why = format!("{} ({reason})", decided.why);
            }
        }
    }
    run.route(&context, &decided.route, &decided.why);
    let record = policy::record(&decided, &signals, &table);
    if decided.human() {
        let id = format!("pause-{}", run.authorizations.len() + run.waits.len() + 1);
        handoff::pause(
            config,
            run,
            &id,
            &decided.why,
            basis,
            &record,
            "continue, redesign, reassign or park this dependency",
        )?;
        return Ok(format!("paused {id}: {}", decided.why));
    }
    let allocation = table
        .allocation(&decided.route)
        .cloned()
        .expect("routes are listed");
    let id = format!("dispatch-{}", run.dispatches.len() + 1);
    let input_identity = basis.identity.clone();
    run.resumed_from = None;
    run.dispatches.push(Dispatch {
        id: id.clone(),
        role: allocation.role,
        route: decided.route.clone(),
        tier: allocation.tier.clone(),
        effort: allocation.effort.clone(),
        dependency: Some(spec.into()),
        input_identity,
        design_revision: dependency.design_revision.clone(),
        scope: scope(&decided.route, spec, &dependency),
        judgments,
        reserved_tokens: next_tokens,
        opened_at: now(),
        result: None,
    });
    Ok(format!(
        "dispatch {id}: {} on {} at {}",
        decided.route, allocation.tier, allocation.effort
    ))
}

fn scope(route: &str, spec: &str, dependency: &Dependency) -> String {
    match route {
        "design" | "design_again" => format!("write the implementation-ready design handoff for {spec}: interfaces, invariants, difficult cases, verification expectations and the remaining unknowns, named"),
        "investigate" => format!("gather the missing evidence for {spec}: what the change touches, in the code, with file references; decide nothing"),
        "routine" => format!("implement {spec} as written; no design decision is open"),
        _ => format!("implement {spec} from the design handoff {} (revision {})",
            dependency.handoff.as_ref().map(|p| p.display().to_string()).unwrap_or_default(),
            dependency.design_revision.clone().unwrap_or_default()),
    }
}

/// Moves the dependency on from a finished dispatch. A verified design
/// records its handoff by content hash, so a later result naming another
/// revision is obsolete; verified implementation waits on landing.
pub fn observe(run: &mut Run, dispatch_id: &str) -> Result<()> {
    let Some(dispatch) = run.dispatches.iter().find(|d| d.id == dispatch_id).cloned() else {
        return Ok(());
    };
    let (Some(spec), Some(result)) = (dispatch.dependency.clone(), dispatch.result.clone()) else {
        return Ok(());
    };
    if result.outcome != Some(Outcome::Verified) {
        return Ok(());
    }
    let Some(dependency) = run.dependency_mut(&spec) else {
        return Ok(());
    };
    match dispatch.role {
        Role::Design => {
            let path = result
                .handoff
                .ok_or_else(|| "a verified design names its handoff file".to_string())?;
            let revision = file_sha256(&path)?;
            dependency.handoff = Some(path);
            dependency.design_revision = Some(revision);
            dependency.status = DependencyStatus::Designed;
        }
        Role::Implement | Role::Routine => {
            dependency.status = DependencyStatus::AwaitingLanding;
        }
        _ => {}
    }
    Ok(())
}

/// Records the host's landing of a dependency at a commit. Only verified
/// work lands; the conductor never treats a Jev answer as permission.
pub fn land(run: &mut Run, spec: &str, commit: &str) -> Result<()> {
    if commit.trim().is_empty() {
        return Err("the landing commit is empty".to_string().into());
    }
    let dependency = run
        .dependency_mut(spec)
        .ok_or_else(|| format!("no dependency {spec}"))?;
    if dependency.status != DependencyStatus::AwaitingLanding {
        return Err(format!(
            "{spec} is {:?}; only verified work awaiting landing lands",
            dependency.status
        )
        .into());
    }
    dependency.status = DependencyStatus::Landed;
    dependency.landed_commit = Some(commit.into());
    Ok(())
}
