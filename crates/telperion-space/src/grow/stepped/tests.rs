//! The stepped executor against the whole tree (fn-211.4, probe (a)).
use super::*;
use crate::grow::partition::{grow_partitioned, Order};
use crate::light::Light;
use std::collections::HashMap;

const LIT: Light = Light {
    extinction: 0.5,
    sky: 0.5,
};

fn request(age: u32, seed: u64) -> Request {
    Request {
        age,
        seed,
        budget: 20_000_000,
        light: LIT,
    }
}

fn grouping(order: Order, chunk: usize) -> Grouping {
    Grouping {
        order,
        chunk,
        sizes: true,
        leaves: true,
    }
}

/// PHASES.md's one unchecked assumption: each grower's living list keeps
/// the whole tree's living order, restricted to it, at every cycle's end.
#[test]
fn each_grower_keeps_the_whole_trees_living_order() {
    let species = crate::oak();
    let r = request(24, 7);
    let expected = Expected::new(&species, r.age, 0);
    let (mut whole, _) = start(&species, r, true, Options::default()).unwrap();
    let (coarse, _) = start(&species, r, true, Options::default()).unwrap();
    let at = (super::counts(&expected), 256.0, grouping(Order::Forward, 1));
    let mut x = Executor::new(coarse, at, r, super::Keep::Digests);
    let mut advanced = Vec::new();
    for cycle in 1..=r.age {
        whole
            .year(cycle, cycle < r.age, &mut advanced, &mut |_| {})
            .unwrap();
        x.cycle(cycle).unwrap();
        let lineage = |g: &Grower, a: &super::super::Apex| g.axes[a.axis].lineage;
        let mut at = HashMap::new();
        for (k, a) in whole.live.iter().enumerate() {
            assert!(
                at.insert(lineage(&whole, a), k).is_none(),
                "one apex a lineage"
            );
        }
        let mut count = 0;
        let growers = std::iter::once(&x.coarse).chain(x.items.iter().map(|i| &i.g));
        for g in growers {
            let seen: Vec<usize> = g.live.iter().map(|a| at[&lineage(g, a)]).collect();
            assert!(
                seen.windows(2).all(|w| w[0] < w[1]),
                "cycle {cycle}: out of order"
            );
            count += seen.len();
        }
        assert_eq!(count, whole.live.len(), "cycle {cycle}");
    }
    assert!(x.items.len() > 10, "{} fine roots", x.items.len());
}

/// Probe (a) through the executor, small: the tree is the whole tree's by
/// lineage, whatever the order and chunking, and it runs every cycle's
/// phases (fn-254: grouped sums part the f32 states at their last places,
/// so the phases' digests no longer match to the bit).
#[test]
fn the_stepped_tree_is_the_whole_tree() {
    for (species, seed) in [(crate::oak(), 7), (crate::beech(), 1), (crate::spruce(), 1)] {
        let r = request(24, seed);
        let expected = Expected::new(&species, r.age, 0);
        let whole = grow_partitioned(&species, r, &expected, &[]).unwrap();
        for (order, chunk) in [(Order::Forward, 1), (Order::Shuffled(3), 7)] {
            let s = grow_stepped(&species, r, &expected, 256.0, grouping(order, chunk)).unwrap();
            assert!(s.roots > 0 && s.fine_grown > 0);
            eprintln!(
                "{order:?}: {} roots, {} of {} phytomers grown in them, cycle 20 {:x?}",
                s.roots,
                s.fine_grown,
                s.fine_grown + s.coarse_grown,
                s.phases[20]
            );
            assert_eq!(s.phases.len(), whole.phases.len(), "{order:?}");
            let agree = crate::agree::agreement(&whole.structure, &s.structure);
            assert!(agree.within(&crate::agree::PATHS), "{order:?}: {agree:?}");
        }
    }
}
