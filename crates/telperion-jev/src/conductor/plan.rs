//! What the run does next, read from the artifacts and the run record with
//! no call and no side effect. Stage eligibility, decision ownership, the
//! dependency in flight, the tuning revision and the gap verdicts are all
//! code's; the step executes the action and Jev is asked only there.
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use super::gapcheck::{self, Verdict, PASSING};
use super::policy;
use super::state::{DependencyStatus, Run};
use super::{Config, Result};
use crate::pipeline::build_id::{BUILD_ID, BUILD_TOOL};
use crate::pipeline::canon::{canonical_sha256, file_sha256, read_json};
use crate::pipeline::consume::REQUIREMENTS_UNMET;
use crate::pipeline::decision::{reconcile, Decision, Status};
use crate::pipeline::gap::HALT_KINDS;
use crate::pipeline::search;
use crate::pipeline::stage::STAGES;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "action", rename_all = "snake_case")]
pub enum Next {
    /// A human decision is required; nothing dispatches until it names the pause.
    Paused { id: String, reason: String },
    /// A dispatched agent has not returned; a repeated step waits, it never re-dispatches.
    AwaitDispatch {
        id: String,
        role: String,
        tier: String,
        effort: String,
    },
    /// A decision only the owner resolves.
    AwaitOwner { decision: String, kind: String },
    /// Open owner-only decisions (fn-127): the run pauses with their handoff
    /// before the gap loop, the stages or tuning.
    OwnerFirst { decisions: Vec<String> },
    /// Requirements the pipeline searches again for before the owner has
    /// them (fn-129): one round each.
    SearchAgain { decisions: Vec<String> },
    /// A decision policy lets the cheap agent resolve, with the options it may choose.
    Routine {
        decision: String,
        kind: String,
        options: Vec<String>,
    },
    /// A pipeline halt the runbook's gap loop takes.
    GapLoop { decision: String },
    /// Verified work waits on the host's landing authority.
    AwaitLanding { spec: String },
    /// A gap spec needs its next judgment and dispatch.
    Dependency {
        spec: String,
        status: DependencyStatus,
    },
    /// The stages from `from` are run in order; a current stage does nothing.
    Stages { from: String },
    /// One tuning revision, with the untried dials the gap check named.
    Tune { revision: u64, focus: Vec<String> },
    /// The latest revision's gaps still wait for their check.
    GapCheck { revision: u64, gaps: Vec<String> },
    /// Every automated prerequisite holds; the packet is assembled.
    Packet,
    /// The packet is written; only the owner's verdict remains.
    Ready,
}

/// Decision kinds that stop the run before any other work: a manifest the
/// pipeline could not admit, a requirement its own searches did not meet.
/// Open, each is the owner's, except a requirement with a search round left.
pub const OWNER_FIRST: [&str; 2] = ["manifest-proposed", REQUIREMENTS_UNMET];

/// The stages' current fingerprint: the manifest, the resolutions and every
/// landed fix. When it matches the one recorded after a full pass, the
/// stages are current and nothing is rerun.
pub fn stage_fingerprint(config: &Config, run: &Run) -> String {
    let paths = config.paths();
    let landed: Vec<Value> = run
        .dependencies
        .iter()
        .filter_map(|d| {
            d.landed_commit
                .as_ref()
                .map(|c| json!({"spec": d.spec, "commit": c}))
        })
        .collect();
    canonical_sha256(&json!({
        "manifest": file_sha256(&paths.manifest()).unwrap_or_default(),
        "resolutions": file_sha256(&paths.resolutions()).unwrap_or_default(),
        "landed": landed,
    }))
}

/// Open decisions in the species folder, reconciled with the resolutions.
pub fn open_decisions(config: &Config) -> Result<Vec<Decision>> {
    let paths = config.paths();
    if !paths.decisions().exists() {
        return Ok(Vec::new());
    }
    let decisions =
        reconcile(&paths).map_err(|err| super::ConductorError::Invalid(err.to_string()))?;
    Ok(decisions
        .into_iter()
        .filter(|d| d.status == Status::Open)
        .collect())
}

/// Who a decision belongs to: the gap loop, the cheap agent under policy, or
/// the owner. Anything the policy does not name is the owner's by rule.
pub fn decision_action(table: &policy::Table, run: &Run, decision: &Decision) -> Next {
    if HALT_KINDS.contains(&decision.kind.as_str()) {
        // The loop ran once for this halt and did not mint a spec: its route
        // was the owner's or the stronger model's, so the halt is theirs now.
        let looped = run
            .dispatches
            .iter()
            .any(|d| d.scope.starts_with(&format!("gap-loop:{}:", decision.id)));
        return if looped {
            Next::AwaitOwner {
                decision: decision.id.clone(),
                kind: decision.kind.clone(),
            }
        } else {
            Next::GapLoop {
                decision: decision.id.clone(),
            }
        };
    }
    if let Some(options) = table.decisions.routine.get(&decision.kind) {
        // A routine dispatch that came back without resolving the decision
        // does not get a second; the decision is the owner's now.
        let tried = run
            .dispatches
            .iter()
            .any(|d| !d.open() && d.scope.starts_with(&format!("resolve {} ", decision.id)));
        if !tried {
            return Next::Routine {
                decision: decision.id.clone(),
                kind: decision.kind.clone(),
                options: options.clone(),
            };
        }
    }
    Next::AwaitOwner {
        decision: decision.id.clone(),
        kind: decision.kind.clone(),
    }
}

fn first_missing_stage(config: &Config) -> Option<&'static str> {
    let paths = config.paths();
    STAGES
        .iter()
        .copied()
        .find(|stage| !paths.artifact(stage).exists())
}

/// The first stage whose artifact records a build other than this one: a
/// code change expired its key (fn-132). An artifact that records no build
/// predates the build id and is left to the stage fingerprint.
pub fn first_stale_stage(config: &Config) -> Option<&'static str> {
    let paths = config.paths();
    STAGES.iter().copied().find(|stage| {
        read_json(&paths.artifact(stage))
            .ok()
            .and_then(|v| v["tools"][BUILD_TOOL].as_str().map(|b| b != BUILD_ID))
            .unwrap_or(false)
    })
}

/// The gap verdicts of the latest revision, read from the run record.
pub fn verdicts(run: &Run, revision: u64) -> Vec<(String, Verdict)> {
    run.gap_checks
        .values()
        .filter(|v| v["revision"].as_u64() == Some(revision))
        .filter_map(|v| {
            let checked: gapcheck::Checked = serde_json::from_value(v.clone()).ok()?;
            Some((checked.gap_id, checked.verdict))
        })
        .collect()
}

pub fn next(config: &Config, run: &Run) -> Result<Next> {
    if let Some(pause) = &run.pause {
        return Ok(Next::Paused {
            id: pause.id.clone(),
            reason: pause.reason.clone(),
        });
    }
    if let Some(dispatch) = run.open_dispatch() {
        return Ok(Next::AwaitDispatch {
            id: dispatch.id.clone(),
            role: dispatch.role.key().into(),
            tier: dispatch.tier.clone(),
            effort: dispatch.effort.clone(),
        });
    }
    let open = open_decisions(config)?;
    let proposal_open = open.iter().any(|d| d.kind == OWNER_FIRST[0]);
    // Stages a code change left stale read the literature again before a
    // search adds to it or the owner is asked for it (fn-132); once per
    // build, so a stage that stops cannot hold the run.
    if !proposal_open && run.stages_build.as_deref() != Some(BUILD_ID) {
        if let Some(stage) = first_stale_stage(config) {
            return Ok(Next::Stages { from: stage.into() });
        }
    }
    // A search waits while a manifest proposal is open: the owner's
    // admission would overwrite the sources a search added.
    let again = if proposal_open {
        Vec::new()
    } else {
        search::searchable(&config.paths(), &open)
    };
    if !again.is_empty() {
        return Ok(Next::SearchAgain { decisions: again });
    }
    let owners: Vec<String> = open
        .iter()
        .filter(|d| OWNER_FIRST.contains(&d.kind.as_str()))
        .map(|d| d.id.clone())
        .collect();
    if !owners.is_empty() {
        return Ok(Next::OwnerFirst { decisions: owners });
    }
    if let Some(dependency) = run
        .dependencies
        .iter()
        .find(|d| d.status != DependencyStatus::Landed)
    {
        return Ok(match dependency.status {
            DependencyStatus::AwaitingLanding | DependencyStatus::Implemented => {
                Next::AwaitLanding {
                    spec: dependency.spec.clone(),
                }
            }
            status => Next::Dependency {
                spec: dependency.spec.clone(),
                status,
            },
        });
    }
    let table = policy::load();
    if let Some(decision) = open.iter().find(|d| !d.blocks.is_empty()) {
        return Ok(decision_action(&table, run, decision));
    }
    if let Some(stage) = first_missing_stage(config) {
        return Ok(Next::Stages { from: stage.into() });
    }
    if run.stage_fingerprint.as_deref() != Some(stage_fingerprint(config, run).as_str()) {
        return Ok(Next::Stages {
            from: STAGES[0].into(),
        });
    }
    if let Some(decision) = open.first() {
        return Ok(decision_action(&table, run, decision));
    }
    let landed = run
        .dependencies
        .iter()
        .filter(|d| d.landed_commit.is_some())
        .count();
    let Some(latest) = run.latest_tuning() else {
        return Ok(Next::Tune {
            revision: 1,
            focus: Vec::new(),
        });
    };
    if latest.landed_count < landed {
        return Ok(Next::Tune {
            revision: latest.revision + 1,
            focus: Vec::new(),
        });
    }
    let result = gapcheck::read_result(&latest.out)?;
    let unchecked: Vec<String> = result
        .gaps
        .iter()
        .filter(|g| g.status != PASSING)
        .filter(|g| {
            run.gap_checks
                .get(&g.id)
                .and_then(|v| v["revision"].as_u64())
                != Some(latest.revision)
        })
        .map(|g| g.id.clone())
        .collect();
    if !unchecked.is_empty() {
        return Ok(Next::GapCheck {
            revision: latest.revision,
            gaps: unchecked,
        });
    }
    let verdicts = verdicts(run, latest.revision);
    let focus: Vec<String> = verdicts
        .iter()
        .filter_map(|(_, v)| match v {
            Verdict::Reachable { dials } => Some(dials.clone()),
            _ => None,
        })
        .flatten()
        .collect();
    if !focus.is_empty() {
        return Ok(Next::Tune {
            revision: latest.revision + 1,
            focus,
        });
    }
    // Nothing is left to dispatch: the packet is assembled, and it is the
    // packet that says ready or names what keeps the species unready.
    match super::packet::read(config)? {
        Some(packet)
            if packet["ready_for_owner_review"] == true
                && packet["tuning_identity"] == latest.run_identity =>
        {
            Ok(Next::Ready)
        }
        _ => Ok(Next::Packet),
    }
}
