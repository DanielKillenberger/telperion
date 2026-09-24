//! Converged is a finish (fn-136). A tuning revision has converged on what
//! the generator can draw when every drawable objective passes, or when the
//! no-progress guard (fn-117) stopped it after rounds that kept nothing. The
//! conductor then records the revision and assembles the owner's packet
//! rather than carrying the stop to the host or asking for another
//! revision; the packet's checklist lists every known gap with its specs.
use serde_json::{json, Value};

use super::gapcheck::PASSING;
use super::Config;
use crate::pipeline::canon::read_json;
use crate::tuning::continuation::Pause;
use crate::tuning::result::EndResult;
use crate::tuning::runaway;
use crate::tuning::unexpressed::Unexpressed;

/// How a stop on no progress with no guard involved begins.
const NO_PROPOSAL: &str = "no supported proposal";
/// The checklist status of a gap the run finishes without.
pub const KNOWN_GAP: &str = "known gap";

/// Why a revision that stopped with `pause`, or ended without one, has
/// converged; `None` when it has not.
pub fn converged(pause: Option<&Pause>, result: &EndResult) -> Option<String> {
    let passing = !result.gaps.is_empty() && result.gaps.iter().all(|g| g.status == PASSING);
    match pause {
        Some(p) if p.reason.starts_with(runaway::REASON) => Some(format!(
            "converged: the no-progress guard stopped the run ({})",
            p.reason
        )),
        Some(p) if passing && p.reason.starts_with(NO_PROPOSAL) => Some(format!(
            "converged: every drawable objective passes and no move is left ({})",
            p.reason
        )),
        None if passing => Some("converged: every drawable objective passes".into()),
        _ => None,
    }
}

/// Every known gap for the packet's checklist: each improvement capability
/// the gate recorded, then each trait the tuning run could not draw.
pub fn known_gaps(config: &Config, traits: &[Unexpressed]) -> Vec<Value> {
    let gate = read_json(&config.paths().artifact("gate")).unwrap_or(Value::Null);
    let capabilities = gate["body"]["capability"]["known_gaps"]
        .as_array()
        .cloned()
        .unwrap_or_default();
    let mut out: Vec<Value> = capabilities
        .iter()
        .map(|g| {
            json!({"id": g["capability"], "priority": KNOWN_GAP, "status": KNOWN_GAP,
                   "captured_by": g["captured_by"], "reason": g["reason"]})
        })
        .collect();
    for gap in traits {
        out.push(
            json!({"id": gap.trait_id, "priority": KNOWN_GAP, "status": KNOWN_GAP,
                        "captured_by": [gap.spec]}),
        );
    }
    out
}
