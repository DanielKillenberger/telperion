use super::*;

#[test]
fn local_anatomy_attachment_taper_and_resolution() {
    let config = GrowthConfig {
        trunk_height: 0.0,
        max_nodes: 10000,
        shell: None,
        ..Default::default()
    };
    let coarse = TwigParams {
        internode_factor: 8.0,
        vigour_variation: 0.0,
        angle_variation: 0.0,
        ..Default::default()
    };
    let straight = HabitParams {
        crookedness: 0.0,
        ..HabitParams::default()
    };
    let mut a = crown();
    append(&mut a, &config, coarse, 7, None, straight).unwrap();
    let mut b = crown();
    append(
        &mut b,
        &config,
        TwigParams {
            internode_factor: 2.0,
            ..coarse
        },
        7,
        None,
        straight,
    )
    .unwrap();
    let lateral_roots = |tree: &Tree| {
        tree.nodes
            .iter()
            .enumerate()
            .filter(|(i, n)| n.kind == NodeKind::Branch && n.branch as usize == *i)
            .map(|(_, n)| {
                (
                    n.position,
                    tree.nodes[n.parent.unwrap() as usize].position,
                    n.base_radius,
                )
            })
            .collect::<Vec<_>>()
    };
    let ar = lateral_roots(&a);
    let br = lateral_roots(&b);
    assert_eq!(ar.len(), br.len());
    for (x, y) in ar.iter().zip(&br) {
        assert!(x.1.distance(y.1) < 1e-10);
        assert!((x.2 - y.2).abs() < 1e-12)
    }
    for tree in [&a, &b] {
        tree.validate_solved().unwrap();
        assert_eq!(&tree.nodes[..2], &crown().nodes);
        let mut twigs = 0;
        for (i, n) in tree.nodes.iter().enumerate().skip(2) {
            let parent = &tree.nodes[n.parent.unwrap() as usize];
            if n.kind == NodeKind::Twig {
                twigs += 1;
                assert!((n.position.distance(parent.position) - coarse.twig.length).abs() < 1e-12);
                assert_eq!(n.radius, coarse.twig.diameter / 2.0);
                assert_eq!(n.start_radius, n.radius)
            } else if n.branch as usize != i {
                assert_eq!(n.start_radius, parent.radius)
            } else {
                assert_eq!(n.start_radius, n.base_radius);
                assert!(n.base_radius <= parent.radius)
            }
            assert!(n.radius <= n.start_radius)
        }
        assert!(twigs > 0);
    }
}
#[test]
fn fork_conservation_and_appended_radius_independence() {
    let mut tree = crown();
    tree.nodes.push(Node {
        position: Vec3::new(1.0, 2.0, 0.0),
        parent: Some(1),
        branch: 2,
        ..Node::root()
    });
    tree.nodes.push(Node {
        position: Vec3::new(-1.0, 2.0, 0.0),
        parent: Some(1),
        branch: 3,
        ..Node::root()
    });
    tree.crossover = 4;
    let e = Envelope::default();
    let p = RadiusParams::default();
    solve(&mut tree, e, p).unwrap();
    assert!(
        (tree.nodes[1].radius.powi(2)
            - tree.nodes[2].start_radius.powi(2)
            - tree.nodes[3].start_radius.powi(2))
        .abs()
            < 1e-12
    );
    assert_eq!(tree.nodes[0].radius, e.height * p.trunk_radius);
    let before = tree.nodes.clone();
    append(
        &mut tree,
        &GrowthConfig {
            trunk_height: 0.0,
            max_nodes: 5000,
            ..Default::default()
        },
        TwigParams::default(),
        2,
        None,
        HabitParams::default(),
    )
    .unwrap();
    solve(&mut tree, e, p).unwrap();
    assert_eq!(&tree.nodes[..4], &before);
}
#[test]
fn shedding_remaps_runs_preserves_transitions_and_caps() {
    let mut tree = crown();
    tree.nodes[1].position = Vec3::new(0.0, 12.0, 0.0);
    let config = GrowthConfig {
        trunk_height: 0.0,
        max_nodes: 20,
        ..Default::default()
    };
    append(
        &mut tree,
        &config,
        TwigParams::default(),
        0,
        None,
        HabitParams::default(),
    )
    .unwrap();
    assert!(tree.diagnostics.node_capped);
    let original = tree.nodes.len();
    let removed = shed(&mut tree, Envelope::default(), 0.0).unwrap();
    assert!(removed > 0);
    assert_eq!(tree.nodes.len() + removed, original);
    assert!(tree.diagnostics.node_capped);
    tree.validate_solved().unwrap();
    for n in tree.nodes.iter().skip(tree.crossover) {
        assert_eq!(tree.nodes[n.branch as usize].branch, n.branch)
    }
}
#[test]
fn shedding_keeps_whole_runs_and_terminal_transition() {
    let mut tree = crown();
    tree.nodes[1].position = Vec3::new(0.0, 12.0, 0.0);
    for (parent, branch, kind, x, y, radius, base) in [
        (1, 2, NodeKind::Twig, 0.0, 12.25, 0.0025, 0.0025),
        (1, 3, NodeKind::Branch, 7.0, 12.0, 0.01, 0.1),
        (3, 3, NodeKind::Branch, 0.0, 13.0, 0.0025, 0.1),
        (4, 5, NodeKind::Twig, 0.0, 13.25, 0.0025, 0.0025),
    ] {
        tree.nodes.push(Node {
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
    assert_eq!(shed(&mut tree, Envelope::default(), 0.0).unwrap(), 1);
    assert_eq!(tree.nodes.len(), 5);
    assert_eq!(tree.nodes[2].branch, 2);
    assert_eq!(tree.nodes[3].branch, 2);
    assert_eq!(tree.nodes[4].branch, 4);
    assert_eq!(tree.nodes[4].parent, Some(3));
    assert_eq!(tree.nodes[4].kind, NodeKind::Twig);
}
/// Laterals whose radius never falls become twigs at the authored generation,
/// and the tree completes without an independent depth ceiling.
#[test]
fn generation_limit_is_explicit() {
    let mut tree = crown();
    let mut t = TwigParams {
        generations: 3,
        length_ratio: 1.0,
        ratio_power: 0.0,
        laterals: 1,
        internode_factor: 32.0,
        limb_radius: 0.0,
        vigour_variation: 0.0,
        angle_variation: 0.0,
        ..Default::default()
    };
    t.twig.diameter = 1e-6;
    t.twig.bearing_diameter = 1e-6;
    append(
        &mut tree,
        &GrowthConfig {
            trunk_height: 0.0,
            max_nodes: 250000,
            ..Default::default()
        },
        t,
        0,
        None,
        HabitParams::default(),
    )
    .unwrap();
    assert!(!tree.diagnostics.level_capped);
    assert!(tree.diagnostics.complete());
    let deepest = tree
        .nodes
        .iter()
        .filter(|n| n.kind == NodeKind::Branch)
        .count();
    assert!(deepest > 0, "the row left no wood at all");
    assert!(
        tree.nodes.iter().any(|n| n.kind == NodeKind::Twig),
        "the row made no twigs"
    );
}
