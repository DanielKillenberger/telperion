use super::*;

fn habit_family(habit: HabitParams) -> SkeletonParams {
    SkeletonParams {
        habit,
        envelope: Envelope {
            height: 16.0,
            crown_base: 0.12,
            spread: 0.3,
            fullness: 0.18,
            shoulder: 1.2,
            irregularity: 0.0,
            lobe_scale: 0.5,
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
        stems: 1,
        stem_divergence: 0.0,
        stem_lean: 0.0,
        stem_lean_spread: 0.0,
        stem_fork_height: 0.0,
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
        stems: 1,
        stem_divergence: 0.0,
        stem_lean: 0.0,
        stem_lean_spread: 0.0,
        stem_fork_height: 0.0,
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
        irregularity: 0.0,
        lobe_scale: 0.5,
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
                assert!(
                    p.envelope
                        .contains(parent.position.lerp(n.position, t), 1e-8, p.seed),
                    "seed {}: outside the perturbed shell",
                    p.seed
                );
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
            irregularity: 0.0,
            lobe_scale: 0.5,
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
        assert!(config
            .shell
            .unwrap()
            .contains(n.position, 1e-9, config.seed));
    }
}
