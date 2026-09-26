//! What every paid look shares: the adapter it shells out to, the objectives
//! it is asked about, the stills it shows, and the words a later question
//! reads back. The contact sheet (`sheet`) is the one comparison a round buys.
use super::{
    engine::Run,
    evaluation::{Image, Trial},
    priority::Gap,
    vision,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::path::PathBuf;

/// Most objectives one look is asked about.
pub const MAX_PRIORITIES: usize = 16;
/// What a trial records when the look it paid for failed.
pub const REVIEW_FAILED: &str = "review failed; attempt charged";
/// Why a round adopted nothing.
pub const STALL: &str = "bundle stall; no strength judged better";

/// The adapter a look is asked through, and the protocol it was written
/// against. The protocol's bytes join the run's identity.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Adapter {
    pub adapter: vision::Adapter,
    pub protocol: PathBuf,
}

/// One objective as the reviewer is shown it.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Priority {
    pub id: String,
    pub observation: String,
}

/// The revision's objectives, in their order.
pub fn objectives(state: &Run) -> Vec<Gap> {
    state
        .approved_priorities()
        .map(|a| a.ordered.clone())
        .unwrap_or_default()
}

/// One track's objectives: those named to it and every unassigned one.
pub fn track_objectives(state: &Run, track: &super::bundle::Track) -> Vec<Gap> {
    super::objectives::of_track(&objectives(state), track)
}

/// The reviewer's words about one attempt, for whoever is asked next: what
/// it moved, how it was judged, and what took it back.
pub fn words(trial: &Trial) -> Option<Value> {
    let mut out = super::bundle::words(trial)?;
    if let Some(veto) = &trial.vetoed {
        out["rolled_back"] = json!({"reasons":veto.reasons,"ledger":veto.ledger,
            "meaning":"this move was adopted and the all-view review that followed took it back"});
    }
    Some(out)
}

/// One image as the prompt may see it: the role it plays and the digest of
/// its bytes, never the file it lives in.
pub(crate) fn redacted(role: &str, image: &Image) -> Value {
    json!({"role":role,"sha256":image.sha256})
}

/// The still a trial holds at `view` and `seed`.
pub(in crate::tuning) fn still(trial: &Trial, view: &str, seed: u32) -> Option<Image> {
    trial
        .comparisons
        .iter()
        .flat_map(|c| c.images.iter())
        .find(|i| i.view == view && i.seed == seed)
        .cloned()
}

/// Every view both trials hold a still for at the run's seed, in the
/// candidate's own order, and whether the two stills differ.
pub(in crate::tuning) fn shared_views(
    current: &Trial,
    candidate: &Trial,
    seed: u32,
) -> Vec<(String, bool)> {
    let stills = |t: &Trial| {
        t.comparisons
            .iter()
            .flat_map(|c| c.images.iter())
            .filter(|i| i.seed == seed)
            .map(|i| (i.view.clone(), i.sha256.clone()))
            .collect::<Vec<_>>()
    };
    let mine = stills(current);
    let mut seen = vec![];
    for (view, sha) in stills(candidate) {
        if seen.iter().any(|(v, _): &(String, bool)| v == &view) {
            continue;
        }
        if let Some((_, theirs)) = mine.iter().find(|(v, _)| v == &view) {
            seen.push((view, &sha != theirs));
        }
    }
    seen
}
