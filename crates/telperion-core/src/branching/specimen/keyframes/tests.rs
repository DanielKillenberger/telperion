use super::*;
use crate::presets::Preset;

fn radii(s: &Specimen, year: u64) -> Vec<(NodeIdentity, [u64; 3])> {
    s.tree
        .nodes
        .iter()
        .filter(|n| {
            n.shoot.birth_year <= year as f64 && n.shoot.death_year.is_none_or(|death| year < death)
        })
        .map(|n| {
            let frames = s
                .keyframes
                .frames
                .get(n.identity.key)
                .expect("living node has no radius keyframes");
            let frame = frames
                .iter()
                .rev()
                .find(|f| f.year <= year)
                .expect("no keyframe at birth");
            (n.identity, frame.radii.map(f64::to_bits))
        })
        .collect()
}

#[test]
fn ten_year_jump_retains_every_annual_radius_read_and_interval() {
    for tolerance in [0.0, 1e-9, 0.0001, 0.001, 1.0] {
        compare_history(tolerance);
    }
}

fn compare_history(tolerance: f64) {
    let mut family = Preset::Ordinary.parameters();
    family.growth.resize_tolerance = tolerance;
    family.age = 10.0;
    let jump = Specimen::build(&family).unwrap();
    family.age = 0.0;
    let mut yearly = Specimen::build(&family).unwrap();
    for year in 0..=10 {
        if year > 0 {
            yearly.advance(1.0).unwrap();
        }
        family.age = year as f64;
        let fresh = Specimen::build(&family).unwrap();
        assert!(
            super::super::tests::bytes(yearly.tree()) == super::super::tests::bytes(fresh.tree())
        );
        assert_eq!(radii(&jump, year), radii(&yearly, year));
        let output: Vec<_> = yearly
            .tree
            .nodes
            .iter()
            .filter(|n| n.shoot.death_year.is_none())
            .map(|n| {
                (
                    n.identity,
                    [n.radius, n.start_radius, n.base_radius].map(f64::to_bits),
                )
            })
            .collect();
        assert_eq!(
            radii(&jump, year),
            output,
            "historical radii differ from the yearly output"
        );
        for node in &yearly.tree.nodes {
            let a: Vec<_> = jump.keyframes.frames[node.identity.key]
                .iter()
                .filter(|f| f.year <= year)
                .copied()
                .collect();
            assert_eq!(
                a, yearly.keyframes.frames[node.identity.key],
                "advance partition changed keyframes"
            );
        }
        for from in 0..=year {
            let interval = |s: &Specimen| {
                s.tree
                    .nodes
                    .iter()
                    .flat_map(|n| {
                        s.keyframes
                            .frames
                            .get(n.identity.key)
                            .into_iter()
                            .flatten()
                            .filter(move |f| from < f.year && f.year <= year)
                            .map(move |f| (n.identity, f.year, f.radii.map(f64::to_bits)))
                    })
                    .collect::<Vec<_>>()
            };
            assert_eq!(interval(&jump), interval(&yearly));
        }
    }
}

#[test]
fn resize_tolerance_accumulates_from_last_frame_without_rewriting_it() {
    let mut family = Preset::Ordinary.parameters();
    family.age = 0.0;
    let mut s = Specimen::build(&family).unwrap();
    let id = s.tree.nodes[0].identity;
    let mut history = Keyframes::default();
    history.record(id, 0, [1.0; 3], 0.25);
    history.record(id, 1, [1.125; 3], 0.25);
    history.record(id, 2, [1.25; 3], 0.25);
    assert_eq!(
        history.frames[id.key].len(),
        1,
        "tolerance equality wrote a frame"
    );
    history.record(id, 3, [1.375, 1.0, 0.5], 0.25);
    assert_eq!(
        history.frames[id.key],
        vec![
            Frame {
                year: 0,
                radii: [1.0; 3]
            },
            Frame {
                year: 3,
                radii: [1.375, 1.0, 1.0]
            },
        ]
    );
    // On a real specimen a large tolerance suppresses resizes, not births.
    let mut family = Preset::Ordinary.parameters();
    family.age = 10.0;
    family.growth.resize_tolerance = 1.0;
    s = Specimen::build(&family).unwrap();
    assert!(s.keyframes.frames.values().all(|frames| frames.len() == 1));
    s.tree().validate_solved().unwrap();
}

#[test]
fn annual_keyframes_do_not_finalize_stored_output_radii() {
    let mut family = Preset::Ordinary.parameters();
    family.age = 8.0;
    let mut s = Specimen::build(&family).unwrap();
    let previous = s.tree.nodes.clone();
    let slice = 9;
    s.slice(slice, family.growth.budget(slice)).unwrap();
    let mut thickened = 0;
    for (i, old) in previous.iter().enumerate() {
        let n = &s.tree.nodes[i];
        if n.shoot.death_year.is_some() {
            continue;
        }
        assert_eq!(
            [n.radius, n.start_radius, n.base_radius],
            [old.radius, old.start_radius, old.base_radius],
            "slice finalized output radii"
        );
        thickened += usize::from(s.keyframes.frames[n.identity.key].last().unwrap().year == slice);
    }
    assert!(thickened > 0, "slice failed to record changed widths");
}

#[test]
fn radius_history_is_monotone_through_extension_forks_and_shedding() {
    let mut family = Preset::Ordinary.parameters();
    // Exercise shedding explicitly; production presets retain mature wood (fn-30).
    family.skeleton.habit.shedding_threshold = 0.45;
    family.age = 12.0;
    let s = Specimen::build(&family).unwrap();
    assert!(s.shed > 0);
    assert!(s.tree.nodes.iter().any(|n| n.kind == NodeKind::Twig));
    let mut children = std::collections::BTreeMap::new();
    for link in s.links.values() {
        if let Some(parent) = link.parent {
            *children.entry(parent).or_insert(0) += 1;
        }
    }
    assert!(
        children.values().any(|&count| count > 1),
        "fixture must fork"
    );
    let mut thickened = 0;
    for n in &s.tree.nodes {
        let frames = s
            .keyframes
            .frames
            .get(n.identity.key)
            .expect("missing history");
        assert_eq!(frames[0].year as f64, n.shoot.birth_year);
        for pair in frames.windows(2) {
            assert!(pair[0].year < pair[1].year);
            assert!(
                pair[0]
                    .radii
                    .iter()
                    .zip(pair[1].radii)
                    .all(|(old, new)| new >= *old),
                "radius history thinned"
            );
            thickened += 1;
        }
        assert!(frames
            .iter()
            .all(|f| n.shoot.death_year.is_none_or(|d| f.year <= d)));
    }
    assert!(thickened > 0);
}

#[test]
fn unread_annual_frames_queue_each_output_only_once() {
    let mut family = Preset::Ordinary.parameters();
    family.age = 0.0;
    let s = Specimen::build(&family).unwrap();
    let id = s.tree.nodes[0].identity;
    let mut history = Keyframes::default();
    for year in 0..173 {
        history.record(id, year, [year as f64 + 1.0; 3], 0.0);
    }
    assert_eq!(history.frames[id.key].len(), 173);
    assert_eq!(
        history.queue_searches, 1,
        "unread annual frames repeatedly search the output queue"
    );
}

#[test]
fn unchanged_canonical_parents_do_not_walk_local_descendants() {
    let mut f = Preset::Ordinary.parameters();
    f.growth.resize_tolerance = 1.0;
    f.age = 0.0;
    let mut s = Specimen::build(&f).unwrap();
    for year in 1..=12 {
        let first = s.tree.nodes.len();
        s.slice(year, f.growth.budget(year)).unwrap();
        let born = s.tree.nodes[first..]
            .iter()
            .filter(|n| n.kind != NodeKind::Structural)
            .count();
        assert!(
            s.cost.keyframe_widths <= born,
            "year {year}: {} local solves for {born} births despite unchanged canonical parents",
            s.cost.keyframe_widths
        );
        s.timeline.as_mut().unwrap().age = crate::growth::Age {
            slice: year,
            remainder: 0,
        };
    }
}
