use telperion_core::{branching, presets::Preset};

#[test]
fn a_fine_scaffold_step_reaches_the_authored_height() {
    let mut f = Preset::Ordinary.parameters();
    f.skeleton.habit.attractor_weight = 0.;
    f.skeleton.habit.apical_dominance = 1.;
    f.skeleton.habit.crookedness = 0.;
    f.skeleton.habit.lateral_orders = 0;
    f.skeleton.bias = telperion_core::bias::BiasParams::NONE;
    f.skeleton.growth.step_distance = Some(0.001);
    f.skeleton.growth.max_nodes = Some(100_000);
    let tree = branching::generate(&f.skeleton, f.radii).unwrap().tree;
    let top = tree.nodes.iter().map(|n| n.position.y).fold(0., f64::max);
    assert!(
        top > f.skeleton.envelope.height * 0.99,
        "axis stopped at {top}"
    );
    assert!(!tree.diagnostics.node_capped);
}

#[test]
fn new_limits_are_named_validated_and_walked() {
    use serde_json::json;
    use telperion_core::{blend, params};
    let a = Preset::Ordinary.parameters();
    let overlay = json!({"canopy":{"clumpSystemOrder":4,"clumpNeighbours":24},"growth":{"workBudget":500000},"radii":{"maxTaperExponent":24},
        "surface":{"socketContainment":0.5},"skeleton":{"samplingAttemptsPerAttractor":128,
        "habit":{"reachProbeSteps":192},"bias":{"supernatural":{"maxWritheMagnitude":2}},
        "twigs":{"maxInternodes":64,"maxDroop":0.7,"curtainStepClearance":0.4}}});
    let b = params::overlay(&a, &overlay).unwrap();
    let walk = params::metadata(&blend::families(&a, &b, 0.5).unwrap());
    for (path, expected) in [
        ("/canopy/clumpSystemOrder", 3.),
        ("/canopy/clumpNeighbours", 18.),
        ("/growth/workBudget", 375000.),
        ("/radii/maxTaperExponent", 18.),
        ("/surface/socketContainment", 0.7),
        ("/skeleton/samplingAttemptsPerAttractor", 96.),
        ("/skeleton/habit/reachProbeSteps", 144.),
        ("/skeleton/bias/supernatural/maxWritheMagnitude", 1.45),
        ("/skeleton/twigs/maxInternodes", 48.),
        ("/skeleton/twigs/maxDroop", 0.525),
        ("/skeleton/twigs/curtainStepClearance", 0.6),
    ] {
        assert!(
            (walk.pointer(path).unwrap().as_f64().unwrap() - expected).abs() < 1e-12,
            "{path}"
        );
    }
    for (field, value) in [
        ("maxInternodes", 0.),
        ("maxDroop", 11.),
        ("curtainStepClearance", 1.1),
        ("lengthRatio", 0.),
        ("angle", 91.),
        ("laterals", 8.),
        ("vigourVariation", 1.),
    ] {
        let value = if matches!(field, "maxInternodes" | "laterals") {
            json!(value as u32)
        } else {
            json!(value)
        };
        let f = params::overlay(&a, &json!({"skeleton":{"twigs":{field:value}}})).unwrap();
        let error = f.skeleton.twigs.resolved().unwrap_err().to_string();
        assert!(error.contains(field), "{field}: {error}");
    }
    let mut f = a;
    f.radii.max_taper_exponent = 65.;
    assert!(f
        .radii
        .resolved()
        .unwrap_err()
        .to_string()
        .contains("maxTaperExponent"));
    f.growth.work_budget = 0;
    assert!(f
        .growth
        .validate()
        .unwrap_err()
        .to_string()
        .contains("workBudget"));
    f.skeleton.habit.reach_probe_steps = 0;
    assert!(f
        .skeleton
        .habit
        .validate()
        .unwrap_err()
        .to_string()
        .contains("reachProbeSteps"));
    f.skeleton.bias.supernatural.max_writhe_magnitude = 9.;
    assert!(f
        .skeleton
        .bias
        .validate()
        .unwrap_err()
        .to_string()
        .contains("maxWritheMagnitude"));
    f.surface.socket_containment = 1.1;
    assert!(f
        .surface
        .validate()
        .unwrap_err()
        .to_string()
        .contains("socketContainment"));
    f.canopy.clump_neighbours = 0;
    let error = telperion_core::foliage::short_shoots(
        &Default::default(),
        f.skeleton.envelope,
        1,
        f.canopy,
    )
    .unwrap_err();
    assert!(error.to_string().contains("clumpNeighbours"));
    let error = params::overlay(&b, &json!({"canopy":{"clumpSystemOrder":-1}})).unwrap_err();
    assert!(error.to_string().contains("clumpSystemOrder"));
}

#[test]
fn envelope_endpoints_and_small_shoulders_take_effect() {
    use telperion_core::envelope::Envelope;
    for fullness in [0., 1.] {
        let e = Envelope {
            fullness,
            shoulder: 0.05,
            ..Default::default()
        };
        e.validate().unwrap();
        let y = e.height * (e.crown_base + (1. - e.crown_base) * 0.5);
        let expected = e.max_radius() * (1. - 0.5_f64.powf(0.05)).powf(20.);
        assert!((e.radius_at(y) - expected).abs() < 1e-14);
    }
}

#[test]
fn sampling_budget_is_explicit_and_exhaustion_is_named() {
    use telperion_core::{envelope::Envelope, rng::Rng};
    let e = Envelope::default();
    for attempts in [0, 1] {
        let error = e
            .sample_with_attempts(100, &mut Rng::new(1), 1, attempts)
            .unwrap_err();
        assert!(error.to_string().contains("samplingAttemptsPerAttractor"));
    }
    assert_eq!(
        e.sample_with_attempts(100, &mut Rng::new(1), 1, 128)
            .unwrap()
            .len(),
        100
    );
}

#[test]
fn old_snapshots_are_refused_and_missing_new_json_fields_get_defaults() {
    let mut f = Preset::Ordinary.parameters();
    f.age = 0.;
    let s = branching::Specimen::build(&f).unwrap();
    let mut bytes = s.snapshot().unwrap();
    assert_eq!(bytes[4], 3);
    bytes[4] = 2;
    assert!(branching::Specimen::from_snapshot(&bytes).is_err());
    let mut wire = serde_json::to_value(f.skeleton.twigs).unwrap();
    for key in ["max_internodes", "max_droop", "curtain_step_clearance"] {
        wire.as_object_mut().unwrap().remove(key);
    }
    let decoded: telperion_core::twigs::TwigParams = serde_json::from_value(wire).unwrap();
    assert_eq!(decoded, f.skeleton.twigs);
}

#[test]
fn caller_budget_above_old_ceiling_grows_the_complete_beech() {
    let mut f = Preset::EuropeanBeech.parameters();
    f.skeleton.seed = 1;
    f.skeleton.habit.laterals_per_station = 3;
    f.skeleton.growth.max_nodes = Some(1_000_000);
    let started = std::time::Instant::now();
    let tree = branching::generate(&f.skeleton, f.radii).unwrap().tree;
    eprintln!(
        "higher-budget beech: nodes={}, capped={}, generation_ms={:.3}",
        tree.nodes.len(),
        tree.diagnostics.node_capped,
        started.elapsed().as_secs_f64() * 1000.
    );
    assert!(
        !tree.diagnostics.node_capped,
        "caller budget was overridden"
    );
    assert!(
        tree.nodes.len() > 250_000,
        "fixture must cross the old ceiling"
    );
    tree.validate().unwrap();
}

#[test]
#[cfg(target_pointer_width = "64")]
fn out_of_range_node_budget_names_the_field() {
    let mut f = Preset::Ordinary.parameters();
    f.skeleton.growth.max_nodes = Some(u32::MAX as usize + 1);
    let error = f.skeleton.resolved_growth(0).unwrap_err().to_string();
    assert!(error.contains("maxNodes"), "{error}");
}
