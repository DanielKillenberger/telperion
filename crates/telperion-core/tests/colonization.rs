use telperion_core::{
    colonization::{colonize, GrowthConfig},
    envelope::Envelope,
    math::Vec3,
};

fn config() -> GrowthConfig {
    GrowthConfig {
        influence_radius: 4.5,
        kill_distance: 1.0,
        step_distance: 0.5,
        trunk_height: 6.0,
        max_nodes: 4000,
        ..Default::default()
    }
}
fn cloud() -> Vec<Vec3> {
    (0..300)
        .map(|i| {
            let t = i as f64 / 300.0;
            let a = i as f64 * 2.399_963;
            Vec3::new(
                a.cos() * 9.0 * t.sqrt(),
                6.0 + t * 4.0,
                a.sin() * 9.0 * t.sqrt(),
            )
        })
        .collect()
}
#[test]
fn deterministic_forking_and_termination() {
    let points = cloud();
    let tree = colonize(&points, Vec3::ZERO, &config(), None).unwrap();
    assert_eq!(
        tree,
        colonize(&points, Vec3::ZERO, &config(), None).unwrap()
    );
    tree.validate().unwrap();
    assert_eq!(tree.crossover, tree.nodes.len());
    assert!(tree.nodes.len() > 50 && tree.nodes.len() < 4000);
    let mut children = vec![0; tree.nodes.len()];
    for node in &tree.nodes {
        if let Some(p) = node.parent {
            children[p as usize] += 1;
        }
    }
    assert!(children.iter().filter(|&&n| n > 1).count() > 5);
    let reached = points
        .iter()
        .filter(|p| tree.nodes.iter().any(|n| n.position.distance(**p) <= 1.0))
        .count();
    assert!(reached > 270);
    for (i, n) in tree.nodes.iter().enumerate().skip(1) {
        let p = n.parent.unwrap() as usize;
        assert!((n.position.distance(tree.nodes[p].position) - 0.5).abs() < 1e-10);
        assert!(!tree.nodes[..i]
            .iter()
            .any(|old| old.parent == n.parent && old.position == n.position));
        if tree.nodes[p].position.y >= 6.0 {
            assert!(n.position.y >= 6.0);
        }
    }
}
#[test]
fn crown_edges_and_outside_approach() {
    let shell = Envelope {
        height: 12.0,
        crown_base: 0.5,
        spread: 0.1,
        ..Default::default()
    };
    for (start, target, reach) in [
        (Vec3::ZERO, Vec3::new(3.0, 8.0, 0.0), 20.0),
        (Vec3::ZERO, Vec3::new(0.0, 20.0, 0.0), 20.0),
        (Vec3::ZERO, Vec3::new(0.0, 20.0, 0.0), 0.1),
        (Vec3::new(4.0, 8.0, 0.0), Vec3::new(0.0, 8.0, 0.0), 20.0),
    ] {
        let cfg = GrowthConfig {
            shell: Some(shell),
            influence_radius: reach,
            kill_distance: 0.1,
            ..config()
        };
        let tree = colonize(&[target], start, &cfg, None).unwrap();
        tree.validate().unwrap();
        assert!(tree.nodes.len() > 2);
        assert!(tree
            .nodes
            .iter()
            .any(|n| n.position.y >= 6.0 && shell.contains(n.position, 0.0)));
        for n in tree.nodes.iter().skip(1) {
            let p = &tree.nodes[n.parent.unwrap() as usize];
            if p.position.y >= 6.0 && shell.contains(p.position, 0.0) {
                assert!(shell.contains(n.position, 0.0));
            }
        }
    }
}
#[test]
fn empty_caps_and_invalid_inputs() {
    assert_eq!(
        colonize(&[], Vec3::ZERO, &config(), None)
            .unwrap()
            .nodes
            .len(),
        1
    );
    for cap in [0, 1, 60] {
        let cfg = GrowthConfig {
            max_nodes: cap,
            ..config()
        };
        let tree = colonize(&cloud(), Vec3::ZERO, &cfg, None).unwrap();
        assert_eq!(tree.nodes.len(), cap);
        assert!(tree.diagnostics.node_capped);
        tree.validate().unwrap();
    }
    for step in [0.0, -1.0, f64::NAN, f64::INFINITY] {
        assert!(colonize(
            &[],
            Vec3::ZERO,
            &GrowthConfig {
                step_distance: step,
                ..config()
            },
            None
        )
        .is_err());
    }
    for magnitude in [f64::NAN, f64::MAX] {
        assert!(colonize(
            &[Vec3::new(magnitude, 0.0, 0.0)],
            Vec3::ZERO,
            &config(),
            None
        )
        .is_err());
        assert!(colonize(
            &cloud(),
            Vec3::ZERO,
            &GrowthConfig {
                influence_radius: magnitude,
                ..config()
            },
            None
        )
        .is_err());
    }
    // The bounded grid must remain safe for large representable coordinates and tiny cells.
    let distant = Vec3::new(1e100, 0.0, 0.0);
    let result = colonize(
        &[distant],
        distant,
        &GrowthConfig {
            influence_radius: 1e-100,
            ..config()
        },
        None,
    )
    .unwrap();
    result.validate().unwrap();
}

#[test]
fn fill_measures_only_classified_shell_wood_and_crown_tips() {
    use telperion_core::{
        colonization::fill::{shell_occupancy, tip_clustering, FillMeasurement},
        tree::{Node, Tree},
    };
    let shell = Envelope {
        height: 10.0,
        crown_base: 0.0,
        spread: 0.5,
        ..Default::default()
    };
    let mut tree = Tree {
        nodes: vec![
            Node::root(),
            Node {
                position: Vec3::new(4.0, 4.0, 0.0),
                parent: Some(0),
                branch: 1,
                ..Node::root()
            },
        ],
        crossover: 2,
        ..Default::default()
    };
    let first = shell_occupancy(&tree, &[false, true], shell, 0.1, 0.45).unwrap();
    match first {
        FillMeasurement::Tested {
            numerator,
            denominator,
        } => {
            assert_eq!(numerator, 1);
            assert!(denominator > 1);
        }
        other => panic!("{other:?}"),
    }
    tree.nodes.push(Node {
        position: Vec3::new(4.1, 4.0, 0.0),
        parent: Some(1),
        branch: 2,
        ..Node::root()
    });
    assert_eq!(
        shell_occupancy(&tree, &[false, true, true], shell, 0.1, 0.45).unwrap(),
        first
    );
    assert_eq!(
        tip_clustering(&tree, &[false, false, true], 0.2).unwrap(),
        FillMeasurement::Tested {
            numerator: 1,
            denominator: 1
        }
    );
    tree.nodes[2].position = Vec3::new(40.0, 4.0, 0.0);
    assert_eq!(
        shell_occupancy(&tree, &[false, true, true], shell, 0.1, 0.45).unwrap(),
        first
    );
    assert_eq!(
        tip_clustering(&tree, &[false, false, true], 0.2).unwrap(),
        FillMeasurement::Tested {
            numerator: 0,
            denominator: 1
        }
    );
    assert_eq!(
        shell_occupancy(&tree, &[false; 3], shell, 0.1, 0.45).unwrap(),
        FillMeasurement::Untested("no-terminals")
    );
    assert!(shell_occupancy(&tree, &[true], shell, 0.1, 0.45).is_err());
    assert!(shell_occupancy(&tree, &[false, true, true], shell, 1e-6, 0.45).is_err());
}

#[test]
fn oversized_climb_step_cannot_skip_the_entire_crown() {
    let shell = Envelope {
        height: 12.0,
        crown_base: 0.5,
        ..Default::default()
    };
    let tree = colonize(
        &[Vec3::new(0.0, 30.0, 0.0)],
        Vec3::ZERO,
        &GrowthConfig {
            step_distance: 20.0,
            influence_radius: 0.1,
            shell: Some(shell),
            ..config()
        },
        None,
    )
    .unwrap();
    assert!(tree.nodes.iter().all(|n| n.position.y <= shell.height));
}
