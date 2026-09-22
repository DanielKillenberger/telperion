//! Accepting a dial move on direction mass rather than a single argmax.
//!
//! The live pilot on 2026-09-21 put 0.94 of `irregularity`'s probability on an
//! increase and still refused the move, because the mass was split between the
//! small and substantial steps and no single option cleared the threshold. The
//! direction and the magnitude are separate questions; this rule reads them
//! separately. It is pure: no state, no model, no I/O.
use super::actions::Action;
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub const RULE: &str = "direction-mass-v1";

/// How much probability stood behind the accepted direction.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Accepted {
    pub action: Action,
    pub direction_mass: f64,
}

const UP: [Action; 2] = [Action::SmallIncrease, Action::SubstantialIncrease];
const DOWN: [Action; 2] = [Action::SmallDecrease, Action::SubstantialDecrease];

fn name(action: Action) -> String {
    serde_json::to_value(action)
        .ok()
        .and_then(|v| v.as_str().map(str::to_string))
        .unwrap_or_default()
}

fn mass(probabilities: &serde_json::Map<String, Value>, of: &[Action]) -> f64 {
    of.iter()
        .filter_map(|a| probabilities.get(&name(*a)))
        .filter_map(Value::as_f64)
        .filter(|p| p.is_finite())
        .sum()
}

/// The move a dial's answer supports, or `None`.
///
/// `available` is the set of actions the dial actually offers, so a dial whose
/// small step is excluded falls to the step it does have in that direction.
/// The probabilities must sum to one within a tolerance; anything else is an
/// answer we cannot read, and the dial is left alone.
pub fn accept(
    probabilities: &Value,
    available: &[Action],
    min_confidence: f64,
) -> Option<Accepted> {
    let probabilities = probabilities.as_object()?;
    let total: f64 = probabilities
        .values()
        .filter_map(Value::as_f64)
        .filter(|p| p.is_finite())
        .sum();
    if !(0.99..=1.01).contains(&total) {
        return None;
    }
    let (up, down) = (mass(probabilities, &UP), mass(probabilities, &DOWN));
    // A tie says nothing about which way to move.
    if up == down {
        return None;
    }
    let (direction_mass, ordered) = if up > down {
        (up, [Action::SmallIncrease, Action::SubstantialIncrease])
    } else {
        (down, [Action::SmallDecrease, Action::SubstantialDecrease])
    };
    if direction_mass < min_confidence {
        return None;
    }
    let offered = |a: Action| available.contains(&a);
    // A magnitude that clears the threshold on its own is the owner's own
    // acceptance rule; otherwise the small step, which is the cautious move.
    let outright = ordered.iter().copied().find(|a| {
        offered(*a)
            && probabilities
                .get(&name(*a))
                .and_then(Value::as_f64)
                .is_some_and(|p| p >= min_confidence)
    });
    let action = outright.or_else(|| ordered.iter().copied().find(|a| offered(*a)))?;
    Some(Accepted {
        action,
        direction_mass,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    const ALL: [Action; 4] = [
        Action::SmallIncrease,
        Action::SubstantialIncrease,
        Action::SmallDecrease,
        Action::SubstantialDecrease,
    ];

    fn accepted(p: Value, available: &[Action]) -> Option<Accepted> {
        accept(&p, available, 0.5)
    }

    /// The six answers the live pilot actually drew, 2026-09-21.
    #[test]
    fn the_live_pilot_answers_score_as_the_owner_expects() {
        // irregularity: 0.94 on an increase, split between the two magnitudes.
        let irregularity = accepted(
            json!({"small_increase":0.50,"substantial_increase":0.44,
                "hold":0.03,"insufficient_evidence":0.03}),
            &ALL,
        )
        .expect("a clear direction must be accepted");
        assert_eq!(irregularity.action, Action::SmallIncrease);
        assert!((irregularity.direction_mass - 0.94).abs() < 1e-9);

        // spacing: 0.60 on an increase.
        let spacing = accepted(
            json!({"small_increase":0.35,"substantial_increase":0.25,
                "hold":0.22,"insufficient_evidence":0.18}),
            &ALL,
        )
        .expect("0.60 clears the threshold");
        assert_eq!(spacing.action, Action::SmallIncrease);
        assert!((spacing.direction_mass - 0.60).abs() < 1e-9);

        // crookedness: 0.45 on an increase, under the threshold.
        assert_eq!(
            accepted(
                json!({"small_increase":0.25,"substantial_increase":0.20,
                    "hold":0.18,"insufficient_evidence":0.37}),
                &ALL
            ),
            None
        );
        // leaves, limbs and taper: the mass sat on insufficient_evidence.
        for p in [
            json!({"small_decrease":0.20,"substantial_decrease":0.16,"hold":0.10,"insufficient_evidence":0.54}),
            json!({"small_decrease":0.18,"substantial_decrease":0.18,"hold":0.10,"insufficient_evidence":0.54}),
            json!({"small_increase":0.21,"substantial_increase":0.10,"hold":0.10,"insufficient_evidence":0.59}),
        ] {
            assert_eq!(accepted(p.clone(), &ALL), None, "{p}");
        }
    }

    #[test]
    fn a_split_direction_a_tie_and_an_excluded_small_step() {
        // Split across directions: neither side reaches the threshold.
        assert_eq!(
            accepted(
                json!({"small_increase":0.30,"small_decrease":0.30,
                    "hold":0.20,"insufficient_evidence":0.20}),
                &ALL
            ),
            None
        );
        // An exact tie says nothing, even when both sides are large.
        assert_eq!(
            accepted(json!({"small_increase":0.50,"small_decrease":0.50}), &ALL),
            None
        );
        // The small step is not offered, so the direction takes what it has.
        let only_substantial = accepted(
            json!({"small_increase":0.40,"substantial_increase":0.40,
                "hold":0.10,"insufficient_evidence":0.10}),
            &[Action::SubstantialIncrease, Action::SubstantialDecrease],
        )
        .expect("the direction is clear and one step is offered");
        assert_eq!(only_substantial.action, Action::SubstantialIncrease);
        // One magnitude clearing the threshold alone is taken outright.
        let outright = accepted(
            json!({"substantial_increase":0.70,"small_increase":0.20,
                "hold":0.05,"insufficient_evidence":0.05}),
            &ALL,
        )
        .unwrap();
        assert_eq!(outright.action, Action::SubstantialIncrease);
        // Probabilities that do not sum to one are unreadable.
        assert_eq!(
            accepted(
                json!({"small_increase":0.9,"substantial_increase":0.9}),
                &ALL
            ),
            None
        );
        assert_eq!(accepted(json!({}), &ALL), None);
    }
}
