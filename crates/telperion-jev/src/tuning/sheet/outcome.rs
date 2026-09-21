//! What one variant did, recorded on the trial that drew it.
//!
//! The sheet answers about the whole set at once; this is the one render's
//! row of it, kept with the words about the set so that whoever is asked next
//! reads the reviewer rather than a number.
use super::{Movement, Render, Verdict, UNCALIBRATED};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Outcome {
    /// The number this render carried on the sheet, or empty when it was not
    /// on one.
    pub label: String,
    pub per_priority: BTreeMap<String, Movement>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub breaks: Vec<String>,
    pub improved: String,
    pub missing: String,
    pub ledger: String,
    pub model: String,
    pub uncalibrated: String,
    /// Code settled this one without asking anyone: the render is the current
    /// tree's render byte for byte.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub inert: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
}

impl Outcome {
    /// One render's row of a sheet the reviewer answered.
    pub fn of(verdict: &Verdict, render: &Render) -> Self {
        Outcome {
            label: render.label.clone(),
            per_priority: render.per_priority.clone(),
            breaks: render.breaks.clone(),
            improved: verdict.improved.clone(),
            missing: verdict.missing.clone(),
            ledger: verdict.ledger.clone(),
            model: verdict.model.clone(),
            uncalibrated: UNCALIBRATED.into(),
            inert: false,
            note: None,
        }
    }
    /// A variant no reviewer was shown, and why code kept it off the sheet.
    pub fn unshown(inert: bool, reason: &str) -> Self {
        Outcome {
            label: String::new(),
            per_priority: BTreeMap::new(),
            breaks: vec![],
            improved: String::new(),
            missing: String::new(),
            ledger: String::new(),
            model: "code".into(),
            uncalibrated: UNCALIBRATED.into(),
            inert,
            note: Some(reason.into()),
        }
    }
}
