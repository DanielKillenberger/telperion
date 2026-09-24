//! The class of a capability the generator does not express yet (fn-136).
//!
//! The host's capability assessment (`packet/capability.json`) classes each
//! missing capability. An `identity` capability is one the species is not
//! recognisable without, and it blocks the run at the gate. An `improvement`
//! only adds realism: the backlog specs it names capture it, and the run
//! carries it as a known gap and finishes without it. Every class carries its
//! reason and who decided it; the owner reverses any class in the packet.
//! A missing capability nobody classified blocks, as it always has.
use std::collections::HashSet;
use std::path::Path;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::pipeline::canon::read_json;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Class {
    Identity,
    Improvement,
}

/// One missing capability as the assessment classes it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Classified {
    pub capability: String,
    pub class: Class,
    pub reason: String,
    /// The backlog specs that capture an improvement.
    #[serde(default)]
    pub captured_by: Vec<String>,
    pub decided_by: String,
    pub decided_on: String,
}

/// A missing improvement the run finishes without, and the specs that
/// capture it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct KnownGap {
    pub capability: String,
    pub captured_by: Vec<String>,
    pub reason: String,
}

fn blank(text: &str) -> bool {
    text.trim().is_empty()
}

/// The assessment's classes, from the record or from the latest of a list of
/// rounds. An absent file, or one with no `classes`, classes nothing; a
/// class that cannot be trusted refuses the whole list.
pub fn read(path: &Path) -> Result<Vec<Classified>, String> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let raw = read_json(path).map_err(|err| err.to_string())?;
    // A list of rounds is read at its latest round.
    let latest = match raw {
        Value::Array(rounds) => rounds.last().cloned().unwrap_or_default(),
        record => record,
    };
    if latest["classes"].is_null() {
        return Ok(Vec::new());
    }
    let classes: Vec<Classified> =
        serde_json::from_value(latest["classes"].clone()).map_err(|err| err.to_string())?;
    let mut seen = HashSet::new();
    for c in &classes {
        let name = &c.capability;
        if blank(name) || !seen.insert(name) {
            return Err(format!("capability {name:?} is blank or classified twice"));
        }
        if blank(&c.reason) || blank(&c.decided_by) || blank(&c.decided_on) {
            return Err(format!("{name} has no reason or no decision"));
        }
        let specs = c.captured_by.iter().filter(|s| !blank(s)).count();
        if c.class == Class::Improvement && (specs == 0 || specs != c.captured_by.len()) {
            return Err(format!("improvement {name} names no backlog spec"));
        }
    }
    Ok(classes)
}

/// Splits the missing capabilities into those that block the run and the
/// known gaps it finishes without.
pub fn split(missing: &[String], classes: &[Classified]) -> (Vec<String>, Vec<KnownGap>) {
    let mut blocking = Vec::new();
    let mut known = Vec::new();
    for name in missing {
        match classes.iter().find(|c| &c.capability == name) {
            Some(c) if c.class == Class::Improvement => known.push(KnownGap {
                capability: name.clone(),
                captured_by: c.captured_by.clone(),
                reason: c.reason.clone(),
            }),
            _ => blocking.push(name.clone()),
        }
    }
    (blocking, known)
}
