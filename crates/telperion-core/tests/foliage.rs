use std::collections::BTreeSet;
use std::ops::Range;
use telperion_core::{
    envelope::Envelope,
    foliage::*,
    math::Vec3,
    presets::Preset,
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

#[test]
fn needles_retain_shaft_width_before_the_distal_point() {
    let e = build_element(species_element(ElementAnatomy::FourSidedNeedle)).unwrap();
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
fn evergreen_needles_clothe_slender_supports_but_not_thick_limbs() {
    let mut t = twig(0.08);
    t.nodes[1].kind = NodeKind::Branch;
    t.crossover = t.nodes.len();
    let stations = Some(TwigPlacement::default());
    let needles = CanopyParams {
        attachment: Attachment::RadialNeedles,
        shoot_radius: 0.005,
        ..bare()
    };
    let placed = place(&t, Envelope::default(), 1, needles, stations).unwrap();
    assert_eq!(placed.matrices.len(), 4);
    assert!(place(
        &t,
        Envelope::default(),
        1,
        CanopyParams {
            attachment: Attachment::Alternate,
            ..needles
        },
        stations
    )
    .unwrap()
    .matrices
    .is_empty());
    t.nodes[1].start_radius = 0.006;
    assert!(place(&t, Envelope::default(), 1, needles, stations)
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

/// The transverse sections a level is chosen from. The species anatomies
/// publish theirs; the generic grid does not, so its layout — attachment
/// point, one range per row, tip — is spelled out from its own parameters.
fn sections(e: &Element, p: ElementParams) -> Vec<Range<usize>> {
    if let Some(a) = &e.anatomy {
        return a.sections.clone();
    }
    let columns = (p.cross_segments + p.cross_segments % 2) as usize;
    let rows = (p.axial_segments - 1) as usize;
    std::iter::once(0..1)
        .chain((0..rows).map(|r| 1 + r * (columns + 1)..1 + (r + 1) * (columns + 1)))
        .chain(std::iter::once(
            1 + rows * (columns + 1)..2 + rows * (columns + 1),
        ))
        .collect()
}

fn triangles(e: &Element, level: &Level) -> Vec<[u32; 3]> {
    e.level_indices[level.indices.start as usize..level.indices.end as usize]
        .as_chunks::<3>()
        .0
        .to_vec()
}

#[test]
fn levels_nest_from_the_widest_section_down_to_the_whole_element() {
    let cases = [
        ("generic blade", ElementParams::default(), 2, usize::MAX),
        ("oak", Preset::OregonWhiteOak.parameters().element, 4, 12),
        (
            "spruce",
            Preset::NorwaySpruce.parameters().element,
            2,
            usize::MAX,
        ),
    ];
    for (name, params, fewest, coarsest_triangles) in cases {
        let e = build_element(params).unwrap();
        let sections = sections(&e, params);
        assert!(
            e.levels.len() >= fewest,
            "{name}: {} levels, wanted at least {fewest}",
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
