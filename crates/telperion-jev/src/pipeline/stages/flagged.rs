//! What select owes the gate beyond its picks (fn-131). A value verify
//! flagged and a resolution dropped (`drop-value`) or sent for another
//! source (`replace-source`) leaves the packet; a required field select
//! cannot fill, or whose value left it, files `requirements-unmet`, which
//! the pipeline searches again for (`pipeline::search`) before the owner
//! has it. No gap passes silently.

use std::collections::BTreeMap;

use serde_json::{json, Value};

use crate::pipeline::canon::canonical_sha256;
use crate::pipeline::consume::{sources_sha256, REQUIREMENTS_UNMET};
use crate::pipeline::decision::{Decision, DecisionParts};
use crate::pipeline::requirements::{asked, Asked};
use crate::pipeline::stage::{Context, StageError};

use super::select::STAGE;

/// The claim kinds verify files per value, and the options select acts on.
pub const CLAIM_KINDS: [&str; 2] = ["claim-contradicted", "claim-unsupported"];
pub const DROP_VALUE: &str = "drop-value";
pub const REPLACE_SOURCE: &str = "replace-source";

/// A value a resolution took out of the packet: the decision, its option,
/// and the source and span the claim was filed on.
pub struct Flag {
    pub id: String,
    pub option: String,
    pub source: Value,
    pub span: Value,
}

impl Flag {
    /// Whether the value select picked now is the one flagged: a value from
    /// another source or span is a new value, and verify judges it again.
    pub fn names(&self, entry: &Value) -> bool {
        entry["source"] == self.source && entry["span"] == self.span
    }

    pub fn reason(&self) -> String {
        format!("{}: {}", self.option, self.id)
    }
}

/// The bound resolutions of claim decisions that drop their value, keyed
/// by the value's JSON Pointer.
pub fn flags(ctx: &Context) -> BTreeMap<String, Flag> {
    ctx.decisions
        .iter()
        .filter(|d| CLAIM_KINDS.contains(&d.kind.as_str()))
        .filter_map(|d| {
            let option = d.resolution.as_ref()?.option.clone();
            let pointer = d.payload["pointer"].as_str()?.to_string();
            [DROP_VALUE, REPLACE_SOURCE]
                .contains(&option.as_str())
                .then(|| {
                    let flag = Flag {
                        id: d.id.clone(),
                        option,
                        source: d.payload["source"].clone(),
                        span: d.payload["span"].clone(),
                    };
                    (pointer, flag)
                })
        })
        .collect()
}

/// The checksum of the drops select acts on, which its key covers.
pub fn flags_sha256(flags: &BTreeMap<String, Flag>) -> String {
    let listed: Vec<Value> = flags
        .iter()
        .map(|(pointer, f)| json!([pointer, f.id, f.option, f.source, f.span]))
        .collect();
    canonical_sha256(&json!(listed))
}

/// A required measured field select left unfilled. The gap the search aims
/// at is quality's, or the want of any size when quality named none.
pub fn unmet(
    ctx: &Context,
    field: &str,
    reason: &str,
    quality: &Value,
    inputs: &BTreeMap<String, String>,
) -> Result<Decision, StageError> {
    let manifest = &ctx.admitted.manifest;
    let gap = match quality["dominant_gap"].as_str() {
        Some(gap) if gap != "none" => gap,
        _ => match asked(manifest, field) {
            Asked::Age => "no_age_indexed_points",
            Asked::Mature => "no_mature_size",
            Asked::Rate => "no_growth_rate",
        },
    };
    let sources: Vec<&str> = manifest.sources.iter().map(|s| s.id.as_str()).collect();
    Ok(Decision::new(
        DecisionParts {
            species: &manifest.species,
            stage: STAGE,
            kind: REQUIREMENTS_UNMET,
            field: Some(field),
            age_years: None,
        },
        &["fit", "generate"],
        inputs.clone(),
        vec![],
        json!({
            "field": field, "level": quality["level"], "bar": quality["bar"],
            "dominant_gap": gap, "reason": reason, "sources_tried": sources,
            "sources_sha256": sources_sha256(&ctx.paths.manifest())?,
        }),
        &["add-sources"],
        "The requirements table asks this field and select filled no value for it. The pipeline searches again for a source aimed at the gap, two rounds at most; after them it is NEEDS_HUMAN and the owner adds sources.",
    ))
}
