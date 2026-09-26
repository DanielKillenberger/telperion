//! Which stage consumes which resolution option, and what is refused.
//!
//! The kinds table names, per decision kind, its options and the stages that
//! act on a resolution carrying one of them. A resolution to one of these
//! kinds with an option outside the list is refused when it is read, naming
//! the options a stage does consume; the stage that acted on one records
//! itself on the decision.

use super::decision::{Decision, Resolution};

/// The option `decision::retire_unfiled` writes on a stage's own open
/// decision that its rerun did not file again. No person chose it and no
/// stage consumes it.
pub const SUPERSEDED: &str = "superseded";

/// The decision kinds whose options a stage consumes: the kind, its options,
/// and the stages that act on a resolution carrying one of them. A kind not
/// listed here is resolved by a person and consumed by no stage.
pub fn consumers(kind: &str) -> Option<(&'static [&'static str], &'static [&'static str])> {
    match kind {
        "coverage-gap" => Some((&["accept-rows", "fix-table", "drop-table"], &["fetch"])),
        _ => None,
    }
}

#[derive(Debug)]
pub enum ReconcileError {
    File(super::canon::CanonError),
    /// A resolution the kinds table refuses, with the reason.
    Refused(String),
}

impl std::fmt::Display for ReconcileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::File(err) => write!(f, "{err}"),
            Self::Refused(reason) => write!(f, "resolution refused: {reason}"),
        }
    }
}

impl From<super::canon::CanonError> for ReconcileError {
    fn from(err: super::canon::CanonError) -> Self {
        Self::File(err)
    }
}

/// Refuses a resolution whose option no stage consumes, naming the kind and
/// the options a stage does consume.
pub fn check_resolutions(
    decisions: &[Decision],
    resolutions: &[Resolution],
) -> Result<(), ReconcileError> {
    for resolution in resolutions {
        let Some(decision) = decisions.iter().find(|d| d.id == resolution.id) else {
            continue;
        };
        let Some((options, stages)) = consumers(&decision.kind) else {
            continue;
        };
        if !options.contains(&resolution.option.as_str()) {
            return Err(ReconcileError::Refused(format!(
                "{}: option {} of kind {} is consumed by no stage; {} consume{} {}",
                resolution.id,
                resolution.option,
                decision.kind,
                stages.join(", "),
                if stages.len() == 1 { "s" } else { "" },
                options.join(", ")
            )));
        }
    }
    Ok(())
}

/// Records `stage` on every resolved decision whose option it consumes and
/// no stage has consumed yet. True when the list changed.
pub fn mark_consumed(decisions: &mut [Decision], stage: &str) -> bool {
    let mut changed = false;
    for decision in decisions.iter_mut() {
        let Some(resolution) = &decision.resolution else {
            continue;
        };
        let consumed = consumers(&decision.kind)
            .map(|(options, stages)| {
                options.contains(&resolution.option.as_str()) && stages.contains(&stage)
            })
            .unwrap_or(false);
        if consumed && decision.consumed_by.is_none() {
            decision.consumed_by = Some(stage.to_string());
            changed = true;
        }
    }
    changed
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use serde_json::{json, Value};

    use super::*;
    use crate::pipeline::decision::{apply_resolutions, DecisionParts};

    fn gap(option: &str) -> (Vec<Decision>, Resolution) {
        let inputs: BTreeMap<String, String> = [("table".to_string(), "aaa".to_string())].into();
        let decision = Decision::new(
            DecisionParts {
                species: "european-ash",
                stage: "fetch",
                kind: "coverage-gap",
                field: Some("height_m"),
                age_years: None,
            },
            &["fit"],
            inputs.clone(),
            vec![],
            json!({}),
            &["accept-rows", "fix-table", "drop-table"],
            "",
        );
        let resolution = Resolution {
            id: decision.id.clone(),
            inputs_sha256: inputs,
            option: option.into(),
            by: "owner".into(),
            at: "2026-09-18".into(),
            note: String::new(),
            payload: Value::Null,
        };
        (vec![decision], resolution)
    }

    #[test]
    fn an_option_no_stage_consumes_is_refused_naming_the_kind_and_the_consumed_ones() {
        let (list, resolution) = gap("ignore");
        let err = check_resolutions(&list, &[resolution])
            .unwrap_err()
            .to_string();
        assert_eq!(
            err,
            "resolution refused: european-ash/fetch/coverage-gap/height_m: option ignore of kind coverage-gap is consumed by no stage; fetch consumes accept-rows, fix-table, drop-table"
        );
        let (list, resolution) = gap("drop-table");
        assert!(check_resolutions(&list, &[resolution]).is_ok());
    }

    #[test]
    fn the_consuming_stage_is_recorded_once() {
        let (mut list, resolution) = gap("drop-table");
        apply_resolutions(&mut list, &[resolution]);
        assert!(!mark_consumed(&mut list, "read"));
        assert!(mark_consumed(&mut list, "fetch"));
        assert_eq!(list[0].consumed_by.as_deref(), Some("fetch"));
        assert!(!mark_consumed(&mut list, "fetch"));
    }
}
