//! fn-49: a triangle whose float32 corners span no area is dropped from its
//! run rather than failing the tree. The run's span in the index buffer
//! shrinks by exactly what it dropped, a vertex left with no triangle faces
//! the way its ring does, and a tree that drops more than two rings' worth
//! still fails, naming the count.
use telperion_core::{
    branching,
    math::Vec3,
    presets::Preset,
    surface::{build, SurfaceMesh, SurfaceParams},
    tree::{Node, Tree},
    Error,
};

const SEGMENTS: usize = 12;

fn params() -> SurfaceParams {
    SurfaceParams {
        radial_segments: SEGMENTS as u32,
        lobes: 0,
        flare_radius: 1.0,
        flare_depth: 0.0,
        ..SurfaceParams::default()
    }
}

fn node(position: Vec3, parent: Option<u32>, radius: f64) -> Node {
    Node {
        position,
        parent,
        radius,
        start_radius: radius.max(1.0),
        ..Node::root()
    }
}

/// One straight run up through `at`: a node at each height, with its radius.
fn run(at: Vec3, rings: &[(f64, f64)]) -> Tree {
    Tree {
        nodes: rings
            .iter()
            .enumerate()
            .map(|(i, &(y, r))| {
                node(
                    at + Vec3::new(0.0, y, 0.0),
                    i.checked_sub(1).map(|p| p as u32),
                    r,
                )
            })
            .collect(),
        ..Tree::default()
    }
}

fn corner(mesh: &SurfaceMesh, index: u32) -> Vec3 {
    let p = &mesh.positions[index as usize * 3..];
    Vec3::new(p[0] as f64, p[1] as f64, p[2] as f64)
}

fn normal(mesh: &SurfaceMesh, index: usize) -> Vec3 {
    let n = &mesh.normals[index * 3..];
    Vec3::new(n[0] as f64, n[1] as f64, n[2] as f64)
}

/// The spans tile the buffer in order, every kept triangle has area in
/// float32, and every vertex carries a unit normal.
fn honest(mesh: &SurfaceMesh) {
    let mut next = 0;
    for run in &mesh.run_table {
        assert_eq!(run.first_index, next);
        next += run.index_count;
    }
    assert_eq!(next as usize, mesh.indices.len());
    for t in mesh.indices.as_chunks::<3>().0 {
        let [a, b, c] = t.map(|i| corner(mesh, i));
        assert!((c - b).cross(a - b).length_squared() > 0.0);
    }
    for i in 0..mesh.normals.len() / 3 {
        assert!((normal(mesh, i).length() - 1.0).abs() < 1e-6);
    }
}

/// A straight run of twelve segments and three rings sweeps 72 triangles.
const WHOLE: usize = 2 * 2 * SEGMENTS + 2 * SEGMENTS;

#[test]
fn a_ring_collapsed_to_one_point_drops_the_strips_that_meet_it() {
    let tree = run(Vec3::ZERO, &[(0.0, 1.0), (4.0, 1e-300), (8.0, 1.0)]);
    let mesh = build(&tree, 24.0, &params()).unwrap();
    for k in 0..SEGMENTS as u32 {
        assert_eq!(corner(&mesh, SEGMENTS as u32 + k), Vec3::new(0.0, 4.0, 0.0));
    }
    assert_eq!(mesh.dropped, 2 * SEGMENTS);
    assert_eq!(mesh.indices.len(), 3 * (WHOLE - 2 * SEGMENTS));
    honest(&mesh);
}

#[test]
fn a_tip_below_float32_resolution_far_out_drops_its_cap() {
    // A millimetre of radius where a float32 step is eight: the tip ring
    // rounds onto the tip itself, and the cap vertex keeps no triangle.
    let at = Vec3::new(1e5, 0.0, 1e5);
    let mesh = build(
        &run(at, &[(0.0, 1.0), (4.0, 1.0), (8.0, 1e-3)]),
        24.0,
        &params(),
    )
    .unwrap();
    assert_eq!(mesh.dropped, 2 * SEGMENTS);
    honest(&mesh);
    let top = 3 * SEGMENTS + 1;
    assert_eq!(corner(&mesh, top as u32), at + Vec3::new(0.0, 8.0, 0.0));
    assert!(normal(&mesh, top).distance(Vec3::Y) < 1e-6);
}

#[test]
fn the_run_table_shrinks_only_the_run_that_dropped() {
    // A limb forks off the trunk's middle node; its own middle ring is either
    // wood or a point.
    let fork = |radius: f64| {
        let mut tree = run(Vec3::ZERO, &[(0.0, 1.0), (4.0, 0.8), (8.0, 0.5)]);
        tree.nodes
            .push(node(Vec3::new(2.0, 6.0, 0.0), Some(1), radius));
        tree.nodes
            .push(node(Vec3::new(4.0, 8.0, 0.0), Some(3), 0.2));
        tree
    };
    let whole = build(&fork(0.25), 24.0, &params()).unwrap();
    let mesh = build(&fork(1e-300), 24.0, &params()).unwrap();
    assert_eq!((whole.dropped, mesh.dropped), (0, 2 * SEGMENTS));
    assert_eq!(whole.run_table.len(), 2);
    assert_eq!(mesh.run_table[0], whole.run_table[0]);
    assert_eq!(
        mesh.run_table[1].index_count,
        whole.run_table[1].index_count - 3 * 2 * SEGMENTS as u32
    );
    honest(&mesh);
    // Each span still addresses its own vertices, and only them.
    let span = |r: usize| {
        let run = mesh.run_table[r];
        let indices = &mesh.indices[run.first_index as usize..][..run.index_count as usize];
        (
            *indices.iter().min().unwrap(),
            *indices.iter().max().unwrap(),
        )
    };
    let (trunk, limb) = (span(0), span(1));
    assert_eq!(trunk.0, 0);
    assert!(trunk.1 < limb.0);
    assert_eq!(limb.1 as usize, mesh.positions.len() / 3 - 1);
}

/// Rings stacked a micrometre apart a hundred kilometres up: one float32
/// position, so the strips between them sweep no area at all.
fn stacked(rings: usize) -> Tree {
    let y = 1e5;
    let mut heights = vec![(y - 4.0, 1.0)];
    heights.extend((0..rings).map(|i| (y + i as f64 * 1e-6, 1.0)));
    heights.push((y + 4.0, 1.0));
    run(Vec3::ZERO, &heights)
}

#[test]
fn a_vertex_left_without_a_triangle_faces_out_from_its_ring() {
    let mesh = build(&stacked(3), 24.0, &params()).unwrap();
    assert_eq!(mesh.dropped, 2 * 2 * SEGMENTS);
    honest(&mesh);
    // The middle ring of three keeps no triangle; each of its vertices faces
    // straight out from the axis it sits around.
    for k in 0..SEGMENTS {
        let v = 2 * SEGMENTS + k;
        let p = corner(&mesh, v as u32);
        let out = Vec3::new(p.x, 0.0, p.z).normalized();
        assert!(normal(&mesh, v).distance(out) < 1e-6, "vertex {k}");
    }
}

#[test]
fn past_two_rings_worth_the_tree_fails_naming_the_count() {
    // Three stacked rings drop exactly the bound and build; a fourth drops
    // one more ring's worth of strips and fails.
    assert_eq!(
        build(&stacked(3), 24.0, &params()).unwrap().dropped,
        4 * SEGMENTS
    );
    assert_eq!(
        build(&stacked(4), 24.0, &params()),
        Err(Error::InvalidValue {
            field: "surface triangles collapsed in float32",
            value: (6 * SEGMENTS).to_string(),
        })
    );
}

#[test]
fn a_position_float32_cannot_hold_is_still_an_error() {
    let tree = run(
        Vec3::new(1e40, 0.0, 0.0),
        &[(0.0, 1.0), (4.0, 0.5), (8.0, 0.25)],
    );
    assert_eq!(
        build(&tree, 24.0, &params()),
        Err(Error::InvalidInput("surface float32 position overflow"))
    );
}

/// fn-134: every run's rings are swept before any run is shaded, so a later
/// run whose positions float32 cannot hold answers before an earlier run whose
/// normals overflow. Base shaded run by run and named the normal first; the
/// owner accepted the new order on 2026-09-24. Both inputs fail either way.
#[test]
fn a_position_overflow_in_a_later_run_answers_before_an_earlier_normal() {
    let tree = Tree {
        nodes: vec![
            node(Vec3::ZERO, None, 1e20),
            node(Vec3::new(0.0, 1e20, 0.0), Some(0), 1e20),
            node(Vec3::new(1e40, 0.0, 0.0), Some(0), 1.0),
        ],
        ..Tree::default()
    };
    let params = SurfaceParams {
        radial_segments: 8,
        ..params()
    };
    assert_eq!(
        build(&tree, 1.0, &params),
        Err(Error::InvalidInput("surface float32 position overflow"))
    );
}

/// The candidate beech of fn-45's round 6c, its wood rows as that branch
/// stated them (it also named a twig generation rail this branch does not
/// carry), failed this seed: two stations of one truncated limb 11.4
/// micrometres apart, 25 m up. A test-only family; no shipped table moves.
/// Every wood row the shipped beech has since moved is stated here, so the
/// reproduction does not drift with the catalogue's table.
#[test]
fn the_reproducing_beech_builds_and_drops_two_triangles() {
    let mut f = Preset::EuropeanBeech.parameters();
    let h = &mut f.skeleton.habit;
    h.apical_dominance = 0.58;
    h.leader_internode = 2.2;
    h.pitch_variation = 10.0;
    h.crookedness = 6.0;
    h.rise_primary = 0.3;
    h.laterals_per_station = 2;
    h.lateral_pitch = 58.0;
    h.rise_secondary = 0.1;
    h.lateral_spacing = 2.2;
    h.lateral_length_ratio = 0.6;
    f.skeleton.envelope.crown_base = 0.05;
    f.skeleton.envelope.spread = 0.52;
    f.skeleton.envelope.fullness = 0.48;
    f.skeleton.envelope.shoulder = 1.5;
    f.skeleton.twigs.generations = telperion_core::twigs::MAX_GENERATIONS;
    f.skeleton.twigs.laterals = 4;
    f.skeleton.twigs.limb_radius = 0.1;
    f.skeleton.twigs.length_ratio = 0.36;
    f.skeleton.twigs.angle = 32.0;
    f.skeleton.twigs.divergence = 180.0;
    f.radii.trunk_radius = 0.014;
    f.radii.length_taper = 0.2;
    f.radii.fork_exponent = 2.6;
    f.skeleton.seed = 266;
    let tree = branching::generate(&f.skeleton, f.radii).unwrap().tree;
    let mesh = build(&tree, f.skeleton.envelope.height, &f.surface).unwrap();
    assert_eq!(mesh.dropped, 2);
    honest(&mesh);
}
