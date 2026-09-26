use super::*;
#[test]
fn capacity_gate_requires_both_phases_to_fit() {
    assert_eq!(size_of::<SurfaceRun>(), 16);
    assert!(envelope(
        6_129_263, 889_552, 12_208, 800, 3_724_874, 21_682_080, 55_597, 8
    ));
    assert!(!envelope(0, 0, 0, 0, 3_724_874, 21_682_080, 55_597, 8));
}
const DRAWN: Sweep = Sweep {
    drawn: true,
    edges: false,
};
/// Both phases on every worker, whatever the admission would say.
fn candidate(tree: &Tree, params: &SurfaceParams) -> Result<SurfaceMesh> {
    use super::super::rings::{distances, rank, table, Scratch};
    let height = 24.0;
    let paths = paths(&tree.nodes)?;
    let segments = segments(params);
    let distance = distances(tree)?;
    let angular = angular::samples(segments, params)?;
    let at = Swept {
        tree,
        height,
        params,
        paths: &paths,
        distance: &distance,
        angular: &angular,
    };
    let ordered = rank(at, &mut Scratch::new(at.longest()?)?)?;
    let mut out = Rings {
        positions: Vec::new(),
        coords: Vec::new(),
        edges: Vec::new(),
        runs: Vec::new(),
        segments,
        workers: None,
    };
    rings(at, &ordered, &mut out, DRAWN, at.longest()?, workers())?;
    let out = table(at, ordered, out)?;
    let faces = faces(&out, &Resweep::new(tree, height, params), workers())?;
    Ok(out.into_mesh(faces))
}
/// Both steps in turn.
fn serial(tree: &Tree, height: f64, params: &SurfaceParams) -> Result<SurfaceMesh> {
    let rings = super::super::rings::rings_mode(tree, height, params, DRAWN, false)?;
    let faces = super::super::faces(&rings, tree, height, params)?;
    Ok(rings.into_mesh(faces))
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
        let expected = serial(&tree, 24.0, &p).unwrap();
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
        serial(&tree, 24.0, &p),
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
    let expected = serial(&t, 24.0, &p).unwrap();
    for after in [0, 1, workers(), workers() + 1] {
        SPAWN_FAILURE.with(|f| f.set(Some(after)));
        let result = candidate(&t, &p);
        SPAWN_FAILURE.with(|f| f.set(None));
        assert!(result.is_err());
        assert_eq!(serial(&t, 24.0, &p).unwrap(), expected);
    }
}
/// A spawn refused in the ring step sweeps the rings again in turn; one
/// refused in the mesh step builds only the faces again, on the same rings.
#[test]
fn admitted_public_build_retries_the_failed_step_in_turn() {
    if workers() < 2 {
        return;
    }
    let p = crate::presets::Preset::from_id("oregon-white-oak")
        .unwrap()
        .parameters();
    let t = crate::pipeline::branching::generate(&p.skeleton, p.radii).unwrap();
    let height = p.skeleton.envelope.height;
    let expected = serial(&t.tree, height, &p.surface).unwrap();
    for (after, calls) in [(1, 2), (workers() + 1, workers() + 2)] {
        SPAWN_CALLS.with(|c| c.set(0));
        SPAWN_FAILURE.with(|f| f.set(Some(after)));
        let actual = super::super::build(&t.tree, height, &p.surface);
        SPAWN_FAILURE.with(|f| f.set(None));
        assert_eq!(SPAWN_CALLS.with(|c| c.get()), calls, "{after}");
        assert_eq!(actual.unwrap(), expected, "{after}");
    }
}
/// A part left short, or a buffer with a part never handed out, is no
/// buffer; parts written in full are.
#[test]
fn an_unfilled_buffer_is_whole_only_when_every_part_is() {
    let whole = |take: &[usize], write: &[usize]| {
        let mut buffer = Unfilled::<u32>::new(4).unwrap();
        let mut parts = buffer.parts();
        for (&n, &w) in take.iter().zip(write) {
            parts.take(n).unwrap().extend(&vec![7; w]);
        }
        buffer.filled().ok()
    };
    assert_eq!(whole(&[1, 3], &[1, 3]), Some(vec![7; 4]));
    assert_eq!(whole(&[1, 3], &[1, 2]), None);
    assert_eq!(whole(&[1, 2], &[1, 2]), None);
}
