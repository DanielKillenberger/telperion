//! The fine roots against the tree P2's chain grower grows whole: the
//! same tree, to the bit, at every cut, in every item order and chunking
//! (fn-209 F1, task 1's acceptance), and on the adversarial cases
//! (`adversarial.rs`); the decision contract's draws (`draws.rs`).
use super::{digest, grow_fine, Fine, Order, Plan};
use crate::alone::grow_alone;
use crate::bounded::Expected;
use crate::grow::Request;
use crate::light::Light;
use crate::species::Species;
use crate::{beech, spruce};

mod adversarial;
mod draws;
mod ground;
mod ticks;

pub(super) fn request(age: u32, seed: u64) -> Request {
    Request {
        age,
        seed,
        budget: 20_000_000,
        light: Light::NEUTRAL,
    }
}

/// The tree P2 grows whole, every chain resolved.
pub(super) fn whole(species: &Species, request: Request, expected: &Expected) -> u64 {
    let mut tree = grow_alone(species, request, expected).expect("grows alone");
    tree.bounded.resolve_all(1).expect("resolves");
    digest(tree.bounded.structure())
}

pub(super) fn at(species: &Species, request: Request, expected: &Expected, cut: f64) -> Fine {
    let plan = Plan {
        cut,
        ..Plan::default()
    };
    grow_fine(species, request, expected, plan).expect("grows")
}

/// The tree at each of `cuts` is the tree grown whole, the cuts moving
/// work across them; the fine trees, by cut.
pub(super) fn same_at_cuts(species: &Species, request: Request, cuts: &[f64]) -> Vec<Fine> {
    let expected = Expected::new(species, request.age, 0);
    let reference = whole(species, request, &expected);
    let trees: Vec<Fine> = cuts
        .iter()
        .map(|&cut| at(species, request, &expected, cut))
        .collect();
    for (cut, fine) in cuts.iter().zip(&trees) {
        assert_eq!(digest(&fine.structure), reference, "at cut {cut}");
    }
    let above: Vec<usize> = trees.iter().map(|t| t.above).collect();
    assert!(
        above.windows(2).any(|w| w[0] != w[1]),
        "the cuts move no work: {above:?}"
    );
    trees
}

/// N in {256, 1024, 4096} (SPLIT.md), and every bud fine or none, on the
/// beech and the spruce at two seeds.
#[test]
fn a_tree_is_the_same_at_every_cut() {
    let cuts = [0.0, 256.0, 1024.0, 4096.0, f64::INFINITY];
    for species in [beech(), spruce()] {
        for seed in [1, 7] {
            same_at_cuts(&species, request(30, seed), &cuts);
        }
    }
}

/// Items taken forward, backward and shuffled, one, seven or 64 to a
/// piece, on one core or four.
#[test]
fn a_tree_is_the_same_in_every_order_and_chunking() {
    for species in [beech(), spruce()] {
        let request = request(30, 1);
        let expected = Expected::new(&species, 30, 0);
        let reference = whole(&species, request, &expected);
        for order in [Order::Forward, Order::Reverse, Order::Shuffled(3)] {
            for (chunk, threads) in [(1, 1), (7, 4), (64, 4)] {
                let plan = Plan {
                    cut: 256.0,
                    order,
                    chunk,
                    threads,
                };
                let fine = grow_fine(&species, request, &expected, plan).expect("grows");
                assert!(fine.roots.len() > 100, "{} roots", fine.roots.len());
                assert_eq!(
                    digest(&fine.structure),
                    reference,
                    "{order:?}, {chunk} a piece on {threads}"
                );
            }
        }
    }
}

/// A species any bud of which reads light is refused by name: its fine
/// roots need F2b's light aggregates.
#[test]
fn a_species_that_reads_light_is_refused() {
    let species = crate::oak();
    let expected = Expected::new(&species, 5, 1);
    let refused = grow_fine(&species, request(5, 1), &expected, Plan::default());
    assert!(
        matches!(refused, Err(crate::Error::Refused { .. })),
        "{:?}",
        refused.err()
    );
}
