//! The Gaps stage: every trait still failing, classed by code from what the
//! run recorded, one line each in `gaps.md` with its evidence.
//!
//! - **reachable**: a live dial moved it in a rendered attempt the reviewer
//!   judged; the line names the dial, the two values it was drawn at and the
//!   renders of both sides.
//! - **identity**: the species is not recognisable without it. No capability
//!   assessment at all, a missing capability the assessment classes identity
//!   or leaves unclassed, and a failing trait no dial moved.
//! - **global**: a capability the assessment classes an improvement, or a
//!   trait the tuning config lists unexpressed, with the specs that capture it.
//! - **unsourced**: a profile field no source settled; the generator's
//!   default stands and Tune sets it from the photographs.
//! - **references**: no reference photograph was found and the tuning config
//!   lists none. It stops nothing; Tune refuses until one is recorded.
//!
//! The runner drafts and never mints: the host reviews each line and writes
//! the spec. Identity gaps stop the run until their spec lands or the host
//! reclasses them in `packet/capability.json`.
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::{inventory, tune, Done, Run, Stage, Stop};
use crate::pipeline::canon::{read_json, write_canonical};
use crate::pipeline::stages::capability_class::{self, Class};
use crate::tuning::bundle::Move;
use crate::tuning::result::{Attempt, EndResult, Still, PASSING};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Kind {
    Reachable,
    Identity,
    Global,
    Unsourced,
    References,
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
    let mut gaps = capability(gate, assessment)?;
    gaps.extend(tuned(result));
    Ok(gaps)
}

/// The capabilities the species needs that the generator does not express,
/// classed by the assessment, and no assessment at all as an identity gap.
pub fn capability(gate: &Value, assessment: &Path) -> Result<Vec<Gap>, String> {
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
    Ok(gaps)
}

/// The traits tuning left failing, and the ones the config lists unexpressed.
fn tuned(result: &EndResult) -> Vec<Gap> {
    let mut gaps = Vec::new();
    for known in &result.known_gaps {
        gaps.push(Gap {
            trait_id: known.trait_id.clone(),
            kind: Kind::Global,
            evidence: vec!["listed unexpressed in the tuning config".into()],
            specs: vec![known.spec.clone()],
        });
    }
    for entry in result.gaps.iter().filter(|g| g.status != PASSING) {
        // Only a move that rendered and that the reviewer judged is evidence
        // a live dial reaches the trait: its dial, both values and the
        // renders of both sides.
        let moves: Vec<String> = entry
            .attempts
            .iter()
            .filter(|a| a.feasible && a.review.is_some())
            .flat_map(|a| a.moves.iter().map(move |m| ab(m, a)))
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
    gaps
}

/// One reachable move: the dial, its two values, and the renders at each.
fn ab(m: &Move, attempt: &Attempt) -> String {
    let side = |stills: &[Still]| -> String {
        let links: Vec<String> = stills
            .iter()
            .map(|s| format!("[{} seed {}]({})", s.view, s.seed, s.path))
            .collect();
        match links.is_empty() {
            true => "no render recorded".into(),
            false => links.join(", "),
        }
    };
    format!(
        "{} {} -> {} (A {}; B {})",
        m.dial,
        m.from,
        m.to,
        side(&attempt.before),
        side(&attempt.after)
    )
}

pub struct Gaps;

impl Stage for Gaps {
    fn name(&self) -> &'static str {
        "gaps"
    }

    fn inputs(&self, run: &Run) -> Result<Vec<PathBuf>, String> {
        let p = &run.paths;
        Ok(vec![
            tune::result(&run.out()),
            p.artifact("gate"),
            p.packet("capability"),
            p.packet("profile"),
            run.tuning.clone(),
            inventory::found(&run.out()),
        ])
    }

    fn outputs(&self, run: &Run) -> Result<Vec<PathBuf>, String> {
        let (json, md) = files(&run.out());
        Ok(vec![json, md])
    }

    fn run(&self, run: &Run) -> Result<Done, String> {
        let (p, out) = (&run.paths, run.out());
        let read = |p: &Path| read_json(p).map_err(|e| e.to_string());
        let tuned: EndResult =
            serde_json::from_value(read(&tune::result(&out))?).map_err(|e| e.to_string())?;
        let mut gaps = classify(&read(&p.artifact("gate"))?, &p.packet("capability"), &tuned)?;
        gaps.extend(unsourced(&read(&p.packet("profile"))?));
        gaps.extend(references(&run.tuning, &out)?);
        write(&out, &gaps)?;
        let count = |k: Kind| gaps.iter().filter(|g| g.kind == k).count();
        Ok(Done::Ran(format!(
            "{} reachable, {} identity, {} global, {} unsourced",
            count(Kind::Reachable),
            count(Kind::Identity),
            count(Kind::Global),
            count(Kind::Unsourced)
        )))
    }

    fn stop(&self, run: &Run) -> Result<Option<Stop>, String> {
        let ids = identity(&run.out())?;
        Ok((!ids.is_empty()).then_some(Stop::IdentityGaps(ids)))
    }
}

/// The references gap, when the run has no photograph to compare against.
pub fn references(template: &Path, out: &Path) -> Result<Option<Gap>, String> {
    Ok(inventory::none(template, out)?.then(|| Gap {
        trait_id: "references".into(),
        kind: Kind::References,
        evidence: vec!["no reference photograph: the Profile stage kept none (<run-dir>/cache/photos/find.json) and the tuning config lists none; Tune refuses until one is recorded".into()],
        specs: vec![],
    }))
}

/// Sets the references line of `gaps.md` as the Profile stage finds it,
/// before the Gaps stage classes the rest.
pub fn note_references(template: &Path, out: &Path) -> Result<(), String> {
    let (json_path, _) = files(out);
    let recorded = read_json(&json_path).map_or(Value::Null, |v| v["gaps"].clone());
    let mut gaps: Vec<Gap> = serde_json::from_value(recorded).unwrap_or_default();
    gaps.retain(|g| g.kind != Kind::References);
    gaps.extend(references(template, out)?);
    write(out, &gaps)
}

/// The profile fields no source settled, which the profile marks unsourced.
pub fn unsourced(profile: &Value) -> Vec<Gap> {
    let metrics = profile["profiles"][0]["metrics"].as_object();
    metrics
        .into_iter()
        .flatten()
        .filter(|(_, m)| m["classification"] == "unsourced")
        .map(|(field, m)| Gap {
            trait_id: field.clone(),
            kind: Kind::Unsourced,
            evidence: vec![format!(
                "{}; the generator's default stands and Tune sets it from the photographs",
                m["note"].as_str().unwrap_or("no source settled it")
            )],
            specs: vec![],
        })
        .collect()
}

/// Writes `gaps.json` and `gaps.md` for `gaps`.
pub fn write(out: &Path, gaps: &[Gap]) -> Result<(), String> {
    let (json_path, md_path) = files(out);
    let value = serde_json::json!({"schema": "runner-gaps", "schema_version": 1, "gaps": gaps});
    write_canonical(&json_path, &value).map_err(|e| e.to_string())?;
    std::fs::write(&md_path, markdown(gaps)).map_err(|e| e.to_string())
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
        (Kind::Unsourced, "Unsourced: no source settled it"),
        (
            Kind::References,
            "References: no photograph to compare against",
        ),
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
