//! The maintained policy: principle question ids citing exact clauses, the
//! boundary's pipeline and stages, the guards' triggers and entries, and the
//! exception registry. Nothing here is derived from STRATEGY.md by code; a
//! clause changes when a person edits this file, and its version with it.

use serde::Deserialize;

use super::boundary::Caller;

pub const POLICY_JSON: &str = include_str!("../../data/principles/policy.json");
pub const EXCEPTIONS_JSON: &str = include_str!("../../data/principles/exceptions.json");

#[derive(Debug, Clone, Deserialize)]
pub struct Policy {
    pub version: String,
    pub principles: Vec<Principle>,
    pub boundary: Boundary,
    /// Paths whose change re-runs the entry-coverage guard.
    pub entry_triggers: Vec<String>,
    /// Paths whose change re-measures the artifact budgets.
    pub artifact_triggers: Vec<String>,
    /// Every package export and the native build, by role; a generation
    /// entry names the test that builds every shipped preset through it.
    pub entries: Vec<Entry>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Entry {
    pub export: String,
    /// `generation`, `consumer` or `distribution`.
    pub role: String,
    #[serde(default)]
    pub guard: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Principle {
    /// Stable question id, e.g. `P-NO-SWITCH`.
    pub id: String,
    pub name: String,
    /// The exact clause, quoted, with where it lives.
    pub clause: String,
    /// Sanctioned readings: what the principle does not forbid.
    #[serde(default)]
    pub allowances: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Boundary {
    /// Modules whose code is the pipeline.
    pub pipeline: Vec<String>,
    /// Build stages, as public paths.
    pub stages: Vec<String>,
}

impl Boundary {
    /// Pipeline code, and a stage module calling its own stages.
    pub fn allows(&self, caller: &Caller) -> bool {
        let inside = |module: &String| {
            caller.symbol == *module || caller.symbol.starts_with(&format!("{module}::"))
        };
        if self.pipeline.iter().any(inside) {
            return true;
        }
        home(&caller.symbol) == home(&caller.stage)
    }
}

/// A path's crate and top module, `telperion_core::foliage` for a leaf stage.
fn home(path: &str) -> Vec<&str> {
    path.split("::").take(2).collect()
}

/// A scoped, sourced exception. It names a principle, the exact caller
/// symbols and stages it covers, why, and the owner decision or strategy
/// allowance it rests on. A claim with no entry creates none.
#[derive(Debug, Clone, Deserialize)]
pub struct Exception {
    pub id: String,
    pub principle: String,
    pub callers: Vec<String>,
    pub stages: Vec<String>,
    pub rationale: String,
    /// `owner-decision` or `strategy-allowance`.
    pub source_kind: String,
    pub source: String,
}

impl Exception {
    pub fn covers(&self, symbol: &str, stage: &str) -> bool {
        self.callers.iter().any(|c| c == symbol) && self.stages.iter().any(|s| s == stage)
    }

    /// Why this entry is not a valid scoped exception, if it is not.
    pub fn defect(&self) -> Option<String> {
        let wide = |s: &String| s.is_empty() || s.contains('*') || s.contains('/') || s.ends_with("::");
        if self.callers.is_empty() || self.callers.iter().any(wide) {
            return Some(format!("{}: callers must name symbols, not a directory or a glob", self.id));
        }
        if self.stages.is_empty() || self.stages.iter().any(wide) {
            return Some(format!("{}: stages must name symbols", self.id));
        }
        if !matches!(self.source_kind.as_str(), "owner-decision" | "strategy-allowance") {
            return Some(format!("{}: source must be an owner decision or a strategy allowance", self.id));
        }
        if self.source.trim().is_empty() || self.rationale.trim().is_empty() {
            return Some(format!("{}: source and rationale are required", self.id));
        }
        None
    }
}

pub fn policy() -> Policy {
    serde_json::from_str(POLICY_JSON).expect("principles/policy.json")
}

pub fn exceptions() -> Vec<Exception> {
    serde_json::from_str(EXCEPTIONS_JSON).expect("principles/exceptions.json")
}

impl Policy {
    pub fn principle(&self, id: &str) -> Option<&Principle> {
        self.principles.iter().find(|p| p.id == id)
    }
}
