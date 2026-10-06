//! fn-211.4, probe (a)'s arithmetic (DESIGN-INTERLEAVE.md after Astra's
//! review): today's tree grown with every reduction that crosses the cut
//! taken as a partitioned run must take it, from the reference's own
//! values, and compared with the whole tree after every phase of every
//! cycle.
//!
//! A partitioned run keeps each axis's own work and every bearer-to-child
//! reduction (they read one axis and its children, in the tree's order),
//! so those are the whole tree's arithmetic wherever the work runs. Three
//! reductions run over the whole tree in its index order and cannot be
//! taken from per-root totals without regrouping their sums:
//!
//! - **sizes**: each PA's Σp and Σp(v/p)^ψ (decision 23), phase (iii);
//! - **leaves**: the lattice's deposits, phase (iv);
//! - **girth**: the leaf-light girth's tree-wide mean in a re-lay.
//!
//! Each is regrouped here as a partition would group it: the structure
//! above the cut in index order, then each fine root's own partial in its
//! index order, the roots taken in the plan's order, `chunk` partials
//! summed before they join the total.
use crate::structure::Vec3;
use std::hash::{DefaultHasher, Hash, Hasher};

/// The order fine roots' partials are combined in.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Order {
    Forward,
    Reverse,
    Shuffled(u64),
}

/// How a partitioned run groups the whole-tree reductions, and which.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Grouping {
    pub order: Order,
    pub chunk: usize,
    pub sizes: bool,
    pub leaves: bool,
    pub girth: bool,
}

/// The phases of a cycle (DESIGN-INTERLEAVE.md, Astra change 1): balance,
/// allocation, growth and waking, then layout, re-lay and light.
pub const PHASES: [&str; 4] = ["balance", "allocation", "growth", "light"];

/// The roots in `order`.
pub(crate) fn ordered(mut roots: Vec<usize>, order: Order) -> Vec<usize> {
    roots.sort_unstable();
    roots.dedup();
    match order {
        Order::Forward => {}
        Order::Reverse => roots.reverse(),
        Order::Shuffled(seed) => {
            let key = crate::lineage::Key::root(seed);
            roots.sort_by_key(|&r| key.child(r as u64).0);
        }
    }
    roots
}

/// Sums `n`-wide values, each owned by the structure above the cut (none)
/// or a fine root, as the grouping takes them: the structure's in order,
/// then each root's partial, `chunk` partials at a time.
pub(crate) fn grouped(items: &[(Option<usize>, Vec<f64>)], n: usize, g: &Grouping) -> Vec<f64> {
    let mut total = vec![0.0; n];
    let mut partial: std::collections::HashMap<usize, Vec<f64>> = Default::default();
    for (root, v) in items {
        let into = match root {
            None => &mut total,
            Some(r) => partial.entry(*r).or_insert_with(|| vec![0.0; n]),
        };
        for (t, x) in into.iter_mut().zip(v) {
            *t += x;
        }
    }
    let roots = ordered(partial.keys().copied().collect(), g.order);
    for chunk in roots.chunks(g.chunk.max(1)) {
        let mut sum = vec![0.0; n];
        for r in chunk {
            for (s, x) in sum.iter_mut().zip(&partial[r]) {
                *s += x;
            }
        }
        for (t, s) in total.iter_mut().zip(sum) {
            *t += s;
        }
    }
    total
}

/// Leaves regrouped: the structure's in their order, then each root's.
pub(crate) fn regroup(
    leaves: &[(Vec3, f64)],
    owner: &[Option<usize>],
    g: &Grouping,
) -> Vec<(Vec3, f64)> {
    let mut out: Vec<(Vec3, f64)> = Vec::with_capacity(leaves.len());
    let mut by_root: std::collections::HashMap<usize, Vec<(Vec3, f64)>> = Default::default();
    for (&leaf, owner) in leaves.iter().zip(owner) {
        match owner {
            None => out.push(leaf),
            Some(r) => by_root.entry(*r).or_default().push(leaf),
        }
    }
    for r in ordered(by_root.keys().copied().collect(), g.order) {
        out.extend(&by_root[&r]);
    }
    out
}

/// A phase's state hashed to the bit.
pub(crate) fn hash(values: impl Iterator<Item = f64>) -> u64 {
    let mut h = DefaultHasher::new();
    for v in values {
        v.to_bits().hash(&mut h);
    }
    h.finish()
}
