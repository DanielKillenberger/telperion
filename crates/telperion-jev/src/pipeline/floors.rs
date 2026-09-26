//! The floor a level must reach (fn-131), set from labelled live answers:
//! an appearance or described level below `level_floor` is the no-match
//! level. Each case is
//! one answer's probability and whether a person admits the pick; the floor
//! is the one that answers the most cases right, the lowest such, so no
//! labelled correct pick is dropped for nothing gained.

use serde::Deserialize;

pub const LEVEL_FLOOR_CASES: &str = include_str!("../../data/cases/level_floor.json");

#[derive(Debug, Clone, Deserialize)]
pub struct FloorCase {
    pub id: String,
    pub asked: String,
    pub picked: String,
    pub probability: f64,
    pub correct: bool,
}

#[derive(Deserialize)]
struct FloorSet {
    cases: Vec<FloorCase>,
}

fn parse(raw: &str) -> Vec<FloorCase> {
    serde_json::from_str::<FloorSet>(raw)
        .expect("floor cases parse")
        .cases
}

pub fn level_floor_cases() -> Vec<FloorCase> {
    parse(LEVEL_FLOOR_CASES)
}

/// How many cases a floor answers right: a correct pick at or above it, a
/// wrong one below it.
pub fn hits(cases: &[FloorCase], floor: f64) -> usize {
    cases
        .iter()
        .filter(|c| (c.probability >= floor) == c.correct)
        .count()
}

/// The lowest of the case probabilities that answers the most cases right.
pub fn calibrate(cases: &[FloorCase]) -> f64 {
    let mut candidates: Vec<f64> = cases.iter().map(|c| c.probability).collect();
    candidates.sort_by(f64::total_cmp);
    let best = candidates
        .iter()
        .map(|&f| hits(cases, f))
        .max()
        .unwrap_or(0);
    candidates
        .into_iter()
        .find(|&f| hits(cases, f) == best)
        .unwrap_or(0.0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::questions::thresholds;

    /// R9: the floor is the one its labelled cases set, and the set holds
    /// a wrong pick the floor must weigh, the no-match side.
    #[test]
    fn the_level_floor_is_calibrated_on_its_labelled_cases() {
        let cases = level_floor_cases();
        assert!(cases.iter().any(|c| !c.correct), "a set with no wrong pick");
        assert!(cases.iter().any(|c| c.correct));
        assert_eq!(calibrate(&cases), thresholds().level_floor);
    }

    #[test]
    fn a_tie_takes_the_lower_floor() {
        let case = |p: f64, correct: bool| FloorCase {
            id: String::new(),
            asked: String::new(),
            picked: String::new(),
            probability: p,
            correct,
        };
        let cases = [
            case(0.3, true),
            case(0.35, false),
            case(0.4, true),
            case(0.9, true),
        ];
        // 0.3 and 0.4 each answer three of four right.
        assert_eq!(calibrate(&cases), 0.3);
        assert_eq!(calibrate(&[case(0.2, false), case(0.6, true)]), 0.6);
    }
}
