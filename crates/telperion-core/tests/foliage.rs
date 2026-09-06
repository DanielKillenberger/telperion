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
        anatomy: None,
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

fn species_element(anatomy: ElementAnatomy) -> ElementParams {
    ElementParams {
        anatomy,
        length: if anatomy == ElementAnatomy::FourSidedNeedle {
            0.02
        } else {
            0.1
        },
        width: if anatomy == ElementAnatomy::FourSidedNeedle {
            0.0015
        } else {
            0.07
        },
        connector_length: if anatomy == ElementAnatomy::FourSidedNeedle {
            0.0007
        } else {
            0.012
        },
        cup: 0.08,
        curl: 0.02,
        ..ElementParams::default()
    }
}

#[test]
fn profile_blades_have_rounded_lobes_and_needles_have_four_sides() {
    for shape in [ElementAnatomy::LobedBlade, ElementAnatomy::FourSidedNeedle] {
        let params = species_element(shape);
        let e = build_element(params).unwrap();
        assert_eq!(e, build_element(params).unwrap());
        let a = e.anatomy.as_ref().expect("explicit biological subset");
        assert!(a.vertices.end < e.positions.len(), "connector excluded");
        let first = &e.positions[a.sections[0].clone()];
        assert!(first
            .iter()
            .all(|v| (v.y - params.connector_length).abs() < 1e-8));
        let tip = e.positions[a.sections.last().unwrap().start];
        assert!((tip.y - params.connector_length - params.length).abs() < 1e-8);
        if shape == ElementAnatomy::LobedBlade {
            assert_eq!(a.unit, FoliageUnit::Leaf);
            let widths: Vec<_> = a
                .sections
                .iter()
                .map(|r| {
                    let row = &e.positions[r.clone()];
                    row.iter().map(|v| v.x).fold(f64::NEG_INFINITY, f64::max)
                        - row.iter().map(|v| v.x).fold(f64::INFINITY, f64::min)
                })
                .collect();
            let lobes = widths
                .windows(3)
                .filter(|w| w[1] > w[0] && w[1] > w[2])
                .count();
            assert!(lobes >= 3, "rounded lateral lobes, got {lobes}");
            let last = &e.positions[a.sections[a.sections.len() - 2].clone()];
            let half = last.last().unwrap().x;
            assert!(
                half > (tip.y - last[0].y),
                "rounded terminal lobe rather than bristle"
            );
        } else {
            assert_eq!(a.unit, FoliageUnit::Needle);
            assert_eq!(first.len(), 4);
            assert!(first.iter().any(|v| v.z > 0.) && first.iter().any(|v| v.z < 0.));
            let mut edges = std::collections::BTreeMap::new();
            for tri in e.indices[a.indices.clone()].as_chunks::<3>().0.iter() {
                for (u, v) in [(tri[0], tri[1]), (tri[1], tri[2]), (tri[2], tri[0])] {
                    *edges.entry((u.min(v), u.max(v))).or_insert(0) += 1;
                }
            }
            assert!(edges.values().all(|n| *n == 2), "closed needle surface");
        }
        for tri in e.indices.as_chunks::<3>().0.iter() {
            let [a, b, c] = [tri[0], tri[1], tri[2]].map(|v| e.positions[v as usize]);
            assert!((b - a).cross(c - a).length() > 0.);
        }
    }
}

#[test]
fn species_attachment_uses_local_twig_and_connected_origins() {
    let mut t = twig(0.08);
    t.nodes[1].position = t.nodes[0].position + Vec3::new(0.08, 0., 0.);
    let stations = TwigPlacement {
        internode_length: 0.01,
        stations_per_internode: 1,
    };
    for (attachment, shape) in [
        (Attachment::Alternate, ElementAnatomy::LobedBlade),
        (Attachment::RadialNeedles, ElementAnatomy::FourSidedNeedle),
    ] {
        let p = CanopyParams {
            attachment,
            divergence: if attachment == Attachment::Alternate {
                180.
            } else {
                90.
            },
            ..bare()
        };
        let instances = place(&t, Envelope::default(), 4, p, Some(stations)).unwrap();
        assert_eq!(instances.matrices.len(), 8);
        assert_eq!(
            instances,
            place(&t, Envelope::default(), 4, p, Some(stations)).unwrap()
        );
        let e = build_element(species_element(shape)).unwrap();
        let mut upper = 0;
        let mut lower = 0;
        for (k, m) in instances.matrices.iter().enumerate() {
            let root = transform_point(m, Vec3::ZERO);
            let center = t.nodes[0].position + Vec3::new(k as f64 * 0.01, 0., 0.);
            let radial = (root - center).normalized();
            assert!(((root - center).length() - 0.0025).abs() < 1e-6);
            let axis = Vec3::new(m[4] as f64, m[5] as f64, m[6] as f64);
            if attachment == Attachment::Alternate {
                if k > 0 {
                    let prev = &instances.matrices[k - 1];
                    let prev_radial =
                        Vec3::new(0., prev[13] as f64 - 10., prev[14] as f64).normalized();
                    assert!(radial.dot(prev_radial) < -0.999);
                }
            } else if radial.y > 0.5 {
                upper += 1;
                assert!(axis.x > 0.3, "upper needles lean toward local twig tip");
            } else if radial.y < -0.5 {
                lower += 1;
                assert!(axis.x < 0.15, "lower needles spread");
            }
            assert!(
                e.positions.iter().any(|v| v.length() < 1e-12),
                "connector starts at attachment"
            );
        }
        if attachment == Attachment::RadialNeedles {
            assert!(upper > 0 && lower > 0);
        }
    }
}

#[test]
fn species_empty_degenerate_and_invalid_controls_are_explicit() {
    for shape in [ElementAnatomy::LobedBlade, ElementAnatomy::FourSidedNeedle] {
        for bad in [0., -1., f64::NAN, f64::INFINITY] {
            assert!(build_element(ElementParams {
                connector_length: bad,
                ..species_element(shape)
            })
            .is_err());
        }
        assert!(build_element(ElementParams {
            card: true,
            ..species_element(shape)
        })
        .is_err());
    }
    for attachment in [Attachment::Alternate, Attachment::RadialNeedles] {
        let p = CanopyParams {
            attachment,
            ..bare()
        };
        assert!(place(&twig(0.04), Envelope::default(), 1, p, None).is_err());
        assert!(place(
            &twig(0.04),
            Envelope::default(),
            1,
            p,
            Some(TwigPlacement {
                stations_per_internode: 2,
                ..TwigPlacement::default()
            })
        )
        .is_err());
        let empty = place(
            &twig(0.),
            Envelope::default(),
            1,
            p,
            Some(TwigPlacement::default()),
        )
        .unwrap();
        assert!(empty.matrices.is_empty());
        let zero = place(
            &twig(0.04),
            Envelope::default(),
            1,
            CanopyParams { size: 0., ..p },
            Some(TwigPlacement::default()),
        )
        .unwrap();
        assert!(zero.matrices.is_empty());
    }
    let instances = Instances {
        matrices: vec![[
            1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1., 0., 0., 10., 0., 1.,
        ]],
    };
    assert!(
        cull(&instances, &Element::default(), Envelope::default(), 1.)
            .unwrap()
            .matrices
            .is_empty()
    );
}

#[test]
fn alternate_stations_continue_across_subdivided_twig_runs() {
    let mut t = twig(0.03);
    let mut middle = t.nodes[1].clone();
    middle.position.y = 10.01;
    t.nodes.insert(1, middle);
    t.nodes[2].parent = Some(1);
    let p = CanopyParams {
        attachment: Attachment::Alternate,
        divergence: 180.,
        ..bare()
    };
    let stations = Some(TwigPlacement {
        internode_length: 0.01,
        stations_per_internode: 1,
    });
    let split = place(&t, Envelope::default(), 3, p, stations).unwrap();
    let whole = place(&twig(0.03), Envelope::default(), 3, p, stations).unwrap();
    assert_eq!(split, whole);
}
