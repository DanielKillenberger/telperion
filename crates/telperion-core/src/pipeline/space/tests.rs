//! The axis order (host decision 17): stage 2 converts the engine's tree in
//! the engine's lineage order, and one tree assembled in two orders
//! converts to one tree.
use super::*;
use crate::presets::Preset;

/// The young beech's grown structure, its trunk's PAs and each PA's
/// internode and spread.
fn grown() -> (Structure, Vec<usize>, Vec<(f64, f64)>) {
    let family = Preset::EuropeanBeech.parameters();
    let species = family.space.species.unwrap();
    let request = Request {
        age: 15,
        seed: 7,
        budget: BUDGET,
        light: family.space.light,
    };
    let structure = telperion_space::grow(&species, request).unwrap();
    let internodes = internodes(&species);
    (structure, family.space.trunk, internodes)
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
    let rows = internodes(&species);
    let tree = convert(&structure, (&[], &[]), (&family.space.trunk, &rows));
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
        // The lay's lengths are Q32: a last place apart for each phytomer.
        assert!(
            (grown - arc).abs() <= 1e-9 * (k - j) as f64,
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
        convert(&other, (&[], &[]), (&trunk, &l)) != convert(&structure, (&[], &[]), (&trunk, &l));
    assert!(unsorted, "the order never reached the conversion");
    let again = telperion_space::lineage_order(other);
    assert_eq!(again.axes, structure.axes, "grow returns the lineage order");
    assert!(
        convert(&again, (&[], &[]), (&trunk, &l)) == convert(&structure, (&[], &[]), (&trunk, &l))
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

/// Host option 3 (fn-212 task 7) on the CPU path: the bearers whose kind
/// or fork moved from their seams' nodes to their attachments, at 30, 40
/// and 80 (`cargo test --profile ci -p telperion-core seams_read_by_their_attachments
/// -- --ignored --nocapture`). Reported, not held.
#[test]
#[ignore]
fn seams_read_by_their_attachments() {
    for preset in [Preset::EuropeanBeech, Preset::NorwaySpruce] {
        for (age, seed) in [(30, 1), (40, 1), (40, 7), (80, 1), (80, 7)] {
            let mut family = preset.parameters();
            (family.age, family.skeleton.seed) = (f64::from(age), seed);
            let tree = grow(GrowInput::of(&family)).unwrap();
            let n = tree.nodes.len();
            let mut seam = vec![false; n];
            for &s in &tree.seams {
                seam[s as usize] = true;
            }
            let (mut all, mut kept, mut was_fork) = (vec![0u32; n], vec![0u32; n], vec![false; n]);
            for (i, node) in tree.nodes.iter().enumerate().skip(1) {
                let p = node.parent.unwrap() as usize;
                all[p] += 1;
                kept[p] += u32::from(!seam[i]);
                was_fork[p] |= seam[i] && node.codominant.is_some();
            }
            let mut is_fork = vec![false; n];
            let mut bearers = std::collections::HashSet::new();
            for a in &tree.attachments {
                kept[a.node as usize] += 1;
                is_fork[a.node as usize] |= a.fork;
                bearers.insert(a.node);
            }
            let thin = STRUCTURAL * tree.nodes[0].radius;
            let kinds = (1..n)
                .filter(|&i| tree.nodes[i].radius < thin && (all[i] == 0) != (kept[i] == 0))
                .count();
            let forks = (0..n).filter(|&i| was_fork[i] != is_fork[i]).count();
            let bare = tree
                .attachments
                .iter()
                .filter(|a| a.seams.is_empty())
                .count();
            // Forks moved at a root that draws a seam: its base radius and
            // its seam node's radius read across a threshold.
            let drawn = (tree.attachments.iter())
                .filter(|a| !a.seams.is_empty())
                .filter(|a| a.fork != tree.nodes[a.seams[0] as usize].codominant.is_some())
                .count();
            println!(
                "{preset:?} ({age}, {seed}): bearers {}, roots {} (drawing nothing at the seam {bare}), \
                 kinds moved {kinds}, forks moved {forks} (forks now {}, before {}; roots \
                 whose seam's fork moved though it draws one {drawn})",
                bearers.len(),
                tree.attachments.len(),
                is_fork.iter().filter(|&&f| f).count(),
                was_fork.iter().filter(|&&f| f).count(),
            );
        }
    }
}

/// fn-212 task 8: each tree's nodes and the phytomers merged into them,
/// at 40 and 80 (`cargo test --profile ci -p telperion-core merges_per_tree
/// -- --ignored --nocapture`). Merged: the phytomers the conversion takes
/// (to the first under `FINEST` on each axis) less the nodes it draws.
/// Reported, not held.
#[test]
#[ignore]
fn merges_per_tree() {
    for preset in [Preset::EuropeanBeech, Preset::NorwaySpruce] {
        for (age, seed) in [(40, 1), (40, 7), (80, 1), (80, 7)] {
            let mut family = preset.parameters();
            (family.age, family.skeleton.seed) = (f64::from(age), seed);
            let species = family.space.species.clone().unwrap();
            let request = Request {
                age,
                seed: u64::from(seed),
                budget: BUDGET,
                light: family.space.light,
            };
            let (structure, ..) = telperion_space::grow_marked(&species, request).unwrap();
            let taken: usize = (structure.axes.iter())
                .map(|a| {
                    a.phytomers
                        .iter()
                        .take_while(|p| p.radius >= FINEST)
                        .count()
                })
                .sum();
            let nodes = grow(GrowInput::of(&family)).unwrap().nodes.len() - 1;
            println!(
                "{preset:?} ({age}, {seed}): nodes {nodes}, taken {taken}, merged {}",
                taken - nodes
            );
        }
    }
}
