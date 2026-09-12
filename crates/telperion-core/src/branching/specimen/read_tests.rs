use super::{tests::bytes, *};
use crate::presets::Preset;

#[test]
fn unread_advances_do_not_pack_and_reads_preserve_internal_indices() {
    let mut f = Preset::OregonWhiteOak.parameters();
    f.age = 12.0;
    let mut s = Specimen::build(&f).unwrap();
    s.advance(1.0).unwrap();
    assert_eq!(s.cost.packed_nodes, 0, "unread advance packed the tree");
    assert!(s.timeline.as_ref().unwrap().unpacked);
    let internal = bytes(&s.tree);
    let tree = s.tree();
    tree.validate_solved().unwrap();
    assert!(tree.nodes[..tree.crossover]
        .iter()
        .all(|n| n.kind == NodeKind::Structural));
    assert!(tree.nodes[tree.crossover..]
        .iter()
        .all(|n| n.kind != NodeKind::Structural));
    assert_eq!(
        s.identities().collect::<Vec<_>>(),
        tree.nodes.iter().map(|n| n.identity).collect::<Vec<_>>()
    );
    for n in &tree.nodes {
        assert_eq!(s.node(n.identity).unwrap(), n);
    }
    assert_eq!(
        bytes(&s.tree),
        internal,
        "read moved the frontier's storage"
    );
    assert!(
        std::ptr::eq(tree, s.tree()),
        "unchanged read rebuilt the cache"
    );
}

#[test]
fn reads_between_advances_reconcile_after_growth_shedding_and_cap_recovery() {
    let mut f = Preset::Ordinary.parameters();
    f.age = 12.25;
    let fresh = Specimen::build(&f).unwrap();
    assert!(fresh.shed > 0);
    f.age = 0.0;
    let mut chain = Specimen::build(&f).unwrap();
    for years in [4.0, 4.0, 4.25] {
        let previous = chain.tree().clone();
        chain.advance(years).unwrap();
        chain.tree().validate_solved().unwrap();
        for n in previous.nodes {
            if let Ok(live) = chain.node(n.identity) {
                assert_eq!(live.position, n.position);
            }
        }
    }
    assert!(
        bytes(chain.tree()) == bytes(fresh.tree()),
        "read cache retained stale wood"
    );
    assert_eq!(chain.shed, fresh.shed);
    let previous = bytes(chain.tree());
    assert!(chain.advance(-1.0).is_err());
    assert_eq!(bytes(chain.tree()), previous);
    chain.set_node_ceiling(chain.tree.nodes.len()).unwrap();
    chain.advance(1.0).unwrap();
    assert!(chain.tree().diagnostics.node_capped);
    chain.set_node_ceiling(NODE_CEILING).unwrap();
    assert!(!chain.tree().diagnostics.node_capped);
    chain.advance(1.0).unwrap();
    f.age = chain.age();
    assert_eq!(
        bytes(chain.tree()),
        bytes(Specimen::build(&f).unwrap().tree())
    );
}

#[test]
fn finalizing_grown_and_shed_records_ignores_stored_output_widths() {
    let mut f = Preset::Ordinary.parameters();
    f.age = 0.0;
    let mut lazy = Specimen::build(&f).unwrap();
    for month in 1..=147 {
        let budget = f.growth.budget(month);
        if budget > 0 {
            lazy.month(month, budget).unwrap();
        }
        lazy.timeline.as_mut().unwrap().age = crate::growth::Age {
            month,
            remainder: 0,
        };
    }
    assert!(lazy.shed > 0 && lazy.tree.nodes.len() > 100);
    let mut poisoned = lazy.clone();
    for n in &mut poisoned.tree.nodes {
        n.radius = 1e6;
        n.start_radius = 1e6;
        if n.kind != NodeKind::Structural {
            n.base_radius = 1e6;
        }
    }
    lazy.finish_widths().unwrap();
    poisoned.finish_widths().unwrap();
    assert!(
        bytes(lazy.tree()) == bytes(poisoned.tree()),
        "finalization read stored output radii instead of the slice record"
    );
    f.age = 12.25;
    assert!(
        bytes(lazy.tree()) == bytes(Specimen::build(&f).unwrap().tree()),
        "skipping intermediate finalization changed growth or shedding"
    );
}
