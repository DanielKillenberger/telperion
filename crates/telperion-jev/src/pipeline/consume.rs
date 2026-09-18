//! Which stage consumes which resolution option, and what is refused.
//!
//! The kinds table names, per decision kind, its options and the stages that
//! act on a resolution carrying one of them. A resolution to one of these
//! kinds with an option outside the list is refused when it is read, naming
//! the options a stage does consume; the stage that acted on one records
//! itself on the decision; a rejected manifest proposal stops every stage
//! after discover.

use super::canon::CanonError;
use super::decision::{Decision, Resolution};
use super::stage::STAGES;

/// The decision kinds whose options a stage consumes: the kind, its options,
/// and the stages that act on a resolution carrying one of them. A kind not
/// listed here is resolved by a person and consumed by no stage.
pub fn consumers(kind: &str) -> Option<(&'static [&'static str], &'static [&'static str])> {
    match kind {
        "manifest-proposed" => Some((&["admit", "reject"], &STAGES[1..])),
        "unavailable-source" => Some((&["retry", "replace-source", "drop-source"], &["fetch"])),
        "coverage-gap" => Some((&["accept-rows", "fix-table", "drop-table"], &["fetch"])),
        "data-insufficient" => Some((&["admit-proxy", "add-sources", "lower-bar"], &["quality"])),
        _ => None,
    }
}

#[derive(Debug)]
pub enum ReconcileError {
    File(CanonError),
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

impl From<CanonError> for ReconcileError {
    fn from(err: CanonError) -> Self {
        Self::File(err)
    }
}

/// Refuses a resolution whose option no stage consumes, naming the kind and
/// the options a stage does consume, and a `replace-source` without a url.
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
        let url = resolution.payload["url"].as_str().unwrap_or_default();
        if resolution.option == "replace-source" && url.trim().is_empty() {
            return Err(ReconcileError::Refused(format!(
                "{}: replace-source names no replacement url in its payload",
                resolution.id
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

/// The id of a rejected manifest proposal that stops `stage`: every stage
/// after discover, until a person edits the seed and discovers again.
pub fn rejected_proposal(decisions: &[Decision], stage: &str) -> Option<String> {
    decisions
        .iter()
        .find(|d| {
            d.kind == "manifest-proposed"
                && d.blocks.iter().any(|s| s == stage)
                && d.resolution.as_ref().map(|r| r.option.as_str()) == Some("reject")
        })
        .map(|d| d.id.clone())
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use serde_json::{json, Value};

    use super::*;
    use crate::pipeline::decision::{apply_resolutions, DecisionParts, Status};

    fn sha(map: &[(&str, &str)]) -> BTreeMap<String, String> {
        map.iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect()
    }

    fn unavailable(option: &str, payload: Value) -> (Vec<Decision>, Resolution) {
        let decision = Decision::new(
            DecisionParts {
                species: "european-ash",
                stage: "fetch",
                kind: "unavailable-source",
                field: Some("M1"),
                age_years: None,
            },
            &["extract"],
            sha(&[("url", "aaa")]),
            vec![],
            json!({}),
            &["retry", "replace-source", "drop-source"],
            "",
        );
        let resolution = Resolution {
            id: decision.id.clone(),
            inputs_sha256: sha(&[("url", "aaa")]),
            option: option.into(),
            by: "owner".into(),
            at: "2026-09-18".into(),
            note: String::new(),
            payload,
        };
        (vec![decision], resolution)
    }

    #[test]
    fn an_option_no_stage_consumes_is_refused_naming_the_kind_and_the_consumed_ones() {
        let (list, resolution) = unavailable("ignore", Value::Null);
        let err = check_resolutions(&list, &[resolution])
            .unwrap_err()
            .to_string();
        assert_eq!(
            err,
            "resolution refused: european-ash/fetch/unavailable-source/M1: option ignore of kind unavailable-source is consumed by no stage; fetch consumes retry, replace-source, drop-source"
        );
        let (list, resolution) = unavailable("drop-source", Value::Null);
        assert!(check_resolutions(&list, &[resolution]).is_ok());
    }

    #[test]
    fn a_replace_source_without_a_url_is_refused() {
        let (list, resolution) = unavailable("replace-source", json!({"note": "x"}));
        let err = check_resolutions(&list, &[resolution])
            .unwrap_err()
            .to_string();
        assert!(
            err.ends_with("replace-source names no replacement url in its payload"),
            "{err}"
        );
        let (list, resolution) =
            unavailable("replace-source", json!({"url": "https://example.test/m1"}));
        assert!(check_resolutions(&list, &[resolution]).is_ok());
    }

    #[test]
    fn the_consuming_stage_is_recorded_once_and_a_rejected_proposal_stops_later_stages() {
        let (mut list, resolution) = unavailable("drop-source", Value::Null);
        apply_resolutions(&mut list, &[resolution]);
        assert!(!mark_consumed(&mut list, "extract"));
        assert!(mark_consumed(&mut list, "fetch"));
        assert_eq!(list[0].consumed_by.as_deref(), Some("fetch"));
        assert!(!mark_consumed(&mut list, "fetch"));
        let mut proposal = Decision::new(
            DecisionParts {
                species: "european-ash",
                stage: "discover",
                kind: "manifest-proposed",
                field: None,
                age_years: None,
            },
            &STAGES[1..],
            BTreeMap::new(),
            vec![],
            json!({}),
            &["admit", "reject"],
            "",
        );
        proposal.status = Status::Resolved;
        proposal.resolution = Some(Resolution {
            id: proposal.id.clone(),
            inputs_sha256: BTreeMap::new(),
            option: "reject".into(),
            by: "owner".into(),
            at: "2026-09-18".into(),
            note: String::new(),
            payload: Value::Null,
        });
        let list = vec![proposal];
        assert_eq!(
            rejected_proposal(&list, "fetch").as_deref(),
            Some("european-ash/discover/manifest-proposed")
        );
        assert!(rejected_proposal(&list, "discover").is_none());
    }
}
