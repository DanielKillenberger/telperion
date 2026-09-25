//! R3: the palm's fn-80 rounds replayed on the stride, with no render, no
//! paid look and no Jev call.
//!
//! What the replay can say is how far the frond-length dial moves per round on
//! the recorded ladder and on the ladder the owner's class scales, stepped by
//! the bundle code itself on the date palm's own wire. What it cannot say is
//! whether a wider bundle would have stood: the recorded run kept none of its
//! five adoptions, and a rolled-back round moves the dial nowhere on either
//! ladder. So every round is replayed at its top rung, the reach the stride
//! allows, and the rollbacks stay the closing review's question.
use serde::Deserialize;
use serde_json::{json, Value};
use std::collections::BTreeMap;
use telperion_jev::tuning::{actions::Dial, bundle, stride::Class};

#[derive(Deserialize)]
struct Priority {
    id: String,
}

#[derive(Deserialize)]
struct Recorded {
    bundle_rounds: usize,
    furthest_frond_length_drawn: f64,
}

#[derive(Deserialize)]
struct Replay {
    priority: Priority,
    magnitudes: BTreeMap<String, Class>,
    dial: Dial,
    bundle_strengths: Vec<f64>,
    max_rounds: usize,
    recorded: Recorded,
    target: f64,
    rounds: Vec<Value>,
}

/// The rachis length the recorded run started from.
const RECORDED_START: f64 = 3.5;

fn replay() -> Replay {
    serde_json::from_str(include_str!("fixtures/fn80-palm-frond-rounds.json")).unwrap()
}

/// The round on which the frond length first reaches the target, drawing
/// each round's top rung of the ladder scaled by `multiplier`.
fn rounds_to_target(fx: &Replay, multiplier: f64) -> Option<usize> {
    let preset = telperion_core::presets::Preset::from_id("date-palm").unwrap();
    let mut wire = telperion_core::params::metadata(&preset.parameters());
    // The run started from the untuned palm's 3.5 m rachis; tuning has since
    // shipped the target itself.
    *wire.pointer_mut(&fx.dial.path).unwrap() = json!(RECORDED_START);
    let top = fx.bundle_strengths.last().unwrap() * multiplier;
    let wanted = [(fx.dial.id.clone(), 1)];
    for round in 1..=fx.max_rounds {
        let dials = std::slice::from_ref(&fx.dial);
        let (drawn, _) = bundle::build(
            "date-palm",
            &wire,
            dials,
            &wanted,
            top,
            "replay",
            "structure",
        )
        .ok()?;
        let to = drawn.moves[0].to;
        *wire.pointer_mut(&fx.dial.path).unwrap() = json!(to);
        if to >= fx.target - 1e-9 {
            return Some(round);
        }
    }
    None
}

#[test]
fn the_owner_s_far_off_brings_the_palm_s_fronds_to_length_in_fewer_than_seven_rounds() {
    let fx = replay();
    assert_eq!(fx.rounds.len(), fx.recorded.bundle_rounds);
    let class = fx.magnitudes[&fx.priority.id];
    assert_eq!(
        class,
        Class::FarOff,
        "the owner's words: far too thin and short"
    );

    // The recorded ladder moves the frond 0.2 m a round at its top rung, so
    // even had every round stood it needed eighteen, not the six it had.
    let recorded = rounds_to_target(&fx, 1.).unwrap();
    assert_eq!(recorded, 18);
    assert!(recorded > fx.recorded.bundle_rounds);
    assert!(fx.recorded.furthest_frond_length_drawn < fx.target);

    // The owner's class draws four times the ladder: 0.8 m a round.
    let scaled = rounds_to_target(&fx, class.multiplier()).unwrap();
    assert_eq!(scaled, 5);
    assert!(scaled < 7);
}
