//! The CPU reference executor under realized mechanics against today's
//! whole-tree engine (fn-209 F1 task 6): the same tree, to the bit, by
//! lineage, at every cut, in every item order and chunking.
use super::request;
use crate::bounded::Expected;
use crate::fine::{digest, grow_fine, Order, Plan};
use crate::grow::{grow, Request};
use crate::species::Species;
use crate::{beech, spruce};

/// The tree at `plan` is today's tree, and the cut moved work.
fn same(species: &Species, request: Request, expected: &Expected, plan: Plan) -> usize {
    let whole = digest(&grow(species, request).expect("grows whole"));
    let fine = grow_fine(species, request, expected, plan).expect("grows fine");
    assert_eq!(digest(&fine.structure), whole, "{plan:?}");
    fine.roots.len()
}

#[test]
fn a_tree_is_todays_at_every_cut() {
    for species in [beech(), spruce()] {
        for seed in [1, 7] {
            let request = request(30, seed);
            let expected = Expected::new(&species, 30, 0);
            let roots: Vec<usize> = [0.0, 256.0, 1024.0, 4096.0, f64::INFINITY]
                .iter()
                .map(|&cut| {
                    let plan = Plan {
                        cut,
                        ..Plan::default()
                    };
                    same(&species, request, &expected, plan)
                })
                .collect();
            assert!(roots.windows(2).any(|w| w[0] != w[1]), "{roots:?}");
        }
    }
}

#[test]
fn a_tree_is_todays_in_every_order_and_chunking() {
    for species in [beech(), spruce()] {
        let request = request(30, 1);
        let expected = Expected::new(&species, 30, 0);
        for order in [Order::Forward, Order::Reverse, Order::Shuffled(3)] {
            for (chunk, threads) in [(1, 1), (7, 4), (64, 4)] {
                let plan = Plan {
                    cut: 256.0,
                    order,
                    chunk,
                    threads,
                };
                assert!(same(&species, request, &expected, plan) > 100);
            }
        }
    }
}
