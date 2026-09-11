use std::collections::BTreeSet;
use std::ops::Range;
use telperion_core::{
    branching,
    envelope::Envelope,
    foliage::*,
    math::Vec3,
    presets::{Family, Preset},
    tree::{Node, NodeKind, Tree},
    Error,
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
        ..Element::default()
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
        CanopyParams {
            shoot_radius: 0.,
            ..p
        },
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

/// The two ends of the element's trait space, as rows and nothing else: a
/// deeply lobed blade and a four-sided shaft, built by the one routine.
fn lobed_blade() -> ElementParams {
    ElementParams {
        length: 0.1,
        width: 0.07,
        connector_length: 0.012,
        cup: 0.08,
        curl: 0.02,
        widest_at: 0.55,
        base_fullness: 0.6,
        tip_sharpness: 0.6,
        lobe_count: 5,
        lobe_depth: 0.7,
        axial_segments: 20,
        ..ElementParams::default()
    }
}
fn four_sided_needle() -> ElementParams {
    ElementParams {
        length: 0.02,
        width: 0.0015,
        connector_length: 0.0007,
        cup: 0.08,
        curl: 0.02,
        widest_at: 0.2,
        base_fullness: 0.2,
        tip_sharpness: 0.2,
        section_roundness: 1.0,
        cross_segments: 4,
        ..ElementParams::default()
    }
}

#[test]
fn one_routine_builds_a_lobed_margin_and_a_four_sided_shaft() {
    for (name, params) in [("blade", lobed_blade()), ("needle", four_sided_needle())] {
        let e = build_element(params).unwrap();
        assert_eq!(e, build_element(params).unwrap(), "{name}: repeatable");
        let a = e
            .anatomy
            .as_ref()
            .expect("every element publishes its sections");
        assert!(
            a.vertices.end < e.positions.len(),
            "{name}: connector excluded"
        );
        let first = &e.positions[a.sections[0].clone()];
        assert!(first
            .iter()
            .all(|v| (v.y - params.connector_length).abs() < 1e-8));
        let tip = e.positions[a.sections.last().unwrap().start];
        assert!((tip.y - params.connector_length - params.length).abs() < 1e-8);
        for tri in e.indices.as_chunks::<3>().0.iter() {
            let [a, b, c] = [tri[0], tri[1], tri[2]].map(|v| e.positions[v as usize]);
            assert!((b - a).cross(c - a).length() > 0., "{name}: real triangle");
        }

        let extent = |r: &Range<usize>, f: fn(Vec3) -> f64| {
            let row = &e.positions[r.clone()];
            row.iter().map(|v| f(*v)).fold(f64::NEG_INFINITY, f64::max)
                - row.iter().map(|v| f(*v)).fold(f64::INFINITY, f64::min)
        };
        let widths: Vec<_> = a.sections.iter().map(|r| extent(r, |v| v.x)).collect();
        if params.section_roundness < 0.5 {
            assert_eq!(a.unit, FoliageUnit::Leaf);
            // One crest per lobe: the margin rises and falls five times.
            let crests = widths
                .windows(3)
                .filter(|w| w[1] > w[0] && w[1] > w[2])
                .count();
            assert_eq!(
                crests, params.lobe_count as usize,
                "{name}: {crests} crests for {} lobes",
                params.lobe_count
            );
            // Sinuses cut toward the midrib and stop at it: section 10 is the
            // crest at mid-blade, section 12 the sinus past it.
            let sinus = widths[12] / widths[10];
            assert!(
                (0.2..0.6).contains(&sinus),
                "{name}: sinus at {sinus} of the crest"
            );
            assert!(widths.iter().all(|w| w.is_finite() && *w >= 0.));
        } else {
            assert_eq!(a.unit, FoliageUnit::Needle);
            let shaft = &e.positions[a.sections[1].clone()];
            // The strip is rolled shut: its two edges are one point, and the
            // section between them spans four sides at the same reach.
            assert!(
                (shaft[0] - shaft[shaft.len() - 1]).length() < 1e-9,
                "{name}: the section closes on itself"
            );
            let sides = &shaft[..shaft.len() - 1];
            let centre = sides.iter().fold(Vec3::ZERO, |a, b| a + *b) / sides.len() as f64;
            let reach: Vec<f64> = sides
                .iter()
                .map(|v| (v.x - centre.x).abs() + (v.z - centre.z).abs())
                .collect();
            assert_eq!(reach.len(), 4, "{name}: four sides");
            let half = params.width / 2.;
            for r in &reach {
                assert!(
                    (r - half).abs() < half * 0.02,
                    "{name}: reach {r} of {half}"
                );
            }
            assert!(extent(&a.sections[1], |v| v.x) > 0. && extent(&a.sections[1], |v| v.z) > 0.);
            // The shaft holds its width to the distal point rather than
            // tapering away like a blade.
            assert!(
                widths[widths.len() - 2] > 0.75 * widths[1],
                "{name}: shaft width"
            );
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
                lean_rise: -0.1,
                ..bare()
            },
            "lean rise",
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
fn needles_retain_shaft_width_before_the_distal_point() {
    let e = build_element(four_sided_needle()).unwrap();
    let sections = &e.anatomy.as_ref().unwrap().sections;
    let width = |i: usize| {
        let row = &e.positions[sections[i].clone()];
        row.iter().map(|p| p.x).fold(f64::NEG_INFINITY, f64::max)
            - row.iter().map(|p| p.x).fold(f64::INFINITY, f64::min)
    };
    // The rejected wedge lost most of its width by the final shaft section.
    assert!(width(sections.len() - 2) > 0.8 * width(1));
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

/// Distance from a point to the nearest point of one triangle: the projection
/// when it lands inside, the nearest point of the three edges otherwise.
/// Written out here rather than borrowed from the builder, so a builder that
/// measures its own deviation wrongly cannot also certify it.
fn distance_to_triangle(p: Vec3, a: Vec3, b: Vec3, c: Vec3) -> f64 {
    let edges = [(a, b), (b, c), (c, a)];
    let normal = (b - a).cross(c - a);
    if normal.length_squared() > 0.0 {
        let q = p - normal * ((p - a).dot(normal) / normal.length_squared());
        if edges
            .iter()
            .all(|(u, v)| (*v - *u).cross(q - *u).dot(normal) >= 0.0)
        {
            return p.distance(q);
        }
    }
    edges
        .iter()
        .map(|(u, v)| {
            let edge = *v - *u;
            let along = if edge.length_squared() > 0.0 {
                ((p - *u).dot(edge) / edge.length_squared()).clamp(0.0, 1.0)
            } else {
                0.0
            };
            p.distance(*u + edge * along)
        })
        .fold(f64::INFINITY, f64::min)
}

/// The transverse sections a level is chosen from: every element built by the
/// one routine publishes its own, whatever its traits say.
fn sections(e: &Element) -> Vec<Range<usize>> {
    e.anatomy
        .as_ref()
        .expect("a built element publishes its sections")
        .sections
        .clone()
}

fn triangles(e: &Element, level: &Level) -> Vec<[u32; 3]> {
    e.level_indices[level.indices.start as usize..level.indices.end as usize]
        .as_chunks::<3>()
        .0
        .to_vec()
}

#[test]
fn levels_nest_from_the_widest_section_down_to_the_whole_element() {
    // The shipped rows, and the corners of the trait space between them: the
    // ladder reads sections and positions only, so it has to build for every
    // one of them without knowing which is which.
    let cases = [
        ("generic blade", ElementParams::default(), 2..=4, usize::MAX),
        (
            "oak",
            Preset::OregonWhiteOak.parameters().element,
            4..=8,
            12,
        ),
        (
            "spruce",
            Preset::NorwaySpruce.parameters().element,
            2..=4,
            usize::MAX,
        ),
        (
            "no lobes at full depth",
            ElementParams {
                lobe_count: 0,
                lobe_depth: 1.0,
                ..ElementParams::default()
            },
            2..=4,
            usize::MAX,
        ),
        (
            "eight lobes cut to the midrib",
            ElementParams {
                lobe_count: 8,
                lobe_depth: 1.0,
                axial_segments: 32,
                ..ElementParams::default()
            },
            4..=8,
            usize::MAX,
        ),
        (
            "a section rolled shut",
            ElementParams {
                section_roundness: 1.0,
                cross_segments: 4,
                ..ElementParams::default()
            },
            2..=4,
            usize::MAX,
        ),
        (
            "every trait at its limit",
            ElementParams {
                lobe_count: 8,
                lobe_depth: 1.0,
                section_roundness: 1.0,
                axial_segments: 32,
                cross_segments: 4,
                ..ElementParams::default()
            },
            3..=6,
            usize::MAX,
        ),
    ];
    for (name, params, wanted, coarsest_triangles) in cases {
        let e = build_element(params).unwrap_or_else(|e| panic!("{name}: {e}"));
        assert!(e.validate().is_ok(), "{name}: the element is not valid");
        let sections = sections(&e);
        // The ladder doubles, so its length follows the logarithm of the
        // element: a longer one is a tail of levels that buy nothing.
        assert!(
            wanted.contains(&e.levels.len()),
            "{name}: {} levels, wanted {wanted:?}",
            e.levels.len()
        );

        let finest = e.levels.last().expect("a level list");
        assert_eq!(finest.deviation, 0.0, "{name}: the finest level is exact");
        assert_eq!(
            e.level_indices[finest.indices.start as usize..finest.indices.end as usize],
            e.indices[..],
            "{name}: the finest level is not the element itself"
        );
        assert!(
            e.levels.windows(2).all(|w| w[0].deviation > w[1].deviation),
            "{name}: deviations do not strictly decrease"
        );
        assert!(
            e.levels[0].indices.len() < finest.indices.len(),
            "{name}: the coarsest level saves no triangles"
        );
        assert!(
            e.levels[0].indices.len() / 3 <= coarsest_triangles,
            "{name}: the coarsest level is {} triangles",
            e.levels[0].indices.len() / 3
        );

        // Every level is a level apart from the one above it: twice the
        // triangles at least. The finest is the element itself and is kept
        // whatever it costs, so it only has to be the larger of the two.
        let counts: Vec<usize> = e.levels.iter().map(|l| l.indices.len() / 3).collect();
        for (n, pair) in counts.windows(2).enumerate() {
            let apart = if n + 2 < counts.len() {
                pair[1] >= pair[0] * 2
            } else {
                pair[1] > pair[0]
            };
            assert!(
                apart,
                "{name}: level {} is {} triangles against level {n}'s {}, no level apart",
                n + 1,
                pair[1],
                pair[0]
            );
        }

        // Base, tip and the widest section, whole, in every level: dropping the
        // widest is what turns a lobed blade into a sliver.
        let widest = |r: &Range<usize>| {
            let row = &e.positions[r.clone()];
            row.iter()
                .flat_map(|a| row.iter().map(move |b| a.distance_squared(*b)))
                .fold(0.0_f64, f64::max)
        };
        let anchor = (0..sections.len())
            .max_by(|a, b| widest(&sections[*a]).total_cmp(&widest(&sections[*b])))
            .expect("a section");
        let mut coarser: Option<BTreeSet<u32>> = None;
        for (n, level) in e.levels.iter().enumerate() {
            let used: BTreeSet<u32> = e.level_indices
                [level.indices.start as usize..level.indices.end as usize]
                .iter()
                .copied()
                .collect();
            for section in [0, anchor, sections.len() - 1] {
                for vertex in sections[section].clone() {
                    assert!(
                        used.contains(&(vertex as u32)),
                        "{name}: level {n} dropped part of section {section}"
                    );
                }
            }
            if let Some(previous) = &coarser {
                assert!(
                    previous.is_subset(&used),
                    "{name}: level {n} does not contain the level above it"
                );
            }

            // Every section vertex the level left out is inside its deviation.
            let faces = triangles(&e, level);
            for section in &sections {
                for vertex in section.clone() {
                    if used.contains(&(vertex as u32)) {
                        continue;
                    }
                    let distance = faces
                        .iter()
                        .map(|t| {
                            let [a, b, c] = t.map(|i| e.positions[i as usize]);
                            distance_to_triangle(e.positions[vertex], a, b, c)
                        })
                        .fold(f64::INFINITY, f64::min);
                    // The coarsest level's tolerance is exactly its own worst
                    // measurement, so the slack here is the round-off between
                    // two independent distance routines and nothing else.
                    assert!(
                        distance <= level.deviation * (1.0 + 1e-9),
                        "{name}: level {n} drops vertex {vertex} {distance} m out, \
                         past its {} m deviation",
                        level.deviation
                    );
                }
            }
            coarser = Some(used);
        }
    }
}

#[test]
fn a_level_list_that_is_not_nested_detail_is_rejected_by_name() {
    let whole = build_element(Preset::OregonWhiteOak.parameters().element).unwrap();
    assert!(whole.validate().is_ok());

    let mut flat = whole.clone();
    let deviation = flat.levels[0].deviation;
    flat.levels[1].deviation = deviation;
    assert_eq!(
        flat.validate().err(),
        Some(Error::InvalidInput("foliage element levels")),
        "deviations that do not decrease are not a level list"
    );

    let mut partial = whole.clone();
    let finest = partial.levels.last_mut().expect("a level list");
    finest.indices.end -= 3;
    assert_eq!(
        partial.validate().err(),
        Some(Error::InvalidInput("foliage element levels")),
        "a finest level short of the whole element is not a level list"
    );
}

/// FNV-1a over the element's positions and its whole index list: what the
/// renderer receives, and nothing about how it was asked for.
fn element_hash(p: ElementParams) -> u64 {
    let e = build_element(p).unwrap_or_else(|err| panic!("{p:?}: {err}"));
    let mut hash = 14695981039346656037_u64;
    for byte in e
        .positions
        .iter()
        .flat_map(|v| [v.x, v.y, v.z])
        .flat_map(f64::to_le_bytes)
        .chain(e.indices.iter().flat_map(|i| i.to_le_bytes()))
    {
        hash = (hash ^ byte as u64).wrapping_mul(1099511628211);
    }
    hash
}

#[test]
fn every_element_trait_moves_every_shipped_preset() {
    for preset in [
        Preset::Ordinary,
        Preset::OregonWhiteOak,
        Preset::NorwaySpruce,
        Preset::Telperion,
        Preset::Laurelin,
    ] {
        let p = preset.parameters().element;
        let base = element_hash(p);
        for (name, stepped) in [
            (
                "lobe count",
                ElementParams {
                    lobe_count: p.lobe_count + 1,
                    // A margin with one more lobe needs the sections to carry
                    // it; the depth is what puts them there.
                    lobe_depth: (p.lobe_depth + 0.1).min(1.0),
                    axial_segments: p.axial_segments.max(2 * p.lobe_count + 3),
                    ..p
                },
            ),
            (
                "lobe depth",
                ElementParams {
                    lobe_depth: (p.lobe_depth - 0.1).abs(),
                    lobe_count: p.lobe_count.max(1),
                    axial_segments: p.axial_segments.max(2 * p.lobe_count.max(1) + 2),
                    ..p
                },
            ),
            (
                "section roundness",
                ElementParams {
                    section_roundness: (p.section_roundness - 0.1).abs(),
                    ..p
                },
            ),
        ] {
            assert_ne!(
                element_hash(stepped),
                base,
                "{preset:?}: {name} moves no vertex"
            );
        }
    }
}

#[test]
fn a_margin_with_more_lobes_than_sections_is_rejected_by_both_names() {
    let entire = ElementParams {
        lobe_count: 5,
        lobe_depth: 0.0,
        axial_segments: 4,
        ..ElementParams::default()
    };
    // Without depth there is no sinus to carry, so the count costs nothing.
    assert!(build_element(entire).is_ok());
    assert_eq!(
        build_element(ElementParams {
            lobe_depth: 0.4,
            ..entire
        })
        .err(),
        Some(Error::InvalidInput(
            "leaf axial segments too few for the lobe count"
        ))
    );
    // A crest and a sinus per lobe, plus the base and the tip, is enough.
    assert!(build_element(ElementParams {
        lobe_depth: 0.4,
        axial_segments: 11,
        ..entire
    })
    .is_ok());
    for (bad, named) in [
        (
            ElementParams {
                lobe_count: 9,
                ..ElementParams::default()
            },
            "leaf lobe count",
        ),
        (
            ElementParams {
                lobe_depth: 1.1,
                ..ElementParams::default()
            },
            "leaf lobe depth",
        ),
        (
            ElementParams {
                section_roundness: -0.01,
                ..ElementParams::default()
            },
            "leaf section roundness",
        ),
        (
            ElementParams {
                section_roundness: f64::NAN,
                ..ElementParams::default()
            },
            "leaf section roundness",
        ),
    ] {
        assert_eq!(build_element(bad).err(), Some(Error::InvalidInput(named)));
    }
}

/// FNV-1a over every instance matrix as the renderer receives it: where each
/// leaf sits and how it leans, and nothing about how it was asked for.
fn placement_hash(f: &Family, tree: &Tree) -> u64 {
    let twig = f.skeleton.twigs.resolved().unwrap().twig;
    let placed = place_on_surface(
        tree,
        f.skeleton.envelope,
        f.skeleton.seed,
        f.canopy,
        Some(TwigPlacement {
            internode_length: twig.internode_length,
            stations_per_internode: twig.stations_per_internode,
        }),
        &f.surface,
    )
    .unwrap_or_else(|err| panic!("{:?}: {err}", f.canopy));
    let mut hash = 14695981039346656037_u64;
    for byte in placed
        .matrices
        .iter()
        .flat_map(|m| m.iter().flat_map(|v| v.to_le_bytes()))
    {
        hash = (hash ^ byte as u64).wrapping_mul(1099511628211);
    }
    hash
}

/// One step inside a trait's own range, whichever way there is room for it.
fn step(v: f64, hi: f64) -> f64 {
    if v + 0.1 <= hi {
        v + 0.1
    } else {
        v - 0.1
    }
}

#[test]
fn every_attachment_trait_moves_every_shipped_preset() {
    for preset in [
        Preset::Ordinary,
        Preset::OregonWhiteOak,
        Preset::NorwaySpruce,
        Preset::Telperion,
        Preset::Laurelin,
    ] {
        let family = preset.parameters();
        let tree = branching::generate(&family.skeleton, family.radii)
            .unwrap()
            .tree;
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

/// Along the blade and across it, base to tip and midrib to margin. The
/// connector has no blade of its own and takes the base's pair.
#[test]
fn a_blade_carries_a_coordinate_in_the_unit_square_at_every_vertex() {
    let element = build_element(ElementParams::default()).unwrap();
    assert_eq!(element.coords.len(), 2 * element.positions.len());
    for c in element.coords.as_chunks::<2>().0 {
        assert!(
            (0. ..=1.).contains(&c[0]) && (0. ..=1.).contains(&c[1]),
            "{c:?} is outside the unit square"
        );
    }
    let anatomy = element.anatomy.as_ref().unwrap();
    let coord = |v: usize| (element.coords[2 * v], element.coords[2 * v + 1]);
    assert_eq!(coord(0), (0., 0.), "the base is not the foot of the midrib");
    let tip = anatomy.sections.last().unwrap().start;
    assert_eq!(coord(tip), (1., 0.), "the tip is not the end of the midrib");

    // Every section stands at one distance along the blade, and they climb it.
    let mut previous = -1.;
    for section in &anatomy.sections {
        let along = coord(section.start).0;
        assert!(along > previous, "{along} does not climb past {previous}");
        for v in section.clone() {
            assert_eq!(coord(v).0, along, "vertex {v} left its own section");
        }
        previous = along;
    }
    // A row spans the blade: the midrib at nought, both margins at one.
    let row: Vec<f32> = anatomy.sections[1].clone().map(|v| coord(v).1).collect();
    assert_eq!(row.iter().cloned().fold(f32::INFINITY, f32::min), 0.);
    assert_eq!(row.iter().cloned().fold(f32::NEG_INFINITY, f32::max), 1.);
    // The peg below the blade carries the base's pair, whole vertices of it.
    for v in anatomy.vertices.end..element.positions.len() {
        assert_eq!(coord(v), (0., 0.), "the connector grew a blade coordinate");
    }

    // A card is the blade squashed to its extent: base and tip, margin across.
    let card = build_element(ElementParams {
        card: true,
        ..ElementParams::default()
    })
    .unwrap();
    assert_eq!(card.coords, vec![0., 1., 0., 1., 1., 1., 1., 1.]);
}
