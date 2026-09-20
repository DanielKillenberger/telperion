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

#[test]
fn shared_contacts_preserve_station_stream_and_rounded_rings() {
    use telperion_core::{
        foliage::{self, TwigPlacement},
        tree::NodeKind,
        Family,
    };
    let mut family = Family::default();
    family.skeleton.envelope.height = 4.0;
    family.canopy.surface_contact = 1.0;
    family.canopy.short_shoot_spacing = 0.0;
    family.canopy.limb_clumping = 0.0;
    let twig = Some(TwigPlacement {
        internode_length: 0.07,
        stations_per_internode: 3,
    });
    let mut tree = Tree {
        nodes: vec![Node {
            radius: 0.2,
            start_radius: 0.2,
            ..Node::root()
        }],
        crossover: 1,
        ..Tree::default()
    };
    for (parent, position, radius, branch) in [
        (0, Vec3::new(0., 1., 0.), 0.1, 1),
        (1, Vec3::new(0.2, 2., 0.1), 0.07, 1),
        (0, Vec3::new(0.4, 1., 0.), 0.15, 2),
        (3, Vec3::new(0.6, 2., 0.2), 0.12, 2),
    ] {
        tree.nodes.push(Node {
            parent: Some(parent),
            position,
            radius,
            start_radius: radius,
            base_radius: radius,
            kind: NodeKind::Twig,
            branch,
            ..Node::root()
        });
    }
    for burial in [0.0, 0.02] {
        family.surface.flare_depth = burial;
        let shared = surface::prepared::prepare_with_contacts(&tree, 4.0, &family.surface)
            .unwrap()
            .unwrap();
        let original = foliage::prepared::prepare_stations(
            &tree,
            family.skeleton.envelope,
            family.canopy,
            twig,
            &family.surface,
        )
        .unwrap()
        .unwrap();
        let borrowed = foliage::prepared::prepare_shared_stations(
            &shared,
            family.skeleton.envelope,
            family.canopy,
            twig,
        )
        .unwrap()
        .unwrap();
        let compact = surface::compact::prepare_with_contacts(&tree, 4.0, &family.surface).unwrap();
        let compact_stations = foliage::prepared::prepare_compact_stations(
            &compact,
            family.skeleton.envelope,
            family.canopy,
            twig,
        )
        .unwrap()
        .unwrap();
        assert_eq!(compact_stations.count, borrowed.count);
        assert_eq!(compact_stations.ring_size, borrowed.ring_size);
        assert_eq!(
            format!("{:?}", compact_stations.segments),
            format!("{:?}", borrowed.segments)
        );
        assert_eq!(original.count, borrowed.count);
        assert!(original.count > 0);
        assert_eq!(original.segments.len(), borrowed.segments.len());
        assert_eq!(original.ring_size, borrowed.ring_size);
        assert_eq!(borrowed.rings.as_ptr(), shared.surface().positions.as_ptr());
        for (mut a, b) in original.segments.into_iter().zip(borrowed.segments) {
            for (old, new) in a.contact.unwrap().into_iter().zip(b.contact.unwrap()) {
                for k in 0..original.ring_size as usize {
                    let point = original.rings[old as usize + k];
                    let p = &borrowed.rings[(new as usize + k) * 3..][..3];
                    assert_eq!(
                        [point.x, point.y, point.z],
                        [p[0] as f64, p[1] as f64, p[2] as f64]
                    );
                }
            }
            a.contact = b.contact;
            assert_eq!(format!("{a:?}"), format!("{b:?}"));
        }
        assert_eq!(
            shared.surface().positions,
            surface::prepared::prepare(&tree, 4.0, &family.surface)
                .unwrap()
                .unwrap()
                .positions
        );
    }
}

#[test]
fn shared_empty_and_validation_outcomes_are_explicit() {
    use telperion_core::{
        foliage::{self, TwigPlacement},
        Family,
    };
    let params = SurfaceParams::default();
    for tree in [
        Tree::default(),
        Tree {
            nodes: vec![Node::root()],
            ..Tree::default()
        },
    ] {
        let shared = surface::prepared::prepare_with_contacts(&tree, 1.0, &params)
            .unwrap()
            .unwrap();
        assert!(shared.surface().positions.is_empty());
    }
    let family = Family::default();
    let tree = Tree::default();
    let shared =
        surface::prepared::prepare_with_contacts(&tree, family.skeleton.envelope.height, &params)
            .unwrap()
            .unwrap();
    let twig = Some(TwigPlacement {
        internode_length: 0.1,
        stations_per_internode: 1,
    });
    let mut env = family.skeleton.envelope;
    env.height += 1.0;
    assert!(foliage::prepared::prepare_shared_stations(&shared, env, family.canopy, twig).is_err());
    let mut canopy = family.canopy;
    canopy.short_shoot_spacing = 0.1;
    assert!(foliage::prepared::prepare_shared_stations(
        &shared,
        family.skeleton.envelope,
        canopy,
        twig
    )
    .unwrap()
    .is_none());
    canopy.size = f64::NAN;
    assert!(foliage::prepared::prepare_shared_stations(
        &shared,
        family.skeleton.envelope,
        canopy,
        twig
    )
    .is_err());
}

#[test]
fn compact_precision_domain_boundaries() {
    use surface::compact::qualified_ring;
    for (centre, radius, expected) in [
        (0.0, 1.0 / 131072.0, true),
        (0.0, 0.5 / 131072.0, false),
        (64.0, 64.0 / 131072.0, true),
        (64.0001, 1.0, false),
        (25.0, 0.0001, false),
        (0.0, 32.0, true),
        (0.0, 32.0001, false),
        (f32::INFINITY, 1.0, false),
        (0.0, f32::NAN, false),
    ] {
        assert_eq!(qualified_ring([centre, -centre, centre], radius), expected);
    }
}
