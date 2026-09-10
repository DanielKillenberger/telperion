use telperion_core::{branching::generate, presets::Preset};
#[test]
fn complete_presets_are_deterministic_and_solved() {
    for preset in [Preset::Ordinary, Preset::Telperion, Preset::Laurelin] {
        let p = preset.parameters();
        let a = generate(&p.skeleton, p.radii).unwrap();
        a.tree.validate_solved().unwrap();
        assert_eq!(a, generate(&p.skeleton, p.radii).unwrap());
        assert!(a.tree.nodes.len() > a.tree.crossover);
        assert!(a.tree.diagnostics.complete());
        for n in a.tree.nodes.iter().skip(a.tree.crossover) {
            assert!(p.skeleton.envelope.contains(n.position, 1e-8));
            assert!(
                n.position
                    .distance(a.tree.nodes[n.parent.unwrap() as usize].position)
                    > 0.0
            );
        }
    }
}

use telperion_core::{
    branching::{append, shed, HabitParams, SkeletonParams},
    colonization::GrowthConfig,
    envelope::Envelope,
    math::Vec3,
    radius::{solve, RadiusParams},
    tree::{Node, NodeKind, Tree},
    twigs::TwigParams,
};
fn crown() -> Tree {
    let mut root = Node::root();
    root.radius = 0.1;
    root.start_radius = 0.1;
    let tip = Node {
        position: Vec3::new(0.0, 1.0, 0.0),
        parent: Some(0),
        radius: 0.1,
        start_radius: 0.1,
        branch: 1,
        ..Node::root()
    };
    Tree {
        nodes: vec![root, tip],
        crossover: 2,
        ..Default::default()
    }
}
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
#[test]
fn generation_limit_is_explicit() {
    let mut tree = crown();
    let mut t = TwigParams {
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
    assert!(tree.diagnostics.level_capped);
    assert!(!tree.diagnostics.complete());
}
#[test]
fn empty_caps_invalid_and_finite_rails() {
    // A pull with nothing to pull towards names both fields; a cloud nobody
    // pulls on costs nothing and grows the rule tree.
    let mut p = SkeletonParams {
        attractors: 0,
        ..Default::default()
    };
    assert_eq!(
        generate(&p, RadiusParams::default()).err(),
        Some(telperion_core::Error::InvalidInput(
            "attractor weight and attractor count"
        ))
    );
    p.habit.attractor_weight = 0.0;
    assert!(
        generate(&p, RadiusParams::default())
            .unwrap()
            .tree
            .nodes
            .len()
            > 1
    );
    p.growth.max_nodes = Some(0);
    let zero = generate(&p, RadiusParams::default()).unwrap();
    assert!(zero.tree.nodes.is_empty());
    assert!(zero.tree.diagnostics.node_capped);
    p.attractors = 500;
    p.habit.attractor_weight = 1.0;
    p.growth.max_nodes = Some(20);
    let capped = generate(&p, RadiusParams::default()).unwrap();
    assert_eq!(capped.tree.nodes.len(), 20);
    assert!(capped.tree.diagnostics.node_capped);
    p.step = f64::NAN;
    assert!(generate(&p, RadiusParams::default()).is_err());
    p.step = 0.022;
    p.twigs.angle = f64::INFINITY;
    assert!(generate(&p, RadiusParams::default()).is_err());
    let t = TwigParams {
        length_ratio: 4.0,
        ratio_power: -1.0,
        laterals: 99,
        ..Default::default()
    }
    .resolved()
    .unwrap();
    assert_eq!((t.length_ratio, t.ratio_power, t.laterals), (1.0, 0.0, 7));
    let mut malformed = crown();
    malformed.nodes[1].parent = Some(1);
    assert!(append(
        &mut malformed,
        &GrowthConfig::default(),
        TwigParams::default(),
        0,
        None,
        HabitParams::default()
    )
    .is_err());
    let mut empty = Tree::default();
    solve(&mut empty, Envelope::default(), RadiusParams::default()).unwrap();
    assert!(empty.nodes.is_empty());
    let mut degenerate = SkeletonParams::default();
    degenerate.envelope.height = 0.0;
    assert_eq!(
        generate(&degenerate, RadiusParams::default())
            .unwrap()
            .tree
            .nodes
            .len(),
        1
    );
}
#[test]
fn family_and_seed_controls_rederive_growth() {
    let mut p = Preset::Telperion.parameters();
    let a = p.skeleton.resolved_growth(p.skeleton.attractors).unwrap();
    p.skeleton.envelope.height *= 0.5;
    let b = p.skeleton.resolved_growth(p.skeleton.attractors).unwrap();
    assert_eq!(a.step_distance / 2.0, b.step_distance);
    assert_eq!(a.max_turn_per_step, b.max_turn_per_step);
    p.skeleton.attractors = 32;
    p.skeleton.growth.max_nodes = Some(2000);
    let a = generate(&p.skeleton, p.radii).unwrap();
    p.skeleton.seed += 1;
    let b = generate(&p.skeleton, p.radii).unwrap();
    assert_ne!(a.tree.nodes, b.tree.nodes);
    let fine = SkeletonParams {
        step: 0.003,
        attractors: 64,
        ..Default::default()
    };
    let c = fine.resolved_growth(64).unwrap();
    assert!(c.influence_radius > 9.0 * c.step_distance);
    let grown = generate(&fine, RadiusParams::default()).unwrap();
    assert!(grown
        .tree
        .nodes
        .iter()
        .any(|n| n.position.y > fine.envelope.height * 0.6));
}

#[test]
fn natural_bias_is_independent_of_disabled_effects() {
    use telperion_core::bias::{BiasParams, SupernaturalParams};
    let mut p = SkeletonParams {
        attractors: 64,
        ..Default::default()
    };
    assert!(!p.bias.supernatural.enabled);
    let ordinary = Preset::Ordinary.parameters();
    assert_eq!(
        (ordinary.surface.lobe_depth, ordinary.surface.twist_rate),
        (0.0, 0.0)
    );
    let natural = generate(&p, RadiusParams::default()).unwrap();
    p.bias.supernatural = SupernaturalParams {
        enabled: false,
        writhe_amplitude: 0.2,
        writhe_wavelength: 0.1,
        spiral_rate: 3.0,
    };
    assert_eq!(natural, generate(&p, RadiusParams::default()).unwrap());
    p.bias.supernatural.enabled = true;
    assert_ne!(
        natural.tree.nodes,
        generate(&p, RadiusParams::default()).unwrap().tree.nodes
    );
    p.bias = BiasParams::NONE;
    assert_ne!(
        natural.tree.nodes,
        generate(&p, RadiusParams::default()).unwrap().tree.nodes
    );
    for preset in [Preset::Telperion, Preset::Laurelin] {
        assert!(preset.parameters().skeleton.bias.supernatural.enabled);
    }
}

fn habit_family(habit: HabitParams) -> SkeletonParams {
    SkeletonParams {
        habit,
        envelope: Envelope {
            height: 16.0,
            crown_base: 0.12,
            spread: 0.3,
            fullness: 0.18,
            shoulder: 1.2,
        },
        bias: telperion_core::bias::BiasParams::NONE,
        ..Default::default()
    }
}
/// A row at full apical dominance whose deeper axes hang: a conifer's corner
/// of the trait space, reached by numbers alone.
fn hanging_row() -> HabitParams {
    HabitParams {
        apical_dominance: 1.0,
        whorl_strength: 1.0,
        leader_internode: 0.9,
        laterals_per_station: 5,
        lateral_pitch: 88.0,
        pitch_variation: 4.0,
        rise_primary: 0.12,
        rise_secondary: -0.8,
        crookedness: 0.0,
        lateral_spacing: 0.2,
        lateral_length_ratio: 0.3,
        lateral_orders: 2,
        attractor_weight: 0.0,
        twig_tip_taper: 0.25,
        shedding_threshold: 0.0,
    }
}
/// The opposite corner: the leader yields early and every axis is crooked.
fn crooked_row() -> HabitParams {
    HabitParams {
        apical_dominance: 0.1,
        whorl_strength: 0.1,
        leader_internode: 2.0,
        laterals_per_station: 5,
        lateral_pitch: 55.0,
        pitch_variation: 20.0,
        rise_primary: 0.12,
        rise_secondary: 0.0,
        crookedness: 24.0,
        lateral_spacing: 1.2,
        lateral_length_ratio: 0.45,
        lateral_orders: 5,
        attractor_weight: 0.0,
        twig_tip_taper: 0.25,
        shedding_threshold: 0.0,
    }
}

#[test]
fn a_persistent_leader_carries_hanging_wood() {
    let p = habit_family(hanging_row());
    let tree = generate(&p, RadiusParams::default()).unwrap().tree;
    let structural = &tree.nodes[..tree.crossover];
    assert!(structural
        .iter()
        .any(|n| (n.position.y - p.envelope.height).abs() < 1e-9));
    let mut leader = 0;
    let mut hanging = 0;
    let mut upturned = 0;
    for n in structural.iter().skip(1) {
        let parent = &tree.nodes[n.parent.unwrap() as usize];
        let delta = n.position - parent.position;
        if n.position.x.hypot(n.position.z) < 1e-10 {
            leader += 1;
            assert!(delta.y > 0.0);
        }
        if delta.y < -0.05 && delta.y.abs() > delta.x.hypot(delta.z) {
            hanging += 1;
        }
        if delta.y > 0.01 && delta.x.hypot(delta.z) > delta.y {
            upturned += 1;
        }
    }
    assert!(
        leader > 10 && hanging > 50 && upturned > 20,
        "{leader} {hanging} {upturned}"
    );
}

#[test]
fn a_crooked_row_subdivides_substantial_axes_without_effects() {
    let mut p = habit_family(crooked_row());
    p.envelope = Envelope {
        height: 20.0,
        crown_base: 0.2,
        spread: 0.55,
        fullness: 0.55,
        shoulder: 2.2,
    };
    let tree = generate(&p, RadiusParams::default()).unwrap().tree;
    let mut children = vec![0; tree.crossover];
    let mut bends = 0;
    let mut forks = 0;
    for n in tree.nodes.iter().take(tree.crossover).skip(1) {
        children[n.parent.unwrap() as usize] += 1;
    }
    for (i, n) in tree.nodes.iter().take(tree.crossover).enumerate().skip(1) {
        let parent = &tree.nodes[n.parent.unwrap() as usize];
        if children[i] > 1 && n.radius > 0.02 {
            forks += 1;
        }
        if let Some(grand) = parent.parent {
            let a = (parent.position - tree.nodes[grand as usize].position).normalized();
            let b = (n.position - parent.position).normalized();
            if a.dot(b) < 0.995 && n.radius > 0.02 {
                bends += 1;
            }
        }
    }
    assert!(forks >= 10 && bends >= 20, "forks {forks}, bends {bends}");
    let local_bends = tree
        .nodes
        .iter()
        .skip(tree.crossover)
        .filter(|n| {
            let parent = &tree.nodes[n.parent.unwrap() as usize];
            if n.kind != NodeKind::Branch || n.branch != parent.branch {
                return false;
            }
            let Some(grand) = parent.parent else {
                return false;
            };
            let a = (parent.position - tree.nodes[grand as usize].position).normalized();
            let b = (n.position - parent.position).normalized();
            a.dot(b) < 0.999
        })
        .count();
    assert!(
        local_bends > 10,
        "natural crookedness must reach local axes: {local_bends}"
    );
    let mut straight = p.clone();
    straight.habit.crookedness = 0.0;
    assert_ne!(
        tree.nodes,
        generate(&straight, RadiusParams::default())
            .unwrap()
            .tree
            .nodes
    );
}

#[test]
fn habit_topology_bounds_seeds_and_limits_are_explicit() {
    for habit in [hanging_row(), crooked_row()] {
        let mut p = habit_family(habit);
        let a = generate(&p, RadiusParams::default()).unwrap();
        assert_eq!(a, generate(&p, RadiusParams::default()).unwrap());
        a.tree.validate_solved().unwrap();
        assert!(a.tree.diagnostics.complete());
        for n in a.tree.nodes.iter().skip(1) {
            let parent = &a.tree.nodes[n.parent.unwrap() as usize];
            assert!(n.position.distance(parent.position) > 1e-9);
            for t in [0.0, 0.25, 0.5, 0.75, 1.0] {
                assert!(p
                    .envelope
                    .contains(parent.position.lerp(n.position, t), 1e-8));
            }
        }
        p.seed += 1;
        assert_ne!(
            a.tree.nodes,
            generate(&p, RadiusParams::default()).unwrap().tree.nodes
        );
        for limit in [0, 1, 20] {
            p.growth.max_nodes = Some(limit);
            let tree = generate(&p, RadiusParams::default()).unwrap().tree;
            assert!(tree.nodes.len() <= limit);
            assert!(tree.diagnostics.node_capped);
            tree.validate().unwrap();
        }
    }
}

#[test]
fn every_habit_trait_is_a_range_and_names_itself() {
    let row = crooked_row();
    for (habit, named) in [
        (
            HabitParams {
                apical_dominance: 1.5,
                ..row
            },
            "apical dominance",
        ),
        (
            HabitParams {
                whorl_strength: f64::NAN,
                ..row
            },
            "whorl strength",
        ),
        (
            HabitParams {
                leader_internode: 0.0,
                ..row
            },
            "leader internode",
        ),
        (
            HabitParams {
                laterals_per_station: 0,
                ..row
            },
            "laterals per station",
        ),
        (
            HabitParams {
                crookedness: 61.0,
                ..row
            },
            "crookedness",
        ),
        (
            HabitParams {
                lateral_spacing: -1.0,
                ..row
            },
            "lateral spacing",
        ),
        (
            HabitParams {
                rise_secondary: -1.5,
                ..row
            },
            "secondary rise per order",
        ),
        (
            HabitParams {
                attractor_weight: 1.5,
                ..row
            },
            "attractor weight",
        ),
        (
            HabitParams {
                shedding_threshold: 2.0,
                ..row
            },
            "shedding threshold",
        ),
    ] {
        assert_eq!(
            generate(&habit_family(habit), RadiusParams::default()).err(),
            Some(telperion_core::Error::InvalidInput(named))
        );
    }
    // Attractors are the one pairing: a pull with nothing to pull towards.
    let mut p = habit_family(HabitParams {
        attractor_weight: 0.5,
        ..row
    });
    p.attractors = 0;
    assert_eq!(
        generate(&p, RadiusParams::default()).err(),
        Some(telperion_core::Error::InvalidInput(
            "attractor weight and attractor count"
        ))
    );
    p.habit.attractor_weight = 0.0;
    assert!(generate(&p, RadiusParams::default()).is_ok());
}

#[test]
fn clipped_local_axis_still_subdivides_before_its_terminal_twig() {
    let mut tree = crown();
    let config = GrowthConfig {
        trunk_height: 0.0,
        shell: Some(Envelope {
            height: 2.0,
            crown_base: 0.0,
            spread: 1.0,
            fullness: 0.5,
            shoulder: 2.0,
        }),
        max_nodes: 10000,
        ..Default::default()
    };
    let t = TwigParams {
        limb_radius: 0.0,
        internode_factor: 8.0,
        angle_variation: 0.0,
        vigour_variation: 0.0,
        ..Default::default()
    };
    append(&mut tree, &config, t, 7, None, HabitParams::default()).unwrap();
    let origins = tree
        .nodes
        .iter()
        .enumerate()
        .skip(2)
        .filter(|(i, n)| n.kind == NodeKind::Branch && n.branch as usize == *i)
        .count();
    assert!(
        origins >= 3,
        "clipped axis lost its lateral branches: {origins}"
    );
    for n in tree.nodes.iter().skip(2) {
        assert!(config.shell.unwrap().contains(n.position, 1e-9));
    }
}
