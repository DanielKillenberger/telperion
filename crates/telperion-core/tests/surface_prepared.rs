use telperion_core::{
    math::Vec3,
    surface::{self, SurfaceParams},
    tree::{Node, Tree},
};
#[test]
fn prepared_preserves_canonical_surface_without_expanded_arrays() {
    let tree = Tree {
        nodes: (0..4)
            .map(|i| Node {
                position: Vec3::new(0.1 * (i as f64).sin(), i as f64, 0.05 * i as f64),
                parent: (i > 0).then(|| i - 1),
                radius: 0.2 / (i + 1) as f64,
                start_radius: 0.2,
                ..Node::root()
            })
            .collect(),
        ..Tree::default()
    };
    let params = SurfaceParams::default();
    let cpu = surface::build(&tree, 4.0, &params).unwrap();
    let p = surface::prepared::prepare(&tree, 4.0, &params)
        .unwrap()
        .unwrap();
    assert_eq!(p.positions, cpu.positions);
    assert_eq!(p.bounds, cpu.bounds);
    assert_eq!(p.run_table, cpu.run_table);
    let indices: Vec<_> = p
        .runs
        .iter()
        .flat_map(|r| (0..r.index_count / 3).flat_map(|f| r.triangle(f, p.segments)))
        .collect();
    assert_eq!(indices, cpu.indices);
    assert!(surface::prepared::prepare(&Tree::default(), 4.0, &params)
        .unwrap()
        .unwrap()
        .positions
        .is_empty());
}

#[test]
fn clumps_reserve_every_buried_ring_and_collapsed_surfaces_fall_back() {
    let mut root = Node::root();
    root.radius = 0.1;
    root.start_radius = 0.1;
    let mut tree = Tree {
        nodes: vec![root],
        ..Tree::default()
    };
    for x in [-1.0, 0.0, 1.0] {
        tree.nodes.push(Node {
            parent: Some(0),
            position: Vec3::new(x, 2.0, 0.0),
            radius: 0.1,
            start_radius: 0.1,
            ..Node::root()
        });
    }
    let params = SurfaceParams::default();
    let p = surface::prepared::prepare(&tree, 4.0, &params)
        .unwrap()
        .unwrap();
    let cpu = surface::build(&tree, 4.0, &params).unwrap();
    assert_eq!(p.positions, cpu.positions);
    assert_eq!(p.run_table, cpu.run_table);
    assert_eq!(p.rings.capacity(), p.rings.len());
    tree.nodes[1].radius = 1e-300;
    assert!(surface::prepared::prepare(&tree, 4.0, &params)
        .unwrap()
        .is_none());
}

#[test]
fn root_only_has_no_surface_or_gpu_work() {
    let p = surface::prepared::prepare(
        &Tree {
            nodes: vec![Node::root()],
            ..Tree::default()
        },
        1.0,
        &SurfaceParams::default(),
    )
    .unwrap()
    .unwrap();
    assert!(p.positions.is_empty() && p.runs.is_empty() && p.bounds.is_none());
    assert_eq!(p.index_count, 0);
}
