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
