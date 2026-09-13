use super::*;
use crate::{presets::Preset, tree::NodeKind};
use std::collections::HashSet;

#[test]
fn retained_frontiers_grow_structure_after_twigs_without_changing_identities() {
    let f = Preset::Ordinary.parameters();
    let mut s = Specimen::new(&f.skeleton, f.radii).unwrap();
    let mut previous = Vec::new();
    let mut after_twigs = false;
    for _ in 0..200 {
        let old_crossover = s.tree().crossover;
        let had_twigs = s.tree().nodes.iter().any(|n| n.kind == NodeKind::Twig);
        s.step(8, 64).unwrap();
        let tree = s.tree();
        tree.validate_solved().unwrap();
        assert!(tree.nodes[..tree.crossover]
            .iter()
            .all(|n| n.kind == NodeKind::Structural));
        assert!(tree.nodes[tree.crossover..]
            .iter()
            .all(|n| n.kind != NodeKind::Structural));
        let ids: HashSet<_> = tree.nodes.iter().map(|n| n.identity).collect();
        assert_eq!(ids.len(), tree.nodes.len());
        for (id, position, parent, branch) in previous {
            let n = s
                .node(id)
                .expect("surviving identity resolves after insertion");
            assert_eq!(n.position, position, "identity moved to another node");
            assert_eq!(n.parent.map(|p| tree.nodes[p as usize].identity), parent);
            assert_eq!(tree.nodes[n.branch as usize].identity, branch);
        }
        assert_eq!(
            s.identities().collect::<Vec<_>>(),
            tree.nodes.iter().map(|n| n.identity).collect::<Vec<_>>()
        );
        previous = tree
            .nodes
            .iter()
            .map(|n| {
                (
                    n.identity,
                    n.position,
                    n.parent.map(|p| tree.nodes[p as usize].identity),
                    tree.nodes[n.branch as usize].identity,
                )
            })
            .collect();
        after_twigs |= had_twigs && tree.crossover > old_crossover;
        if s.finished() {
            break;
        }
    }
    assert!(
        after_twigs,
        "the test must exercise structure born after twigs"
    );
}

#[test]
fn whole_build_replays_the_retained_frontiers_exactly() {
    let f = Preset::Ordinary.parameters();
    let a = Specimen::grow(&f.skeleton, f.radii).unwrap();
    let b = Specimen::grow(&f.skeleton, f.radii).unwrap();
    assert!(
        bytes(a.tree()) == bytes(b.tree()),
        "full builds differ byte-for-byte"
    );
    assert_eq!(
        a.identities().collect::<Vec<_>>(),
        b.identities().collect::<Vec<_>>()
    );
}

#[test]
fn shedding_compacts_siblings_without_reusing_birth_identities() {
    let f = Preset::Ordinary.parameters();
    let mut s = Specimen::new(&f.skeleton, f.radii).unwrap();
    s.tree.nodes = vec![
        Node::root(),
        Node {
            position: Vec3::new(0.0, 12.0, 0.0),
            parent: Some(0),
            branch: 1,
            ..Node::root()
        },
    ];
    s.tree.crossover = 2;
    for (parent, branch, kind, x, y, radius, base) in [
        (1, 2, NodeKind::Twig, 0.0, 12.25, 0.0025, 0.0025),
        (1, 3, NodeKind::Branch, 7.0, 12.0, 0.01, 0.1),
        (3, 3, NodeKind::Branch, 0.0, 13.0, 0.0025, 0.1),
        (4, 5, NodeKind::Twig, 0.0, 13.25, 0.0025, 0.0025),
    ] {
        s.tree.nodes.push(Node {
            position: Vec3::new(x, y, 0.0),
            parent: Some(parent),
            radius,
            start_radius: base,
            base_radius: base,
            branch,
            kind,
            ..Node::root()
        });
    }
    s.identify();
    let retired = s.tree.nodes[2].identity;
    let siblings: Vec<_> = s.tree.nodes[3..].iter().map(|n| n.identity).collect();
    assert_eq!(shed(&mut s.tree, Envelope::default(), 0.0).unwrap(), 1);
    s.remap_after_shedding();
    assert_eq!(s.identities().skip(2).collect::<Vec<_>>(), siblings);
    assert!(!s.identities().any(|id| id == retired));
    s.tree.nodes.push(Node {
        parent: Some(1),
        branch: 5,
        ..Node::root()
    });
    s.identify();
    assert_eq!(s.tree.nodes.last().unwrap().identity.birth_order(), 6);
    assert_eq!(s.tree.nodes[3].branch, 2);
    assert_eq!(s.tree.nodes[4].parent, Some(3));
}

#[test]
fn scaffold_resume_preserves_each_axis_stream_and_spent_attractors() {
    let f = Preset::Ordinary.parameters();
    let a = Specimen::new(&f.skeleton, f.radii).unwrap();
    let mut full = a.clone();
    full.step(usize::MAX, 0).unwrap();
    let mut sliced = a;
    while !sliced.scaffold.finished() {
        sliced.step(7, 0).unwrap();
    }
    assert!(
        bytes(sliced.tree()) == bytes(full.tree()),
        "resumed scaffold differs byte-for-byte"
    );
}

pub(super) fn bytes(tree: &Tree) -> Vec<u8> {
    let mut out = Vec::new();
    out.extend((tree.crossover as u64).to_le_bytes());
    out.extend([
        tree.diagnostics.node_capped as u8,
        tree.diagnostics.level_capped as u8,
        tree.diagnostics.attraction_capped as u8,
    ]);
    for n in &tree.nodes {
        out.extend(n.identity.to_le_bytes());
        out.extend(n.parent.unwrap_or(u32::MAX).to_le_bytes());
        out.extend(n.branch.to_le_bytes());
        out.push(n.kind as u8);
        out.push(n.shoot.bud_fate as u8);
        out.extend(n.shoot.birth_year.to_le_bytes());
        out.extend(n.shoot.death_year.unwrap_or(u64::MAX).to_le_bytes());
        out.extend((n.shoot.vigour_events.len() as u64).to_le_bytes());
        for event in &n.shoot.vigour_events {
            out.extend(event.year.to_le_bytes());
            out.extend(event.vigour.to_le_bytes());
            out.extend(event.low_slices.to_le_bytes());
        }
        out.extend(n.shoot.vigour().to_le_bytes());
        out.extend(n.shoot.low_slices().to_le_bytes());
        out.extend(
            [
                n.position.x,
                n.position.y,
                n.position.z,
                n.radius,
                n.start_radius,
                n.base_radius,
            ]
            .into_iter()
            .flat_map(f64::to_le_bytes),
        );
    }
    out
}

#[test]
fn local_resume_preserves_pending_runs_across_irregular_budgets() {
    let f = Preset::Ordinary.parameters();
    let mut full = Specimen::new(&f.skeleton, f.radii).unwrap();
    full.step(usize::MAX, 0).unwrap();
    let mut sliced = full.clone();
    full.step(0, usize::MAX).unwrap();
    for budget in [1, 7, 31, 4, 17].into_iter().cycle() {
        if sliced.local.finished() {
            break;
        }
        sliced.step(0, budget).unwrap();
        assert!(!sliced.tree.diagnostics.node_capped);
    }
    assert!(
        bytes(sliced.tree()) == bytes(full.tree()),
        "resumed local runs differ byte-for-byte"
    );
}

#[test]
fn retired_generation_cannot_resolve_a_reused_slot() {
    let f = Preset::Ordinary.parameters();
    let mut s = Specimen::new(&f.skeleton, f.radii).unwrap();
    s.tree.nodes = vec![Node::root(), Node::root(), Node::root()];
    s.identify();
    let retired = s.tree.nodes[1].identity;
    let survivor = s.tree.nodes[2].identity;
    s.tree.nodes.remove(1);
    s.remap_after_shedding();
    assert!(s.identities.get(retired.key).is_none());
    assert_eq!(s.node(survivor).unwrap().identity, survivor);
    s.tree.nodes.push(Node::root());
    s.identify();
    let born = s.tree.nodes[2].identity;
    // The allocator really reused a slot: this must exercise the ABA case.
    assert_eq!(
        retired.key.data().as_ffi() as u32,
        born.key.data().as_ffi() as u32
    );
    assert_ne!(retired.key, born.key);
    assert!(born.birth_order() > survivor.birth_order());
    assert_eq!(
        s.node(retired),
        Err(Error::InvalidInput("stale node identity"))
    );
    assert_eq!(s.node(born).unwrap(), &s.tree.nodes[2]);
    assert_eq!(s.node(survivor).unwrap(), &s.tree.nodes[1]);
}
