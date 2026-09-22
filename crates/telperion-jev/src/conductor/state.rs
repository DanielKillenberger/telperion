//! The run record: what the conductor has to remember beyond the pipeline's
//! own artifacts to resume without repeating work. Stage state is never
//! copied here; the stages carry their own keys. What lives here is the
//! budget, every dispatch with its input identity and result, every gap
//! dependency, every tuning revision, and the pause a human has to resolve.
use std::collections::BTreeMap;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use super::dispatch::Dispatch;
use super::{now, ConductorError, Config, Result, SCHEMA_VERSION};
use crate::pipeline::canon::{read_json, write_canonical};
use crate::tuning::continuation::{Basis, HumanDecision, Pause};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Budget {
    pub max_tokens: u64,
    pub attempt_max_tokens: u64,
    pub max_dispatches: u64,
    pub max_tuning_revisions: u64,
    /// Tokens known to be spent: dispatch usage, Jev usage and tuning runs.
    pub tokens: u64,
    /// False once a finished dispatch reported no usage; the continuation
    /// contract then refuses to reserve until a human reconciles it.
    pub usage_known: bool,
    pub jev_calls: u64,
}

impl Budget {
    pub fn remaining(&self) -> u64 {
        self.max_tokens.saturating_sub(self.tokens)
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum DependencyStatus {
    /// The spec exists; nobody has designed or implemented for it yet.
    Awaiting,
    Designed,
    Implemented,
    /// Verified work that waits on the host's landing authority.
    AwaitingLanding,
    Landed,
}

/// A generator spec the species run depends on: attached when an open spec
/// already covers the gap, or minted by the owner after a new gap escalated.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Dependency {
    pub spec: String,
    pub gap_id: String,
    /// `existing` or `minted`.
    pub origin: String,
    pub status: DependencyStatus,
    #[serde(default)]
    pub design_revision: Option<String>,
    #[serde(default)]
    pub handoff: Option<PathBuf>,
    #[serde(default)]
    pub landed_commit: Option<String>,
    pub attached_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct TuningRevision {
    pub revision: u64,
    pub out: PathBuf,
    pub run_identity: String,
    pub machine_ready: bool,
    pub bootstrap: bool,
    pub tokens: u64,
    pub gaps: usize,
    /// How many dependencies had landed when this revision ran, so a later
    /// landing asks for the next one.
    pub landed_count: usize,
    pub at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Wait {
    pub reason: String,
    pub from: String,
    #[serde(default)]
    pub to: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Run {
    pub schema: String,
    pub schema_version: u32,
    pub species: String,
    pub spec: String,
    pub policy_version: u32,
    pub started_at: String,
    pub updated_at: String,
    /// How many times a start found this record and resumed it.
    pub resumes: u64,
    pub budget: Budget,
    pub dispatches: Vec<Dispatch>,
    pub dependencies: Vec<Dependency>,
    /// Landed commits whose stages have rerun since, halt or not; a landing
    /// not in this list sends the stages before any halt is acted on.
    #[serde(default)]
    pub stages_rerun_for: Vec<String>,
    pub tuning: Vec<TuningRevision>,
    /// Every route the policy took, in order: `<context>=<route>: <why>`.
    pub routes: Vec<String>,
    pub waits: Vec<Wait>,
    #[serde(default)]
    pub pause: Option<Pause>,
    #[serde(default)]
    pub authorizations: Vec<HumanDecision>,
    /// Gap ids the run has checked, with the verdict, so a resume asks again
    /// only when the tuning revision changed.
    #[serde(default)]
    pub gap_checks: BTreeMap<String, Value>,
    /// The fingerprint the stages were last found current under.
    #[serde(default)]
    pub stage_fingerprint: Option<String>,
}

impl Run {
    pub fn new(config: &Config) -> Self {
        let at = now();
        Self {
            stages_rerun_for: Vec::new(),
            schema: "conductor-run".into(),
            schema_version: SCHEMA_VERSION,
            species: config.species.clone(),
            spec: config.spec.clone(),
            policy_version: super::policy::load().version,
            started_at: at.clone(),
            updated_at: at,
            resumes: 0,
            budget: Budget {
                max_tokens: config.budget.max_tokens,
                attempt_max_tokens: config.budget.attempt_max_tokens,
                max_dispatches: config.budget.max_dispatches,
                max_tuning_revisions: config.budget.max_tuning_revisions,
                tokens: 0,
                usage_known: true,
                jev_calls: 0,
            },
            dispatches: Vec::new(),
            dependencies: Vec::new(),
            tuning: Vec::new(),
            routes: Vec::new(),
            waits: Vec::new(),
            pause: None,
            authorizations: Vec::new(),
            gap_checks: BTreeMap::new(),
            stage_fingerprint: None,
        }
    }

    /// Loads the record, or creates it. An existing record is read with its
    /// budgets as they were: a restart replenishes nothing, and a config
    /// that names a different species or spec is refused. `start` counts
    /// the resume; a status read is not an interruption.
    pub fn open(config: &Config) -> Result<Self> {
        let path = config.run_file();
        if !path.exists() {
            return Ok(Self::new(config));
        }
        let mut run: Run = serde_json::from_value(read_json(&path)?)
            .map_err(|err| ConductorError::Invalid(format!("{}: {err}", path.display())))?;
        run.charge_unknown_usage();
        if run.species != config.species || run.spec != config.spec {
            return Err(format!(
                "{} belongs to {} ({}); this config names {} ({})",
                path.display(),
                run.species,
                run.spec,
                config.species,
                config.spec
            )
            .into());
        }
        Ok(run)
    }

    pub fn save(&mut self, config: &Config) -> Result<PathBuf> {
        self.updated_at = now();
        let path = config.run_file();
        write_canonical(
            &path,
            &serde_json::to_value(&*self).expect("run serializes"),
        )?;
        Ok(path)
    }

    /// A record written before unknown usage charged its reservation: every
    /// finished dispatch without a count is charged now, once, and the run's
    /// usage is known again.
    pub fn charge_unknown_usage(&mut self) {
        let mut charged = 0u64;
        for d in self.dispatches.iter_mut() {
            if let Some(r) = d.result.as_mut() {
                if r.usage.is_none() && !r.usage_is_reservation {
                    charged = charged.saturating_add(d.reserved_tokens);
                    r.usage_is_reservation = true;
                }
            }
        }
        if charged > 0 || !self.budget.usage_known {
            self.budget.tokens = self.budget.tokens.saturating_add(charged);
            self.budget.usage_known = true;
        }
    }

    pub fn open_dispatch(&self) -> Option<&Dispatch> {
        self.dispatches.iter().find(|d| d.open())
    }

    pub fn dependency_mut(&mut self, spec: &str) -> Option<&mut Dependency> {
        self.dependencies.iter_mut().find(|d| d.spec == spec)
    }

    pub fn latest_tuning(&self) -> Option<&TuningRevision> {
        self.tuning.last()
    }

    pub fn route(&mut self, context: &str, route: &str, why: &str) {
        self.routes.push(format!("{context}={route}: {why}"));
    }

    pub fn wait(&mut self, reason: &str) {
        if self.waits.last().is_some_and(|w| w.to.is_none()) {
            return;
        }
        self.waits.push(Wait {
            reason: reason.into(),
            from: now(),
            to: None,
        });
    }

    pub fn end_wait(&mut self) {
        if let Some(wait) = self.waits.last_mut().filter(|w| w.to.is_none()) {
            wait.to = Some(now());
        }
    }

    /// Pauses for a scoped human decision. No further work dispatches while
    /// the pause stands, and only the decision that names it resumes.
    pub fn pause(&mut self, id: &str, reason: &str, basis: Basis, requested: &str) {
        self.pause = Some(Pause {
            id: id.into(),
            identity: basis.identity.clone(),
            reason: reason.into(),
            basis,
            decision_requested: requested.into(),
        });
        self.wait(&format!("human: {reason}"));
    }

    pub fn resume(&mut self, decision: HumanDecision) -> Result<()> {
        let pause = self
            .pause
            .as_ref()
            .ok_or_else(|| ConductorError::Invalid("the run is not paused".into()))?;
        pause.resume(&decision)?;
        self.pause = None;
        self.end_wait();
        self.authorizations.push(decision);
        Ok(())
    }

    pub fn summary(&self) -> Value {
        json!({
            "species": self.species,
            "spec": self.spec,
            "resumes": self.resumes,
            "tokens": self.budget.tokens,
            "remaining_tokens": self.budget.remaining(),
            "usage_known": self.budget.usage_known,
            "dispatches": self.dispatches.len(),
            "open_dispatch": self.open_dispatch().map(|d| d.id.clone()),
            "dependencies": self.dependencies.iter().map(|d| json!({"spec": d.spec, "status": d.status})).collect::<Vec<_>>(),
            "tuning_revisions": self.tuning.len(),
            "paused": self.pause.as_ref().map(|p| p.id.clone()),
        })
    }
}
