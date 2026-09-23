//! Which stage consumes which resolution option, and what is refused.
//!
//! The kinds table names, per decision kind, its options and the stages that
//! act on a resolution carrying one of them. A resolution to one of these
//! kinds with an option outside the list is refused when it is read, naming
//! the options a stage does consume; the stage that acted on one records
//! itself on the decision; a rejected manifest proposal stops every stage
//! after discover.

use std::path::Path;

use serde_json::Value;

use super::canon::{canonical_sha256, read_json, CanonError};
use super::decision::{Decision, Resolution, Status};
use super::stage::STAGES;

/// A required field or appearance trait below the requirements table's bar,
/// whose only option adds sources: the pipeline's for two search rounds on
/// a field (`search`), the owner's after them and for a trait.
pub const REQUIREMENTS_UNMET: &str = "requirements-unmet";

/// The option `decision::retire_unfiled` writes on a stage's own open
/// decision that its rerun did not file again. No person chose it and no
/// stage consumes it; `hold_unmet` leaves it resolved.
pub const SUPERSEDED: &str = "superseded";

/// The decision kinds whose options a stage consumes: the kind, its options,
/// and the stages that act on a resolution carrying one of them. A kind not
/// listed here is resolved by a person and consumed by no stage.
pub fn consumers(kind: &str) -> Option<(&'static [&'static str], &'static [&'static str])> {
    match kind {
        "manifest-proposed" => Some((&["admit", "reject"], &STAGES[1..])),
        "unavailable-source" => Some((&["retry", "replace-source", "drop-source"], &["fetch"])),
        "coverage-gap" => Some((&["accept-rows", "fix-table", "drop-table"], &["fetch"])),
        "data-insufficient" => Some((&["admit-proxy", "add-sources", "lower-bar"], &["quality"])),
        REQUIREMENTS_UNMET => Some((&["add-sources"], &["quality", "select"])),
        // A flagged value (fn-131): select drops it on drop-value and on
        // replace-source, and files its requirement for the search again.
        "claim-contradicted" | "claim-unsupported" => {
            Some((&["accept", "replace-source", "drop-value"], &["select"]))
        }
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
/// the options a stage does consume, and a `replace-source` of an
/// unavailable source without a url; a claim's `replace-source` is the
/// pipeline's own search and names none.
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
        let fetched = stages.contains(&"fetch");
        if resolution.option == "replace-source" && fetched && url.trim().is_empty() {
            return Err(ReconcileError::Refused(format!(
                "{}: replace-source names no replacement url in its payload",
                resolution.id
            )));
        }
    }
    Ok(())
}

/// The checksum of the manifest's `sources` as written on disk, so an
/// `add-sources` resolution can tell whether a source was added.
pub fn sources_sha256(manifest: &Path) -> Result<String, CanonError> {
    Ok(canonical_sha256(&read_json(manifest)?["sources"]))
}

/// Reopens every resolved requirements-unmet decision whose manifest sources
/// are still the ones it was filed against: a resolution that adds no source
/// leaves the decision open. An appearance trait's decision (`select`) also
/// counts a source id added to the trait's own list (`traits` is the
/// manifest's `appearance`), which the pipeline may add (fn-129); a field
/// select filed (fn-131) counts the manifest's sources, as quality's does. A decision
/// its stage superseded is not held: the field passed with the sources it
/// had. The note is set, never appended, so a rerun is byte-identical. True
/// when the list changed.
pub fn hold_unmet(decisions: &mut [Decision], sources: &str, traits: &Value) -> bool {
    let mut changed = false;
    for decision in decisions.iter_mut() {
        let list = trait_list(traits, decision);
        let unchanged = decision.payload["sources_sha256"].as_str() == Some(sources)
            && (list.is_null() || list == decision.payload["sources_tried"]);
        let superseded = decision
            .resolution
            .as_ref()
            .is_some_and(|r| r.option == SUPERSEDED);
        if decision.kind != REQUIREMENTS_UNMET
            || decision.status != Status::Resolved
            || !unchanged
            || superseded
        {
            continue;
        }
        let Some(resolution) = decision.resolution.take() else {
            continue;
        };
        decision.status = Status::Open;
        let base = decision
            .note
            .split("[void resolution")
            .next()
            .unwrap_or_default()
            .trim();
        decision.note = format!(
            "{base} [void resolution by {} at {}: no source was added]",
            resolution.by, resolution.at
        );
        changed = true;
    }
    changed
}

/// The `sources` list of the trait a decision names, or null.
fn trait_list(traits: &Value, decision: &Decision) -> Value {
    traits
        .as_array()
        .into_iter()
        .flatten()
        .find(|t| t["trait_name"].as_str() == decision.field.as_deref())
        .map_or(Value::Null, |t| t["sources"].clone())
}

/// The open decisions that stop the run for the owner: NEEDS_HUMAN.
pub fn owner_stops(decisions: &[Decision]) -> Vec<String> {
    decisions
        .iter()
        .filter(|d| d.kind == REQUIREMENTS_UNMET && d.status == Status::Open)
        .map(|d| d.id.clone())
        .collect()
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

    fn unmet(sources: &str) -> Decision {
        Decision::new(
            DecisionParts {
                species: "date-palm",
                stage: "quality",
                kind: REQUIREMENTS_UNMET,
                field: Some("frond_length_m"),
                age_years: None,
            },
            &["select", "fit", "generate"],
            sha(&[("fetch.json", "aaa")]),
            vec![],
            json!({"sources_sha256": sources}),
            &["add-sources"],
            "NEEDS_HUMAN",
        )
    }

    fn resolve(decision: &Decision, option: &str) -> Resolution {
        Resolution {
            id: decision.id.clone(),
            inputs_sha256: decision.inputs_sha256.clone(),
            option: option.into(),
            by: "cheap-driver".into(),
            at: "2026-09-23".into(),
            note: String::new(),
            payload: Value::Null,
        }
    }

    #[test]
    fn a_required_field_cannot_be_lowered_and_an_add_that_adds_nothing_stays_open() {
        let list = vec![unmet("before")];
        let err = check_resolutions(&list, &[resolve(&list[0], "lower-bar")])
            .unwrap_err()
            .to_string();
        assert!(
            err.ends_with("option lower-bar of kind requirements-unmet is consumed by no stage; quality, select consume add-sources"),
            "{err}"
        );
        let add = resolve(&list[0], "add-sources");
        assert!(check_resolutions(&list, std::slice::from_ref(&add)).is_ok());
        let mut held = list.clone();
        apply_resolutions(&mut held, std::slice::from_ref(&add));
        assert!(hold_unmet(&mut held, "before", &Value::Null));
        assert_eq!(held[0].status, Status::Open);
        assert_eq!(owner_stops(&held), vec![held[0].id.clone()]);
        let once = held[0].note.clone();
        apply_resolutions(&mut held, std::slice::from_ref(&add));
        hold_unmet(&mut held, "before", &Value::Null);
        assert_eq!(held[0].note, once);
        assert!(once.ends_with("no source was added]"), "{once}");
        let mut added = list;
        apply_resolutions(&mut added, &[add]);
        assert!(!hold_unmet(&mut added, "after", &Value::Null));
        assert_eq!(added[0].status, Status::Resolved);
        assert!(owner_stops(&added).is_empty());
    }
}
