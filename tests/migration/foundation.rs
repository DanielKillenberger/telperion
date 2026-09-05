use telperion_core::{
    envelope::Envelope,
    math::Vec3,
    noise::Noise,
    rng::Rng,
    tree::{Node, Tree},
};

#[test]
fn seeded_stream_matches_reference() {
    let mut rng = Rng::new(0);
    let expected = [0.3921251413412392, 0.8505931859835982, 0.684420470148325];
    for value in expected {
        assert_eq!(rng.next_f64(), value);
    }
    let mut a = Rng::new(u32::MAX);
    let mut b = a.clone();
    for _ in 0..1000 {
        let x = a.next_f64();
        assert_eq!(x, b.next_f64());
        assert!((0.0..1.0).contains(&x));
    }
}

#[test]
fn envelope_boundaries_and_invalid_values() {
    let e = Envelope::default();
    assert_eq!(e.radius_at(0.0), 0.0);
    assert_eq!(e.radius_at(e.height), 0.0);
    assert!(e.contains(Vec3::ZERO, 0.0));
    assert!(!e.contains(Vec3::new(0.0, -1.0, 0.0), 0.0));
    assert!(!e.contains(Vec3::new(0.0, e.height + 1.0, 0.0), 0.0));
    assert!(!e.contains(Vec3::new(0.01, 1.0, 0.0), 0.0));
    for bad in [f64::NAN, f64::INFINITY, -1.0] {
        assert!(Envelope { height: bad, ..e }.validate().is_err());
    }
    assert!(Envelope { spread: 0.0, ..e }
        .sample(10, &mut Rng::new(1))
        .unwrap()
        .is_empty());
    let points = e.sample(100, &mut Rng::new(1)).unwrap();
    assert_eq!(points.len(), 100);
    assert!(points.iter().all(|&p| e.contains(p, 1e-12)));
    let y = e.height * (e.crown_base + (1.0 - e.crown_base) * e.fullness);
    assert!(e.distance_to_profile(e.radius_at(y), y) < 0.01);
}

#[test]
fn vector_and_noise_boundaries() {
    assert_eq!(Vec3::ZERO.normalized(), Vec3::ZERO);
    assert_eq!(Vec3::X.cross(Vec3::Y), Vec3::Z);
    let noise = Noise::new(123);
    assert_eq!(noise.at(Vec3::ZERO), 0.0);
    let p = Vec3::new(-0.2, 0.7, 1.3);
    assert_eq!(noise.at(p), Noise::new(123).at(p));
    assert!((noise.at(p) - noise.at(p + Vec3::X * 1e-7)).abs() < 1e-5);
    assert!(noise.curl(p, 1.0).is_finite());
}

#[test]
fn tree_validation_rejects_bad_topology_and_nonfinite_storage() {
    let mut tree = Tree::default();
    assert!(tree.validate().is_ok());
    tree.nodes.push(Node::root());
    assert!(tree.validate().is_ok());
    tree.nodes[0].parent = Some(0);
    assert!(tree.validate().is_err());
    tree.nodes[0].parent = None;
    tree.nodes[0].position.x = f64::NAN;
    assert!(tree.validate().is_err());
}

#[test]
fn bias_and_sampling_limits_are_explicit() {
    use telperion_core::{
        bias::{BiasParams, GrowthBias},
        Error,
    };
    let e = Envelope::default();
    assert!(matches!(
        e.sample(1_000_001, &mut Rng::new(0)),
        Err(Error::ResourceLimit(_))
    ));
    assert!(GrowthBias::new(
        e,
        0,
        BiasParams {
            lean: f64::NAN,
            ..BiasParams::default()
        }
    )
    .is_err());
    let none = GrowthBias::new(e, 0, BiasParams::NONE).unwrap();
    assert_eq!(none.apply(Vec3::ZERO, Vec3::Y, 0.5), Vec3::Y);
    let bias = GrowthBias::new(e, 0, BiasParams::default()).unwrap();
    for step in [0.01, 0.5, 10.0] {
        let direction = bias.apply(Vec3::ZERO, Vec3::Y, step);
        assert!(direction.is_finite() && direction.y > 0.0);
        assert!((direction.length() - 1.0).abs() < 1e-12);
    }
    assert!((Vec3::X.rotate(Vec3::Y, std::f64::consts::FRAC_PI_2) + Vec3::Z).length() < 1e-12);
    assert_eq!(Vec3::X.distance_squared(Vec3::Y), 2.0);
}

#[test]
fn noise_seed_range_flow_and_feature_scale() {
    let noise = Noise::new(1);
    let other = Noise::new(2);
    let mut seed_difference: f64 = 0.0;
    let roughness = |spacing: f64| {
        (1..400)
            .map(|i| {
                (noise.at(Vec3::new(i as f64 * spacing, 7.5, -3.25))
                    - noise.at(Vec3::new((i - 1) as f64 * spacing, 7.5, -3.25)))
                .abs()
            })
            .sum::<f64>()
            / 399.0
    };
    for i in 0..2000 {
        let p = Vec3::new(i as f64 * 0.137, i as f64 * 0.311, i as f64 * 0.079);
        assert!(noise.fbm(p).abs() <= 1.0);
        seed_difference = seed_difference.max((noise.at(p) - other.at(p)).abs());
    }
    assert!(seed_difference > 0.1);
    assert!(roughness(0.01) < roughness(1.0) / 10.0);
    assert!(
        (noise.at(Vec3::new(3.0 - 1e-6, 0.5, 0.5)) - noise.at(Vec3::new(3.0 + 1e-6, 0.5, 0.5)))
            .abs()
            < 1e-4
    );
    let wavelength = 4.0;
    let h = 1e-3;
    for p in [
        Vec3::new(0.5, 1.5, 2.5),
        Vec3::new(-6.25, 11.75, 3.125),
        Vec3::new(17.5, -2.5, 8.75),
    ] {
        let dx = (noise.curl(p + Vec3::X * h, wavelength).x
            - noise.curl(p - Vec3::X * h, wavelength).x)
            / (2.0 * h);
        let dy = (noise.curl(p + Vec3::Y * h, wavelength).y
            - noise.curl(p - Vec3::Y * h, wavelength).y)
            / (2.0 * h);
        let dz = (noise.curl(p + Vec3::Z * h, wavelength).z
            - noise.curl(p - Vec3::Z * h, wavelength).z)
            / (2.0 * h);
        assert!((dx + dy + dz).abs() < noise.curl(p, wavelength).length() / wavelength * 0.05);
    }
    let alignment = |wave| {
        noise
            .curl(Vec3::ZERO, wave)
            .normalized()
            .dot(noise.curl(Vec3::X, wave).normalized())
    };
    assert!(alignment(40.0) > 0.9);
    assert!(alignment(40.0) > alignment(0.5));
}

#[test]
fn bias_preserves_scale_sampling_and_upward_progress() {
    use telperion_core::bias::{BiasParams, GrowthBias, MIN_STEPS_PER_BEND};
    let e = Envelope::default();
    let step = e.height * 0.022;
    let floor = MIN_STEPS_PER_BEND * step / e.height;
    let field = |params| GrowthBias::new(e, 1, params).unwrap();
    let defaults = BiasParams::default();
    let short = field(BiasParams {
        writhe_wavelength: 0.001,
        ..defaults
    });
    let sampled = field(BiasParams {
        writhe_wavelength: floor,
        ..defaults
    });
    let fast = field(BiasParams {
        spiral_rate: 40.0,
        ..defaults
    });
    let limited = field(BiasParams {
        spiral_rate: 1.0 / floor,
        ..defaults
    });
    for t in [0.1, 0.35, 0.6, 0.9] {
        let p = Vec3::new(0.05 * e.height, t * e.height, 0.0);
        assert!((short.apply(p, Vec3::Y, step) - sampled.apply(p, Vec3::Y, step)).length() < 1e-12);
        assert!((fast.apply(p, Vec3::Y, step) - limited.apply(p, Vec3::Y, step)).length() < 1e-12);
        for height in [4.0, 24.0, 60.0, 150.0, 400.0] {
            let scaled = GrowthBias::new(Envelope { height, ..e }, 1, defaults).unwrap();
            let direction = scaled.apply(p * (height / e.height), Vec3::Y, height * 0.022);
            assert!((direction - field(defaults).apply(p, Vec3::Y, step)).length() < 1e-9);
        }
        for seed in 1..=5 {
            let extreme = GrowthBias::new(
                e,
                seed,
                BiasParams {
                    gravitropism: 3.0,
                    lean: 2.0,
                    writhe_amplitude: 3.0,
                    writhe_wavelength: 0.02,
                    spiral_rate: 40.0,
                },
            )
            .unwrap();
            let direction = extreme.apply(p, Vec3::Y, step);
            assert!(direction.y > 0.0);
            assert!((direction.length() - 1.0).abs() < 1e-12);
        }
    }
    let p = Vec3::new(3.0, 12.0, 2.0);
    let longer = field(BiasParams {
        writhe_wavelength: floor * 2.0,
        ..defaults
    });
    let slower = field(BiasParams {
        spiral_rate: 0.5 / floor,
        ..defaults
    });
    assert!((longer.apply(p, Vec3::Y, step) - sampled.apply(p, Vec3::Y, step)).length() > 1e-6);
    assert!((slower.apply(p, Vec3::Y, step) - limited.apply(p, Vec3::Y, step)).length() > 1e-6);
}

#[test]
fn envelope_profile_has_authored_maximum_and_continuous_shoulder() {
    for fullness in [0.1, 0.45, 0.9] {
        let e = Envelope {
            fullness,
            ..Envelope::default()
        };
        let widest = e.height * (e.crown_base + (1.0 - e.crown_base) * fullness);
        assert!((e.radius_at(widest) - e.max_radius()).abs() < 1e-12);
        for [radius, y] in e.profile() {
            assert!((0.0..=e.max_radius()).contains(&radius));
            assert!((e.height * e.crown_base..=e.height).contains(&y));
        }
        assert!((e.radius_at(widest - 1e-6) - e.radius_at(widest + 1e-6)).abs() < 1e-5);
    }
}
