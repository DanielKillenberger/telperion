//! The pipeline admits its own sources (fn-129).
//!
//! A proposed source is admitted when Jev judged it the best evidence for
//! the field it was found for (the ranking chose it) and its rights class is
//! `open-licence` or `public-cite-only`. A draft manifest is admitted whole
//! or not at all: every new source passes, the admitted sources are kept as
//! they are, and nothing else in the manifest changes - no field, bar,
//! appearance trait or schema version. Anything else goes to the owner. An
//! admission writes the manifest with each new source's rights class and a
//! resolution `by: pipeline`, so the stages read it as they read a person's.

use std::path::Path;

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use super::adapter::FetchAdapter;
use super::canon::{read_json, write_canonical, CanonError};
use super::decision::Resolution;
use super::judge::Judge;
use super::manifest::{Manifest, Source};
use super::rights::{self, admits};

/// Who a resolution the pipeline writes is recorded as by.
pub const BY: &str = "pipeline";

/// One proposed source and what the two checks found.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Checked {
    pub id: String,
    pub url: String,
    /// The fields whose ranking chose this source; empty when none did.
    pub fields: Vec<String>,
    /// The rights class, or `None` when it was not asked.
    pub class: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ledger: Option<String>,
}

impl Checked {
    pub fn passes(&self) -> bool {
        !self.fields.is_empty() && self.class.as_deref().is_some_and(admits)
    }
}

/// What the pipeline decided about one draft: admitted, or the owner's with
/// every reason it is not the pipeline's.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Verdict {
    pub admitted: bool,
    pub reasons: Vec<String>,
    pub sources: Vec<Checked>,
}

/// The reasons a draft is not a sources-only addition to `admitted`: a
/// changed key other than `sources`, a changed or removed admitted source,
/// or no new source at all. Empty when the draft only appends sources.
pub fn structural(admitted: &Manifest, draft: &Manifest) -> Vec<String> {
    let mut reasons = Vec::new();
    let (before, after) = (value_of(admitted), value_of(draft));
    let keys: std::collections::BTreeSet<&String> = before
        .as_object()
        .into_iter()
        .chain(after.as_object())
        .flat_map(|map| map.keys())
        .collect();
    for key in keys.into_iter().filter(|k| *k != "sources") {
        if before.get(key) != after.get(key) {
            reasons.push(format!("the draft changes {key}"));
        }
    }
    let kept = admitted.sources.len() <= draft.sources.len()
        && admitted.sources[..] == draft.sources[..admitted.sources.len()];
    if !kept {
        reasons.push("the draft changes or removes an admitted source".into());
    } else if draft.sources.len() == admitted.sources.len() {
        reasons.push("the draft adds no source".into());
    }
    reasons
}

fn value_of(manifest: &Manifest) -> Value {
    serde_json::to_value(manifest).expect("manifest serializes")
}

/// The verdict over the structural reasons and every checked source.
pub fn verdict(mut reasons: Vec<String>, sources: Vec<Checked>) -> Verdict {
    for source in &sources {
        if source.fields.is_empty() {
            reasons.push(format!("{}: no ranking chose it for a field", source.id));
        }
        match source.class.as_deref() {
            Some(class) if admits(class) => {}
            Some(class) => reasons.push(format!("{}: rights class {class}", source.id)),
            None => reasons.push(format!("{}: rights not classified", source.id)),
        }
    }
    Verdict {
        admitted: reasons.is_empty(),
        reasons,
        sources,
    }
}

/// Fetches each proposed source's licence evidence and asks Jev its class.
/// A source no ranking chose is not fetched: it cannot pass anyway.
pub fn classify(
    adapter: &dyn FetchAdapter,
    judge: &Judge<'_>,
    proposed: &[(Source, Vec<String>)],
) -> Result<Vec<Checked>, crate::caller::CallerError> {
    let mut out = Vec::new();
    for (source, fields) in proposed {
        let (class, ledger) = if fields.is_empty() {
            (None, None)
        } else {
            let state = rights::evidence(adapter, &source.url, &source.title);
            let classified = rights::classify(judge, &state)?;
            (Some(classified.class), Some(classified.ledger))
        };
        out.push(Checked {
            id: source.id.clone(),
            url: source.url.clone(),
            fields: fields.clone(),
            class,
            ledger,
        });
    }
    Ok(out)
}

/// The pipeline's verdict on a draft: the structural check first, so a
/// draft the owner must see anyway spends no fetch and no call on rights.
pub fn judge_draft(
    adapter: &dyn FetchAdapter,
    judge: &Judge<'_>,
    admitted: &Manifest,
    draft: &Manifest,
    proposed: &[(Source, Vec<String>)],
) -> Result<Verdict, crate::caller::CallerError> {
    let reasons = structural(admitted, draft);
    if !reasons.is_empty() {
        return Ok(Verdict {
            admitted: false,
            reasons,
            sources: Vec::new(),
        });
    }
    Ok(verdict(reasons, classify(adapter, judge, proposed)?))
}

/// The draft with every passing source in its admitted form.
pub fn admitted_draft(draft: &Manifest, verdict: &Verdict) -> Manifest {
    let mut out = draft.clone();
    for source in out.sources.iter_mut() {
        if let Some(checked) = verdict.sources.iter().find(|c| c.id == source.id) {
            *source = with_rights(source, checked);
        }
    }
    out
}

/// The admitted form of a passing source: its class recorded, and a rights
/// line that says who classified it and how its content is used.
pub fn with_rights(source: &Source, checked: &Checked) -> Source {
    let class = checked.class.clone().unwrap_or_default();
    Source {
        rights: format!(
            "{class}, classified by the pipeline from the page's licence statements (ledger {}); numbers are cited, no text is reproduced",
            checked.ledger.as_deref().unwrap_or("none")
        ),
        rights_class: Some(class),
        ..source.clone()
    }
}

/// Writes the manifest the pipeline admitted.
pub fn write_manifest(path: &Path, manifest: &Manifest) -> Result<(), CanonError> {
    write_canonical(path, &value_of(manifest)).map(|_| ())
}

/// Adds `resolution` to the resolutions file, replacing one with the same
/// id and keeping every other entry and key.
pub fn record_resolution(path: &Path, resolution: &Resolution) -> Result<(), CanonError> {
    let mut value = if path.exists() {
        read_json(path)?
    } else {
        json!({"resolutions": []})
    };
    let mut list: Vec<Value> = value["resolutions"].as_array().cloned().unwrap_or_default();
    list.retain(|r| r["id"] != json!(resolution.id));
    list.push(serde_json::to_value(resolution).expect("resolution serializes"));
    value["resolutions"] = json!(list);
    write_canonical(path, &value).map(|_| ())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn manifest() -> Manifest {
        let mut value = crate::pipeline::manifest::tests::minimal();
        value["schema_version"] = json!(2);
        serde_json::from_value(value).unwrap()
    }

    fn source(id: &str) -> Source {
        Source {
            id: id.into(),
            url: format!("https://example.test/{id}"),
            title: id.into(),
            sha256: None,
            rights: "unstated".into(),
            rights_class: None,
            tables: vec![],
        }
    }

    fn checked(id: &str, fields: &[&str], class: Option<&str>) -> Checked {
        Checked {
            id: id.into(),
            url: format!("https://example.test/{id}"),
            fields: fields.iter().map(|f| f.to_string()).collect(),
            class: class.map(str::to_string),
            ledger: Some("rights:1".into()),
        }
    }

    type Edit = fn(&mut Manifest);

    /// R2: a draft that changes anything but its sources is the owner's.
    #[test]
    fn a_draft_that_changes_a_field_a_bar_an_appearance_trait_or_the_schema_goes_to_the_owner() {
        let admitted = manifest();
        let mut added = admitted.clone();
        added.sources.push(source("P2"));
        assert!(structural(&admitted, &added).is_empty());
        let edits: [(&str, Edit); 5] = [
            ("fields", |m| m.fields[0].condition = "stand_grown".into()),
            ("fields", |m| {
                m.fields[0].bar = crate::pipeline::manifest::Sufficiency::ProxyOnly
            }),
            ("appearance", |m| {
                m.appearance.push(crate::pipeline::manifest::Appearance {
                    trait_name: "bark_colour".into(),
                    sources: vec!["S1".into()],
                })
            }),
            ("schema_version", |m| m.schema_version = 1),
            ("admitted source", |m| m.sources[0].rights = "edited".into()),
        ];
        for (what, edit) in edits {
            let mut draft = added.clone();
            edit(&mut draft);
            let reasons = structural(&admitted, &draft);
            assert!(
                reasons.iter().any(|r| r.contains(what)),
                "{what}: {reasons:?}"
            );
        }
        assert_eq!(
            structural(&admitted, &admitted),
            vec!["the draft adds no source".to_string()]
        );
    }

    /// R1: every new source must be chosen for a field and carry an
    /// admitting class; one that does not sends the whole draft to the owner.
    #[test]
    fn a_restricted_or_unclassified_or_unranked_source_sends_the_draft_to_the_owner() {
        let good = || {
            vec![
                checked("P2", &["height_m"], Some("open-licence")),
                checked("P3", &["dbh_m"], Some("public-cite-only")),
            ]
        };
        assert!(verdict(vec![], good()).admitted);
        let table = [
            (
                checked("P4", &["height_m"], Some("restricted")),
                "rights class restricted",
            ),
            (
                checked("P4", &["height_m"], Some("none")),
                "rights class none",
            ),
            (checked("P4", &["height_m"], None), "rights not classified"),
            (
                checked("P4", &[], Some("open-licence")),
                "no ranking chose it",
            ),
        ];
        for (bad, reason) in table {
            let mut sources = good();
            sources.push(bad);
            let verdict = verdict(vec![], sources);
            assert!(!verdict.admitted, "{reason}");
            assert!(
                verdict.reasons.iter().any(|r| r.contains(reason)),
                "{reason}: {:?}",
                verdict.reasons
            );
        }
        let held = verdict(vec!["the draft changes fields".into()], good());
        assert!(!held.admitted);
    }

    #[test]
    fn an_admitted_source_records_its_class_and_a_resolution_replaces_its_own_id() {
        let admitted = with_rights(
            &source("P2"),
            &checked("P2", &["height_m"], Some("open-licence")),
        );
        assert_eq!(admitted.rights_class.as_deref(), Some("open-licence"));
        assert!(admitted
            .rights
            .starts_with("open-licence, classified by the pipeline"));
        assert!(admitted.rights.contains("no text is reproduced"));
        let dir = std::env::temp_dir().join(format!(
            "jev-admission-{}-{}",
            std::process::id(),
            crate::ledger::new_entry_id()
        ));
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("resolutions.json");
        write_canonical(
            &path,
            &json!({"resolutions": [{"id": "keep"}, {"id": "x"}], "note": "n"}),
        )
        .unwrap();
        let resolution = Resolution {
            id: "x".into(),
            inputs_sha256: Default::default(),
            option: "admit".into(),
            by: BY.into(),
            at: "2026-09-23".into(),
            note: String::new(),
            payload: Value::Null,
        };
        record_resolution(&path, &resolution).unwrap();
        let written = read_json(&path).unwrap();
        assert_eq!(written["note"], "n");
        let list = written["resolutions"].as_array().unwrap();
        assert_eq!(list.len(), 2);
        assert_eq!(list[1]["by"], BY);
    }
}
