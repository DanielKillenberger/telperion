//! The CPU reference executor under realized mechanics against today's
//! whole-tree engine (fn-209 F1 task 6): the same tree, to the bit, by
//! lineage, at every cut, in every item order and chunking.
use super::request;
use crate::bounded::Expected;
use crate::fine::{digest, grow_fine, Fine, Order, Plan};
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

/// The tree at each of `cuts` is today's; the fine trees, by cut.
fn at_cuts(species: &Species, request: Request, cuts: &[f64]) -> Vec<Fine> {
    let whole = digest(&grow(species, request).expect("grows whole"));
    let expected = Expected::new(species, request.age, 0);
    cuts.iter()
        .map(|&cut| {
            let plan = Plan {
                cut,
                ..Plan::default()
            };
            let fine = grow_fine(species, request, &expected, plan).expect("grows fine");
            assert_eq!(digest(&fine.structure), whole, "at cut {cut}");
            fine
        })
        .collect()
}

const CUTS: [f64; 4] = [0.0, 64.0, 1024.0, f64::INFINITY];

/// The oak with its light switched off and no shed pipe retained, so its
/// fine roots can be grown here: relays at every stop, epicormic
/// sleeping buds, shedding at every order.
fn unlit_oak() -> Species {
    let mut species = crate::oak();
    for s in &mut species.states {
        (s.shade_hazard, s.shade_size) = (0.0, 0.0);
        (s.balance_hazard, s.leaf_girth, s.retained) = (0.0, 0.0, 0.0);
    }
    species
}

/// Fractional lifespans, woken roots whose sleep is a fraction among them.
#[test]
fn fractional_lifespans_are_todays() {
    let mut species = unlit_oak();
    for (k, s) in species.states.iter_mut().enumerate() {
        if s.lifespan > 0.0 {
            s.lifespan = (s.lifespan * 1.37 + 0.11 * k as f64).max(0.6);
        }
    }
    let trees = at_cuts(&species, request(32, 3), &CUTS);
    assert!(trees.iter().flat_map(|t| &t.roots).any(|r| r.woken));
}

/// Relays and sleeping buds inside roots; roots shed by what they
/// realized, and roots on bearers shed by what theirs realized.
#[test]
fn relays_dormancy_and_shedding_are_todays() {
    for species in [unlit_oak(), spruce()] {
        let trees = at_cuts(&species, request(24, 1), &CUTS);
        let fine = &trees[2];
        let relays = fine.structure.axes[fine.above..]
            .iter()
            .filter(|a| matches!(a.origin, crate::Origin::Relay { .. }))
            .count();
        let shed = fine.roots.iter().filter(|r| r.at.is_none()).count();
        assert!(shed > 0, "no root shed");
        if species.states.iter().any(|s| s.relay_ended > 0.0) {
            assert!(relays > 0, "no relay inside a root");
        }
    }
}

/// A tree past its budget, the roots' growth counted with the structure
/// above the cut, is refused at every cut as today's engine refuses it.
#[test]
fn a_tree_past_its_budget_is_refused_at_every_cut() {
    let (species, request) = (
        spruce(),
        Request {
            budget: 500,
            ..request(30, 1)
        },
    );
    let whole = grow(&species, request).map(|t| digest(&t));
    assert!(whole.is_err(), "the whole tree stands");
    let expected = Expected::new(&species, 30, 0);
    for cut in CUTS {
        let plan = Plan {
            cut,
            ..Plan::default()
        };
        let fine = grow_fine(&species, request, &expected, plan).map(|f| digest(&f.structure));
        assert_eq!(fine, whole, "at cut {cut}");
    }
}
