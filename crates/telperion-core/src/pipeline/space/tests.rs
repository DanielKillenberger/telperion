//! The axis order (host decision 17): stage 2 converts the engine's tree in
//! the engine's lineage order, and one tree assembled in two orders
//! converts to one tree.
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
    let ids = telperion_space::identities(structure);
    let mut depth = vec![0usize; ids.len()];
    for (i, axis) in structure.axes.iter().enumerate() {
        depth[i] = axis.origin.parent().map_or(0, |p| depth[p] + 1);
    }
    let mut order: Vec<usize> = (0..ids.len()).collect();
    order.sort_by_key(|&i| (depth[i], std::cmp::Reverse(ids[i])));
    let mut at = vec![0usize; order.len()];
    for (k, &i) in order.iter().enumerate() {
        at[i] = k;
    }
    let mut s = structure.clone();
    s.axes = order
        .iter()
        .map(|&i| {
            let mut axis = structure.axes[i].clone();
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

/// Stage 2 converts the engine's tree as the engine returns it (host
/// decision 17): in its lineage order, which a tree assembled in any other
/// order takes again, so it converts to one tree.
#[test]
fn a_tree_assembled_in_another_order_converts_to_the_same_tree() {
    let (structure, trunk) = grown();
    let other = reversed(&structure);
    assert_ne!(other.axes, structure.axes, "the orders differ");
    // Without the order the assembly reaches the tree.
    let unsorted = convert(&other, &trunk) != convert(&structure, &trunk);
    assert!(unsorted, "the order never reached the conversion");
    let again = telperion_space::lineage_order(other);
    assert_eq!(again.axes, structure.axes, "grow returns the lineage order");
    assert!(convert(&again, &trunk) == convert(&structure, &trunk));
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
    assert!(
        refused.contains("more than 4,000,000 phytomers"),
        "{refused}"
    );
    assert!(refused.contains("4 GB Wasm memory"), "{refused}");
    assert_eq!(KEPT, None, "no ceiling natively");
}

/// A tree expected to keep more than the ceiling is refused before it
/// grows, in the predictor's milliseconds (host decision 15); one under
/// it, and every tree natively, passes on to growth.
#[test]
fn a_tree_expected_over_the_ceiling_is_refused_before_it_grows() {
    let family = Preset::EuropeanBeech.parameters();
    let species = family.space.species.unwrap();
    let expected = telperion_space::expected_kept(&species, 15)
        .unwrap()
        .unwrap();
    let under = expected.ceil() as usize;
    assert!(foreseen(&species, 15, Some(under)).is_ok());
    let refused = foreseen(&species, 15, Some(under - 1)).unwrap_err();
    assert!(
        refused.to_string().contains("4 GB Wasm memory"),
        "{refused}"
    );
    assert!(foreseen(&species, 15, None).is_ok());
}

/// A species whose buds read light has no expected count: the lit oak
/// keeps far fewer than the light-neutral predictor counts, so it is held
/// after growth alone.
#[test]
fn a_tree_that_reads_light_is_held_after_growth() {
    let family = Preset::OregonWhiteOak.parameters();
    let species = family.space.species.unwrap();
    assert_eq!(telperion_space::expected_kept(&species, 15).unwrap(), None);
    assert!(foreseen(&species, 15, Some(0)).is_ok());
}

/// Each converted node's identity, the root's left out.
fn identities(tree: &Tree) -> Vec<u64> {
    tree.nodes[1..]
        .iter()
        .map(|n| n.identity.birth_order())
        .collect()
}

/// Each phytomer's key by its lineage: its axis's identity and its place
/// along the axis.
fn keys(structure: &Structure) -> std::collections::HashMap<(u64, usize), u64> {
    let ids = telperion_space::identities(structure);
    let mut keys = std::collections::HashMap::new();
    for (axis, &id) in structure.axes.iter().zip(&ids) {
        for (k, p) in axis.phytomers.iter().enumerate() {
            keys.insert((id, k), p.key());
        }
    }
    keys
}

/// Every node of an engine tree has its own identity (fn-220 R1, R2), so a
/// draw keyed by it (short shoots, flowers) differs from one piece of wood
/// to the next; the identity is the key of the phytomer that made it.
#[test]
fn every_engine_node_has_its_own_identity() {
    let (structure, trunk) = grown();
    let tree = convert(&structure, &trunk);
    let ids = identities(&tree);
    let distinct: std::collections::HashSet<u64> = ids.iter().copied().collect();
    assert_eq!(distinct.len(), ids.len(), "two nodes share an identity");
    let made: std::collections::HashSet<u64> = keys(&structure).into_values().collect();
    assert!(
        ids.iter().all(|id| made.contains(id)),
        "not a phytomer's key"
    );
}

/// A node keeps its identity as the tree ages (fn-220 R2): the same
/// lineage holds the same key a year on, and most of last year's nodes
/// are this year's by identity.
#[test]
fn a_node_keeps_its_identity_across_ages() {
    let family = Preset::EuropeanBeech.parameters();
    let species = family.space.species.unwrap();
    let at = |age| {
        let request = Request {
            age,
            seed: 7,
            budget: BUDGET,
            light: family.space.light,
        };
        telperion_space::grow(&species, request).unwrap()
    };
    let (young, old) = (at(14), at(15));
    let (before, after) = (keys(&young), keys(&old));
    let shared = before.keys().filter(|k| after.contains_key(k)).count();
    assert!(shared * 2 > before.len(), "{shared} of {}", before.len());
    for (lineage, key) in &before {
        if let Some(later) = after.get(lineage) {
            assert_eq!(key, later, "a lineage changed its key");
        }
    }
    let trunk = &family.space.trunk;
    let last: std::collections::HashSet<u64> =
        identities(&convert(&young, trunk)).into_iter().collect();
    let kept = identities(&convert(&old, trunk))
        .iter()
        .filter(|id| last.contains(id))
        .count();
    assert!(kept * 2 > last.len(), "{kept} of {} kept", last.len());
}

/// A node's identity does not depend on where the fine cut falls (fn-220
/// R2): the tree grown whole and expanded below any cut converts to the
/// same identities.
#[test]
fn a_node_keeps_its_identity_across_cuts() {
    let family = Preset::EuropeanBeech.parameters();
    let species = family.space.species.unwrap();
    let request = Request {
        age: 15,
        seed: 7,
        budget: BUDGET,
        light: family.space.light,
    };
    let trunk = &family.space.trunk;
    let whole = telperion_space::grow_whole(&species, request).unwrap();
    let reference = identities(&convert(&whole, trunk));
    for cut in [0.0, 16.0, 1024.0] {
        let plan = telperion_space::Plan {
            cut,
            ..Default::default()
        };
        let fine = telperion_space::grow_fine(&species, request, plan).unwrap();
        let tree = convert(&telperion_space::lineage_order(fine.structure), trunk);
        assert_eq!(identities(&tree), reference, "cut {cut}");
    }
}
