//! Decisions stop a stage; a person resolves them in a separate file.
//!
//! A decision has a stable id derived from species, stage, kind, field and
//! age, a typed payload per kind, and a status. The list is appended
//! atomically and deduplicated by id on rerun. A resolution binds only while
//! its input checksums match the decision's; a stale one is void and the
//! decision reopens. Which stage consumes which option, and the refusal of
//! an option no stage consumes, live in `consume`.

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use super::canon::{read_json, write_canonical, CanonError};
use super::consume::{check_resolutions, hold_unmet, sources_sha256, ReconcileError};
use super::stage::Paths;

pub const DECISIONS_SCHEMA_VERSION: u32 = 1;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Status {
    Open,
    Resolved,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Decision {
    pub id: String,
    pub species: String,
    pub stage: String,
    pub kind: String,
    pub status: Status,
    /// Stages this decision stops while open. A field-scoped decision lists
    /// the field so a stage can exclude that field and run the rest.
    pub blocks: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field: Option<String>,
    pub inputs_sha256: BTreeMap<String, String>,
    pub ledger: Vec<String>,
    pub payload: Value,
    pub options: Vec<String>,
    pub note: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution: Option<Resolution>,
    /// The stage that acted on the resolution, once one has.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub consumed_by: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Resolution {
    pub id: String,
    pub inputs_sha256: BTreeMap<String, String>,
    pub option: String,
    pub by: String,
    pub at: String,
    #[serde(default)]
    pub note: String,
    /// What the option needs beside its name: `replace-source` carries `url`.
    #[serde(default, skip_serializing_if = "Value::is_null")]
    pub payload: Value,
}

pub struct DecisionParts<'a> {
    pub species: &'a str,
    pub stage: &'a str,
    pub kind: &'a str,
    pub field: Option<&'a str>,
    pub age_years: Option<f64>,
}

/// `species/stage/kind[/field[/age]]`, stable across runs and drivers.
pub fn decision_id(parts: &DecisionParts<'_>) -> String {
    let mut id = format!("{}/{}/{}", parts.species, parts.stage, parts.kind);
    if let Some(field) = parts.field {
        id.push('/');
        id.push_str(field);
    }
    if let Some(age) = parts.age_years {
        id.push_str(&format!("/{age}"));
    }
    id
}

impl Decision {
    pub fn new(
        parts: DecisionParts<'_>,
        blocks: &[&str],
        inputs_sha256: BTreeMap<String, String>,
        ledger: Vec<String>,
        payload: Value,
        options: &[&str],
        note: &str,
    ) -> Self {
        Self {
            id: decision_id(&parts),
            species: parts.species.into(),
            stage: parts.stage.into(),
            kind: parts.kind.into(),
            status: Status::Open,
            blocks: blocks.iter().map(|s| s.to_string()).collect(),
            field: parts.field.map(str::to_string),
            inputs_sha256,
            ledger,
            payload,
            options: options.iter().map(|s| s.to_string()).collect(),
            note: note.into(),
            resolution: None,
            consumed_by: None,
        }
    }

    pub fn blocks_stage(&self, stage: &str) -> bool {
        self.status == Status::Open && self.blocks.iter().any(|s| s == stage)
    }
}

fn decisions_value(list: &[Decision]) -> Value {
    json!({
        "schema": "decisions",
        "schema_version": DECISIONS_SCHEMA_VERSION,
        "decisions": list,
    })
}

pub fn write_decisions(path: &Path, list: &[Decision]) -> Result<(), CanonError> {
    write_canonical(path, &decisions_value(list)).map(|_| ())
}

pub fn read_decisions(path: &Path) -> Result<Vec<Decision>, CanonError> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let value = read_json(path)?;
    serde_json::from_value(value["decisions"].clone()).map_err(|error| CanonError::Json {
        path: path.to_path_buf(),
        error: error.to_string(),
    })
}

pub fn read_resolutions(path: &Path) -> Result<Vec<Resolution>, CanonError> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let value = read_json(path)?;
    serde_json::from_value(value["resolutions"].clone()).map_err(|error| CanonError::Json {
        path: path.to_path_buf(),
        error: error.to_string(),
    })
}

/// Appends `new` to the list at `path`, keeping one entry per id (the
/// existing entry wins so a resolved decision is not reopened by a rerun
/// that reissues it with the same inputs; changed inputs reissue it open).
pub fn append_decisions(path: &Path, new: Vec<Decision>) -> Result<Vec<Decision>, CanonError> {
    let mut list = read_decisions(path)?;
    let mut ids: BTreeSet<String> = list.iter().map(|d| d.id.clone()).collect();
    for decision in new {
        if let Some(existing) = list.iter_mut().find(|d| d.id == decision.id) {
            if existing.inputs_sha256 != decision.inputs_sha256 {
                *existing = decision;
            }
            continue;
        }
        ids.insert(decision.id.clone());
        list.push(decision);
    }
    list.sort_by(|a, b| a.id.cmp(&b.id));
    write_canonical(path, &decisions_value(&list))?;
    Ok(list)
}

/// Retires the open decisions of `stage` that a rerun with changed inputs
/// did not file again: the gate they named has passed or was reissued
/// under another id. The first live run of the gap loop left a registry
/// gate open after the registration landed, and the conductor was sent
/// after it. A decision filed with the same inputs is left alone.
pub fn retire_unfiled(
    path: &Path,
    stage: &str,
    filed: &[String],
    inputs: &BTreeMap<String, String>,
    at: &str,
) -> Result<Vec<String>, CanonError> {
    let mut list = read_decisions(path)?;
    let mut retired = Vec::new();
    for decision in list.iter_mut() {
        let stale = decision.stage == stage
            && decision.status == Status::Open
            && !filed.contains(&decision.id)
            && decision.inputs_sha256 != *inputs;
        if !stale {
            continue;
        }
        decision.status = Status::Resolved;
        decision.resolution = Some(Resolution {
            id: decision.id.clone(),
            inputs_sha256: decision.inputs_sha256.clone(),
            option: "superseded".into(),
            by: format!("{stage} stage rerun"),
            at: at.into(),
            note: "the stage reran with changed inputs and did not file this decision again".into(),
            payload: Value::Null,
        });
        retired.push(decision.id.clone());
    }
    if !retired.is_empty() {
        write_canonical(path, &decisions_value(&list))?;
    }
    Ok(retired)
}

/// Applies the resolutions a person wrote. A resolution whose checksums no
/// longer match the decision's is void: the decision stays open and the note
/// names the stale resolution.
pub fn apply_resolutions(decisions: &mut [Decision], resolutions: &[Resolution]) {
    for decision in decisions.iter_mut() {
        let Some(resolution) = resolutions.iter().find(|r| r.id == decision.id) else {
            continue;
        };
        if resolution.inputs_sha256 == decision.inputs_sha256
            && decision.options.contains(&resolution.option)
        {
            decision.status = Status::Resolved;
            decision.resolution = Some(resolution.clone());
        } else {
            decision.status = Status::Open;
            decision.resolution = None;
            // Set, never append: a rerun that reconciles the same stale
            // resolution must leave the list byte-identical.
            let base = decision
                .note
                .split("[void resolution")
                .next()
                .unwrap_or_default()
                .trim();
            decision.note = format!(
                "{base} [void resolution by {} at {}: inputs changed or option unknown]",
                resolution.by, resolution.at
            );
        }
    }
}

/// Reads decisions and resolutions, refuses a resolution the kinds table
/// refuses, applies the rest, holds open a requirements-unmet resolution
/// that added no source, and rewrites the list.
pub fn reconcile(paths: &Paths) -> Result<Vec<Decision>, ReconcileError> {
    let mut list = read_decisions(&paths.decisions())?;
    let resolutions = read_resolutions(&paths.resolutions())?;
    check_resolutions(&list, &resolutions)?;
    apply_resolutions(&mut list, &resolutions);
    if paths.manifest().exists() {
        hold_unmet(&mut list, &sources_sha256(&paths.manifest())?);
    }
    if !list.is_empty() {
        write_decisions(&paths.decisions(), &list)?;
    }
    Ok(list)
}

/// Open decisions that stop `stage`, split into global ones and field-scoped ones.
pub fn open_for_stage(decisions: &[Decision], stage: &str) -> (Vec<String>, BTreeSet<String>) {
    let mut global = Vec::new();
    let mut fields = BTreeSet::new();
    for decision in decisions.iter().filter(|d| d.blocks_stage(stage)) {
        match &decision.field {
            Some(field) => {
                fields.insert(field.clone());
            }
            None => global.push(decision.id.clone()),
        }
    }
    (global, fields)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sha(map: &[(&str, &str)]) -> BTreeMap<String, String> {
        map.iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect()
    }

    fn miss(field: &str, inputs: &[(&str, &str)]) -> Decision {
        Decision::new(
            DecisionParts {
                species: "oregon-white-oak",
                stage: "curve-fit",
                kind: "tolerance-miss",
                field: Some(field),
                age_years: Some(26.7),
            },
            &["generate"],
            sha(inputs),
            vec!["fit:abc".into()],
            json!({"field": field}),
            &["accept-composed-reference", "reject"],
            "note",
        )
    }

    fn scratch() -> std::path::PathBuf {
        let dir = std::env::temp_dir().join(format!(
            "jev-decisions-{}-{}",
            std::process::id(),
            crate::ledger::new_entry_id()
        ));
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn id_is_species_stage_kind_field_age() {
        assert_eq!(
            miss("dbh_m", &[]).id,
            "oregon-white-oak/curve-fit/tolerance-miss/dbh_m/26.7"
        );
    }

    #[test]
    fn append_deduplicates_by_id_and_sorts() {
        let dir = scratch();
        let path = dir.join("decisions.json");
        append_decisions(&path, vec![miss("height_m", &[]), miss("dbh_m", &[])]).unwrap();
        let list = append_decisions(&path, vec![miss("dbh_m", &[])]).unwrap();
        assert_eq!(list.len(), 2);
        assert!(list[0].id < list[1].id);
    }

    #[test]
    fn a_rerun_with_changed_inputs_retires_the_gates_it_did_not_file_again() {
        let dir = scratch();
        let path = dir.join("decisions.json");
        let mut registry = miss("registry", &[]);
        registry.stage = "gate".into();
        registry.inputs_sha256 = sha(&[("select.json", "old")]);
        let mut capability = miss("capability", &[]);
        capability.stage = "gate".into();
        capability.inputs_sha256 = sha(&[("select.json", "old")]);
        append_decisions(&path, vec![registry, capability]).unwrap();
        // The rerun files only the capability gate under new inputs.
        let retired = retire_unfiled(
            &path,
            "gate",
            &[miss("capability", &[]).id],
            &sha(&[("select.json", "new")]),
            "2026-09-22",
        )
        .unwrap();
        assert_eq!(retired, vec![miss("registry", &[]).id]);
        let list = read_decisions(&path).unwrap();
        let reg = list
            .iter()
            .find(|d| d.field.as_deref() == Some("registry"))
            .unwrap();
        assert_eq!(reg.status, Status::Resolved);
        assert_eq!(reg.resolution.as_ref().unwrap().option, "superseded");
        // Same inputs again: nothing is retired, the file is untouched.
        let again =
            retire_unfiled(&path, "gate", &[], &sha(&[("select.json", "old")]), "x").unwrap();
        assert!(again.is_empty());
    }

    #[test]
    fn a_stale_resolution_is_void_and_the_decision_reopens() {
        let mut list = vec![miss("dbh_m", &[("points", "aaa")])];
        let good = Resolution {
            id: list[0].id.clone(),
            inputs_sha256: sha(&[("points", "aaa")]),
            option: "reject".into(),
            by: "owner".into(),
            at: "2026-09-18".into(),
            note: String::new(),
            payload: Value::Null,
        };
        apply_resolutions(&mut list, std::slice::from_ref(&good));
        assert_eq!(list[0].status, Status::Resolved);
        let stale = Resolution {
            inputs_sha256: sha(&[("points", "bbb")]),
            ..good
        };
        apply_resolutions(&mut list, std::slice::from_ref(&stale));
        assert_eq!(list[0].status, Status::Open);
        assert!(list[0].resolution.is_none());
        assert!(list[0].note.contains("void resolution"), "{}", list[0].note);
        // A second reconcile of the same stale resolution changes nothing: the
        // note is set, never appended, so a rerun's decision list is byte-identical.
        let once = list[0].note.clone();
        apply_resolutions(&mut list, &[stale]);
        assert_eq!(list[0].note, once);
        assert_eq!(once.matches("void resolution").count(), 1);
    }

    #[test]
    fn an_unknown_option_does_not_resolve() {
        let mut list = vec![miss("dbh_m", &[])];
        let r = Resolution {
            id: list[0].id.clone(),
            inputs_sha256: BTreeMap::new(),
            option: "anything".into(),
            by: "owner".into(),
            at: "now".into(),
            note: String::new(),
            payload: Value::Null,
        };
        apply_resolutions(&mut list, &[r]);
        assert_eq!(list[0].status, Status::Open);
    }

    #[test]
    fn open_for_stage_splits_global_and_field_scoped() {
        let mut global = miss("dbh_m", &[]);
        global.field = None;
        global.id = "oak/discover/manifest-proposed".into();
        let list = vec![global, miss("height_m", &[])];
        let (g, f) = open_for_stage(&list, "generate");
        assert_eq!(g, vec!["oak/discover/manifest-proposed".to_string()]);
        assert!(f.contains("height_m"));
        assert!(open_for_stage(&list, "fetch").0.is_empty());
    }
}
