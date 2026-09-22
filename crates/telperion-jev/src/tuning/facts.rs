//! The still-side numbers said in words, for whoever proposes the next move.
//!
//! Jev is shown the score's terms as plain statements and never as a score:
//! they are telemetry, and nothing here selects a tree. The beech run of
//! 2026-09-21 turned on a proportion nobody had stated - the owner's "the
//! whole crown needs to be stretched vertically", which the numbers had said
//! all along - because the router was only ever shown raw arrays.
//!
//! The five terms, their order and their meaning are `evaluation::METRICS`
//! and `measure()` in `scripts/compare-references.py`: width over height of
//! the tree's box; the crown base as a share of that height; the share of the
//! box the tree's own mask fills; the root-mean-square deviation of the
//! outline from its best-fit ellipse; and the mean of the centre two fifths
//! of the box in 0..255, which reads as brightness.
use super::{
    engine::Run,
    evaluation::{Comparison, METRICS},
};
use serde_json::{json, Value};

pub const MEANING: &str = "Measured facts, not a score: they never select a tree.";
/// Below this a difference is not worth a sentence.
const NOTABLE: f64 = 0.03;

/// What each term is called, and which way to say it when the render is above
/// or below the photograph.
const TERMS: [(&str, &str, &str); 5] = [
    (
        "crown width over height",
        "too wide for its height",
        "too narrow for its height",
    ),
    (
        "crown base height share",
        "the crown starts too high on the trunk",
        "the crown starts too low on the trunk",
    ),
    (
        "share of the frame the tree occupies",
        "too much of the frame is tree",
        "too little of the frame is tree",
    ),
    (
        "outline deviation",
        "the outline is too irregular",
        "the outline is too smooth",
    ),
    ("brightness", "too bright", "too dark"),
];

fn sentence(reference: &str, index: usize, observed: f64, target: f64) -> Option<String> {
    if !target.is_finite() || target == 0. || !observed.is_finite() {
        return None;
    }
    let relative = (observed - target) / target;
    if relative.abs() <= NOTABLE {
        return None;
    }
    let (name, above, below) = TERMS[index];
    let short = |v: f64| {
        format!("{v:.4}")
            .trim_end_matches('0')
            .trim_end_matches('.')
            .to_string()
    };
    let percent = (relative.abs() * 100.).round() as i64;
    let direction = if relative > 0. { above } else { below };
    Some(format!(
        "{reference}: {name} is {} against the photograph's {} ({percent}% {direction})",
        short(observed),
        short(target),
    ))
}

/// One plain sentence per term that differs by more than three per cent, for
/// every reference view the current trial measured.
pub fn sentences(comparisons: &[Comparison]) -> Vec<String> {
    let mut out = vec![];
    for comparison in comparisons {
        for index in 0..METRICS.len() {
            let Some(observed) = comparison.observed[index] else {
                continue;
            };
            if let Some(line) = sentence(
                &comparison.reference,
                index,
                observed,
                comparison.target[index],
            ) {
                out.push(line);
            }
        }
    }
    out
}

/// What the current tree measures, as the router and the proposer are shown it.
pub fn measured(state: &Run) -> Value {
    let facts = state
        .current
        .and_then(|i| state.trials.get(i))
        .map(|trial| sentences(&trial.comparisons))
        .unwrap_or_default();
    json!({"meaning":MEANING,"facts":facts,
        "terms":METRICS,
        "omitted":"a term within three per cent of the photograph is not listed"})
}
