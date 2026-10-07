//! The canonical axis order (host, 2026-10-07): one tree assembled in two
//! orders converts to one tree, and the order is the engine's own.
use super::*;
use crate::presets::Preset;

/// The young beech's grown structure.
fn grown() -> (Structure, Vec<usize>) {
    let family = Preset::EuropeanBeech.parameters();
    let species = family.space.species.unwrap();
    let request = Request {
        age: 15,
        seed: 7,
        budget: BUDGET,
        light: family.space.light,
    };
    let structure = telperion_space::grow(&species, request).unwrap();
    (structure, family.space.trunk)
}

/// `structure` reassembled with its siblings in the reverse of their
/// canonical order, parents still first.
fn reversed(structure: &Structure) -> Structure {
    let mut s = canonical(structure.clone()).unwrap();
    let ids = telperion_space::identities(&s);
    let mut depth = vec![0usize; ids.len()];
    for (i, axis) in s.axes.iter().enumerate() {
        depth[i] = axis.origin.parent().map_or(0, |p| depth[p] + 1);
    }
    let mut order: Vec<usize> = (0..ids.len()).collect();
    order.sort_by_key(|&i| (depth[i], std::cmp::Reverse(ids[i])));
    let mut at = vec![0usize; order.len()];
    for (k, &i) in order.iter().enumerate() {
        at[i] = k;
    }
    let axes = std::mem::take(&mut s.axes);
    s.axes = order
        .iter()
        .map(|&i| {
            let mut axis = axes[i].clone();
            match &mut axis.origin {
                Origin::Seed => {}
                Origin::Lateral { parent, .. }
                | Origin::Continuation { parent }
                | Origin::Relay { parent, .. } => *parent = at[*parent],
            }
            axis
        })
        .collect();
    s
}

#[test]
fn a_tree_assembled_in_another_order_converts_to_the_same_tree() {
    let (structure, trunk) = grown();
    let other = reversed(&structure);
    assert_ne!(other.axes, structure.axes, "the orders differ");
    // Without the sort the order reaches the tree.
    let unsorted = convert(&other, &trunk) != convert(&structure, &trunk);
    assert!(unsorted, "the order never reached the conversion");
    let a = convert(&canonical(structure).unwrap(), &trunk);
    let b = convert(&canonical(other).unwrap(), &trunk);
    assert!(a == b, "one tree, two orders, two conversions");
}

#[test]
fn the_order_is_the_engines_canonical_order() {
    let (structure, _) = grown();
    let theirs = telperion_space::canonical(&structure);
    assert_eq!(canonical(structure).unwrap().axes, theirs.axes);
}

/// A tree over the Wasm ceiling is refused by name, and the name states
/// the ceiling the code holds; natively there is none.
#[test]
fn a_tree_over_the_wasm_ceiling_is_refused_by_name() {
    let (structure, _) = grown();
    let kept: usize = structure.axes.iter().map(|a| a.phytomers.len()).sum();
    assert!(fits(&structure, Some(kept)).is_ok());
    let refused = fits(&structure, Some(kept - 1)).unwrap_err().to_string();
    // The message names the bound by hand: the two move together.
    assert_eq!(WASM_KEPT, 4_000_000);
    assert!(refused.contains("more than 4,000,000 phytomers"), "{refused}");
    assert!(refused.contains("4 GB Wasm memory"), "{refused}");
    assert_eq!(KEPT, None, "no ceiling natively");
}
