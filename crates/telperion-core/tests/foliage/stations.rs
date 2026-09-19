use super::*;

#[test]
fn a_shoot_can_carry_more_than_512_authored_stations() {
    let t = twig(12.);
    let p = CanopyParams {
        max_instances: 600,
        ..bare()
    };
    let leaves = place(
        &t,
        Envelope::default(),
        7,
        p,
        Some(TwigPlacement::default()),
        twig_box(12.),
    )
    .unwrap();
    assert_eq!(leaves.len(), 600);
    let error = place(
        &t,
        Envelope::default(),
        7,
        CanopyParams {
            max_instances: 599,
            ..p
        },
        Some(TwigPlacement::default()),
        twig_box(12.),
    )
    .unwrap_err();
    assert!(error.to_string().contains("foliage instance budget"));
}

#[test]
fn twig_stations_and_owned_deterministic_frames() {
    let t = twig(0.04);
    let before = t.clone();
    let p = TwigPlacement {
        stations_per_internode: 2,
        ..TwigPlacement::default()
    };
    let box_of = twig_box(0.04);
    let a = place(&t, Envelope::default(), 7, bare(), Some(p), box_of).unwrap();
    assert_eq!(a.len(), 4);
    assert_eq!(
        a,
        place(&t, Envelope::default(), 7, bare(), Some(p), box_of).unwrap()
    );
    assert_eq!(t, before);
    for (i, m) in a.matrices().enumerate() {
        let pos = transform_point(&m, Vec3::ZERO);
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
        // The blade faces +Z at roundness 0; the connector's sides face out.
        let blade = e
            .anatomy
            .as_ref()
            .map_or(0..e.indices.len(), |a| a.indices.clone());
        for tri in e.indices[blade].as_chunks::<3>().0 {
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
            twig_box(0.25),
        )
        .unwrap();
        let b = a.bounds(&e).unwrap().unwrap();
        for m in a.matrices() {
            for v in &e.positions {
                let p = transform_point(&m, *v);
                assert!(b.contains(p));
            }
        }
    }
}
#[test]
fn shell_membership_empty_and_invalid() {
    let e = build_element(ElementParams::default()).unwrap();
    let env = Envelope::default();
    let a = place(
        &twig(0.25),
        env,
        3,
        bare(),
        Some(TwigPlacement::default()),
        twig_box(0.25),
    )
    .unwrap();
    assert!(!a.is_empty());
    assert!(cull(a.clone(), &e, env, 0.).unwrap().is_empty());
    assert_eq!(cull(a.clone(), &e, env, 1.).unwrap(), a);
    assert_eq!(Instances::default().bounds(&e).unwrap(), None);
    assert!(place(&Tree::default(), env, 3, bare(), None, twig_box(0.))
        .unwrap()
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
            None,
            twig_box(1.)
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
        Some(TwigPlacement::default()),
        twig_box(1.)
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
        }),
        twig_box(1.)
    )
    .is_err());
    let mut bad = twig(1.);
    bad.nodes[1].position.x = f64::MAX;
    assert!(place(
        &bad,
        env,
        1,
        bare(),
        Some(TwigPlacement::default()),
        twig_box(1.)
    )
    .is_err());
    // A box that is not a box is refused by name, whatever it holds.
    let mut warped = a.clone();
    warped.reference.extent.x = f64::NAN;
    assert_eq!(
        warped.validate().err(),
        Some(Error::InvalidInput("foliage reference box"))
    );
    assert_eq!(
        cull(warped, &e, env, 1.).err(),
        Some(Error::InvalidInput("foliage reference box"))
    );
    let mut reversed = a.clone();
    reversed.reference.extent.y = -1.;
    assert_eq!(
        cull(reversed, &e, env, 1.).err(),
        Some(Error::InvalidInput("foliage reference box"))
    );
}

#[test]
fn malformed_element_is_rejected_even_when_first_vertex_is_outside_shell() {
    let e = Element {
        positions: vec![Vec3::ZERO, Vec3::new(f64::MAX, 0., 0.)],
        indices: vec![],
        anatomy: None,
        ..Element::default()
    };
    let a = place(
        &twig(0.04),
        Envelope::default(),
        7,
        bare(),
        Some(TwigPlacement::default()),
        twig_box(0.04),
    )
    .unwrap();
    assert!(cull(a, &e, Envelope::default(), 1.).is_err());
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
    // The stem stands at x = 5 and runs to y = 4, and a station stands off it
    // by the wood's own radius, a metre at the root. The box's y grid lands
    // on 4 exactly, so a leaf at the tip stays at the tip.
    let box_of = Reference::spanning(Vec3::new(3., -1., -2.), Vec3::new(7., 4., 2.));
    let a = place(&t, Envelope::default(), 7, p, None, box_of).unwrap();
    assert_eq!(a.len(), 9);
    let mut doubled = t.clone();
    let mut duplicate = t.nodes[0].clone();
    duplicate.parent = Some(0);
    duplicate.branch = 1;
    doubled.nodes.insert(1, duplicate);
    doubled.nodes[2].parent = Some(1);
    doubled.nodes[2].branch = 2;
    assert_eq!(
        place(&doubled, Envelope::default(), 7, p, None, box_of).unwrap(),
        a
    );
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
        box_of,
    )
    .unwrap();
    assert_eq!(clumped.len(), a.len() + 4);
    for i in a.len()..clumped.len() {
        let y = clumped.position(i).y;
        assert!((3. ..=4.).contains(&y));
    }
    assert!(place(
        &t,
        Envelope::default(),
        7,
        CanopyParams {
            shoot_radius: 0.,
            ..p
        },
        None,
        box_of
    )
    .unwrap()
    .is_empty());
    assert!(place(
        &t,
        Envelope::default(),
        7,
        CanopyParams {
            shoot_radius: 0.,
            ..p
        },
        Some(TwigPlacement::default()),
        box_of
    )
    .unwrap()
    .is_empty());
    let mut crowded = twig(1.);
    crowded.nodes[1].position.y = 100.;
    assert!(place(
        &crowded,
        Envelope::default(),
        7,
        CanopyParams {
            max_instances: 512,
            ..p
        },
        Some(TwigPlacement::default()),
        Reference::spanning(Vec3::new(-1., 9., -1.), Vec3::new(1., 101., 1.))
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
    // The crown the two leaves below stand in: the deep one at 15 m and the
    // one just under the crown's base at 7.21 m.
    let box_of = Reference::spanning(Vec3::new(-1., -1., -1.), Vec3::new(1., 16., 1.));
    let mut a = Instances::new(box_of);
    a.push(&m);
    assert!(cull(a.clone(), &small, env, 0.45).unwrap().is_empty());
    let long = build_element(ElementParams {
        length: 8.,
        ..ElementParams::default()
    })
    .unwrap();
    assert_eq!(cull(a.clone(), &long, env, 0.45).unwrap(), a);
    let mut under = m;
    under[13] = (env.height * env.crown_base + 0.01) as f32;
    let mut underside = Instances::new(box_of);
    underside.push(&under);
    assert_eq!(
        cull(underside.clone(), &small, env, 0.45).unwrap(),
        underside
    );
    // A leaf cannot be malformed any more - three words are three words - so
    // what the cull refuses is a box that is not a box.
    let mut invalid = underside.clone();
    invalid.reference.min.y = f64::NAN;
    assert_eq!(
        cull(invalid, &small, env, 0.45).err(),
        Some(Error::InvalidInput("foliage reference box"))
    );
}

/// One copy of the crown, whatever the cull drops. The buffer handed in is the
/// buffer handed back: a second one at the input's own length is 470 MB on the
/// spruce, resident for as long as the first.
#[test]
fn cull_retains_in_the_buffer_it_was_given() {
    let e = build_element(ElementParams::default()).unwrap();
    let env = Envelope::default();

    // Deep inside the crown, and just under its base: the first is dropped at
    // this shell and the second stays, so survivors have to move down inside
    // the block the caller's vector already owned.
    let deep = [
        0., 1., 0., 0., 1., 0., 0., 0., 0., 0., -1., 0., 0., 15., 0., 1.,
    ];
    let mut shell = deep;
    shell[13] = (env.height * env.crown_base + 0.01) as f32;
    let box_of = Reference::spanning(Vec3::new(-1., -1., -1.), Vec3::new(1., 16., 1.));
    let mut mixed = Instances::new(box_of);
    mixed.push(&deep);
    mixed.push(&shell);
    let survivor = mixed.leaves[1];
    let (before, capacity) = (mixed.leaves.as_ptr(), mixed.leaves.capacity());
    let kept = cull(mixed, &e, env, 0.45).unwrap();
    assert_eq!(kept.leaves, vec![survivor], "the wrong leaf survived");
    assert_eq!(kept.leaves.as_ptr(), before, "kept leaves moved buffer");
    assert_eq!(kept.leaves.capacity(), capacity, "capacity was shrunk");

    // A cull that drops every leaf still hands the original allocation back.
    let a = place(
        &twig(0.25),
        env,
        3,
        bare(),
        Some(TwigPlacement::default()),
        twig_box(0.25),
    )
    .unwrap();
    assert!(!a.is_empty());
    let (before, capacity) = (a.leaves.as_ptr(), a.leaves.capacity());
    let empty = cull(a, &e, env, 0.).unwrap();
    assert!(empty.is_empty());
    assert_eq!(empty.leaves.as_ptr(), before, "empty crown moved buffer");
    assert_eq!(empty.leaves.capacity(), capacity, "capacity was shrunk");
}
