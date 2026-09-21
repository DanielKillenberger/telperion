//! Which variants a sheet is worth showing, and the sheet it then asks for.
//!
//! A variant that draws the current tree, or another variant's tree, teaches
//! the reviewer nothing and costs a render slot. Code settles those here, off
//! the stills' own bytes, before anyone is paid to look.
use super::{plan, Plan, Request, VERSION};
use crate::tuning::{
    engine::Run,
    evaluation::Image,
    priority::Gap,
    progress::{self, Priority},
};
use serde_json::{json, Value};

pub const INERT: &str =
    "inert: render byte-identical to the current tree; this bundle changes nothing visible at this strength";

/// A variant the sheet will not show, and why.
#[derive(Debug, Clone)]
pub struct NotShown {
    pub trial: usize,
    pub inert: bool,
    pub reason: String,
}

/// What the round's variants look like before anyone is paid to look at them.
#[derive(Debug)]
pub struct Look {
    /// The trials the sheet shows beside the current tree.
    pub shown: Vec<usize>,
    pub not_shown: Vec<NotShown>,
    /// Absent when nothing survives: a sheet needs the current tree and at
    /// least one variant that differs from it.
    pub plan: Option<Plan>,
}

/// The view the sheet is taken at: one a tuning priority named and some
/// variant changed, else any view some variant changed.
fn view_for(alive: &[(usize, Vec<(String, bool)>)], priorities: &[Gap]) -> Option<String> {
    let changed = |view: &str| {
        alive
            .iter()
            .any(|(_, shared)| shared.iter().any(|(v, differs)| v == view && *differs))
    };
    priorities
        .iter()
        .flat_map(|gap| gap.views.iter().cloned())
        .find(|view| changed(view))
        .or_else(|| {
            alive
                .iter()
                .flat_map(|(_, shared)| shared.iter())
                .find(|(_, differs)| *differs)
                .map(|(view, _)| view.clone())
        })
}

/// The sheet for one round's variants. `variants` arrives in ascending
/// strength, so of two variants that draw the same tree the smaller strength
/// is the one that stays on the sheet.
pub fn look(
    state: &Run,
    species: &str,
    references: &[Image],
    current: usize,
    variants: &[usize],
    priorities: &[Gap],
) -> Result<Look, String> {
    if priorities.is_empty() {
        return Err("no tuning-routed priority to review".into());
    }
    let here = state.trials.get(current).ok_or("no current trial")?;
    let mut not_shown = vec![];
    let mut alive = vec![];
    for index in variants {
        let trial = state.trials.get(*index).ok_or("no variant trial")?;
        let shared = progress::shared_views(here, trial, state.seed);
        if shared.is_empty() {
            not_shown.push(NotShown {
                trial: *index,
                inert: false,
                reason: "no still the current tree also holds".into(),
            });
        } else if shared.iter().all(|(_, differs)| !differs) {
            not_shown.push(NotShown {
                trial: *index,
                inert: true,
                reason: INERT.into(),
            });
        } else {
            alive.push((*index, shared));
        }
    }
    let Some(view) = view_for(&alive, priorities) else {
        return Ok(Look {
            shown: vec![],
            not_shown,
            plan: None,
        });
    };
    let mut shown: Vec<(usize, Image)> = vec![];
    for (index, _) in alive {
        let Some(image) = progress::still(&state.trials[index], &view, state.seed) else {
            not_shown.push(NotShown {
                trial: index,
                inert: false,
                reason: format!("no still at the {view} view"),
            });
            continue;
        };
        if let Some((twin, _)) = shown.iter().find(|(_, other)| other.sha256 == image.sha256) {
            not_shown.push(NotShown {
                trial: index,
                inert: false,
                reason: format!("draws the same tree as {}", state.trials[*twin].label),
            });
            continue;
        }
        if shown.len() >= 4 {
            not_shown.push(NotShown {
                trial: index,
                inert: false,
                reason: "the sheet shows at most five renders".into(),
            });
            continue;
        }
        shown.push((index, image));
    }
    if shown.is_empty() {
        return Ok(Look {
            shown: vec![],
            not_shown,
            plan: None,
        });
    }
    let here_still = progress::still(here, &view, state.seed)
        .ok_or("the current tree has no still at the reviewed view")?;
    let plan = plan(
        species,
        &view,
        state.seed,
        references,
        (&here.key, &here_still),
        &shown
            .iter()
            .map(|(index, image)| (state.trials[*index].key.clone(), image.clone()))
            .collect::<Vec<_>>(),
        &priorities
            .iter()
            .map(|gap| Priority {
                id: gap.id.clone(),
                observation: gap.observation.clone(),
            })
            .collect::<Vec<_>>(),
        &state.owner_notes,
    )?;
    Ok(Look {
        shown: shown.into_iter().map(|(index, _)| index).collect(),
        not_shown,
        plan: Some(plan),
    })
}

/// What a round would send, for pricing before any variant exists.
pub fn skeleton(state: &Run, references: &[Image]) -> Value {
    json!({"schema":VERSION,"prompt_sha256":Request::prompt_hash(),"seed":state.seed,
        "references":references,"owner_notes":state.owner_notes,
        "priorities":progress::tuning_priorities(state).iter().map(|gap| Priority{
            id:gap.id.clone(),observation:gap.observation.clone()}).collect::<Vec<_>>()})
}
