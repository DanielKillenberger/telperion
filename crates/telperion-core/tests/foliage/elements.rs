use super::*;

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
