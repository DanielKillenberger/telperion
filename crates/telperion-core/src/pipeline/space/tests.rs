//! The axis order (host decision 17): stage 2 converts the engine's tree in
//! the engine's lineage order, and one tree assembled in two orders
//! converts to one tree.
use super::*;
use crate::presets::Preset;

/// The young beech's grown structure, its trunk's PAs and each PA's
/// internode and spread.
fn grown() -> (Structure, Vec<usize>, Vec<PaRow>) {
    let family = Preset::EuropeanBeech.parameters();
    let species = family.space.species.unwrap();
    let request = Request {
        age: 15,
        seed: 7,
        budget: BUDGET,
        light: family.space.light,
    };
    let structure = telperion_space::grow(&species, request).unwrap();
    (structure, family.space.trunk, pa_rows(&species))
}

/// Each node's grown wood is its internodes as they were laid (fn-212's
/// measure under fn-222's spread): at a spread of 0.7 every node drawn
/// after another on its axis records the laid lengths of the phytomers
/// merged into it, each its PA's internode at its scale and own factor.
#[test]
fn grown_wood_is_the_internodes_as_laid_at_a_spread() {
    let family = Preset::EuropeanBeech.parameters();
    let mut species = family.space.species.unwrap();
    // No wander, which turns a node about a pivot of its own and so moves
    // its tip off its internode's length.
    for s in &mut species.states {
        (s.form.internode_spread, s.form.wander) = (0.7, 0.0);
    }
    let request = Request {
        age: 15,
        seed: 7,
        budget: BUDGET,
        light: family.space.light,
    };
    let structure = telperion_space::grow(&species, request).unwrap();
    let rows = pa_rows(&species);
    let tree = convert(&structure, (&family.space.trunk, &rows));
    // Each drawn node by its key: its axis and the phytomer it is drawn
    // at. An axis carried on into an age keeps its chain's lineage, so a
    // key two phytomers share names neither.
    let mut at = std::collections::HashMap::new();
    for (a, axis) in structure.axes.iter().enumerate() {
        for k in 0..axis.phytomers.len() {
            let key = crate::tree::node_key(axis.lineage, k);
            at.entry(key)
                .and_modify(|e| *e = None)
                .or_insert(Some((a, k)));
        }
    }
    let laid = |axis: &telperion_space::Axis, k: usize| {
        let from = k
            .checked_sub(1)
            .map_or(axis.base, |j| axis.phytomers[j].tip);
        let d = axis.phytomers[k].tip;
        ((d.x - from.x).powi(2) + (d.y - from.y).powi(2) + (d.z - from.z).powi(2)).sqrt()
    };
    let (mut checked, mut varied) = (0, 0);
    for i in 1..tree.nodes.len() {
        let parent = tree.nodes[i].parent.unwrap() as usize;
        let here = at.get(&tree.keys[i]).copied().flatten();
        let (Some((a, k)), Some((b, j))) = (here, at.get(&tree.keys[parent]).copied().flatten())
        else {
            continue;
        };
        if a != b || j >= k {
            continue;
        }
        let axis = &structure.axes[a];
        let arc: f64 = (j + 1..=k).map(|m| laid(axis, m)).sum();
        let internode = species.states[axis.pa].internode;
        let plain: f64 = (j + 1..=k)
            .map(|m| internode * axis.phytomers[m].scale)
            .sum();
        let grown = tree.grown[i];
        // The lay is f32 (fn-254): its lengths a few of its places apart
        // for each phytomer (2.3e-7 m measured).
        assert!(
            (grown - arc).abs() <= 5e-7 * (k - j) as f64,
            "node {i}: grown {grown} against laid {arc}"
        );
        checked += 1;
        varied += usize::from((arc - plain).abs() > 1e-6 * arc);
    }
    assert!(
        checked > 1000 && varied > checked / 2,
        "{checked} nodes, {varied} varied"
    );
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
    let (structure, trunk, l) = grown();
    let other = reversed(&structure);
    assert_ne!(other.axes, structure.axes, "the orders differ");
    // Without the order the assembly reaches the tree.
    let unsorted =
        convert(&other, (&trunk, &l)) != convert(&structure, (&trunk, &l));
    assert!(unsorted, "the order never reached the conversion");
    let again = telperion_space::lineage_order(other);
    assert_eq!(again.axes, structure.axes, "grow returns the lineage order");
    assert!(
        convert(&again, (&trunk, &l)) == convert(&structure, (&trunk, &l))
    );
}

/// A tree over the Wasm ceiling is refused by name, and the name states
/// the ceiling the code holds; natively there is none.
#[test]
fn a_tree_over_the_wasm_ceiling_is_refused_by_name() {
    let (structure, _, _) = grown();
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
/// draw keyed by it (clusters, flowers) differs from one piece of wood
/// to the next; the identity is the key of the phytomer that made it, or a
/// relay's join's under its first phytomer (fn-244).
#[test]
fn every_engine_node_has_its_own_identity() {
    let (structure, trunk, rows) = grown();
    let tree = convert(&structure, (&trunk, &rows));
    let ids = identities(&tree);
    let distinct: std::collections::HashSet<u64> = ids.iter().copied().collect();
    assert_eq!(distinct.len(), ids.len(), "two nodes share an identity");
    let joins = structure.axes.iter().filter(|a| a.joined().is_some());
    let joins = joins.filter_map(|a| a.phytomers.first().map(|p| telperion_space::join_key(p.key())));
    let made: std::collections::HashSet<u64> = keys(&structure).into_values().chain(joins).collect();
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
    let (trunk, rows) = (&family.space.trunk, pa_rows(&species));
    let last: std::collections::HashSet<u64> =
        identities(&convert(&young, (trunk, &rows)))
            .into_iter()
            .collect();
    let kept = identities(&convert(&old, (trunk, &rows)))
        .iter()
        .filter(|id| last.contains(id))
        .count();
    assert!(kept * 2 > last.len(), "{kept} of {} kept", last.len());
}
