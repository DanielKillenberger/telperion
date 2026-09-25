//! The host adopts a dependency it built outside the conductor's dispatches.
//! fn-110 and fn-144 were designed and built by agents the host sent before
//! the conductor opened its own dispatch, and `land` refused them because
//! only a verified implementation dispatch sets `AwaitingLanding`; the run
//! kept re-judging finished work. Adoption records that implementation after
//! the fact, closes what the conductor had open for it, and lands it.
use super::dependency::{land, observe};
use super::dispatch::{Dispatch, DispatchResult, Outcome, Role};
use super::state::{DependencyStatus, Run};
use super::{now, Result};

pub const ROUTE: &str = "host_built";

/// Records a verified `host_built` implementation of `spec` from the host's
/// result and lands it at `commit`. Refuses an unknown spec, a dependency
/// that is not Designed or Awaiting, and a result that is not verified;
/// a refusal changes nothing.
pub fn adopt(
    run: &mut Run,
    spec: &str,
    commit: &str,
    mut result: DispatchResult,
) -> Result<String> {
    if commit.trim().is_empty() {
        return Err("the landing commit is empty".to_string().into());
    }
    let dependency = run
        .dependencies
        .iter()
        .find(|d| d.spec == spec)
        .cloned()
        .ok_or_else(|| format!("no dependency {spec}"))?;
    if !matches!(
        dependency.status,
        DependencyStatus::Designed | DependencyStatus::Awaiting
    ) {
        return Err(format!(
            "{spec} is {:?}; only a Designed or Awaiting dependency is adopted \
             (verified work awaiting landing lands with `land`)",
            dependency.status
        )
        .into());
    }
    if let Some(why) = unverified(&result) {
        return Err(format!("the result is not verified: {why}").into());
    }
    match (&result.design_revision, &dependency.design_revision) {
        (Some(named), Some(current)) if named != current => {
            return Err(
                format!("the result names design revision {named}; {spec} is at {current}").into(),
            )
        }
        (Some(named), None) => {
            return Err(
                format!("the result names design revision {named}; {spec} has no design").into(),
            )
        }
        _ => result.design_revision = dependency.design_revision.clone(),
    }
    let closed = close_open(run, spec, commit)?;
    let id = format!("dispatch-{}", run.dispatches.len() + 1);
    run.dispatches.push(Dispatch {
        id: id.clone(),
        role: Role::Implement,
        route: ROUTE.into(),
        tier: "host".into(),
        effort: result.actual_effort.clone(),
        dependency: Some(spec.into()),
        input_identity: result.input_identity.clone(),
        design_revision: dependency.design_revision.clone(),
        scope: format!("the host built {spec} outside the conductor's dispatches"),
        judgments: Vec::new(),
        reserved_tokens: Some(0),
        opened_at: now(),
        result: None,
    });
    if run.ingest(&id, result)? != Outcome::Verified {
        return Err(format!("{id} did not verify").into());
    }
    observe(run, &id)?;
    land(run, spec, commit)?;
    run.route(
        &format!("dependency:{spec}"),
        ROUTE,
        &format!("the host built it and adopted {id}; landed at {commit}"),
    );
    Ok(format!(
        "adopt {spec}: {id} recorded host_built and landed at {commit}{}",
        if closed.is_empty() {
            String::new()
        } else {
            format!("; closed {} as obsolete", closed.join(", "))
        }
    ))
}

/// Why a host's result does not count as verified work, if it does not.
fn unverified(result: &DispatchResult) -> Option<String> {
    if result.verification != "verified" {
        return Some(format!("verification is {:?}", result.verification));
    }
    if let Some(failure) = &result.failure {
        return Some(format!("it reports a failure: {failure}"));
    }
    if result.actual_model.trim().is_empty() || result.actual_effort.trim().is_empty() {
        return Some("it names no model or effort".into());
    }
    match result.outcome {
        None | Some(Outcome::Verified) => None,
        Some(other) => Some(format!("its outcome is {other:?}")),
    }
}

/// Closes every open dispatch of `spec` as obsolete, the way `ingest` marks
/// a result for another input identity, with a note naming the adoption.
fn close_open(run: &mut Run, spec: &str, commit: &str) -> Result<Vec<String>> {
    let open: Vec<(String, Option<String>)> = run
        .dispatches
        .iter()
        .filter(|d| d.open() && d.dependency.as_deref() == Some(spec))
        .map(|d| (d.id.clone(), d.design_revision.clone()))
        .collect();
    for (id, design_revision) in &open {
        let superseded = DispatchResult {
            input_identity: format!("{ROUTE}:{commit}"),
            design_revision: design_revision.clone(),
            actual_model: "none".into(),
            actual_effort: "none".into(),
            usage: None,
            cost_usd: None,
            wall_ms: None,
            verification: String::new(),
            observed: String::new(),
            handoff: None,
            failure: Some(format!(
                "obsolete: the host built {spec} outside the conductor and adopted it at {commit}"
            )),
            usage_is_reservation: false,
            outcome: None,
            finished_at: String::new(),
        };
        run.ingest(id, superseded)?;
    }
    Ok(open.into_iter().map(|(id, _)| id).collect())
}
