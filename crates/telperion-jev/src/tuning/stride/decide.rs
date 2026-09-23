//! One track's class for one round, and what the round's outcome does to it.
use super::{shown, Class, Judged, LABEL};
use crate::tuning::{
    bundle::Track,
    engine::{Run, Services},
    progress,
};

/// The class a track's bundle is drawn at this round, and the multiplier on
/// the configured ladder it maps to.
#[derive(Debug, Clone, PartialEq)]
pub struct Decision {
    pub class: Option<Class>,
    pub multiplier: f64,
}

/// What the reviewer said most recently on a round that graded this priority:
/// what the whole set still lacks, then what looks wrong in that render.
fn finding(state: &Run, priority: &str) -> Option<String> {
    state.trials.iter().rev().find_map(|t| {
        let sheet = t.sheet.as_ref()?;
        if !sheet.per_priority.contains_key(priority) {
            return None;
        }
        let words = std::iter::once(sheet.missing.clone())
            .chain(sheet.wrong.iter().cloned())
            .filter(|w| !w.trim().is_empty())
            .collect::<Vec<_>>();
        (!words.is_empty()).then(|| words.join(" "))
    })
}

/// A class the track may not be drawn above, set when the direction on a dial
/// the track's last raised bundle moved has turned round.
fn flipped(state: &mut Run, track: &Track, wanted: &[(String, i8)]) -> Option<String> {
    let adopted = state.strides.get(&track.name)?.adopted?;
    if adopted == Class::Near {
        return None;
    }
    let bundle = state
        .current
        .and_then(|i| state.trials[i].bundle.as_ref())?;
    let dial = bundle.moves.iter().find_map(|m| {
        let sign = if m.direction == "up" { 1 } else { -1 };
        wanted
            .iter()
            .any(|(id, s)| id == &m.dial && *s == -sign)
            .then(|| m.dial.clone())
    })?;
    let cap = adopted.lower();
    state.strides.entry(track.name.clone()).or_default().cap = Some(cap);
    Some(format!(
        "capped at {} after {dial} turned round from the bundle at {}",
        cap.name(),
        adopted.name()
    ))
}

/// The class Jev chose, or why none is used.
fn read(state: &Run, judged: &Judged) -> Result<Class, String> {
    let at = format!(
        "{} at {:?} against {}",
        judged.choice, judged.confidence, judged.threshold
    );
    if !judged
        .confidence
        .is_some_and(|c| c.is_finite() && c >= judged.threshold && c <= 1.)
    {
        return Err(format!("default: jev {at}, below the threshold"));
    }
    let class = Class::parse(&judged.choice).ok_or(format!("default: jev {at}"))?;
    if class > Class::Near && !judged.calibrated && state.pilot_authority().is_err() {
        return Err(format!(
            "default: jev {at} refused, the gap_magnitude set has not qualified and no experimental authority is scoped"
        ));
    }
    Ok(class)
}

/// The class this round's words select, and where it came from. Owner first
/// and free; then one question on the lead tuning priority's latest finding.
fn choose(
    state: &mut Run,
    services: &mut dyn Services,
    save: &mut dyn FnMut(&Run) -> Result<(), String>,
) -> Result<(Option<Class>, String), String> {
    let Some(priority) = progress::tuning_priorities(state).into_iter().next() else {
        return Ok((None, "default: no tuning priority".into()));
    };
    if let Some(class) = services.owner_magnitude(&priority.id) {
        return Ok((Some(class), format!("owner on {}", priority.id)));
    }
    let Some(finding) = finding(state, &priority.id) else {
        return Ok((None, format!("default: no finding yet on {}", priority.id)));
    };
    if !services.offers_gap_magnitude() {
        return Ok((None, "default: no gap-magnitude question".into()));
    }
    let asked = shown(&priority.observation, &finding);
    let allowance = services.gap_magnitude_tokens(&asked);
    state.push_judgment_input(LABEL, asked.clone());
    state.reserve(0, 0, allowance, 0, LABEL, save)?;
    let answer = services.gap_magnitude(&asked)?;
    let judged = state.settle(answer, allowance)?;
    Ok(match read(state, &judged) {
        Ok(class) => {
            let trust = if judged.calibrated {
                ""
            } else {
                ", uncalibrated under experimental authority"
            };
            let source = format!(
                "jev at {:?} against {}{trust} on {}",
                judged.confidence, judged.threshold, priority.id
            );
            (Some(class), source)
        }
        Err(reason) => (None, reason),
    })
}

/// One track's class for this round, recorded in the route notes.
pub(in crate::tuning) fn decide(
    state: &mut Run,
    services: &mut dyn Services,
    save: &mut dyn FnMut(&Run) -> Result<(), String>,
    track: &Track,
    wanted: &[(String, i8)],
) -> Result<Decision, String> {
    let turned = flipped(state, track, wanted);
    let (chosen, source) = choose(state, services, save)?;
    let cap = state.strides.get(&track.name).and_then(|s| s.cap);
    let class = match (chosen, cap) {
        (Some(c), Some(cap)) if c > cap => Some(cap),
        (chosen, _) => chosen,
    };
    let multiplier = class.map_or(1., Class::multiplier);
    let mut note = format!(
        "stride class {} from {source}, multiplier {multiplier}",
        class.map_or("none", Class::name)
    );
    if class != chosen {
        note += &format!(
            "; {} capped at {}",
            chosen.map_or("none", Class::name),
            cap.map_or("none", Class::name)
        );
    }
    if let Some(turned) = turned {
        note += &format!("; {turned}");
    }
    state
        .routes
        .push(crate::tuning::bundle::note_for(track, note));
    Ok(Decision { class, multiplier })
}

/// What the track's turn did to its stride. An adoption that stood lifts the
/// cap; one rolled back at a raised class caps the next rounds a level lower.
pub(in crate::tuning) fn settle(state: &mut Run, track: &Track, decision: &Decision, kept: bool) {
    let standing = state.strides.entry(track.name.clone()).or_default();
    if kept {
        standing.cap = None;
        standing.adopted = decision.class;
        return;
    }
    let Some(class) = decision.class.filter(|c| *c > Class::Near) else {
        return;
    };
    standing.cap = Some(class.lower());
    let note = format!(
        "stride capped at {} after the bundle at {} was rolled back",
        class.lower().name(),
        class.name()
    );
    state
        .routes
        .push(crate::tuning::bundle::note_for(track, note));
}
