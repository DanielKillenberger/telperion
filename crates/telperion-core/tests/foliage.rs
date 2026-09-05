use telperion_core::{
    envelope::Envelope,
    foliage::*,
    math::Vec3,
    tree::{Node, NodeKind, Tree},
};
fn twig(length: f64) -> Tree {
    let mut root = Node::root();
    root.position = Vec3::new(0., 10., 0.);
    root.radius = 1.;
    root.start_radius = 1.;
    Tree {
        nodes: vec![
            root,
            Node {
                position: Vec3::new(0., 10. + length, 0.),
                parent: Some(0),
                radius: 0.0025,
                start_radius: 0.0025,
                base_radius: 0.0025,
                branch: 1,
                kind: NodeKind::Twig,
            },
        ],
        crossover: 1,
        ..Tree::default()
    }
}
fn bare() -> CanopyParams {
    CanopyParams {
        outward: 0.,
        upward: 0.,
        scatter: 0.,
        size_variation: 0.,
        divergence: 90.,
        ..CanopyParams::default()
    }
}
#[test]
fn twig_stations_and_owned_deterministic_frames() {
    let t = twig(0.04);
    let before = t.clone();
    let p = TwigPlacement {
        stations_per_internode: 2,
        ..TwigPlacement::default()
    };
    let a = place(&t, Envelope::default(), 7, bare(), Some(p)).unwrap();
    assert_eq!(a.matrices.len(), 4);
    assert_eq!(
        a,
        place(&t, Envelope::default(), 7, bare(), Some(p)).unwrap()
    );
    assert_eq!(t, before);
    for (i, m) in a.matrices.iter().enumerate() {
        let pos = transform_point(m, Vec3::ZERO);
        assert!((pos.y - 10. - (i / 2) as f64 * 0.02).abs() < 1e-6);
        assert!((pos.x.hypot(pos.z) - 0.0025).abs() < 1e-6);
        let x = Vec3::new(m[0] as f64, m[1] as f64, m[2] as f64);
        let y = Vec3::new(m[4] as f64, m[5] as f64, m[6] as f64);
        let z = Vec3::new(m[8] as f64, m[9] as f64, m[10] as f64);
        assert!((y.cross(z) - x).length() < 1e-6);
        assert!(x.dot(y).abs() < 1e-6);
        assert!((y.length() - 1.).abs() < 1e-6);
    }
}
#[test]
fn element_anatomy_and_bounds() {
    for card in [false, true] {
        let e = build_element(ElementParams {
            card,
            ..ElementParams::default()
        })
        .unwrap();
        for tri in e.indices.as_chunks::<3>().0 {
            let a = e.positions[tri[0] as usize];
            let b = e.positions[tri[1] as usize];
            let c = e.positions[tri[2] as usize];
            assert!((b - a).cross(c - a).z > 0.);
        }
        let a = place(
            &twig(0.25),
            Envelope::default(),
            3,
            CanopyParams::default(),
            Some(TwigPlacement::default()),
        )
        .unwrap();
        let b = a.bounds(&e).unwrap().unwrap();
        for m in &a.matrices {
            for v in &e.positions {
                let p = transform_point(m, *v);
                assert!(b.contains(p));
            }
        }
    }
}
#[test]
fn shell_membership_empty_and_invalid() {
    let e = build_element(ElementParams::default()).unwrap();
    let env = Envelope::default();
    let a = place(&twig(0.25), env, 3, bare(), Some(TwigPlacement::default())).unwrap();
    assert!(!a.matrices.is_empty());
    assert!(cull(&a, &e, env, 0.).unwrap().matrices.is_empty());
    assert_eq!(cull(&a, &e, env, 1.).unwrap(), a);
    assert_eq!(Instances::default().bounds(&e).unwrap(), None);
    assert!(place(&Tree::default(), env, 3, bare(), None)
        .unwrap()
        .matrices
        .is_empty());
    for bad in [f64::NAN, f64::INFINITY, -1., 1e300] {
        assert!(place(
            &twig(1.),
            env,
            1,
            CanopyParams {
                size: bad,
                ..bare()
            },
            None
        )
        .is_err());
        assert!(build_element(ElementParams {
            length: bad,
            ..ElementParams::default()
        })
        .is_err());
    }
    assert!(place(
        &twig(1.),
        env,
        1,
        CanopyParams {
            max_instances: 1,
            ..bare()
        },
        Some(TwigPlacement::default())
    )
    .is_err());
    assert!(place(
        &twig(1.),
        env,
        1,
        bare(),
        Some(TwigPlacement {
            stations_per_internode: 0,
            ..TwigPlacement::default()
        })
    )
    .is_err());
    let mut bad = twig(1.);
    bad.nodes[1].position.x = f64::MAX;
    assert!(place(&bad, env, 1, bare(), Some(TwigPlacement::default())).is_err());
}

#[test]
fn malformed_element_is_rejected_even_when_first_vertex_is_outside_shell() {
    let e = Element {
        positions: vec![Vec3::ZERO, Vec3::new(f64::MAX, 0., 0.)],
        indices: vec![],
    };
    let a = place(
        &twig(0.04),
        Envelope::default(),
        7,
        bare(),
        Some(TwigPlacement::default()),
    )
    .unwrap();
    assert!(cull(&a, &e, Envelope::default(), 1.).is_err());
}

#[test]
fn fallback_folds_zero_edges_and_clumps_at_terminal_tip() {
    let mut t = twig(4.);
    t.nodes[0].position = Vec3::new(5., 0., 0.);
    t.nodes[1].position = Vec3::new(5., 4., 0.);
    t.nodes[1].kind = NodeKind::Branch;
    let p = CanopyParams {
        shoot_radius: 1.,
        spacing: 0.02,
        clump: 0,
        ..bare()
    };
    let a = place(&t, Envelope::default(), 7, p, None).unwrap();
    assert_eq!(a.matrices.len(), 9);
    let mut doubled = t.clone();
    let mut duplicate = t.nodes[0].clone();
    duplicate.parent = Some(0);
    duplicate.branch = 1;
    doubled.nodes.insert(1, duplicate);
    doubled.nodes[2].parent = Some(1);
    doubled.nodes[2].branch = 2;
    assert_eq!(place(&doubled, Envelope::default(), 7, p, None).unwrap(), a);
    let clumped = place(
        &t,
        Envelope::default(),
        7,
        CanopyParams {
            clump: 4,
            clump_span: 0.25,
            ..p
        },
        None,
    )
    .unwrap();
    assert_eq!(clumped.matrices.len(), a.matrices.len() + 4);
    for m in &clumped.matrices[a.matrices.len()..] {
        assert!(m[13] >= 3. && m[13] <= 4.);
    }
    assert!(place(
        &t,
        Envelope::default(),
        7,
        CanopyParams {
            shoot_radius: 0.,
            ..p
        },
        None
    )
    .unwrap()
    .matrices
    .is_empty());
    assert!(place(
        &t,
        Envelope::default(),
        7,
        p,
        Some(TwigPlacement::default())
    )
    .unwrap()
    .matrices
    .is_empty());
    let mut crowded = twig(1.);
    crowded.nodes[1].position.y = 100.;
    assert!(place(
        &crowded,
        Envelope::default(),
        7,
        p,
        Some(TwigPlacement::default())
    )
    .is_err());
}

#[test]
fn shell_keeps_leaf_extent_and_crown_underside() {
    let env = Envelope::default();
    let small = build_element(ElementParams::default()).unwrap();
    let m = [
        0., 1., 0., 0., 1., 0., 0., 0., 0., 0., -1., 0., 0., 15., 0., 1.,
    ];
    let a = Instances { matrices: vec![m] };
    assert!(cull(&a, &small, env, 0.45).unwrap().matrices.is_empty());
    let long = build_element(ElementParams {
        length: 8.,
        ..ElementParams::default()
    })
    .unwrap();
    assert_eq!(cull(&a, &long, env, 0.45).unwrap(), a);
    let mut underside = m;
    underside[13] = (env.height * env.crown_base + 0.01) as f32;
    let underside = Instances {
        matrices: vec![underside],
    };
    assert_eq!(cull(&underside, &small, env, 0.45).unwrap(), underside);
    let mut invalid = m;
    invalid[15] = 0.;
    assert!(cull(
        &Instances {
            matrices: vec![invalid]
        },
        &small,
        env,
        0.45
    )
    .is_err());
}
