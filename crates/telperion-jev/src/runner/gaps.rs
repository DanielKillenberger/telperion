//! The Gaps stage: every trait still failing, classed by code from what the
//! run recorded, one line each in `gaps.md` with its evidence.
//!
//! - **reachable**: a live dial moved it during tuning; the line names the
//!   dial and the two values it was drawn at.
//! - **identity**: the species is not recognisable without it. No capability
//!   assessment at all, a missing capability the assessment classes identity
//!   or leaves unclassed, and a failing trait no dial moved.
//! - **global**: a capability the assessment classes an improvement, or a
//!   trait the tuning config lists unexpressed, with the specs that capture it.
//!
//! The runner drafts and never mints: the host reviews each line and writes
//! the spec. Identity gaps stop the run until their spec lands or the host
//! reclasses them in `packet/capability.json`.
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::pipeline::canon::{read_json, write_canonical};
use crate::pipeline::stages::capability_class::{self, Class};
use crate::tuning::result::{EndResult, PASSING};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Kind {
    Reachable,
    Identity,
    Global,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Gap {
    #[serde(rename = "trait")]
    pub trait_id: String,
    pub kind: Kind,
    pub evidence: Vec<String>,
    /// The specs that capture it, where the record names any.
    pub specs: Vec<String>,
}

pub fn files(out: &Path) -> (PathBuf, PathBuf) {
    (out.join("gaps.json"), out.join("gaps.md"))
}

/// Classes every failing trait from the gate's capability record, the
/// assessment's classes and the tuning result.
pub fn classify(gate: &Value, assessment: &Path, result: &EndResult) -> Result<Vec<Gap>, String> {
    let classes = capability_class::read(assessment)?;
    let capability = &gate["body"]["capability"];
    let names = |key: &str| -> Vec<String> {
        capability[key]
            .as_array()
            .into_iter()
            .flatten()
            .filter_map(|v| v.as_str().or(v["capability"].as_str()).map(str::to_string))
            .collect()
    };
    let mut gaps = Vec::new();
    // No recorded requirement is no assessment at all, which blocks as an
    // unclassed capability does.
    if names("required").is_empty() {
        gaps.push(Gap {
            trait_id: "capability-assessment".into(),
            kind: Kind::Identity,
            evidence: vec![
                "no capability assessment is recorded; the host writes packet/capability.json"
                    .into(),
            ],
            specs: vec![],
        });
    }
    for name in names("missing").into_iter().chain(names("unrecognised")) {
        let class = classes.iter().find(|c| c.capability == name);
        let kind = match class.map(|c| c.class) {
            Some(Class::Improvement) => Kind::Global,
            _ => Kind::Identity,
        };
        let evidence = match class {
            Some(c) => format!(
                "capability {name}: {} ({}, {})",
                c.reason, c.decided_by, c.decided_on
            ),
            None => format!(
                "capability {name}: the generator does not express it and no class is recorded"
            ),
        };
        let specs = class.map(|c| c.captured_by.clone()).unwrap_or_default();
        gaps.push(Gap {
            trait_id: name,
            kind,
            evidence: vec![evidence],
            specs,
        });
    }
    for known in &result.known_gaps {
        gaps.push(Gap {
            trait_id: known.trait_id.clone(),
            kind: Kind::Global,
            evidence: vec!["listed unexpressed in the tuning config".into()],
            specs: vec![known.spec.clone()],
        });
    }
    for entry in result.gaps.iter().filter(|g| g.status != PASSING) {
        let moves: Vec<String> = entry
            .attempts
            .iter()
            .flat_map(|a| &a.moves)
            .map(|m| format!("{} {} -> {}", m.dial, m.from, m.to))
            .collect();
        let mut evidence: Vec<String> = entry.reviewer_words.clone();
        evidence.extend(
            entry
                .stills
                .iter()
                .map(|s| format!("still {} ({} seed {})", s.path, s.view, s.seed)),
        );
        let kind = match moves.is_empty() {
            true => {
                evidence.insert(0, format!("{}: no live dial moved it", entry.status));
                Kind::Identity
            }
            false => {
                evidence.splice(0..0, moves);
                Kind::Reachable
            }
        };
        gaps.push(Gap {
            trait_id: entry.id.clone(),
            kind,
            evidence,
            specs: vec![],
        });
    }
    Ok(gaps)
}

/// Writes `gaps.json` and `gaps.md`; the word counts each class.
pub fn run(gate: &Path, assessment: &Path, result: &Path, out: &Path) -> Result<String, String> {
    let read = |p: &Path| read_json(p).map_err(|e| e.to_string());
    let tuned: EndResult = serde_json::from_value(read(result)?).map_err(|e| e.to_string())?;
    let gaps = classify(&read(gate)?, assessment, &tuned)?;
    let (json_path, md_path) = files(out);
    let value = serde_json::json!({"schema": "runner-gaps", "schema_version": 1, "gaps": gaps});
    write_canonical(&json_path, &value).map_err(|e| e.to_string())?;
    std::fs::write(&md_path, markdown(&gaps)).map_err(|e| e.to_string())?;
    let count = |k: Kind| gaps.iter().filter(|g| g.kind == k).count();
    Ok(format!(
        "{} reachable, {} identity, {} global",
        count(Kind::Reachable),
        count(Kind::Identity),
        count(Kind::Global)
    ))
}

/// The identity gaps on disk: what stops the run.
pub fn identity(out: &Path) -> Result<Vec<String>, String> {
    let (json_path, _) = files(out);
    let value = read_json(&json_path).map_err(|e| e.to_string())?;
    let gaps: Vec<Gap> =
        serde_json::from_value(value["gaps"].clone()).map_err(|e| e.to_string())?;
    Ok(gaps
        .into_iter()
        .filter(|g| g.kind == Kind::Identity)
        .map(|g| g.trait_id)
        .collect())
}

fn markdown(gaps: &[Gap]) -> String {
    let mut out = String::from(
        "# Gaps\n\nEvery trait still failing after tuning, classed by the runner from what the run \
         recorded. The host reviews each line and writes any spec; the runner mints none.\n",
    );
    for (kind, title) in [
        (Kind::Identity, "Identity: the species waits on these"),
        (Kind::Global, "Global: backlog"),
        (Kind::Reachable, "Reachable: a live dial moves it"),
    ] {
        out.push_str(&format!("\n## {title}\n\n"));
        let listed: Vec<&Gap> = gaps.iter().filter(|g| g.kind == kind).collect();
        if listed.is_empty() {
            out.push_str("None.\n");
        }
        for gap in listed {
            let specs = match gap.specs.is_empty() {
                true => String::new(),
                false => format!(" ({})", gap.specs.join(", ")),
            };
            out.push_str(&format!(
                "- **{}**{specs}: {}\n",
                gap.trait_id,
                gap.evidence.join("; ")
            ));
        }
    }
    out
}
