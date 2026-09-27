//! A bundle the generator will not build is taken apart (fn-179).
//!
//! The beech's first Tune after fn-170 raised `codominance` in every one of
//! sixteen bundles while its fork rows left every part on one heading; the
//! generator refused each build, and the loop drew the same refused bundle
//! for five rounds. Now a draw refused at every strength is halved with
//! `split` and its halves are drawn the next round, without a new proposal;
//! a dial refused alone is dropped for the rest of the revision. What to draw
//! is read back from the trials, so a resumed run takes the same path.
use super::{id, split, track::Track, Move};
use crate::tuning::{
    engine::Run,
    evaluation::{unbuilt, Trial},
};

/// Dials and the direction each moves, as `build` draws them.
pub(super) type Wanted = Vec<(String, i8)>;

fn wanted(moves: &[Move]) -> Wanted {
    let sign = |m: &Move| if m.direction == "up" { 1 } else { -1 };
    moves.iter().map(|m| (m.dial.clone(), sign(m))).collect()
}

/// What makes two draws the same draw, whatever their strength.
fn signature(wanted: &[(String, i8)]) -> Wanted {
    let mut rows = wanted.to_vec();
    rows.sort();
    rows
}

/// One draw of this revision: its moves as first drawn from one tree, and the
/// reason every strength of it failed to build there, if every strength did.
struct Draw {
    moves: Vec<Move>,
    refused: Option<String>,
}

/// Every bundle draw of this revision whose trial `here` admits, by index,
/// one per tree and signature: a draw that built from one tree says nothing
/// of the same draw from another.
fn draws(state: &Run, here: impl Fn(usize, &Trial) -> bool) -> Vec<Draw> {
    let mut out: Vec<((Option<String>, Wanted), Draw)> = vec![];
    for (index, trial) in state.trials.iter().enumerate() {
        if !state.measured_here(&trial.identity) || !here(index, trial) {
            continue;
        }
        let Some(bundle) = trial.bundle.as_ref() else {
            continue;
        };
        let reason = trial
            .reason
            .clone()
            .filter(|r| !trial.feasible && unbuilt(r));
        let key = (trial.base.clone(), signature(&wanted(&bundle.moves)));
        match out.iter_mut().find(|(k, _)| *k == key) {
            Some((_, draw)) => draw.refused = draw.refused.take().and(reason),
            None => out.push((
                key,
                Draw {
                    moves: bundle.moves.clone(),
                    refused: reason,
                },
            )),
        }
    }
    out.into_iter().map(|(_, draw)| draw).collect()
}

/// The dials this revision drew alone, from any tree, and the generator
/// refused at every strength there.
pub(super) fn dropped(state: &Run) -> Vec<String> {
    let mut out: Vec<String> = vec![];
    for draw in draws(state, |_, _| true) {
        if let ([only], Some(_)) = (draw.moves.as_slice(), &draw.refused) {
            if !out.contains(&only.dial) {
                out.push(only.dial.clone());
            }
        }
    }
    out
}

/// `wanted` without the dials dropped for the revision.
fn kept(state: &Run, wanted: &[(String, i8)]) -> Wanted {
    let dropped = dropped(state);
    wanted
        .iter()
        .filter(|(dial, _)| !dropped.contains(dial))
        .cloned()
        .collect()
}

/// The halves this track still owes: of every draw it made this revision
/// that the generator refused at every strength, each half the track has not
/// drawn yet, from whichever tree - an adoption between the refusal and its
/// halves does not forget them. A half holding a dial dropped for the
/// revision is not drawn: the dial would refuse it again, and the rest come
/// back with the next proposal.
pub(super) fn halves(state: &Run, track: &Track) -> Vec<Wanted> {
    let ours = |_, t: &Trial| {
        let (Some(base), Some(b)) = (t.base.as_deref(), t.bundle.as_ref()) else {
            return false;
        };
        b.id == id(&b.moves, b.strength, base, &track.name)
    };
    let (drawn, dropped) = (draws(state, ours), dropped(state));
    let seen: Vec<Wanted> = drawn.iter().map(|d| signature(&wanted(&d.moves))).collect();
    let mut out: Vec<Wanted> = vec![];
    for draw in drawn
        .iter()
        .filter(|d| d.refused.is_some() && d.moves.len() > 1)
    {
        let (a, b) = split(&draw.moves, &state.dials);
        for half in [a, b].map(|half| wanted(&half)) {
            let key = signature(&half);
            let fresh = !seen.contains(&key) && !out.iter().any(|h| signature(h) == key);
            if fresh && !half.iter().any(|(dial, _)| dropped.contains(dial)) {
                out.push(half);
            }
        }
    }
    out
}

/// What a track draws this turn: the halves it still owes, else
/// the dials Jev supported less the families an isolated part already lost
/// here, so the next bundle is a different bundle rather than the same one
/// again, and less the dials dropped for the revision. `None` when nothing is
/// left to draw.
pub(super) fn turn(
    state: &mut Run,
    base: &str,
    track: &Track,
    wanted: &[(String, i8)],
) -> Option<Vec<Wanted>> {
    let halves = halves(state, track);
    if !halves.is_empty() {
        return Some(halves);
    }
    let eligible = super::worse::eligible(state, base, track, wanted)?;
    let wanted = kept(state, &eligible);
    if wanted.is_empty() {
        let line = super::round::note(
            track,
            "every supported dial is dropped for the revision".into(),
        );
        state.routes.push(line);
        return None;
    }
    Some(vec![wanted])
}

/// Whether some track still owes halves.
pub(in crate::tuning) fn pending(state: &Run, tracks: &[Track]) -> bool {
    super::track::all(tracks)
        .iter()
        .any(|track| !halves(state, track).is_empty())
}

/// Records every draw among `trials` that failed to build at every strength:
/// its halves come next, or its one dial is dropped for the revision.
pub(super) fn note(state: &mut Run, track: &Track, trials: &[usize]) {
    let lines: Vec<String> = draws(state, |index, _| trials.contains(&index))
        .into_iter()
        .filter_map(|draw| {
            let reason = draw.refused?;
            Some(match draw.moves.as_slice() {
                [only] => format!(
                    "{} dropped for the revision: alone it failed to build: {reason}",
                    only.dial
                ),
                moves => format!(
                    "bundle of {} dials failed to build at every strength: {reason}; \
                     its halves are drawn next round",
                    moves.len()
                ),
            })
        })
        .collect();
    for line in lines {
        let line = super::round::note(track, line);
        state.routes.push(line);
    }
}
