//! The species conductor (fn-89): one persistent run that carries a species
//! from the admitted manifest through the pipeline's stages, the gap loop,
//! the tuning run's gap list, the design and implementation dispatches a gap
//! spec needs, and the readiness packet the owner reviews.
//!
//! Code owns stage eligibility, budgets, dispatch tracking and resume. Jev
//! answers bounded questions over explicit evidence (design complexity,
//! remaining implementation complexity after a design exists, whether a gap
//! is reachable with an untried dial or covered by an open spec, and the
//! shared continuation trio); `data/conductor-policy.json` maps those answers
//! to a route, and every route the table cannot justify is the human's. The
//! conductor designs nothing, implements nothing and accepts nothing: a new
//! gap is packaged and escalated, and only the owner's verdict accepts.

pub mod cases;
pub mod cli;
pub mod dependency;
pub mod dispatch;
pub mod gapcheck;
pub mod handoff;
pub mod packet;
pub mod plan;
pub mod policy;
pub mod questions;
pub mod report;
pub mod state;
pub mod step;

use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::caller::CallerError;
use crate::pipeline::canon::CanonError;
use crate::pipeline::stage::Paths;

pub const SCHEMA_VERSION: u32 = 1;

#[derive(Debug)]
pub enum ConductorError {
    File(CanonError),
    Invalid(String),
    Caller(CallerError),
}

impl std::fmt::Display for ConductorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::File(err) => write!(f, "{err}"),
            Self::Invalid(msg) => write!(f, "conductor: {msg}"),
            Self::Caller(err) => write!(f, "conductor: {err}"),
        }
    }
}

impl std::error::Error for ConductorError {}

impl From<CanonError> for ConductorError {
    fn from(err: CanonError) -> Self {
        Self::File(err)
    }
}

impl From<CallerError> for ConductorError {
    fn from(err: CallerError) -> Self {
        Self::Caller(err)
    }
}

impl From<String> for ConductorError {
    fn from(msg: String) -> Self {
        Self::Invalid(msg)
    }
}

pub type Result<T> = std::result::Result<T, ConductorError>;

/// The run's configuration, written by a person once per species run. Every
/// path is relative to the working directory the conductor runs from.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Config {
    pub species: String,
    /// The species spec the run belongs to; gap specs become its dependencies.
    pub spec: String,
    /// The species' catalogue folder (`--dir`) and the run directory (`--run-dir`).
    pub dir: PathBuf,
    pub run_dir: PathBuf,
    /// The Flow tree the open specs are read from; defaults to `.flow`.
    #[serde(default = "default_flow")]
    pub flow: PathBuf,
    /// The tuning loop's typed config (`tuning::live::Config`), run per revision.
    pub tuning_config: PathBuf,
    /// The runbook's binaries. Absent binaries stop the step that needs them.
    #[serde(default = "default_pipeline_binary")]
    pub species_pipeline: PathBuf,
    #[serde(default = "default_tuning_binary")]
    pub tuning_loop: PathBuf,
    /// Extra arguments every stage command carries (`--example`, `--profile-id`).
    #[serde(default)]
    pub stage_args: Vec<String>,
    /// Optional caps; a run with no budget block carries none.
    #[serde(default)]
    pub budget: BudgetConfig,
    #[serde(default = "default_model")]
    pub judgment_model: String,
    /// True once the continuation question set has been validated on labelled
    /// cases; an unvalidated judgment cannot authorize unattended continuation.
    #[serde(default)]
    pub continuation_validated: bool,
}

/// Every cap is optional and an absent one is no cap (fn-117). Spend is
/// recorded whether or not a cap is set.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct BudgetConfig {
    /// The run's total token allowance across every dispatch, Jev call and tuning run.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u64>,
    /// The bound one dispatch or tuning round may reserve.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attempt_max_tokens: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_dispatches: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_tuning_revisions: Option<u64>,
}

fn default_flow() -> PathBuf {
    PathBuf::from(".flow")
}
fn default_pipeline_binary() -> PathBuf {
    PathBuf::from("target/release/species-pipeline")
}
fn default_tuning_binary() -> PathBuf {
    PathBuf::from("target/release/tuning-loop")
}
fn default_model() -> String {
    crate::caller::MODEL.into()
}

impl Config {
    pub fn load(path: &Path) -> Result<Self> {
        let raw = crate::pipeline::canon::read_json(path)?;
        let config: Config = serde_json::from_value(raw)
            .map_err(|err| ConductorError::Invalid(format!("{}: {err}", path.display())))?;
        if config.species.trim().is_empty() || config.spec.trim().is_empty() {
            return Err("config names no species or no spec".to_string().into());
        }
        let budget = &config.budget;
        if budget.max_tokens == Some(0)
            || budget.attempt_max_tokens == Some(0)
            || budget
                .attempt_max_tokens
                .zip(budget.max_tokens)
                .is_some_and(|(attempt, total)| attempt > total)
        {
            return Err(
                "config budget: attempt bound must be positive and inside the total"
                    .to_string()
                    .into(),
            );
        }
        Ok(config)
    }

    pub fn paths(&self) -> Paths {
        Paths::with_run(&self.dir, &self.run_dir)
    }

    /// The conductor's own directory under the run directory.
    pub fn conductor_dir(&self) -> PathBuf {
        self.run_dir.join("conductor")
    }
    pub fn run_file(&self) -> PathBuf {
        self.conductor_dir().join("run.json")
    }
    pub fn tuning_dir(&self, revision: u64) -> PathBuf {
        self.conductor_dir().join(format!("tuning-{revision}"))
    }
    pub fn packet_file(&self) -> PathBuf {
        self.conductor_dir().join("packet.json")
    }
    pub fn report_file(&self) -> PathBuf {
        self.conductor_dir().join("report.json")
    }
    pub fn gaps_dir(&self) -> PathBuf {
        self.conductor_dir().join("gaps")
    }
    pub fn ledger_dir(&self) -> PathBuf {
        self.paths().ledger().join("entries")
    }
}

/// The clock, once per record, so a test can pin it.
pub fn now() -> String {
    std::env::var("TELPERION_CONDUCTOR_CLOCK").unwrap_or_else(|_| crate::caller::now_rfc3339())
}
