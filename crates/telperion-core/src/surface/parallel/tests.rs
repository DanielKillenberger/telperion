use super::*;
#[test]
fn capacity_gate_requires_both_phases_to_fit() {
    assert_eq!(size_of::<Run>(), 32);
    assert!(envelope(
        6_129_263, 889_552, 12_208, 800, 3_724_874, 21_682_080, 55_597, 8
    ));
    assert!(!envelope(0, 0, 0, 0, 3_724_874, 21_682_080, 55_597, 8));
}
fn candidate(tree: &Tree, params: &SurfaceParams) -> Result<SurfaceMesh> {
    let paths = paths(&tree.nodes)?;
    let segments = params.radial_segments.max(params.lobes * 4) as usize;
    let mut distance = filled(tree.nodes.len(), 0.0)?;
    for i in 1..tree.nodes.len() {
        let parent = tree.nodes[i].parent.unwrap() as usize;
        distance[i] =
            distance[parent] + tree.nodes[parent].position.distance(tree.nodes[i].position);
    }
    let longest = paths.runs.iter().map(|r| r.end - r.start).max().unwrap() + 1;
    let mut samples = reserved(longest)?;
    let mut ordered = Vec::new();
    let mut rings = 0;
    for (i, run) in paths.runs.iter().enumerate() {
        sample_path(tree, 24.0, params, &paths, run, &distance, &mut samples);
        ordered.push((i, samples.iter().map(|s| s.r).fold(0.0, f64::max)));
        rings += samples.len();
    }
    ordered.sort_by(|a, b| b.1.total_cmp(&a.1));
    let vertices = rings * segments + paths.runs.len() * 2;
    build(
        tree,
        24.0,
        params,
        paths,
        distance,
        ordered,
        angular::samples(segments, params)?,
        longest,
        vertices,
        rings * segments * 6,
        workers(),
        None,
    )
}
fn tree() -> Tree {
    use crate::tree::Node;
    Tree {
        nodes: (0..25)
            .map(|i| Node {
                position: Vec3::new((i % 5) as f64, i as f64 * 2.0, (i % 3) as f64),
                parent: if i == 0 { None } else { Some((i - 1) / 2) },
                radius: 1.0 / (i + 1) as f64,
                start_radius: 1.0,
                ..Node::root()
            })
            .collect(),
        ..Tree::default()
    }
}
#[test]
fn independent_slices_match_serial_with_modulation_and_caps() {
    if workers() < 2 {
        return;
    }
    let tree = tree();
    for twist in [0.0, 0.37] {
        let p = SurfaceParams {
            twist_rate: twist,
            ..Default::default()
        };
        let expected = build_mode(&tree, 24.0, &p, None, None, false).unwrap();
        let actual = candidate(&tree, &p).unwrap();
        assert_eq!(actual, expected);
        for (a, b) in actual
            .positions
            .iter()
            .chain(&actual.coords)
            .chain(&actual.normals)
            .zip(
                expected
                    .positions
                    .iter()
                    .chain(&expected.coords)
                    .chain(&expected.normals),
            )
        {
            assert_eq!(a.to_bits(), b.to_bits());
        }
    }
}
#[test]
fn collapsed_candidate_is_discarded_and_serial_keeps_its_error() {
    if workers() < 2 {
        return;
    }
    let mut tree = tree();
    for node in &mut tree.nodes {
        node.radius = 1e-300;
        node.start_radius = 1e-300;
    }
    let p = SurfaceParams::default();
    assert!(candidate(&tree, &p).is_err());
    assert!(matches!(
        build_mode(&tree, 24.0, &p, None, None, false),
        Err(Error::InvalidValue {
            field: "surface triangles collapsed in float32",
            ..
        })
    ));
}
#[test]
fn failed_worker_does_not_skip_other_joins() {
    use std::sync::atomic::{AtomicBool, Ordering};
    let finished = AtomicBool::new(false);
    thread::scope(|scope| {
        let mut handles = std::array::from_fn(|_| None);
        handles[0] = Some(scope.spawn(|| -> Result<()> { Err(failed()) }));
        handles[1] = Some(scope.spawn(|| -> Result<()> {
            finished.store(true, Ordering::SeqCst);
            Ok(())
        }));
        assert!(join_all(&mut handles).is_err());
        assert!(finished.load(Ordering::SeqCst));
        assert!(handles.iter().all(Option::is_none));
    });
}
#[test]
fn spawn_failure_in_either_phase_returns_to_serial_after_join() {
    if workers() < 2 {
        return;
    }
    let t = tree();
    let p = SurfaceParams::default();
    let expected = build_mode(&t, 24.0, &p, None, None, false).unwrap();
    for after in [0, 1, workers(), workers() + 1] {
        SPAWN_FAILURE.with(|f| f.set(Some(after)));
        let result = candidate(&t, &p);
        SPAWN_FAILURE.with(|f| f.set(None));
        assert!(result.is_err());
        assert_eq!(
            build_mode(&t, 24.0, &p, None, None, false).unwrap(),
            expected
        );
    }
}
#[test]
fn admitted_public_build_retries_once_after_started_worker_spawn_failure() {
    if workers() < 2 {
        return;
    }
    let p = crate::presets::Preset::from_id("oregon-white-oak")
        .unwrap()
        .parameters();
    let t = crate::branching::generate(&p.skeleton, p.radii).unwrap();
    let expected = build_mode(
        &t.tree,
        p.skeleton.envelope.height,
        &p.surface,
        None,
        None,
        false,
    )
    .unwrap();
    SPAWN_CALLS.with(|c| c.set(0));
    SPAWN_FAILURE.with(|f| f.set(Some(1)));
    let actual = super::super::build(&t.tree, p.skeleton.envelope.height, &p.surface);
    SPAWN_FAILURE.with(|f| f.set(None));
    assert_eq!(
        SPAWN_CALLS.with(|c| c.get()),
        2,
        "production admission/fallback was not exercised exactly once"
    );
    assert_eq!(actual.unwrap(), expected);
}
