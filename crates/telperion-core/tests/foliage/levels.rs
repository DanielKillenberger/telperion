use super::*;

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
