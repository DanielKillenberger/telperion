//! A tree by lineage, whatever order its axes were grown or assembled in
//! (fn-209 F1, R1: "identical topology by lineage"). An axis's identity is
//! its path from the seed: its bearer's identity and how it stands there,
//! a lateral by its node, slot and waking, a continuation or relay by its
//! kind and PA. Two builds of one tree hold the same identities, and the
//! canonical order (by depth, then identity) makes them the same list.
use crate::lineage::Key;
use crate::structure::{Axis, Origin, Structure};

/// Each axis's identity, by index.
pub fn identities(structure: &Structure) -> Vec<u64> {
    identities_of(&structure.axes)
}

fn identities_of(axes: &[Axis]) -> Vec<u64> {
    let mut ids = vec![0u64; axes.len()];
    for (i, axis) in axes.iter().enumerate() {
        ids[i] = match axis.origin {
            Origin::Seed => Key::root(0).child(axis.pa as u64).0,
            Origin::Lateral {
                parent,
                node,
                slot,
                woken,
                ..
            } => {
                Key(ids[parent])
                    .child(1)
                    .child(node as u64)
                    .child(u64::from(slot))
                    .child(u64::from(woken))
                    .0
            }
            Origin::Continuation { parent } => Key(ids[parent]).child(2).child(axis.pa as u64).0,
            Origin::Relay { parent, .. } => Key(ids[parent]).child(3).child(axis.pa as u64).0,
        };
    }
    ids
}

/// The tree in canonical order, by depth and then identity, its parents
/// renumbered. Panics where two axes share an identity.
pub fn canonical(structure: &Structure) -> Structure {
    let ids = identities(structure);
    let mut depth = vec![0usize; structure.axes.len()];
    for (i, axis) in structure.axes.iter().enumerate() {
        depth[i] = axis.origin.parent().map_or(0, |p| depth[p] + 1);
    }
    let mut order: Vec<usize> = (0..structure.axes.len()).collect();
    order.sort_by_key(|&i| (depth[i], ids[i]));
    for pair in order.windows(2) {
        assert_ne!(ids[pair[0]], ids[pair[1]], "two axes share an identity");
    }
    let mut at = vec![usize::MAX; order.len()];
    for (k, &i) in order.iter().enumerate() {
        at[i] = k;
    }
    let axes = order
        .iter()
        .map(|&i| {
            let mut axis = structure.axes[i].clone();
            renumber(&mut axis, &at);
            axis
        })
        .collect();
    Structure {
        age: structure.age,
        pas: structure.pas,
        axes,
    }
}

fn renumber(axis: &mut Axis, at: &[usize]) {
    match &mut axis.origin {
        Origin::Seed => {}
        Origin::Lateral { parent, .. }
        | Origin::Continuation { parent }
        | Origin::Relay { parent, .. } => *parent = at[*parent],
    }
}

/// Every bit of the tree in canonical order, folded: two builds are the
/// same tree exactly where their digests are equal (to the hash).
pub fn digest(structure: &Structure) -> u64 {
    let tree = canonical(structure);
    let mut h = Key::root(u64::from(tree.age));
    let mut put = |v: u64| h = h.child(v);
    for axis in &tree.axes {
        let (kind, parent, node, slot, whorl, woken) = match axis.origin {
            Origin::Seed => (0, 0, 0, 0, 0.0, false),
            Origin::Lateral {
                parent,
                node,
                slot,
                whorl,
                woken,
            } => (1, parent, node, slot, whorl, woken),
            Origin::Continuation { parent } => (2, parent, 0, 0, 0.0, false),
            Origin::Relay { parent, node } => (3, parent, node, 0, 0.0, false),
        };
        for v in [
            kind,
            parent as u64,
            node as u64,
            u64::from(slot),
            whorl.to_bits(),
        ] {
            put(v);
        }
        put(u64::from(woken));
        put(axis.lineage);
        put(axis.pa as u64);
        put(u64::from(axis.birth));
        put(axis.apex_end.map_or(u64::MAX, u64::from));
        for v in [axis.vigour, axis.blend, axis.alive, axis.rank, axis.sleep] {
            put(v.to_bits());
        }
        for v in [axis.base, axis.heading, axis.side] {
            put_vec(&mut put, v);
        }
        for &u in &axis.units {
            put(u.to_bits());
        }
        for p in &axis.phytomers {
            put(u64::from(p.cycle));
            for v in [p.tip, p.heading, p.side] {
                put_vec(&mut put, v);
            }
            for v in [p.radius, p.scale, p.rank, p.size, p.light] {
                put(v.to_bits());
            }
            put(p.key);
        }
    }
    h.0
}

fn put_vec(put: &mut impl FnMut(u64), v: crate::structure::Vec3) {
    put(v.x.to_bits());
    put(v.y.to_bits());
    put(v.z.to_bits());
}

/// The tree in its canonical lineage order (host, 2026-10-07, decision 7's
/// order): parents first, breadth first from the seed, each axis's
/// children in the order their parents were placed, each parent's sorted
/// by lineage and, where two share one, by identity. Every `grow` returns
/// its tree so, whichever path built it.
pub fn lineage_order(structure: Structure) -> Structure {
    let Structure { age, pas, mut axes } = structure;
    let n = axes.len();
    let ids = identities_of(&axes);
    let keys: Vec<(u64, u64)> = axes
        .iter()
        .zip(&ids)
        .map(|(a, &id)| (a.lineage, id))
        .collect();
    let parents: Vec<Option<usize>> = axes.iter().map(|a| a.origin.parent()).collect();
    // Each parent's children, by index, in one table: `children[start[p]..start[p + 1]]`.
    let mut start = vec![0usize; n + 1];
    for &p in parents.iter().flatten() {
        start[p + 1] += 1;
    }
    for p in 0..n {
        start[p + 1] += start[p];
    }
    let mut fill = start.clone();
    let mut children = vec![0usize; start[n]];
    let mut order: Vec<usize> = Vec::with_capacity(n);
    for (i, parent) in parents.iter().enumerate() {
        match *parent {
            Some(p) => {
                children[fill[p]] = i;
                fill[p] += 1;
            }
            None => order.push(i),
        }
    }
    order.sort_by_key(|&i| keys[i]);
    let mut k = 0;
    while k < order.len() {
        let p = order[k];
        let kids = &mut children[start[p]..start[p + 1]];
        kids.sort_by_key(|&i| keys[i]);
        order.extend_from_slice(kids);
        k += 1;
    }
    // The axes moved to their places in place, a cycle of the
    // permutation at a time: place `k` takes axis `order[k]`.
    let mut placed = vec![false; n];
    for first in 0..n {
        let mut k = first;
        while !placed[k] {
            placed[k] = true;
            let from = order[k];
            if from == first {
                break;
            }
            axes.swap(k, from);
            k = from;
        }
    }
    let mut at = vec![usize::MAX; n];
    for (k, &i) in order.iter().enumerate() {
        at[i] = k;
    }
    for axis in &mut axes {
        renumber(axis, &at);
    }
    Structure { age, pas, axes }
}
