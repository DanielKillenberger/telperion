//! Guard 3, artifact budgets: each shipped Wasm module and package script,
//! built by the fixed recipe, against its checked-in budget. A budget that
//! moves carries a new measurement and a Decisions reference, or it fails.

use std::collections::BTreeMap;
use std::path::Path;

use serde::Deserialize;

pub const BUDGETS_JSON: &str = include_str!("../../data/principles/budgets.json");

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct Budgets {
    /// The build every size is measured from.
    pub recipe: String,
    pub artifacts: Vec<Budget>,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct Budget {
    /// Package-relative path, e.g. `dist/telperion-field.wasm`.
    pub path: String,
    pub max_bytes: u64,
    pub evidence: Evidence,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct Evidence {
    pub measured_bytes: u64,
    /// The build measured: a released version or a revision.
    pub measured_on: String,
    /// The spec, PR or Decisions line that accepted this budget.
    pub decisions: String,
}

pub fn budgets() -> Budgets {
    parse(BUDGETS_JSON).expect("principles/budgets.json")
}

pub fn parse(text: &str) -> Result<Budgets, String> {
    serde_json::from_str(text).map_err(|e| format!("budgets.json: {e}"))
}

/// Every artifact over its budget or missing from `sizes`.
pub fn check(budgets: &Budgets, sizes: &BTreeMap<String, u64>) -> Vec<String> {
    let mut out = Vec::new();
    for b in &budgets.artifacts {
        match sizes.get(&b.path) {
            None => out.push(format!("{}: not built by the recipe ({})", b.path, budgets.recipe)),
            Some(&size) if size > b.max_bytes => out.push(format!(
                "{}: {size} bytes, budget {} (+{:.1}% over {} measured on {})",
                b.path,
                b.max_bytes,
                100.0 * (size as f64 / b.evidence.measured_bytes as f64 - 1.0),
                b.evidence.measured_bytes,
                b.evidence.measured_on
            )),
            Some(_) => {}
        }
    }
    out
}

/// Budget entries in `head` that moved from `base` without new evidence.
pub fn change_defects(base: Option<&Budgets>, head: &Budgets) -> Vec<String> {
    let mut out = Vec::new();
    for b in &head.artifacts {
        let before = base.and_then(|base| base.artifacts.iter().find(|a| a.path == b.path));
        if b.evidence.measured_bytes > b.max_bytes {
            out.push(format!("{}: evidence measures over its own budget", b.path));
        }
        if b.evidence.decisions.trim().is_empty() || b.evidence.measured_on.trim().is_empty() {
            out.push(format!("{}: evidence names no build or Decisions reference", b.path));
        }
        let Some(before) = before else { continue };
        if before.max_bytes != b.max_bytes && before.evidence == b.evidence {
            out.push(format!(
                "{}: budget moved {} -> {} without a new measurement and Decisions reference",
                b.path, before.max_bytes, b.max_bytes
            ));
        }
    }
    out
}

/// Sizes of the budgeted artifacts that exist under `root`.
pub fn measure(budgets: &Budgets, root: &Path) -> BTreeMap<String, u64> {
    budgets
        .artifacts
        .iter()
        .filter_map(|b| Some((b.path.clone(), std::fs::metadata(root.join(&b.path)).ok()?.len())))
        .collect()
}
