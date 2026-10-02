use super::*;
use crate::{presets::Preset, tree::NodeKind};

#[test]
fn whole_build_replays_the_retained_frontiers_exactly() {
    let f = Preset::Ordinary.parameters();
    let a = Specimen::grow(&f.skeleton, f.radii, false).unwrap();
    let b = Specimen::grow(&f.skeleton, f.radii, false).unwrap();
    assert!(
        bytes(&a.tree) == bytes(&b.tree),
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

fn bytes(tree: &Tree) -> Vec<u8> {
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
        out.extend(n.codominant.map_or(u64::MAX, f64::to_bits).to_le_bytes());
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
