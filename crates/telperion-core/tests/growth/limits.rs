use super::*;

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
