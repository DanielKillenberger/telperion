use super::{tests::bytes, *};
use crate::presets::Preset;

#[test]
fn shedding_keeps_storage_and_keys_but_packs_only_living_wood() {
    let mut family = Preset::Ordinary.parameters();
    family.age = 8.0;
    family.skeleton.habit.shedding_threshold = 0.0;
    let mut specimen = Specimen::build(&family).unwrap();
    let original = specimen.tree.clone();
    let root = original
        .nodes
        .iter()
        .position(|n| n.kind == NodeKind::Branch)
        .unwrap();
    let cut = original.nodes[root].identity;
    let mut expected = specimen.tree().clone();
    let mut map = vec![None; expected.nodes.len()];
    let mut count = 0;
    for (i, node) in expected.nodes.iter().enumerate() {
        if node.identity != cut && node.parent.is_none_or(|p| map[p as usize].is_some()) {
            map[i] = Some(count);
            count += 1;
        }
    }
    let mut index = 0;
    expected.nodes.retain_mut(|node| {
        let keep = map[index].is_some();
        index += 1;
        if keep {
            node.parent = node.parent.map(|p| map[p as usize].unwrap());
            node.branch = map[node.branch as usize].unwrap();
        }
        keep
    });
    specimen.retire(&[cut]);
    assert!(
        bytes(specimen.tree()) == bytes(&expected),
        "death filter changed packed bytes"
    );
    assert_eq!(
        specimen.tree.nodes.len(),
        original.nodes.len(),
        "shedding deleted chronicle nodes"
    );
    for (i, node) in original.nodes.iter().enumerate() {
        assert_eq!(
            specimen.identities[node.identity.key], i,
            "death moved an identity slot"
        );
        assert_eq!(specimen.tree.nodes[i].identity, node.identity);
        let links = &specimen.links[node.identity.key];
        assert_eq!(
            links.parent,
            node.parent.map(|p| original.nodes[p as usize].identity)
        );
        assert_eq!(links.run, original.nodes[node.branch as usize].identity);
        assert_eq!(specimen.tree.nodes[i].parent, node.parent);
        assert_eq!(specimen.tree.nodes[i].branch, node.branch);
    }
    assert!(specimen.node(cut).is_err());
    specimen.tree().validate_solved().unwrap();
    let packed = specimen.tree();
    assert!(packed.nodes[..packed.crossover]
        .iter()
        .all(|n| n.kind == NodeKind::Structural));
    assert!(packed.nodes[packed.crossover..]
        .iter()
        .all(|n| n.kind != NodeKind::Structural));
}

#[test]
fn dead_records_stay_frozen_and_slots_are_never_reused_by_later_growth() {
    let mut family = Preset::Ordinary.parameters();
    family.age = 12.0;
    let mut specimen = Specimen::build(&family).unwrap();
    let dead: Vec<_> = specimen
        .tree
        .nodes
        .iter()
        .enumerate()
        .filter(|(_, n)| n.shoot.death_year.is_some())
        .map(|(i, n)| (i, n.clone()))
        .collect();
    assert!(!dead.is_empty(), "fixture must shed");
    let count = specimen.tree.nodes.len();
    let keys: Vec<_> = specimen.tree.nodes.iter().map(|n| n.identity.key).collect();
    specimen.advance(3.0).unwrap();
    assert!(
        specimen.tree.nodes.len() > count,
        "fixture must grow after death"
    );
    for (i, node) in dead {
        assert_eq!(specimen.tree.nodes[i], node, "growth rewrote a dead record");
        assert_eq!(specimen.identities[node.identity.key], i);
        assert!(specimen.node(node.identity).is_err());
        assert!(specimen
            .placements()
            .unwrap()
            .iter()
            .all(|p| p.identity.shoot != node.identity));
    }
    assert!(specimen.tree.nodes[count..]
        .iter()
        .all(|n| !keys.contains(&n.identity.key)));
    assert!(specimen
        .tree()
        .nodes
        .iter()
        .all(|n| n.shoot.death_year.is_none()));
}

#[test]
fn vigour_records_preserve_every_earlier_observation_and_counter() {
    let mut family = Preset::Ordinary.parameters();
    family.age = 7.0;
    let mut specimen = Specimen::build(&family).unwrap();
    let previous = specimen.tree.nodes.clone();
    assert!(previous.iter().any(|n| n.shoot.vigour_events.len() > 1));
    specimen.advance(3.0).unwrap();
    for old in previous {
        let i = specimen.identities[old.identity.key];
        let events = &specimen.tree.nodes[i].shoot.vigour_events;
        assert!(
            events.starts_with(&old.shoot.vigour_events),
            "growth overwrote vigour history"
        );
        assert!(events.windows(2).all(|pair| pair[0].year < pair[1].year));
        assert!(events.iter().all(|e| e.year <= 10));
    }
}
