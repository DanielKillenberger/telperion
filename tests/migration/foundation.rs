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
