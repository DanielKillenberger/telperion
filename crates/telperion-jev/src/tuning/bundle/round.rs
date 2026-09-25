//! The round a bundle makes.
//!
//! Every dial Jev supported moves together, at each configured strength; one
//! contact sheet judges the lot; and when the best variant is better but
//! breaks something, the bundle is halved until the breaking dials are
//! isolated. Single steps on single dials could not reach a look that needs
//! several rows at once, which is why the round moves them together.
use super::{
    build, directions, isolate::isolate, merge, track, track::ensure_views, track::Track, worse,
    Bundle,
};
use crate::tuning::{
    engine::{Proposal, Run, Services},
    progress,
    sheet::{self, Outcome},
    stride,
};
use serde_json::Value;

/// One variant the round drew: the trial it became and the wire it patches.
/// Its strength is its bundle's own.
pub(super) struct Variant {
    pub trial: usize,
    pub overlay: Value,
}

/// A bundle already tried from the tree the loop is standing on. Its identity
/// is its dials and directions, its strength and that tree.
fn tried(state: &Run, bundle: &str) -> bool {
    state.trials.iter().any(|t| {
        state.measured_here(&t.identity) && t.bundle.as_ref().is_some_and(|b| b.id == bundle)
    })
}

/// One ordinary evaluation of one bundle: reserved, drawn, recorded.
#[allow(clippy::too_many_arguments)]
pub(super) fn evaluate(
    state: &mut Run,
    services: &mut dyn Services,
    save: &mut dyn FnMut(&Run) -> Result<(), String>,
    bundle: &Bundle,
    overlay: &Value,
    old: usize,
    parent: Option<String>,
    label: &str,
    ledger: Option<String>,
) -> Result<usize, String> {
    let mut overrides = state.overrides.clone();
    merge(&mut overrides, overlay);
    state.reserve(
        1,
        services.evaluation_images(),
        0,
        0,
        "candidate evaluation",
        save,
    )?;
    let mut trial = services.evaluate(overrides, state.budget.rounds, label, ledger);
    trial.base = Some(state.trials[old].key.clone());
    trial.evidence = state.visual.as_ref().map(|v| v.ledger.clone());
    trial.bundle = Some(bundle.clone());
    trial.parent_bundle = parent;
    state.pending = None;
    let index = state.trials.len();
    state.trials.push(trial);
    if !state.trials[index].feasible {
        let reason = state.trials[index].reason.clone().unwrap_or_default();
        state.routes.push(format!("{label} infeasible: {reason}"));
    }
    save(state)?;
    Ok(index)
}

/// Records every variant code kept off the sheet, and why.
pub(super) fn note_unshown(state: &mut Run, look: &sheet::Look) {
    for not in &look.not_shown {
        let label = state.trials[not.trial].label.clone();
        state.trials[not.trial].sheet = Some(Outcome::unshown(not.inert, &not.reason));
        state
            .routes
            .push(format!("{label} not shown: {}", not.reason));
    }
}

/// A paid sheet that failed or would not bind. Every variant it would have
/// judged is recorded as charged, so the round ends without an adoption and
/// nobody buys the same look again.
pub(super) fn note_failure(state: &mut Run, look: &sheet::Look) {
    for index in &look.shown {
        state.trials[*index].reason = Some(progress::REVIEW_FAILED.into());
    }
    state.routes.push(sheet::FAILED_NOTE.into());
}

/// Writes each shown variant's row of the verdict onto its own trial, and
/// returns what the adoption table is asked about.
pub(super) fn read_back(
    state: &mut Run,
    look: &sheet::Look,
    verdict: &sheet::Verdict,
    current: usize,
) -> Vec<(String, f64)> {
    // The tree the loop is standing on was on the sheet too, and what the
    // reviewer says is wrong with it is the plainest thing it said.
    let here = state.trials[current].key.clone();
    if let Some(render) = verdict.render(&here) {
        state.trials[current].sheet = Some(Outcome::of(verdict, render));
    }
    let mut shown = vec![];
    for index in &look.shown {
        let key = state.trials[*index].key.clone();
        let strength = state.trials[*index]
            .bundle
            .as_ref()
            .map_or(0., |b| b.strength);
        if let Some(render) = verdict.render(&key) {
            state.trials[*index].sheet = Some(Outcome::of(verdict, render));
        }
        shown.push((key, strength));
    }
    shown
}

/// Makes one variant the tree the loop stands on, and takes the closing look.
pub(super) fn keep(
    state: &mut Run,
    services: &mut dyn Services,
    save: &mut dyn FnMut(&Run) -> Result<(), String>,
    key: &str,
    others: &[String],
    variants: &[Variant],
) -> Result<bool, String> {
    let index = state
        .trials
        .iter()
        .position(|t| t.key == key)
        .ok_or("lost the adopted variant")?;
    let overlay = variants
        .iter()
        .find(|v| v.trial == index)
        .map(|v| v.overlay.clone())
        .ok_or("lost the adopted wire")?;
    // Where to put the run back, taken before anything moves.
    let restore = crate::tuning::veto::restore_point(state);
    state.trials[index].adopted_over = others.to_vec();
    state.trials[index].adopted = true;
    state.current = Some(index);
    let mut effective = state.effective.clone();
    merge(&mut effective, &overlay);
    state.effective = effective;
    state.overrides = state.trials[index].overrides.clone();
    state.assess(services, save)?;
    crate::tuning::veto::settle(state, services, save, restore, index)
}

/// The other variants the reviewer also judged adoptable. Owner-facing only.
fn passed_over(verdict: &sheet::Verdict, shown: &[(String, f64)], kept: &str) -> Vec<String> {
    shown
        .iter()
        .filter(|(key, _)| {
            key != kept
                && verdict
                    .render(key)
                    .is_some_and(crate::tuning::sheet::Render::adoptable)
        })
        .map(|(key, _)| key.clone())
        .collect()
}

/// What one track did with its turn.
#[derive(PartialEq)]
enum Turn {
    Kept,
    Stalled,
    /// Every strength of this track's bundle was already tried here.
    AllTried,
    /// No strength of it could be drawn at all.
    NothingDrawn,
}

/// What a track's stall is called. The implicit track has no name to give.
pub(in crate::tuning) fn note(track: &Track, text: String) -> String {
    if track.name.is_empty() {
        text
    } else {
        format!("track {}: {text}", track.name)
    }
}

/// One track's turn: its own bundle, its own sheet, its own adoption.
fn one_track(
    state: &mut Run,
    services: &mut dyn Services,
    save: &mut dyn FnMut(&Run) -> Result<(), String>,
    track: &Track,
    wanted: &[(String, i8)],
    ledger: Option<String>,
) -> Result<Turn, String> {
    // Taken again per track: an earlier track may have moved the tree.
    let old = state.current.ok_or("no current trial")?;
    let base = state.trials[old].key.clone();
    let strengths = services.bundle_strengths();
    if strengths.is_empty() {
        return Err("no bundle strength configured".into());
    }
    // A family an isolated part already lost on this tree is left out, so the
    // next bundle is a different bundle rather than the same one again.
    let Some(wanted) = worse::eligible(state, &base, track, wanted) else {
        save(state)?;
        return Ok(Turn::AllTried);
    };
    let wanted = &wanted[..];
    // How far this round moves is the size of the gap the words name.
    let stride = stride::decide(state, services, save, track, wanted)?;
    let mut planned = vec![];
    let mut refused = 0;
    for strength in strengths.iter().map(|s| s * stride.multiplier) {
        match build(
            &state.preset,
            &state.effective,
            &state.dials,
            wanted,
            strength,
            &base,
            &track.name,
        ) {
            Err(reason) => state.routes.push(note(
                track,
                format!("bundle at strength {strength} not drawn: {reason}"),
            )),
            Ok((bundle, _)) if tried(state, &bundle.id) => {
                refused += 1;
                state.routes.push(note(
                    track,
                    format!("bundle repeat refused: strength {strength} was already tried from this tree"),
                ));
            }
            Ok(drawn) => planned.push(drawn),
        }
    }
    if planned.is_empty() {
        save(state)?;
        return Ok(if refused > 0 {
            Turn::AllTried
        } else {
            Turn::NothingDrawn
        });
    }
    // The tree the variants are compared with needs the track's view too.
    ensure_views(state, services, save, old, &track.extra_views)?;
    let mut variants = vec![];
    for (bundle, overlay) in planned {
        let label = format!("bundle@{}", bundle.strength);
        let trial = evaluate(
            state,
            services,
            save,
            &bundle,
            &overlay,
            old,
            None,
            &label,
            ledger.clone(),
        )?;
        if state.trials[trial].feasible {
            ensure_views(state, services, save, trial, &track.extra_views)?;
            variants.push(Variant { trial, overlay });
        }
    }
    let priorities = progress::track_priorities(state, track);
    let drawn = variants.iter().map(|v| v.trial).collect::<Vec<_>>();
    let look = services.sheet_request(state, old, &drawn, &priorities, track.view.as_deref())?;
    note_unshown(state, &look);
    let Some(plan) = &look.plan else {
        state
            .routes
            .push(note(track, progress::stall(services.selection())));
        save(state)?;
        return Ok(Turn::Stalled);
    };
    let verdict = match sheet::review(state, services, save, plan) {
        Ok(verdict) => verdict,
        Err(reason) => {
            note_failure(state, &look);
            save(state)?;
            return Err(reason);
        }
    };
    let shown = read_back(state, &look, &verdict, old);
    save(state)?;
    if let Some(key) = sheet::adopt(&verdict, &shown) {
        let others = passed_over(&verdict, &shown, &key);
        let kept = keep(state, services, save, &key, &others, &variants)?;
        return settled(state, save, track, &stride, kept);
    }
    // Better but breaking is halved until the breaking dials are isolated;
    // worse everywhere is cut once by family, to learn which part was good.
    let clean = match sheet::to_split(&verdict, &shown) {
        Some(start) => isolate(
            state,
            services,
            save,
            old,
            &base,
            track,
            ledger,
            &mut variants,
            start,
        )?,
        None => worse::split(
            state,
            services,
            save,
            old,
            &base,
            track,
            ledger,
            &mut variants,
            &shown,
        )?,
    };
    let Some(key) = clean else {
        state
            .routes
            .push(note(track, progress::stall(services.selection())));
        save(state)?;
        return Ok(Turn::Stalled);
    };
    let kept = keep(state, services, save, &key, &[], &variants)?;
    settled(state, save, track, &stride, kept)
}

/// The turn an adoption ends, with what it did to the track's stride.
fn settled(
    state: &mut Run,
    save: &mut dyn FnMut(&Run) -> Result<(), String>,
    track: &Track,
    decision: &stride::Decision,
    kept: bool,
) -> Result<Turn, String> {
    stride::settle(state, track, decision, kept);
    save(state)?;
    Ok(if kept { Turn::Kept } else { Turn::Stalled })
}

/// One bundle round: one bundle per track, in order. `true` when some track
/// kept a variant and took the closing look.
pub(in crate::tuning) fn round(
    state: &mut Run,
    proposals: Vec<Proposal>,
    services: &mut dyn Services,
    save: &mut dyn FnMut(&Run) -> Result<(), String>,
) -> Result<bool, String> {
    let wanted = directions(&proposals);
    if wanted.is_empty() {
        return Err("no supported proposal; bounded diagnosis required".into());
    }
    let ledger = proposals.first().map(|p| p.ledger.clone());
    let tracks = track::all(&services.tracks());
    let shares = track::assign(&tracks, &state.dials, &wanted);
    let mut outcomes = vec![];
    for (track, moves) in tracks.iter().zip(shares) {
        // No objective, no turn: a track nobody aims at draws nothing.
        if progress::track_priorities(state, track).is_empty() {
            state.routes.push(note(
                track,
                "skipped: no objective routed to this track".into(),
            ));
            continue;
        }
        if moves.is_empty() {
            state
                .routes
                .push(note(track, "no supported dial this round".into()));
            continue;
        }
        outcomes.push(one_track(
            state,
            services,
            save,
            track,
            &moves,
            ledger.clone(),
        )?);
    }
    if outcomes.is_empty() {
        return Err("no supported proposal; bounded diagnosis required".into());
    }
    if outcomes.iter().all(|o| *o == Turn::AllTried) {
        return Err("bundle already tried; no new direction".into());
    }
    if outcomes.iter().all(|o| *o == Turn::NothingDrawn) {
        return Err("no bundle this round could draw; bounded diagnosis required".into());
    }
    Ok(outcomes.contains(&Turn::Kept))
}
