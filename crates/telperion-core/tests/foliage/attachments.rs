use super::*;

#[test]
fn species_attachment_uses_local_twig_and_connected_origins() {
    let mut t = twig(0.08);
    t.nodes[1].position = t.nodes[0].position + Vec3::new(0.08, 0., 0.);
    let stations = TwigPlacement {
        internode_length: 0.01,
        stations_per_internode: 1,
    };
    for (p, shape) in [
        (
            CanopyParams {
                forward_lean: 0.25,
                divergence: 180.,
                ..bare()
            },
            lobed_blade(),
        ),
        (
            CanopyParams {
                forward_lean: 0.05,
                lean_rise: 1.2,
                divergence: 90.,
                ..bare()
            },
            four_sided_needle(),
        ),
    ] {
        let instances = place(&t, Envelope::default(), 4, p, Some(stations)).unwrap();
        assert_eq!(instances.matrices.len(), 8);
        assert_eq!(
            instances,
            place(&t, Envelope::default(), 4, p, Some(stations)).unwrap()
        );
        let e = build_element(shape).unwrap();
        let mut upper = 0;
        let mut lower = 0;
        for (k, m) in instances.matrices.iter().enumerate() {
            let root = transform_point(m, Vec3::ZERO);
            let center = t.nodes[0].position + Vec3::new(k as f64 * 0.01, 0., 0.);
            let radial = (root - center).normalized();
            assert!(((root - center).length() - 0.0025).abs() < 1e-6);
            let axis = Vec3::new(m[4] as f64, m[5] as f64, m[6] as f64);
            if p.lean_rise == 0. {
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
        if p.lean_rise > 0. {
            assert!(upper > 0 && lower > 0);
        }
    }
}

#[test]
fn a_signed_lean_turns_the_leaf_back_down_its_shoot_and_toward_the_ground() {
    // The four orientation rows are signed: the same numbers that lean a blade
    // toward the tip of its shoot and toward the sky lean it, negative, back
    // along the shoot and toward the ground. A weeping shoot needs both.
    let mut t = twig(0.08);
    t.nodes[1].position = t.nodes[0].position + Vec3::new(0.08, 0., 0.);
    let stations = TwigPlacement {
        internode_length: 0.01,
        stations_per_internode: 1,
    };
    let axes = |p| {
        place(&t, Envelope::default(), 4, p, Some(stations))
            .unwrap()
            .matrices
            .iter()
            .map(|m| Vec3::new(m[4] as f64, m[5] as f64, m[6] as f64))
            .collect::<Vec<_>>()
    };
    // The shoot runs along +X, so forward lean is the axis's own x.
    let forward = axes(CanopyParams {
        forward_lean: 0.6,
        ..bare()
    });
    let back = axes(CanopyParams {
        forward_lean: -0.6,
        ..bare()
    });
    for (ahead, behind) in forward.iter().zip(&back) {
        assert!(ahead.x > 0.4, "a positive lean did not reach the tip");
        assert!(behind.x < -0.4, "a negative lean did not reach back");
    }
    // Upward is the axis's own y, and downward is the same row signed: every
    // station turns toward the ground where it turned toward the sky.
    let sky = axes(CanopyParams {
        upward: 0.8,
        ..bare()
    });
    let down = axes(CanopyParams {
        upward: -0.8,
        ..bare()
    });
    let mean = |axes: &[Vec3]| axes.iter().map(|a| a.y).sum::<f64>() / axes.len() as f64;
    assert!(
        mean(&down) < mean(&sky) - 0.4,
        "a negative upward left the crown where a positive one put it"
    );
    assert!(
        down.iter().any(|a| a.y < -0.5),
        "no leaf turned toward the ground"
    );
    let outward = axes(CanopyParams {
        outward: -0.9,
        ..bare()
    });
    assert!(
        outward
            .iter()
            .zip(&down)
            .any(|(inward, other)| inward != other),
        "a negative outward moved nothing"
    );
}

#[test]
fn species_empty_degenerate_and_invalid_controls_are_explicit() {
    for shape in [lobed_blade(), four_sided_needle()] {
        for bad in [0., -1., f64::NAN, f64::INFINITY, shape.length * 1.01] {
            assert!(build_element(ElementParams {
                connector_length: bad,
                ..shape
            })
            .is_err());
        }
        // A card is the authored extent and nothing else: it carries neither
        // a lobed margin nor a rounded section, and says so by name.
        assert_eq!(
            build_element(ElementParams {
                card: true,
                ..shape
            })
            .err(),
            Some(Error::InvalidInput(
                "leaf card carries no lobes and no section roundness"
            ))
        );
    }
    // Lean and contact are ranged numbers, and each is refused by its own name.
    for (bad, message) in [
        (
            CanopyParams {
                forward_lean: 1.5,
                ..bare()
            },
            "forward lean",
        ),
        (
            CanopyParams {
                lean_rise: -2.5,
                ..bare()
            },
            "lean rise",
        ),
        (
            CanopyParams {
                forward_lean: -1.5,
                ..bare()
            },
            "forward lean",
        ),
        (
            CanopyParams {
                outward: -1.5,
                ..bare()
            },
            "outward",
        ),
        (
            CanopyParams {
                surface_contact: 1.5,
                ..bare()
            },
            "surface contact",
        ),
    ] {
        assert_eq!(
            place(
                &twig(0.04),
                Envelope::default(),
                1,
                bad,
                Some(TwigPlacement::default())
            )
            .err(),
            Some(Error::InvalidInput(message))
        );
    }
    let p = CanopyParams {
        forward_lean: 0.25,
        surface_contact: 0.5,
        ..bare()
    };
    // Stations to an internode is a number, not a mode: a leaf that leans and
    // seats takes several of them as readily as one.
    assert_eq!(
        place(
            &twig(0.04),
            Envelope::default(),
            1,
            p,
            Some(TwigPlacement {
                stations_per_internode: 2,
                ..TwigPlacement::default()
            })
        )
        .unwrap()
        .matrices
        .len(),
        4
    );
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
fn leaned_stations_continue_across_subdivided_twig_runs() {
    let mut t = twig(0.03);
    let mut middle = t.nodes[1].clone();
    middle.position.y = 10.01;
    t.nodes.insert(1, middle);
    t.nodes[2].parent = Some(1);
    let p = CanopyParams {
        forward_lean: 0.25,
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

#[test]
fn shoot_radius_alone_clothes_slender_supports_and_spares_thick_limbs() {
    let mut t = twig(0.08);
    t.nodes[1].kind = NodeKind::Branch;
    t.crossover = t.nodes.len();
    let stations = Some(TwigPlacement::default());
    let clothing = CanopyParams {
        shoot_radius: 0.005,
        ..bare()
    };
    let placed = place(&t, Envelope::default(), 1, clothing, stations).unwrap();
    assert_eq!(placed.matrices.len(), 4);
    // The same wood, the same leaf: at zero the trait clothes nothing beyond
    // the runs the twig layer marked, and this branch is not one of them.
    assert!(place(
        &t,
        Envelope::default(),
        1,
        CanopyParams {
            shoot_radius: 0.,
            ..clothing
        },
        stations
    )
    .unwrap()
    .matrices
    .is_empty());
    t.nodes[1].start_radius = 0.006;
    assert!(place(&t, Envelope::default(), 1, clothing, stations)
        .unwrap()
        .matrices
        .is_empty());
}

#[test]
fn every_attachment_trait_moves_every_shipped_preset() {
    for preset in [
        Preset::Ordinary,
        Preset::OregonWhiteOak,
        Preset::NorwaySpruce,
        Preset::EuropeanBeech,
        Preset::SilverBirch,
        Preset::Telperion,
        Preset::Laurelin,
    ] {
        let family = preset.parameters();
        let tree = crate::specimens::tree(&family);
        let base = placement_hash(&family, &tree);
        let p = family.canopy;
        for (name, canopy) in [
            (
                "forward lean",
                CanopyParams {
                    forward_lean: step(p.forward_lean, 1.),
                    ..p
                },
            ),
            (
                "lean rise",
                CanopyParams {
                    lean_rise: step(p.lean_rise, 2.),
                    ..p
                },
            ),
            (
                "surface contact",
                CanopyParams {
                    surface_contact: step(p.surface_contact, 1.),
                    ..p
                },
            ),
        ] {
            let stepped = Family {
                canopy,
                ..family.clone()
            };
            assert_ne!(
                placement_hash(&stepped, &tree),
                base,
                "{preset:?}: {name} moves no leaf"
            );
        }
    }
}
