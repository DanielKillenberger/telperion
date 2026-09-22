//! What the reviewer is asked about, and what the two trials' own stills say
//! before anyone is paid to look at them.
use super::{Priority, Request, Verdict, UNCALIBRATED, VERSION};
use crate::sha256_hex;
use crate::tuning::{
    engine::Run,
    evaluation::{Image, Trial},
    priority::Gap,
};
use serde_json::{json, Value};
use std::collections::BTreeMap;

/// One image as the prompt may see it: the role it plays on the sheet and the
/// digest of its bytes, never the file it lives in.
pub(crate) fn redacted(role: &str, image: &Image) -> Value {
    json!({"role":role,"sha256":image.sha256})
}

/// The request with every path taken out of it. The adapter serializes this
/// into the prompt, because a render's file name can carry the trial key, the
/// seed or the label the reviewer is not allowed to read off it.
pub fn prompt_request(request: &Request) -> Value {
    json!({"schema":request.schema,"target_species":request.target_species,
        "view":request.view,"seed":request.seed,
        "references":request.references.iter().map(|i| redacted("reference",i))
            .collect::<Vec<_>>(),
        "a":redacted("a",&request.a),"b":redacted("b",&request.b),
        "priorities":request.priorities,"owner_notes":request.owner_notes})
}

/// Which side the candidate is shown as. Both keys decide it together, so the
/// answer cannot be read off either one alone, and it is recorded either way.
pub fn candidate_side(current_key: &str, candidate_key: &str) -> String {
    let digest = sha256_hex(format!("{current_key}|{candidate_key}").as_bytes());
    let last = digest.as_bytes()[digest.len() - 1];
    if last % 2 == 0 { "a" } else { "b" }.into()
}

/// What the two trials' own stills say before anyone is paid to look at them.
#[derive(Debug)]
pub enum Look {
    /// Every still the two share is byte for byte the same picture. The move
    /// changed nothing visible by itself, and no reviewer is asked.
    Inert,
    /// Ask about this request. The flag is false when no view a priority named
    /// differs, so the review fell back to another view they share.
    Ask(Request, String, bool),
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

/// What code settles by itself when the candidate draws the current tree.
pub fn inert() -> Verdict {
    Verdict {
        per_priority: BTreeMap::new(),
        improved: String::new(),
        missing: String::new(),
        regressions: vec![],
        ledger: String::new(),
        model: "code".into(),
        candidate_is: "n/a".into(),
        uncalibrated: UNCALIBRATED.into(),
        inert: true,
        note: Some(
            "render byte-identical to the current tree; this move changes nothing visible by itself"
                .into(),
        ),
    }
}

pub(in crate::tuning) fn still(trial: &Trial, view: &str, seed: u32) -> Option<Image> {
    trial
        .comparisons
        .iter()
        .flat_map(|c| c.images.iter())
        .find(|i| i.view == view && i.seed == seed)
        .cloned()
}

/// The request for one candidate against the tree it came from.
pub fn request(
    state: &Run,
    species: &str,
    references: &[Image],
    current: &Trial,
    candidate: &Trial,
    priorities: &[Gap],
) -> Result<Look, String> {
    if priorities.is_empty() {
        return Err("no tuning-routed priority to review".into());
    }
    let shared = shared_views(current, candidate, state.seed);
    if shared.is_empty() {
        return Err("no shared still for progress review".into());
    }
    if shared.iter().all(|(_, differs)| !differs) {
        return Ok(Look::Inert);
    }
    let named = priorities
        .iter()
        .flat_map(|g| g.views.iter().cloned())
        .find(|v| shared.iter().any(|(s, differs)| s == v && *differs));
    let from_priority = named.is_some();
    let view = named
        .or_else(|| {
            shared
                .iter()
                .find(|(_, differs)| *differs)
                .map(|(v, _)| v.clone())
        })
        .ok_or("no shared still for progress review")?;
    let (current_still, candidate_still) = (
        still(current, &view, state.seed).ok_or("no shared still for progress review")?,
        still(candidate, &view, state.seed).ok_or("no shared still for progress review")?,
    );
    let side = candidate_side(&current.key, &candidate.key);
    let (a, b) = if side == "a" {
        (candidate_still, current_still)
    } else {
        (current_still, candidate_still)
    };
    let request = Request {
        schema: VERSION.into(),
        target_species: species.into(),
        view: view.clone(),
        seed: state.seed,
        references: references
            .iter()
            .filter(|i| i.view == view)
            .cloned()
            .collect(),
        a,
        b,
        priorities: priorities
            .iter()
            .map(|g| Priority {
                id: g.id.clone(),
                observation: g.observation.clone(),
            })
            .collect(),
        owner_notes: state.owner_notes.clone(),
    };
    request.verify()?;
    Ok(Look::Ask(request, side, from_priority))
}
