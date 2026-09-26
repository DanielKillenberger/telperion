//! What a species run tunes toward: the owner's priorities first, then every
//! trait its references show that the generator can draw.
//!
//! The palm's three revisions aimed at the owner's three notes alone, so the
//! materials track found "no supported dial" in 13 rounds: nothing it could
//! move was anybody's objective. Code turns each expressible `core` and
//! `secondary` inventory trait into an objective; a `variation` trait and one
//! listed unexpressed are recorded, never aimed at.
use super::{
    bundle::Track,
    engine::{Run, Services},
    priority::{Checkpoint, Evidence, Gap},
    reference_first::{Inventory, Priority},
    state::Cell,
    unexpressed::Unexpressed,
};
use serde::{Deserialize, Serialize};

/// An inventory trait that is not an objective, and why.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LeftOut {
    #[serde(rename = "trait")]
    pub trait_id: String,
    pub reason: String,
}

/// The views a trait is judged at: the shots of its references that the run
/// assesses, each once, in citation order.
fn views(inventory: &Inventory, cited: &[String], required: &[Cell]) -> Vec<String> {
    let mut out: Vec<String> = vec![];
    for id in cited {
        let Some(shot) = inventory.request.references.iter().find(|r| &r.id == id) else {
            continue;
        };
        let view = &shot.image.view;
        if required.iter().any(|c| &c.view == view) && !out.contains(view) {
            out.push(view.clone());
        }
    }
    out
}

/// The packet's evidence for one trait: the current render of each view it is
/// judged at, then each reference it cites. A view with no render is dropped.
fn cited(
    inventory: &Inventory,
    references: &[String],
    views: Vec<String>,
    evidence: &[Evidence],
) -> (Vec<String>, Vec<String>) {
    let mut ids = vec![];
    let mut kept = vec![];
    for view in views {
        if let Some(render) = evidence
            .iter()
            .find(|e| e.role == "render" && e.image.view == view)
        {
            ids.push(render.id.clone());
            kept.push(view);
        }
    }
    for id in references {
        let Some(shot) = inventory.request.references.iter().find(|r| &r.id == id) else {
            continue;
        };
        let found = evidence
            .iter()
            .find(|e| e.role == "reference" && e.image.sha256 == shot.image.sha256);
        if let Some(e) = found.filter(|e| !ids.contains(&e.id)) {
            ids.push(e.id.clone());
        }
    }
    (ids, kept)
}

/// Every inventory trait, as an objective or as a recorded reason it is not.
pub fn from_inventory(
    inventory: &Inventory,
    unexpressed: &[Unexpressed],
    required: &[Cell],
    evidence: &[Evidence],
) -> (Vec<Gap>, Vec<LeftOut>) {
    let (mut gaps, mut left_out) = (vec![], vec![]);
    let mut leave = |id: &str, reason: String| {
        left_out.push(LeftOut {
            trait_id: id.into(),
            reason,
        })
    };
    for t in &inventory.traits {
        if t.priority == Priority::Variation {
            leave(&t.id, "variation: recorded, not an objective".into());
            continue;
        }
        if let Some(u) = unexpressed.iter().find(|u| u.trait_id == t.id) {
            leave(&t.id, format!("unexpressed until {} lands", u.spec));
            continue;
        }
        let judged = views(inventory, &t.reference_ids, required);
        let (evidence_ids, views) = cited(inventory, &t.reference_ids, judged, evidence);
        let has = |role: &str| {
            evidence_ids
                .iter()
                .any(|id| evidence.iter().any(|e| &e.id == id && e.role == role))
        };
        if views.is_empty() || !has("render") || !has("reference") {
            leave(
                &t.id,
                "no assessed view renders it beside its references".into(),
            );
            continue;
        }
        gaps.push(Gap {
            id: t.id.clone(),
            observation: t.observation.clone(),
            evidence_ids,
            views,
            track: None,
        });
    }
    (gaps, left_out)
}

/// The proposed approval: the owner's priorities, then the inventory's
/// objectives not already among them.
pub fn propose(owner: Vec<Gap>, traits: Vec<Gap>) -> Vec<Gap> {
    let mut out = owner;
    for gap in traits {
        if !out.iter().any(|g| g.id == gap.id) {
            out.push(gap);
        }
    }
    out
}

/// A gap's track must be one the run declares.
pub fn verify_tracks(ordered: &[Gap], tracks: &[Track]) -> Result<(), String> {
    match ordered.iter().find(|g| {
        g.track
            .as_ref()
            .is_some_and(|t| !tracks.iter().any(|k| &k.name == t))
    }) {
        Some(gap) => Err(format!(
            "priority {} names track {}, which the run does not declare",
            gap.id,
            gap.track.as_deref().unwrap_or_default()
        )),
        None => Ok(()),
    }
}

/// A track's objectives, in approval order: its named gaps and every
/// unassigned one.
pub fn of_track(objectives: &[Gap], track: &Track) -> Vec<Gap> {
    objectives
        .iter()
        .filter(|g| g.belongs_to(&track.name))
        .cloned()
        .collect()
}

/// The owner's priorities from the last approval they gave, still citable in
/// this packet, in their order.
fn owner(state: &Run, evidence: &[Evidence]) -> Vec<Gap> {
    let Some(last) = state.approval.as_ref() else {
        return vec![];
    };
    last.ordered
        .iter()
        .filter(|g| g.id.starts_with("owner-"))
        .filter(|g| {
            g.evidence_ids
                .iter()
                .all(|id| evidence.iter().any(|e| &e.id == id))
                && g.views.iter().all(|v| {
                    evidence
                        .iter()
                        .any(|e| e.role == "render" && &e.image.view == v)
                })
        })
        .cloned()
        .collect()
}

/// Writes the proposed approval onto a fresh checkpoint.
pub(super) fn offer(
    state: &Run,
    services: &dyn Services,
    checkpoint: &mut Checkpoint,
) -> Result<(), String> {
    let (traits, left_out) = match services.inventory()? {
        Some(inventory) => from_inventory(
            &inventory,
            &services.unexpressed(),
            &state.required,
            &checkpoint.evidence,
        ),
        None => (vec![], vec![]),
    };
    checkpoint.proposed = propose(owner(state, &checkpoint.evidence), traits);
    checkpoint.left_out = left_out;
    checkpoint.verify()
}
