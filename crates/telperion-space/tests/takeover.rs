//! fn-286: a relay runs back onto the line of the axis it takes over, by
//! its PA's `takeover`, and none of it leaves the tree as it was.
mod walk;
use telperion_space::{Origin, Species, Structure, Vec3};

/// The walk's limb, relaying at every stop, turned off the line by 0.6
/// radians, taken over by `takeover`.
fn grown(takeover: f64, seed: u64) -> Structure {
    let mut species: Species = walk::species();
    let limb = &mut species.states[1];
    (limb.relay, limb.relay_ended, limb.relay_failed) = (1.0, 1.0, 1.0);
    (limb.insertion, limb.takeover) = (0.6, takeover);
    walk::tree(&species, seed).unwrap()
}

/// Each relay's turn in radians off its parent's heading at its node, to
/// its first phytomer as drawn.
fn turns(tree: &Structure) -> Vec<f64> {
    let angle = |a: Vec3, b: Vec3| a.dot(b).clamp(-1.0, 1.0).acos();
    tree.axes
        .iter()
        .filter_map(|a| match a.origin {
            Origin::Relay { parent, node } => {
                let p = &tree.axes[parent];
                let along = p.phytomers.get(node).map_or(p.heading, |q| q.heading);
                a.phytomers.first().map(|q| angle(along, q.heading))
            }
            _ => None,
        })
        .collect()
}

fn mean(v: &[f64]) -> f64 {
    v.iter().sum::<f64>() / v.len() as f64
}

#[test]
fn a_relay_runs_back_onto_the_line_it_takes_over_by_degree() {
    for seed in 1..=3 {
        let [none, half, whole] = [0.0, 0.5, 1.0].map(|t| turns(&grown(t, seed)));
        assert!(!none.is_empty(), "seed {seed} relays");
        let [none, half, whole] = [&none, &half, &whole].map(|v| mean(v));
        assert!(
            none > half && half > whole,
            "seed {seed}: the turn falls with the take-over: {none} {half} {whole}"
        );
    }
}

/// A take-over moves only relays' wood: with no relay, the tree is the
/// same to the bit whatever it says.
#[test]
fn a_take_over_without_relays_is_dormant() {
    let tree = |takeover: f64| {
        let mut species: Species = walk::species();
        let limb = &mut species.states[1];
        (limb.relay, limb.relay_ended, limb.relay_failed) = (0.0, 0.0, 0.0);
        limb.takeover = takeover;
        walk::tree(&species, 1).unwrap()
    };
    assert_eq!(tree(0.0), tree(1.0));
}

/// Each continuation of a relay: its turn in radians off the relay's
/// heading at its last node, to its own first phytomer as drawn, and the
/// larger of the turns either side of it, the relay's last and the
/// continuation's first, as the walk turns step to step.
fn handovers(tree: &Structure) -> Vec<(f64, f64)> {
    let angle = |a: Vec3, b: Vec3| a.dot(b).clamp(-1.0, 1.0).acos();
    let step = |p: &[telperion_space::Phytomer]| -> f64 {
        p.windows(2).map(|w| angle(w[0].heading, w[1].heading)).fold(0.0, f64::max)
    };
    tree.axes
        .iter()
        .filter_map(|a| match a.origin {
            Origin::Continuation { parent } => {
                let p = &tree.axes[parent];
                let relayed = matches!(p.origin, Origin::Relay { .. });
                let (last, first) = (p.phytomers.last()?, a.phytomers.first()?);
                let n = p.phytomers.len();
                let around = step(&p.phytomers[n.saturating_sub(2)..])
                    .max(step(&a.phytomers[..a.phytomers.len().min(2)]));
                relayed.then(|| (angle(last.heading, first.heading), around))
            }
            _ => None,
        })
        .collect()
}

/// fn-286 review: a relay's continuation carries on what its take-over
/// has left, so with nothing else bending the limb, the walk turns no more
/// where the relay's age hands on than step to step either side of it (a
/// 0.46 rad hand-over at seed 1 before the fix).
#[test]
fn a_continuation_carries_its_relays_take_over_on() {
    for seed in 1..=3 {
        let mut species: Species = walk::species();
        for state in &mut species.states {
            (state.straightening, state.erection) = (0.0, 0.0);
            (state.form.tropism, state.form.wander, state.form.sag) = (0.0, 0.0, 0.0);
        }
        let limb = &mut species.states[1];
        (limb.relay, limb.relay_ended, limb.relay_failed) = (1.0, 1.0, 1.0);
        (limb.insertion, limb.takeover) = (0.6, 1.0);
        let turns = handovers(&walk::tree(&species, seed).unwrap());
        assert!(!turns.is_empty(), "seed {seed} continues a relay");
        for (at, around) in turns {
            assert!(
                at <= 1.5 * around + 0.02,
                "seed {seed}: a hand-over turns {at} rad, the steps around it {around}"
            );
        }
    }
}
