use telperion_core::{
    math::Vec3,
    surface::{build, SurfaceParams},
    tree::{Node, Tree},
};
fn straight() -> Tree {
    Tree {
        nodes: (0..3)
            .map(|i| Node {
                position: Vec3::new(0.0, i as f64 * 4.0, 0.0),
                parent: if i == 0 { None } else { Some(i - 1) },
                radius: 1.0 / (i + 1) as f64,
                start_radius: 1.0,
                ..Node::root()
            })
            .collect(),
        ..Tree::default()
    }
}
#[test]
fn tapered_closed_surface_has_normals_and_bounds() {
    let params = SurfaceParams {
        lobes: 0,
        radial_segments: 8,
        flare_radius: 1.0,
        flare_depth: 0.0,
        ..SurfaceParams::default()
    };
    let mesh = build(&straight(), 24.0, &params).unwrap();
    assert_eq!(mesh.positions.len(), 26 * 3);
    assert_eq!(mesh.indices.len(), 3 * 8 * 6);
    assert_eq!(mesh.normals.len(), mesh.positions.len());
    assert_eq!(mesh.bounds.unwrap().min.y, 0.0);
    assert_eq!(mesh.bounds.unwrap().max.y, 8.0);
    for i in 0..3 {
        for k in 0..8 {
            let p = &mesh.positions[(i * 8 + k) * 3..];
            assert!(((p[0] as f64).hypot(p[2] as f64) - 1.0 / (i + 1) as f64).abs() < 1e-6);
        }
    }
    let mut edges = std::collections::HashSet::new();
    let mut volume = 0.0;
    let point = |i: u32| {
        let p = &mesh.positions[i as usize * 3..];
        Vec3::new(p[0] as f64, p[1] as f64, p[2] as f64)
    };
    for t in mesh.indices.as_chunks::<3>().0 {
        for k in 0..3 {
            assert!(edges.insert((t[k], t[(k + 1) % 3])));
        }
        let (a, b, c) = (point(t[0]), point(t[1]), point(t[2]));
        assert!((b - a).cross(c - a).length() > 0.0);
        volume += a.dot(b.cross(c));
    }
    assert!(volume > 0.0);
    for &(a, b) in &edges {
        assert!(edges.contains(&(b, a)));
    }
    for n in mesh.normals.as_chunks::<3>().0 {
        assert!(((n[0] * n[0] + n[1] * n[1] + n[2] * n[2]).sqrt() - 1.0).abs() < 1e-6);
    }
}
#[test]
fn empty_degenerate_and_invalid_are_atomic() {
    let p = SurfaceParams::default();
    assert!(build(&Tree::default(), 24.0, &p).unwrap().bounds.is_none());
    let mut tree = straight();
    tree.nodes[1].position = Vec3::ZERO;
    let mesh = build(&tree, 24.0, &p).unwrap();
    assert_eq!(mesh.runs, 1);
    tree.nodes[2].position = Vec3::ZERO;
    assert!(build(&tree, 24.0, &p).unwrap().positions.is_empty());
    for invalid in [f64::NAN, f64::INFINITY, -1.0, 0.0] {
        assert!(build(&straight(), invalid, &p).is_err());
    }
    tree.nodes[1].parent = Some(2);
    assert!(build(&tree, 24.0, &p).is_err());
    assert!(build(
        &straight(),
        24.0,
        &SurfaceParams {
            lobe_depth: f64::NAN,
            ..p
        }
    )
    .is_err());
    let mut huge = straight();
    huge.nodes[2].position.y = 1e300;
    assert!(build(&huge, 24.0, &p).is_err());
}
#[test]
fn fork_socket_is_contained_and_thickest_child_continues() {
    let mut tree = straight();
    tree.nodes[2].position = Vec3::new(2.0, 6.0, 0.0);
    let mut side = tree.nodes[2].clone();
    side.position.x = -2.0;
    side.start_radius = 0.5;
    side.radius = 0.3;
    tree.nodes.push(side);
    let params = SurfaceParams {
        radial_segments: 24,
        flare_radius: 1.0,
        ..SurfaceParams::default()
    };
    let mesh = build(&tree, 24.0, &params).unwrap();
    assert_eq!(mesh.runs, 2);
    let side_start = (4 * 24 + 2) * 3;
    let limit =
        tree.nodes[1].radius * (1.0 - params.lobe_depth) * (std::f64::consts::PI / 24.0).cos();
    for p in mesh.positions[side_start..side_start + 24 * 3]
        .as_chunks::<3>()
        .0
    {
        assert!(
            Vec3::new(p[0] as f64, p[1] as f64, p[2] as f64).distance(tree.nodes[1].position)
                <= limit * (1.0 + 1e-6)
        );
    }
    assert_eq!(mesh, build(&tree, 24.0, &params).unwrap());
}
#[test]
fn root_only_and_float32_limits() {
    let p = SurfaceParams::default();
    let root = Tree {
        nodes: vec![Node::root()],
        ..Tree::default()
    };
    assert!(build(&root, 24.0, &p).unwrap().positions.is_empty());
    let mut tree = straight();
    for n in &mut tree.nodes {
        n.position.x = 1e40;
    }
    assert!(build(&tree, 24.0, &p).is_err());
    for n in &mut tree.nodes {
        n.position.x = 1e10;
        n.radius = 1e-10;
        n.start_radius = 1e-10;
    }
    assert!(build(&tree, 24.0, &p).is_err());
    for params in [
        SurfaceParams {
            radial_segments: 2,
            ..p
        },
        SurfaceParams { lobes: 17, ..p },
        SurfaceParams {
            fork_socket: 1.0,
            ..p
        },
    ] {
        assert!(build(&straight(), 24.0, &params).is_err());
    }
}
#[test]
fn reversed_path_keeps_finite_frames() {
    let mut tree = straight();
    tree.nodes[2].position = Vec3::new(0.0, 2.0, 0.0);
    let mesh = build(&tree, 24.0, &SurfaceParams::default()).unwrap();
    assert!(mesh
        .positions
        .iter()
        .chain(&mesh.normals)
        .all(|v| v.is_finite()));
}
